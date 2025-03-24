static USAGE: &str = r#"
Filters CSV data by whether the given regex matches a row.

The regex is applied to selected field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found, unless the '--not-one' flag is used.

When --quick is enabled, no output is produced and exitcode 0 is returned on 
the first match.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_search.rs.

Usage:
    qsv search [options] <regex> [<input>]
    qsv search --help

search arguments:
    <regex>                Regular expression to match. Uses Rust regex syntax.
                           See https://docs.rs/regex/latest/regex/index.html#syntax
                           or https://regex101.com with the Rust flavor for more info.
    <input>                The CSV file to read. If not given, reads from stdin.

search options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    --literal              Treat the regex as a literal string. This allows
                           you to search for exact matches that even contain
                           regex special characters.
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -v, --invert-match     Select only rows that did not match
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.
    -f, --flag <column>    If given, the command will not filter rows
                           but will instead flag the found rows in a new
                           column named <column>, with the row numbers
                           of the matched rows and 0 for the non-matched rows.
                           If column is named M, only the M column will be written
                           to the output, and only matched rows are returned.
    -Q, --quick            Return on first match with an exitcode of 0, returning
                           the row number of the first match to stderr.
                           Return exit code 1 if no match is found.
                           No output is produced.
    --preview-match <arg>  Preview the first N matches or all the matches found in
                           N milliseconds, whichever occurs first. Returns the preview to
                           stderr. Output is still written to stdout or --output as usual.
    -c, --count            Return number of matches to stderr.
    --size-limit <mb>      Set the approximate size limit (MB) of the compiled
                           regular expression. If the compiled expression exceeds this 
                           number, then a compilation error is returned.
                           Modify this only if you're getting regular expression
                           compilation errors. [default: 50]
    --dfa-size-limit <mb>  Set the approximate size of the cache (MB) used by the regular
                           expression engine's Discrete Finite Automata.
                           Modify this only if you're getting regular expression
                           compilation errors. [default: 10]
    --json                 Output the result as JSON. Fields are written
                           as key-value pairs. The key is the column name.
                           The value is the field value. The output is a
                           JSON array. If --no-headers is set, then
                           the keys are the column indices (zero-based).
                           Automatically sets --quiet.
    --not-one              Use exit code 0 instead of 1 for no match found.
                           
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
    -q, --quiet            Do not return number of matches to stderr.
"#;

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use log::info;
use regex::bytes::RegexBuilder;
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    select::SelectColumns,
    util,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    arg_regex:           String,
    flag_literal:        bool,
    flag_select:         SelectColumns,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_invert_match:   bool,
    flag_unicode:        bool,
    flag_ignore_case:    bool,
    flag_flag:           Option<String>,
    flag_size_limit:     usize,
    flag_dfa_size_limit: usize,
    flag_json:           bool,
    flag_not_one:        bool,
    flag_quick:          bool,
    flag_preview_match:  Option<u64>,
    flag_count:          bool,
    flag_progressbar:    bool,
    flag_quiet:          bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let regex_unicode = if util::get_envvar_flag("QSV_REGEX_UNICODE") {
        true
    } else {
        args.flag_unicode
    };

    let arg_regex = if args.flag_literal {
        regex::escape(&args.arg_regex)
    } else {
        args.arg_regex.clone()
    };

    let pattern = RegexBuilder::new(&arg_regex)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    // args struct booleans in hot loop assigned to local variables
    // to help the compiler optimize the code & hopefully use registers
    let flag_quick = args.flag_quick;
    let flag_quiet = args.flag_quiet || args.flag_json;
    let flag_json = args.flag_json;
    let flag_no_headers = args.flag_no_headers;
    let flag_not_one = args.flag_not_one;

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let mut json_wtr = if flag_json {
        util::create_json_writer(args.flag_output.as_ref(), DEFAULT_WTR_BUFFER_CAPACITY * 4)?
    } else {
        Box::new(std::io::sink())
    };

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let mut matches_only = false;

    let flag_flag = args.flag_flag.is_some_and(|column_name| {
        // if --flag column is "M", then we only output the M column
        if column_name == "M" {
            headers.clear();
            matches_only = true;
        }
        headers.push_field(column_name.as_bytes());
        true
    });

    if !rconfig.no_headers && !flag_quick && !flag_json {
        wtr.write_record(&headers)?;
    }

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let mut record = csv::ByteRecord::new();
    let mut flag_rowi: u64 = 0;
    let mut match_ctr: u64 = 0;
    let mut row_ctr: u64 = 0;
    let mut m;
    let invert_match = args.flag_invert_match;

    #[allow(unused_assignments)]
    let mut matched_rows = String::with_capacity(20); // to save on allocs

    let mut json_started = false;
    let mut is_first = true;

    // if preview_match is set, we do an initial loop for the
    // first N matches or all the matches found in N milliseconds
    if let Some(preview_match) = args.flag_preview_match {
        // create a stderr CSV writer
        let mut stderr_wtr = csv::WriterBuilder::new().from_writer(std::io::stderr());

        let mut stderr_jsonwtr = if flag_json {
            util::create_json_writer(Some(&"stderr".to_string()), DEFAULT_WTR_BUFFER_CAPACITY * 4)?
        } else {
            Box::new(std::io::sink())
        };

        if !rconfig.no_headers && !flag_quick && !flag_json {
            stderr_wtr.write_record(&headers)?;
        }

        // write the JSON array start
        if flag_json {
            json_wtr.write_all(b"[")?;
            stderr_jsonwtr.write_all(b"[")?;
            json_started = true;
        }

        let mut preview_match_ctr = 0;
        let mut is_first_stderr = true;
        let mut match_row;

        // make preview-match timeout at least 10 ms
        let preview_timeout = std::time::Duration::from_millis(std::cmp::max(preview_match, 10));
        let start_time = std::time::Instant::now();
        while rdr.read_byte_record(&mut record)? {
            row_ctr += 1;

            m = sel.select(&record).any(|f| pattern.is_match(f));
            if invert_match {
                m = !m;
            }
            if m {
                preview_match_ctr += 1;
                if flag_quick {
                    break;
                }
                if preview_match_ctr <= preview_match {
                    if flag_json {
                        util::write_json_record(
                            &mut stderr_jsonwtr,
                            flag_no_headers,
                            &headers,
                            &record,
                            &mut is_first_stderr,
                        )?;
                        util::write_json_record(
                            &mut json_wtr,
                            flag_no_headers,
                            &headers,
                            &record,
                            &mut is_first,
                        )?;
                    } else {
                        stderr_wtr.write_record(&record)?;
                        wtr.write_byte_record(&record)?;
                    }
                }
                if preview_match_ctr >= preview_match || start_time.elapsed() > preview_timeout {
                    break;
                }
            }

            if flag_flag {
                flag_rowi += 1;
                match_row = if m {
                    itoa::Buffer::new()
                        .format(flag_rowi)
                        .clone_into(&mut matched_rows);
                    matched_rows.as_bytes()
                } else {
                    b"0"
                };
                if matches_only {
                    if match_row == b"0" {
                        continue;
                    }
                    record.clear();
                }
                record.push_field(match_row);
                if flag_json {
                    util::write_json_record(
                        &mut json_wtr,
                        flag_no_headers,
                        &headers,
                        &record,
                        &mut is_first,
                    )?;
                } else {
                    wtr.write_byte_record(&record)?;
                }
            } else if m {
                if flag_json {
                    util::write_json_record(
                        &mut json_wtr,
                        flag_no_headers,
                        &headers,
                        &record,
                        &mut is_first,
                    )?;
                } else {
                    wtr.write_byte_record(&record)?;
                }
            }
        }
        match_ctr = preview_match_ctr;
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(row_ctr);
        }
        stderr_wtr.flush()?;
        if !flag_quiet {
            eprintln!(
                "Previewed {} matches in {} initial records in {} ms.",
                preview_match_ctr,
                row_ctr,
                start_time.elapsed().as_millis()
            );
        }
        if flag_json {
            stderr_jsonwtr.write_all(b"]")?;
            stderr_jsonwtr.flush()?;
        }
    }

    if flag_json && !json_started {
        json_wtr.write_all(b"[")?;
    }

    let mut match_row;
    while rdr.read_byte_record(&mut record)? {
        row_ctr += 1;

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }
        m = sel.select(&record).any(|f| pattern.is_match(f));
        if invert_match {
            m = !m;
        }
        if m {
            match_ctr += 1;
            if flag_quick {
                break;
            }
        }

        if flag_flag {
            flag_rowi += 1;
            match_row = if m {
                itoa::Buffer::new()
                    .format(flag_rowi)
                    .clone_into(&mut matched_rows);
                matched_rows.as_bytes()
            } else {
                b"0"
            };
            if matches_only {
                if match_row == b"0" {
                    continue;
                }
                record.clear();
            }
            record.push_field(match_row);
            if flag_json {
                util::write_json_record(
                    &mut json_wtr,
                    flag_no_headers,
                    &headers,
                    &record,
                    &mut is_first,
                )?;
            } else {
                wtr.write_byte_record(&record)?;
            }
        } else if m {
            if flag_json {
                util::write_json_record(
                    &mut json_wtr,
                    flag_no_headers,
                    &headers,
                    &record,
                    &mut is_first,
                )?;
            } else {
                wtr.write_byte_record(&record)?;
            }
        }
    }
    if flag_json {
        json_wtr.write_all(b"]")?;
        json_wtr.flush()?;
    } else {
        wtr.flush()?;
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            " - {} matches found in {} records.",
            HumanCount(match_ctr),
            HumanCount(progress.length().unwrap()),
        ));
        util::finish_progress(&progress);
    }

    if args.flag_count && !args.flag_quick {
        if !flag_quiet {
            eprintln!("{match_ctr}");
        }
        info!("matches: {match_ctr}");
    }

    if match_ctr == 0 && !flag_not_one {
        return Err(CliError::NoMatch());
    } else if args.flag_quick {
        if !args.flag_quiet {
            eprintln!("{row_ctr}");
        }
        info!("quick search first match at {row_ctr}");
    }

    Ok(())
}
