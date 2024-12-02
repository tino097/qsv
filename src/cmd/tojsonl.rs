#![allow(unused_assignments)]
static USAGE: &str = r#"
Smartly converts CSV to a newline-delimited JSON (JSONL/NDJSON).

By computing stats on the CSV first, it "smartly" infers the appropriate JSON data type
for each column (string, number, boolean, null).

It will infer a column as boolean if its cardinality is 2, and the first character of
the values are one of the following case-insensitive combinations:
  t/f; t/null; 1/0; 1/null; y/n & y/null are treated as true/false.

The `tojsonl` command will reuse a `stats.csv.data.jsonl` file if it exists and is
current (i.e. stats generated with --cardinality and --infer-dates options) and will
skip recomputing stats.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_tojsonl.rs.

Usage:
    qsv tojsonl [options] [<input>]
    qsv tojsonl --help

Tojsonl options:
    --trim                 Trim leading and trailing whitespace from fields
                           before converting to JSON.
    --no-boolean           Do not infer boolean fields.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    -b, --batch <size>     The number of rows per batch to load into memory,
                           before running in parallel. Automatically determined
                           for CSV files with more than 50000 rows.
                           Set to 0 to load all rows in one batch.
                           Set to 1 to force batch optimization even for files with
                           less than 50000 rows.
                           [default: 50000]                           

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -o, --output <file>    Write output to <file> instead of stdout.
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{fmt::Write, path::PathBuf, str::FromStr};

use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::Deserialize;
use serde_json::{Map, Value};
use strum_macros::EnumString;

use super::schema::infer_schema_from_stats;
use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

#[derive(Deserialize, Clone)]
struct Args {
    arg_input:       Option<String>,
    flag_trim:       bool,
    flag_no_boolean: bool,
    flag_jobs:       Option<usize>,
    flag_batch:      usize,
    flag_delimiter:  Option<Delimiter>,
    flag_output:     Option<String>,
    flag_memcheck:   bool,
}

impl From<std::fmt::Error> for CliError {
    fn from(err: std::fmt::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

#[derive(PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
enum JsonlType {
    Boolean,
    String,
    Number,
    Integer,
    Null,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;
    let work_input = util::process_input(
        vec![PathBuf::from(
            // if no input file is specified, read from stdin "-"
            args.arg_input.clone().unwrap_or_else(|| "-".to_string()),
        )],
        &tmpdir,
        "",
    )?;

    // safety: there's at least one valid element in work_input
    let input_filename = work_input[0]
        .canonicalize()?
        .into_os_string()
        .into_string()
        .unwrap();
    let conf = Config::new(Some(&input_filename)).delimiter(args.flag_delimiter);

    // we're loading the entire file into memory, we need to check avail mem
    util::mem_file_check(
        &std::path::PathBuf::from(input_filename.clone()),
        false,
        args.flag_memcheck,
    )?;

    let record_count = util::count_rows(&conf)?;

    // we're calling the schema command to infer data types and enums
    let schema_args = util::SchemaArgs {
        // we only do three, as we're only inferring boolean based on enum
        // i.e. we only inspect a field if its boolean if its domain
        // is just two values. if its more than 2, that's all we need know
        // for boolean inferencing
        flag_enum_threshold:  3,
        // ignore case for enum constraints
        // so we can properly infer booleans. e.g. if a field has a domain of
        // True, False, true, false, TRUE, FALSE that it is still a boolean
        // with a case-insensitive cardinality of 2
        flag_ignore_case:     true,
        flag_strict_dates:    false,
        flag_pattern_columns: crate::select::SelectColumns::parse("")?,
        // json doesn't have a date type, so don't infer dates
        flag_dates_whitelist: "none".to_string(),
        flag_prefer_dmy:      false,
        flag_force:           false,
        flag_stdout:          false,
        flag_jobs:            Some(util::njobs(args.flag_jobs)),
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            Some(input_filename.clone()),
        flag_memcheck:        args.flag_memcheck,
    };
    // build schema for each field by their inferred type, min/max value/length, and unique values
    let properties_map: Map<String, Value> =
        match infer_schema_from_stats(&schema_args, &input_filename) {
            Ok(map) => map,
            Err(e) => {
                return fail_clierror!("Failed to infer field types: {e}");
            },
        };

    let mut rdr = conf.reader()?;

    // TODO: instead of abusing csv writer to write jsonl file
    // just use a normal buffered writer
    let mut wtr = Config::new(args.flag_output.as_ref())
        .flexible(true)
        .no_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .writer()?;

    let headers = rdr.headers()?.clone();

    // if there are less than 3 records, we can't infer boolean fields
    let no_boolean = if record_count < 3 {
        true
    } else {
        args.flag_no_boolean
    };

    let mut lowercase_buffer = String::new();

    // create a vec lookup about inferred field data types
    let mut field_type_vec: Vec<JsonlType> = Vec::with_capacity(headers.len());
    for (_field_name, field_def) in &properties_map {
        let Some(field_map) = field_def.as_object() else {
            return fail!("Cannot create field map");
        };
        let prelim_type = field_map.get("type").unwrap();
        let field_values_enum = field_map.get("enum");

        // log::debug!("prelim_type: {prelim_type} field_values_enum: {field_values_enum:?}");

        if !no_boolean {
            // check if a field has a boolean data type
            // by checking its enum constraint
            if let Some(domain) = field_values_enum {
                if let Some(vals) = domain.as_array() {
                    // if this field only has a domain of two values
                    if vals.len() == 2 {
                        let val1 = if vals[0].is_null() {
                            '_'
                        } else {
                            // check the first domain value, if its an integer
                            // see if its 1 or 0
                            if let Some(int_val) = vals[0].as_u64() {
                                match int_val {
                                    1 => '1',
                                    0 => '0',
                                    _ => '*', // its something else
                                }
                            } else if let Some(str_val) = vals[0].as_str() {
                                // else, if its a string, get the first character of val1 lowercase
                                boolcheck(str_val, &mut lowercase_buffer)
                            } else {
                                '*'
                            }
                        };
                        // same as above, but for the 2nd domain value
                        let val2 = if vals[1].is_null() {
                            '_'
                        } else if let Some(int_val) = vals[1].as_u64() {
                            match int_val {
                                1 => '1',
                                0 => '0',
                                _ => '*',
                            }
                        } else if let Some(str_val) = vals[1].as_str() {
                            boolcheck(str_val, &mut lowercase_buffer)
                        } else {
                            '*'
                        };
                        // log::debug!("val1: {val1} val2: {val2}");

                        // check if the domain of two values is truthy or falsy
                        // i.e. if first character, case-insensitive is "t", "1" or "y" - truthy
                        // "f", "0", "n" or null - falsy
                        // if it is, infer a boolean field
                        if let ('t', 'f' | '_')
                        | ('f' | '_', 't')
                        | ('1', '0' | '_')
                        | ('0' | '_', '1')
                        | ('y', 'n' | '_')
                        | ('n' | '_', 'y') = (val1, val2)
                        {
                            field_type_vec.push(JsonlType::Boolean);
                            continue;
                        }
                    }
                }
            }
        }

        // ok to use index access and unwrap here as we know
        // we have at least one element in the prelim_type as_array
        field_type_vec.push(
            JsonlType::from_str(
                prelim_type.as_array().unwrap()[0]
                    .as_str()
                    .unwrap_or("null"),
            )
            .unwrap_or(JsonlType::String),
        );
    }

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    let num_jobs = util::njobs(args.flag_jobs);

    // reuse batch buffers
    let batchsize = util::optimal_batch_size(&conf, args.flag_batch, num_jobs);
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(true) => batch.push(std::mem::take(&mut batch_record)),
                Ok(false) => break, // nothing else to add to batch
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // process batch in parallel
        batch
            .par_iter()
            .map(|record_item| {
                let mut record = record_item.clone();
                let mut json_string = String::new();
                let mut temp_string2 = String::new();

                let mut header_key = Value::String(String::new());
                let mut temp_val = Value::String(String::new());

                if args.flag_trim {
                    record.trim();
                }
                write!(json_string, "{{").unwrap();
                for (idx, field) in record.iter().enumerate() {
                    let field_val = if let Some(field_type) = field_type_vec.get(idx) {
                        match field_type {
                            JsonlType::String => {
                                if field.is_empty() {
                                    "null"
                                } else {
                                    // we round-trip thru serde_json to escape the str
                                    // per json spec (https://www.json.org/json-en.html)
                                    temp_val = field.into();
                                    temp_string2 = temp_val.to_string();
                                    &temp_string2
                                }
                            },
                            JsonlType::Null => "null",
                            JsonlType::Integer | JsonlType::Number => field,
                            JsonlType::Boolean => {
                                if let 't' | 'y' | '1' = boolcheck(field, &mut temp_string2) {
                                    "true"
                                } else {
                                    "false"
                                }
                            },
                        }
                    } else {
                        "null"
                    };
                    header_key = headers[idx].into();
                    if field_val.is_empty() {
                        write!(json_string, r#"{header_key}:null,"#).unwrap();
                    } else {
                        write!(json_string, r#"{header_key}:{field_val},"#).unwrap();
                    }
                }
                json_string.pop(); // remove last comma
                json_string.push('}');
                record.clear();
                record.push_field(&json_string);
                record
            })
            .collect_into_vec(&mut batch_results);

        // rayon collect() guarantees original order, so we can just append results each batch
        for result_record in &batch_results {
            wtr.write_record(result_record)?;
        }

        batch.clear();
    } // end of batch loop

    Ok(wtr.flush()?)
}

#[inline]
/// check if a field is a boolean
/// by checking the first character of the field
/// and the field's domain is true/false, yes/no
fn boolcheck(field_str: &str, lowercase_buffer: &mut String) -> char {
    let mut chars = field_str.chars();
    let mut first_char = chars.next().unwrap_or('_');
    first_char.make_ascii_lowercase();

    if field_str.len() < 2 {
        return first_char;
    }

    // we use to_lowercase_into to avoid allocations for this function
    // which is called in a hot loop
    util::to_lowercase_into(field_str, lowercase_buffer);
    match lowercase_buffer.as_str() {
        "true" | "false" | "yes" | "no" => first_char,
        _ => '_',
    }
}
