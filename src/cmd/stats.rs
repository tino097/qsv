static USAGE: &str = r#"
Compute summary statistics & infers data types for each column in a CSV.

> NOTE: `stats` is heavily optimized for speed. It assumes the CSV is well-formed and
UTF-8 encoded. If you encounter problems generating stats, use `qsv validate` to confirm the
input CSV is valid.

Summary statistics includes sum, min/max/range, sort order, min/max/sum/avg length, mean,
standard error of the mean (SEM), stddev, variance, coefficient of variation (CV), nullcount,
max_precision, sparsity, quartiles, interquartile range (IQR), lower/upper fences, skewness, median,
cardinality, mode/s & "antimode/s", and median absolute deviation (MAD). Note that some statistics
require loading the entire file into memory, so they must be enabled explicitly. 

By default, the following "streaming" statistics are reported for *every* column:
sum, min/max/range values, sort order, min/max/sum/avg length, mean, sem, stddev, variance, cv,
nullcount, max_precision & sparsity. The default set of statistics corresponds to ones that can be
computed efficiently on a stream of data (i.e., constant memory) and works with arbitrarily large CSVs.

The following additional "non-streaming" statistics require loading the entire file into memory:
cardinality, mode/antimode, median, MAD, quartiles and its related measures (IQR,
lower/upper fences & skewness).

When computing "non-streaming" statistics, an Out-Of-Memory (OOM) heuristic check is done.
If the file is larger than the available memory minus a headroom buffer of 20% (which can be
adjusted using the QSV_FREEMEMORY_HEADROOM_PCT environment variable), processing will be
preemptively prevented.

"Antimode" is the least frequently occurring non-zero value and is the opposite of mode.
It returns "*ALL" if all the values are unique, and only returns a preview of the first
10 antimodes.

If you need all the antimode values of a column, run the `frequency` command with --limit set
to zero. The resulting frequency table will have all the antimode values.

Summary statistics for dates are also computed when --infer-dates is enabled, with DateTime
results in rfc3339 format and Date results in "yyyy-mm-dd" format in the UTC timezone.
Date range, stddev, variance, MAD & IQR are returned in days, not timestamp milliseconds.

Each column's data type is also inferred (NULL, Integer, String, Float, Date, DateTime and
Boolean with --infer-boolean option).
For String data types, it also determines if the column is all ASCII characters.
Unlike the sniff command, stats' data type inferences are GUARANTEED, as the entire file
is scanned, and not just sampled.

Note that the Date and DateTime data types are only inferred with the --infer-dates option 
as its an expensive operation to match a date candidate against 19 possible date formats,
with each format, having several variants.

The date formats recognized and its sub-variants along with examples can be found at 
https://github.com/dathere/qsv-dateparser?tab=readme-ov-file#accepted-date-formats.

Computing statistics on a large file can be made MUCH faster if you create an index for it
first with 'qsv index' to enable multithreading. With an index, the file is split into equal
chunks and each chunk is processed in parallel. The number of chunks is determined by the
number of logical CPUs detected. You can override this by setting the --jobs option.

As stats is a central command in qsv, and can be expensive to compute, `stats` caches results
in <FILESTEM>.stats.csv & if the --stats-json option is used, <FILESTEM>.stats.csv.data.jsonl
(e.g., qsv stats nyc311.csv will create nyc311.stats.csv & nyc311.stats.csv.data.jsonl).
The arguments used to generate the cached stats are saved in <FILESTEM>.stats.csv.jsonl.

If stats have already been computed for the input file with similar arguments and the file
hasn't changed, the stats will be loaded from the cache instead of recomputing it.

These cached stats are also used by other qsv commands (currently `schema` & `tojsonl`) to
load the stats into memory faster. If the cached stats are not current (i.e., the input file
is newer than the cached stats), the cached stats will be ignored and recomputed. For example,
see the "boston311" test files in 
https://github.com/dathere/qsv/blob/4529d51273218347fef6aca15ac24e22b85b2ec4/tests/test_stats.rs#L608.

Examples:

Compute "streaming" statistics for the "nyc311.csv" file:
   $ qsv stats nyc311.csv

Compute all statistics for the "nyc311.csv" file:
    $ qsv stats --everything nyc311.csv
    $ qsv stats -E nyc311.csv

Compute all statistics for the "nyc311.csv", inferring dates using default date column name patterns:
    $ qsv stats -E --infer-dates nyc311.csv

Compute all statistics for the "nyc311.csv", inferring dates only for columns with "_date" & "_dte":
    $ qsv stats -E --infer-dates --dates-whitelist _date,_dte nyc311.csv

In addition, also infer boolean data types for the "nyc311.csv" file:
    $ qsv stats -E --infer-dates --dates-whitelist _date --infer-boolean nyc311.csv

In addition to basis "streaming" stats, also compute the cardinality for the "nyc311.csv" file:
    $ qsv stats --cardinality nyc311.csv

Prefer DMY format when inferring dates for the "nyc311.csv" file:
    $ qsv stats -E --infer-dates --prefer-dmy nyc311.csv    

Infer data types only for the "nyc311.csv" file:
    $ qsv stats --typesonly nyc311.csv

Infer data types only, including boolean and date types for the "nyc311.csv" file:
    $ qsv stats --typesonly --infer-boolean --infer-dates nyc311.csv

Automatically create an index for the "nyc311.csv" file if it's larger than 5MB
and there is no existing index file:
    $ qsv stats -E --cache-threshold -5000000 nyc311.csv

Auto-create an index for the "nyc311.csv" file if it's larger than 5MB and delete the index
and the stats cache file after the stats run:
    $ qsv stats -E --cache-threshold -5000005 nyc311.csv

Prompt for CSV/TSV/TAB file to compute stats for:
    $ qsv prompt -F tsv,csv,tab | qsv stats -E | qsv table

Prompt for a file to save the stats to in the ~/Documents directory:
    $ qsv stats -E nyc311.csv | qsv prompt -d ~/Documents --fd-output

Prompt for both INPUT and OUTPUT files in the ~/Documents dir with custom prompts:
    $ qsv prompt -m 'Select a CSV file to summarize' -d ~/Documents -F csv | \
      qsv stats -E --infer-dates | \
      qsv prompt -m 'Save summary to...' -d ~/Documents --fd-output --save-fname summarystats.csv

For more examples, see https://github.com/dathere/qsv/tree/master/resources/test
For more info, see https://github.com/dathere/qsv/wiki/Supplemental#stats-command-output-explanation

Usage:
    qsv stats [options] [<input>]
    qsv stats --help

stats options:
    -s, --select <arg>        Select a subset of columns to compute stats for.
                              See 'qsv select --help' for the format details.
                              This is provided here because piping 'qsv select'
                              into 'qsv stats' will prevent the use of indexing.
    -E, --everything          Compute all statistics available.
    --typesonly               Infer data types only and do not compute statistics.
                              Note that if you want to infer dates, you'll still need to use
                              the --infer-dates and --dates-whitelist options.
    --infer-boolean           Infer boolean data type. This automatically enables
                              the --cardinality option. When a column's cardinality is 2,
                              and the 2 values' first characters are 0/1, t/f & y/n
                              case-insensitive, the data type is inferred as boolean.
    --mode                    Compute the mode/s & antimode/s. Multimodal-aware.
                              This requires loading all CSV data in memory.
    --cardinality             Compute the cardinality.
                              This requires loading all CSV data in memory.
    --median                  Compute the median.
                              This requires loading all CSV data in memory.
    --mad                     Compute the median absolute deviation (MAD).
                              This requires loading all CSV data in memory.
    --quartiles               Compute the quartiles, the IQR, the lower/upper inner/outer
                              fences and skewness.
                              This requires loading all CSV data in memory.
    --round <decimal_places>  Round statistics to <decimal_places>. Rounding is done following
                              Midpoint Nearest Even (aka "Bankers Rounding") rule.
                              https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html
                              If set to the sentinel value 9999, no rounding is done.
                              For dates - range, stddev & IQR are always at least 5 decimal places as
                              they are reported in days, and 5 places gives us millisecond precision.
                              [default: 4]
    --nulls                   Include NULLs in the population size for computing
                              mean and standard deviation.
    --infer-dates             Infer date/datetime datatypes. This is an expensive
                              option and should only be used when you know there
                              are date/datetime fields.
                              Also, if timezone is not specified in the data, it'll
                              be set to UTC.
    --dates-whitelist <list>  The case-insensitive patterns to look for when 
                              shortlisting fields for date inferencing.
                              i.e. if the field's name has any of these patterns,
                              it is shortlisted for date inferencing.
                              Set to "all" to inspect ALL fields for
                              date/datetime types. Ignored if --infer-dates is false.

                              Note that false positive date matches WILL most likely occur
                              when using "all" as unix epoch timestamps are just numbers.
                              Be sure to only use "all" if you know ALL the columns you're
                              inspecting are dates, boolean or string fields.

                              To avoid false positives, preprocess the file first 
                              with the `datefmt` command to convert unix epoch timestamp
                              columns to RFC3339 format.
                              [default: date,time,due,open,close,created]
    --prefer-dmy              Parse dates in dmy format. Otherwise, use mdy format.
                              Ignored if --infer-dates is false.
    --force                   Force recomputing stats even if valid precomputed stats
                              cache exists.
    -j, --jobs <arg>          The number of jobs to run in parallel.
                              This works only when the given CSV has an index.
                              Note that a file handle is opened for each job.
                              When not set, the number of jobs is set to the
                              number of CPUs detected.
    --stats-jsonl             Also write the stats in JSONL format. 
                              If set, the stats will be written to <FILESTEM>.stats.csv.data.jsonl.
                              Note that this option used internally by other qsv commands
                              (currently `frequency`, `schema`. `tojsonl` & `sqlp`) to load cached stats. 
                              You can preemptively create the stats-jsonl file by using
                              this option BEFORE running the `frequency`, `schema` & `tojsonl`
                              commands and they will automatically use it.
 -c, --cache-threshold <arg>  When greater than 1, the threshold in milliseconds before caching
                              stats results. If a stats run takes longer than this threshold,
                              the stats results will be cached.
                              Set to 0 to suppress caching. 
                              Set to 1 to force caching.
                              Set to a negative number to automatically create an index
                              when the input file size is greater than abs(arg) in bytes.
                              If the negative number ends with 5, it will delete the index
                              file and the stats cache file after the stats run. Otherwise,
                              the index file and the cache files are kept.
                              [default: 5000]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. i.e., They will be included
                           in statistics.
    -d, --delimiter <arg>  The field delimiter for READING CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
                           This option is ignored when computing default, streaming
                           statistics, as it is not needed.
"#;

/*
DEVELOPER NOTE: stats is heavily optimized and is a central command in qsv.

It was the primary reason I created the qsv fork as I needed to do GUARANTEED data type
inferencing & to compile smart Data Dictionaries in the most performant way possible
for Datapusher+ (https://github.com/dathere/datapusher-plus).

It underpins the `schema` and `validate` commands - enabling the automatic creation of
a JSONschema based on a CSV's summary statistics; and use the generated JSONschema to
quickly validate complex CSVs (NYC's 311 data) at almost 930,000 records/sec.

It's type inferences are also used by the `tojsonl` command to generate properly typed
JSONL files; the `frequency` command to short-circuit frequency table generation for columns
with all unique values; and the `schema` command to set the data type of a column and identify
low-cardinality columns for enum generation in the JSONschema.

To safeguard against undefined behavior, `stats` is the most extensively tested command,
with ~500 tests.
*/

use std::{
    default::Default,
    fmt, fs, io,
    io::Write,
    iter::repeat,
    path::{Path, PathBuf},
    str,
    sync::OnceLock,
};

use crossbeam_channel;
use itertools::Itertools;
use qsv_dateparser::parse_with_preference;
use serde::{Deserialize, Serialize};
use simd_json::{prelude::ValueAsScalar, OwnedValue};
use smallvec::SmallVec;
use stats::{merge_all, Commute, MinMax, OnlineStats, Unsorted};
use tempfile::NamedTempFile;
use threadpool::ThreadPool;

use self::FieldType::{TDate, TDateTime, TFloat, TInteger, TNull, TString};
use crate::{
    config::{get_delim_by_extension, Config, Delimiter},
    select::{SelectColumns, Selection},
    util, CliResult,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:            Option<String>,
    pub flag_select:          SelectColumns,
    pub flag_everything:      bool,
    pub flag_typesonly:       bool,
    pub flag_infer_boolean:   bool,
    pub flag_mode:            bool,
    pub flag_cardinality:     bool,
    pub flag_median:          bool,
    pub flag_mad:             bool,
    pub flag_quartiles:       bool,
    pub flag_round:           u32,
    pub flag_nulls:           bool,
    pub flag_infer_dates:     bool,
    pub flag_dates_whitelist: String,
    pub flag_prefer_dmy:      bool,
    pub flag_force:           bool,
    pub flag_jobs:            Option<usize>,
    pub flag_stats_jsonl:     bool,
    pub flag_cache_threshold: isize,
    pub flag_output:          Option<String>,
    pub flag_no_headers:      bool,
    pub flag_delimiter:       Option<Delimiter>,
    pub flag_memcheck:        bool,
}

// this struct is used to serialize/deserialize the stats to
// the "".stats.csv.json" file which we check to see
// if we can skip recomputing stats.
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
struct StatsArgs {
    arg_input:            String,
    flag_select:          String,
    flag_everything:      bool,
    flag_typesonly:       bool,
    flag_infer_boolean:   bool,
    flag_mode:            bool,
    flag_cardinality:     bool,
    flag_median:          bool,
    flag_mad:             bool,
    flag_quartiles:       bool,
    flag_round:           u32,
    flag_nulls:           bool,
    flag_infer_dates:     bool,
    flag_dates_whitelist: String,
    flag_prefer_dmy:      bool,
    flag_no_headers:      bool,
    flag_delimiter:       String,
    flag_output_snappy:   bool,
    canonical_input_path: String,
    canonical_stats_path: String,
    record_count:         u64,
    date_generated:       String,
    compute_duration_ms:  u64,
    qsv_version:          String,
}

impl StatsArgs {
    // this is for deserializing the stats.csv.jsonl file
    fn from_owned_value(value: &OwnedValue) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            arg_input:            value["arg_input"].as_str().unwrap_or_default().to_string(),
            flag_select:          value["flag_select"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            flag_everything:      value["flag_everything"].as_bool().unwrap_or_default(),
            flag_typesonly:       value["flag_typesonly"].as_bool().unwrap_or_default(),
            flag_infer_boolean:   value["flag_infer_boolean"].as_bool().unwrap_or_default(),
            flag_mode:            value["flag_mode"].as_bool().unwrap_or_default(),
            flag_cardinality:     value["flag_cardinality"].as_bool().unwrap_or_default(),
            flag_median:          value["flag_median"].as_bool().unwrap_or_default(),
            flag_mad:             value["flag_mad"].as_bool().unwrap_or_default(),
            flag_quartiles:       value["flag_quartiles"].as_bool().unwrap_or_default(),
            flag_round:           value["flag_round"].as_u64().unwrap_or_default() as u32,
            flag_nulls:           value["flag_nulls"].as_bool().unwrap_or_default(),
            flag_infer_dates:     value["flag_infer_dates"].as_bool().unwrap_or_default(),
            flag_dates_whitelist: value["flag_dates_whitelist"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            flag_prefer_dmy:      value["flag_prefer_dmy"].as_bool().unwrap_or_default(),
            flag_no_headers:      value["flag_no_headers"].as_bool().unwrap_or_default(),
            flag_delimiter:       value["flag_delimiter"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            flag_output_snappy:   value["flag_output_snappy"].as_bool().unwrap_or_default(),
            canonical_input_path: value["canonical_input_path"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            canonical_stats_path: value["canonical_stats_path"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            record_count:         value["record_count"].as_u64().unwrap_or_default(),
            date_generated:       value["date_generated"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            compute_duration_ms:  value["compute_duration_ms"].as_u64().unwrap_or_default(),
            qsv_version:          value["qsv_version"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StatsData {
    pub field:                String,
    // type is a reserved keyword in Rust
    // so we escape it as r#type
    // we need to do this for serde to work
    pub r#type:               String,
    pub is_ascii:             bool,
    pub sum:                  Option<f64>,
    pub min:                  Option<String>,
    pub max:                  Option<String>,
    pub range:                Option<f64>,
    pub sort_order:           Option<String>,
    pub min_length:           Option<usize>,
    pub max_length:           Option<usize>,
    pub sum_length:           Option<usize>,
    pub avg_length:           Option<f64>,
    pub mean:                 Option<f64>,
    pub sem:                  Option<f64>,
    pub stddev:               Option<f64>,
    pub variance:             Option<f64>,
    pub cv:                   Option<f64>,
    pub nullcount:            u64,
    pub max_precision:        Option<u32>,
    pub sparsity:             Option<f64>,
    pub mad:                  Option<f64>,
    pub lower_outer_fence:    Option<f64>,
    pub lower_inner_fence:    Option<f64>,
    pub q1:                   Option<f64>,
    pub q2_median:            Option<f64>,
    pub q3:                   Option<f64>,
    pub iqr:                  Option<f64>,
    pub upper_inner_fence:    Option<f64>,
    pub upper_outer_fence:    Option<f64>,
    pub skewness:             Option<f64>,
    pub cardinality:          u64,
    pub mode:                 Option<String>,
    pub mode_count:           Option<u64>,
    pub mode_occurrences:     Option<u64>,
    pub antimode:             Option<String>,
    pub antimode_count:       Option<u64>,
    pub antimode_occurrences: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JsonTypes {
    Int,
    Float,
    Bool,
    String,
}

// we use this to serialize the StatsData data structure
// to a JSONL file using serde_json
const STATSDATA_TYPES_ARRAY: [JsonTypes; MAX_STAT_COLUMNS] = [
    JsonTypes::String, //field
    JsonTypes::String, //type
    JsonTypes::Bool,   //is_ascii
    JsonTypes::Float,  //sum
    JsonTypes::String, //min
    JsonTypes::String, //max
    JsonTypes::Float,  //range
    JsonTypes::String, //sort_order
    JsonTypes::Int,    //min_length
    JsonTypes::Int,    //max_length
    JsonTypes::Int,    //sum_length
    JsonTypes::Float,  //avg_length
    JsonTypes::Float,  //mean
    JsonTypes::Float,  //sem
    JsonTypes::Float,  //stddev
    JsonTypes::Float,  //variance
    JsonTypes::Float,  //cv
    JsonTypes::Int,    //nullcount
    JsonTypes::Int,    //max_precision
    JsonTypes::Float,  //sparsity
    JsonTypes::Float,  //mad
    JsonTypes::Float,  //lower_outer_fence
    JsonTypes::Float,  //lower_inner_fence
    JsonTypes::Float,  //q1
    JsonTypes::Float,  //q2_median
    JsonTypes::Float,  //q3
    JsonTypes::Float,  //iqr
    JsonTypes::Float,  //upper_inner_fence
    JsonTypes::Float,  //upper_outer_fence
    JsonTypes::Float,  //skewness
    JsonTypes::Int,    //cardinality
    JsonTypes::String, //mode
    JsonTypes::Int,    //mode_count
    JsonTypes::Int,    //mode_occurrences
    JsonTypes::String, //antimode
    JsonTypes::Int,    //antimode_count
    JsonTypes::Int,    //antimode_occurrences
];

static INFER_DATE_FLAGS: OnceLock<SmallVec<[bool; 8]>> = OnceLock::new();
static RECORD_COUNT: OnceLock<u64> = OnceLock::new();

// standard overflow and underflow strings
// for sum, sum_length and avg_length
const OVERFLOW_STRING: &str = "*OVERFLOW*";
const UNDERFLOW_STRING: &str = "*UNDERFLOW*";

// number of milliseconds per day
const MS_IN_DAY: f64 = 86_400_000.0;
const MS_IN_DAY_INT: i64 = 86_400_000;
// number of decimal places when rounding days
// 5 decimal places give us millisecond precision
const DAY_DECIMAL_PLACES: u32 = 5;

// maximum number of output columns
const MAX_STAT_COLUMNS: usize = 37;

// maximum number of antimodes to display
const MAX_ANTIMODES: usize = 10;
// maximum length of antimode string before truncating and appending "..."
const MAX_ANTIMODE_LEN: usize = 100;

// we do this so this is evaluated at compile-time
pub const fn get_stats_data_types() -> [JsonTypes; MAX_STAT_COLUMNS] {
    STATSDATA_TYPES_ARRAY
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;
    if args.flag_typesonly {
        args.flag_everything = false;
        args.flag_mode = false;
        args.flag_cardinality = false;
        args.flag_median = false;
        args.flag_quartiles = false;
        args.flag_mad = false;
    }

    // inferring boolean requires inferring cardinality
    if args.flag_infer_boolean && !args.flag_cardinality {
        args.flag_cardinality = true;
    }

    // check prefer_dmy env var
    args.flag_prefer_dmy = args.flag_prefer_dmy || util::get_envvar_flag("QSV_PREFER_DMY");

    // set stdout output flag
    let stdout_output_flag = args.flag_output.is_none();

    // save the current args, we'll use it to generate
    // the stats.csv.json file
    let mut current_stats_args = StatsArgs {
        arg_input:            format!("{:?}", args.arg_input),
        flag_select:          format!("{:?}", args.flag_select),
        flag_everything:      args.flag_everything,
        flag_typesonly:       args.flag_typesonly,
        flag_infer_boolean:   args.flag_infer_boolean,
        flag_mode:            args.flag_mode,
        flag_cardinality:     args.flag_cardinality,
        flag_median:          args.flag_median,
        flag_mad:             args.flag_mad,
        flag_quartiles:       args.flag_quartiles,
        flag_round:           args.flag_round,
        flag_nulls:           args.flag_nulls,
        flag_infer_dates:     args.flag_infer_dates,
        flag_dates_whitelist: args.flag_dates_whitelist.clone(),
        flag_prefer_dmy:      args.flag_prefer_dmy,
        flag_no_headers:      args.flag_no_headers,
        flag_delimiter:       format!("{:?}", args.flag_delimiter.clone()),
        // when we write to stdout, we don't use snappy compression
        // when we write to a file with the --output option, we use
        // snappy compression if the file ends with ".sz"
        flag_output_snappy:   if stdout_output_flag {
            false
        } else {
            let p = args.flag_output.clone().unwrap();
            p.to_ascii_lowercase().ends_with(".sz")
        },
        canonical_input_path: String::new(),
        canonical_stats_path: String::new(),
        record_count:         0,
        date_generated:       String::new(),
        compute_duration_ms:  0,
        // save the qsv version in the stats.csv.json file
        // so cached stats are automatically invalidated
        // when the qsv version changes
        qsv_version:          env!("CARGO_PKG_VERSION").to_string(),
    };

    // create a temporary file to store the <FILESTEM>.stats.csv file
    let stats_csv_tempfile = if current_stats_args.flag_output_snappy {
        tempfile::Builder::new().suffix(".sz").tempfile()?
    } else {
        NamedTempFile::new()?
    };

    // find the delimiter to use based on the extension of the output file
    // and if we need to snappy compress the output
    let (output_extension, output_delim, snappy) = if let Some(ref output_path) = args.flag_output {
        get_delim_by_extension(Path::new(&output_path), b',')
    } else {
        (String::new(), b',', false)
    };
    let stats_csv_tempfile_fname = format!(
        "{stem}.{prime_ext}{snappy_ext}",
        //safety: we know the tempfile is a valid NamedTempFile, so we can use unwrap
        stem = stats_csv_tempfile.path().to_str().unwrap(),
        prime_ext = output_extension,
        snappy_ext = if snappy { ".sz" } else { "" }
    );

    // we will write the stats to a temp file
    let wconfig = Config::new(Some(stats_csv_tempfile_fname.clone()).as_ref())
        .delimiter(Some(Delimiter(output_delim)));
    let mut wtr = wconfig.writer()?;

    let mut rconfig = args.rconfig();
    if let Some(format_error) = rconfig.format_error {
        return fail_incorrectusage_clierror!("{format_error}");
    }
    let mut stdin_tempfile_path = None;

    if rconfig.is_stdin() {
        // read from stdin and write to a temp file
        log::info!("Reading from stdin");
        let mut stdin_file = NamedTempFile::new()?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        drop(stdin_handle);
        let (_file, tempfile_path) = stdin_file
            .keep()
            .or(Err("Cannot keep temporary file".to_string()))?;
        stdin_tempfile_path = Some(tempfile_path.clone());
        args.arg_input = Some(tempfile_path.to_string_lossy().to_string());
        rconfig.path = Some(tempfile_path);
    } else {
        // check if the input file exists
        if let Some(path) = rconfig.path.clone() {
            if !path.exists() {
                return fail_clierror!("File {:?} does not exist", path.display());
            }
        }
    }

    let mut compute_stats = true;
    let mut create_cache = args.flag_cache_threshold == 1
        || args.flag_stats_jsonl
        || args.flag_cache_threshold.is_negative();

    let mut autoindex_set = false;

    let write_stats_jsonl = args.flag_stats_jsonl;

    if let Some(path) = rconfig.path.clone() {
        //safety: we know the path is a valid PathBuf, so we can use unwrap
        let path_file_stem = path.file_stem().unwrap().to_str().unwrap();
        let stats_file = stats_path(&path, false)?;
        // check if <FILESTEM>.stats.csv file already exists.
        // If it does, check if it was compiled using the same args.
        // However, if the --force flag is set,
        // recompute the stats even if the args are the same.
        if stats_file.exists() && !args.flag_force {
            let stats_args_json_file = stats_file.with_extension("csv.json");
            let existing_stats_args_json_str =
                match fs::read_to_string(stats_args_json_file.clone()) {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!(
                            "Could not read {path_file_stem}.stats.csv.json: {e:?}, recomputing..."
                        );
                        fs::remove_file(&stats_file)?;
                        fs::remove_file(&stats_args_json_file)?;
                        String::new()
                    },
                };

            let time_saved: u64;
            // deserialize the existing stats args json
            let existing_stats_args_json: StatsArgs = {
                let mut json_buffer = existing_stats_args_json_str.into_bytes();
                match simd_json::to_owned_value(&mut json_buffer) {
                    Ok(value) => {
                        // Convert OwnedValue to StatsArgs
                        match StatsArgs::from_owned_value(&value) {
                            Ok(mut stat_args) => {
                                // we init these fields to empty values because we don't want to
                                // compare them when checking if the
                                // args are the same
                                stat_args.canonical_input_path = String::new();
                                stat_args.canonical_stats_path = String::new();
                                stat_args.record_count = 0;
                                stat_args.date_generated = String::new();
                                time_saved = stat_args.compute_duration_ms;
                                stat_args.compute_duration_ms = 0;
                                stat_args
                            },
                            Err(e) => {
                                time_saved = 0;
                                log::warn!(
                                    "Could not deserialize {path_file_stem}.stats.csv.json: \
                                     {e:?}, recomputing..."
                                );
                                fs::remove_file(&stats_file)?;
                                fs::remove_file(&stats_args_json_file)?;
                                StatsArgs::default()
                            },
                        }
                    },
                    Err(e) => {
                        time_saved = 0;
                        log::warn!(
                            "Could not parse {path_file_stem}.stats.csv.json: {e:?}, \
                             recomputing..."
                        );
                        fs::remove_file(&stats_file)?;
                        fs::remove_file(&stats_args_json_file)?;
                        StatsArgs::default()
                    },
                }
            };

            // check if the cached stats are current (ie the stats file is newer than the input
            // file), use the same args or if the --everything flag was set, and
            // all the other non-stats args are equal. If so, we don't need to recompute the stats
            let input_file_modified = fs::metadata(&path)?.modified()?;
            let stats_file_modified = fs::metadata(&stats_file)?.modified()?;
            #[allow(clippy::nonminimal_bool)]
            if stats_file_modified > input_file_modified
                && (existing_stats_args_json == current_stats_args
                    || existing_stats_args_json.flag_everything
                        && existing_stats_args_json.flag_infer_dates
                            == current_stats_args.flag_infer_dates
                        && existing_stats_args_json.flag_dates_whitelist
                            == current_stats_args.flag_dates_whitelist
                        && existing_stats_args_json.flag_prefer_dmy
                            == current_stats_args.flag_prefer_dmy
                        && existing_stats_args_json.flag_no_headers
                            == current_stats_args.flag_no_headers
                        && existing_stats_args_json.flag_dates_whitelist
                            == current_stats_args.flag_dates_whitelist
                        && existing_stats_args_json.flag_delimiter
                            == current_stats_args.flag_delimiter
                        && existing_stats_args_json.flag_nulls == current_stats_args.flag_nulls
                        && existing_stats_args_json.qsv_version == current_stats_args.qsv_version)
            {
                log::info!(
                    "{path_file_stem}.stats.csv already exists and is current. Skipping compute \
                     and using cached stats instead - {time_saved} secs saved...",
                );
                compute_stats = false;
            } else {
                log::info!(
                    "{path_file_stem}.stats.csv already exists, but is older than the input file \
                     or the args have changed, recomputing...",
                );
                fs::remove_file(&stats_file)?;
            }
        }
        if compute_stats {
            let start_time = std::time::Instant::now();

            // we're loading the entire file into memory, we need to check avail mem
            if args.flag_everything
                || args.flag_mode
                || args.flag_cardinality
                || args.flag_median
                || args.flag_quartiles
                || args.flag_mad
            {
                util::mem_file_check(&path, false, args.flag_memcheck)?;
            }

            // check if flag_cache_threshold is a negative number,
            // if so, set the autoindex_size to absolute of the number
            if args.flag_cache_threshold.is_negative() {
                rconfig.autoindex_size = args.flag_cache_threshold.unsigned_abs() as u64;
                autoindex_set = true;
            }

            // we need to count the number of records in the file to calculate sparsity
            // safety: we know util::count_rows() will not return an Err, so we can use unwrap
            let record_count = RECORD_COUNT.get_or_init(|| util::count_rows(&rconfig).unwrap());
            // log::info!("scanning {record_count} records...");

            let (headers, stats) = match rconfig.indexed()? {
                None => args.sequential_stats(&args.flag_dates_whitelist),
                Some(idx) => {
                    let idx_count = idx.count();
                    if let Some(num_jobs) = args.flag_jobs {
                        if num_jobs == 1 {
                            args.sequential_stats(&args.flag_dates_whitelist)
                        } else {
                            args.parallel_stats(&args.flag_dates_whitelist, idx_count)
                        }
                    } else {
                        args.parallel_stats(&args.flag_dates_whitelist, idx_count)
                    }
                },
            }?;

            let stats_sr_vec = args.stats_to_records(stats);
            let mut work_br;

            // vec we use to compute dataset-level fingerprint hash
            let mut stats_br_vec: Vec<csv::ByteRecord> = Vec::with_capacity(stats_sr_vec.len());

            let stats_headers_sr = args.stat_headers();
            wtr.write_record(&stats_headers_sr)?;
            let fields = headers.iter().zip(stats_sr_vec);
            for (i, (header, stat)) in fields.enumerate() {
                let header = if args.flag_no_headers {
                    i.to_string().into_bytes()
                } else {
                    header.to_vec()
                };
                let stat = stat.iter().map(str::as_bytes);
                work_br = vec![&*header]
                    .into_iter()
                    .chain(stat)
                    .collect::<csv::ByteRecord>();
                wtr.write_record(&work_br)?;
                stats_br_vec.push(work_br);
            }

            // Add dataset-level stats as additional rows ====================
            let num_stats_fields = stats_headers_sr.len();
            let mut dataset_stats_br = csv::ByteRecord::with_capacity(128, num_stats_fields);

            // Helper closure to write a dataset stat row
            let mut write_dataset_stat = |name: &[u8], value: u64| -> CliResult<()> {
                dataset_stats_br.clear();
                dataset_stats_br.push_field(name);
                // Fill middle columns with empty strings
                for _ in 2..num_stats_fields {
                    dataset_stats_br.push_field(b"");
                }
                // write qsv__value as last column
                dataset_stats_br.push_field(itoa::Buffer::new().format(value).as_bytes());
                wtr.write_byte_record(&dataset_stats_br)
                    .map_err(std::convert::Into::into)
            };

            // Write qsv__rowcount
            write_dataset_stat(b"qsv__rowcount", *record_count)?;

            // Write qsv__columncount
            let ds_column_count = headers.len() as u64;
            write_dataset_stat(b"qsv__columncount", ds_column_count)?;

            // Write qsv__filesize_bytes
            let ds_filesize_bytes = fs::metadata(&path)?.len();
            write_dataset_stat(b"qsv__filesize_bytes", ds_filesize_bytes)?;

            // Compute hash of stats for data fingerprinting
            let stats_hash = {
                let mut hash_input = Vec::with_capacity(16);

                // First, create a stable representation of the stats
                for record in &stats_br_vec {
                    // Take first 16 columns only
                    for field in record.iter().take(16) {
                        let s = String::from_utf8_lossy(field);
                        // Standardize number format
                        if let Ok(f) = s.parse::<f64>() {
                            hash_input.extend_from_slice(format!("{f:.10}").as_bytes());
                        } else {
                            hash_input.extend_from_slice(field);
                        }
                        hash_input.push(0x1F); // field separator
                    }
                    hash_input.push(b'\n');
                }

                // Add dataset stats
                hash_input.extend_from_slice(
                    format!("{record_count}\x1F{ds_column_count}\x1F{ds_filesize_bytes}\n")
                        .as_bytes(),
                );
                sha256::digest(hash_input.as_slice())
            };

            dataset_stats_br.clear();
            dataset_stats_br.push_field(b"qsv__fingerprint_hash");
            // Fill middle columns with empty strings
            for _ in 2..num_stats_fields {
                dataset_stats_br.push_field(b"");
            }
            // write qsv__value as last column
            dataset_stats_br.push_field(stats_hash.as_bytes());
            wtr.write_byte_record(&dataset_stats_br)?;

            // update the stats args json metadata ===============
            // if the stats run took longer than the cache threshold and the threshold > 0,
            // cache the stats so we don't have to recompute it next time
            current_stats_args.compute_duration_ms = start_time.elapsed().as_millis() as u64;
            create_cache = create_cache
                || current_stats_args.compute_duration_ms > args.flag_cache_threshold as u64;

            // only init these info if we're creating a stats cache
            if create_cache {
                // safety: we know the path is a valid PathBuf, so we can use unwrap
                current_stats_args.canonical_input_path =
                    path.canonicalize()?.to_str().unwrap().to_string();
                current_stats_args.record_count = *record_count;
                current_stats_args.date_generated = chrono::Utc::now().to_rfc3339();
            }
        }
    }

    // ensure create_cache is false if the user specified --cache-threshold 0
    if args.flag_cache_threshold == 0 {
        create_cache = false;
    };

    wtr.flush()?;

    if let Some(pb) = stdin_tempfile_path {
        // remove the temp file we created to store stdin
        std::fs::remove_file(pb)?;
    }

    let currstats_filename = if compute_stats {
        // we computed the stats, use the stats temp file
        stats_csv_tempfile_fname
    } else {
        // we didn't compute the stats, re-use the existing stats file
        // safety: we know the path is a valid PathBuf, so we can use unwrap
        stats_path(rconfig.path.as_ref().unwrap(), false)?
            .to_str()
            .unwrap()
            .to_owned()
    };

    if rconfig.is_stdin() {
        // if we read from stdin, copy the temp stats file to "stdin.stats.csv"
        // safety: we know the path is a valid PathBuf, so we can use unwrap
        let mut stats_pathbuf = stats_path(rconfig.path.as_ref().unwrap(), true)?;
        fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;

        // save the stats args to "stdin.stats.csv.json"
        stats_pathbuf.set_extension("csv.json");
        std::fs::write(
            stats_pathbuf,
            serde_json::to_string_pretty(&current_stats_args)?,
        )?;
    } else if let Some(path) = rconfig.path {
        // if we read from a file, copy the temp stats file to "<FILESTEM>.stats.csv"
        let mut stats_pathbuf = path.clone();
        stats_pathbuf.set_extension("stats.csv");
        // safety: we know the path is a valid PathBuf, so we can use unwrap
        if currstats_filename != stats_pathbuf.to_str().unwrap() {
            // if the stats file is not the same as the input file, copy it
            fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;
        }

        if args.flag_cache_threshold.is_negative() && args.flag_cache_threshold % 10 == -5 {
            // if the cache threshold is a negative number ending in 5,
            // delete both the index file and the stats cache file
            if autoindex_set {
                let index_file = path.with_extension("csv.idx");
                log::debug!("deleting index file: {}", index_file.display());
                if std::fs::remove_file(index_file.clone()).is_err() {
                    // fails silently if it can't remove the index file
                    log::warn!("Could not remove index file: {}", index_file.display());
                }
            }
            create_cache = false;
        }

        if !create_cache {
            // remove the stats cache file
            if fs::remove_file(stats_pathbuf.clone()).is_err() {
                // fails silently if it can't remove the stats file
                log::warn!(
                    "Could not remove stats cache file: {}",
                    stats_pathbuf.display()
                );
            }
        }

        if compute_stats && create_cache {
            // save the stats args to "<FILESTEM>.stats.csv.json"
            // if we computed the stats
            stats_pathbuf.set_extension("csv.json");
            // write empty file first so we can canonicalize it
            std::fs::File::create(stats_pathbuf.clone())?;
            // safety: we know the path is a valid PathBuf, so we can use unwrap
            current_stats_args.canonical_stats_path = stats_pathbuf
                .clone()
                .canonicalize()?
                .to_str()
                .unwrap()
                .to_string();
            std::fs::write(
                stats_pathbuf.clone(),
                serde_json::to_string_pretty(&current_stats_args)?,
            )?;

            // save the stats data to "<FILESTEM>.stats.csv.data.jsonl"
            if write_stats_jsonl {
                stats_pathbuf.set_extension("data.jsonl");
                util::csv_to_jsonl(&currstats_filename, &get_stats_data_types(), &stats_pathbuf)?;
            }
        }
    }

    if stdout_output_flag {
        // if we're outputting to stdout, copy the stats file to stdout
        let currstats = fs::read_to_string(currstats_filename)?;
        io::stdout().write_all(currstats.as_bytes())?;
        io::stdout().flush()?;
    } else if let Some(output) = args.flag_output {
        // if we're outputting to a file, copy the stats file to the output file
        if currstats_filename != output {
            // if the stats file is not the same as the output file, copy it
            fs::copy(currstats_filename, output)?;
        }
    }

    Ok(())
}

impl Args {
    fn sequential_stats(&self, whitelist: &str) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(self.flag_infer_dates, &headers, whitelist)?;

        let stats = self.compute(&sel, rdr.byte_records());
        Ok((headers, stats))
    }

    fn parallel_stats(
        &self,
        whitelist: &str,
        idx_count: u64,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        // N.B. This method doesn't handle the case when the number of records
        // is zero correctly. So we use `sequential_stats` instead.
        if idx_count == 0 {
            return self.sequential_stats(whitelist);
        }

        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(self.flag_infer_dates, &headers, whitelist)?;

        let njobs = util::njobs(self.flag_jobs);
        let chunk_size = util::chunk_size(idx_count as usize, njobs);
        let nchunks = util::num_of_chunks(idx_count as usize, chunk_size);

        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(0);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                // safety: indexed() is safe as we know we have an index file
                // if it does return an Err, you have a bigger problem as the index file was
                // modified WHILE stats is running and you NEED to abort if that
                // happens, however unlikely
                let mut idx = unsafe {
                    args.rconfig()
                        .indexed()
                        .unwrap_unchecked()
                        .unwrap_unchecked()
                };
                idx.seek((i * chunk_size) as u64)
                    .expect("File seek failed.");
                let it = idx.byte_records().take(chunk_size);
                // safety: this will only return an Error if the channel has been disconnected
                unsafe {
                    send.send(args.compute(&sel, it)).unwrap_unchecked();
                }
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap_or_default()))
    }

    fn stats_to_records(&self, stats: Vec<Stats>) -> Vec<csv::StringRecord> {
        let round_places = self.flag_round;
        let infer_boolean = self.flag_infer_boolean;
        let mut records = Vec::with_capacity(stats.len());
        records.extend(repeat(csv::StringRecord::new()).take(stats.len()));
        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let mut results = Vec::with_capacity(stats.len());
        for mut stat in stats {
            let (send, recv) = crossbeam_channel::bounded(0);
            results.push(recv);
            pool.execute(move || {
                // safety: this will only return an Error if the channel has been disconnected
                // which will not happen in this case
                send.send(stat.to_record(round_places, infer_boolean))
                    .unwrap();
            });
        }
        for (i, recv) in results.into_iter().enumerate() {
            // safety: results.len() == records.len() so we know the index is valid
            // and doesn't require a bounds check.
            // The unwrap on recv.recv() is safe as the channel is bounded
            unsafe {
                *records.get_unchecked_mut(i) = recv.recv().unwrap();
            }
        }
        records
    }

    #[inline]
    fn compute<I>(&self, sel: &Selection, it: I) -> Vec<Stats>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let sel_len = sel.len();
        let mut stats = self.new_stats(sel_len);

        // safety: we know INFER_DATE_FLAGS is Some because we called init_date_inference
        let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();

        // so we don't need to get infer_boolean/prefer_dmy from big args struct for each iteration
        // and hopefully the compiler will optimize this and use registers in the hot loop
        let infer_boolean = self.flag_infer_boolean;
        let prefer_dmy = self.flag_prefer_dmy;

        let mut i;
        #[allow(unused_assignments)]
        let mut current_row = csv::ByteRecord::with_capacity(1024, sel_len);
        for row in it {
            i = 0;
            // safety: because we're using iterators and INFER_DATE_FLAGS has the same size,
            // we know we don't need to bounds check
            unsafe {
                current_row = row.unwrap_unchecked();
                for field in sel.select(&current_row) {
                    stats.get_unchecked_mut(i).add(
                        field,
                        *infer_date_flags.get_unchecked(i),
                        infer_boolean,
                        prefer_dmy,
                    );
                    i += 1;
                }
            }
        }
        stats
    }

    #[inline]
    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?.clone();
        let sel = self.rconfig().selection(&headers)?;
        Ok((sel.select(&headers).collect(), sel))
    }

    #[inline]
    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    #[inline]
    fn new_stats(&self, record_len: usize) -> Vec<Stats> {
        let mut stats: Vec<Stats> = Vec::with_capacity(record_len);
        stats.extend(
            repeat(Stats::new(WhichStats {
                include_nulls: self.flag_nulls,
                sum:           !self.flag_typesonly,
                range:         !self.flag_typesonly || self.flag_infer_boolean,
                dist:          !self.flag_typesonly,
                cardinality:   self.flag_everything || self.flag_cardinality,
                median:        !self.flag_everything && self.flag_median && !self.flag_quartiles,
                mad:           self.flag_everything || self.flag_mad,
                quartiles:     self.flag_everything || self.flag_quartiles,
                mode:          self.flag_everything || self.flag_mode,
                typesonly:     self.flag_typesonly,
            }))
            .take(record_len),
        );
        stats
    }

    pub fn stat_headers(&self) -> csv::StringRecord {
        if self.flag_typesonly {
            return csv::StringRecord::from(vec!["field", "type"]);
        }

        // with --everything, we have MAX_STAT_COLUMNS columns at most
        let mut fields = Vec::with_capacity(MAX_STAT_COLUMNS);

        // these are the standard stats columns that are always output
        // the "streaming" stats
        fields.extend_from_slice(&[
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "mean",
            "sem",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
        ]);

        // these are the stats columns that are only output if the user requested them
        let all = self.flag_everything;
        if self.flag_median && !self.flag_quartiles && !all {
            fields.push("median");
        }
        if self.flag_mad || all {
            fields.push("mad");
        }
        if self.flag_quartiles || all {
            fields.extend_from_slice(&[
                "lower_outer_fence",
                "lower_inner_fence",
                "q1",
                "q2_median",
                "q3",
                "iqr",
                "upper_inner_fence",
                "upper_outer_fence",
                "skewness",
            ]);
        }
        if self.flag_cardinality || all {
            fields.push("cardinality");
        }
        if self.flag_mode || all {
            fields.extend_from_slice(&[
                "mode",
                "mode_count",
                "mode_occurrences",
                "antimode",
                "antimode_count",
                "antimode_occurrences",
            ]);
        }

        // we add the qsv__value field at the end for dataset-level stats
        fields.push("qsv__value");

        csv::StringRecord::from(fields)
    }
}

/// returns the path to the stats file
fn stats_path(stats_csv_path: &Path, stdin_flag: bool) -> io::Result<PathBuf> {
    let parent = stats_csv_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?;
    let fstem = stats_csv_path
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let new_fname = if stdin_flag {
        "stdin.stats.csv".to_string()
    } else {
        format!("{}.stats.csv", fstem.to_string_lossy())
    };

    Ok(parent.join(new_fname))
}

fn init_date_inference(
    infer_dates: bool,
    headers: &csv::ByteRecord,
    flag_whitelist: &str,
) -> Result<(), String> {
    if !infer_dates {
        // we're not inferring dates, set INFER_DATE_FLAGS to all false
        INFER_DATE_FLAGS
            .set(SmallVec::from_elem(false, headers.len()))
            .map_err(|_| "Cannot init empty date inference flags".to_string())?;
        return Ok(());
    }

    let infer_date_flags = if flag_whitelist.eq_ignore_ascii_case("all") {
        log::info!("inferring dates for ALL fields");
        SmallVec::from_elem(true, headers.len())
    } else {
        let mut header_str = String::new();
        let whitelist_lower = flag_whitelist.to_lowercase();
        log::info!("inferring dates with date-whitelist: {whitelist_lower}");

        let whitelist: SmallVec<[&str; 8]> = whitelist_lower.split(',').map(str::trim).collect();
        headers
            .iter()
            .map(|header| {
                util::to_lowercase_into(
                    simdutf8::basic::from_utf8(header).unwrap_or_default(),
                    &mut header_str,
                );
                whitelist
                    .iter()
                    .any(|whitelist_item| header_str.contains(whitelist_item))
            })
            .collect()
    };

    INFER_DATE_FLAGS
        .set(infer_date_flags)
        .map_err(|e| format!("Cannot init date inference flags: {e:?}"))?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
struct WhichStats {
    include_nulls: bool,
    sum:           bool,
    range:         bool,
    dist:          bool,
    cardinality:   bool,
    median:        bool,
    mad:           bool,
    quartiles:     bool,
    mode:          bool,
    typesonly:     bool,
}

impl Commute for WhichStats {
    #[inline]
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Stats {
    typ:           FieldType,
    is_ascii:      bool,
    sum:           Option<TypedSum>,
    sum_stotlen:   u64,
    minmax:        Option<TypedMinMax>,
    online:        Option<OnlineStats>,
    nullcount:     u64,
    max_precision: u16,
    modes:         Option<Unsorted<Vec<u8>>>,
    median:        Option<Unsorted<f64>>,
    mad:           Option<Unsorted<f64>>,
    quartiles:     Option<Unsorted<f64>>,
    which:         WhichStats,
}

#[inline]
fn timestamp_ms_to_rfc3339(timestamp: i64, typ: FieldType) -> String {
    let date_val = chrono::DateTime::from_timestamp_millis(timestamp)
        .unwrap_or_default()
        .to_rfc3339();

    // if type = Date, only return the date component
    // do not return the time component
    if typ == TDate {
        return date_val[..10].to_string();
    }
    date_val
}

impl Stats {
    fn new(which: WhichStats) -> Stats {
        let (mut sum, mut minmax, mut online, mut modes, mut median, mut quartiles, mut mad) =
            (None, None, None, None, None, None, None);
        if which.sum {
            sum = Some(TypedSum::default());
        }
        if which.range {
            minmax = Some(TypedMinMax::default());
        }
        if which.dist {
            online = Some(stats::OnlineStats::default());
        }
        if which.mode || which.cardinality {
            modes = Some(stats::Unsorted::default());
        }
        if which.quartiles {
            quartiles = Some(stats::Unsorted::default());
        } else if which.median {
            median = Some(stats::Unsorted::default());
        }
        if which.mad {
            mad = Some(stats::Unsorted::default());
        }
        Stats {
            typ: FieldType::default(),
            is_ascii: true,
            sum,
            sum_stotlen: 0,
            minmax,
            online,
            nullcount: 0,
            max_precision: 0,
            modes,
            median,
            mad,
            quartiles,
            which,
        }
    }

    #[inline]
    fn add(&mut self, sample: &[u8], infer_dates: bool, infer_boolean: bool, prefer_dmy: bool) {
        let (sample_type, timestamp_val) =
            FieldType::from_sample(infer_dates, prefer_dmy, sample, self.typ);
        self.typ.merge(sample_type);

        // we're inferring --typesonly, so don't add samples to compute statistics
        // unless we need to --infer-boolean. In which case, we need --cardinality
        // and --range, so we need to add samples.
        if self.which.typesonly && !infer_boolean {
            return;
        }

        let t = self.typ;
        if let Some(v) = self.sum.as_mut() {
            v.add(t, sample);
        };
        if let Some(v) = self.minmax.as_mut() {
            if let Some(ts_val) = timestamp_val {
                v.add(t, itoa::Buffer::new().format(ts_val).as_bytes());
            } else {
                v.add(t, sample);
            }
        };
        if let Some(v) = self.modes.as_mut() {
            v.add(sample.to_vec());
        };
        if sample_type == TNull {
            self.nullcount += 1;
        }
        match t {
            TString => {
                self.is_ascii &= sample.is_ascii();
            },
            TFloat | TInteger => {
                if sample_type == TNull {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                } else {
                    // safety: we know the sample is a valid f64, so we can use unwrap
                    let n = fast_float2::parse(sample).unwrap();
                    if let Some(v) = self.median.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.mad.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.quartiles.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.online.as_mut() {
                        v.add(&n);
                    }
                    if t == TFloat {
                        let mut ryu_buffer = ryu::Buffer::new();
                        // safety: we know that n is a valid f64
                        // so there will always be a fraction part, even if it's 0
                        let fractpart = ryu_buffer.format_finite(n).split('.').next_back().unwrap();
                        self.max_precision = std::cmp::max(
                            self.max_precision,
                            (if *fractpart == *"0" {
                                0
                            } else {
                                fractpart.len()
                            }) as u16,
                        );
                    }
                }
            },
            TNull => {
                if self.which.include_nulls {
                    if let Some(v) = self.online.as_mut() {
                        v.add_null();
                    };
                }
            },
            TDateTime | TDate => {
                if sample_type == TNull {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                // if ts_val.is_some() then we successfully inferred a date from the sample
                // and the timestamp value is not None
                } else if let Some(ts_val) = timestamp_val {
                    // calculate date statistics by adding date samples as timestamps to
                    // millisecond precision.
                    #[allow(clippy::cast_precision_loss)]
                    let n = ts_val as f64;
                    if let Some(v) = self.median.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.mad.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.quartiles.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.online.as_mut() {
                        v.add(&n);
                    }
                }
            },
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_record(&mut self, round_places: u32, infer_boolean: bool) -> csv::StringRecord {
        // we're doing typesonly and not inferring boolean, just return the type
        if self.which.typesonly && !infer_boolean {
            return csv::StringRecord::from(vec![self.typ.to_string()]);
        }

        let typ = self.typ;
        // prealloc memory for performance
        // we have MAX_STAT_COLUMNS columns at most with --everything
        let mut pieces = Vec::with_capacity(MAX_STAT_COLUMNS);

        let empty = String::new;

        // min/max/range/sort_order
        // we do this first as we want to get the sort_order, so we can skip sorting if not
        // required. We also need to do this before --infer-boolean because we need to know
        // the min/max values to determine if the range is equal to the supported boolean
        // ranges (0/1, f/t, n/y)
        let mut minmax_range_sortorder_pieces = Vec::with_capacity(4);
        let mut minval_lower: char = '\0';
        let mut maxval_lower: char = '\0';
        let mut column_sorted = false;
        if let Some(mm) = self
            .minmax
            .as_ref()
            .and_then(|mm| mm.show(typ, round_places))
        {
            // get first character of min/max values
            minval_lower = mm.0.chars().next().unwrap_or_default().to_ascii_lowercase();
            maxval_lower = mm.1.chars().next().unwrap_or_default().to_ascii_lowercase();
            if mm.3.starts_with("Ascending") {
                column_sorted = true;
            }
            minmax_range_sortorder_pieces.extend_from_slice(&[mm.0, mm.1, mm.2, mm.3]);
        } else {
            minmax_range_sortorder_pieces.extend_from_slice(&[empty(), empty(), empty(), empty()]);
        }

        // modes/antimodes & cardinality
        // we do this second because we can use the sort order with cardinality, to skip sorting
        // if its not required. This makes not only cardinality computation faster, it also makes
        // modes/antimodes computation faster.
        // We also need to know the cardinality to --infer-boolean should that be enabled
        let mut cardinality = 0;
        let mut mc_pieces = Vec::with_capacity(7);
        match self.modes.as_mut() {
            None => {
                if self.which.cardinality {
                    mc_pieces.push(empty());
                }
                if self.which.mode {
                    mc_pieces.extend_from_slice(&[empty(), empty(), empty(), empty()]);
                }
            },
            Some(ref mut v) => {
                if self.which.cardinality {
                    cardinality = v.cardinality(column_sorted, 1);
                    mc_pieces.push(itoa::Buffer::new().format(cardinality).to_owned());
                }
                if self.which.mode {
                    // mode/s
                    let (modes_result, modes_count, mode_occurrences) = v.modes();
                    let modes_list = modes_result
                        .iter()
                        .map(|c| String::from_utf8_lossy(c))
                        .join(",");
                    mc_pieces.extend_from_slice(&[
                        modes_list,
                        modes_count.to_string(),
                        mode_occurrences.to_string(),
                    ]);

                    // antimode/s
                    if mode_occurrences == 0 {
                        // all the values are unique
                        // so instead of returning everything, just say *ALL
                        mc_pieces.extend_from_slice(&[
                            "*ALL".to_string(),
                            "0".to_string(),
                            "1".to_string(),
                        ]);
                    } else {
                        let (antimodes_result, antimodes_count, antimode_occurrences) =
                            v.antimodes();
                        let mut antimodes_list = String::new();

                        // We only store the first 10 antimodes
                        // so if antimodes_count > 10, add the "*PREVIEW: " prefix
                        if antimodes_count > MAX_ANTIMODES {
                            antimodes_list.push_str("*PREVIEW: ");
                        }

                        let antimodes_vals = &antimodes_result
                            .iter()
                            .map(|c| String::from_utf8_lossy(c))
                            .join(",");
                        if antimodes_vals.starts_with(',') {
                            antimodes_list.push_str("NULL");
                        }
                        antimodes_list.push_str(antimodes_vals);

                        // and truncate at 100 characters with an ellipsis
                        if antimodes_list.len() > MAX_ANTIMODE_LEN {
                            util::utf8_truncate(&mut antimodes_list, MAX_ANTIMODE_LEN + 1);
                            antimodes_list.push_str("...");
                        }

                        mc_pieces.extend_from_slice(&[
                            antimodes_list,
                            antimodes_count.to_string(),
                            antimode_occurrences.to_string(),
                        ]);
                    }
                }
            },
        }

        // type
        if cardinality == 2 && infer_boolean {
            // if cardinality is 2, it's a boolean if its values' first character are 0/1, f/t, n/y
            if (minval_lower == '0' && maxval_lower == '1')
                || (minval_lower == 'f' && maxval_lower == 't')
                || (minval_lower == 'n' && maxval_lower == 'y')
            {
                pieces.push("Boolean".to_string());
            } else {
                pieces.push(typ.to_string());
            }
        } else {
            pieces.push(typ.to_string());
        }

        // we're doing --typesonly with --infer-boolean, we don't need to calculate anything else
        if self.which.typesonly && infer_boolean {
            return csv::StringRecord::from(pieces);
        }

        // is_ascii
        if typ == FieldType::TString {
            pieces.push(self.is_ascii.to_string());
        } else {
            pieces.push(empty());
        }

        // sum
        let stotlen =
            if let Some((stotlen_work, sum)) = self.sum.as_ref().and_then(|sum| sum.show(typ)) {
                if typ == FieldType::TFloat {
                    if let Ok(f64_val) = fast_float2::parse::<f64, &[u8]>(sum.as_bytes()) {
                        pieces.push(util::round_num(f64_val, round_places));
                    } else {
                        pieces.push(format!("ERROR: Cannot convert {sum} to a float."));
                    }
                } else {
                    pieces.push(sum);
                }
                stotlen_work
            } else {
                pieces.push(empty());
                0
            };

        // min/max/range/sort_order
        // actually append it here - to preserve legacy ordering of columns
        pieces.extend_from_slice(&minmax_range_sortorder_pieces);

        // min/max/sum/avg length
        if typ == FieldType::TDate || typ == FieldType::TDateTime {
            // returning min/max length for dates doesn't make sense
            // especially since we convert the date stats to rfc3339 format
            pieces.extend_from_slice(&[empty(), empty(), empty(), empty()]);
        } else if let Some(mm) = self.minmax.as_ref().and_then(TypedMinMax::len_range) {
            pieces.extend_from_slice(&[mm.0, mm.1]);
            // we have a sum_length
            if stotlen > 0 {
                if stotlen < u64::MAX {
                    // so we can compute avg_length
                    pieces.push(itoa::Buffer::new().format(stotlen).to_owned());
                    #[allow(clippy::cast_precision_loss)]
                    pieces.push(util::round_num(
                        stotlen as f64 / *RECORD_COUNT.get().unwrap_or(&1) as f64,
                        4,
                    ));
                } else {
                    // however, we saturated the sum, it means we had an overflow
                    // so we return OVERFLOW_STRING for sum and avg length
                    pieces.extend_from_slice(&[
                        OVERFLOW_STRING.to_string(),
                        OVERFLOW_STRING.to_string(),
                    ]);
                }
            } else {
                pieces.extend_from_slice(&[empty(), empty()]);
            }
        } else {
            pieces.extend_from_slice(&[empty(), empty(), empty(), empty()]);
        }

        // mean, sem, stddev, variance & cv
        if typ == TString || typ == TNull {
            pieces.extend_from_slice(&[empty(), empty(), empty(), empty(), empty()]);
        } else if let Some(ref v) = self.online {
            let std_dev = v.stddev();
            #[allow(clippy::cast_precision_loss)]
            let sem = std_dev / (v.len() as f64).sqrt();
            let mean = v.mean();
            let cv = (std_dev / mean) * 100_f64;
            if self.typ == TFloat || self.typ == TInteger {
                pieces.extend_from_slice(&[
                    util::round_num(mean, round_places),
                    util::round_num(sem, round_places),
                    util::round_num(std_dev, round_places),
                    util::round_num(v.variance(), round_places),
                    util::round_num(cv, round_places),
                ]);
            } else {
                // by the time we get here, the type is a TDateTime or TDate
                pieces.push(timestamp_ms_to_rfc3339(mean as i64, typ));
                // instead of returning sem, stdev & variance as timestamps, return it in
                // days as its more human readable and practical for real-world use cases
                // Round to at least 5 decimal places, so we have millisecond precision
                pieces.push(util::round_num(
                    sem / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    std_dev / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    v.variance() / (MS_IN_DAY * MS_IN_DAY),
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(cv, round_places));
            }
        } else {
            pieces.extend_from_slice(&[empty(), empty(), empty(), empty(), empty()]);
        }

        // nullcount
        pieces.push(itoa::Buffer::new().format(self.nullcount).to_owned());

        // max precision
        if typ == TFloat {
            pieces.push(self.max_precision.to_string());
        } else {
            pieces.push(empty());
        }

        // sparsity
        #[allow(clippy::cast_precision_loss)]
        let sparsity: f64 = self.nullcount as f64 / *RECORD_COUNT.get().unwrap_or(&1) as f64;
        pieces.push(util::round_num(sparsity, round_places));

        // median
        let mut existing_median = None;
        if let Some(v) = self.median.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else {
                existing_median = v.median();
                existing_median
            }
        }) {
            if typ == TDateTime || typ == TDate {
                pieces.push(timestamp_ms_to_rfc3339(v as i64, typ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.median {
            pieces.push(empty());
        }

        // median absolute deviation (MAD)
        if let Some(v) = self.mad.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else {
                v.mad(existing_median)
            }
        }) {
            if typ == TDateTime || typ == TDate {
                // like stddev, return MAD in days
                pieces.push(util::round_num(
                    v / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.mad {
            pieces.push(empty());
        }

        // quartiles
        match self.quartiles.as_mut().and_then(|v| match typ {
            TInteger | TFloat | TDate | TDateTime => v.quartiles(),
            _ => None,
        }) {
            None => {
                if self.which.quartiles {
                    pieces.extend_from_slice(&[
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                    ]);
                }
            },
            Some((q1, q2, q3)) => {
                let iqr = q3 - q1;

                // use fused multiply add (mul_add) when possible
                // fused mul_add is more accurate & is more performant if the
                // target architecture has a dedicated `fma` CPU instruction
                // https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add

                // lower_outer_fence = "q1 - (3.0 * iqr)"
                let lof = 3.0f64.mul_add(-iqr, q1);
                // lower_inner_fence = "q1 - (1.5 * iqr)"
                let lif = 1.5f64.mul_add(-iqr, q1);

                // upper inner fence = "q3 + (1.5 * iqr)"
                let uif = 1.5_f64.mul_add(iqr, q3);
                // upper_outer_fence = "q3 + (3.0 * iqr)"
                let uof = 3.0_f64.mul_add(iqr, q3);

                // calculate skewness using Quantile-based measures
                // https://en.wikipedia.org/wiki/Skewness#Quantile-based_measures
                // https://blogs.sas.com/content/iml/2017/07/19/quantile-skewness.html
                // quantile skewness = ((q3 - q2) - (q2 - q1)) / iqr;
                // which is also (q3 - (2.0 * q2) + q1) / iqr
                // which in turn, is the basis of the fused multiply add version below
                let skewness = (2.0f64.mul_add(-q2, q3) + q1) / iqr;

                if typ == TDateTime || typ == TDate {
                    // casting from f64 to i64 is OK, per
                    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
                    // as values larger/smaller than what i64 can handle will automatically
                    // saturate to i64 max/min values.
                    pieces.extend_from_slice(&[
                        timestamp_ms_to_rfc3339(lof as i64, typ),
                        timestamp_ms_to_rfc3339(lif as i64, typ),
                        timestamp_ms_to_rfc3339(q1 as i64, typ),
                        timestamp_ms_to_rfc3339(q2 as i64, typ), // q2 = median
                        timestamp_ms_to_rfc3339(q3 as i64, typ),
                        // return iqr in days - there are 86,400,000 ms in a day
                        util::round_num(
                            (q3 - q1) / MS_IN_DAY,
                            u32::max(round_places, DAY_DECIMAL_PLACES),
                        ),
                        timestamp_ms_to_rfc3339(uif as i64, typ),
                        timestamp_ms_to_rfc3339(uof as i64, typ),
                    ]);
                } else {
                    pieces.extend_from_slice(&[
                        util::round_num(lof, round_places),
                        util::round_num(lif, round_places),
                        util::round_num(q1, round_places),
                        util::round_num(q2, round_places), // q2 = median
                        util::round_num(q3, round_places),
                        util::round_num(iqr, round_places),
                        util::round_num(uif, round_places),
                        util::round_num(uof, round_places),
                    ]);
                }
                pieces.push(util::round_num(skewness, round_places));
            },
        }

        // mode/modes/antimodes & cardinality
        // append it here to preserve legacy ordering of columns
        pieces.extend_from_slice(&mc_pieces);

        // add an empty field for qsv__value
        pieces.push(empty());

        csv::StringRecord::from(pieces)
    }
}

impl Commute for Stats {
    #[inline]
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.is_ascii &= other.is_ascii;
        self.sum.merge(other.sum);
        self.sum_stotlen = self.sum_stotlen.saturating_add(other.sum_stotlen);
        self.minmax.merge(other.minmax);
        self.online.merge(other.online);
        self.nullcount += other.nullcount;
        self.max_precision = std::cmp::max(self.max_precision, other.max_precision);
        self.modes.merge(other.modes);
        self.median.merge(other.median);
        self.quartiles.merge(other.quartiles);
        self.which.merge(other.which);
    }
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
enum FieldType {
    // The default - TNull, is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    #[default]
    TNull,
    TString,
    TFloat,
    TInteger,
    TDate,
    TDateTime,
}

impl FieldType {
    /// infer data type from a given sample & current type inference
    /// infer_dates signals if date inference should be attempted
    /// returns the inferred type and if infer_dates is true,
    /// the date in ms since the epoch if the type is a date or datetime
    /// otherwise, None
    #[inline]
    pub fn from_sample(
        infer_dates: bool,
        prefer_dmy: bool,
        sample: &[u8],
        current_type: FieldType,
    ) -> (FieldType, Option<i64>) {
        // faster than sample.len() == 0 or sample.is_empty() per microbenchmarks
        if b"" == sample {
            return (FieldType::TNull, None);
        }

        // no need to do type checking if current_type is already a String
        if current_type == FieldType::TString {
            return (FieldType::TString, None);
        }

        if let Ok(s) = simdutf8::basic::from_utf8(sample) {
            // Check for integer, with leading zero check for strings like zip codes
            if atoi_simd::parse::<i64>(s.as_bytes()).is_ok() {
                if s == "0" || !s.starts_with('0') {
                    return (FieldType::TInteger, None);
                }
                // If starts with '0' but not "0", it's a string
                return (FieldType::TString, None);
            }

            // Check for float
            if fast_float2::parse::<f64, &[u8]>(s.as_bytes()).is_ok() {
                return (FieldType::TFloat, None);
            }

            // Check for date/datetime if infer_dates is true
            if infer_dates {
                if let Ok(parsed_date) = parse_with_preference(s, prefer_dmy) {
                    let ts_val = parsed_date.timestamp_millis();
                    // check if the tstamp (Unix Epoch format) modulo by 86400000 (ms in a day)
                    // if the remainder is 0, we says its Date type candidate, otherwise, its
                    // DateTime. this is a performance optimization to avoid
                    // parsing the date to rfc3339 and then checking if the time
                    // component is T00:00:00, as was done previously
                    // see https://stackoverflow.com/questions/40948290/is-it-safe-to-use-modulo-operator-with-unix-epoch-timestamp
                    if ts_val % MS_IN_DAY_INT == 0 {
                        return (FieldType::TDate, Some(ts_val));
                    }
                    return (FieldType::TDateTime, Some(ts_val));
                }
            }
        } else {
            // If not valid UTF-8, it's a binary string, return as TString
            return (FieldType::TString, None);
        };

        // Default to TString if none of the above conditions are met
        (FieldType::TString, None)
    }
}

impl Commute for FieldType {
    #[inline]
    #[allow(clippy::match_same_arms)]
    // we allow match_same_arms because we want are optimizing for
    // performance and not readability, as match arms are evaluated in order
    // so we want to put the most common cases first
    fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TString, TString) => TString,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // date data types
            (TDate, TDate) => TDate,
            (TDateTime | TDate, TDateTime) | (TDateTime, TDate) => TDateTime,
            // anything else is a String
            (_, _) => TString,
        };
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

/// `TypedSum` keeps a rolling sum of the data seen.
/// It sums integers until it sees a float, at which point it sums floats.
/// It also counts the total length of strings.
#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
struct TypedSum {
    integer: i64,
    stotlen: u64, // sum of the total length of strings
    float:   Option<f64>,
}

impl TypedSum {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        if b"" == sample {
            return;
        }
        #[allow(clippy::cast_precision_loss)]
        match typ {
            TFloat => {
                self.stotlen = self.stotlen.saturating_add(sample.len() as u64);
                if let Ok(float_sample) = fast_float2::parse::<f64, &[u8]>(sample) {
                    if let Some(ref mut f) = self.float {
                        *f += float_sample;
                    } else {
                        self.float = Some((self.integer as f64) + float_sample);
                    }
                }
            },
            TInteger => {
                self.stotlen = self.stotlen.saturating_add(sample.len() as u64);
                if let Some(ref mut float) = self.float {
                    // safety: we know that the sample is a valid f64
                    *float += fast_float2::parse::<f64, &[u8]>(sample).unwrap();
                } else {
                    // so we don't panic on overflow/underflow, use saturating_add
                    self.integer = self
                        .integer
                        // safety: we know that the sample is a valid i64
                        .saturating_add(atoi_simd::parse::<i64>(sample).unwrap());
                }
            },
            TString => {
                self.stotlen = self.stotlen.saturating_add(sample.len() as u64);
            },
            // we don't need to do anything for TNull, TDate or TDateTime
            // as they don't have a sum
            _ => {},
        }
    }

    fn show(&self, typ: FieldType) -> Option<(u64, String)> {
        match typ {
            TNull | TDate | TDateTime => None,
            TInteger => {
                match self.integer {
                    // with saturating_add, if this is equal to i64::MAX or i64::MIN
                    // we overflowed/underflowed
                    i64::MAX => Some((self.stotlen, OVERFLOW_STRING.to_string())),
                    i64::MIN => Some((self.stotlen, UNDERFLOW_STRING.to_string())),
                    _ => Some((
                        self.stotlen,
                        itoa::Buffer::new().format(self.integer).to_owned(),
                    )),
                }
            },
            TFloat => Some((
                self.stotlen,
                ryu::Buffer::new()
                    .format(self.float.unwrap_or(0.0))
                    .to_owned(),
            )),
            TString => Some((self.stotlen, String::new())),
        }
    }
}

impl Commute for TypedSum {
    #[inline]
    fn merge(&mut self, other: TypedSum) {
        #[allow(clippy::cast_precision_loss)]
        match (self.float, other.float) {
            (Some(f1), Some(f2)) => self.float = Some(f1 + f2),
            (Some(f1), None) => self.float = Some(f1 + (other.integer as f64)),
            (None, Some(f2)) => self.float = Some((self.integer as f64) + f2),
            (None, None) => self.integer = self.integer.saturating_add(other.integer),
        }
        self.stotlen = self.stotlen.saturating_add(other.stotlen);
    }
}

/// `TypedMinMax` keeps track of minimum/maximum/range/sort_order values for each possible type
/// where min/max/range/sort_order makes sense.
#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
struct TypedMinMax {
    strings:  MinMax<Vec<u8>>,
    str_len:  MinMax<usize>,
    integers: MinMax<i64>,
    floats:   MinMax<f64>,
    dates:    MinMax<i64>,
}

impl TypedMinMax {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        let sample_len = sample.len();
        self.str_len.add(sample_len);
        if sample_len == 0 {
            return;
        }
        self.strings.add(sample.to_vec());
        // safety: we can use unwrap below since we know the data type of the sample
        match typ {
            TString | TNull => {},
            TFloat => {
                let n = fast_float2::parse::<f64, &[u8]>(sample).unwrap();

                self.floats.add(n);
                self.integers.add(n as i64);
            },
            TInteger => {
                let n = atoi_simd::parse::<i64>(sample).unwrap();
                self.integers.add(n);
                #[allow(clippy::cast_precision_loss)]
                self.floats.add(n as f64);
            },
            // it must be a TDate or TDateTime
            // we use "_" here instead of "TDate | TDateTime" for the match to avoid
            // the overhead of matching on the OR value, however minor
            _ => {
                let n = atoi_simd::parse::<i64>(sample).unwrap();
                self.dates.add(n);
            },
        }
    }

    fn len_range(&self) -> Option<(String, String)> {
        if let (Some(min), Some(max)) = (self.str_len.min(), self.str_len.max()) {
            Some((
                itoa::Buffer::new().format(*min).to_owned(),
                itoa::Buffer::new().format(*max).to_owned(),
            ))
        } else {
            None
        }
    }

    #[inline]
    fn show(&self, typ: FieldType, round_places: u32) -> Option<(String, String, String, String)> {
        match typ {
            TNull => None,
            TString => {
                if let (Some(min), Some(max), sort_order) = (
                    self.strings.min(),
                    self.strings.max(),
                    self.strings.sort_order(),
                ) {
                    let min = String::from_utf8_lossy(min).to_string();
                    let max = String::from_utf8_lossy(max).to_string();
                    let sort_order = sort_order.to_string();
                    Some((min, max, String::new(), sort_order))
                } else {
                    None
                }
            },
            TInteger => {
                if let (Some(min), Some(max), sort_order) = (
                    self.integers.min(),
                    self.integers.max(),
                    self.integers.sort_order(),
                ) {
                    Some((
                        itoa::Buffer::new().format(*min).to_owned(),
                        itoa::Buffer::new().format(*max).to_owned(),
                        itoa::Buffer::new().format(*max - *min).to_owned(),
                        sort_order.to_string(),
                    ))
                } else {
                    None
                }
            },
            TFloat => {
                if let (Some(min), Some(max), sort_order) = (
                    self.floats.min(),
                    self.floats.max(),
                    self.floats.sort_order(),
                ) {
                    Some((
                        ryu::Buffer::new().format(*min).to_owned(),
                        ryu::Buffer::new().format(*max).to_owned(),
                        util::round_num(*max - *min, round_places),
                        sort_order.to_string(),
                    ))
                } else {
                    None
                }
            },
            TDateTime | TDate => {
                if let (Some(min), Some(max), sort_order) =
                    (self.dates.min(), self.dates.max(), self.dates.sort_order())
                {
                    Some((
                        timestamp_ms_to_rfc3339(*min, typ),
                        timestamp_ms_to_rfc3339(*max, typ),
                        // return in days, not timestamp in milliseconds
                        #[allow(clippy::cast_precision_loss)]
                        util::round_num(
                            (*max - *min) as f64 / MS_IN_DAY,
                            u32::max(round_places, 5),
                        ),
                        sort_order.to_string(),
                    ))
                } else {
                    None
                }
            },
        }
    }
}

impl Commute for TypedMinMax {
    #[inline]
    fn merge(&mut self, other: TypedMinMax) {
        self.strings.merge(other.strings);
        self.str_len.merge(other.str_len);
        self.integers.merge(other.integers);
        self.floats.merge(other.floats);
        self.dates.merge(other.dates);
    }
}
