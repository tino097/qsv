static USAGE: &str = r#"
Joins two sets of CSV data on the specified columns using the Pola.rs engine.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

Unlike the join command, joinp can process files larger than RAM, is multithreaded,
has join key validation, pre-join filtering, supports asof joins & its output columns
can be coalesced (no duplicate columns).

However, joinp doesn't have an --ignore-case option.

Returns the shape of the join result (number of rows, number of columns) to stderr.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_joinp.rs.

Usage:
    qsv joinp [options] <columns1> <input1> <columns2> <input2>
    qsv joinp --cross [--validate <arg>] <input1> <input2> [--output <file>]
    qsv joinp --help

joinp arguments:
    Both <input1> and <input2> files need to have headers. Stdin is not supported.

    The columns arguments specify the columns to join for each input. Columns are
    referenced by name. Specify multiple columns by separating them with a comma.
    Both <columns1> and <columns2> must specify exactly the same number of columns.

    Note that <input1> is the left CSV data set and <input2> is the right CSV data set.

joinp options:
    -i, --ignore-case      When set, joins are done case insensitively.
    --left                 Do a 'left outer' join. This returns all rows in
                           first CSV data set, including rows with no
                           corresponding row in the second data set. When no
                           corresponding row exists, it is padded out with
                           empty fields.
    --left-anti            This returns only the rows in the first CSV data set
                           that do not have a corresponding row in the second
                           data set. The output schema is the same as the
                           first dataset.
    --left-semi            This returns only the rows in the first CSV data set
                           that have a corresponding row in the second data set.
                           The output schema is the same as the first data set.
    --right                Do a 'right outer' join. This returns all rows in
                           second CSV data set, including rows with no
                           corresponding row in the first data set. When no
                           corresponding row exists, it is padded out with
                           empty fields. (This is the reverse of 'outer left'.)
    --full                 Do a 'full outer' join. This returns all rows in
                           both data sets with matching records joined. If
                           there is no match, the missing side will be padded
                           out with empty fields.
    --cross                USE WITH CAUTION.
                           This returns the cartesian product of the CSV
                           data sets given. The number of rows return is
                           equal to N * M, where N and M correspond to the
                           number of rows in the given data sets, respectively.
                           The columns1 and columns2 arguments are ignored.

    --coalesce             Force the join to coalesce columns with the same name.
                           For inner joins, this is not necessary as the join
                           columns are automatically coalesced.

    --filter-left <arg>    Filter the left CSV data set by the given Polars SQL
                           expression BEFORE the join. Only rows that evaluates
                           to true are used in the join.
    --filter-right <arg>   Filter the right CSV data set by the given Polars SQL
                           expression BEFORE the join. Only rows that evaluates
                           to true are used in the join.
    --validate <arg>       Validate the join keys BEFORE performing the join.
                           Valid values are:
                             none - No validation is performed.
                             onetomany - join keys are unique in the left data set.
                             manytoone - join keys are unique in the right data set.
                             onetoone - join keys are unique in both left & right data sets.
                           [default: none]

                           JOIN OPTIONS:
    --nulls                When set, joins will work on empty fields.
                           Otherwise, empty fields are completely ignored.
    --streaming            When set, the join will be done in a streaming fashion.
                           Only use this when you get out of memory errors.

                           POLARS CSV PARSING OPTIONS:
    --try-parsedates       When set, will attempt to parse the columns as dates.
                           If the parse fails, columns remain as strings.
                           This is useful when the join keys are formatted as 
                           dates with differing date formats, as the date formats
                           will be normalized. Note that this will be automatically 
                           enabled when using asof joins.
    --infer-len <arg>      The number of rows to scan when inferring the schema of the CSV.
                           Set to 0 to do a full table scan (warning: very slow).
                           [default: 10000]
    --cache-schema         Create and cache Polars schema JSON files.
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
    --low-memory           Use low memory mode when parsing CSVs. This will use less memory
                           but will be slower. It will also process the join in streaming mode.
                           Only use this when you get out of memory errors.
    --no-optimizations     Disable non-default join optimizations. This will make joins slower.
                           Only use this when you get join errors.                           
    --ignore-errors        Ignore errors when parsing CSVs. If set, rows with errors
                           will be skipped. If not set, the query will fail.
                           Only use this when debugging queries, as polars does batched
                           parsing and will skip the entire batch where the error occurred.
                           To get more detailed error messages, set the environment variable
                           POLARS_BACKTRACE_IN_ERR=1 before running the join.
    --decimal-comma        Use comma as the decimal separator when parsing CSVs.
                           Otherwise, use period as the decimal separator.
                           Note that you'll need to set --delimiter to an alternate delimiter
                           other than the default comma if you are using this option.

                           ASOF JOIN OPTIONS:
    --asof                 Do an 'asof' join. This is similar to a left inner
                           join, except we match on nearest key rather than
                           equal keys. Note that both CSV data sets will be SORTED
                           AUTOMATICALLY on the join columns.
                           Particularly useful for time series data.
    --left_by <arg>        Do an 'asof_by' join - a special implementation of the asof
                           join that searches for the nearest keys within a subgroup
                           set by the asof_by columns. This specifies the column/s for
                           the left CSV. Columns are referenced by name. Specify
                           multiple columns by separating them with a comma.
    --right_by <arg>       Do an 'asof_by' join. This specifies the column/s for
                           the right CSV.     
    --strategy <arg>       The strategy to use for the asof join:
                             backward - For each row in the first CSV data set,
                                        we find the last row in the second data set
                                        whose key is less than or equal to the key
                                        in the first data set.
                             forward -  For each row in the first CSV data set,
                                        we find the first row in the second data set
                                        whose key is greater than or equal to the key
                                        in the first data set.
                             nearest -  selects the last row in the second data set
                                        whose value is nearest to the value in the
                                        first data set.
                           [default: backward]
    --tolerance <arg>      The tolerance for the nearest asof join. This is only
                           used when the nearest strategy is used. The
                           tolerance is a positive integer that specifies
                           the maximum number of rows to search for a match.

                           If the join is done on a column of type Date, Time or
                           DateTime, then the tolerance is interpreted using
                           the following language:
                                1d - 1 day
                                1h - 1 hour
                                1m - 1 minute
                                1s - 1 second
                                1ms - 1 millisecond
                                1us - 1 microsecond
                                1ns - 1 nanosecond
                                1w - 1 week
                                1mo - 1 month
                                1q - 1 quarter
                                1y - 1 year
                                1i - 1 index count
                             Or combine them: “3d12h4m25s” # 3 days, 12 hours,
                             4 minutes, and 25 seconds
                             Suffix with “_saturating” to indicate that dates too
                             large for their month should saturate at the largest date
                             (e.g. 2022-02-29 -> 2022-02-28) instead of erroring.

                             OUTPUT FORMAT OPTIONS:
   --sql-filter <SQL>        The SQL expression to apply against the join result.
                             Ordinarily used to select columns and filter rows from 
                             the join result. Be sure to select from the "join_result"
                             table when formulating the SQL expression.
                             (e.g. "select c1, c2 as colname from join_result where c2 > 20")
   --datetime-format <fmt>   The datetime format to use writing datetimes.
                             See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                             for the list of valid format specifiers.
   --date-format <fmt>       The date format to use writing dates.
   --time-format <fmt>       The time format to use writing times.
   --float-precision <arg>   The number of digits of precision to use when writing floats.
                             (default: 6)
   --null-value <arg>        The string to use when writing null values.
                             (default: <empty string>)

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading/writing CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not return join shape to stderr.
"#;

use std::{
    env,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    str,
};

use polars::{datatypes::AnyValue, prelude::*, sql::SQLContext};
use serde::Deserialize;
use tempfile::tempdir;

use crate::{
    cmd::sqlp::compress_output_if_needed, config::Delimiter, util, util::get_stats_records,
    CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_columns1:          String,
    arg_input1:            String,
    arg_columns2:          String,
    arg_input2:            String,
    flag_left:             bool,
    flag_left_anti:        bool,
    flag_left_semi:        bool,
    flag_right:            bool,
    flag_full:             bool,
    flag_cross:            bool,
    flag_coalesce:         bool,
    flag_filter_left:      Option<String>,
    flag_filter_right:     Option<String>,
    flag_validate:         Option<String>,
    flag_nulls:            bool,
    flag_streaming:        bool,
    flag_try_parsedates:   bool,
    flag_decimal_comma:    bool,
    flag_infer_len:        usize,
    flag_cache_schema:     bool,
    flag_low_memory:       bool,
    flag_no_optimizations: bool,
    flag_ignore_errors:    bool,
    flag_asof:             bool,
    flag_left_by:          Option<String>,
    flag_right_by:         Option<String>,
    flag_strategy:         Option<String>,
    flag_tolerance:        Option<String>,
    flag_sql_filter:       Option<String>,
    flag_datetime_format:  Option<String>,
    flag_date_format:      Option<String>,
    flag_time_format:      Option<String>,
    flag_float_precision:  Option<usize>,
    flag_null_value:       String,
    flag_output:           Option<String>,
    flag_delimiter:        Option<Delimiter>,
    flag_quiet:            bool,
    flag_ignore_case:      bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // always try to parse dates when its an asof join
    // just in case the user doesn't specify it
    // and they're using date/time/datetime columns
    if args.flag_asof {
        args.flag_try_parsedates = true;
    }

    let tmpdir = tempdir()?;
    let join = args.new_join(&tmpdir)?;

    let flag_validate = args
        .flag_validate
        .unwrap_or_else(|| "none".to_string())
        .to_lowercase();
    let validation = match flag_validate.as_str() {
        // no unique checks
        "manytomany" | "none" => JoinValidation::ManyToMany,
        // join keys are unique in the left data set
        "onetomany" => JoinValidation::OneToMany,
        // join keys are unique in the right data set
        "manytoone" => JoinValidation::ManyToOne,
        // join keys are unique in both left & right data sets
        "onetoone" => JoinValidation::OneToOne,
        s => return fail_incorrectusage_clierror!("Invalid join validation: {s}"),
    };

    let join_shape: (usize, usize) = match (
        args.flag_left,
        args.flag_left_anti,
        args.flag_left_semi,
        args.flag_right,
        args.flag_full,
        args.flag_cross,
        args.flag_asof,
    ) {
        (false, false, false, false, false, false, false) => {
            join.run(JoinType::Inner, validation, false)
        },
        (true, false, false, false, false, false, false) => {
            join.run(JoinType::Left, validation, false)
        },
        (false, true, false, false, false, false, false) => {
            join.run(JoinType::Anti, validation, false)
        },
        (false, false, true, false, false, false, false) => {
            join.run(JoinType::Semi, validation, false)
        },
        (false, false, false, true, false, false, false) => {
            join.run(JoinType::Right, validation, false)
        },
        (false, false, false, false, true, false, false) => {
            join.run(JoinType::Full, validation, false)
        },
        (false, false, false, false, false, true, false) => {
            join.run(JoinType::Cross, validation, false)
        },
        (false, false, false, false, false, false, true) => {
            // safety: flag_strategy is always is_some() as it has a default value
            args.flag_strategy = Some(args.flag_strategy.unwrap().to_lowercase());
            let strategy = match args.flag_strategy.as_deref() {
                Some("backward") | None => AsofStrategy::Backward,
                Some("forward") => AsofStrategy::Forward,
                Some("nearest") => AsofStrategy::Nearest,
                Some(s) => return fail_clierror!("Invalid asof strategy: {}", s),
            };

            let mut asof_options = AsOfOptions {
                strategy,
                ..Default::default()
            };

            if strategy == AsofStrategy::Nearest {
                if let Some(ref tolerance) = args.flag_tolerance {
                    // If the tolerance is a positive integer, it is tolerance number of rows.
                    // Otherwise, it is a tolerance date language spec.
                    if let Ok(numeric_tolerance) = atoi_simd::parse_pos::<u64>(tolerance.as_bytes())
                    {
                        asof_options.tolerance = Some(AnyValue::UInt64(numeric_tolerance));
                    } else {
                        asof_options.tolerance_str = Some(tolerance.into());
                    }
                }
            }
            if args.flag_left_by.is_some() {
                asof_options.left_by = Some(
                    args.flag_left_by
                        .unwrap()
                        .split(',')
                        .map(PlSmallStr::from_str)
                        .collect(),
                );
            }
            if args.flag_right_by.is_some() {
                asof_options.right_by = Some(
                    args.flag_right_by
                        .unwrap()
                        .split(',')
                        .map(PlSmallStr::from_str)
                        .collect(),
                );
            }
            join.run(JoinType::AsOf(asof_options), validation, true)
        },
        _ => fail_incorrectusage_clierror!("Please pick exactly one join operation."),
    }?;

    if !args.flag_quiet {
        eprintln!("{join_shape:?}");
    }

    Ok(())
}

struct JoinStruct {
    left_lf:          LazyFrame,
    left_sel:         String,
    right_lf:         LazyFrame,
    right_sel:        String,
    output:           Option<String>,
    delim:            u8,
    coalesce:         bool,
    streaming:        bool,
    no_optimizations: bool,
    sql_filter:       Option<String>,
    datetime_format:  Option<String>,
    date_format:      Option<String>,
    time_format:      Option<String>,
    float_precision:  Option<usize>,
    null_value:       String,
    ignore_case:      bool,
}

impl JoinStruct {
    fn run(
        mut self,
        jointype: JoinType,
        validation: JoinValidation,
        asof_join: bool,
    ) -> CliResult<(usize, usize)> {
        let mut left_selcols: Vec<_> = self
            .left_sel
            .split(',')
            .map(polars::lazy::dsl::col)
            .collect();
        let mut right_selcols: Vec<_> = self
            .right_sel
            .split(',')
            .map(polars::lazy::dsl::col)
            .collect();

        // If ignore_case is enabled, create lowercase versions of the join columns
        if self.ignore_case {
            // Create temporary lowercase versions of join columns in left dataframe
            for col in &left_selcols {
                self.left_lf = self
                    .left_lf
                    .with_column(col.clone().str().to_lowercase().alias(format!(
                        "_qsv-{}-lower",
                        col.to_string()
                            .trim_start_matches(r#"col(""#)
                            .trim_end_matches(r#"")"#)
                    )));
            }

            // Create temporary lowercase versions of join columns in right dataframe
            for col in &right_selcols {
                self.right_lf = self
                    .right_lf
                    .with_column(col.clone().str().to_lowercase().alias(format!(
                        "_qsv-{}-lower",
                        col.to_string()
                            .trim_start_matches(r#"col(""#)
                            .trim_end_matches(r#"")"#)
                    )));
            }

            // Create new vectors for the lowercase column names
            let left_selcols_w: Vec<_> = left_selcols
                .iter()
                .map(|col| {
                    polars::lazy::dsl::col(format!(
                        "_qsv-{}-lower",
                        col.to_string()
                            .trim_start_matches(r#"col(""#)
                            .trim_end_matches(r#"")"#)
                    ))
                })
                .collect();
            left_selcols = left_selcols_w;
            let right_selcols_w: Vec<_> = right_selcols
                .iter()
                .map(|col| {
                    polars::lazy::dsl::col(format!(
                        "_qsv-{}-lower",
                        col.to_string()
                            .trim_start_matches(r#"col(""#)
                            .trim_end_matches(r#"")"#)
                    ))
                })
                .collect();
            right_selcols = right_selcols_w;
        }

        let left_selcols_len = left_selcols.len();
        let right_selcols_len = right_selcols.len();

        if left_selcols_len != right_selcols_len {
            return fail_incorrectusage_clierror!(
                "Both columns1 ({left_selcols:?}) and columns2 ({right_selcols:?}) must specify \
                 the same number of columns ({left_selcols_len } != {right_selcols_len})."
            );
        }

        let coalesce_flag = if self.coalesce {
            JoinCoalesce::CoalesceColumns
        } else {
            JoinCoalesce::JoinSpecific
        };

        let mut optflags = OptFlags::from_bits_truncate(0);
        if self.no_optimizations {
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
        }

        optflags.set(OptFlags::STREAMING, self.streaming);

        // log::debug!("Optimization flags: {optimization_flags:?}");

        let join_results = if jointype == JoinType::Cross {
            // cross join doesn't need join columns
            self.left_lf
                .with_optimizations(optflags)
                .join_builder()
                .with(self.right_lf.with_optimizations(optflags))
                .how(JoinType::Cross)
                .coalesce(coalesce_flag)
                .allow_parallel(true)
                .validate(validation)
                .finish()
                .collect()?
        } else {
            if asof_join {
                // sort by the asof columns, as asof joins require sorted join column data
                let left_selcols_vec: Vec<PlSmallStr> =
                    self.left_sel.split(',').map(PlSmallStr::from_str).collect();

                self.left_lf = self
                    .left_lf
                    .sort(left_selcols_vec, SortMultipleOptions::default());

                let right_selcols_vec: Vec<PlSmallStr> = self
                    .right_sel
                    .split(',')
                    .map(PlSmallStr::from_str)
                    .collect();

                self.right_lf = self
                    .right_lf
                    .sort(right_selcols_vec, SortMultipleOptions::default());
            }

            self.left_lf
                .with_optimizations(optflags)
                .join_builder()
                .with(self.right_lf.with_optimizations(optflags))
                .left_on(left_selcols)
                .right_on(right_selcols)
                .how(jointype)
                .coalesce(coalesce_flag)
                .allow_parallel(true)
                .validate(validation)
                .finish()
                .collect()?
        };

        let mut results_df = if let Some(sql_filter) = &self.sql_filter {
            let mut ctx = SQLContext::new();
            ctx.register("join_result", join_results.lazy());
            ctx.execute(sql_filter)
                .and_then(polars::prelude::LazyFrame::collect)?
        } else {
            join_results
        };

        // if self.ignore_case, remove the temporary lowercase columns from the dataframe
        if self.ignore_case {
            // Get all column names
            let cols = results_df.get_column_names();
            // Filter out the lowercase columns (those with "_qsv-*-lower" pattern)
            let keep_cols: Vec<String> = cols
                .iter()
                .filter(|&col| !(col.starts_with("_qsv-") && col.ends_with("-lower")))
                .map(|&s| s.to_string())
                .collect();
            // Select only the non-lowercase columns
            results_df = results_df.select(keep_cols)?;
        }

        let mut out_delim = self.delim;
        let mut out_writer = match self.output {
            Some(ref output_file) => {
                out_delim = tsvssv_delim(output_file, self.delim);

                // no need to use buffered writer here, as CsvWriter already does that
                let path = Path::new(&output_file);
                Box::new(File::create(path).unwrap()) as Box<dyn Write>
            },
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        // shape is the number of rows and columns
        let join_shape = results_df.shape();

        CsvWriter::new(&mut out_writer)
            .include_header(true)
            .with_separator(out_delim)
            .with_datetime_format(self.datetime_format)
            .with_date_format(self.date_format)
            .with_time_format(self.time_format)
            .with_float_precision(self.float_precision)
            .with_null_value(self.null_value)
            .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
            .finish(&mut results_df)?;

        compress_output_if_needed(self.output)?;

        Ok(join_shape)
    }
}

impl Args {
    fn new_join(&mut self, tmpdir: &tempfile::TempDir) -> CliResult<JoinStruct> {
        // =============== NEW_JOIN HELPER FUNCTIONS =================

        // Helper function to create a LazyFrameReader with common settings
        fn create_lazy_reader(
            file_path: &str,
            comment_char: Option<&PlSmallStr>,
            args: &Args,
            delim: u8,
        ) -> LazyCsvReader {
            LazyCsvReader::new(file_path)
                .with_has_header(true)
                .with_missing_is_null(args.flag_nulls)
                .with_comment_prefix(comment_char.cloned())
                .with_separator(tsvssv_delim(file_path, delim))
                .with_try_parse_dates(args.flag_try_parsedates)
                .with_decimal_comma(args.flag_decimal_comma)
                .with_low_memory(args.flag_low_memory)
                .with_ignore_errors(args.flag_ignore_errors)
        }

        // Helper function to handle schema creation from stats
        fn create_schema_from_stats(input_path: &Path, args: &Args) -> CliResult<Schema> {
            let schema_args = util::SchemaArgs {
                flag_enum_threshold:  0,
                flag_ignore_case:     false,
                flag_strict_dates:    false,
                flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
                flag_dates_whitelist: String::new(),
                flag_prefer_dmy:      false,
                flag_force:           false,
                flag_stdout:          false,
                flag_jobs:            Some(util::njobs(None)),
                flag_no_headers:      false,
                flag_delimiter:       args.flag_delimiter,
                arg_input:            Some(input_path.to_string_lossy().into_owned()),
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
                                if min.parse::<i32>().is_ok() && max.parse::<i32>().is_ok() {
                                    polars::datatypes::DataType::Int32
                                } else {
                                    polars::datatypes::DataType::Int64
                                }
                            },
                            "Float" => {
                                let min = stat.min.as_ref().unwrap();
                                let max = stat.max.as_ref().unwrap();
                                if min.parse::<f32>().is_ok() && max.parse::<f32>().is_ok() {
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
            Ok(schema)
        }

        // Helper function to setup a LazyFrame with schema handling
        #[inline]
        fn setup_lazy_frame(
            input_path: &Path,
            comment_char: Option<&PlSmallStr>,
            args: &Args,
            delim: u8,
            debuglog_flag: bool,
        ) -> CliResult<(LazyFrame, bool)> {
            let schema_file = input_path.canonicalize()?.with_extension("pschema.json");
            let mut create_schema = args.flag_cache_schema;

            let mut reader =
                create_lazy_reader(input_path.to_str().unwrap(), comment_char, args, delim);

            if create_schema {
                let mut valid_schema_exists = schema_file.exists()
                    && schema_file.metadata()?.modified()? > input_path.metadata()?.modified()?;

                if !valid_schema_exists {
                    let schema = create_schema_from_stats(input_path, args)?;
                    let stats_schema = Arc::new(schema);
                    let stats_schema_json = serde_json::to_string_pretty(&stats_schema)?;

                    let mut file = BufWriter::new(File::create(&schema_file)?);
                    file.write_all(stats_schema_json.as_bytes())?;
                    file.flush()?;
                    if debuglog_flag {
                        log::debug!("Saved schema to file: {}", schema_file.display());
                    }
                    valid_schema_exists = true;
                }

                if valid_schema_exists {
                    let file = File::open(&schema_file)?;
                    let mut buf_reader = BufReader::new(file);
                    let mut schema_json = String::with_capacity(100);
                    buf_reader.read_to_string(&mut schema_json)?;
                    let schema: Schema = serde_json::from_str(&schema_json)?;
                    reader = reader.with_schema(Some(Arc::new(schema)));
                    create_schema = false;
                } else {
                    reader = reader.with_infer_schema_length(Some(args.flag_infer_len));
                    create_schema = true;
                }
            } else {
                reader = reader.with_infer_schema_length(if args.flag_infer_len == 0 {
                    None
                } else {
                    Some(args.flag_infer_len)
                });
            }

            Ok((reader.finish()?, create_schema))
        }

        // ============ START OF NEW_JOIN MAIN CODE ==============
        let debuglog_flag = log::log_enabled!(log::Level::Debug);

        let delim = if let Some(delimiter) = self.flag_delimiter {
            delimiter.as_byte()
        } else {
            b','
        };

        let comment_char = if let Ok(comment_char) = env::var("QSV_COMMENT_CHAR") {
            Some(PlSmallStr::from_string(comment_char))
        } else {
            None
        };

        // Check if input files exist
        let mut input1_path = PathBuf::from(&self.arg_input1);
        if !input1_path.exists() {
            return fail_clierror!("Input file {} does not exist.", self.arg_input1);
        }
        let mut input2_path = PathBuf::from(&self.arg_input2);
        if !input2_path.exists() {
            return fail_clierror!("Input file {} does not exist.", self.arg_input2);
        }

        // Handle snappy compression for left input
        if input1_path.extension().and_then(std::ffi::OsStr::to_str) == Some("sz") {
            let decompressed_path = util::decompress_snappy_file(&input1_path, tmpdir)?;
            self.arg_input1.clone_from(&decompressed_path);
            input1_path = PathBuf::from(decompressed_path);
        }

        // Setup left LazyFrame
        let (mut left_lf, create_left_schema) = setup_lazy_frame(
            &input1_path,
            comment_char.as_ref(),
            self,
            delim,
            debuglog_flag,
        )?;

        if create_left_schema {
            let schema = left_lf.collect_schema()?;
            let schema_json = serde_json::to_string_pretty(&schema)?;
            let schema_file = input1_path.canonicalize()?.with_extension("pschema.json");
            let mut file = BufWriter::new(File::create(&schema_file)?);
            file.write_all(schema_json.as_bytes())?;
            file.flush()?;
            if debuglog_flag {
                log::debug!("Saved left schema to file: {}", schema_file.display());
            }
        }

        // Apply left filter if needed
        if let Some(filter_left) = &self.flag_filter_left {
            let filter_left_expr = polars::sql::sql_expr(filter_left)?;
            left_lf = left_lf.filter(filter_left_expr);
        }

        // Handle snappy compression for right input
        if input2_path.extension().and_then(std::ffi::OsStr::to_str) == Some("sz") {
            let decompressed_path = util::decompress_snappy_file(&input2_path, tmpdir)?;
            self.arg_input2.clone_from(&decompressed_path);
            input2_path = PathBuf::from(decompressed_path);
        }

        // Setup right LazyFrame
        let (mut right_lf, create_right_schema) = setup_lazy_frame(
            &input2_path,
            comment_char.as_ref(),
            self,
            delim,
            debuglog_flag,
        )?;

        if create_right_schema {
            let schema = right_lf.collect_schema()?;
            let schema_json = serde_json::to_string_pretty(&schema)?;
            let schema_file = input2_path.canonicalize()?.with_extension("pschema.json");
            let mut file = BufWriter::new(File::create(&schema_file)?);
            file.write_all(schema_json.as_bytes())?;
            file.flush()?;
            if debuglog_flag {
                log::debug!("Saved right schema to file: {}", schema_file.display());
            }
        }

        // Apply right filter if needed
        if let Some(filter_right) = &self.flag_filter_right {
            let filter_right_expr = polars::sql::sql_expr(filter_right)?;
            right_lf = right_lf.filter(filter_right_expr);
        }

        Ok(JoinStruct {
            left_lf,
            left_sel: self.arg_columns1.clone(),
            right_lf,
            right_sel: self.arg_columns2.clone(),
            output: self.flag_output.clone(),
            delim,
            coalesce: self.flag_coalesce,
            streaming: self.flag_streaming,
            no_optimizations: self.flag_no_optimizations,
            sql_filter: self.flag_sql_filter.clone(),
            datetime_format: self.flag_datetime_format.clone(),
            date_format: self.flag_date_format.clone(),
            time_format: self.flag_time_format.clone(),
            float_precision: self.flag_float_precision,
            null_value: if self.flag_null_value == "<empty string>" {
                String::new()
            } else {
                self.flag_null_value.clone()
            },
            ignore_case: self.flag_ignore_case,
        })
    }
}

/// if the file has a TSV/TAB or SSV extension, we automatically use
/// tab or semicolon as the delimiter
/// otherwise, we use the delimiter specified by the user
/// if the file has a .sz extension, we check the original file extension
/// to determine the delimiter
pub fn tsvssv_delim<P: AsRef<Path>>(file: P, orig_delim: u8) -> u8 {
    let inputfile_extension = file
        .as_ref()
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default()
        .to_ascii_lowercase();

    match inputfile_extension.as_str() {
        "tsv" | "tab" => b'\t',
        "ssv" => b';',
        "sz" => {
            // its a snappy compressed file
            // check what the original file extension is
            let orig_filestem = file
                .as_ref()
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default()
                .to_ascii_lowercase();
            let orig_extension = Path::new(&orig_filestem).extension().unwrap_or_default();
            if orig_extension == "tsv" || orig_extension == "tab" {
                b'\t'
            } else if orig_extension == "ssv" {
                b';'
            } else {
                orig_delim
            }
        },
        _ => orig_delim,
    }
}

#[test]
fn test_tsvssv_delim() {
    assert_eq!(tsvssv_delim("test.tsv", b','), b'\t');
    assert_eq!(tsvssv_delim("test.tab", b','), b'\t');
    assert_eq!(tsvssv_delim("test.ssv", b','), b';');
    assert_eq!(tsvssv_delim("test.sz", b','), b',');
    assert_eq!(tsvssv_delim("test.csv", b','), b',');
    assert_eq!(tsvssv_delim("test.TSV", b','), b'\t');
    assert_eq!(tsvssv_delim("test.Tab", b','), b'\t');
    assert_eq!(tsvssv_delim("test.SSV", b','), b';');
    assert_eq!(tsvssv_delim("test.sZ", b','), b',');
    assert_eq!(tsvssv_delim("test.CsV", b','), b',');
    assert_eq!(tsvssv_delim("test", b','), b',');
    assert_eq!(tsvssv_delim("test.csv.sz", b','), b',');
    assert_eq!(tsvssv_delim("test.tsv.sz", b','), b'\t');
    assert_eq!(tsvssv_delim("test.tab.sz", b','), b'\t');
    assert_eq!(tsvssv_delim("test.ssv.sz", b','), b';');
    assert_eq!(tsvssv_delim("test.csV.Sz", b','), b',');
    assert_eq!(tsvssv_delim("test.TSV.SZ", b','), b'\t');
    assert_eq!(tsvssv_delim("test.Tab.sZ", b','), b'\t');
    assert_eq!(tsvssv_delim("test.SSV.sz", b','), b';');
}
