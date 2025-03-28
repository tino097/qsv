static USAGE: &str = r#"
Joins two sets of CSV data on the specified columns.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

Joins are always done by ignoring leading and trailing whitespace. By default,
joins are done case sensitively, but this can be disabled with the --ignore-case
flag.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_join.rs.

Usage:
    qsv join [options] <columns1> <input1> <columns2> <input2>
    qsv join --help

input arguments:
    <input1>                is the first CSV data set to join.
    <input2>                is the second CSV data set to join.
    <columns1> & <columns2> are the columns to join on for each input.

    The columns arguments specify the columns to join for each input. Columns can
    be referenced by name or index, starting at 1. Specify multiple columns by
    separating them with a comma. Specify a range of columns with `-`. Both
    columns1 and columns2 must specify exactly the same number of columns.
    (See 'qsv select --help' for the full syntax.)

    For <input1> and <input2>, specifying `-` indicates reading from stdin.
    e.g. 'qsv frequency -s Agency nyc311.csv | qsv join value - id nycagencyinfo.csv'

join options:
    --left                 Do a 'left outer' join. This returns all rows in
                           first CSV data set, including rows with no
                           corresponding row in the second data set. When no
                           corresponding row exists, it is padded out with
                           empty fields.
    --left-anti            Do a 'left anti' join. This returns all rows in
                           first CSV data set that has no match with the 
                           second data set.
    --left-semi            Do a 'left semi' join. This returns all rows in
                           first CSV data set that has a match with the 
                           second data set.
    --right                Do a 'right outer' join. This returns all rows in
                           second CSV data set, including rows with no
                           corresponding row in the first data set. When no
                           corresponding row exists, it is padded out with
                           empty fields. (This is the reverse of 'outer left'.)
    --right-anti           This returns only the rows in the second CSV data set
                           that do not have a corresponding row in the first
                           data set. The output schema is the same as the
                           second dataset.
    --right-semi           This returns only the rows in the second CSV data set
                           that have a corresponding row in the first data set.
                           The output schema is the same as the second data set.
    --full                 Do a 'full outer' join. This returns all rows in
                           both data sets with matching records joined. If
                           there is no match, the missing side will be padded
                           out with empty fields. (This is the combination of
                           'outer left' and 'outer right'.)
    --cross                USE WITH CAUTION.
                           This returns the cartesian product of the CSV
                           data sets given. The number of rows return is
                           equal to N * M, where N and M correspond to the
                           number of rows in the given data sets, respectively.
    --nulls                When set, joins will work on empty fields.
                           Otherwise, empty fields are completely ignored.
                           (In fact, any row that has an empty field in the
                           key specified is ignored.)
    --keys-output <file>   Write successfully joined keys to <file>.
                           This means that the keys are written to the output
                           file when a match is found, with the exception of
                           anti joins, where keys are written when NO match
                           is found.
                           Cross joins do not write keys.

                           JOIN KEY TRANSFORMATION OPTIONS:
                           Note that transformations are applied to TEMPORARY
                           join key columns. The original columns are not modified
                           and the TEMPORARY columns are removed after the join.
-i, --ignore-case           When set, joins are done case insensitively.
-z, --ignore-leading-zeros  When set, leading zeros are ignored in join keys.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{collections::hash_map::Entry, fmt, io, iter::repeat_n, mem::swap, str};

use byteorder::{BigEndian, WriteBytesExt};
use foldhash::{HashMap, HashMapExt};
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter, SeekRead},
    index::Indexed,
    select::{SelectColumns, Selection},
    util,
    util::ByteString,
};

#[derive(Deserialize)]
struct Args {
    arg_columns1:              SelectColumns,
    arg_input1:                String,
    arg_columns2:              SelectColumns,
    arg_input2:                String,
    flag_left:                 bool,
    flag_left_anti:            bool,
    flag_left_semi:            bool,
    flag_right:                bool,
    flag_right_anti:           bool,
    flag_right_semi:           bool,
    flag_full:                 bool,
    flag_cross:                bool,
    flag_output:               Option<String>,
    flag_no_headers:           bool,
    flag_nulls:                bool,
    flag_delimiter:            Option<Delimiter>,
    flag_keys_output:          Option<String>,
    flag_ignore_case:          bool,
    flag_ignore_leading_zeros: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut state = args.new_io_state()?;
    match (
        args.flag_left,
        args.flag_left_anti,
        args.flag_left_semi,
        args.flag_right,
        args.flag_right_anti,
        args.flag_right_semi,
        args.flag_full,
        args.flag_cross,
    ) {
        // default inner join
        (false, false, false, false, false, false, false, false) => {
            state.write_headers()?;
            state.inner_join()
        },
        // left join
        (true, false, false, false, false, false, false, false) => {
            state.write_headers()?;
            state.outer_join(false)
        },
        // left anti join
        (false, true, false, false, false, false, false, false) => {
            state.write_headers1()?;
            state.left_join(true)
        },
        // left semi join
        (false, false, true, false, false, false, false, false) => {
            state.write_headers1()?;
            state.left_join(false)
        },
        // right join
        (false, false, false, true, false, false, false, false) => {
            state.write_headers()?;
            state.outer_join(true)
        },
        // right anti join
        // swap left and right data sets and run left anti join
        (false, false, false, false, true, false, false, false) => {
            let mut swapped_join = state;
            swap(&mut swapped_join.rdr1, &mut swapped_join.rdr2);
            swap(&mut swapped_join.sel1, &mut swapped_join.sel2);
            swapped_join.write_headers1()?;
            swapped_join.left_join(true)
        },
        // right semi join
        // swap left and right data sets and run left semi join
        (false, false, false, false, false, true, false, false) => {
            let mut swapped_join = state;
            swap(&mut swapped_join.rdr1, &mut swapped_join.rdr2);
            swap(&mut swapped_join.sel1, &mut swapped_join.sel2);
            swapped_join.write_headers1()?;
            swapped_join.left_join(false)
        },
        // full outer join
        (false, false, false, false, false, false, true, false) => {
            state.write_headers()?;
            state.full_outer_join()
        },
        // cross join
        (false, false, false, false, false, false, false, true) => {
            state.write_headers()?;
            state.cross_join()
        },
        _ => fail_incorrectusage_clierror!("Please pick exactly one join operation."),
    }
}

struct IoState<R, W: io::Write> {
    wtr:        csv::Writer<W>,
    rdr1:       csv::Reader<R>,
    sel1:       Selection,
    rdr2:       csv::Reader<R>,
    sel2:       Selection,
    no_headers: bool,
    casei:      bool,
    zerosi:     bool,
    nulls:      bool,
    keys_wtr:   KeysWriter,
}

impl<R: io::Read + io::Seek, W: io::Write> IoState<R, W> {
    fn write_headers(&mut self) -> CliResult<()> {
        if !self.no_headers {
            let mut headers = self.rdr1.byte_headers()?.clone();
            headers.extend(self.rdr2.byte_headers()?.iter());
            self.wtr.write_record(&headers)?;
        }
        Ok(())
    }

    fn write_headers1(&mut self) -> CliResult<()> {
        if !self.no_headers {
            let headers = self.rdr1.byte_headers()?;
            self.wtr.write_record(headers)?;
        }
        Ok(())
    }

    fn inner_join(mut self) -> CliResult<()> {
        let mut scratch = csv::ByteRecord::new();
        let mut validx =
            ValueIndex::new(self.rdr2, &self.sel2, self.casei, self.zerosi, self.nulls)?;
        let mut row = csv::ByteRecord::new();
        let mut key;

        while self.rdr1.read_byte_record(&mut row)? {
            key = get_row_key(&self.sel1, &row, self.casei, self.zerosi);
            if let Some(rows) = validx.values.get(&key) {
                self.keys_wtr.write_key(&key)?;

                for &rowi in rows {
                    validx.idx.seek(rowi as u64)?;

                    validx.idx.read_byte_record(&mut scratch)?;

                    let combined = row.iter().chain(scratch.iter());
                    self.wtr.write_record(combined)?;
                }
            }
        }
        self.wtr.flush()?;
        self.keys_wtr.flush()?;
        Ok(())
    }

    fn outer_join(mut self, right: bool) -> CliResult<()> {
        if right {
            swap(&mut self.rdr1, &mut self.rdr2);
            swap(&mut self.sel1, &mut self.sel2);
        }

        let mut scratch = csv::ByteRecord::new();
        let (_, pad2) = self.get_padding()?;
        let mut validx =
            ValueIndex::new(self.rdr2, &self.sel2, self.casei, self.zerosi, self.nulls)?;
        let mut row = csv::ByteRecord::new();
        let mut key;

        while self.rdr1.read_byte_record(&mut row)? {
            key = get_row_key(&self.sel1, &row, self.casei, self.zerosi);
            match validx.values.get(&key) {
                Some(rows) => {
                    self.keys_wtr.write_key(&key)?;

                    for &rowi in rows {
                        validx.idx.seek(rowi as u64)?;
                        let row1 = row.iter();
                        validx.idx.read_byte_record(&mut scratch)?;
                        if right {
                            self.wtr.write_record(scratch.iter().chain(row1))?;
                        } else {
                            self.wtr.write_record(row1.chain(&scratch))?;
                        }
                    }
                },
                _ => {
                    if right {
                        self.wtr.write_record(pad2.iter().chain(&row))?;
                    } else {
                        self.wtr.write_record(row.iter().chain(&pad2))?;
                    }
                },
            }
        }
        self.wtr.flush()?;
        self.keys_wtr.flush()?;
        Ok(())
    }

    fn left_join(mut self, anti: bool) -> CliResult<()> {
        let validx = ValueIndex::new(self.rdr2, &self.sel2, self.casei, self.zerosi, self.nulls)?;
        let mut row = csv::ByteRecord::new();
        let mut key;

        while self.rdr1.read_byte_record(&mut row)? {
            key = get_row_key(&self.sel1, &row, self.casei, self.zerosi);
            #[allow(clippy::map_entry)]
            if !validx.values.contains_key(&key) {
                if anti {
                    self.keys_wtr.write_key(&key)?;
                    self.wtr.write_record(&row)?;
                }
            } else if !anti {
                self.keys_wtr.write_key(&key)?;
                self.wtr.write_record(&row)?;
            }
        }
        self.wtr.flush()?;
        self.keys_wtr.flush()?;
        Ok(())
    }

    fn full_outer_join(mut self) -> CliResult<()> {
        let mut scratch = csv::ByteRecord::new();
        let (pad1, pad2) = self.get_padding()?;
        let mut validx =
            ValueIndex::new(self.rdr2, &self.sel2, self.casei, self.zerosi, self.nulls)?;

        // Keep track of which rows we've written from rdr2.
        let mut rdr2_written: Vec<_> = repeat_n(false, validx.num_rows).collect();
        let mut row1 = csv::ByteRecord::new();
        let mut key;

        while self.rdr1.read_byte_record(&mut row1)? {
            key = get_row_key(&self.sel1, &row1, self.casei, self.zerosi);
            match validx.values.get(&key) {
                Some(rows) => {
                    self.keys_wtr.write_key(&key)?;

                    for &rowi in rows {
                        rdr2_written[rowi] = true;

                        validx.idx.seek(rowi as u64)?;
                        validx.idx.read_byte_record(&mut scratch)?;
                        self.wtr.write_record(row1.iter().chain(&scratch))?;
                    }
                },
                _ => {
                    self.wtr.write_record(row1.iter().chain(&pad2))?;
                },
            }
        }

        // OK, now write any row from rdr2 that didn't get joined with a row
        // from rdr1.
        for (i, &written) in rdr2_written.iter().enumerate() {
            if !written {
                validx.idx.seek(i as u64)?;
                validx.idx.read_byte_record(&mut scratch)?;
                self.wtr.write_record(pad1.iter().chain(&scratch))?;
            }
        }
        self.wtr.flush()?;
        self.keys_wtr.flush()?;
        Ok(())
    }

    fn cross_join(mut self) -> CliResult<()> {
        let mut pos = csv::Position::new();
        pos.set_byte(0);
        let mut row2 = csv::ByteRecord::new();
        let mut row1 = csv::ByteRecord::new();
        let rdr2_has_headers = self.rdr2.has_headers();
        while self.rdr1.read_byte_record(&mut row1)? {
            self.rdr2.seek(pos.clone())?;
            if rdr2_has_headers {
                // Read and skip the header row, since CSV readers disable
                // the header skipping logic after being seeked.
                self.rdr2.read_byte_record(&mut row2)?;
            }
            while self.rdr2.read_byte_record(&mut row2)? {
                self.wtr.write_record(row1.iter().chain(&row2))?;
            }
        }
        Ok(self.wtr.flush()?)
    }

    fn get_padding(&mut self) -> CliResult<(csv::ByteRecord, csv::ByteRecord)> {
        let len1 = self.rdr1.byte_headers()?.len();
        let len2 = self.rdr2.byte_headers()?.len();
        Ok((repeat_n(b"", len1).collect(), repeat_n(b"", len2).collect()))
    }
}

impl Args {
    fn new_io_state(
        &self,
    ) -> CliResult<IoState<Box<dyn SeekRead + 'static>, Box<dyn io::Write + 'static>>> {
        let rconf1 = Config::new(Some(self.arg_input1.clone()).as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.arg_columns1.clone());
        let rconf2 = Config::new(Some(self.arg_input2.clone()).as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.arg_columns2.clone());

        let mut rdr1 = match rconf1.reader_file_stdin() {
            Ok(rdr1) => rdr1,
            Err(e) => return fail_clierror!("Failed to read input1: {e}"),
        };
        let mut rdr2 = match rconf2.reader_file_stdin() {
            Ok(rdr2) => rdr2,
            Err(e) => return fail_clierror!("Failed to read input2: {e}"),
        };
        let (sel1, sel2) = self.get_selections(&rconf1, &mut rdr1, &rconf2, &mut rdr2)?;

        let keys_wtr = if self.flag_cross {
            KeysWriter::new(None)?
        } else {
            KeysWriter::new(self.flag_keys_output.as_ref())?
        };

        Ok(IoState {
            wtr: Config::new(self.flag_output.as_ref()).writer()?,
            rdr1,
            sel1,
            rdr2,
            sel2,
            no_headers: rconf1.no_headers,
            casei: self.flag_ignore_case,
            zerosi: self.flag_ignore_leading_zeros,
            nulls: self.flag_nulls,
            keys_wtr,
        })
    }

    #[allow(clippy::unused_self)]
    fn get_selections<R: io::Read>(
        &self,
        rconf1: &Config,
        rdr1: &mut csv::Reader<R>,
        rconf2: &Config,
        rdr2: &mut csv::Reader<R>,
    ) -> CliResult<(Selection, Selection)> {
        let headers1 = rdr1.byte_headers()?;
        let headers2 = rdr2.byte_headers()?;
        let select1 = rconf1.selection(headers1)?;
        let select2 = rconf2.selection(headers2)?;
        if select1.len() != select2.len() {
            return fail_incorrectusage_clierror!(
                "Column selections must have the same number of columns, but found column \
                 selections with {} and {} columns.",
                select1.len(),
                select2.len()
            );
        }
        Ok((select1, select2))
    }
}

struct ValueIndex<R> {
    // This maps tuples of values to corresponding rows.
    values:   HashMap<Vec<ByteString>, Vec<usize>>,
    idx:      Indexed<R, io::Cursor<Vec<u8>>>,
    num_rows: usize,
}

impl<R: io::Read + io::Seek> ValueIndex<R> {
    /// Creates a new ValueIndex by reading a CSV and building indexes for
    /// both row positions and values.
    ///
    /// This function reads through a CSV file once to build two indexes:
    /// 1. A mapping of selected column values to the row numbers where they appear
    /// 2. A byte offset index for random access to rows in the CSV
    ///
    /// # Arguments
    ///
    /// * `rdr` - A CSV reader that implements Read + Seek
    /// * `sel` - A Selection that specifies which columns to index
    /// * `casei` - If true, indexed values are compared case-insensitively
    /// * `zerosi` - If true, indexed values are compared without leading zeros
    /// * `nulls` - If true, indexed rows with empty values are included
    ///
    /// # Returns
    ///
    /// Returns a ValueIndex containing:
    /// * `values` - HashMap mapping column values to row numbers
    /// * `idx` - Indexed CSV reader for random access
    /// * `num_rows` - Total number of data rows processed
    ///
    /// # Notes
    ///
    /// - Header rows are included in the byte offset index but not the value index
    /// - Values are trimmed and optionally converted to lowercase before indexing
    /// - Rows with empty indexed values are skipped unless nulls=true
    fn new(
        mut rdr: csv::Reader<R>,
        sel: &Selection,
        casei: bool,
        zerosi: bool,
        nulls: bool,
    ) -> CliResult<ValueIndex<R>> {
        let mut val_idx = HashMap::with_capacity(20_000);
        let mut row_idx = io::Cursor::new(Vec::with_capacity(8 * 20_000));
        let (mut rowi, mut count) = (0_usize, 0_usize);

        // This logic is kind of tricky. Basically, we want to include
        // the header row in the line index (because that's what csv::index
        // does), but we don't want to include header values in the ValueIndex.
        if rdr.has_headers() {
            // ... so if there are headers, we make sure that we've parsed
            // them, and write the offset of the header row to the index.
            rdr.byte_headers()?;
            row_idx.write_u64::<BigEndian>(0)?;
            count += 1;
        } else {
            // ... and if there are no headers, we seek to the beginning and
            // index everything.
            let mut pos = csv::Position::new();
            pos.set_byte(0);
            rdr.seek(pos)?;
        }

        let mut row = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut row)? {
            // This is a bit hokey. We're doing this manually instead of using
            // the `csv-index` crate directly so that we can create both
            // indexes in one pass.
            row_idx.write_u64::<BigEndian>(row.position().unwrap().byte())?;

            let fields: Vec<_> = sel
                .select(&row)
                .map(|v| {
                    if let Ok(s) = simdutf8::basic::from_utf8(v) {
                        let cased_bytes_vec = if casei {
                            s.trim().to_lowercase().into_bytes()
                        } else {
                            s.trim().as_bytes().to_vec()
                        };
                        if zerosi {
                            if cased_bytes_vec.iter().all(|&b| b == b'0')
                                && !cased_bytes_vec.is_empty()
                            {
                                vec![b'0']
                            } else {
                                cased_bytes_vec
                                    .iter()
                                    .skip_while(|&b| *b == b'0')
                                    .copied()
                                    .collect()
                            }
                        } else {
                            cased_bytes_vec
                        }
                    } else {
                        v.to_vec()
                    }
                })
                .collect();
            if nulls || !fields.iter().any(std::vec::Vec::is_empty) {
                match val_idx.entry(fields) {
                    Entry::Vacant(v) => {
                        let mut rows = Vec::with_capacity(4);
                        rows.push(rowi);
                        v.insert(rows);
                    },
                    Entry::Occupied(mut v) => {
                        v.get_mut().push(rowi);
                    },
                }
            }
            rowi += 1;
            count += 1;
        }

        row_idx.write_u64::<BigEndian>(count as u64)?;
        let idx = Indexed::open(rdr, io::Cursor::new(row_idx.into_inner()))?;
        Ok(ValueIndex {
            values: val_idx,
            idx,
            num_rows: rowi,
        })
    }
}

impl<R> fmt::Debug for ValueIndex<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Sort the values by order of first appearance.
        let mut kvs = self.values.iter().collect::<Vec<_>>();
        kvs.sort_by(|&(_, v1), &(_, v2)| v1[0].cmp(&v2[0]));
        for (keys, rows) in kvs {
            // This is just for debugging, so assume Unicode for now.
            let keys = keys
                .iter()
                .map(|k| String::from_utf8(k.clone()).unwrap())
                .collect::<Vec<_>>();
            writeln!(f, "({}) => {rows:?}", keys.join(", "))?;
        }
        Ok(())
    }
}

#[inline]
/// Extracts key values from a CSV row based on the given selection and options.
///
/// # Arguments
///
/// * `sel` - The selection that specifies which fields to extract from the row
/// * `row` - The CSV row to extract values from
/// * `casei` - If true, converts extracted values to lowercase for case-insensitive comparison
/// * `zerosi` - If true, removes leading zeros from numeric values
///
/// # Returns
///
/// A vector of ByteStrings containing the extracted and processed key values.
///
/// # Processing
///
/// For each selected field:
/// 1. Attempts to convert the bytes to a UTF-8 string
/// 2. If successful:
///    - Trims leading/trailing whitespace
///    - Optionally converts to lowercase if `casei` is true
///    - If `zerosi` is true:
///      * For all-zero values, returns a single "0" byte
///      * Otherwise, strips leading zeros
///    - Converts back to bytes
/// 3. If not valid UTF-8, returns the original bytes unchanged
fn get_row_key(
    sel: &Selection,
    row: &csv::ByteRecord,
    casei: bool,
    zerosi: bool,
) -> Vec<ByteString> {
    let key: Vec<_> = sel
        .select(row)
        .map(|v| {
            if let Ok(s) = simdutf8::basic::from_utf8(v) {
                let cased_bytes_vec = if casei {
                    s.trim().to_lowercase().into_bytes()
                } else {
                    s.trim().as_bytes().to_vec()
                };
                if zerosi {
                    if cased_bytes_vec.iter().all(|&b| b == b'0') && !cased_bytes_vec.is_empty() {
                        vec![b'0']
                    } else {
                        cased_bytes_vec
                            .iter()
                            .skip_while(|&b| *b == b'0')
                            .copied()
                            .collect()
                    }
                } else {
                    cased_bytes_vec
                }
            } else {
                v.to_vec()
            }
        })
        .collect();
    key
}

struct KeysWriter {
    writer:  csv::Writer<Box<dyn io::Write>>,
    enabled: bool,
}

impl KeysWriter {
    fn new(keys_path: Option<&String>) -> CliResult<Self> {
        let (writer, enabled) = if let Some(path) = keys_path {
            (Config::new(Some(path)).writer()?, true)
        } else {
            let sink: Box<dyn io::Write> = Box::new(std::io::sink());
            (csv::WriterBuilder::new().from_writer(sink), false)
        };

        Ok(Self { writer, enabled })
    }

    #[inline]
    fn write_key(&mut self, key: &[ByteString]) -> CliResult<()> {
        if self.enabled {
            self.writer.write_record(key)?;
        }
        Ok(())
    }

    fn flush(&mut self) -> CliResult<()> {
        if self.enabled {
            self.writer.flush()?;
        }
        Ok(())
    }
}
