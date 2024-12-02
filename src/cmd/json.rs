static USAGE: &str = r#"
Convert JSON to CSV.

The JSON data is expected to be non-empty and non-nested as either:

1. An array of objects where:
   A. All objects are non-empty, have non-empty and unique keys, and the same keys are in each object.
   B. Values are not objects or arrays.
2. An object where values are not objects or arrays and the object is as described above.

Objects with duplicate keys are not recommended as only one key and its values may be used.

If your JSON data is not in the expected format and/or is nested or complex, try using
the --jaq option to pass a jq-like filter before parsing with the above constraints.

As an example, say we have the following JSON data in a file fruits.json:

[
    {
        "fruit": "apple",
        "price": 2.50,
        "calories": 95
    },
    {
        "fruit": "banana",
        "price": 3.00,
        "calories": 105
    }
]

To convert it to CSV format run:

qsv json fruits.json

And the following is printed to the terminal:

fruit,price,calories
apple,2.5,95
banana,3.0,105

The order of the columns in the CSV file will be the same as the order of the keys in the first JSON object.
The order of the rows in the CSV file will be the same as the order of the objects in the JSON array.

If you want to select/reorder/drop columns in the output CSV, use the --select option, for example:

qsv json fruits.json --select price,fruit

And the following is printed to the terminal:

price,fruit
2.5,apple
3.0,banana

Note: Trailing zeroes in decimal numbers after the decimal are truncated (2.50 becomes 2.5).

If the JSON data was provided using stdin then either use - or do not provide a file path.
For example you may copy the JSON data above to your clipboard then run:

qsv clipboard | qsv json

When JSON data is nested or complex, try using the --jaq option and provide a filter value.
The --jaq option uses jaq (like jq). You may learn more here: https://github.com/01mf02/jaq

For example we have a .json file with a "data" key and the value being the same array as before:

{
    "data": [...]
}

We may run the following to select the JSON file and convert the nested array to CSV:

qsv prompt -F json | qsv json --jaq .data

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_json.rs.

Usage:
    qsv json [options] [<input>]
    qsv json --help

json options:
    --jaq <filter>         Filter JSON data using jaq syntax (https://github.com/01mf02/jaq),
                           which is identical to the popular JSON command-line tool - jq.
                           https://jqlang.github.io/jq/
                           Note that the filter is applied BEFORE converting JSON to CSV
    -s, --select <cols>    Select, reorder or drop columns for output.
                           Otherwise, all the columns will be output in the same order as
                           the first object's keys in the JSON data.
                           See 'qsv select --help' for the full syntax.
                           Note however that <cols> NEED to be a comma-delimited list
                           of column NAMES and NOT column INDICES.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{env, io::Read};

use jaq_interpret::{Ctx, FilterT, ParseCtx, RcIter, Val};
use json_objects_to_csv::{flatten_json_object::Flattener, Json2Csv};
use serde::Deserialize;

use crate::{config, select::SelectColumns, util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:   Option<String>,
    flag_jaq:    Option<String>,
    flag_select: Option<SelectColumns>,
    flag_output: Option<String>,
}

impl From<json_objects_to_csv::Error> for CliError {
    fn from(err: json_objects_to_csv::Error) -> Self {
        match err {
            json_objects_to_csv::Error::Flattening(err) => {
                CliError::Other(format!("Flattening error: {err}"))
            },
            json_objects_to_csv::Error::FlattenedKeysCollision => {
                CliError::Other(format!("Flattening Key Collision error: {err}"))
            },
            json_objects_to_csv::Error::WrittingCSV(err) => {
                CliError::Other(format!("Writing CSV error: {err}"))
            },
            json_objects_to_csv::Error::ParsingJson(err) => {
                CliError::Other(format!("Parsing JSON error: {err}"))
            },
            json_objects_to_csv::Error::InputOutput(err) => CliError::Io(err),
            json_objects_to_csv::Error::IntoFile(err) => CliError::Io(err.into()),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    fn get_value_from_stdin() -> CliResult<serde_json::Value> {
        // Create a buffer in memory for stdin
        let mut buffer: Vec<u8> = Vec::new();
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        stdin_handle.read_to_end(&mut buffer)?;
        drop(stdin_handle);

        // Return the JSON contents of the buffer as serde_json::Value
        match serde_json::from_slice(&buffer) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from stdin: {err}"),
        }
    }

    fn get_value_from_path(path: String) -> CliResult<serde_json::Value> {
        // Open the file in read-only mode with buffer.
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::with_capacity(config::DEFAULT_RDR_BUFFER_CAPACITY, file);

        // Return the JSON contents of the file as serde_json::Value
        match serde_json::from_reader(reader) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from file: {err}"),
        }
    }

    let args: Args = util::get_args(USAGE, argv)?;

    let mut value = match args.arg_input {
        Some(path) if path == "-" => get_value_from_stdin()?,
        Some(path) => get_value_from_path(path)?,
        None => get_value_from_stdin()?,
    };

    if value.is_null() {
        return fail_clierror!("No JSON data found.");
    }

    if let Some(filter) = args.flag_jaq {
        // Parse jaq filter based on JSON input
        let mut defs = ParseCtx::new(Vec::new());
        let (f, _errs) = jaq_parse::parse(filter.as_str(), jaq_parse::main());
        let f = defs.compile(f.unwrap());
        let inputs = RcIter::new(core::iter::empty());
        let out = f
            .run((Ctx::new([], &inputs), Val::from(value.clone())))
            .filter_map(std::result::Result::ok);

        #[allow(clippy::from_iter_instead_of_collect)]
        let jaq_value = serde_json::Value::from_iter(out);

        // from_iter creates a Value::Array even if the JSON data is an array,
        // so we unwrap this generated Value::Array to get the actual filtered output.
        // This allows the user to filter with '.data' for {"data": [...]} instead of not being able
        // to use '.data'. Both '.data' and '.data[]' should work with this implementation.
        value = if jaq_value
            .as_array()
            .is_some_and(|arr| arr.first().is_some_and(serde_json::Value::is_array))
        {
            jaq_value.as_array().unwrap().first().unwrap().to_owned()
        } else {
            jaq_value
        };
    }

    if value.is_null() {
        return fail_clierror!("All JSON data filtered.");
    }

    let first_dict = if value.is_array() {
        value
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|val| val.as_object())
            .ok_or_else(|| CliError::Other("Expected an array of objects in JSON".to_string()))?
    } else {
        value
            .as_object()
            .ok_or_else(|| CliError::Other("Expected a JSON object".to_string()))?
    };
    if first_dict.is_empty() {
        return Err(CliError::Other(
            "Expected a non-empty JSON object".to_string(),
        ));
    }
    let mut first_dict_headers: Vec<&str> = Vec::new();
    for key in first_dict.keys() {
        if key.is_empty() {
            return Err(CliError::Other("Expected a non-empty JSON key".to_string()));
        }
        if first_dict_headers.contains(&key.as_str()) {
            return Err(CliError::Other(format!(
                "Expected non-duplicate keys, found key: {key}"
            )));
        }
        first_dict_headers.push(key.as_str());
    }

    // STEP 1: create an intermediate CSV tempfile from the JSON data
    // we need to do this so we can use qsv select to reorder headers to first dict's keys order
    // as the order of the headers in the CSV file is not guaranteed to be the same as the order of
    // the keys in the JSON object
    let temp_dir = env::temp_dir();
    let intermediate_csv = temp_dir
        .join("intermediate.csv")
        .to_string_lossy()
        .into_owned();

    // convert JSON to CSV
    // its inside a block so all the unneeded resources are freed & flushed after the block ends
    {
        let empty_values = vec![serde_json::Value::Null; 1];
        let values = if value.is_array() {
            value.as_array().unwrap_or(&empty_values)
        } else {
            &vec![value.clone()]
        };

        let flattener = Flattener::new();
        let intermediate_csv_writer = csv::WriterBuilder::new()
            .buffer_capacity(config::DEFAULT_WTR_BUFFER_CAPACITY)
            .from_path(intermediate_csv.clone())?;
        Json2Csv::new(flattener).convert_from_array(values, intermediate_csv_writer)?;
    }

    // STEP 2: select the columns to use in the final output
    // if --select is not specified, select in the order of the first dict's keys
    // safety: we checked that first_dict is not empty so headers is not empty
    let sel_cols = args
        .flag_select
        .unwrap_or_else(|| SelectColumns::parse(&first_dict_headers.join(",")).unwrap());

    let sel_rconfig = config::Config::new(Some(intermediate_csv).as_ref()).no_headers(false);
    let mut intermediate_csv_rdr = sel_rconfig.reader()?;
    let byteheaders = intermediate_csv_rdr.byte_headers()?;

    // and write the selected columns to the final CSV file
    let sel = sel_rconfig.select(sel_cols).selection(byteheaders)?;
    let mut read_record = csv::ByteRecord::new();
    let mut write_record = csv::ByteRecord::new();
    let mut final_csv_wtr = config::Config::new(args.flag_output.as_ref())
        .no_headers(false)
        .writer()?;
    final_csv_wtr.write_record(sel.iter().map(|&i| &byteheaders[i]))?;
    while intermediate_csv_rdr.read_byte_record(&mut read_record)? {
        write_record.clear();
        write_record.extend(sel.iter().map(|&i| &read_record[i]));
        final_csv_wtr.write_byte_record(&write_record)?;
    }

    Ok(final_csv_wtr.flush()?)
}
