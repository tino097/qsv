static USAGE: &str = r#"
Read CSV data with special commenting, quoting, trimming, line-skipping &
non UTF-8 encoding rules and transforms it to a "normalized", UTF-8 encoded CSV.

Generally, all qsv commands support basic options like specifying the delimiter
used in CSV data. However, this does not cover all possible types of CSV data. For
example, some CSV files don't use '"' for quotes or use different escaping styles.

Also, CSVs with preamble lines can have them skipped with the --skip-lines & --auto-skip
options. Similarly, --skip-lastlines allows epilogue lines to be skipped.

Finally, non UTF-8 encoded files are "lossy" saved to UTF-8 by default, replacing all
invalid UTF-8 sequences with �. Note though that this is not true transcoding.

If you need to properly transcode non UTF-8 files, you'll need to use a tool like `iconv`
before processing it with qsv - e.g. to convert an ISO-8859-1 encoded file to UTF-8:
    `iconv -f ISO-8859-1 -t UTF-8 input.csv -o utf8_output.csv`.

You can change this behavior with the --encoding-errors option.

See https://github.com/dathere/qsv#utf-8-encoding for more details.

This command is typically used at the beginning of a data pipeline (thus the name `input`)
to normalize & prepare CSVs for further processing with other qsv commands.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_input.rs.

Usage:
    qsv input [options] [<input>]
    qsv input --help

input options:
    --quote <arg>            The quote character to use. [default: "]
    --escape <arg>           The escape character to use. When not specified,
                             quotes are escaped by doubling them.
    --no-quoting             Disable quoting completely when reading CSV data.
    --quote-style <arg>      The quoting style to use when writing CSV data.
                             Possible values: all, necessary, nonnumeric and never.
                              All: Quotes all fields.
                              Necessary: Quotes fields only when necessary - when fields
                               contain a quote, delimiter or record terminator. 
                               Quotes are also necessary when writing an empty record 
                               (which is indistinguishable from a record with one empty field).
                              NonNumeric: Quotes all fields that are non-numeric.
                              Never: Never write quotes. Even if it produces invalid CSV.
                             [default: necessary]
    --skip-lines <arg>       The number of preamble lines to skip.
    --auto-skip              Sniffs a CSV for preamble lines and automatically
                             skips them. Takes precedence over --skip-lines option.
                             Does not work with <stdin>.
    --skip-lastlines <arg>   The number of epilogue lines to skip.
    --trim-headers           Trim leading & trailing whitespace & quotes from header values.
    --trim-fields            Trim leading & trailing whitespace from field values.
    --comment <char>         The comment character to use. When set, lines
                             starting with this character will be skipped.
    --encoding-errors <arg>  How to handle UTF-8 encoding errors.
                             Possible values: replace, skip, strict.
                               replace: Replace invalid UTF-8 sequences with �.
                                  skip: Fields with encoding errors are "<SKIPPED>".
                                strict: Fail on any encoding errors.
                             [default: replace]

Common options:
    -h, --help               Display this message
    -o, --output <file>      Write output to <file> instead of stdout.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

use std::{env, str::FromStr};

use log::{debug, info, warn};
use serde::Deserialize;
use strum_macros::EnumString;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive)]
#[allow(non_camel_case_types)]
enum EncodingHandling {
    Replace,
    Skip,
    Strict,
}

#[derive(Deserialize)]
struct Args {
    arg_input:            Option<String>,
    flag_output:          Option<String>,
    flag_delimiter:       Option<Delimiter>,
    flag_quote:           Delimiter,
    flag_escape:          Option<Delimiter>,
    flag_no_quoting:      bool,
    flag_quote_style:     String,
    flag_skip_lines:      Option<u64>,
    flag_skip_lastlines:  Option<u64>,
    flag_auto_skip:       bool,
    flag_trim_headers:    bool,
    flag_trim_fields:     bool,
    flag_comment:         Option<char>,
    flag_encoding_errors: String,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let trim_setting = match (args.flag_trim_headers, args.flag_trim_fields) {
        (false, false) => csv::Trim::None,
        (true, true) => csv::Trim::All,
        (true, false) => csv::Trim::Headers,
        (false, true) => csv::Trim::Fields,
    };

    let Ok(encode_handler) = EncodingHandling::from_str(&args.flag_encoding_errors) else {
        return fail_incorrectusage_clierror!(
            "Invalid --encoding-errors option: {}. Valid values: replace, skip, strict.",
            args.flag_encoding_errors
        );
    };

    if args.flag_auto_skip {
        std::env::set_var("QSV_SNIFF_PREAMBLE", "1");
    }

    let comment_char: Option<u8> = if let Ok(cmt_char) = env::var("QSV_COMMENT_CHAR") {
        Some(cmt_char.as_bytes().first().unwrap().to_owned())
    } else {
        args.flag_comment.map(|char| char as u8)
    };

    let mut rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .quote(args.flag_quote.as_byte())
        .comment(comment_char)
        .trim(trim_setting);
    if args.flag_auto_skip {
        std::env::remove_var("QSV_SNIFF_PREAMBLE");
    }
    let mut wconfig = Config::new(args.flag_output.as_ref());

    if let Some(escape) = args.flag_escape {
        rconfig = rconfig.escape(Some(escape.as_byte())).double_quote(false);
    }
    if args.flag_no_quoting {
        rconfig = rconfig.quoting(false);
    }
    wconfig = wconfig.quote_style(match args.flag_quote_style.as_str() {
        "necessary" => csv::QuoteStyle::Necessary,
        "all" => csv::QuoteStyle::Always,
        "nonnumeric" => csv::QuoteStyle::NonNumeric,
        "never" => csv::QuoteStyle::Never,
        _ => {
            return fail_incorrectusage_clierror!(
                "Invalid --quote-style option: {}. Valid values: all, necessary, nonnumeric, \
                 never.",
                args.flag_quote_style
            );
        },
    });

    if args.flag_auto_skip || args.flag_skip_lines.is_some() || args.flag_skip_lastlines.is_some() {
        rconfig = rconfig.flexible(true);
    }

    let mut total_lines = 0_u64;
    if let Some(skip_llines) = args.flag_skip_lastlines {
        // use the regular count_rows to get the row_count
        // as Polars doesn't support skipping last lines
        let row_count = util::count_rows_regular(&rconfig)?;
        if skip_llines > row_count {
            return fail_incorrectusage_clierror!(
                "--skip-lastlines: {skip_llines} is greater than row_count: {row_count}."
            );
        }
        info!("Set to skip last {skip_llines} lines...");
        total_lines = row_count.saturating_sub(skip_llines);
    }

    let mut rdr = rconfig.reader()?;
    let mut wtr = wconfig.writer()?;
    let mut row = csv::ByteRecord::new();
    let mut str_row = csv::StringRecord::new();

    let preamble_rows: u64 = if args.flag_auto_skip {
        info!("auto-skip on...");
        rconfig.preamble_rows
    } else if args.flag_skip_lines.is_some() {
        // safety: we already checked that skip_lines is some
        args.flag_skip_lines.unwrap()
    } else {
        0
    };

    if preamble_rows > 0 {
        info!("skipping {preamble_rows} preamble rows...");
        for _i in 1..=preamble_rows {
            rdr.read_byte_record(&mut row)?;
        }
        if total_lines.saturating_sub(preamble_rows) > 0 {
            total_lines -= preamble_rows;
        }
    }
    // the first rdr record is the header, since we have no_headers = true.
    // If trim_setting is equal to Headers or All, we "manually" trim the first record
    if trim_setting == csv::Trim::Headers || trim_setting == csv::Trim::All {
        info!("trimming headers...");
        rdr.read_byte_record(&mut row)?;
        row.trim();

        for field in &row {
            // we also trim excess quotes from the header, to be consistent with safenames
            str_row.push_field(String::from_utf8_lossy(field).trim_matches('"'));
        }
        wtr.write_record(&str_row)?;
    }

    let mut idx = 1_u64;
    let mut not_utf8 = false;
    let mut lossy_field;
    let debug_log = log::log_enabled!(log::Level::Debug);

    'main: loop {
        match rdr.read_byte_record(&mut row) {
            Ok(moredata) => {
                if !moredata {
                    break 'main;
                }
            },
            Err(e) => {
                return fail_clierror!("Invalid CSV. Last valid row ({idx}): {e}");
            },
        };

        str_row.clear();
        for field in &row {
            if let Ok(utf8_field) = simdutf8::basic::from_utf8(field) {
                str_row.push_field(utf8_field);
            } else {
                match encode_handler {
                    EncodingHandling::Replace => {
                        lossy_field = String::from_utf8_lossy(field);
                        str_row.push_field(&lossy_field);
                        if debug_log {
                            debug!("REPLACE: Invalid UTF8 - row {idx} in \"{lossy_field}\".");
                        }
                        not_utf8 = true;
                    },
                    EncodingHandling::Skip => {
                        str_row.push_field("<SKIPPED>");
                        if debug_log {
                            lossy_field = String::from_utf8_lossy(field);
                            debug!("SKIPPED: Invalid UTF8 - row {idx} in \"{lossy_field}\".");
                        }
                        not_utf8 = true;
                    },
                    EncodingHandling::Strict => {
                        lossy_field = String::from_utf8_lossy(field);
                        return fail_encoding_clierror!(
                            "STRICT. Invalid UTF8 - row {idx} in \"{lossy_field}\"."
                        );
                    },
                }
            };
        }
        wtr.write_record(&str_row)?;
        idx += 1;

        if total_lines > 0 && idx > total_lines {
            break 'main;
        }
    }

    if not_utf8 {
        match encode_handler {
            EncodingHandling::Replace => warn!(
                "Some rows contained invalid UTF-8 sequences. These sequences were replaced with \
                 the U+FFFD (�) replacement character."
            ),
            EncodingHandling::Skip => warn!(
                "Some fields contained invalid UTF-8 sequences. These fields set to \"<SKIPPED>\"."
            ),
            // STRICT is unreachable because we return early if we encounter invalid UTF-8
            EncodingHandling::Strict => unreachable!(),
        }
    }

    info!("Wrote {} rows...", idx - 1);
    Ok(wtr.flush()?)
}
