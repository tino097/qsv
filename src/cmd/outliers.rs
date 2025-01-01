static USAGE: &str = r#"
Identify or remove outliers in CSV data.

Usage:
    qsv outliers remove [<input>]
    qsv outliers [options] [<input>]
    qsv outliers --help

outliers options:
    -s, --select <arg>      Select specific columns to analyze for outliers
                            By default all columns are analyzed.
                            See 'qsv select --help' for the format details.
    -m, --method <method>   Method to use for outlier detection:
                              outer - Use outer fences (Q3 + 3.0×IQR) [default]
                              inner - Use inner fences (Q3 + 1.5×IQR)
                              both  - Show outliers using both fence types
    --force                 Force recomputing stats even if cache exists
    -q, --quiet             Don't show detailed outlier information, only summary

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)

Notes:
    - Uses the stats cache if available (see 'qsv stats --help')
    - For numeric columns: Values outside the IQR fences are considered outliers
    - For dates: Values are converted to days before outlier detection

Examples:
    # Find outliers in all numeric columns using outer fences
    qsv outliers data.csv

    # Find outliers in specific columns using inner fences
    qsv outliers -s "temperature,pressure" -m inner data.csv

    # Show both inner and outer fence outliers with minimal output
    qsv outliers -m both -q data.csv
"#;

use std::{fs::File, io, path::Path, str};

use csv::ByteRecord;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;

use crate::{
    cmd::stats::StatsData,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
    util::{get_stats_records, StatsMode},
    CliResult,
};

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static ANTIMODE_CACHE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

// Helper function to get or create cached antimodes
fn get_cached_antimodes(antimode: &str) -> HashSet<String> {
    let cache = ANTIMODE_CACHE.get_or_init(|| Mutex::new(HashSet::new()));
    let mut cache = cache.lock().unwrap();
    if cache.is_empty() {
        cache.extend(
            antimode
                .split('|')
                .map(String::from)
                .collect::<HashSet<_>>()
        );
    }
    cache.clone()
}

#[derive(Deserialize)]
struct Args {
    cmd_remove:     bool,
    arg_input:      Option<String>,
    flag_select:    SelectColumns,
    flag_method:    Option<String>,
    flag_force:     bool,
    flag_quiet:     bool,
    flag_delimiter: Option<Delimiter>,
    flag_output:    Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
enum FenceType {
    Inner,
    Outer,
    Both,
}

impl FenceType {
    fn from_str(s: &str) -> FenceType {
        match s.to_lowercase().as_str() {
            "inner" => FenceType::Inner,
            // "outer" => FenceType::Outer,
            "both" => FenceType::Both,
            _ => FenceType::Outer, // default
        }
    }
}

// Helper function to determine if a value is an outlier based on fences
fn is_outlier(value: f64, lower_fence: f64, upper_fence: f64) -> bool {
    value < lower_fence || value > upper_fence
}

struct CsvSetup {
    reader:         csv::Reader<Box<dyn io::Read + Send>>,
    writer:         csv::Writer<Box<dyn io::Write>>,
    headers:        ByteRecord,
    selected_stats: Vec<StatsData>,
    progress_bar:   Option<ProgressBar>,
}

fn setup_csv(
    args: &Args,
    csv_stats: &[StatsData],
    write_outlier_headers: bool,
) -> CliResult<CsvSetup> {
    // Setup CSV reader with selection
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .select(args.flag_select.clone());
    let mut rdr = rconfig.reader()?;

    // Get headers and create selection
    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    // Filter stats to only include selected columns
    let selected_stats: Vec<StatsData> = csv_stats
        .iter()
        .enumerate()
        .filter(|(idx, _)| sel.contains(idx))
        .map(|(_, stat)| stat.clone())
        .collect();

    // Setup CSV writer
    let wtr: Box<dyn io::Write> = match args.flag_output {
        Some(ref output_path) => Box::new(File::create(Path::new(output_path))?),
        None => Box::new(io::stdout()),
    };
    let mut csv_wtr = csv::WriterBuilder::new()
        .delimiter(args.flag_delimiter.unwrap_or(Delimiter(b',')).0)
        .from_writer(wtr);

    // Write headers based on mode
    if write_outlier_headers {
        csv_wtr.write_record([
            "column",
            "data_type",
            "value",
            "record_number",
            "fence_type",
            "reason",
            "lower_fence",
            "upper_fence",
        ])?;
    } else {
        csv_wtr.write_record(&headers)?;
    }

    // Setup progress bar
    let progress_bar = if args.flag_quiet {
        None
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] Processing record {pos}")
                .unwrap(),
        );
        Some(pb)
    };

    Ok(CsvSetup {
        reader: rdr,
        writer: csv_wtr,
        headers,
        selected_stats,
        progress_bar,
    })
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Get stats records (we still need these for the fences/thresholds)
    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: String::new(),
        flag_prefer_dmy:      false,
        flag_force:           args.flag_force,
        flag_stdout:          false,
        flag_jobs:            None,
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            args.arg_input.clone(),
        flag_memcheck:        false,
    };
    let (_csv_fields, csv_stats) = get_stats_records(&schema_args, StatsMode::Outliers)?;
    eprintln!("csv_stats: {:#?}", csv_stats);

    if args.cmd_remove {
        remove_outliers(&args, &csv_stats)
    } else {
        identify_outliers(&args, &csv_stats)
    }
}

// New function to handle the remove subcommand
fn remove_outliers(args: &Args, csv_stats: &[StatsData]) -> CliResult<()> {
    let mut setup = setup_csv(args, csv_stats, false)?;
    let method = FenceType::from_str(args.flag_method.as_deref().unwrap_or("outer"));
    let mut record = ByteRecord::new();
    let mut record_count = 0u64;
    let mut removed_count = 0u64;

    while setup.reader.read_byte_record(&mut record)? {
        record_count += 1;
        if let Some(pb) = &setup.progress_bar {
            pb.set_position(record_count);
        }

        let mut is_outlier = false;

        // Check each selected column for outliers
        for (col_idx, stat) in setup.selected_stats.iter().enumerate() {
            let field = record.get(col_idx).unwrap_or_default();

            match stat.r#type.as_str() {
                "Integer" | "Float" => {
                    if let Some(val) = str::from_utf8(field)
                        .ok()
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        is_outlier |= is_numeric_outlier(val, stat, &method);
                    }
                },
                "String" => {
                    if let Ok(val) = str::from_utf8(field) {
                        is_outlier |= is_string_outlier(val, stat);
                    }
                },
                _ => {},
            }

            if is_outlier {
                break; // No need to check other columns if we found an outlier
            }
        }

        // Write record only if it's not an outlier
        if is_outlier {
            removed_count += 1;
        } else {
            setup.writer.write_record(&record)?;
        }
    }

    if let Some(pb) = &setup.progress_bar {
        pb.finish_with_message(format!(
            "Processed {record_count} records, removed {removed_count} outliers"
        ));
    }

    setup.writer.flush()?;
    Ok(())
}

// New helper function for checking numeric outliers without writing
fn is_numeric_outlier(value: f64, stat: &StatsData, method: &FenceType) -> bool {
    if let (Some(lower_inner), Some(upper_inner), Some(lower_outer), Some(upper_outer)) = (
        stat.lower_inner_fence,
        stat.upper_inner_fence,
        stat.lower_outer_fence,
        stat.upper_outer_fence,
    ) {
        let (is_inner, is_outer) = (
            is_outlier(value, lower_inner, upper_inner),
            is_outlier(value, lower_outer, upper_outer),
        );

        match method {
            FenceType::Inner => is_inner,
            FenceType::Outer => is_outer,
            FenceType::Both => is_inner || is_outer,
        }
    } else {
        false
    }
}

// Helper function for checking string outliers
fn is_string_outlier(value: &str, stat: &StatsData) -> bool {
    // Check string length outliers
    if let (Some(mean_len), Some(stddev_len)) = (stat.avg_length, stat.stddev_length) {
        #[allow(clippy::cast_precision_loss)]
        let len = value.len() as f64;
        let z_score = (len - mean_len) / stddev_len;
        if z_score.abs() > 3.0 {
            return true;
        }
    }

    // Check rare categories with cached antimodes
    if let Some(ref antimode) = stat.antimode {
        if !antimode.starts_with("*ALL") {
            let cached_antimodes = get_cached_antimodes(antimode);
            if cached_antimodes.contains(value) {
                return true;
            }
        }
    }

    false
}

fn identify_outliers(args: &Args, csv_stats: &[StatsData]) -> CliResult<()> {
    let mut setup = setup_csv(args, csv_stats, true)?;
    let method = FenceType::from_str(args.flag_method.as_deref().unwrap_or("outer"));
    let mut record = ByteRecord::new();
    let mut record_count = 0u64;

    while setup.reader.read_byte_record(&mut record)? {
        record_count += 1;
        if let Some(pb) = &setup.progress_bar {
            pb.set_position(record_count);
        }

        // Process each selected column
        for stat in &setup.selected_stats {
            let col_idx = setup
                .headers
                .iter()
                .position(|h| h == stat.field.as_bytes())
                .unwrap_or(0);
            let field = record.get(col_idx).unwrap_or_default();

            match stat.r#type.as_str() {
                "Integer" | "Float" => {
                    if let Some(val) = str::from_utf8(field)
                        .ok()
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        check_numeric_outlier(val, stat, &method, record_count, &mut setup.writer)?;
                    }
                },
                "String" => {
                    if let Ok(val) = str::from_utf8(field) {
                        check_string_outlier(val, stat, record_count, &mut setup.writer)?;
                    }
                },
                _ => {},
            }
        }
    }

    if let Some(pb) = &setup.progress_bar {
        pb.finish_with_message(format!("Processed {record_count} records"));
    }

    setup.writer.flush()?;
    Ok(())
}

// Helper function to check numeric outliers
fn check_numeric_outlier(
    value: f64,
    stat: &StatsData,
    method: &FenceType,
    record_no: u64,
    csv_wtr: &mut csv::Writer<Box<dyn io::Write>>,
) -> CliResult<()> {
    if let (Some(lower_inner), Some(upper_inner), Some(lower_outer), Some(upper_outer)) = (
        stat.lower_inner_fence,
        stat.upper_inner_fence,
        stat.lower_outer_fence,
        stat.upper_outer_fence,
    ) {
        let (is_inner, is_outer) = (
            is_outlier(value, lower_inner, upper_inner),
            is_outlier(value, lower_outer, upper_outer),
        );

        match (method, is_inner, is_outer) {
            (FenceType::Inner | FenceType::Both, true, _) | (FenceType::Outer, _, true) => {
                let (fence_type, lower, upper) = if is_outer {
                    (FenceType::Outer, lower_outer, upper_outer)
                } else {
                    (FenceType::Inner, lower_inner, upper_inner)
                };

                csv_wtr.write_record([
                    &stat.field,
                    &stat.r#type,
                    &value.to_string(),
                    &record_no.to_string(),
                    &format!("{fence_type:?}"),
                    &format!(
                        "Outside {} fences ({:.2}, {:.2})",
                        if is_outer { "outer" } else { "inner" },
                        lower,
                        upper
                    ),
                    &lower.to_string(),
                    &upper.to_string(),
                ])?;
            },
            _ => {},
        }
    }
    Ok(())
}

// Helper function to check string outliers
fn check_string_outlier(
    value: &str,
    stat: &StatsData,
    record_no: u64,
    csv_wtr: &mut csv::Writer<Box<dyn io::Write>>,
) -> CliResult<()> {
    // Check string length outliers
    if let (Some(mean_len), Some(stddev_len)) = (stat.avg_length, stat.stddev_length) {
        eprintln!(
            "mean_len: {mean_len}, stddev_len: {stddev_len} value_len: {}",
            value.len()
        );
        #[allow(clippy::cast_precision_loss)]
        let len = value.len() as f64;
        let z_score = (len - mean_len) / stddev_len;

        if z_score.abs() > 3.0 {
            csv_wtr.write_record([
                &stat.field,
                &stat.r#type,
                value,
                &record_no.to_string(),
                "Both",
                &format!("Unusual length: {len} (z-score: {z_score:.2})"),
                "",
                "",
            ])?;
        }
    }

    // Check rare categories
    if let Some(ref antimode) = stat.antimode {
        if !antimode.starts_with("*ALL") {
            // let antimodes: Vec<&str> = antimode.split(',').collect();
            if antimode.split('|').any(|x| x == value) {
                csv_wtr.write_record([
                    &stat.field,
                    &stat.r#type,
                    value,
                    &record_no.to_string(),
                    "Both",
                    "Rare category (antimode)",
                    "",
                    "",
                ])?;
            }
        }
    }
    Ok(())
}
