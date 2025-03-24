static USAGE: &str = r#"
applydp is a slimmed-down version of apply specifically created for Datapusher+.
It "applies" a series of transformation functions to given CSV column/s. This can be used to
perform typical data-wrangling tasks and/or to harmonize some values, etc.

It has three subcommands:
 1. operations*   - 18 string, format & regex operators.
 2. emptyreplace* - replace empty cells with <--replacement> string.
 3. dynfmt        - Dynamically constructs a new column from other columns using
                    the <--formatstr> template.
    * subcommand is multi-column capable.

OPERATIONS (multi-column capable)
Multiple operations can be applied, with the comma-delimited operation series
applied in order:

  trim => Trim the cell
  trim,upper => Trim the cell, then transform to uppercase

Operations support multi-column transformations. Just make sure the
number of transformed columns with the --rename option is the same. e.g.:

$ qsv applydp operations trim,upper col1,col2,col3 -r newcol1,newcol2,newcol3 file.csv

It has 18 supported operations:

  * len: Return string length
  * lower: Transform to lowercase
  * upper: Transform to uppercase
  * squeeze: Compress consecutive whitespaces
  * squeeze0: Remove whitespace
  * trim: Trim (drop whitespace left & right of the string)
  * ltrim: Left trim whitespace
  * rtrim: Right trim whitespace
  * mtrim: Trims --comparand matches left & right of the string (Rust trim_matches)
  * mltrim: Left trim --comparand matches (Rust trim_start_matches)
  * mrtrim: Right trim --comparand matches (Rust trim_end_matches)
  * strip_prefix: Removes specified prefix in --comparand
  * strip_suffix: Remove specified suffix in --comparand
  * escape - escape (Rust escape_default)
  * replace: Replace all matches of a pattern (using --comparand)
      with a string (using --replacement) (Rust replace)
  * regex_replace: Replace all regex matches in --comparand w/ --replacement.
      Specify <NULL> as --replacement to remove matches.
  * round: Round numeric values to the specified number of decimal places using
      Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
      Specify the number of decimal places with --formatstr (default: 3).
  * copy: Mark a column for copying


Examples:
Trim, then transform to uppercase the surname field.

  $ qsv applydp operations trim,upper surname file.csv

Trim, then transform to uppercase the surname field and rename the column uppercase_clean_surname.

  $ qsv applydp operations trim,upper surname -r uppercase_clean_surname file.csv

Trim, then transform to uppercase the surname field and 
save it to a new column named uppercase_clean_surname.

  $ qsv applydp operations trim,upper surname -c uppercase_clean_surname file.csv

Trim, squeeze, then transform to uppercase in place ALL fields that end with "_name" 
  
    $ qsv applydp operations trim,squeeze,upper \_name$\ file.csv

Trim, then transform to uppercase the firstname and surname fields and
rename the columns ufirstname and usurname.

  $ qsv applydp operations trim,upper firstname,surname -r ufirstname,usurname file.csv

Trim parentheses & brackets from the description field.

  $ qsv applydp operations mtrim description --comparand '()<>' file.csv

Replace ' and ' with ' & ' in the description field.

  $ qsv applydp replace description --comparand ' and ' --replacement ' & ' file.csv

You can also use this subcommand command to make a copy of a column:

  $ qsv applydp operations copy col_to_copy -c col_copy file.csv

EMPTYREPLACE (multi-column capable)
Replace empty cells with <--replacement> string.
Non-empty cells are not modified. See the `fill` command for more complex empty field operations.

Examples:
Replace empty cells in file.csv Measurement column with 'None'.

$ qsv applydp emptyreplace Measurement --replacement None file.csv

Replace empty cells in file.csv Measurement column with 'Unknown Measurement'.

$ qsv applydp emptyreplace Measurement --replacement 'Unknown Measurement' file.csv

Replace empty cells in file.csv M1,M2 and M3 columns with 'None'.

$ qsv applydp emptyreplace M1,M2,M3 --replacement None file.csv

Replace all empty cells in file.csv for columns that start with 'Measurement' with 'None'.

$ qsv applydp emptyreplace '/^Measurement/' --replacement None file.csv

Replace all empty cells in file.csv for columns that start with 'observation'
case insensitive with 'None'.

$ qsv applydp emptyreplace --replacement None '/(?i)^observation/' file.csv

DYNFMT
Dynamically constructs a new column from other columns using the <--formatstr> template.
The template can contain arbitrary characters. To insert a column value, enclose the
column name in curly braces, replacing all non-alphanumeric characters with underscores.

If you need to dynamically construct a column with more complex formatting requirements and
computed values, check out the py command to take advantage of Python's f-string formatting.

Examples:
Create a new column 'mailing address' from 'house number', 'street', 'city' and 'zip-code' columns:

  $ qsv applydp dynfmt --formatstr '{house_number} {street}, {city} {zip_code} USA' -c 'mailing address' file.csv

Create a new column 'FullName' from 'FirstName', 'MI', and 'LastName' columns:

  $ qsv applydp dynfmt --formatstr 'Sir/Madam {FirstName} {MI}. {LastName}' -c FullName file.csv

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_applydp.rs.

Usage:
qsv applydp operations <operations> [options] <column> [<input>]
qsv applydp emptyreplace --replacement=<string> [options] <column> [<input>]
qsv applydp dynfmt --formatstr=<string> [options] --new-column=<name> [<input>]
qsv applydp --help

apply arguments:
    <column>                    The column/s to apply the transformation to.
                                Note that the <column> argument supports multiple columns
                                for the operations & emptyreplace subcommands.
                                See 'qsv select --help' for the format details.

    OPERATIONS subcommand:
        <operations>            The operation/s to apply.
        <column>                The column/s to apply the operations to.

    EMPTYREPLACE subcommand:
        --replacement=<string>  The string to to use to replace empty values.
        <column>                The column/s to check for emptiness.

    DYNFMT subcommand:
        --formatstr=<string>    The template to use for the dynfmt operation.
                                See DYNFMT example above for more details.
        --new-column=<name>     Put the generated values in a new column.

    <input>                     The input file to read from. If not specified, reads from stdin.

applydp options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    -C, --comparand=<string>    The string to compare against for replace & similarity operations.
    -R, --replacement=<string>  The string to use for the replace & emptyreplace operations.
    -f, --formatstr=<string>    This option is used by several subcommands:

                                OPERATIONS:
                                  round
                                    The number of decimal places to round to (default: 3)

                                DYNFMT: the template to use to construct a new column.

    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                Set to 0 to load all rows in one batch.
                                [default: 50000]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
"#;

use std::{str::FromStr, sync::OnceLock};

use dynfmt2::Format;
use log::debug;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::Deserialize;
use smallvec::SmallVec;
use strum_macros::EnumString;

use crate::{
    CliResult,
    clitypes::CliError,
    config::{Config, Delimiter},
    regex_oncelock,
    select::SelectColumns,
    util,
    util::replace_column_value,
};

#[derive(Clone, EnumString, PartialEq)]
#[strum(use_phf)]
#[strum(ascii_case_insensitive)]
#[allow(non_camel_case_types)]
enum Operations {
    Copy,
    Escape,
    Len,
    Lower,
    Ltrim,
    Mltrim,
    Mrtrim,
    Mtrim,
    Regex_Replace,
    Replace,
    Round,
    Rtrim,
    Squeeze,
    Squeeze0,
    Strip_Prefix,
    Strip_Suffix,
    Trim,
    Upper,
}

#[derive(Deserialize)]
struct Args {
    arg_column:       SelectColumns,
    cmd_operations:   bool,
    arg_operations:   String,
    cmd_dynfmt:       bool,
    cmd_emptyreplace: bool,
    arg_input:        Option<String>,
    flag_rename:      Option<String>,
    flag_comparand:   String,
    flag_replacement: String,
    flag_formatstr:   String,
    flag_batch:       usize,
    flag_jobs:        Option<usize>,
    flag_new_column:  Option<String>,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
}

static REGEX_REPLACE: OnceLock<Regex> = OnceLock::new();
static ROUND_PLACES: OnceLock<u32> = OnceLock::new();

// default number of decimal places to round to
const DEFAULT_ROUND_PLACES: u32 = 3;

const NULL_VALUE: &str = "<null>";

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    // safety: we just checked that sel is not empty in the previous line
    let column_index = *sel.iter().next().unwrap();

    let mut headers = rdr.headers()?.clone();

    if let Some(new_name) = args.flag_rename {
        let new_col_names = util::ColumnNameParser::new(&new_name).parse()?;
        if new_col_names.len() != sel.len() {
            return fail_incorrectusage_clierror!(
                "Number of new columns does not match input column selection."
            );
        }
        for (i, col_index) in sel.iter().enumerate() {
            headers = replace_column_value(&headers, *col_index, &new_col_names[i]);
        }
    }

    // for dynfmt, safe_headers are the "safe" version of colnames - alphanumeric only,
    // all other chars replaced with underscore
    // dynfmt_fields are the columns used in the dynfmt --formatstr option
    // we prep it so we only populate the lookup vec with the index of these columns
    // so SimpleCurlyFormat is performant
    let dynfmt_template = if args.cmd_dynfmt {
        if args.flag_no_headers {
            return fail_incorrectusage_clierror!("dynfmt/calcconv subcommand requires headers.");
        }

        let mut dynfmt_template_wrk = args.flag_formatstr.clone();
        let mut dynfmt_fields = Vec::new();

        // first, get the fields used in the dynfmt template
        let formatstr_re: &'static Regex = crate::regex_oncelock!(r"\{(?P<key>\w+)?\}");
        for format_fields in formatstr_re.captures_iter(&args.flag_formatstr) {
            // safety: we already checked that the regex match is valid
            dynfmt_fields.push(format_fields.name("key").unwrap().as_str());
        }
        // we sort the fields so we can do binary_search
        dynfmt_fields.sort_unstable();

        // now, get the indices of the columns for the lookup vec
        let (safe_headers, _) = util::safe_header_names(&headers, false, false, None, "", true);
        for (i, field) in safe_headers.iter().enumerate() {
            if dynfmt_fields.binary_search(&field.as_str()).is_ok() {
                let field_with_curly = format!("{{{field}}}");
                let field_index = format!("{{{i}}}");
                dynfmt_template_wrk = dynfmt_template_wrk.replace(&field_with_curly, &field_index);
            }
        }
        debug!("dynfmt_fields: {dynfmt_fields:?}  dynfmt_template: {dynfmt_template_wrk}");
        dynfmt_template_wrk
    } else {
        String::new()
    };

    #[derive(PartialEq)]
    enum ApplydpSubCmd {
        Operations,
        DynFmt,
        EmptyReplace,
    }

    let mut ops_vec = SmallVec::<[Operations; 4]>::new();

    let applydp_cmd = if args.cmd_operations {
        match validate_operations(
            &args.arg_operations.split(',').collect(),
            &args.flag_comparand,
            &args.flag_replacement,
            &args.flag_new_column,
            &args.flag_formatstr,
        ) {
            Ok(operations_vec) => ops_vec = operations_vec,
            Err(e) => return Err(e),
        }
        ApplydpSubCmd::Operations
    } else if args.cmd_dynfmt {
        ApplydpSubCmd::DynFmt
    } else if args.cmd_emptyreplace {
        ApplydpSubCmd::EmptyReplace
    } else {
        return fail!("Unknown applydp subcommand.");
    };

    if !rconfig.no_headers {
        if let Some(new_column) = &args.flag_new_column {
            headers.push_field(new_column);
        }
        wtr.write_record(&headers)?;
    }

    // if there is a regex_replace operation and replacement is <NULL> case-insensitive,
    // we set it to empty string
    let flag_replacement = if applydp_cmd == ApplydpSubCmd::Operations
        && ops_vec.contains(&Operations::Regex_Replace)
        && args.flag_replacement.to_ascii_lowercase() == NULL_VALUE
    {
        String::new()
    } else {
        args.flag_replacement
    };
    let flag_comparand = args.flag_comparand;
    let flag_new_column = args.flag_new_column;

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    // reuse batch buffers
    let batchsize: usize = if args.flag_batch == 0 {
        util::count_rows(&rconfig)? as usize
    } else {
        args.flag_batch
    };
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    util::njobs(args.flag_jobs);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(std::mem::take(&mut batch_record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual applydp command via Rayon parallel iterator
        batch
            .par_iter()
            .with_min_len(1024)
            .map(|record_item| {
                let mut record = record_item.clone();
                match applydp_cmd {
                    ApplydpSubCmd::Operations => {
                        let mut cell = String::new();
                        for col_index in sel.iter() {
                            record[*col_index].clone_into(&mut cell);
                            applydp_operations(
                                &ops_vec,
                                &mut cell,
                                &flag_comparand,
                                &flag_replacement,
                            );
                            if flag_new_column.is_some() {
                                record.push_field(&cell);
                            } else {
                                record = replace_column_value(&record, *col_index, &cell);
                            }
                        }
                    },
                    ApplydpSubCmd::EmptyReplace => {
                        let mut cell = String::new();
                        for col_index in sel.iter() {
                            record[*col_index].clone_into(&mut cell);
                            if cell.trim().is_empty() {
                                cell = flag_replacement.clone();
                            }
                            if flag_new_column.is_some() {
                                record.push_field(&cell);
                            } else {
                                record = replace_column_value(&record, *col_index, &cell);
                            }
                        }
                    },
                    ApplydpSubCmd::DynFmt => {
                        let mut cell = record[column_index].to_owned();
                        if !cell.is_empty() {
                            let mut record_vec: Vec<String> = Vec::with_capacity(record.len());
                            for field in &record {
                                record_vec.push(field.to_string());
                            }
                            if let Ok(formatted) =
                                dynfmt2::SimpleCurlyFormat.format(&dynfmt_template, record_vec)
                            {
                                cell = formatted.to_string();
                            }
                        }
                        if flag_new_column.is_some() {
                            record.push_field(&cell);
                        } else {
                            record = replace_column_value(&record, column_index, &cell);
                        }
                    },
                }

                record
            })
            .collect_into_vec(&mut batch_results);

        // rayon collect() guarantees original order, so we can just append results each batch
        for result_record in &batch_results {
            wtr.write_record(result_record)?;
        }

        batch.clear();
    } // end batch loop

    Ok(wtr.flush()?)
}

// validate applydp operations for required options
// and prepare operations enum vec
fn validate_operations(
    operations: &Vec<&str>,
    flag_comparand: &str,
    flag_replacement: &str,
    flag_new_column: &Option<String>,
    flag_formatstr: &str,
) -> Result<SmallVec<[Operations; 4]>, CliError> {
    let mut copy_invokes = 0_u8;
    let mut regex_replace_invokes = 0_u8;
    let mut replace_invokes = 0_u8;
    let mut strip_invokes = 0_u8;

    let mut ops_vec = SmallVec::with_capacity(operations.len());

    for op in operations {
        let Ok(operation) = Operations::from_str(op) else {
            return fail_incorrectusage_clierror!("Unknown '{op}' operation");
        };
        match operation {
            Operations::Copy => {
                if flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--new_column (-c) is required for copy operation."
                    );
                }
                copy_invokes = copy_invokes.saturating_add(1);
            },
            Operations::Mtrim | Operations::Mltrim | Operations::Mrtrim => {
                if flag_comparand.is_empty() {
                    return fail_incorrectusage_clierror!(
                        "--comparand (-C) is required for match trim operations."
                    );
                }
            },
            Operations::Regex_Replace => {
                if flag_comparand.is_empty() || flag_replacement.is_empty() {
                    return fail_incorrectusage_clierror!(
                        "--comparand (-C) and --replacement (-R) are required for regex_replace \
                         operation."
                    );
                }
                if regex_replace_invokes == 0 {
                    let re = match regex::Regex::new(flag_comparand) {
                        Ok(re) => re,
                        Err(err) => {
                            return fail_clierror!("regex_replace expression error: {err:?}");
                        },
                    };
                    let _ = REGEX_REPLACE.set(re);
                }
                regex_replace_invokes = regex_replace_invokes.saturating_add(1);
            },
            Operations::Replace => {
                if flag_comparand.is_empty() || flag_replacement.is_empty() {
                    return fail_incorrectusage_clierror!(
                        "--comparand (-C) and --replacement (-R) are required for replace \
                         operation."
                    );
                }
                replace_invokes = replace_invokes.saturating_add(1);
            },
            Operations::Strip_Prefix | Operations::Strip_Suffix => {
                if flag_comparand.is_empty() {
                    return fail_incorrectusage_clierror!(
                        "--comparand (-C) is required for strip operations."
                    );
                }
                strip_invokes = strip_invokes.saturating_add(1);
            },
            Operations::Round => {
                if ROUND_PLACES
                    .set(
                        flag_formatstr
                            .parse::<u32>()
                            .unwrap_or(DEFAULT_ROUND_PLACES),
                    )
                    .is_err()
                {
                    return fail!("Cannot initialize Round precision.");
                };
            },
            _ => {},
        }
        ops_vec.push(operation);
    }
    if copy_invokes > 1 || regex_replace_invokes > 1 || replace_invokes > 1 || strip_invokes > 1 {
        return fail_incorrectusage_clierror!(
            "you can only use copy({copy_invokes}), regex_replace({regex_replace_invokes}), \
             replace({replace_invokes}), and strip({strip_invokes}) ONCE per operation series."
        );
    };

    Ok(ops_vec) // no validation errors
}

#[inline]
fn applydp_operations(
    ops_vec: &SmallVec<[Operations; 4]>,
    cell: &mut String,
    comparand: &str,
    replacement: &str,
) {
    for op in ops_vec {
        match op {
            Operations::Len => {
                *cell = itoa::Buffer::new().format(cell.len()).to_owned();
            },
            Operations::Lower => {
                *cell = cell.to_lowercase();
            },
            Operations::Upper => {
                *cell = cell.to_uppercase();
            },
            Operations::Squeeze => {
                let squeezer: &'static Regex = regex_oncelock!(r"\s+");
                *cell = squeezer.replace_all(cell, " ").into_owned();
            },
            Operations::Squeeze0 => {
                let squeezer: &'static Regex = regex_oncelock!(r"\s+");
                *cell = squeezer.replace_all(cell, "").into_owned();
            },
            Operations::Trim => {
                *cell = String::from(cell.trim());
            },
            Operations::Ltrim => {
                *cell = String::from(cell.trim_start());
            },
            Operations::Rtrim => {
                *cell = String::from(cell.trim_end());
            },
            Operations::Mtrim => {
                let chars_to_trim: &[char] = &comparand.chars().collect::<Vec<_>>();
                *cell = String::from(cell.trim_matches(chars_to_trim));
            },
            Operations::Mltrim => {
                *cell = String::from(cell.trim_start_matches(comparand));
            },
            Operations::Mrtrim => {
                *cell = String::from(cell.trim_end_matches(comparand));
            },
            Operations::Escape => {
                *cell = cell.escape_default().to_string();
            },
            Operations::Strip_Prefix => {
                if let Some(stripped) = cell.strip_prefix(comparand) {
                    *cell = String::from(stripped);
                }
            },
            Operations::Strip_Suffix => {
                if let Some(stripped) = cell.strip_suffix(comparand) {
                    *cell = String::from(stripped);
                }
            },
            Operations::Replace => {
                *cell = cell.replace(comparand, replacement);
            },
            Operations::Regex_Replace => {
                // safety: we set REGEX_REPLACE in validate_operations()
                let regexreplace = REGEX_REPLACE.get().unwrap();
                *cell = regexreplace.replace_all(cell, replacement).into_owned();
            },
            Operations::Round => {
                if let Ok(num) = cell.parse::<f64>() {
                    // safety: we set ROUND_PLACES in validate_operations()
                    *cell = util::round_num(num, *ROUND_PLACES.get().unwrap());
                }
            },
            Operations::Copy => {}, // copy is a noop
        }
    }
}
