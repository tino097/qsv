static USAGE: &str = r#"
Randomly samples CSV data.

It supports three sampling methods:
- INDEXED: the default sampling method when an index is present.
  Uses random I/O to sample efficiently, as it only visits records selected
  by random indexing, using CONSTANT memory proportional to the <sample-size>.
  The number of records in the output is exactly equal to the <sample-size>.

- RESERVOIR: the default sampling method when NO INDEX is present.
  Visits every CSV record exactly once, using memory proportional to <sample-size>.
  The number of records in the output is exactly equal to the <sample-size>.
  https://en.wikipedia.org/wiki/Reservoir_sampling

- BERNOULLI: the sampling method when the --bernoulli option is specified.
  Visits every CSV record exactly once and selects records with a given probability
  as specified by the <sample-size> argument. It uses constant memory.
  The number of records in the output follows a binomial distribution with
  parameters n (input size) and p (sample-size as probability).
  https://en.wikipedia.org/wiki/Bernoulli_sampling

Supports sampling from CSVs on remote URLs.

This command is intended to provide a means to sample from a CSV data set that
is too big to fit into memory (for example, for use with commands like
'qsv stats' with the '--everything' option). 

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_sample.rs.

Usage:
    qsv sample [options] <sample-size> [<input>]
    qsv sample --help

sample arguments:
    <input>                The CSV file to sample. This can be a local file,
                           stdin, or a URL (http and https schemes supported).

    <sample-size>          When using INDEXED or RESERVOIR sampling, the number of records to sample.
                           When using BERNOULLI sampling, the probability of selecting each record
                           (between 0 and 1).

sample options:
    --seed <number>        Random Number Generator (RNG) seed.
    --rng <kind>           The Random Number Generator (RNG) algorithm to use.
                           Three RNGs are supported:
                            - standard: Use the standard RNG.
                              1.5 GB/s throughput.
                            - faster: Use faster RNG using the Xoshiro256Plus algorithm.
                              8 GB/s throughput.
                            - cryptosecure: Use cryptographically secure HC128 algorithm.
                              Recommended by eSTREAM (https://www.ecrypt.eu.org/stream/).
                              2.1 GB/s throughput though slow initialization.
                           [default: standard]
    --bernoulli            Use Bernoulli sampling instead of indexed or reservoir sampling.
                           When this flag is set, the sample-size must be between
                           0 and 1 and represents the probability of selecting each record.

                           REMOTE FILE OPTIONS:
    --user-agent <agent>   Specify custom user agent to use when the input is a URL.
                           It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    --timeout <secs>       Timeout for downloading URLs in seconds.
                           [default: 30]
    --max-size <mb>        Maximum size of the file to download in MB before sampling.
                           Will download the entire file if not specified.
                           If the CSV is partially downloaded, the sample will be taken
                           only from the downloaded portion.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will be considered as part of
                           the population to sample from. (When not set, the
                           first row is the header row and will always appear
                           in the output.)
    -d, --delimiter <arg>  The field delimiter for reading/writing CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{io, str::FromStr};

use rand::{
    distr::{Bernoulli, Distribution},
    rngs::StdRng,
    Rng, SeedableRng,
};
use rand_hc::Hc128Rng;
use rand_xoshiro::Xoshiro256Plus;
use serde::Deserialize;
use strum_macros::EnumString;
use tempfile::NamedTempFile;
use url::Url;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_sample_size: f64,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_seed:       Option<u64>,
    flag_rng:        String,
    flag_user_agent: Option<String>,
    flag_timeout:    Option<u16>,
    flag_max_size:   Option<u64>,
    flag_bernoulli:  bool,
}

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
enum RngKind {
    Standard,
    Faster,
    Cryptosecure,
}

// trait to handle different RNG types
trait RngProvider: Sized {
    type RngType: Rng + SeedableRng;

    fn get_name() -> &'static str;

    fn create(seed: Option<u64>) -> Self::RngType {
        if let Some(seed) = seed {
            Self::RngType::seed_from_u64(seed) // DevSkim: ignore DS148264
        } else {
            Self::RngType::from_os_rng()
        }
    }
}

// Implement for each RNG type
struct StandardRng;
impl RngProvider for StandardRng {
    type RngType = StdRng;

    fn get_name() -> &'static str {
        "standard"
    }
}

struct FasterRng;
impl RngProvider for FasterRng {
    type RngType = Xoshiro256Plus;

    fn get_name() -> &'static str {
        "faster"
    }
}

struct CryptoRng;
impl RngProvider for CryptoRng {
    type RngType = Hc128Rng;

    fn get_name() -> &'static str {
        "cryptosecure"
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let Ok(rng_kind) = RngKind::from_str(&args.flag_rng) else {
        return fail_incorrectusage_clierror!(
            "Invalid RNG algorithm `{}`. Supported RNGs are: standard, faster, cryptosecure.",
            args.flag_rng
        );
    };

    let temp_download = NamedTempFile::new()?;

    args.arg_input = match args.arg_input {
        Some(uri) if Url::parse(&uri).is_ok() && uri.starts_with("http") => {
            let max_size_bytes = args.flag_max_size.map(|mb| mb * 1024 * 1024);

            // its a remote file, download it first
            let future = util::download_file(
                &uri,
                temp_download.path().to_path_buf(),
                false,
                args.flag_user_agent,
                args.flag_timeout,
                max_size_bytes,
            );
            tokio::runtime::Runtime::new()?.block_on(future)?;
            // safety: temp_download is a NamedTempFile, so we know can unwrap.to_string
            Some(temp_download.path().to_str().unwrap().to_string())
        },
        Some(uri) => Some(uri), // local file
        None => None,
    };

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .flexible(true)
        .skip_format_check(true);

    let mut sample_size = args.arg_sample_size;

    let mut wtr = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_delimiter)
        .writer()?;

    if args.flag_bernoulli {
        if sample_size >= 1.0 || sample_size <= 0.0 {
            return fail_incorrectusage_clierror!(
                "Bernoulli sampling requires a probability between 0 and 1"
            );
        }

        let mut rdr = rconfig.reader()?;
        rconfig.write_headers(&mut rdr, &mut wtr)?;
        sample_bernoulli(&mut rdr, &mut wtr, sample_size, args.flag_seed, &rng_kind)?;
    } else if let Some(mut idx) = rconfig.indexed()? {
        // an index is present, so use random indexing
        #[allow(clippy::cast_precision_loss)]
        if sample_size < 1.0 {
            sample_size *= idx.count() as f64;
        }
        rconfig.write_headers(&mut *idx, &mut wtr)?;

        let sample_count = sample_size as usize;
        let total_count = idx.count().try_into().unwrap();

        match rng_kind {
            RngKind::Standard => {
                log::info!("doing standard INDEXED sampling...");
                let mut rng = StandardRng::create(args.flag_seed);
                sample_indices(&mut rng, total_count, sample_count, |i| {
                    idx.seek(i as u64)?;
                    Ok(wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?)
                })?;
            },
            RngKind::Faster => {
                log::info!("doing --faster INDEXED sampling...");
                let mut rng = FasterRng::create(args.flag_seed);
                sample_indices(&mut rng, total_count, sample_count, |i| {
                    idx.seek(i as u64)?;
                    Ok(wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?)
                })?;
            },
            RngKind::Cryptosecure => {
                log::info!("doing --cryptosecure INDEXED sampling...");
                let mut rng = CryptoRng::create(args.flag_seed);
                sample_indices(&mut rng, total_count, sample_count, |i| {
                    idx.seek(i as u64)?;
                    Ok(wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?)
                })?;
            },
        }
    } else {
        // bernoulli sampling is not specified nor is an index present
        // so we do reservoir sampling
        #[allow(clippy::cast_precision_loss)]
        if sample_size < 1.0 {
            let Ok(row_count) = util::count_rows(&rconfig) else {
                return fail!("Cannot get rowcount. Percentage sampling requires a rowcount.");
            };
            sample_size *= row_count as f64;
        }
        let mut rdr = rconfig.reader()?;
        rconfig.write_headers(&mut rdr, &mut wtr)?;

        sample_reservoir(
            &mut rdr,
            &mut wtr,
            sample_size as u64,
            args.flag_seed,
            &rng_kind,
        )?;
    }

    Ok(wtr.flush()?)
}

fn sample_reservoir<R: io::Read, W: io::Write>(
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    sample_size: u64,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    let mut reservoir = Vec::with_capacity(sample_size as usize);
    let mut records = rdr.byte_records().enumerate();

    // Pre-fill reservoir
    // Note that we use by_ref() to avoid consuming the iterator
    // and we only take the first sample_size records
    for (_, row) in records.by_ref().take(sample_size as usize) {
        reservoir.push(row?);
    }

    match rng_kind {
        RngKind::Standard => {
            do_reservoir_sampling::<StandardRng>(&mut records, &mut reservoir, sample_size, seed)
        },
        RngKind::Faster => {
            do_reservoir_sampling::<FasterRng>(&mut records, &mut reservoir, sample_size, seed)
        },
        RngKind::Cryptosecure => {
            do_reservoir_sampling::<CryptoRng>(&mut records, &mut reservoir, sample_size, seed)
        },
    }?;

    // Write the reservoir to output
    for record in reservoir {
        wtr.write_byte_record(&record)?;
    }

    Ok(())
}

// Generic reservoir sampling implementation using constant memory
fn do_reservoir_sampling<T: RngProvider>(
    records: &mut impl Iterator<Item = (usize, Result<csv::ByteRecord, csv::Error>)>,
    reservoir: &mut [csv::ByteRecord],
    sample_size: u64,
    seed: Option<u64>,
) -> CliResult<()> {
    log::info!("doing {} RESERVOIR sampling...", T::get_name());
    let mut rng = T::create(seed);
    let mut random_idx: usize;

    // Process remaining records using Algorithm R (Robert Floyd)
    for (i, row) in records {
        random_idx = rng.random_range(0..=i);
        if random_idx < sample_size as usize {
            unsafe {
                *reservoir.get_unchecked_mut(random_idx) = row?;
            }
        }
    }
    Ok(())
}

fn sample_bernoulli<R: io::Read, W: io::Write>(
    rdr: &mut csv::Reader<R>,
    wtr: &mut csv::Writer<W>,
    probability: f64,
    seed: Option<u64>,
    rng_kind: &RngKind,
) -> CliResult<()> {
    let mut records = rdr.byte_records();

    match rng_kind {
        RngKind::Standard => {
            do_bernoulli_sampling::<StandardRng>(&mut records, wtr, probability, seed)
        },
        RngKind::Faster => do_bernoulli_sampling::<FasterRng>(&mut records, wtr, probability, seed),
        RngKind::Cryptosecure => {
            do_bernoulli_sampling::<CryptoRng>(&mut records, wtr, probability, seed)
        },
    }
}

// Generic bernoulli sampling implementation using constant memory
fn do_bernoulli_sampling<T: RngProvider>(
    records: &mut impl Iterator<Item = Result<csv::ByteRecord, csv::Error>>,
    wtr: &mut csv::Writer<impl io::Write>,
    probability: f64,
    seed: Option<u64>,
) -> CliResult<()> {
    log::info!("doing {} BERNOULLI sampling...", T::get_name());
    let mut rng = T::create(seed);

    let dist =
        Bernoulli::new(probability).map_err(|_| "probability must be between 0.0 and 1.0")?;

    for row in records {
        if dist.sample(&mut rng) {
            wtr.write_byte_record(&row?)?;
        }
    }
    Ok(())
}

// Helper function to sample indices using constant memory
fn sample_indices<F>(
    rng: &mut impl Rng,
    total_count: usize,
    sample_count: usize,
    mut process_index: F,
) -> CliResult<()>
where
    F: FnMut(usize) -> CliResult<()>,
{
    use rayon::prelude::ParallelSliceMut;

    if sample_count > total_count {
        return fail!("Sample size cannot be larger than population size");
    }

    // Store selected indices in a sorted vec of size k
    let mut selected = Vec::with_capacity(sample_count);

    // Fill first k positions
    for i in 0..sample_count {
        selected.push(i);
    }

    // Process remaining positions using reservoir sampling
    for i in sample_count..total_count {
        let j = rng.random_range(0..=i);
        if j < sample_count {
            unsafe { *selected.get_unchecked_mut(j) = i };
        }
    }

    // Process indices in order to avoid seeking back and forth
    selected.par_sort_unstable();
    for idx in selected {
        process_index(idx)?;
    }

    Ok(())
}
