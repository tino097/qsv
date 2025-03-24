static USAGE: &str = r#"
Returns the rows in the range specified (starting at 0, half-open interval).
The range does not include headers.

If the start of the range isn't specified, then the slice starts from the first
record in the CSV data.

If the end of the range isn't specified, then the slice continues to the last
record in the CSV data.

This operation can be made much faster by creating an index with 'qsv index'
first. With an index, the command requires parsing just the rows that are
sliced. Without an index, all rows up to the first row in the slice must be
parsed.

Usage:
    qsv slice [options] [<input>]
    qsv slice --help

slice options:
    -s, --start <arg>      The index of the record to slice from.
                           If negative, starts from the last record.
    -e, --end <arg>        The index of the record to slice to.
    -l, --len <arg>        The length of the slice (can be used instead
                           of --end).
    -i, --index <arg>      Slice a single record (shortcut for -s N -l 1).
                           If negative, starts from the last record.
    --json                 Output the result as JSON. Fields are written
                           as key-value pairs. The key is the column name.
                           The value is the field value. The output is a
                           JSON array. If --no-headers is set, then
                           the keys are the column indices (zero-based).
    --invert               slice all records EXCEPT those in the specified range.

Examples:
    # Slice from the 3rd record to the end
    qsv slice --start 2 data.csv

    # Slice the first three records
    qsv slice --start 0 --end 2 data.csv
    qsv slice --len 3 data.csv
    qsv slice -l 3 data.csv

    # Slice the last record
    qsv slice -s -1 data.csv

    # Slice the last 10 records
    qsv slice -s -10 data.csv

    # Get everything except the last 10 records
    qsv slice -s -10 --invert data.csv

    # Slice the first three records of the last 10 records
    qsv slice -s -10 -l 3 data.csv

    # Slice the second record
    qsv slice --index 1 data.csv
    qsv slice -i 1 data.csv

    # Slice from the second record, two records
    qsv slice -s 1 --len 2 data.csv

    # Slice records 10 to 20 as JSON   
    qsv slice -s 9 -e 19 --json data.csv
    qsv slice -s 9 -l 10 --json data.csv

    # Slice records 1 to 9 and 21 to the end as JSON
    qsv slice -s 9 -l 10 --invert --json data.csv

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Otherwise, the first row will always
                           appear in the output as the header row.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    index::Indexed,
    util,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_start:      Option<isize>,
    flag_end:        Option<usize>,
    flag_len:        Option<usize>,
    flag_index:      Option<isize>,
    flag_json:       bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_invert:     bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

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

    args.arg_input = Some(input_filename);

    match args.rconfig().indexed()? {
        Some(idxed) => args.with_index(idxed),
        _ => args.no_index(),
    }
}

impl Args {
    fn no_index(&self) -> CliResult<()> {
        let mut rdr = self.rconfig().reader()?;

        let (start, end) = self.range()?;
        if self.flag_json {
            let headers = rdr.byte_headers()?.clone();
            let records = rdr.byte_records().enumerate().filter_map(move |(i, r)| {
                let should_include = if self.flag_invert {
                    i < start || i >= end
                } else {
                    i >= start && i < end
                };
                if should_include {
                    Some(r.unwrap())
                } else {
                    None
                }
            });
            util::write_json(
                self.flag_output.as_ref(),
                self.flag_no_headers,
                &headers,
                records,
            )
        } else {
            let mut wtr = self.wconfig().writer()?;
            self.rconfig().write_headers(&mut rdr, &mut wtr)?;

            for (i, r) in rdr.byte_records().enumerate() {
                if self.flag_invert == (i < start || i >= end) {
                    wtr.write_byte_record(&r?)?;
                }
            }
            Ok(wtr.flush()?)
        }
    }

    fn with_index(&self, mut indexed_file: Indexed<fs::File, fs::File>) -> CliResult<()> {
        let (start, end) = self.range()?;
        if end - start == 0 && !self.flag_invert {
            return Ok(());
        }

        if self.flag_json {
            let headers = indexed_file.byte_headers()?.clone();
            let total_rows = util::count_rows(&self.rconfig())?;
            let records = if self.flag_invert {
                let mut records: Vec<csv::ByteRecord> =
                    Vec::with_capacity(start + (total_rows as usize - end));
                // Get records before start
                indexed_file.seek(0)?;
                for r in indexed_file.byte_records().take(start) {
                    records.push(r.unwrap());
                }

                // Get records after end
                indexed_file.seek(end as u64)?;
                for r in indexed_file.byte_records().take(total_rows as usize - end) {
                    records.push(r.unwrap());
                }
                records
            } else {
                indexed_file.seek(start as u64)?;
                indexed_file
                    .byte_records()
                    .take(end - start)
                    .map(|r| r.unwrap())
                    .collect::<Vec<_>>()
            };
            util::write_json(
                self.flag_output.as_ref(),
                self.flag_no_headers,
                &headers,
                records.into_iter(),
            )
        } else {
            let mut wtr = self.wconfig().writer()?;
            self.rconfig().write_headers(&mut *indexed_file, &mut wtr)?;

            let total_rows = util::count_rows(&self.rconfig())? as usize;
            if self.flag_invert {
                // Get records before start
                indexed_file.seek(0)?;
                for r in indexed_file.byte_records().take(start) {
                    wtr.write_byte_record(&r?)?;
                }

                // Get records after end
                indexed_file.seek(end as u64)?;
                for r in indexed_file.byte_records().take(total_rows - end) {
                    wtr.write_byte_record(&r?)?;
                }
            } else {
                indexed_file.seek(start as u64)?;
                for r in indexed_file.byte_records().take(end - start) {
                    wtr.write_byte_record(&r?)?;
                }
            }
            Ok(wtr.flush()?)
        }
    }

    fn range(&self) -> CliResult<(usize, usize)> {
        let mut start = None;
        if let Some(start_arg) = self.flag_start {
            if start_arg < 0 {
                start = Some(
                    (util::count_rows(&self.rconfig())? as usize)
                        .abs_diff(start_arg.unsigned_abs()),
                );
            } else {
                start = Some(start_arg as usize);
            }
        }
        let index = if let Some(flag_index) = self.flag_index {
            if flag_index < 0 {
                let index = (util::count_rows(&self.rconfig())? as usize)
                    .abs_diff(flag_index.unsigned_abs());
                Some(index)
            } else {
                Some(flag_index as usize)
            }
        } else {
            None
        };
        Ok(util::range(start, self.flag_end, self.flag_len, index)?)
    }

    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
    }

    fn wconfig(&self) -> Config {
        Config::new(self.flag_output.as_ref())
    }
}
