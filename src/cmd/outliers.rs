static USAGE: &str = r#"
Detect outliers in numeric columns using statistical methods.

Usage:
    qsv outliers [options] [<input>]
    qsv outliers --help

outliers options:
    -s, --select <arg>       Select specific columns to analyze for outliers
                            (comma separated). By default all numeric columns
                            are analyzed.
    -m, --method <method>    Method to use for outlier detection:
                              outer - Use outer fences (Q3 + 3.0×IQR) [default]
                              inner - Use inner fences (Q3 + 1.5×IQR)
                              both  - Show outliers using both fence types
    --force                 Force recomputing stats even if cache exists
    -q, --quiet            Don't show detailed outlier information, only summary

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                          Must be a single character. (default: ,)

Notes:
    - Uses the stats cache if available (see 'qsv stats --help')
    - For numeric columns: Values outside the IQR fences are considered outliers
    - For dates: Values are converted to days before outlier detection
    - Outputs both a summary count and detailed list of outliers per column
    - The --quiet flag suppresses detailed outlier listings

Examples:
    # Find outliers in all numeric columns using outer fences
    qsv outliers data.csv

    # Find outliers in specific columns using inner fences
    qsv outliers -s "temperature,pressure" -m inner data.csv

    # Show both inner and outer fence outliers with minimal output
    qsv outliers -m both -q data.csv
"#;

use std::{collections::HashMap, fs::File, io, path::Path, str};

use csv::{ByteRecord, Reader};
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

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_select:    SelectColumns,
    flag_method:    Option<String>,
    flag_force:     bool,
    flag_quiet:     bool,
    flag_delimiter: Option<Delimiter>,
    flag_output:    Option<String>,
}

#[derive(Debug)]
struct OutlierResult {
    column:          String,
    data_type:       String,
    outlier_count:   usize,
    outlier_details: Vec<OutlierDetail>,
}

#[derive(Debug)]
struct OutlierDetail {
    value:      String,
    reason:     String,
    fence_type: FenceType, // inner or outer
    record_no:  u64,       // Add this field
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
            "outer" => FenceType::Outer,
            "both" => FenceType::Both,
            _ => FenceType::Outer, // default
        }
    }
}

// Helper function to determine if a value is an outlier based on fences
fn is_outlier(value: f64, lower_fence: f64, upper_fence: f64) -> bool {
    value < lower_fence || value > upper_fence
}

fn process_outliers(
    // rdr: &mut Reader<Box<dyn io::Read>>,
    rdr: &mut Reader<Box<dyn io::Read + Send>>, // Add + Send trait bound
    stats: &[StatsData],
    method: FenceType,
    quiet: bool,
) -> CliResult<Vec<OutlierResult>> {
    let mut results: Vec<OutlierResult> = stats
        .iter()
        .map(|stat| OutlierResult {
            column:          stat.field.clone(),
            data_type:       stat.r#type.clone(),
            outlier_count:   0,
            outlier_details: Vec::new(),
        })
        .collect();

    eprintln!("results: {:#?}", results);

    // Create index map for column positions
    let headers = rdr.headers()?.clone();
    let col_indices: HashMap<_, _> = headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name.to_string(), i))
        .collect();
    eprintln!("col_indices: {:#?}", col_indices);

    let pb = if !quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] Processing record {pos}")
                .unwrap(),
        );
        Some(pb)
    } else {
        None
    };

    let mut record = ByteRecord::new();
    let mut record_count = 0;
    while rdr.read_byte_record(&mut record)? {
        record_count += 1;
        if let Some(pb) = &pb {
            pb.set_position(record_count);
        }

        for (result_idx, stat) in stats.iter().enumerate() {
            let col_idx = match col_indices.get(&stat.field) {
                Some(idx) => idx,
                None => continue,
            };

            // Get the field as a byte slice
            let field = record.get(*col_idx).unwrap_or_default();

            match stat.r#type.as_str() {
                "Integer" | "Float" => {
                    if let (
                        Some(lower_inner),
                        Some(upper_inner),
                        Some(lower_outer),
                        Some(upper_outer),
                    ) = (
                        stat.lower_inner_fence,
                        stat.upper_inner_fence,
                        stat.lower_outer_fence,
                        stat.upper_outer_fence,
                    ) {
                        // Parse the bytes directly as a float
                        // if let Ok(val) = str::from_utf8(field)
                        //     .ok()
                        //     .and_then(|s| s.parse::<f64>().ok())
                        // {
                        //     let (is_inner, is_outer) = (
                        //         is_outlier(val, lower_inner, upper_inner),
                        //         is_outlier(val, lower_outer, upper_outer),
                        //     );
                        if let Some(val) = str::from_utf8(field)
                            .ok()
                            .and_then(|s| s.parse::<f64>().ok())
                        {
                            let (is_inner, is_outer) = (
                                is_outlier(val, lower_inner, upper_inner),
                                is_outlier(val, lower_outer, upper_outer),
                            );

                            match (method.clone(), is_inner, is_outer) {
                                (FenceType::Inner, true, _)
                                | (FenceType::Outer, _, true)
                                | (FenceType::Both, true, _) => {
                                    results[result_idx].outlier_count += 1;
                                    results[result_idx].outlier_details.push(OutlierDetail {
                                        value:      val.to_string(),
                                        reason:     format!(
                                            "Outside {} fences ({:.2}, {:.2})",
                                            if is_outer { "outer" } else { "inner" },
                                            if is_outer { lower_outer } else { lower_inner },
                                            if is_outer { upper_outer } else { upper_inner }
                                        ),
                                        fence_type: if is_outer {
                                            FenceType::Outer
                                        } else {
                                            FenceType::Inner
                                        },
                                        record_no:  record_count,
                                    });
                                },
                                _ => {},
                            }
                        }
                    }
                },
                "String" => {
                    // Convert bytes to string only when needed
                    if let Ok(val) = str::from_utf8(field) {
                        // Check string length outliers
                        if let (Some(mean_len), Some(stddev_len)) =
                            (stat.avg_length, stat.stddev_length)
                        {
                            let len = val.len() as f64;
                            let z_score = (len - mean_len) / stddev_len;

                            if z_score.abs() > 3.0 {
                                results[result_idx].outlier_count += 1;
                                results[result_idx].outlier_details.push(OutlierDetail {
                                    value:      val.to_string(),
                                    reason:     format!(
                                        "Unusual length: {} (z-score: {:.2})",
                                        len, z_score
                                    ),
                                    fence_type: FenceType::Both,
                                    record_no:  record_count,
                                });
                            }
                        }

                        // Check rare categories
                        if let Some(ref antimode) = stat.antimode {
                            if !antimode.starts_with("*ALL") {
                                let antimodes: Vec<&str> = antimode.split(',').collect();
                                if antimodes.contains(&val) {
                                    results[result_idx].outlier_count += 1;
                                    results[result_idx].outlier_details.push(OutlierDetail {
                                        value:      val.to_string(),
                                        reason:     "Rare category (antimode)".to_string(),
                                        fence_type: FenceType::Both,
                                        record_no:  record_count,
                                    });
                                }
                            }
                        }
                    }
                },
                _ => {},
            }
        }
    }

    if let Some(pb) = &pb {
        pb.finish_with_message(format!("Processed {} records", record_count));
    }

    results.retain(|result| result.outlier_count > 0);
    Ok(results)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Get stats records
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
        flag_delimiter:       args.flag_delimiter.clone(),
        arg_input:            args.arg_input.clone(),
        flag_memcheck:        false,
    };

    let (_csv_fields, csv_stats) = get_stats_records(&schema_args, StatsMode::FrequencyForceStats)?;

    // Read CSV file using Config
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    // Read the CSV file
    // let mut csv_reader = LazyCsvReader::new(&args.arg_input)
    //     .with_has_header(!args.flag_no_headers)
    //     .with_delimiter(args.flag_delimiter.unwrap_or(Delimiter(b',')).0);

    // let df = csv_reader.finish()?.collect()?;

    // Process selected columns
    // let selected_stats = if let Some(select) = args.flag_select {
    //     let selected: Vec<String> = select.split(',').map(String::from).collect();
    //     csv_stats
    //         .into_iter()
    //         .filter(|stat| selected.contains(&stat.field))
    //         .collect()
    // } else {
    //     csv_stats
    // };

    // Process selected columns
    // let selected_stats: Vec<StatsData> = csv_stats.into_iter().filter(|(_, stat)|
    // sel.contains(&stat.field)).collect();

    let mut selected_stats: Vec<StatsData> = Vec::new();
    for (idx, stat) in csv_stats.iter().enumerate() {
        if sel.contains(&idx) {
            selected_stats.push(stat.clone());
        }
    }
    eprintln!("selected_stats: {:#?}", selected_stats);

    // Process outliers
    let method = FenceType::from_str(args.flag_method.as_deref().unwrap_or("outer"));
    let results = process_outliers(&mut rdr, &selected_stats, method, args.flag_quiet)?;

    // Write results
    let mut wtr: Box<dyn io::Write> = match args.flag_output {
        Some(ref output_path) => Box::new(File::create(Path::new(output_path))?),
        None => Box::new(io::stdout()),
    };

    // Write summary
    if results.is_empty() {
        writeln!(wtr, "No outliers found")?;
    } else {
        writeln!(wtr, "\nOutlier Analysis Summary:")?;
        writeln!(wtr, "=======================")?;

        for result in &results {
            writeln!(wtr, "\nColumn: {} ({})", result.column, result.data_type)?;
            writeln!(wtr, "Found {} outliers", result.outlier_count)?;

            if !args.flag_quiet {
                writeln!(wtr, "\nOutlier Details:")?;
                for detail in &result.outlier_details {
                    writeln!(
                        wtr,
                        "  - Record #{:<6} | Value: {:<20} | Reason: {}",
                        detail.record_no, detail.value, detail.reason
                    )?;
                }
            }
        }
    }

    Ok(())
}
