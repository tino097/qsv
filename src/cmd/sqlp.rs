static USAGE: &str = r#"
Run blazing-fast Polars SQL queries against several CSVs - replete with joins, aggregations,
grouping, table functions, sorting, and more - working on larger than memory CSV files.

Polars SQL is a SQL dialect, converting SQL queries to fast Polars LazyFrame expressions
(see https://docs.pola.rs/user-guide/sql/intro/).

For a list of SQL functions and keywords supported by Polars SQL, see
https://docs.pola.rs/py-polars/html/reference/sql/index.html though be aware that it's for
the Python version of Polars, so there will be some minor syntax differences.

Returns the shape of the query result (number of rows, number of columns) to stderr.

Example queries:

   qsv sqlp data.csv 'select * from data where col1 > 10 order by all desc limit 20'

   qsv sqlp data.csv 'select col1, col2 as friendlyname from data' --format parquet --output data.parquet

  # enclose column names with spaces in double quotes
   qsv sqlp data.csv 'select "col 1", "col 2" from data'

   qsv sqlp data.csv data2.csv 'select * from data join data2 on data.colname = data2.colname'

   qsv sqlp data.csv data2.csv 'SELECT col1 FROM data WHERE col1 IN (SELECT col2 FROM data2)'

  # Use dollar-quoting to avoid escaping reserved characters in literals.
  https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING 
   qsv sqlp data.csv "SELECT * FROM data WHERE col1 = $$O'Reilly$$"
   qsv sqlp data.csv 'SELECT * FROM data WHERE col1 = $SomeTag$Diane's horse "Twinkle"$SomeTag$'

  # Unions and Joins are supported.
   qsv sqlp data1.csv data2.csv 'SELECT * FROM data1 UNION ALL BY NAME SELECT * FROM data2'

   qsv sqlp tbl_a.csv tbl_b.csv tbl_c.csv "SELECT * FROM tbl_a \
     RIGHT ANTI JOIN tbl_b USING (b) \
     LEFT SEMI JOIN tbl_c USING (c)"

  # use "_t_N" aliases to refer to input files, where N is the 1-based index
  # of the input file/s. For example, _t_1 refers to the first input file, _t_2
  # refers to the second input file, and so on.
   qsv sqlp data.csv data2.csv 'select * from _t_1 join _t_2 on _t_1.colname = _t_2.colname'

   qsv sqlp data.csv 'SELECT col1, count(*) AS cnt FROM data GROUP BY col1 ORDER BY cnt DESC, col1 ASC'

   qsv sqlp data.csv "select lower(col1), substr(col2, 2, 4) from data WHERE starts_with(col1, 'foo')"

   qsv sqlp data.csv "select COALESCE(NULLIF(col2, ''), 'foo') from data"

   qsv sqlp tbl1.csv "SELECT x FROM tbl1 WHERE x IN (SELECT y FROM tbl1)"

  # Natural Joins are supported too! (https://www.w3resource.com/sql/joins/natural-join.php)
   qsv sqlp data1.csv data2.csv data3.csv \
    "SELECT COLUMNS('^[^:]+$') FROM data1 NATURAL JOIN data2 NATURAL JOIN data3 ORDER BY COMPANY_ID",

  # Use a SQL script to run a long, complex SQL query or to run SEVERAL SQL queries.
  # When running several queries, each query needs to be separated by a semicolon,
  # the last query will be returned as the result.
  # Typically, earlier queries are used to create tables that can be used in later queries.
  # Note that scripts support single-line comments starting with '--' so feel free to
  # add comments to your script.
  # In long, complex scripts that produce multiple temporary tables, note that you can use
  # `truncate table <table_name>;` to free up memory used by temporary tables. Otherwise,
  # the memory used by the temporary tables won't be freed until the script finishes.
  # See test_sqlp/sqlp_boston311_sql_script() for an example.
   qsv sqlp data.csv data2.csv data3.csv data4.csv script.sql --format json --output data.json

  # use Common Table Expressions (CTEs) using WITH to simplify complex queries
   qsv sqlp people.csv "WITH millennials AS (SELECT * FROM people WHERE age >= 25 and age <= 40) \
     SELECT * FROM millennials WHERE STARTS_WITH(name,'C')"

  # CASE statement
   qsv sqlp data.csv "select CASE WHEN col1 > 10 THEN 'foo' WHEN col1 > 5 THEN 'bar' ELSE 'baz' END from data"
   qsv sqlp data.csv "select CASE col*5 WHEN 10 THEN 'foo' WHEN 5 THEN 'bar' ELSE 'baz' END from _t_1"

  # spaceship operator: "<=>" (three-way comparison operator)
  #  returns -1 if left < right, 0 if left == right, 1 if left > right
  # https://en.wikipedia.org/wiki/Three-way_comparison#Spaceship_operator
   qsv sqlp data.csv data2.csv "select data.c2 <=> data2.c2 from data join data2 on data.c1 = data2.c1"

  # support ^@ ("starts with"), and ~~ (like) ,~~* (ilike),!~~ (not like),!~~* (not ilike) operators
    qsv sqlp data.csv "select * from data WHERE col1 ^@ 'foo'"
    qsv sqlp data.csv "select c1 ^@ 'a' AS c1_starts_with_a from data"
    qsv sqlp data.csv "select c1 ~~* '%B' AS c1_ends_with_b_caseinsensitive from data"

  # support SELECT * ILIKE wildcard syntax
    # select all columns from customers where the column contains 'a' followed by an 'e'
    # with any characters (or no characters), in between, case-insensitive
    # if customers.csv has columns LastName, FirstName, Address, City, State, Zip
    # this query will return all columns for all rows except the columns that don't
    # contain 'a' followed by an 'e' - i.e. except City and Zip
    qsv sqlp customers.csv "SELECT * ILIKE '%a%e%' FROM customers ORDER BY LastName, FirstName"

  # regex operators: "~" (contains pattern, case-sensitive); "~*" (contains pattern, case-insensitive)
  #   "!~" (does not contain pattern, case-sensitive); "!~*" (does not contain pattern, case-insensitive)
   qsv sqlp data.csv "select * from data WHERE col1 ~ '^foo' AND col2 > 10"
   qsv sqlp data.csv "select * from data WHERE col1 !~* 'bar$' AND col2 > 10"

  # regexp_like function: regexp_like(<string>, <pattern>, <optional flags>)
  # returns true if <string> matches <pattern>, false otherwise
  #   <optional flags> can be one or more of the following:
  #   'c' (case-sensitive - default), 'i' (case-insensitive), 'm' (multiline)
   qsv sqlp data.csv "select * from data WHERE regexp_like(col1, '^foo') AND col2 > 10"
  # case-insensitive regexp_like
   qsv sqlp data.csv "select * from data WHERE regexp_like(col1, '^foo', 'i') AND col2 > 10"

  # regexp match using a literal pattern
   qsv sqlp data.csv "select idx,val from data WHERE val regexp '^foo'"

  # regexp match using patterns from another column
   qsv sqlp data.csv "select idx,val from data WHERE val regexp pattern_col"

  # use Parquet, JSONL and Arrow files in SQL queries
   qsv sqlp data.csv "select * from data join read_parquet('data2.parquet') as t2 ON data.c1 = t2.c1"
   qsv sqlp data.csv "select * from data join read_ndjson('data2.jsonl') as t2 on data.c1 = t2.c1"
   qsv sqlp data.csv "select * from data join read_ipc('data2.arrow') as t2 ON data.c1 = t2.c1"
   qsv sqlp SKIP_INPUT "select * from read_parquet('data.parquet') order by col1 desc limit 100"
   qsv sqlp SKIP_INPUT "select * from read_ndjson('data.jsonl') as t1 join read_ipc('data.arrow') as t2 on t1.c1 = t2.c1" 

  # you can also directly load CSVs using the Polars read_csv() SQL function. This is useful when
  # you want to bypass the regular CSV parser (with SKIP_INPUT) and use Polars' multithreaded,
  # mem-mapped CSV parser instead - making for dramatically faster queries at the cost of CSV parser
  # configurability (i.e. limited to comma delimiter, no CSV comments, etc.).
   qsv sqlp SKIP_INPUT "select * from read_csv('data.csv') order by col1 desc limit 100"

   # note that you can also use read_csv() to read compressed files directly
   # gzip, zstd and zlib automatic decompression are supported
   qsv sqlp SKIP_INPUT "select * from read_csv('data.csv.gz')"
   qsv sqlp SKIP_INPUT "select * from read_csv('data.csv.zst')"
   qsv sqlp SKIP_INPUT "select * from read_csv('data.csv.zlib')"

  Note that sqlp will automatically use this "fast path" read_csv() optimization when there 
  is only one input CSV file, no CSV parsing options are used, its not a SQL script and the
  `--no-optimizations` flag is not set.

  # apart from using Polar's table functions, you can also use SKIP_INPUT when the SELECT
  # statement doesn't require an input file
   qsv sqlp SKIP_INPUT "SELECT 1 AS one, '2' AS two, 3.0 AS three"

  # use stdin as input
   cat data.csv | qsv sqlp - 'select * from stdin'
   cat data.csv | qsv sqlp - data2.csv 'select * from stdin join data2 on stdin.col1 = data2.col1'

  # automatic snappy decompression/compression
   qsv sqlp data.csv.sz 'select * from data where col1 > 10' --output result.csv.sz

  # explain query plan
   qsv sqlp data.csv 'explain select * from data where col1 > 10 order by col2 desc limit 20'

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_sqlp.rs.

Usage:
    qsv sqlp [options] <input>... <sql>
    qsv sqlp --help

sqlp arguments:
    input                  The CSV file/s to query. Use '-' for standard input.
                           If input is a directory, all files in the directory will be read as input.
                           If the input is a file with a '.infile-list' extension, the
                           file will be read as a list of files to use as input.
                           If the input are snappy compressed file(s), it will be
                           decompressed automatically.
                           Column headers are required. Use 'qsv rename _all_generic --no-headers'
                           to add generic column names (_col_N) to a CSV with no headers.
                           If you are using Polars SQL's table functions like read_csv() & read_parquet()
                           to read input files directly, you can use the special value 'SKIP_INPUT'
                           to skip input preprocessing.

    sql                    The SQL query/ies to run. Each input file will be available as a table
                           named after the file name (without the extension), or as "_t_N"
                           where N is the 1-based index.
                           If the query ends with ".sql", it will be read as a SQL script file,
                           with each SQL query separated by a semicolon. It will execute the queries
                           in order, and the result of the LAST query will be returned as the result.
                           SQL scripts support single-line comments starting with '--'.

sqlp options:
    --format <arg>            The output format to use. Valid values are:
                                csv      Comma-separated values
                                json     JSON
                                jsonl    JSONL (JSON Lines)
                                parquet  Apache Parquet
                                arrow    Apache Arrow IPC
                                avro     Apache Avro
                              [default: csv]

                              POLARS CSV INPUT PARSING OPTIONS:
    --try-parsedates          Automatically try to parse dates/datetimes and time.
                              If parsing fails, columns remain as strings.
                              Note that if dates are not well-formatted in your CSV,
                              that you may want to try to set `--ignore-errors` to relax
                              the CSV parsing of dates.
    --infer-len <arg>         The number of rows to scan when inferring the schema of the CSV.
                              Set to 0 to do a full table scan (warning: can be slow).
                              [default: 10000]
    --cache-schema            Create and cache Polars schema JSON files.
                              If specified and the schema file/s do not exist, it will check if a
                              stats cache is available. If so, it will use it to derive a Polars schema
                              and save it. If there's no stats cache, it will infer the schema 
                              using --infer-len and save the inferred schemas. 
                              Each schema file will have the same file stem as the corresponding
                              input file, with the extension ".pschema.json"
                              (data.csv's Polars schema file will be data.pschema.json)
                              If the file/s exists, it will load the schema instead of inferring it
                              (ignoring --infer-len) and attempt to use it for each corresponding
                              Polars "table" with the same file stem.
    --streaming               Use streaming mode when parsing CSVs. This will use less memory
                              but will be slower. Only use this when you get out of memory errors.
    --low-memory              Use low memory mode when parsing CSVs. This will use less memory
                              but will be slower. Only use this when you get out of memory errors.
    --no-optimizations        Disable non-default query optimizations. This will make queries slower.
                              Use this when you get query errors or to force CSV parsing when there
                              is only one input file, no CSV parsing options are used and its not
                              a SQL script. Otherwise, the CSV will be read directly into a LazyFrame
                              using the fast path with the multithreaded, mem-mapped read_csv()
                              Polars SQL function which is much faster though not as configurable than
                              the regular CSV parser.
    --truncate-ragged-lines   Truncate ragged lines when parsing CSVs. If set, rows with more
                              columns than the header will be truncated. If not set, the query
                              will fail. Use this only when you get an error about ragged lines.
    --ignore-errors           Ignore errors when parsing CSVs. If set, rows with errors
                              will be skipped. If not set, the query will fail.
                              Only use this when debugging queries, as Polars does batched
                              parsing and will skip the entire batch where the error occurred.
                              To get more detailed error messages, set the environment variable
                              POLARS_BACKTRACE_IN_ERR=1 before running the query.
    --rnull-values <arg>      The comma-delimited list of case-sensitive strings to consider as
                              null values when READING CSV files (e.g. NULL, NONE, <empty string>).
                              Use "<empty string>" to consider an empty string a null value.
                              [default: <empty string>]
    --decimal-comma           Use comma as the decimal separator when parsing CSVs.
                              Otherwise, use period as the decimal separator.
                              Note that you'll need to set --delimiter to an alternate delimiter
                              other than the default comma if you are using this option.

                              CSV OUTPUT FORMAT ONLY:
    --datetime-format <fmt>   The datetime format to use writing datetimes.
                              See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                              for the list of valid format specifiers.
    --date-format <fmt>       The date format to use writing dates.
    --time-format <fmt>       The time format to use writing times.
    --float-precision <arg>   The number of digits of precision to use when writing floats.
    --wnull-value <arg>       The string to use when WRITING null values.
                              [default: <empty string>]

                              ARROW/AVRO/PARQUET OUTPUT FORMATS ONLY:
    --compression <arg>       The compression codec to use when writing arrow or parquet files.
                                For Arrow, valid values are: zstd, lz4, uncompressed
                                For Avro, valid values are: deflate, snappy, uncompressed (default)
                                For Parquet, valid values are: zstd, lz4raw, gzip, snappy, uncompressed
                              [default: zstd]

                              PARQUET OUTPUT FORMAT ONLY:
    --compress-level <arg>    The compression level to use when using zstd or gzip compression.
                              When using zstd, valid values are -7 to 22, with -7 being the
                              lowest compression level and 22 being the highest compression level.
                              When using gzip, valid values are 1-9, with 1 being the lowest
                              compression level and 9 being the highest compression level.
                              Higher compression levels are slower.
                              The zstd default is 3, and the gzip default is 6.
    --statistics              Compute column statistics when writing parquet files.
    
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading and writing CSV data.
                           Must be a single character. [default: ,]
    -Q, --quiet            Do not return result shape to stderr.
"#;

use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    fs::File,
    io,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

use polars::{
    datatypes::PlSmallStr,
    io::avro::{AvroWriter, Compression as AvroCompression},
    prelude::{
        Arc, CsvWriter, DataFrame, GzipLevel, IpcCompression, IpcWriter, JsonFormat, JsonWriter,
        LazyCsvReader, LazyFileListReader, NullValues, OptFlags, ParquetCompression, ParquetWriter,
        Schema, SerWriter, StatisticsOptions, ZstdLevel,
    },
    sql::SQLContext,
};
use regex::Regex;
use serde::Deserialize;

use crate::{
    cmd::joinp::tsvssv_delim,
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    util,
    util::{get_stats_records, process_input},
    CliResult,
};

static DEFAULT_GZIP_COMPRESSION_LEVEL: u8 = 6;
static DEFAULT_ZSTD_COMPRESSION_LEVEL: i32 = 3;

#[derive(Deserialize, Clone)]
struct Args {
    arg_input:                  Vec<PathBuf>,
    arg_sql:                    String,
    flag_format:                String,
    flag_try_parsedates:        bool,
    flag_infer_len:             usize,
    flag_cache_schema:          bool,
    flag_streaming:             bool,
    flag_low_memory:            bool,
    flag_no_optimizations:      bool,
    flag_ignore_errors:         bool,
    flag_truncate_ragged_lines: bool,
    flag_decimal_comma:         bool,
    flag_datetime_format:       Option<String>,
    flag_date_format:           Option<String>,
    flag_time_format:           Option<String>,
    flag_float_precision:       Option<usize>,
    flag_rnull_values:          String,
    flag_wnull_value:           String,
    flag_compression:           String,
    flag_compress_level:        Option<i32>,
    flag_statistics:            bool,
    flag_output:                Option<String>,
    flag_delimiter:             Option<Delimiter>,
    flag_quiet:                 bool,
}

#[derive(Default, Clone, PartialEq)]
enum OutputMode {
    #[default]
    Csv,
    Json,
    Jsonl,
    Parquet,
    Arrow,
    Avro,
    None,
}

// shamelessly copied from
// https://github.com/pola-rs/polars-cli/blob/main/src/main.rs
impl OutputMode {
    fn execute_query(
        &self,
        query: &str,
        ctx: &mut SQLContext,
        mut delim: u8,
        args: Args,
    ) -> CliResult<(usize, usize)> {
        let mut df = DataFrame::default();
        let execute_inner = || {
            df = ctx
                .execute(query)
                .and_then(polars::prelude::LazyFrame::collect)?;

            // we don't want to write anything if the output mode is None
            if matches!(self, OutputMode::None) {
                return Ok(());
            }

            let w = match args.flag_output {
                Some(path) => {
                    delim = tsvssv_delim(path.clone(), delim);
                    Box::new(File::create(path)?) as Box<dyn Write>
                },
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };
            let mut w = io::BufWriter::with_capacity(256_000, w);

            let out_result = match self {
                OutputMode::Csv => CsvWriter::new(&mut w)
                    .with_separator(delim)
                    .with_datetime_format(args.flag_datetime_format)
                    .with_date_format(args.flag_date_format)
                    .with_time_format(args.flag_time_format)
                    .with_float_precision(args.flag_float_precision)
                    .with_null_value(args.flag_wnull_value)
                    .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
                    .finish(&mut df),
                OutputMode::Json => JsonWriter::new(&mut w)
                    .with_json_format(JsonFormat::Json)
                    .finish(&mut df),
                OutputMode::Jsonl => JsonWriter::new(&mut w)
                    .with_json_format(JsonFormat::JsonLines)
                    .finish(&mut df),
                OutputMode::Parquet => {
                    let compression: PqtCompression = args
                        .flag_compression
                        .parse()
                        .unwrap_or(PqtCompression::Uncompressed);

                    let parquet_compression = match compression {
                        PqtCompression::Uncompressed => ParquetCompression::Uncompressed,
                        PqtCompression::Snappy => ParquetCompression::Snappy,
                        PqtCompression::Lz4Raw => ParquetCompression::Lz4Raw,
                        PqtCompression::Gzip => {
                            let gzip_level = args
                                .flag_compress_level
                                .unwrap_or_else(|| DEFAULT_GZIP_COMPRESSION_LEVEL.into())
                                as u8;
                            ParquetCompression::Gzip(Some(GzipLevel::try_new(gzip_level)?))
                        },
                        PqtCompression::Zstd => {
                            let zstd_level = args
                                .flag_compress_level
                                .unwrap_or(DEFAULT_ZSTD_COMPRESSION_LEVEL);
                            ParquetCompression::Zstd(Some(ZstdLevel::try_new(zstd_level)?))
                        },
                    };

                    let statistics_options = if args.flag_statistics {
                        StatisticsOptions {
                            min_value:      true,
                            max_value:      true,
                            distinct_count: true,
                            null_count:     true,
                        }
                    } else {
                        StatisticsOptions {
                            min_value:      false,
                            max_value:      false,
                            distinct_count: false,
                            null_count:     false,
                        }
                    };

                    ParquetWriter::new(&mut w)
                        .with_row_group_size(Some(768 ^ 2))
                        .with_statistics(statistics_options)
                        .with_compression(parquet_compression)
                        .finish(&mut df)
                        .map(|_| ())
                },
                OutputMode::Arrow => {
                    let compression: ArrowCompression = args
                        .flag_compression
                        .parse()
                        .unwrap_or(ArrowCompression::Uncompressed);

                    let ipc_compression: Option<IpcCompression> = match compression {
                        ArrowCompression::Uncompressed => None,
                        ArrowCompression::Lz4 => Some(IpcCompression::LZ4),
                        ArrowCompression::Zstd => Some(IpcCompression::ZSTD),
                    };

                    IpcWriter::new(&mut w)
                        .with_compression(ipc_compression)
                        .finish(&mut df)
                },
                OutputMode::Avro => {
                    let compression: QsvAvroCompression = args
                        .flag_compression
                        .parse()
                        .unwrap_or(QsvAvroCompression::Uncompressed);

                    let avro_compression = match compression {
                        QsvAvroCompression::Uncompressed => None,
                        QsvAvroCompression::Deflate => Some(AvroCompression::Deflate),
                        QsvAvroCompression::Snappy => Some(AvroCompression::Snappy),
                    };

                    AvroWriter::new(&mut w)
                        .with_compression(avro_compression)
                        .finish(&mut df)
                },
                OutputMode::None => Ok(()),
            };

            w.flush()?;
            out_result
        };

        match execute_inner() {
            Ok(()) => Ok(df.shape()),
            Err(e) => {
                fail_clierror!("Failed to execute query: {query}: {e}")
            },
        }
    }
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "csv" => Ok(OutputMode::Csv),
            "json" => Ok(OutputMode::Json),
            "jsonl" => Ok(OutputMode::Jsonl),
            "parquet" => Ok(OutputMode::Parquet),
            "arrow" => Ok(OutputMode::Arrow),
            "avro" => Ok(OutputMode::Avro),
            _ => Err(format!("Invalid output mode: {s}")),
        }
    }
}

#[derive(Default, Copy, Clone)]
enum PqtCompression {
    Uncompressed,
    Gzip,
    Snappy,
    #[default]
    Zstd,
    Lz4Raw,
}
#[derive(Default, Copy, Clone)]
enum ArrowCompression {
    #[default]
    Uncompressed,
    Lz4,
    Zstd,
}

#[derive(Default, Copy, Clone)]
enum QsvAvroCompression {
    #[default]
    Uncompressed,
    Deflate,
    Snappy,
}

impl FromStr for PqtCompression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "uncompressed" => Ok(PqtCompression::Uncompressed),
            "gzip" => Ok(PqtCompression::Gzip),
            "snappy" => Ok(PqtCompression::Snappy),
            "lz4raw" => Ok(PqtCompression::Lz4Raw),
            "zstd" => Ok(PqtCompression::Zstd),
            _ => Err(format!("Invalid Parquet compression format: {s}")),
        }
    }
}

impl FromStr for ArrowCompression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "uncompressed" => Ok(ArrowCompression::Uncompressed),
            "lz4" => Ok(ArrowCompression::Lz4),
            "zstd" => Ok(ArrowCompression::Zstd),
            _ => Err(format!("Invalid Arrow compression format: {s}")),
        }
    }
}

impl FromStr for QsvAvroCompression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "uncompressed" => Ok(QsvAvroCompression::Uncompressed),
            "deflate" => Ok(QsvAvroCompression::Deflate),
            "snappy" => Ok(QsvAvroCompression::Snappy),
            _ => Err(format!("Invalid Avro compression format: {s}")),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;

    let mut skip_input = false;
    args.arg_input = if args.arg_input == [PathBuf::from_str("SKIP_INPUT").unwrap()] {
        skip_input = true;
        Vec::new()
    } else {
        process_input(args.arg_input, &tmpdir, "")?
    };

    let rnull_values = if args.flag_rnull_values == "<empty string>" {
        vec![PlSmallStr::EMPTY]
    } else {
        args.flag_rnull_values
            .split(',')
            .map(|value| {
                if value == "<empty string>" {
                    PlSmallStr::EMPTY
                } else {
                    PlSmallStr::from_str(value)
                }
            })
            .collect()
    };

    if args.flag_wnull_value == "<empty string>" {
        args.flag_wnull_value.clear();
    };

    let output_mode: OutputMode = args.flag_format.parse().unwrap_or(OutputMode::Csv);
    let no_output: OutputMode = OutputMode::None;

    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };

    let comment_char = if let Ok(comment_char) = env::var("QSV_COMMENT_CHAR") {
        Some(PlSmallStr::from_string(comment_char))
    } else {
        None
    };

    let mut optflags = OptFlags::from_bits_truncate(0);
    if args.flag_no_optimizations {
        optflags |= OptFlags::TYPE_COERCION;
    } else {
        optflags |= OptFlags::PROJECTION_PUSHDOWN
            | OptFlags::PREDICATE_PUSHDOWN
            | OptFlags::CLUSTER_WITH_COLUMNS
            | OptFlags::TYPE_COERCION
            | OptFlags::SIMPLIFY_EXPR
            | OptFlags::FILE_CACHING
            | OptFlags::SLICE_PUSHDOWN
            | OptFlags::COMM_SUBPLAN_ELIM
            | OptFlags::COMM_SUBEXPR_ELIM
            | OptFlags::ROW_ESTIMATE
            | OptFlags::FAST_PROJECTION;
    };

    optflags.set(OptFlags::STREAMING, args.flag_streaming);

    // check if the input is a SQL script (ends with .sql)
    let is_sql_script = std::path::Path::new(&args.arg_sql)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("sql"));

    // if infer_len is 0, its not a SQL script, and there is only one input CSV, we can infer the
    // schema of the CSV more intelligently by counting the number of rows in the file instead of
    // scanning the entire file with a 0 infer_len which triggers a full table scan.
    args.flag_infer_len =
        if args.flag_infer_len == 0 && !is_sql_script && !skip_input && args.arg_input.len() == 1 {
            let rconfig =
                Config::new(Some(args.arg_input[0].to_string_lossy().to_string()).as_ref())
                    .delimiter(args.flag_delimiter)
                    .no_headers(false);
            util::count_rows(&rconfig).unwrap_or(0) as usize
        } else {
            args.flag_infer_len
        };

    // gated by log::log_enabled!(log::Level::Debug) to avoid the
    // relatively expensive overhead of generating the debug string
    // for the optimization flags struct
    let debuglog_flag = log::log_enabled!(log::Level::Debug);
    if debuglog_flag {
        log::debug!("Optimization flags: {optflags:?}");
        log::debug!(
            "Delimiter: {delim} Infer_schema_len: {infer_len} try_parse_dates: {parse_dates} \
             ignore_errors: {ignore_errors}, low_memory: {low_memory}, float_precision: \
             {float_precision:?}, skip_input: {skip_input}, is_sql_script: {is_sql_script}",
            infer_len = args.flag_infer_len,
            parse_dates = args.flag_try_parsedates,
            ignore_errors = args.flag_ignore_errors,
            low_memory = args.flag_low_memory,
            float_precision = args.flag_float_precision,
        );
    }

    let mut ctx = SQLContext::new();
    let mut table_aliases = HashMap::with_capacity(args.arg_input.len());
    let mut lossy_table_name = Cow::default();
    let mut table_name;

    // if there is only one input file and its a CSV and no CSV parsing options are used,
    // we can use the fast path to read the CSV directly into a LazyFrame without having to
    // parse and register it as a table in the SQL context using Polars SQL's read_csv function
    // The user can also skip all these heuristics and force the fast path by using the special
    // value '<SKIP_INPUT>' as the input file.
    if skip_input {
        // we don't need to do anything here, as we are skipping input
        if debuglog_flag {
            // Using the slow path to read and parse the CSV/s into tables in the SQL context.
            log::debug!("Skipping input processing...");
        }
    } else if args.arg_input.len() == 1
        && !is_sql_script
        && delim == b','
        && !args.flag_no_optimizations
        && !args.flag_try_parsedates
        && args.flag_infer_len == 10_000 // make sure this matches the usage text default
        && !args.flag_cache_schema
        && !args.flag_streaming
        && !args.flag_low_memory
        && !args.flag_truncate_ragged_lines
        && !args.flag_ignore_errors
        && rnull_values == vec![PlSmallStr::EMPTY]
        && !args.flag_decimal_comma
        && comment_char.is_none()
        && std::path::Path::new(&args.arg_input[0])
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csv"))
    {
        // replace all instances of the FROM clause case-insensitive in the SQL query with the
        // read_csv function using a regex
        let input = &args.arg_input[0];
        let table_name = Path::new(input)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_else(|| {
                lossy_table_name = input.to_string_lossy();
                &lossy_table_name
            });
        let sql = args.arg_sql.clone();
        // the regex is case-insensitive and allows for the table name to be enclosed in single or
        // double quotes or not enclosed at all. It also allows for the table name to be
        // aliased as _t_1.
        let from_clause_regex =
            Regex::new(&format!(r#"(?i)FROM\s+['"]?({table_name}|_t_1)['"]?"#))?;
        let modified_query = from_clause_regex.replace_all(
            &sql,
            format!("FROM read_csv('{}')", input.to_string_lossy()),
        );
        args.arg_sql = modified_query.to_string();
        if debuglog_flag {
            log::debug!("Using fast path - Modified Query: {modified_query}");
        }
    } else {
        // --------------------------------------------
        // we have more than one input and/or we are using CSV parsing options, so we need to
        // parse the CSV first, and register the input files as tables in the SQL context
        // AKA the "slow path"
        // --------------------------------------------

        if debuglog_flag {
            log::debug!("Using the slow path...");
        }

        let cache_schemas = args.flag_cache_schema;

        for (idx, table) in args.arg_input.iter().enumerate() {
            // as we are using the table name as alias, we need to make sure that the table name is
            // a valid identifier. if its not utf8, we use the lossy version
            table_name = Path::new(table)
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_else(|| {
                    lossy_table_name = table.to_string_lossy();
                    &lossy_table_name
                });

            table_aliases.insert(table_name.to_string(), format!("_t_{}", idx + 1));

            if debuglog_flag {
                log::debug!(
                    "Registering table: {table_name} as {alias}",
                    alias = table_aliases.get(table_name).unwrap(),
                );
            }

            // we build the lazyframe, accounting for the --cache-schema flag
            let mut create_schema = cache_schemas;
            let mut lf = if cache_schemas {
                let mut work_lf = LazyCsvReader::new(table)
                    .with_has_header(true)
                    .with_missing_is_null(true)
                    .with_comment_prefix(comment_char.clone())
                    .with_null_values(Some(NullValues::AllColumns(rnull_values.clone())))
                    .with_separator(tsvssv_delim(table, delim))
                    .with_try_parse_dates(args.flag_try_parsedates)
                    .with_ignore_errors(args.flag_ignore_errors)
                    .with_truncate_ragged_lines(args.flag_truncate_ragged_lines)
                    .with_decimal_comma(args.flag_decimal_comma)
                    .with_low_memory(args.flag_low_memory);

                let schema_file = table.canonicalize()?.with_extension("pschema.json");

                //  check if the pschema.json file exists and is newer than the table file
                let mut valid_schema_exists = schema_file.exists()
                    && schema_file.metadata()?.modified()? > table.metadata()?.modified()?;

                if !valid_schema_exists {
                    // we don't have a valid pschema.json file,
                    // check if we have stats, as we can derive pschema.json file from it
                    let schema_args = util::SchemaArgs {
                        flag_enum_threshold:  0,
                        flag_ignore_case:     false,
                        flag_strict_dates:    false,
                        // we still get all the stats columns so we can use the stats cache
                        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
                        flag_dates_whitelist: String::new(),
                        flag_prefer_dmy:      false,
                        flag_force:           false,
                        flag_stdout:          false,
                        flag_jobs:            Some(util::njobs(None)),
                        flag_no_headers:      false,
                        flag_delimiter:       args.flag_delimiter,
                        arg_input:            Some(table.to_string_lossy().into_owned()),
                        flag_memcheck:        false,
                    };
                    let (csv_fields, csv_stats) =
                        get_stats_records(&schema_args, util::StatsMode::PolarsSchema)?;

                    let mut schema = Schema::with_capacity(csv_stats.len());
                    for (idx, stat) in csv_stats.iter().enumerate() {
                        schema.insert(
                            PlSmallStr::from_str(
                                simdutf8::basic::from_utf8(csv_fields.get(idx).unwrap()).unwrap(),
                            ),
                            {
                                let datatype = &stat.r#type;
                                #[allow(clippy::match_same_arms)]
                                match datatype.as_str() {
                                    "String" => polars::datatypes::DataType::String,
                                    "Integer" => {
                                        let min = stat.min.as_ref().unwrap();
                                        let max = stat.max.as_ref().unwrap();
                                        if min.parse::<i32>().is_ok() && max.parse::<i32>().is_ok()
                                        {
                                            polars::datatypes::DataType::Int32
                                        } else {
                                            polars::datatypes::DataType::Int64
                                        }
                                    },
                                    "Float" => {
                                        let min = stat.min.as_ref().unwrap();
                                        let max = stat.max.as_ref().unwrap();
                                        if min.parse::<f32>().is_ok() && max.parse::<f32>().is_ok()
                                        {
                                            polars::datatypes::DataType::Float32
                                        } else {
                                            polars::datatypes::DataType::Float64
                                        }
                                    },
                                    "Boolean" => polars::datatypes::DataType::Boolean,
                                    "Date" => polars::datatypes::DataType::Date,
                                    _ => polars::datatypes::DataType::String,
                                }
                            },
                        );
                    }
                    let stats_schema = Arc::new(schema);
                    let stats_schema_json = serde_json::to_string_pretty(&stats_schema)?;

                    let mut file = BufWriter::new(File::create(&schema_file)?);
                    file.write_all(stats_schema_json.as_bytes())?;
                    file.flush()?;
                    if debuglog_flag {
                        log::debug!("Saved stats_schema to file: {}", schema_file.display());
                    }
                    valid_schema_exists = true;
                }

                if valid_schema_exists {
                    // We have a valid pschema.json file!
                    // load the schema and deserialize it and use it with the lazy frame
                    let file = File::open(&schema_file)?;
                    let mut buf_reader = BufReader::new(file);
                    let mut schema_json = String::with_capacity(100);
                    buf_reader.read_to_string(&mut schema_json)?;
                    let schema: Schema = serde_json::from_str(&schema_json)?;
                    if debuglog_flag {
                        log::debug!("Loaded schema from file: {}", schema_file.display());
                    }
                    work_lf = work_lf.with_schema(Some(Arc::new(schema)));
                    create_schema = false;
                } else {
                    // there is no valid pschema.json file, infer the schema using --infer-len
                    work_lf = work_lf.with_infer_schema_length(Some(args.flag_infer_len));
                    create_schema = true;
                }
                work_lf.finish()?
            } else {
                // --cache-schema is not enabled, we always --infer-len schema
                LazyCsvReader::new(table)
                    .with_has_header(true)
                    .with_missing_is_null(true)
                    .with_comment_prefix(comment_char.clone())
                    .with_null_values(Some(NullValues::AllColumns(rnull_values.clone())))
                    .with_separator(tsvssv_delim(table, delim))
                    .with_infer_schema_length(Some(args.flag_infer_len))
                    .with_try_parse_dates(args.flag_try_parsedates)
                    .with_ignore_errors(args.flag_ignore_errors)
                    .with_truncate_ragged_lines(args.flag_truncate_ragged_lines)
                    .with_decimal_comma(args.flag_decimal_comma)
                    .with_low_memory(args.flag_low_memory)
                    .finish()?
            };
            ctx.register(table_name, lf.clone().with_optimizations(optflags));

            // the lazy frame's schema has been updated and --cache-schema is enabled
            // update the pschema.json file, if necessary
            if create_schema {
                let schema = lf.collect_schema()?;
                let schema_json = serde_json::to_string_pretty(&schema)?;

                let schema_file = table.canonicalize()?.with_extension("pschema.json");
                let mut file = BufWriter::new(File::create(&schema_file)?);
                file.write_all(schema_json.as_bytes())?;
                file.flush()?;
                if debuglog_flag {
                    log::debug!("Saved schema to file: {}", schema_file.display());
                }
            }
        }
    }

    if debuglog_flag && !skip_input {
        let tables_in_context = ctx.get_tables();
        log::debug!("Table(s) registered in SQL Context: {tables_in_context:?}");
    }

    // check if the query is a SQL script
    let queries = if is_sql_script {
        let mut file = File::open(&args.arg_sql)?;
        let mut sql_script = String::new();
        file.read_to_string(&mut sql_script)?;

        // remove comments from the SQL script
        // we only support single-line comments in SQL scripts
        // i.e. comments that start with "--" and end at the end of the line
        // so the regex is performant and simple
        let comment_regex = Regex::new(r"^--.*$")?;
        let sql_script = comment_regex.replace_all(&sql_script, "");
        sql_script
            .split(';')
            .map(std::string::ToString::to_string)
            .filter(|s| !s.trim().is_empty())
            .collect()
    } else {
        // its not a sql script, just a single query
        vec![args.arg_sql.clone()]
    };

    if debuglog_flag {
        log::debug!("SQL query/ies({}): {queries:?}", queries.len());
    }

    let num_queries = queries.len();
    let last_query: usize = num_queries.saturating_sub(1);
    let mut is_last_query;
    let mut current_query = String::new();
    let mut query_result_shape = (0_usize, 0_usize);
    let mut now = Instant::now();

    for (idx, query) in queries.iter().enumerate() {
        // check if this is the last query in the script
        is_last_query = idx == last_query;

        // replace aliases in query
        current_query.clone_from(query);
        for (table_name, table_alias) in &table_aliases {
            // we quote the table name to avoid issues with reserved keywords and
            // other characters that are not allowed in identifiers
            current_query = current_query.replace(table_alias, &(format!(r#""{table_name}""#)));
        }

        if debuglog_flag {
            log::debug!("Executing query {idx}: {current_query}");
            now = Instant::now();
        }
        query_result_shape = if is_last_query {
            // if this is the last query, we use the output mode specified by the user
            output_mode.execute_query(&current_query, &mut ctx, delim, args.clone())?
        } else {
            // this is not the last query, we only execute the query, but don't write the output
            no_output.execute_query(&current_query, &mut ctx, delim, args.clone())?
        };
        if debuglog_flag {
            log::debug!(
                "Query {idx} successfully executed in {elapsed:?} seconds: {query_result_shape:?}",
                elapsed = now.elapsed().as_secs_f32()
            );
        }
    }

    compress_output_if_needed(args.flag_output)?;

    if !args.flag_quiet {
        eprintln!("{query_result_shape:?}");
    }

    Ok(())
}

/// if the output ends with ".sz", we snappy compress the output
/// and replace the original output with the compressed output
pub fn compress_output_if_needed(
    output_file: Option<String>,
) -> Result<(), crate::clitypes::CliError> {
    use crate::cmd::snappy::compress;

    if let Some(output) = output_file {
        if std::path::Path::new(&output)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("sz"))
        {
            log::info!("Compressing output with Snappy");

            // we need to copy the output to a tempfile first, and then
            // compress the tempfile to the original output sz file
            let mut tempfile = tempfile::NamedTempFile::new()?;
            io::copy(&mut File::open(output.clone())?, tempfile.as_file_mut())?;
            tempfile.flush()?;

            // safety: we just created the tempfile, so we know that the path is valid utf8
            // https://github.com/Stebalien/tempfile/issues/192
            let input_fname = tempfile.path().to_str().unwrap();
            let input = File::open(input_fname)?;
            let output_sz_writer = std::fs::File::create(output)?;
            compress(
                input,
                output_sz_writer,
                util::max_jobs(),
                DEFAULT_WTR_BUFFER_CAPACITY,
            )?;
        }
    };
    Ok(())
}
