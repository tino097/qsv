static USAGE: &str = r#"
Splits the given CSV data into chunks. It has three modes: by size (rowcount),
by number of chunks and by kb-size.

See `partition` command for splitting by a column value.

When splitting by size, the CSV data is split into chunks of the given number of
rows. The last chunk may have fewer rows if the number of records is not evenly
divisible by the given rowcount.

When splitting by number of chunks, the CSV data is split into the given number of
chunks. The number of rows in each chunk is determined by the number of records in
the CSV data and the number of desired chunks. If the number of records is not evenly
divisible by the number of chunks, the last chunk will have fewer records.

When splitting by kb-size, the CSV data is split into chunks of the given size in kilobytes.
The number of rows in each chunk may vary, but the size of each chunk will not exceed the
desired size.

Uses multithreading to go faster if the CSV has an index when splitting by size or
by number of chunks. Splitting by kb-size is always done sequentially with a single thread.

The default is to split by size with a chunk size of 500.

The files are written to the directory given with the name '{start}.csv',
where {start} is the index of the first record of the chunk (starting at 0).

Examples:
    qsv split outdir --size 100 --filename chunk_{}.csv input.csv
    # This will create files with names like chunk_0.csv, chunk_100.csv, etc.
    # in the directory 'outdir', creating the directory if it does not exist.

    qsv split outdir/subdir -s 100 --filename chunk_{}.csv --pad 5 input.csv
    # This will create files with names like chunk_00000.csv, chunk_00100.csv, etc.
    # in the directory 'outdir/subdir', creating the directories if they do not exist.

    qsv split . -s 100 input.csv
    # This will create files like 0.csv, 100.csv, etc. in the current directory.

    qsv split outdir --kb-size 1000 input.csv
    # This will create files with names like 0.csv, 994.csv, etc. in the directory
    # 'outdir', creating the directory if it does not exist. Each file will be close
    # to 1000KB in size.

    cat in.csv | qsv split mysplitoutput -s 1000

    qsv split outdir --chunks 10 input.csv

    qsv split splitoutdir -c 10 -j 4 input.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_split.rs.

Usage:
    qsv split [options] (--size <arg> | --chunks <arg> | --kb-size <arg>) <outdir> [<input>]
    qsv split --help

split arguments:
    <outdir>              The directory where the output files will be written.
                          If it does not exist, it will be created.
    <input>               The CSV file to read. If not given, input is read from
                          STDIN.

split options:
    -s, --size <arg>       The number of records to write into each chunk.
                           [default: 500]
    -c, --chunks <arg>     The number of chunks to split the data into.
                           This option is mutually exclusive with --size.
                           The number of rows in each chunk is determined by
                           the number of records in the CSV data and the number
                           of desired chunks. If the number of records is not evenly
                           divisible by the number of chunks, the last chunk will
                           have fewer records.
    -k, --kb-size <arg>    The size of each chunk in kilobytes. The number of rows
                           in each chunk may vary, but the size of each chunk will
                           not exceed the desired size.
                           This option is mutually exclusive with --size and --chunks.

    -j, --jobs <arg>       The number of splitting jobs to run in parallel.
                           This only works when the given CSV data has
                           an index already created. Note that a file handle
                           is opened for each job.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    --filename <filename>  A filename template to use when constructing
                           the names of the output files.  The string '{}'
                           will be replaced by the zero-based row number
                           of the first row in the chunk.
                           [default: {}.csv]
    --pad <arg>            The zero padding width that is used in the
                           generated filename.
                           [default: 0] 

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. Otherwise, the first row will
                           appear in all chunks as the header row.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not display an output summary to stderr.
"#;

use std::{fs, io, path::Path};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    util::{self, FilenameTemplate},
    CliResult,
};

#[derive(Clone, Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_outdir:      String,
    flag_size:       usize,
    flag_chunks:     Option<usize>,
    flag_kb_size:    Option<usize>,
    flag_jobs:       Option<usize>,
    flag_filename:   FilenameTemplate,
    flag_pad:        usize,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_quiet:      bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.flag_size == 0 {
        return fail_incorrectusage_clierror!("--size must be greater than 0.");
    }

    // check if outdir is set correctly
    if Path::new(&args.arg_outdir).is_file() && args.arg_input.is_none() {
        return fail_incorrectusage_clierror!("<outdir> is not specified or is a file.");
    }

    fs::create_dir_all(&args.arg_outdir)?;

    if let Some(kb_size) = args.flag_kb_size {
        args.split_by_kb_size(kb_size)
    } else {
        // we're splitting by rowcount or by number of chunks
        match args.rconfig().indexed()? {
            Some(idx) => args.parallel_split(&idx),
            None => args.sequential_split(),
        }
    }
}

impl Args {
    fn split_by_kb_size(&self, chunk_size: usize) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();

        let header_byte_size = if self.flag_no_headers {
            0
        } else {
            let mut headerbuf_wtr = csv::WriterBuilder::new().from_writer(vec![]);
            headerbuf_wtr.write_byte_record(&headers)?;

            // safety: we know the inner vec is valid
            headerbuf_wtr.into_inner().unwrap().len()
        };

        let mut wtr = self.new_writer(&headers, 0, self.flag_pad)?;
        let mut i = 0;
        let mut num_chunks = 0;
        let mut row = csv::ByteRecord::new();
        let chunk_size_bytes = chunk_size * 1024;
        let mut chunk_size_bytes_left = chunk_size_bytes - header_byte_size;

        let mut not_empty = rdr.read_byte_record(&mut row)?;
        let mut curr_size_bytes;
        let mut next_size_bytes;
        wtr.write_byte_record(&row)?;

        while not_empty {
            let mut buf_curr_wtr = csv::WriterBuilder::new().from_writer(vec![]);
            buf_curr_wtr.write_byte_record(&row)?;

            curr_size_bytes = buf_curr_wtr.into_inner().unwrap().len();

            not_empty = rdr.read_byte_record(&mut row)?;
            next_size_bytes = if not_empty {
                let mut buf_next_wtr = csv::WriterBuilder::new().from_writer(vec![]);
                buf_next_wtr.write_byte_record(&row)?;

                buf_next_wtr.into_inner().unwrap().len()
            } else {
                0
            };

            if curr_size_bytes + next_size_bytes >= chunk_size_bytes_left {
                wtr.flush()?;
                wtr = self.new_writer(&headers, i, self.flag_pad)?;
                chunk_size_bytes_left = chunk_size_bytes - header_byte_size;
                num_chunks += 1;
            }
            if next_size_bytes > 0 {
                wtr.write_byte_record(&row)?;
                chunk_size_bytes_left -= curr_size_bytes;
                i += 1;
            }
        }
        wtr.flush()?;

        if !self.flag_quiet {
            eprintln!(
                "Wrote chunk/s to '{}'. Size/chunk: <= {}KB; Num chunks: {}",
                Path::new(&self.arg_outdir).canonicalize()?.display(),
                chunk_size,
                num_chunks + 1
            );
        }

        Ok(())
    }

    fn sequential_split(&self) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();

        #[allow(clippy::cast_precision_loss)]
        let chunk_size = if let Some(flag_chunks) = self.flag_chunks {
            let count = util::count_rows(&rconfig)?;
            let chunk = flag_chunks;
            if chunk == 0 {
                return fail_incorrectusage_clierror!("--chunk must be greater than 0.");
            }
            (count as f64 / chunk as f64).ceil() as usize
        } else {
            self.flag_size
        };

        let mut wtr = self.new_writer(&headers, 0, self.flag_pad)?;
        let mut i = 0;
        let mut nchunks: usize = 0;
        let mut row = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut row)? {
            if i > 0 && i % chunk_size == 0 {
                wtr.flush()?;
                nchunks += 1;
                wtr = self.new_writer(&headers, i, self.flag_pad)?;
            }
            wtr.write_byte_record(&row)?;
            i += 1;
        }
        wtr.flush()?;

        if !self.flag_quiet {
            eprintln!(
                "Wrote {} chunk/s to '{}'. Rows/chunk: {} Num records: {}",
                nchunks + 1,
                Path::new(&self.arg_outdir).canonicalize()?.display(),
                chunk_size,
                i
            );
        }

        Ok(())
    }

    fn parallel_split(&self, idx: &Indexed<fs::File, fs::File>) -> CliResult<()> {
        let chunk_size;
        let idx_count = idx.count();

        #[allow(clippy::cast_precision_loss)]
        let nchunks = if let Some(flag_chunks) = self.flag_chunks {
            chunk_size = (idx_count as f64 / flag_chunks as f64).ceil() as usize;
            flag_chunks
        } else {
            chunk_size = self.flag_size;
            util::num_of_chunks(idx_count as usize, self.flag_size)
        };
        if nchunks == 1 {
            // there's only one chunk, we can just do a sequential split
            // which has less overhead and better error handling
            return self.sequential_split();
        }

        util::njobs(self.flag_jobs);

        // safety: we cannot use ? here because we're in a closure
        (0..nchunks).into_par_iter().for_each(|i| {
            let conf = self.rconfig();
            // safety: safe to unwrap because we know the file is indexed
            let mut idx = conf.indexed().unwrap().unwrap();
            // safety: the only way this can fail is if the file first row of the chunk
            // is not a valid CSV record, which is impossible because we're reading
            // from a file with a valid index
            let headers = idx.byte_headers().unwrap();

            let mut wtr = self
                // safety: the only way this can fail is if we cannot create a file
                .new_writer(headers, i * chunk_size, self.flag_pad)
                .unwrap();

            // safety: we know that there is more than one chunk, so we can safely
            // seek to the start of the chunk
            idx.seek((i * chunk_size) as u64).unwrap();
            let mut write_row;
            for row in idx.byte_records().take(chunk_size) {
                write_row = row.unwrap();
                wtr.write_byte_record(&write_row).unwrap();
            }
            // safety: safe to unwrap because we know the writer is a file
            // the only way this can fail is if we cannot write to the file
            wtr.flush().unwrap();
        });

        if !self.flag_quiet {
            eprintln!(
                "Wrote {} chunk/s to '{}'. Rows/chunk: {} Num records: {}",
                nchunks,
                Path::new(&self.arg_outdir).canonicalize()?.display(),
                chunk_size,
                idx_count
            );
        }

        Ok(())
    }

    fn new_writer(
        &self,
        headers: &csv::ByteRecord,
        start: usize,
        width: usize,
    ) -> CliResult<csv::Writer<Box<dyn io::Write + 'static>>> {
        let dir = Path::new(&self.arg_outdir);
        let path = dir.join(self.flag_filename.filename(&format!("{start:0>width$}")));
        let spath = Some(path.display().to_string());
        let mut wtr = Config::new(spath.as_ref()).writer()?;
        if !self.rconfig().no_headers {
            wtr.write_record(headers)?;
        }
        Ok(wtr)
    }

    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
    }
}
