static USAGE: &str = r#"
Apply a series of transformation functions to given CSV column/s. This can be used to
perform typical data-wrangling tasks and/or to harmonize some values, etc.

It has four subcommands:
 1. operations*   - 40 string, format, currency, regex & NLP operators.
 2. emptyreplace* - replace empty cells with <--replacement> string.
 3. dynfmt        - Dynamically constructs a new column from other columns using
                    the <--formatstr> template.
 4. calcconv      - parse and evaluate math expressions, with support for units
                    and conversions.
    * subcommand is multi-column capable.

OPERATIONS (multi-column capable)
Multiple operations can be applied, with the comma-delimited operation series
applied in order:

  trim => Trim the cell
  trim,upper => Trim the cell, then transform to uppercase
  lower,simdln => Lowercase the cell, then compute the normalized 
      Damerau-Levenshtein similarity to --comparand

Operations support multi-column transformations. Just make sure the
number of transformed columns with the --rename option is the same. e.g.:

# trim and fold to uppercase the col1,col2 and col3 columns and rename them
# to newcol1,newcol2 and newcol3

 $ qsv apply operations trim,upper col1,col2,col3 -r newcol1,newcol2,newcol3 file.csv

It has 40 supported operations:

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
  * encode62: base62 encode
  * decode62: base62 decode
  * encode64: base64 encode
  * decode64: base64 decode
  * crc32: crc32 checksum
  * replace: Replace all matches of a pattern (using --comparand)
      with a string (using --replacement) (Rust replace)
  * regex_replace: Replace all regex matches in --comparand w/ --replacement.
      Specify <NULL> as --replacement to remove matches.
  * titlecase - capitalizes English text using Daring Fireball titlecase style
      https://daringfireball.net/2008/05/title_case
  * censor: profanity filter. Add additional comma-delimited profanities with --comparand.
  * censor_check: check if profanity is detected (boolean).
      Add additional comma-delimited profanities with -comparand.
  * censor_count: count of profanities detected.
      Add additional comma-delimited profanities with -comparand.
  * round: Round numeric values to the specified number of decimal places using
      Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
      Specify the number of decimal places with --formatstr (default: 3).
  * thousands: Add thousands separators to numeric values.
      Specify the separator policy with --formatstr (default: comma). The valid policies are:
      comma, dot, space, underscore, hexfour (place a space every four hex digits) and
      indiancomma (place a comma every two digits, except the last three digits).
      The decimal separator can be specified with --replacement (default: '.')
  * currencytonum: Gets the numeric value of a currency. Supports currency symbols
      (e.g. $,¥,£,€,֏,₱,₽,₪,₩,ƒ,฿,₫) and strings (e.g. USD, EUR, RMB, JPY, etc.). 
      Recognizes point, comma and space separators. Is "permissive" by default, meaning it
      will allow no or non-ISO currency symbols. To enforce strict parsing, which will require
      a valid ISO currency symbol, set the --formatstr to "strict".
  * numtocurrency: Convert a numeric value to a currency. Specify the currency symbol
      with --comparand. Automatically rounds values to two decimal places. Specify
      "euro" formatting (e.g. 1.000,00 instead of 1,000.00 ) by setting --formatstr to "euro".
      Specify conversion rate by setting --replacement to a number.
  * gender_guess: Guess the gender of a name.
  * copy: Mark a column for copying
  * simdl: Damerau-Levenshtein similarity to --comparand
  * simdln: Normalized Damerau-Levenshtein similarity to --comparand (between 0.0 & 1.0)
  * simjw: Jaro-Winkler similarity to --comparand (between 0.0 & 1.0)
  * simsd: Sørensen-Dice similarity to --comparand (between 0.0 & 1.0)
  * simhm: Hamming distance to --comparand. Num of positions characters differ.
  * simod: Optimal String Alignment (OSA) Distance to --comparand.
  * eudex: Multi-lingual sounds like --comparand (boolean)
       Tested on English, Catalan, German, Spanish, Swedish and Italian dictionaries.
       It supports all C1 letters (e.g. ü, ö, æ, ß, é, etc.) and takes their sound into account.
       It should work on other European languages that use the Latin alphabet.
  * sentiment: Normalized VADER sentiment score (English only - between -1.0 to 1.0).
  * whatlang: Language Detection for 87 supported languages, with default confidence threshold
       of 0.9, which can be overridden by assigning 0.0 to 1.0 to --comparand.
       If language detection confidence is below the threshold, it will still show the best language
       guess, followed by the confidence score, ending with a question mark.
       If you want to always displays the confidence score, end the --comparand value with a
       question mark (e.g. 0.9?)
       https://github.com/greyblake/whatlang-rs/blob/master/SUPPORTED_LANGUAGES.md

Examples:
Trim, then transform to uppercase the surname field.

  $ qsv apply operations trim,upper surname file.csv

Trim, then transform to uppercase the surname field and rename the column uppercase_clean_surname.

  $ qsv apply operations trim,upper surname -r uppercase_clean_surname file.csv

Trim, then transform to uppercase the surname field and
save it to a new column named uppercase_clean_surname.

  $ qsv apply operations trim,upper surname -c uppercase_clean_surname file.csv

Trim, then transform to uppercase the firstname and surname fields and
rename the columns ufirstname and usurname.

  $ qsv apply operations trim,upper firstname,surname -r ufirstname,usurname file.csv

Trim parentheses & brackets from the description field.

  $ qsv apply operations mtrim description --comparand '()<>' file.csv

Replace ' and ' with ' & ' in the description field.

  $ qsv apply replace description --comparand ' and ' --replacement ' & ' file.csv

Extract the numeric value of the Salary column in a new column named Salary_num.

  $ qsv apply operations currencytonum Salary -c Salary_num file.csv

Convert the USD_Price to PHP_Price using the currency symbol "PHP" with a conversion rate of 60.

  $ qsv apply operations numtocurrency USD_Price -C PHP -R 60 -c PHP_Price file.csv

Base64 encode the text_col column & save the encoded value into new column named encoded & decode it.

  $ qsv apply operations encode64 text_col -c encoded file.csv | qsv apply operations decode64 encoded

Compute the Normalized Damerau-Levenshtein similarity of the neighborhood column to the string 'Roxbury'
and save it to a new column named dln_roxbury_score.

  $ qsv apply operations lower,simdln neighborhood --comparand roxbury -c dln_roxbury_score boston311.csv

You can also use this subcommand command to make a copy of a column:

  $ qsv apply operations copy col_to_copy -c col_copy file.csv

EMPTYREPLACE (multi-column capable)
Replace empty cells with <--replacement> string.
Non-empty cells are not modified. See the `fill` command for more complex empty field operations.

Examples:
Replace empty cells in file.csv Measurement column with 'None'.

$ qsv apply emptyreplace Measurement --replacement None file.csv

Replace empty cells in file.csv Measurement column with 'Unknown Measurement'.

$ qsv apply emptyreplace Measurement --replacement 'Unknown Measurement' file.csv

Replace empty cells in file.csv M1,M2 and M3 columns with 'None'.

$ qsv apply emptyreplace M1,M2,M3 --replacement None file.csv

Replace all empty cells in file.csv for columns that start with 'Measurement' with 'None'.

$ qsv apply emptyreplace '/^Measurement/' --replacement None file.csv

Replace all empty cells in file.csv for columns that start with 'observation'
case insensitive with 'None'.

$ qsv apply emptyreplace --replacement None '/(?i)^observation/' file.csv

DYNFMT
Dynamically constructs a new column from other columns using the <--formatstr> template.
The template can contain arbitrary characters. To insert a column value, enclose the
column name in curly braces, replacing all non-alphanumeric characters with underscores.

If you need to dynamically construct a column with more complex formatting requirements and
computed values, check out the py command to take advantage of Python's f-string formatting.

Examples:
Create a new column 'mailing address' from 'house number', 'street', 'city' and 'zip-code' columns:

  $ qsv apply dynfmt --formatstr '{house_number} {street}, {city} {zip_code} USA' -c 'mailing address' file.csv

Create a new column 'FullName' from 'FirstName', 'MI', and 'LastName' columns:

  $ qsv apply dynfmt --formatstr 'Sir/Madam {FirstName} {MI}. {LastName}' -c FullName file.csv

CALCCONV
Parse and evaluate math expressions into a new column, with support for units and conversions.
The math expression is built dynamically using the <--formatstr> template, similar to the DYNFMT
subcommand, with the addition that if the literal '<UNIT>' is found at the end of the template, the
inferred unit will be appended to the result.

For a complete list of supported units, constants, operators and functions, see https://docs.rs/cpc

Examples:
Do simple arithmetic:
$ qsv apply calcconv --formatstr '{col1} + {col2} * {col3}' --new-column result file.csv

With support for operators like % and ^:
$ qsv apply calcconv --formatstr '{col1} % 3' --new-column remainder file.csv

Convert from one unit to another:
$ qsv apply calcconv --formatstr '{col1}mb in gigabytes' -c gb file.csv
$ qsv apply calcconv --formatstr '{col1} Fahrenheit in Celsius" -c metric_temperature file.csv

Mix units and conversions are automatically done for you:
$ qsv apply calcconv --formatstr '{col1}km + {col2}mi in meters' -c meters file.csv

You can append the inferred unit at the end of the result by ending the expression with '<UNIT>':
$ qsv apply calcconv --formatstr '({col1} + {col2})km to light years <UNIT>' -c light_years file.csv

You can even do complex temporal unit conversions:
$ qsv apply calcconv --formatstr '{col1}m/s + {col2}mi/h in kilometers per h' -c kms_per_h file.csv

Use math functions - see https://docs.rs/cpc/latest/cpc/enum.FunctionIdentifier.html for list of functions:
$ qsv apply calcconv --formatstr 'round(sqrt{col1}^4)! liters' -c liters file.csv

Use percentages:
$ qsv apply calcconv --formatstr '10% of abs(sin(pi)) horsepower to watts' -c watts file.csv

And use very large numbers:
$ qsv apply calcconv --formatstr '{col1} Billion Trillion * {col2} quadrillion vigintillion' -c num_atoms file.csv 

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_apply.rs.

Usage:
qsv apply operations <operations> [options] <column> [<input>]
qsv apply emptyreplace --replacement=<string> [options] <column> [<input>]
qsv apply dynfmt --formatstr=<string> [options] --new-column=<name> [<input>]
qsv apply calcconv --formatstr=<string> [options] --new-column=<name> [<input>]
qsv apply --help

apply arguments:
    <column>                        The column/s to apply the transformation to.
                                    Note that the <column> argument supports multiple columns
                                    for the operations & emptyreplace subcommands.
                                    See 'qsv select --help' for the format details.

    OPERATIONS subcommand:
        <operations>                The operation/s to apply.
        <column>                    The column/s to apply the operations to.

    EMPTYREPLACE subcommand:
        --replacement=<string>      The string to use to replace empty values.
        <column>                    The column/s to check for emptiness.

    DYNFMT subcommand:
        --formatstr=<string>        The template to use for the dynfmt operation.
                                    See DYNFMT example above for more details.
        --new-column=<name>         Put the generated values in a new column.

    CALCONV subcommand:
        --formatstr=<string>        The calculation/conversion expression to use.
        --new-column=<name>         Put the calculated/converted values in a new column.
        
    <input>                     The input file to read from. If not specified, reads from stdin.

apply options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    -C, --comparand=<string>    The string to compare against for replace & similarity operations.
                                Also used with numtocurrency operation to specify currency symbol.
    -R, --replacement=<string>  The string to use for the replace & emptyreplace operations.
                                Also used with numtocurrency operation to conversion rate.
    -f, --formatstr=<string>    This option is used by several subcommands:

                                OPERATIONS:
                                  currencytonum
                                    If set to "strict", will require a valid ISO currency symbol,
                                    with the currency symbol at the beginning of the string.
                                    Otherwise, only parse the numeric part of the string and ignore
                                    the currency symbol altogether.
                                    (default: permissive)

                                  numtocurrency
                                    If set to "euro", will format the currency to use "." instead of ","
                                    as separators (e.g. 1.000,00 instead of 1,000.00 )

                                  thousands
                                    The thousands separator policy to use. The available policies are:
                                    comma, dot, space, underscore, hexfour (place a space every four 
                                    hex digits) and indiancomma (place a comma every two digits,
                                    except the last three digits). (default: comma)

                                  round
                                    The number of decimal places to round to (default: 3)

                                DYNFMT: the template to use to construct a new column.

    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                Automatically determined for CSV files with more than 50000 rows.
                                Set to 0 to load all rows in one batch. Set to 1 to force batch optimization
                                even for files with less than 50000 rows.
                                [default: 50000]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

use std::{str::FromStr, sync::OnceLock};

use base62;
use censor::{Censor, Sex, Zealous};
use cpc::{eval, units::Unit};
use crc32fast;
use data_encoding::BASE64;
use dynfmt2::Format;
use eudex::Hash;
use gender_guesser::Gender;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::debug;
use qsv_currency::Currency;
use qsv_vader_sentiment_analysis::SentimentIntensityAnalyzer;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::Deserialize;
use smallvec::SmallVec;
use strsim::{
    damerau_levenshtein, hamming, jaro_winkler, normalized_damerau_levenshtein, osa_distance,
    sorensen_dice,
};
use strum_macros::EnumString;
use thousands::{Separable, SeparatorPolicy, policies};
use titlecase::titlecase;
use whatlang::detect;

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
    Censor,
    Censor_Check,
    Censor_Count,
    Copy,
    Crc32,
    Currencytonum,
    Decode62,
    Decode64,
    Encode62,
    Encode64,
    Escape,
    Eudex,
    Gender_Guess,
    Len,
    Lower,
    Ltrim,
    Mltrim,
    Mrtrim,
    Mtrim,
    Numtocurrency,
    Regex_Replace,
    Replace,
    Round,
    Rtrim,
    Sentiment,
    Simdl,
    Simdln,
    Simhm,
    Simjw,
    Simod,
    Simsd,
    Squeeze,
    Squeeze0,
    Strip_Prefix,
    Strip_Suffix,
    Thousands,
    Titlecase,
    Trim,
    Upper,
    Whatlang,
}

#[derive(Deserialize)]
struct Args {
    arg_column:       SelectColumns,
    cmd_operations:   bool,
    arg_operations:   String,
    cmd_dynfmt:       bool,
    cmd_emptyreplace: bool,
    cmd_calcconv:     bool,
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
    flag_progressbar: bool,
}

static CENSOR: OnceLock<Censor> = OnceLock::new();
static CRC32: OnceLock<crc32fast::Hasher> = OnceLock::new();
static EUDEX_COMPARAND_HASH: OnceLock<eudex::Hash> = OnceLock::new();
static REGEX_REPLACE: OnceLock<Regex> = OnceLock::new();
static SENTIMENT_ANALYZER: OnceLock<SentimentIntensityAnalyzer> = OnceLock::new();
static THOUSANDS_POLICY: OnceLock<SeparatorPolicy> = OnceLock::new();
static ROUND_PLACES: OnceLock<u32> = OnceLock::new();
static WHATLANG_CONFIDENCE_THRESHOLD: OnceLock<f64> = OnceLock::new();
static GENDER_GUESSER: OnceLock<gender_guesser::Detector> = OnceLock::new();

// default confidence threshold for whatlang language detection - 90% confidence
const DEFAULT_THRESHOLD: f64 = 0.9;

// default number of decimal places to round to
const DEFAULT_ROUND_PLACES: u32 = 3;

const NULL_VALUE: &str = "<null>";

// for thousands operator
static INDIANCOMMA_POLICY: SeparatorPolicy = SeparatorPolicy {
    separator: ",",
    groups:    &[3, 2],
    digits:    thousands::digits::ASCII_DECIMAL,
};

// valid subcommands
#[derive(PartialEq)]
enum ApplySubCmd {
    Operations,
    DynFmt,
    EmptyReplace,
    CalcConv,
}

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
    let dynfmt_template = if args.cmd_dynfmt || args.cmd_calcconv {
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

    let mut ops_vec = SmallVec::<[Operations; 4]>::new();

    let apply_cmd = if args.cmd_operations {
        match validate_operations(
            &args.arg_operations.split(',').collect(),
            &args.flag_comparand,
            &args.flag_replacement,
            args.flag_new_column.as_ref(),
            &args.flag_formatstr,
        ) {
            Ok(operations_vec) => ops_vec = operations_vec,
            Err(e) => return Err(e),
        }
        ApplySubCmd::Operations
    } else if args.cmd_dynfmt {
        ApplySubCmd::DynFmt
    } else if args.cmd_emptyreplace {
        ApplySubCmd::EmptyReplace
    } else if args.cmd_calcconv {
        ApplySubCmd::CalcConv
    } else {
        return fail_incorrectusage_clierror!("Unknown apply subcommand.");
    };

    if !rconfig.no_headers {
        if let Some(new_column) = &args.flag_new_column {
            headers.push_field(new_column);
        }
        wtr.write_record(&headers)?;
    }

    // if there is a regex_replace operation and replacement is <NULL> case-insensitive,
    // we set it to empty string
    let flag_replacement = if apply_cmd == ApplySubCmd::Operations
        && ops_vec.contains(&Operations::Regex_Replace)
        && args.flag_replacement.to_ascii_lowercase() == NULL_VALUE
    {
        String::new()
    } else {
        args.flag_replacement
    };
    let flag_comparand = args.flag_comparand;
    let flag_formatstr = args.flag_formatstr;
    let flag_new_column = args.flag_new_column;

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    let num_jobs = util::njobs(args.flag_jobs);

    // reuse batch buffers
    let batchsize = util::optimal_batch_size(&rconfig, args.flag_batch, num_jobs);
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

        // do actual apply command via Rayon parallel iterator
        batch
            .par_iter()
            .with_min_len(1024)
            .map(|record_item| {
                let mut record = record_item.clone();
                match apply_cmd {
                    ApplySubCmd::Operations => {
                        let mut cell = String::new();
                        for col_index in &*sel {
                            record[*col_index].clone_into(&mut cell);
                            apply_operations(
                                &ops_vec,
                                &mut cell,
                                &flag_comparand,
                                &flag_replacement,
                                &flag_formatstr,
                            );
                            if flag_new_column.is_some() {
                                record.push_field(&cell);
                            } else {
                                record = replace_column_value(&record, *col_index, &cell);
                            }
                        }
                    },
                    ApplySubCmd::EmptyReplace => {
                        let mut cell = String::new();
                        for col_index in &*sel {
                            record[*col_index].clone_into(&mut cell);
                            if cell.trim().is_empty() {
                                cell.clone_from(&flag_replacement);
                            }
                            if flag_new_column.is_some() {
                                record.push_field(&cell);
                            } else {
                                record = replace_column_value(&record, *col_index, &cell);
                            }
                        }
                    },
                    ApplySubCmd::DynFmt => {
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
                    ApplySubCmd::CalcConv => {
                        let result = if record[column_index].is_empty() {
                            String::new()
                        } else {
                            let mut cell = record[column_index].to_owned();
                            let mut record_vec: Vec<String> = Vec::with_capacity(record.len());
                            for field in &record {
                                record_vec.push(field.to_string());
                            }
                            if let Ok(formatted) =
                                dynfmt2::SimpleCurlyFormat.format(&dynfmt_template, record_vec)
                            {
                                cell = formatted.to_string();
                            }

                            let mut append_unit = false;
                            let cell_for_eval = if cell.ends_with("<UNIT>") {
                                append_unit = true;
                                cell.trim_end_matches("<UNIT>")
                            } else {
                                &cell
                            };
                            match eval(cell_for_eval, true, Unit::Celsius, false) {
                                Ok(answer) => {
                                    if append_unit {
                                        format!("{} {:?}", answer.value, answer.unit)
                                    } else {
                                        answer.value.to_string()
                                    }
                                },
                                Err(e) => {
                                    format!("ERROR: {e}")
                                },
                            }
                        };

                        if flag_new_column.is_some() {
                            record.push_field(&result);
                        } else {
                            record = replace_column_value(&record, column_index, &result);
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

        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    if show_progress {
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}

// validate apply operations for required options
// and prepare operations enum vec
fn validate_operations(
    operations: &Vec<&str>,
    flag_comparand: &str,
    flag_replacement: &str,
    flag_new_column: Option<&String>,
    flag_formatstr: &str,
) -> Result<SmallVec<[Operations; 4]>, CliError> {
    let mut censor_invokes = 0_u8;
    let mut copy_invokes = 0_u8;
    let mut eudex_invokes = 0_u8;
    let mut regex_replace_invokes = 0_u8;
    let mut replace_invokes = 0_u8;
    let mut sentiment_invokes = 0_u8;
    let mut sim_invokes = 0_u8;
    let mut strip_invokes = 0_u8;
    let mut whatlang_invokes = 0_u8;

    let mut ops_vec = SmallVec::with_capacity(operations.len());

    for op in operations {
        let Ok(operation) = Operations::from_str(op) else {
            return fail_incorrectusage_clierror!("Unknown '{op}' operation");
        };
        match operation {
            Operations::Censor | Operations::Censor_Check | Operations::Censor_Count => {
                if flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--new_column (-c) is required for censor operations."
                    );
                }
                if censor_invokes == 0
                    && CENSOR
                        .set({
                            let mut censored_words = Censor::Standard + Zealous + Sex;
                            for word in flag_comparand.split(',') {
                                censored_words += word.trim();
                            }
                            censored_words
                        })
                        .is_err()
                {
                    return fail!("Cannot initialize Censor engine.");
                }
                censor_invokes = censor_invokes.saturating_add(1);
            },
            Operations::Copy => {
                if flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--new_column (-c) is required for copy operation."
                    );
                }
                copy_invokes = copy_invokes.saturating_add(1);
            },
            Operations::Eudex => {
                if flag_comparand.is_empty() || flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--comparand (-C) and --new_column (-c) is required for eudex."
                    );
                }
                if eudex_invokes == 0
                    && EUDEX_COMPARAND_HASH
                        .set(eudex::Hash::new(flag_comparand))
                        .is_err()
                {
                    return fail!("Cannot initialize Eudex.");
                }
                eudex_invokes = eudex_invokes.saturating_add(1);
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
                    #[allow(clippy::let_underscore_untyped)]
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
            Operations::Sentiment => {
                if flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--new_column (-c) is required for sentiment operation."
                    );
                }
                if sentiment_invokes == 0
                    && SENTIMENT_ANALYZER
                        .set(SentimentIntensityAnalyzer::new())
                        .is_err()
                {
                    return fail!("Cannot initialize Sentiment Analyzer.");
                }

                sentiment_invokes = sentiment_invokes.saturating_add(1);
            },
            Operations::Simdl
            | Operations::Simdln
            | Operations::Simjw
            | Operations::Simsd
            | Operations::Simhm
            | Operations::Simod => {
                if flag_comparand.is_empty() || flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--comparand (-C) and --new_column (-c) is required for similarity \
                         operations."
                    );
                }
                sim_invokes = sim_invokes.saturating_add(1);
            },
            Operations::Strip_Prefix | Operations::Strip_Suffix => {
                if flag_comparand.is_empty() {
                    return fail!("--comparand (-C) is required for strip operations.");
                }
                strip_invokes = strip_invokes.saturating_add(1);
            },
            Operations::Thousands => {
                let separator_policy = match flag_formatstr {
                    "dot" => policies::DOT_SEPARATOR,
                    "space" => policies::SPACE_SEPARATOR,
                    "underscore" => policies::UNDERSCORE_SEPARATOR,
                    "hexfour" => policies::HEX_FOUR,
                    "indiancomma" => INDIANCOMMA_POLICY,
                    _ => policies::COMMA_SEPARATOR,
                };
                if THOUSANDS_POLICY.set(separator_policy).is_err() {
                    return fail!("Cannot initialize Thousands policy.");
                }
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
                }
            },
            Operations::Crc32 => {
                if CRC32.set(crc32fast::Hasher::new()).is_err() {
                    return fail!("Cannot initialize CRC32 Hasher.");
                }
            },
            Operations::Whatlang => {
                if flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--new_column (-c) is required for whatlang language detection."
                    );
                }

                if whatlang_invokes == 0
                    && WHATLANG_CONFIDENCE_THRESHOLD
                        .set(if flag_comparand.is_empty() {
                            DEFAULT_THRESHOLD
                        } else {
                            let preparsed_threshold;
                            let show_confidence = if flag_comparand.ends_with('?') {
                                preparsed_threshold = flag_comparand.trim_end_matches('?');
                                true
                            } else {
                                preparsed_threshold = flag_comparand;
                                false
                            };
                            let desired_threshold = preparsed_threshold
                                .parse::<f64>()
                                .unwrap_or(DEFAULT_THRESHOLD);
                            // desired threshold can be 0.0 to 1.0
                            let final_threshold = if (0.0..=1.0).contains(&desired_threshold) {
                                desired_threshold
                            } else {
                                // its outside the valid range
                                // just set it to the default threshold
                                DEFAULT_THRESHOLD
                            };
                            if show_confidence {
                                final_threshold * -1.0
                            } else {
                                final_threshold
                            }
                        })
                        .is_err()
                {
                    return fail!("cannot initialize WhatLang language detection.");
                }
                whatlang_invokes = whatlang_invokes.saturating_add(1);
            },
            Operations::Gender_Guess => {
                if flag_new_column.is_none() {
                    return fail_incorrectusage_clierror!(
                        "--new_column (-c) is required for Gender_Guess"
                    );
                }
                if GENDER_GUESSER.set(gender_guesser::Detector::new()).is_err() {
                    return fail!("Cannot initialize Gender Detector.");
                }
            },
            _ => {},
        }
        ops_vec.push(operation);
    }
    if censor_invokes > 1
        || copy_invokes > 1
        || eudex_invokes > 1
        || regex_replace_invokes > 1
        || replace_invokes > 1
        || sentiment_invokes > 1
        || sim_invokes > 1
        || strip_invokes > 1
        || whatlang_invokes > 1
    {
        return fail_incorrectusage_clierror!(
            "you can only use censor({censor_invokes}), copy({copy_invokes}), \
             eudex({eudex_invokes}), regex_replace({regex_replace_invokes}), \
             replace({replace_invokes}), sentiment({sentiment_invokes}), \
             similarity({sim_invokes}), strip({strip_invokes}), and whatlang({whatlang_invokes}) \
             ONCE per operation series."
        );
    }

    Ok(ops_vec) // no validation errors
}

#[inline]
fn apply_operations(
    ops_vec: &SmallVec<[Operations; 4]>,
    cell: &mut String,
    comparand: &str,
    replacement: &str,
    formatstr: &str,
) {
    for op in ops_vec {
        match op {
            Operations::Len => {
                itoa::Buffer::new().format(cell.len()).clone_into(cell);
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
            Operations::Encode64 => {
                *cell = BASE64.encode(cell.as_bytes());
            },
            Operations::Decode64 => {
                let mut output = vec![0; BASE64.decode_len(cell.len()).unwrap_or_default()];
                *cell = match BASE64.decode_mut(cell.as_bytes(), &mut output) {
                    Ok(len) => simdutf8::basic::from_utf8(&output[0..len])
                        .unwrap_or_default()
                        .to_owned(),
                    Err(e) => format!("decoding64 error: {e:?}"),
                };
            },
            Operations::Encode62 => {
                *cell = match cell.parse::<u128>() {
                    Ok(num) => base62::encode(num),
                    Err(e) => format!("encode62 error: {e:?}"),
                };
            },
            Operations::Decode62 => {
                *cell = match base62::decode(cell.as_str()) {
                    Ok(decoded) => decoded.to_string(),
                    Err(e) => format!("decode62 error: {e:?}"),
                };
            },
            Operations::Crc32 => {
                // safety: we set CRC32 in validate_operations()
                // this approach is still better than using the simple hash() function
                // despite the use of clone(), because it allows us to use the
                // same hasher for multiple columns.
                // OTOH, the simple hash() function keeps creating a new hasher for every call,
                // including selection of the best algorithm repeatedly at runtime
                let mut crc32_hasher = CRC32.get().unwrap().clone();
                crc32_hasher.update(cell.as_bytes());
                itoa::Buffer::new()
                    .format(crc32_hasher.finalize())
                    .clone_into(cell);
            },
            Operations::Gender_Guess => {
                // safety: we set GENDER_GUESSER in validate_operations()
                let gender_detector = GENDER_GUESSER.get().unwrap();
                *cell = match gender_detector.get_gender(cell) {
                    Gender::Male => "Male".to_owned(),
                    Gender::Female => "Female".to_owned(),
                    Gender::MayBeMale => "MayBeMale".to_owned(),
                    Gender::MayBeFemale => "MayBeFemale".to_owned(),
                    Gender::BothMaleFemale => "BothMaleFemale".to_owned(),
                    Gender::NotSure => "NotSure".to_owned(),
                    Gender::NotFound => "NotFound".to_owned(),
                };
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
            Operations::Titlecase => {
                *cell = titlecase(cell);
            },
            Operations::Replace => {
                *cell = cell.replace(comparand, replacement);
            },
            Operations::Regex_Replace => {
                // safety: we set REGEX_REPLACE in validate_operations()
                let regexreplace = REGEX_REPLACE.get().unwrap();
                *cell = regexreplace.replace_all(cell, replacement).into_owned();
            },
            Operations::Censor => {
                // safety: we set CENSOR in validate_operations()
                let censor = CENSOR.get().unwrap();
                *cell = censor.censor(cell);
            },
            Operations::Censor_Check => {
                // safety: we set CENSOR in validate_operations()
                let censor = CENSOR.get().unwrap();
                if censor.check(cell) { "true" } else { "false" }.clone_into(cell);
            },
            Operations::Censor_Count => {
                // safety: we set CENSOR in validate_operations()
                let censor = CENSOR.get().unwrap();
                itoa::Buffer::new()
                    .format(censor.count(cell))
                    .clone_into(cell);
            },
            Operations::Thousands => {
                if let Ok(num) = cell.parse::<f64>() {
                    //safety: we set THOUSANDS_POLICY in validate_operations()
                    let mut temp_string = num.separate_by_policy(*THOUSANDS_POLICY.get().unwrap());

                    // if there is a decimal separator (fractional part > 0.0), use the requested
                    // decimal separator in --replacement
                    if num.fract() > 0.0 {
                        // if replacement is empty, use the default decimal separator (.)
                        *cell = if replacement.is_empty() {
                            temp_string
                        } else {
                            // else replace the decimal separator (last '.') w/ the requested one
                            match temp_string.rfind('.') {
                                Some(last_dot) => {
                                    temp_string.replace_range(last_dot..=last_dot, replacement);
                                    temp_string
                                },
                                None => temp_string,
                            }
                        };
                    } else {
                        *cell = temp_string;
                    }
                }
            },
            Operations::Round => {
                if let Ok(num) = cell.parse::<f64>() {
                    // safety: we set ROUND_PLACES in validate_operations()
                    *cell = util::round_num(num, *ROUND_PLACES.get().unwrap());
                }
            },
            Operations::Currencytonum => {
                // Handle currency strings with 3 decimal places by appending a 0
                // to make it 4 decimal places without affecting the value
                // This is a workaround around current limitation of qsv-currency
                // and also of upstream currency-rs, that it cannot
                // handle currency amounts properly with three decimal places
                let fract_3digits: &'static Regex = regex_oncelock!(r"\.\d\d\d$");
                let cell_val = if fract_3digits.is_match(cell) {
                    format!("{cell}0")
                } else {
                    cell.clone()
                };

                if let Ok(currency_val) = Currency::from_str(&cell_val) {
                    if formatstr == "strict" {
                        // Process ISO currency values
                        let currency_coins = currency_val.value();
                        let coins = format!("{:03}", &currency_coins);

                        if currency_val.is_iso_currency() {
                            if coins == "000" {
                                *cell = "0.00".to_string();
                            } else {
                                let coinlen = coins.len();
                                if coinlen > 2 {
                                    let decpoint = coinlen - 2;
                                    let coin_num = &coins[..decpoint];
                                    let coin_frac = &coins[decpoint..];
                                    *cell = format!("{coin_num}.{coin_frac}");
                                }
                            }
                        }
                    } else {
                        // For non-strict mode, extract numeric parts from currency strings
                        // using regex that handles various formats including thousand separators
                        // we ignore the currency symbol altogether
                        let numparts_re: &'static Regex =
                            regex_oncelock!(r"-?(?:\d{1,3}(?:[,. ]\d{3})+|(?:\d+))(?:[,.]\d+)?");

                        if let Some(numparts) = numparts_re.find(cell) {
                            // use the same workaround as above to handle 3 decimal places
                            let numparts_str = numparts.as_str();
                            let numparts_val = if fract_3digits.is_match(numparts_str) {
                                format!("{numparts_str}0")
                            } else {
                                numparts_str.to_string()
                            };

                            if let Ok(extracted_currency) = Currency::from_str(&numparts_val) {
                                let currency_coins = extracted_currency.value();
                                let coins = format!("{:03}", &currency_coins);

                                if coins == "000" {
                                    *cell = "0.00".to_string();
                                } else {
                                    let coinlen = coins.len();
                                    if coinlen > 2 {
                                        let decpoint = coinlen - 2;
                                        let coin_num = &coins[..decpoint];
                                        let coin_frac = &coins[decpoint..];
                                        *cell = format!("{coin_num}.{coin_frac}");
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Operations::Numtocurrency => {
                // same 3 decimal place workaround as currencytonum
                let fract_3digits2: &'static Regex = regex_oncelock!(r"\.\d\d\d$");
                let cell_val = if fract_3digits2.is_match(cell) {
                    format!("{cell}0")
                } else {
                    cell.clone()
                };

                if let Ok(currency_value) = Currency::from_str(&cell_val) {
                    let currency_wrk = currency_value
                        .convert(replacement.parse::<f64>().unwrap_or(1.0_f64), comparand);
                    *cell = if formatstr == "euro" {
                        format!("{currency_wrk:e}")
                    } else {
                        format!("{currency_wrk}")
                    };
                }
            },
            Operations::Simdl => {
                itoa::Buffer::new()
                    .format(damerau_levenshtein(cell, comparand))
                    .clone_into(cell);
            },
            Operations::Simdln => {
                ryu::Buffer::new()
                    .format_finite(normalized_damerau_levenshtein(cell, comparand))
                    .clone_into(cell);
            },
            Operations::Simjw => {
                ryu::Buffer::new()
                    .format_finite(jaro_winkler(cell, comparand))
                    .clone_into(cell);
            },
            Operations::Simsd => {
                ryu::Buffer::new()
                    .format_finite(sorensen_dice(cell, comparand))
                    .clone_into(cell);
            },
            Operations::Simhm => {
                let ham_val = hamming(cell, comparand);
                match ham_val {
                    Ok(val) => itoa::Buffer::new().format(val).clone_into(cell),
                    Err(_) => *cell = String::from("ERROR: Different lengths"),
                }
            },
            Operations::Simod => {
                itoa::Buffer::new()
                    .format(osa_distance(cell, comparand))
                    .clone_into(cell);
            },
            Operations::Eudex => {
                // safety: we set EUDEX_COMPARAND_HASH in validate_operations()
                let eudex_comparand_hash = EUDEX_COMPARAND_HASH.get().unwrap();
                let cell_hash = Hash::new(cell);
                *cell = format!("{}", (cell_hash - *eudex_comparand_hash).similar());
            },
            Operations::Sentiment => {
                // safety: we set SENTIMENT_ANALYZER in validate_operations()
                let sentiment_analyzer = SENTIMENT_ANALYZER.get().unwrap();
                let sentiment_scores = sentiment_analyzer.polarity_scores(cell);
                ryu::Buffer::new()
                    .format_finite(*sentiment_scores.get("compound").unwrap_or(&0.0))
                    .clone_into(cell);
            },
            Operations::Whatlang => {
                let lang_info = detect(cell);
                if let Some(lang_info) = lang_info {
                    // safety: we set WHATLANG_CONFIDENCE_THRESHOLD in validate_operations()
                    let whatlang_confidence_threshold =
                        *WHATLANG_CONFIDENCE_THRESHOLD.get().unwrap();
                    let lang_confidence = lang_info.confidence();
                    let lang = lang_info.lang();
                    if lang_confidence >= whatlang_confidence_threshold.abs() {
                        if whatlang_confidence_threshold >= 0.0 {
                            *cell = format!("{lang:?}");
                        } else {
                            *cell = format!("{lang:?}({lang_confidence:.3})");
                        }
                    } else {
                        // if confidence < confidence_threshold; show best-guessed language,
                        // confidence to 3 decimal places enclosed in parens and
                        // end with a question mark
                        *cell = format!("{lang:?}({lang_confidence:.3})?");
                    }
                }
            },
            Operations::Copy => {}, // copy is a noop
        }
    }
}
