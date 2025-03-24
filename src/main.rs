#![cfg_attr(
    clippy,
    allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        // things are often more readable this way
        clippy::needless_raw_string_hashes,
        clippy::cast_lossless,
        clippy::module_name_repetitions,
        clippy::type_complexity,
        clippy::zero_prefixed_literal,
        // correctly used
        clippy::enum_glob_use,
        clippy::result_unit_err,
        // not practical
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::struct_excessive_bools,
        // preference
        clippy::doc_markdown,
        clippy::unnecessary_wraps,
        // false positive
        clippy::needless_doctest_main,
        // noisy
        clippy::missing_errors_doc,
        clippy::use_self,
        clippy::cognitive_complexity,
        clippy::option_if_let_else,
    ),
    warn(
        clippy::missing_asserts_for_indexing,
    )
)]

use std::{env, io, time::Instant};

extern crate qsv_docopt as docopt;
use docopt::Docopt;
use rand::Rng;
use serde::Deserialize;

use crate::{
    clitypes::{CURRENT_COMMAND, CliError, CliResult, QsvExitCode},
    config::SPONSOR_MESSAGE,
};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "jemallocator")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod clitypes;
mod cmd;
mod config;
mod index;
mod lookup;
mod odhtcache;
mod select;
mod util;

static USAGE: &str = r#"
Usage:
    qsv <command> [<args>...]
    qsv [options]

Options:
    --list               List all commands available.
    --envlist            List all qsv-relevant environment variables.
    -u, --update         Update qsv to the latest release from GitHub.
    -U, --updatenow      Update qsv to the latest release from GitHub without confirming.
    -h, --help           Display this message
    <command> -h         Display the command help message
    -v, --version        Print version info, mem allocator, features installed, 
                         max_jobs, num_cpus, build info then exit"#;

#[derive(Deserialize)]
struct Args {
    arg_command:    Option<Command>,
    flag_list:      bool,
    flag_envlist:   bool,
    flag_update:    bool,
    flag_updatenow: bool,
}

fn main() -> QsvExitCode {
    util::qsv_custom_panic();

    let mut enabled_commands = String::new();
    #[cfg(all(feature = "apply", feature = "feature_capable"))]
    enabled_commands.push_str("    apply       Apply series of transformations to a column\n");

    enabled_commands.push_str(
        "    behead      Drop header from CSV file
    cat         Concatenate by row or column\n",
    );

    #[cfg(all(feature = "clipboard", feature = "feature_capable"))]
    enabled_commands
        .push_str("    clipboard   Provide input from clipboard or output to clipboard\n");

    enabled_commands.push_str(
        "    count       Count records
    datefmt     Format date/datetime strings
    dedup       Remove redundant rows
    describegpt Infer extended metadata using a LLM
    diff        Find the difference between two CSVs
    edit        Replace a cell's value specified by row and column
    enum        Add a new column enumerating CSV lines
    excel       Exports an Excel sheet to a CSV
    exclude     Excludes the records in one CSV from another
    explode     Explode rows based on some column separator
    extdedup    Remove duplicates rows from an arbitrarily large text file
    extsort     Sort arbitrarily large text file\n",
    );

    #[cfg(all(feature = "fetch", feature = "feature_capable"))]
    enabled_commands.push_str(
        "    fetch       Fetches data from web services for every row using HTTP Get.
    fetchpost   Fetches data from web services for every row using HTTP Post.\n",
    );

    enabled_commands.push_str(
        "    fill        Fill empty values
    fixlengths  Makes all records have same length
    flatten     Show one field per line
    fmt         Format CSV output (change field delimiter)\n",
    );

    #[cfg(all(feature = "foreach", feature = "feature_capable"))]
    enabled_commands.push_str("    foreach     Loop over a CSV file to execute bash commands\n");

    enabled_commands.push_str("    frequency   Show frequency tables\n");

    #[cfg(all(feature = "geocode", not(feature = "lite")))]
    enabled_commands
        .push_str("    geocode     Geocodes a location against the Geonames cities database.\n");

    enabled_commands.push_str(
        "    headers     Show header names
    help        Show this usage message
    index       Create CSV index for faster access
    input       Read CSVs w/ special quoting, skipping, trimming & transcoding rules
    join        Join CSV files\n",
    );

    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    enabled_commands.push_str("    joinp       Join CSV files using the Pola.rs engine\n");

    enabled_commands.push_str(
        "    json        Convert JSON to CSV
    jsonl       Convert newline-delimited JSON files to CSV\n",
    );

    #[cfg(all(feature = "lens", feature = "feature_capable"))]
    enabled_commands.push_str("    lens        View a CSV file interactively\n");

    #[cfg(all(feature = "luau", feature = "feature_capable"))]
    enabled_commands.push_str("    luau        Execute Luau script on CSV data\n");

    enabled_commands.push_str("    partition   Partition CSV data based on a column value\n");

    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    enabled_commands.push_str("    pivotp      Pivots CSV files using the Pola.rs engine\n");

    enabled_commands.push_str("    pro         Interact with the qsv pro API\n");

    #[cfg(all(feature = "prompt", feature = "feature_capable"))]
    enabled_commands.push_str("    prompt      Open a file dialog to pick a file\n");

    enabled_commands.push_str("    pseudo      Pseudonymise the values of a column\n");

    #[cfg(all(feature = "python", feature = "feature_capable"))]
    enabled_commands.push_str("    py          Evaluate a Python expression on CSV data\n");

    enabled_commands.push_str(
        "    rename      Rename the columns of CSV data efficiently
    replace     Replace patterns in CSV data
    reverse     Reverse rows of CSV data
    safenames   Modify a CSV's header names to db-safe names
    sample      Randomly sample CSV data
    schema      Generate JSON Schema from CSV data
    search      Search CSV data with a regex
    searchset   Search CSV data with a regex set
    select      Select, re-order, duplicate or drop columns
    slice       Slice records from CSV
    snappy      Compress/decompress data using the Snappy algorithm
    sniff       Quickly sniff CSV metadata
    sort        Sort CSV data in alphabetical, numerical, reverse or random order
    sortcheck   Check if a CSV is sorted
    split       Split CSV data into many files\n",
    );

    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    enabled_commands.push_str(
        "    sqlp        Run a SQL query against several CSVs using the Pola.rs engine\n",
    );

    enabled_commands.push_str(
        "    stats       Infer data types and compute summary statistics
    table       Align CSV data into columns
    template    Render templates using CSV data
    tojsonl     Convert CSV to newline-delimited JSON\n",
    );

    #[cfg(all(feature = "to", feature = "feature_capable"))]
    enabled_commands
        .push_str("    to          Convert CSVs to PostgreSQL/XLSX/SQLite/Data Package\n");

    enabled_commands.push_str(
        "    transpose   Transpose rows/columns of CSV data
    validate    Validate CSV data for RFC4180-compliance or with JSON Schema",
    );
    let num_commands = enabled_commands.split('\n').count();

    let now = Instant::now();
    #[allow(unused_variables)]
    let (qsv_args, logger_handle) = match util::init_logger() {
        Ok((qsv_args, logger_handle)) => (qsv_args, logger_handle),
        Err(e) => {
            eprintln!("{e}");
            return QsvExitCode::Bad;
        },
    };

    let args: Args = Docopt::new(format!("{USAGE}\n\n{SPONSOR_MESSAGE}"))
        .and_then(|d| {
            d.options_first(true)
                .version(Some(util::version()))
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());

    if util::load_dotenv().is_err() {
        return QsvExitCode::Bad;
    }

    if args.flag_list {
        wout!("Installed commands ({num_commands}):\n{enabled_commands}\n\n{SPONSOR_MESSAGE}");
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    } else if args.flag_envlist {
        util::show_env_vars();
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    }
    if args.flag_update || args.flag_updatenow {
        util::log_end(qsv_args, now);
        if let Err(e) = util::qsv_check_for_update(false, args.flag_updatenow) {
            eprintln!("{e}");
            return QsvExitCode::Bad;
        }
        return QsvExitCode::Good;
    }
    match args.arg_command {
        None => {
            werr!(
                "qsv is a suite of CSV command line utilities.\n\nPlease choose one of the \
                 following {num_commands} commands:\n{enabled_commands}\n\n{SPONSOR_MESSAGE}"
            );

            // if no command is specified, auto-check for updates 50% of the time
            let mut rng = rand::rng(); //DevSkim: ignore DS148264
            if rng.random_range(0..2) == 0 {
                _ = util::qsv_check_for_update(true, false);
            }
            util::log_end(qsv_args, now);
            QsvExitCode::Good
        },
        Some(cmd) => match cmd.run() {
            Ok(()) => {
                util::log_end(qsv_args, now);
                QsvExitCode::Good
            },
            Err(CliError::Help(usage_text)) => {
                wout!("{usage_text}");
                util::log_end(qsv_args, now);
                QsvExitCode::Good
            },
            Err(CliError::Flag(err)) => {
                werr!("{err}");
                util::log_end(qsv_args, now);
                QsvExitCode::IncorrectUsage
            },
            Err(CliError::IncorrectUsage(err)) => {
                werr!("usage error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::IncorrectUsage
            },
            Err(CliError::Csv(err)) => {
                werr!("csv error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
                wwarn!("broken pipe warning");
                util::log_end(qsv_args, now);
                QsvExitCode::Warning
            },
            Err(CliError::Io(err)) => {
                werr!("io error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::NoMatch()) => {
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::Other(msg)) => {
                werr!("{msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::Network(msg)) => {
                werr!("network error: {msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::NetworkError
            },
            Err(CliError::OutOfMemory(msg)) => {
                werr!("out of memory error: {msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::OutOfMemory
            },
            Err(CliError::Encoding(msg)) => {
                werr!("encoding error: {msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::EncodingError
            },
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    #[cfg(all(feature = "apply", feature = "feature_capable"))]
    Apply,
    Behead,
    Cat,
    #[cfg(all(feature = "clipboard", feature = "feature_capable"))]
    Clipboard,
    Count,
    Datefmt,
    Dedup,
    Describegpt,
    Diff,
    Edit,
    Enum,
    Excel,
    Exclude,
    Explode,
    ExtDedup,
    ExtSort,
    #[cfg(all(feature = "fetch", feature = "feature_capable"))]
    Fetch,
    #[cfg(all(feature = "fetch", feature = "feature_capable"))]
    FetchPost,
    Fill,
    FixLengths,
    Flatten,
    Fmt,
    #[cfg(all(feature = "foreach", not(feature = "lite")))]
    ForEach,
    Frequency,
    #[cfg(all(feature = "geocode", feature = "feature_capable"))]
    Geocode,
    Headers,
    Help,
    Index,
    Input,
    Join,
    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    JoinP,
    Json,
    Jsonl,
    #[cfg(all(feature = "lens", feature = "feature_capable"))]
    Lens,
    #[cfg(all(feature = "luau", feature = "feature_capable"))]
    Luau,
    Partition,
    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    PivotP,
    Pro,
    #[cfg(all(feature = "prompt", feature = "feature_capable"))]
    Prompt,
    Pseudo,
    #[cfg(all(feature = "python", feature = "feature_capable"))]
    Py,
    Rename,
    Replace,
    Reverse,
    Safenames,
    Sample,
    Schema,
    Search,
    SearchSet,
    Select,
    Slice,
    Snappy,
    Sniff,
    Sort,
    SortCheck,
    Split,
    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    SqlP,
    Stats,
    Table,
    Template,
    Transpose,
    #[cfg(all(feature = "to", feature = "feature_capable"))]
    To,
    Tojsonl,
    Validate,
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;

        assert!(argv.len() > 1);
        if !argv[1].chars().all(char::is_lowercase) {
            return fail_incorrectusage_clierror!(
                "qsv expects commands in lowercase. Did you mean '{}'?",
                argv[1].to_lowercase()
            );
        }

        CURRENT_COMMAND.get_or_init(|| argv[1].to_lowercase());
        match self {
            Command::Behead => cmd::behead::run(argv),
            #[cfg(all(feature = "apply", feature = "feature_capable"))]
            Command::Apply => cmd::apply::run(argv),
            Command::Cat => cmd::cat::run(argv),
            #[cfg(all(feature = "clipboard", feature = "feature_capable"))]
            Command::Clipboard => cmd::clipboard::run(argv),
            Command::Count => cmd::count::run(argv),
            Command::Datefmt => cmd::datefmt::run(argv),
            Command::Dedup => cmd::dedup::run(argv),
            Command::Describegpt => cmd::describegpt::run(argv),
            Command::Diff => cmd::diff::run(argv),
            Command::Edit => cmd::edit::run(argv),
            Command::Enum => cmd::enumerate::run(argv),
            Command::Excel => cmd::excel::run(argv),
            Command::Exclude => cmd::exclude::run(argv),
            Command::Explode => cmd::explode::run(argv),
            Command::ExtDedup => cmd::extdedup::run(argv),
            Command::ExtSort => cmd::extsort::run(argv),
            #[cfg(all(feature = "fetch", feature = "feature_capable"))]
            Command::Fetch => cmd::fetch::run(argv),
            #[cfg(all(feature = "fetch", feature = "feature_capable"))]
            Command::FetchPost => cmd::fetchpost::run(argv),
            #[cfg(all(feature = "foreach", not(feature = "lite")))]
            Command::ForEach => cmd::foreach::run(argv),
            Command::Fill => cmd::fill::run(argv),
            Command::FixLengths => cmd::fixlengths::run(argv),
            Command::Flatten => cmd::flatten::run(argv),
            Command::Fmt => cmd::fmt::run(argv),
            Command::Frequency => cmd::frequency::run(argv),
            #[cfg(all(feature = "geocode", feature = "feature_capable"))]
            Command::Geocode => cmd::geocode::run(argv),
            Command::Headers => cmd::headers::run(argv),
            Command::Help => {
                wout!("{USAGE}\n\n{SPONSOR_MESSAGE}");
                util::qsv_check_for_update(true, false)?;
                Ok(())
            },
            Command::Index => cmd::index::run(argv),
            Command::Input => cmd::input::run(argv),
            Command::Join => cmd::join::run(argv),
            #[cfg(all(feature = "polars", feature = "feature_capable"))]
            Command::JoinP => cmd::joinp::run(argv),
            Command::Json => cmd::json::run(argv),
            Command::Jsonl => cmd::jsonl::run(argv),
            #[cfg(all(feature = "lens", feature = "feature_capable"))]
            Command::Lens => cmd::lens::run(argv),
            #[cfg(all(feature = "luau", feature = "feature_capable"))]
            Command::Luau => cmd::luau::run(argv),
            Command::Partition => cmd::partition::run(argv),
            #[cfg(all(feature = "polars", feature = "feature_capable"))]
            Command::PivotP => cmd::pivotp::run(argv),
            Command::Pro => cmd::pro::run(argv),
            #[cfg(all(feature = "prompt", feature = "feature_capable"))]
            Command::Prompt => cmd::prompt::run(argv),
            Command::Pseudo => cmd::pseudo::run(argv),
            #[cfg(all(feature = "python", feature = "feature_capable"))]
            Command::Py => cmd::python::run(argv),
            Command::Rename => cmd::rename::run(argv),
            Command::Replace => cmd::replace::run(argv),
            Command::Reverse => cmd::reverse::run(argv),
            Command::Safenames => cmd::safenames::run(argv),
            Command::Sample => cmd::sample::run(argv),
            Command::Schema => cmd::schema::run(argv),
            Command::Search => cmd::search::run(argv),
            Command::SearchSet => cmd::searchset::run(argv),
            Command::Select => cmd::select::run(argv),
            Command::Slice => cmd::slice::run(argv),
            Command::Snappy => cmd::snappy::run(argv),
            Command::Sniff => cmd::sniff::run(argv),
            Command::Sort => cmd::sort::run(argv),
            Command::SortCheck => cmd::sortcheck::run(argv),
            Command::Split => cmd::split::run(argv),
            #[cfg(all(feature = "polars", feature = "feature_capable"))]
            Command::SqlP => cmd::sqlp::run(argv),
            Command::Stats => cmd::stats::run(argv),
            Command::Table => cmd::table::run(argv),
            Command::Template => cmd::template::run(argv),
            Command::Transpose => cmd::transpose::run(argv),
            #[cfg(all(feature = "to", feature = "feature_capable"))]
            Command::To => cmd::to::run(argv),
            Command::Tojsonl => cmd::tojsonl::run(argv),
            Command::Validate => cmd::validate::run(argv),
        }
    }
}
