## qsv: Blazing-fast Data-Wrangling toolkit

[![Linux build status](https://github.com/dathere/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/dathere/qsv/actions/workflows/rust.yml)
[![Windows build status](https://github.com/dathere/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/dathere/qsv/actions/workflows/rust-windows.yml)
[![macOS build status](https://github.com/dathere/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/dathere/qsv/actions/workflows/rust-macos.yml)
[![Security audit](https://github.com/dathere/qsv/actions/workflows/security-audit.yml/badge.svg)](https://github.com/dathere/qsv/actions/workflows/security-audit.yml)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/05fdfbc6ddb24e7c829d0c6c3ce82d18)](https://app.codacy.com/gh/dathere/qsv/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Clones](https://img.shields.io/badge/dynamic/json?color=success&label=clones&query=count&url=https://gist.githubusercontent.com/jqnatividad/13f60ad0b54856a55f60b8e653079349/raw/clone.json&logo=github)](https://github.com/MShawon/github-clone-count-badge)
[![Discussions](https://img.shields.io/github/discussions/dathere/qsv)](https://github.com/dathere/qsv/discussions)
[![Crates.io](https://img.shields.io/crates/v/qsv.svg?logo=crates.io)](https://crates.io/crates/qsv)
[![Crates.io downloads](https://img.shields.io/crates/d/qsv?color=orange&label=crates.io%20downloads)](https://crates.io/crates/qsv)
[![Prebuilt Downloads](https://img.shields.io/github/downloads/dathere/qsv/total?logo=github&label=prebuilt%20downloads)](https://github.com/dathere/qsv/releases/latest)
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.82.0-red?logo=rust)](#minimum-supported-rust-version)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjqnatividad%2Fqsv.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjqnatividad%2Fqsv?ref=badge_shield)

<div align="center">

 &nbsp;          |  Table of Contents
:--------------------------|:-------------------------
![qsv logo](docs/images/qsv-logo.png)<br/>[_Hi-ho "Quicksilver" away!_](https://www.youtube.com/watch?v=p9lf76xOA5k)<br/><sub><sup>[logo details](https://github.com/dathere/qsv/discussions/295)</sup></sub><br/>|qsv is a command line program for querying, slicing,<br>indexing, analyzing, filtering, enriching, transforming,<br>sorting, validating, joining, formatting & converting</br>tabular data (CSV, spreadsheets, [etc.](#file-formats)).<br>Commands are simple, composable & ___"blazing fast"___.<br><br>* [Commands](#available-commands)<br>* [Installation Options](#installation-options)<br> * [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour) / [Notebooks](contrib/notebooks/) / [Lessons & Exercises](https://100.dathere.com)<br>* [Cookbook](https://github.com/dathere/qsv/wiki/Cookbook#cookbook)<br>* [FAQ](https://github.com/dathere/qsv/discussions/categories/faq)<br>* [Performance Tuning](docs/PERFORMANCE.md#performance-tuning)<br>* 👉 [Benchmarks](https://qsv.dathere.com/benchmarks) 🚀<br>* [Environment Variables](docs/ENVIRONMENT_VARIABLES.md)<br>* [Feature Flags](#feature-flags)<br>* [Goals/Non-goals](#goals--non-goals)<br>* [Testing](#testing)<br>* [NYC School of Data 2022](https://docs.google.com/presentation/d/e/2PACX-1vQ12ndZL--gkz0HLQRaxqsNOwzddkv1iUKB3sq661yA77OPlAsmHJHpjaqt9s9QEf73VqMfb0cv4jHU/pub?start=false&loop=false&delayms=3000)/[csv,conf,v8](https://docs.google.com/presentation/d/10T_3MyIqS5UsKxJaOY7Ktrd-GfhJelQImlE_qYmtuis/edit#slide=id.g2e0f1e7aa0e_0_62) slides<br>* [Sponsor](#sponsor)
</div>
<div align="center">

## Try it out at [qsv.dathere.com](https://qsv.dathere.com)! <!-- markdownlint-disable-line -->

</div>

| <a name="available-commands">Command | Description |
| --- | --- |
| [apply](/src/cmd/apply.rs#L2)✨<br>📇🚀🧠🤖🔣👆| Apply series of string, date, math & currency transformations to given CSV column/s. It also has some basic [NLP](https://en.wikipedia.org/wiki/Natural_language_processing) functions ([similarity](https://crates.io/crates/strsim), [sentiment analysis](https://crates.io/crates/vader_sentiment), [profanity](https://docs.rs/censor/latest/censor/), [eudex](https://github.com/ticki/eudex#eudex-a-blazingly-fast-phonetic-reductionhashing-algorithm), [language](https://crates.io/crates/whatlang) & [name gender](https://github.com/Raduc4/gender_guesser?tab=readme-ov-file#gender-guesser)) detection.  |
| <a name="applydp_deeplink"></a>[applydp](/src/cmd/applydp.rs#L2)✨<br>📇🚀🔣👆 ![CKAN](docs/images/ckan.png)| applydp is a slimmed-down version of `apply` with only [Datapusher+](https://github.com/dathere/datapusher-plus) relevant subcommands/operations (`qsvdp` binary variant only). |
| [behead](/src/cmd/behead.rs#L2) | Drop headers from a CSV.  |
| [cat](/src/cmd/cat.rs#L2)<br>🗄️ | Concatenate CSV files by row or by column. |
| [clipboard](/src/cmd/clipboard.rs#L2)✨ | Provide input from the clipboard or save output to the clipboard. |
| [count](/src/cmd/count.rs#L3)<br>📇🏎️🐻‍❄️ | Count the rows and optionally compile record width statistics of a CSV file. (11.87 seconds for a 15gb, 27m row NYC 311 dataset without an index. Instantaneous with an index.) If the `polars` feature is enabled, uses Polars' multithreaded, mem-mapped CSV reader for fast counts even without an index |
| [datefmt](/src/cmd/datefmt.rs#L2)<br>📇🚀👆 | Formats recognized date fields ([19 formats recognized](https://docs.rs/qsv-dateparser/latest/qsv_dateparser/#accepted-date-formats)) to a specified date format using [strftime date format specifiers](https://docs.rs/chrono/latest/chrono/format/strftime/). |
| [dedup](/src/cmd/dedup.rs#L2)<br>🤯🚀👆 | Remove duplicate rows (See also `extdedup`, `extsort`, `sort` & `sortcheck` commands). |
| [describegpt](/src/cmd/describegpt.rs#L2)<br>🌐🤖🪄 | Infer extended metadata about a CSV using a GPT model from [OpenAI's API](https://platform.openai.com/docs/introduction) or an LLM from another API compatible with the OpenAI API specification such as [Ollama](https://ollama.com) or [Jan](https://jan.ai). |
| [diff](/src/cmd/diff.rs#L2)<br>🚀 | Find the difference between two CSVs with ludicrous speed!<br/>e.g. _compare two CSVs with 1M rows x 9 columns in under 600ms!_ |
| [edit](/src/cmd/edit.rs#L2) | Replace the value of a cell specified by its row and column. |
| [enum](/src/cmd/enumerate.rs#L2)<br>👆 | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.  |
| [excel](/src/cmd/excel.rs#L2)<br>🚀 | Exports a specified Excel/ODS sheet to a CSV file. |
| [exclude](/src/cmd/exclude.rs#L2)<br>📇👆 | Removes a set of CSV data from another set based on the specified columns.  |
| [explode](/src/cmd/explode.rs#L2)<br>🔣👆 | Explode rows into multiple ones by splitting a column value based on the given separator.  |
| [extdedup](/src/cmd/extdedup.rs#L2)<br>👆 | Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped, [on-disk hash table](https://crates.io/crates/odht). Unlike the `dedup` command, this command does not load the entire file into memory nor does it sort the deduped file. |
| [extsort](/src/cmd/extsort.rs#L2)<br>🚀📇👆 | Sort an arbitrarily large CSV/text file using a multithreaded [external merge sort](https://en.wikipedia.org/wiki/External_sorting) algorithm. |
| [fetch](/src/cmd/fetch.rs#L3)✨<br>📇🧠🌐 | Send/Fetch data to/from web services for every row using **HTTP Get**. Comes with [HTTP/2](https://http2-explained.haxx.se/en/part1) [adaptive flow control](https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518), [jaq](https://github.com/01mf02/jaq?tab=readme-ov-file#jaq) JSON query language support, dynamic throttling ([RateLimit](https://www.ietf.org/archive/id/draft-ietf-httpapi-ratelimit-headers-06.html)) & caching with available persistent caching using [Redis](https://redis.io/) or a disk-cache. |
| [fetchpost](/src/cmd/fetchpost.rs#L3)✨<br>📇🧠🌐 | Similar to `fetch`, but uses **HTTP Post** ([HTTP GET vs POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)). Supports both HTML form (application/x-www-form-urlencoded) and JSON (application/json) content types. |
| [fill](/src/cmd/fill.rs#L2)<br>👆 | Fill empty values.  |
| [fixlengths](/src/cmd/fixlengths.rs#L2) | Force a CSV to have same-length records by either padding or truncating them. |
| [flatten](/src/cmd/flatten.rs#L2) | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](/src/cmd/fmt.rs#L2) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)  |
| [foreach](/src/cmd/foreach.rs#L2)✨<br>📇 | Execute a shell command once per line in given CSV file. |
| [frequency](/src/cmd/frequency.rs#L2)<br>📇😣🏎️👆🪄 | Build [frequency tables](https://statisticsbyjim.com/basics/frequency-table/) of each column. Uses multithreading to go faster if an index is present. |
| [geocode](/src/cmd/geocode.rs#L2)✨<br>📇🧠🌐🚀🔣👆 | Geocodes a location against an updatable local copy of the [Geonames](https://www.geonames.org/) cities database. With caching and multi-threading, it geocodes up to 360,000 records/sec! |
| [headers](/src/cmd/headers.rs#L2)<br>🗄️ | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](/src/cmd/index.rs#L2) | Create an index (📇) for a CSV. This is very quick (even the 15gb, 28m row NYC 311 dataset takes all of 14 seconds to index) & provides constant time indexing/random access into the CSV. With an index, `count`, `sample` & `slice` work instantaneously; random access mode is enabled in `luau`; and multithreading (🏎️) is enabled for the `frequency`, `split`, `stats`, `schema` & `tojsonl` commands. |
| [input](/src/cmd/input.rs#L2) | Read CSV data with special commenting, quoting, trimming, line-skipping & non-UTF8 encoding handling rules. Typically used to "normalize" a CSV for further processing with other qsv commands. |
| [join](/src/cmd/join.rs#L2)<br>👆 | Inner, outer, right, cross, anti & semi joins. Automatically creates a simple, in-memory hash index to make it fast.  |
| [joinp](/src/cmd/joinp.rs#L2)✨<br>🚀🐻‍❄️🪄 | Inner, outer, right, cross, anti, semi & asof joins using the [Pola.rs](https://www.pola.rs) engine. Unlike the `join` command, `joinp` can process files larger than RAM, is multithreaded, has join key validation, pre-join filtering, supports [asof joins](https://pola-rs.github.io/polars/py-polars/html/reference/dataframe/api/polars.DataFrame.join_asof.html) (which is [particularly useful for time series data](https://github.com/dathere/qsv/blob/30cc920d0812a854fcbfedc5db81788a0600c92b/tests/test_joinp.rs#L509-L983)) & its output columns can be coalesced. |
| [json](/src/cmd/json.rs#L2)<br>👆 | Convert JSON to CSV.
| [jsonl](/src/cmd/jsonl.rs#L2)<br>🚀🔣 | Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. See `tojsonl` command to convert CSV to JSONL.
| [lens](/src/cmd/lens.rs#L2)✨ | Interactively view, search & filter a CSV using the [csvlens](https://github.com/YS-L/csvlens#csvlens) engine.
| <a name="luau_deeplink"></a><br>[luau](/src/cmd/luau.rs#L2) 👑✨<br>📇🌐🔣📚 ![CKAN](docs/images/ckan.png) | Create multiple new computed columns, filter rows, compute aggregations and build complex data pipelines by executing a [Luau](https://luau-lang.org) [0.653](https://github.com/Roblox/luau/releases/tag/0.653) expression/script for every row of a CSV file ([sequential mode](https://github.com/dathere/qsv/blob/bb72c4ef369d192d85d8b7cc6e972c1b7df77635/tests/test_luau.rs#L254-L298)), or using [random access](https://www.webopedia.com/definitions/random-access/) with an index ([random access mode](https://github.com/dathere/qsv/blob/bb72c4ef369d192d85d8b7cc6e972c1b7df77635/tests/test_luau.rs#L367-L415)).<br>Can process a single Luau expression or [full-fledged data-wrangling scripts using lookup tables](https://github.com/dathere/qsv-lookup-tables#example) with discrete BEGIN, MAIN and END sections.<br> It is not just another qsv command, it is qsv's [Domain-specific Language](https://en.wikipedia.org/wiki/Domain-specific_language) (DSL) with [numerous qsv-specific helper functions](https://github.com/dathere/qsv/blob/113eee17b97882dc368b2e65fec52b86df09f78b/src/cmd/luau.rs#L1356-L2290) to build production data pipelines. |
| [partition](/src/cmd/partition.rs#L2)<br>👆 | Partition a CSV based on a column value. |
| [pro](/src/cmd/pro.rs#L2) | Interact with the [qsv pro](https://qsvpro.dathere.com) API. |
| [prompt](/src/cmd/prompt.rs#L2)✨ | Open a file dialog to either pick a file as input or save output to a file. |
| [pseudo](/src/cmd/pseudo.rs#L2)<br>🔣👆 | [Pseudonymise](https://en.wikipedia.org/wiki/Pseudonymization) the value of the given column by replacing them with an incremental identifier.  |
| [py](/src/cmd/python.rs#L2)✨<br>📇🔣 | Create a new computed column or filter rows by evaluating a python expression on every row of a CSV file. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting, [with the ability to evaluate Python expressions as well](https://github.com/dathere/qsv/blob/4cd00dca88addf0d287247fa27d40563b6d46985/src/cmd/python.rs#L23-L31). |
| [rename](/src/cmd/rename.rs#L2) |  Rename the columns of a CSV efficiently. |
| [replace](/src/cmd/replace.rs#L2)<br>📇👆 | Replace CSV data using a regex. Applies the regex to each field individually. |
| [reverse](/src/cmd/reverse.rs#L2)<br>📇🤯 | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key. If an index is present, it works with constant memory. Otherwise, it will load all the data into memory. |
| <a name="safenames_deeplink"></a>[safenames](/src/cmd/safenames.rs#L2)<br>![CKAN](docs/images/ckan.png) | Modify headers of a CSV to only have ["safe" names](/src/cmd/safenames.rs#L5-L14) - guaranteed "database-ready"/"CKAN-ready" names.  |
| [sample](/src/cmd/sample.rs#L2)<br>📇🌐🏎️ | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling), using memory proportional to the sample size. If an index is present, using random indexing with constant memory. Supports sampling from CSVs on remote URLs. |
| <a name="schema_deeplink"></a>[schema](/src/cmd/schema.rs#L2)<br>📇😣🏎️👆🪄 | Infer schema from CSV data, replete with data type & domain/range validation & output in [JSON Schema](https://json-schema.org/) format. Uses multithreading to go faster if an index is present. See [`validate`](#validate_deeplink) command to use the generated JSON Schema to validate if similar CSVs comply with the schema. |
| [search](/src/cmd/search.rs#L2)<br>📇👆 | Run a regex over a CSV. Applies the regex to selected fields & shows only matching rows.  |
| [searchset](/src/cmd/searchset.rs#L2)<br>📇👆 | _Run multiple regexes over a CSV in a single pass._ Applies the regexes to each field individually & shows only matching rows.  |
| [select](/src/cmd/select.rs#L2)<br>👆 | Select, re-order, reverse, duplicate or drop columns.  |
| [slice](/src/cmd/slice.rs#L2)<br>📇🏎️ | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).  |
| <a name="snappy_deeplink"></a>[snappy](/src/cmd/snappy.rs#L2)<br>🚀🌐 | Does streaming compression/decompression of the input using Google's [Snappy](https://github.com/google/snappy/blob/main/docs/README.md) framing format ([more info](#snappy-compressiondecompression)). |
| [sniff](/src/cmd/sniff.rs#L2)<br>📇🌐 ![CKAN](docs/images/ckan.png) | Quickly sniff & infer CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, average record length, number of records, content length & estimated number of records if sniffing a CSV on a URL, number of fields, field names & data types). It is also a general mime type detector. |
| [sort](/src/cmd/sort.rs#L2)<br>🚀🤯👆 | Sorts CSV data in alphabetical (with case-insensitive option), numerical, reverse, unique or random (with optional seed) order (See also `extsort` & `sortcheck` commands).  |
| [sortcheck](/src/cmd/sortcheck.rs#L2)<br>📇👆 | Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count. |
| [split](/src/cmd/split.rs#L2)<br>📇🏎️ | Split one CSV file into many CSV files. It can split by number of rows, number of chunks or file size. Uses multithreading to go faster if an index is present when splitting by rows or chunks. |
| [sqlp](/src/cmd/sqlp.rs#L2)✨<br>📇🚀🐻‍❄️🗄️🪄 | Run [Polars](https://pola.rs) SQL queries against several CSVs - converting queries to blazing-fast [LazyFrame](https://docs.pola.rs/user-guide/lazy/using/) expressions, processing larger than memory CSV files. Query results can be saved in CSV, JSON, JSONL, Parquet, Apache Arrow IPC and Apache Avro formats. Supports automatic decompression of gzip, zstd and zlib compressed input files using the `read_csv()` table function. |
| [stats](/src/cmd/stats.rs#L2)<br>📇🤯🏎️👆🪄 | Compute [summary statistics](https://en.wikipedia.org/wiki/Summary_statistics) (sum, min/max/range, sort order, min/max/sum/avg length, mean, standard error of the mean (SEM), stddev, variance, Coefficient of Variation (CV), nullcount, max precision, sparsity, quartiles, Interquartile Range (IQR), lower/upper fences, skewness, median, mode/s, antimode/s & cardinality) & make GUARANTEED data type inferences (Null, String, Float, Integer, Date, DateTime, Boolean) for each column in a CSV ([more info](https://github.com/dathere/qsv/wiki/Supplemental#stats-command-output-explanation)).<br>Uses multithreading to go faster if an index is present (with an index, can compile "streaming" stats on NYC's 311 data (15gb, 28m rows) in less than 7.3 seconds!). |
| [table](/src/cmd/table.rs#L2)<br>🤯 | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter). To interactively view a CSV, use the `lens` command. |
| [template](/src/cmd/template.rs#L2)<br>📇🚀🔣📚![CKAN](docs/images/ckan.png) | Renders a template using CSV data with the [MiniJinja](https://docs.rs/minijinja/latest/minijinja/) template engine ([Example](https://github.com/dathere/qsv/blob/4645ec07b5befe3b0c0e49bf0f547315d0d7514b/src/cmd/template.rs#L18-L44)). |
| [to](/src/cmd/to.rs#L2)✨<br>🚀🗄️ | Convert CSV files to [PostgreSQL](https://www.postgresql.org), [SQLite](https://www.sqlite.org/index.html), XLSX and [Data Package](https://datahub.io/docs/data-packages/tabular). |
| [tojsonl](/src/cmd/tojsonl.rs#L3)<br>📇😣🚀🔣🪄 | Smartly converts CSV to a newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV. |
| [transpose](/src/cmd/transpose.rs#L2)<br>🤯 | Transpose rows/columns of a CSV.  |
| <a name="validate_deeplink"></a>[validate](/src/cmd/validate.rs#L2)<br>📇🚀🌐📚![CKAN](docs/images/ckan.png) | Validate CSV data [_blazingly-fast_](https://github.com/Stranger6667/jsonschema-rs?tab=readme-ov-file#performance "using jsonschema-rs - the fastest JSON Schema validator for Rust") using [JSON Schema Validation (Draft 2020-12)](https://json-schema.org/draft/2020-12/json-schema-validation.html) (e.g. _up to 780,031 rows/second_[^1] using [NYC's 311 schema](https://github.com/dathere/qsv/blob/master/resources/test/311_Service_Requests_from_2010_to_Present-2022-03-04.csv.schema.json) generated by the [`schema`](#schema_deeplink) command) & put invalid records into a separate file with an accompanying detailed validation error report file.<br>Supports a custom `currency` format with [ISO-4217](https://en.wikipedia.org/wiki/ISO_4217) validation, and a custom `dynamicEnum` keyword that supports enum validation against a CSV on the filesystem, or on a URL (http/https/ckan and dathere URL schemes supported).<br>If no JSON schema file is provided, validates if a CSV conforms to the [RFC 4180 standard](#rfc-4180-csv-standard) and is UTF-8 encoded. |

<div style="text-align: right"><sub><sup>Performance metrics compiled on an M2 Pro 12-core Mac Mini with 32gb RAM</sup></sub></div>

✨: enabled by a [feature flag](#feature-flags).  
📇: uses an index when available.  
🤯: loads entire CSV into memory, though `dedup`, `stats` & `transpose` have "streaming" modes as well.  
😣: uses additional memory proportional to the cardinality of the columns in the CSV.  
🧠: expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.  
🗄️: [Extended input support](#extended-input-support).  
🐻‍❄️: command powered by [![polars 0.44.2](https://img.shields.io/badge/polars-0.44.2-blue) at the py-1.16.0 tag](https://github.com/pola-rs/polars/releases/tag/py-1.16.0).  
🤖: command uses Natural Language Processing & General AI techniques.  
🏎️: multithreaded and/or faster when an index (📇) is available.  
🚀: multithreaded even without an index.  
![CKAN](docs/images/ckan.png) : has [CKAN](https://ckan.org)-aware integration options.  
🌐: has web-aware options.  
🔣: requires UTF-8 encoded input.  
👆: has powerful column selector support. See [`select`](https://github.com/dathere/qsv/blob/master/src/cmd/select.rs#L2) for syntax.  
🪄: "automagical" commands that uses stats and/or frequency tables to work "smarter" & "faster".  
📚: has lookup table support, enabling runtime "lookups" against local or remote reference CSVs.

[^1]: see [`validate_index` benchmark](https://qsv.dathere.com/benchmarks)

## Installation Options

### Option 0: TLDR Quick Install
qsv's big brother - [qsv pro](https://qsvpro.dathere.com) is available for download from its website and on the [Microsoft Store](https://apps.microsoft.com/detail/xpffdj3f1jsztf?mode=full). Apart from a Graphical User Interface, it's superpowered with additional features and capabilities - an API, [CKAN](https://ckan.org) integration, a Natural Language interface, no false-positive Windows Defender warnings and more!

<a href="https://apps.microsoft.com/detail/xpffdj3f1jsztf?mode=full">
<img
src="https://get.microsoft.com/images/en-us%20light.svg"
width="200"  /></a>

### Option 1: Download Prebuilt Binaries

Full-featured prebuilt [binary variants](#variants) of the latest qsv version for Linux, macOS & Windows are available [for download](https://github.com/dathere/qsv/releases/latest), including binaries compiled with [Rust Nightly](https://stackoverflow.com/questions/70745970/rust-nightly-vs-beta-version) ([more info](https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md#nightly-release-builds)).

These prebuilt binaries are also built with CPU optimizations enabled for x86_64 (e.g. [SSE4.2](https://en.wikipedia.org/wiki/SSE4#SSE4.2), [AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Advanced_Vector_Extensions_2), [AVX512](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Advanced_Vector_Extensions_512), etc. on Intel and AMD processors) and Apple Silicon processors ([ARM64 SIMD NEON](https://eclecticlight.co/2021/08/06/accelerating-the-m1-mac-an-introduction-to-simd/)) for even more performance gains.

For Windows, an MSI Installer wrapping the x86_64-pc-windows-msvc build is also available for download.

For macOS, ["ad-hoc" signatures](https://users.rust-lang.org/t/distributing-cli-apps-on-macos/70223) are used to sign our binaries, so you will need to [set appropriate Gatekeeper security settings](https://support.apple.com/en-us/HT202491) or run the following command to remove the quarantine attribute from qsv before you run it for the first time:

```bash
# replace qsv with qsvlite or qsvdp if you installed those binary variants
xattr -d com.apple.quarantine qsv
```

An additional benefit of using the prebuilt binaries is that they have the `self_update` feature enabled, allowing you to quickly update qsv to the latest version with a simple `qsv --update`. For further security, the `self_update` feature only fetches [releases from this GitHub repo](https://github.com/dathere/qsv/releases) and automatically verifies the signature of the downloaded zip archive before installing the update.

> **_NOTE:_** The `luau` feature is not available in `musl` prebuilt binaries[^3].

#### Manually verifying the Integrity of the Prebuilt Binaries Zip Archives
All prebuilt binaries zip archives are signed with [zipsign](https://github.com/Kijewski/zipsign#zipsign) with the following public key [qsv-zipsign-public.key](https://github.com/dathere/qsv/raw/master/src/qsv-zipsign-public.key). To verify the integrity of the downloaded zip archives:

```bash
# if you don't have zipsign installed yet
cargo install zipsign

# verify the integrity of the downloaded prebuilt binary zip archive
# after downloading the zip archive and the qsv-zipsign-public.key file.
# replace <PREBUILT-BINARY-ARCHIVE.zip> with the name of the downloaded zip archive
# e.g. zipsign verify zip qsv-0.118.0-aarch64-apple-darwin.zip qsv-zipsign-public.key
zipsign verify zip <PREBUILT-BINARY-ARCHIVE.zip> qsv-zipsign-public.key
```

### Option 2: Package Managers & Distributions

qsv is also distributed by several package managers and distros.

[![Packaging status](https://repology.org/badge/vertical-allrepos/qsv.svg)](https://repology.org/project/qsv/versions)

```bash
# using Homebrew on macOS/Linux (https://formulae.brew.sh/formula/qsv#default)
brew install qsv

# using Scoop on Windows (https://scoop.sh/#/apps?q=qsv)
scoop install qsv

# using MacPorts on macOS (https://ports.macports.org/port/qsv/)
sudo port install qsv

# using Nixpkgs on Linux/macOS (https://search.nixos.org/packages?channel=unstable&show=qsv&from=0&size=50&sort=relevance&type=packages&query=qsv)
nix-shell -p qsv

## using other Linux package managers
# Arch Linux AUR (https://aur.archlinux.org/packages/qsv)
yay -S qsv

# Void Linux (https://voidlinux.org/packages/?arch=x86_64&q=qsv)
sudo xbps-install qsv
```

Note that qsv provided by these package managers/distros enable different features (Homebrew, for instance, only enables the `apply` and `luau` features. However, it does automatically install shell completion for `bash`, `fish` and `zsh` shells).

To find out what features are enabled in a package/distro's qsv, run `qsv --version` ([more info](https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md#version-details)).

In the true spirit of open source, these packages are maintained by volunteers who wanted to make qsv easier to install in various environments. They are much appreciated, and we loosely collaborate with the package maintainers through GitHub, but know that these packages are maintained by third-parties.

datHere also maintains a Debian package targeting the latest Ubuntu LTS on x86_64 architecture to make it easier to install qsv with DataPusher+.

To install qsv on Ubuntu/Debian:

```bash
wget -O - https://dathere.github.io/qsv-deb-releases/qsv-deb.gpg | sudo gpg --dearmor -o /usr/share/keyrings/qsv-deb.gpg
echo "deb [signed-by=/usr/share/keyrings/qsv-deb.gpg] https://dathere.github.io/qsv-deb-releases ./" | sudo tee /etc/apt/sources.list.d/qsv.list
sudo apt update
sudo apt install qsv
```

### Option 3: Install with Rust

If you have [Rust installed](https://www.rust-lang.org/tools/install), you can also install from source using Rust's cargo command[^2]:

[^2]: Of course, you'll also need a linker & a C compiler. Linux users should generally install GCC or Clang, according to their distribution’s documentation.
For example, if you use Ubuntu, you can install the `build-essential` package. On macOS, you can get a C compiler by running `$ xcode-select --install`.
For Windows, this means installing [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/). When prompted for workloads, include "Desktop Development with C++",
the Windows 10 or 11 SDK & the English language pack, along with any other language packs your require.

```bash
cargo install qsv --locked --features all_features
```

The binary will be installed in `~/.cargo/bin`.

To install different [variants](#variants) and enable optional features, use cargo `--features` (see [Feature Flags](#feature-flags) for more info):

```bash
# to install qsv with all features enabled
cargo install qsv --locked --bin qsv --features feature_capable,apply,luau,fetch,foreach,python,to,self_update,polars
# or shorthand
cargo install qsv --locked --bin qsv -F all_features

# or enable only the apply and polars features
cargo install qsv --locked --bin qsv -F feature_capable,apply,polars

# or to install qsvlite
cargo install qsv --locked --bin qsvlite -F lite

# or to install qsvdp
cargo install qsv --locked --bin qsvdp -F datapusher_plus,luau,polars
```

> **_NOTE:_** if you get compilation errors when running `cargo install`, use Option 4 to compile from source using `cargo build`. The errors are usually due to `cargo install` only using the latest release version of qsv's dependencies, and ignoring `patch.crates-io` entries in our Cargo.toml.

### Option 4: Compile from Source

Compiling from source also works similarly[^2]:

```bash
git clone https://github.com/dathere/qsv.git
cd qsv
cargo build --release --locked --bin qsv --features all_features
```

The compiled binary will end up in `./target/release/`.

To compile different [variants](#variants) and enable optional [features](#feature-flags):

```bash
# to compile qsv with all features enabled
cargo build --release --locked --bin qsv --features feature_capable,apply,luau,fetch,foreach,python,to,self_update,polars
# shorthand
cargo build --release --locked --bin qsv -F all_features

# or build qsv with only the fetch and foreach features enabled
cargo build --release --locked --bin qsv -F feature_capable,fetch,foreach

# for qsvlite
cargo build --release --locked --bin qsvlite -F lite

# for qsvdp
cargo build --release --locked --bin qsvdp -F datapusher_plus,luau,polars
```

> **_NOTE:_** To build with Rust nightly, see [Nightly Release Builds](docs/PERFORMANCE.md#nightly-release-builds).

### Variants

There are four binary variants of qsv:

* `qsv` - [feature](#feature-flags)-capable(✨), with the [prebuilt binaries](https://github.com/dathere/qsv/releases/latest) enabling all applicable features except Python [^3]
* `qsvpy` - same as `qsv` but with the Python feature enabled. Three subvariants are available - qsvpy310, qsvpy311 & qsvpy312 - which are compiled with the latest patch version of Python 3.10, 3.11 & 3.12 respectively.
* `qsvlite` - all features disabled (~13% of the size of `qsv`)
* `qsvdp` - optimized for use with [DataPusher+](https://github.com/dathere/datapusher-plus) with only DataPusher+ relevant commands; an embedded [`luau`](#luau_deeplink) interpreter; [`applydp`](#applydp_deeplink), a slimmed-down version of the `apply` feature; the `--progressbar` option disabled; and the self-update only checking for new releases, requiring an explicit `--update` (~12% of the the size of `qsv`).

> **_NOTE:_** There are "portable" subvariants of qsv available with the "p" suffix - `qsvp`, `qsvplite` and `qsvpdp`. These subvariants are compiled without any CPU features enabled. Use these subvariants if you're getting "Illegal instruction" errors when running the regular qsv binaries.

[^3]: The `luau`feature is NOT enabled by default on the prebuilt binaries for musl platforms. This is because we cross-compile using GitHub Action Runners using Ubuntu 20.04 LTS with the [musl libc](https://musl.libc.org/) toolchain. However, Ubuntu is a glibc-based, not a musl-based distro. We get around this by [cross-compiling](https://blog.logrocket.com/guide-cross-compilation-rust/).   
Unfortunately, this prevents us from cross-compiling binaries with the `luau` feature enabled as doing so requires statically linking the host OS libc library. If you need the `luau` feature on `musl`, you will need to compile from source on your own musl-based Linux Distro (e.g. Alpine, Void, [etc.](https://wiki.musl-libc.org/projects-using-musl)).  

### Shell Completion
qsv has extensive, extendable [shell completion](https://en.wikipedia.org/wiki/Command-line_completion) support. It currently supports the following shells: `bash`, `zsh`, `powershell`, `fish`, `nushell`, `fig` & `elvish`.

To enable shell completion, see the [Shell Completion](contrib/completions/README.md) documentation. If you're using Bash, you can also follow the step-by-step tutorial at [100.dathere.com](https://100.dathere.com/exercises-setup.html#optional-set-up-qsv-completions).  

## Regular Expression Syntax

The `--select` option and several commands (`apply`, `applydp`, `datefmt`, `exclude`, `fetchpost`, `replace`, `schema`, `search`, `searchset`, `select`, `sqlp` & `stats`) allow the user to specify regular expressions. We use the [`regex`](https://docs.rs/regex) crate to parse, compile and execute these expressions. [^4]

[^4]: This is the same regex engine used by [`ripgrep`](https://github.com/BurntSushi/ripgrep#ripgrep-rg) - the [blazingly fast grep replacement](https://blog.burntsushi.net/ripgrep/) that powers Visual Studio's [magical](https://lab.cccb.org/en/arthur-c-clarke-any-sufficiently-advanced-technology-is-indistinguishable-from-magic/) ["Find in Files"](https://github.com/microsoft/vscode-ripgrep) feature.

Its syntax can be found [here](https://docs.rs/regex/latest/regex/#syntax) and *"is similar to other regex engines, but it lacks several features that are not known how to implement efficiently. This includes, but is not limited to, look-around and backreferences. In exchange, all regex searches in this crate have worst case O(m * n) time complexity, where m is proportional to the size of the regex and n is proportional to the size of the string being searched."*

If you want to test your regular expressions, [regex101](https://regex101.com) supports the syntax used by the `regex` crate. Just select the "Rust" flavor.

## File formats

qsv recognizes UTF-8/ASCII encoded, CSV (`.csv`), SSV (`.ssv`) and TSV files (`.tsv` & `.tab`). CSV files are assumed to have "," (comma) as a delimiter, SSV files have ";" (semicolon) as a delimiter
and TSV files, "\t" (tab) as a delimiter. The delimiter is a single ascii character that can be set either by the `--delimiter` command-line option or
with the `QSV_DEFAULT_DELIMITER` environment variable or automatically detected when `QSV_SNIFF_DELIMITER` is set.

When using the `--output` option, qsv will UTF-8 encode the file & automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, semicolon for `.ssv`, tab for `.tsv` & `.tab` files.

JSON files are recognized & converted to CSV with the [`json`](/src/cmd/json.rs#L2) command.
[JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/) files are also recognized & converted to/from CSV with the [`jsonl`](/src/cmd/jsonl.rs#L2) and [`tojsonl`](/src/cmd/tojsonl.rs#L2) commands respectively.

The `fetch` & `fetchpost` commands also produces JSONL files when its invoked without the `--new-column` option & TSV files with the `--report` option.

The `excel`, `safenames`, `sniff`, `sortcheck` & `validate` commands produce JSON files with their JSON options following the [JSON API 1.1 specification](https://jsonapi.org/format/), so it can return detailed machine-friendly metadata that can be used by other systems.

The `schema` command produces a [JSON Schema Validation (Draft 7)](https://json-schema.org/draft/2020-12/json-schema-validation.html) file with the ".schema.json" file extension, which can be used with the `validate` command to validate other CSV files with an identical schema.

The `excel` command recognizes Excel & Open Document Spreadsheet(ODS) files (`.xls`, `.xlsx`, `.xlsm`, `.xlsb` & `.ods` files).

Speaking of Excel, if you're having trouble opening qsv-generated CSV files in Excel, set the QSV_OUTPUT_BOM environment variable to add a [Byte Order Mark](https://en.wikipedia.org/wiki/Byte_order_mark) to the beginning of the generated CSV file. This is a workaround for [Excel's UTF-8 encoding detection bug](https://stackoverflow.com/questions/155097/microsoft-excel-mangles-diacritics-in-csv-files).

The `to` command converts CSVs to `.xlsx`, [Parquet](https://parquet.apache.org) & [Data Package](https://datahub.io/docs/data-packages/tabular) files, and populates [PostgreSQL](https://www.postgresql.org) and [SQLite](https://www.sqlite.org/index.html) databases.

The `sqlp` command returns query results in CSV, JSON, JSONL, Parquet, [Apache Arrow IPC](https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format) & [Apache AVRO](https://avro.apache.org) formats. Polars SQL also supports reading external files directly in various formats with its `read_csv`, `read_ndjson`, `read_parquet` & `read_ipc` [table functions](https://github.com/pola-rs/polars/blob/91a423fea2dc067837db65c3608e3cbc1112a6fc/crates/polars-sql/src/table_functions.rs#L18-L43).

The `sniff` command can also detect the mime type of any file with the `--no-infer` or `--just-mime` options, may it be local or remote (http and https schemes supported).
It can detect more than 130 file formats, including MS Office/Open Document files, JSON, XML, PDF, PNG, JPEG and specialized geospatial formats like GPX, GML, KML, TML, TMX, TSX, TTML.
Click [here](https://docs.rs/file-format/latest/file_format/#reader-features) for a complete list.

### Extended Input Support

The `cat`, `headers`, `sqlp` & `to` commands have extended input support (🗄️). If the input is `-` or empty, the command will try to use stdin as input. If it's not, it will check if its a directory, and if so, add all the files in the directory as input files.

If its a file, it will first check if it has an `.infile-list` extension. If it does, it will load the text file and parse each line as an input file path. This is a much faster and convenient way to process a large number of input files, without having to pass them all as separate command-line arguments. Further, the file paths can be anywhere in the file system, even on separate volumes. If an input file path is not fully qualified, it will be treated as relative to the current working directory. Empty lines and lines starting with `#` are ignored. Invalid file paths will be logged as warnings and skipped.

For both directory and `.infile-list` input, snappy compressed files with a `.sz` extension will be automatically decompressed.

Finally, if its just a regular file, it will be treated as a regular input file.

### Automatic Compression/Decompression

qsv supports _automatic compression/decompression_ using the [Snappy frame format](https://github.com/google/snappy/blob/main/framing_format.txt). Snappy was chosen instead of more popular compression formats like gzip because it was designed for [high-performance streaming compression & decompression](https://github.com/google/snappy/tree/main/docs#readme) (up to 2.58 gb/sec compression, 0.89 gb/sec decompression).

For all commands except the `index`, `extdedup` & `extsort` commands, if the input file has an ".sz" extension, qsv will _automatically_ do streaming decompression as it reads it. Further, if the input file has an extended CSV/TSV ".sz" extension (e.g nyc311.csv.sz/nyc311.tsv.sz/nyc311.tab.sz), qsv will also use the file extension to determine the delimiter to use.   

Similarly, if the `--output` file has an ".sz" extension, qsv will _automatically_ do streaming compression as it writes it.
If the output file has an extended CSV/TSV ".sz" extension, qsv will also use the file extension to determine the delimiter to use.  

Note however that compressed files cannot be indexed, so index-accelerated commands (`frequency`, `schema`, `split`, `stats`, `tojsonl`) will not be multithreaded. Random access is also disabled without an index, so `slice` will not be instantaneous and `luau`'s random-access mode will not be available.

There is also a dedicated [`snappy`](/src/cmd/snappy.rs#L2) command with four subcommands for direct snappy file operations — a multithreaded `compress` subcommand (4-5x faster than the built-in, single-threaded auto-compression); a `decompress` subcommand with detailed compression metadata; a `check` subcommand to quickly inspect if a file has a Snappy header; and a `validate` subcommand to confirm if a Snappy file is valid.

The `snappy` command can be used to compress/decompress ANY file, not just CSV/TSV files.

Using the `snappy` command, we can compress NYC's 311 data (15gb, 28m rows) to 4.95 gb in _5.77 seconds_ with the multithreaded `compress` subcommand - _2.58 gb/sec_ with a 0.33 (3.01:1) compression ratio.  With `snappy decompress`, we can roundtrip decompress the same file in _16.71 seconds_ - _0.89 gb/sec_.

Compare that to [zip 3.0](https://infozip.sourceforge.net/Zip.html), which compressed the same file to 2.9 gb in _248.3 seconds on the same machine - 43x slower at 0.06 gb/sec_ with a 0.19 (5.17:1) compression ratio - for just an additional 14% (2.45 gb) of saved space. zip also took 4.3x longer to roundtrip decompress the same file in _72 seconds_ - _0.20 gb/sec_.

> **_NOTE:_** In addition to `snappy` support, the `sqlp` command also supports automatic decompression of gzip, zstd and zlib compressed input files using the [`read_csv()` table function](https://github.com/dathere/qsv/blob/aa3b20f8ba3ae41b02a3c5d445092571f064b90d/src/cmd/sqlp.rs#L120-L130). It also supports automatic compression of output files when using the Arrow, Avro and Parquet output formats (using the [`--format`](https://github.com/dathere/qsv/blob/aa3b20f8ba3ae41b02a3c5d445092571f064b90d/src/cmd/sqlp.rs#L178-L185) and [`--compression`](https://github.com/dathere/qsv/blob/aa3b20f8ba3ae41b02a3c5d445092571f064b90d/src/cmd/sqlp.rs#L246-L261) options).

## RFC 4180 CSV Standard

qsv follows the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) CSV standard. However, in real life, CSV formats vary significantly & qsv is actually not strictly compliant with the specification so it can process "real-world" CSV files.
qsv leverages the awesome [Rust CSV](https://docs.rs/csv/latest/csv/) crate to read/write CSV files.

Click [here](https://docs.rs/csv-core/latest/csv_core/struct.Reader.html#rfc-4180) to find out more about how qsv conforms to the standard using this crate.

When dealing with "atypical" CSV files, you can use the `input` & `fmt` commands to normalize them to be RFC 4180-compliant.

## UTF-8 Encoding

qsv requires UTF-8 encoded input (of which ASCII is a subset).

Should you need to re-encode CSV/TSV files, you can use the `input` command to "lossy save" to UTF-8 - replacing invalid UTF-8 sequences with `�` ([U+FFFD REPLACEMENT CHARACTER](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html)).

Alternatively, if you want to truly transcode to UTF-8, there are several utilities like [`iconv`](https://en.wikipedia.org/wiki/Iconv) that you can use to do so on [Linux/macOS](https://stackoverflow.com/questions/805418/how-can-i-find-encoding-of-a-file-via-a-script-on-linux) & [Windows](https://superuser.com/questions/1163753/converting-text-file-to-utf-8-on-windows-command-prompt).

### Windows Powershell and Windows Excel Usage Note

Unlike other modern operating systems, Microsoft Windows' [default encoding](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_character_encoding?view=powershell-7.4) [is UTF16-LE](https://stackoverflow.com/questions/66072117/why-does-windows-use-utf-16le). This will cause problems when redirecting qsv's output to a CSV file in Powershell & trying to open it with Excel - everything will be in the first column, as the UTF16-LE encoded CSV file will not be properly recognized by Excel.

```
# the following command will produce a UTF16-LE encoded CSV file on Windows
qsv stats wcp.csv > wcpstats.csv
```

Which is weird, since you'd think [Microsoft's own Excel would properly recognize UTF16-LE encoded CSV files](https://answers.microsoft.com/en-us/msoffice/forum/all/opening-csv-file-with-utf16-encoding-in-excel-2010/ed522cb9-e88d-4b82-b88e-a2d4bd99f874?auth=1). Regardless, to create a properly UTF-8 encoded file on Windows, use the `--output` option instead:

```
# so instead of redirecting stdout to a file on Windows
qsv stats wcp.csv > wcpstats.csv

# do this instead, so it will be properly UTF-8 encoded
qsv stats wcp.csv --output wcpstats.csv
```

Alternatively, qsv can add a [Byte Order Mark](https://en.wikipedia.org/wiki/Byte_order_mark) (BOM) to the beginning of a CSV to indicate it's UTF-8 encoded. You can do this by setting the `QSV_OUTPUT_BOM` environment variable to `1`.

This will allow Excel on Windows to properly recognize the CSV file as UTF-8 encoded.

Note that this is not a problem with Excel on macOS, as macOS (like most other *nixes) uses UTF-8 as its default encoding.

Nor is it a problem with qsv output files produced on other operating systems, as Excel on Windows can properly recognize UTF-8 encoded CSV files.

## Interpreters
For complex data-wrangling tasks, you can use Luau and Python scripts.

Luau is recommended over Python for complex data-wrangling tasks as it is faster, more memory-efficient, has no external dependencies and has several data-wrangling helper functions as qsv's DSL.

See [Luau vs Python](docs/INTERPRETERS.md) for more info.

## Memory Management
qsv supports three memory allocators - mimalloc (default), jemalloc and the standard allocator.<br>See [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info.

It also has Out-of-Memory prevention, with two modes - NORMAL (default) & CONSERVATIVE.<br>See [Out-of-Memory Prevention](docs/PERFORMANCE.md#out-of-memory-oom-prevention) for more info.

## Environment Variables & dotenv file support

qsv supports an extensive list of environment variables and supports `.env` files to set them.

For details, see [Environment Variables](docs/ENVIRONMENT_VARIABLES.md) and the [`dotenv.template.yaml`](dotenv.template) file.
## Feature Flags

qsv has several [feature flags](https://doc.rust-lang.org/cargo/reference/features.html) that can be used to enable/disable optional features.

See [Features](docs/FEATURES.md) for more info.

## Minimum Supported Rust Version

qsv's MSRV policy is to require the latest stable [Rust version](https://github.com/rust-lang/rust/blob/master/RELEASES.md) that is [supported by Homebrew](https://formulae.brew.sh/formula/rust#default), currently [![HomeBrew](https://img.shields.io/homebrew/v/rust?logo=homebrew)](https://formulae.brew.sh/formula/rust). 
qsv itself may upgrade its MSRV, but a new qsv release will only be made once Homebrew supports the latest Rust stable.

## Goals / Non-Goals

QuickSilver's goals, in priority order, are to be:
* **As Fast as Possible** - To do so, it has frequent releases, an aggressive MSRV policy, takes advantage of CPU features, employs [various caching strategies](docs/PERFORMANCE.md#caching), uses [HTTP/2](https://www.cloudflare.com/learning/performance/http2-vs-http1.1/#:~:text=Multiplexing%3A%20HTTP%2F1.1%20loads%20resources,resource%20blocks%20any%20other%20resource.), and is multithreaded when possible and it makes sense. It also uses the latest dependencies when possible, and will use Cargo [`patch`](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section) to get unreleased fixes/features from its dependencies. See [Performance](docs/PERFORMANCE.md) for more info.
* **Able to Process Very Large Files** - Most qsv commands are streaming, using constant memory, and can process arbitrarily large CSV files. For those commands that require loading the entire CSV into memory (denoted by 🤯), qsv has Out-of-Memory prevention, batch processing strategies and "ext"ernal commands that use the disk to process larger than memory files. See [Memory Management](docs/PERFORMANCE.md#memory-management) for more info.
* **A Complete Data-Wrangling Toolkit** - qsv aims to be a comprehensive data-wrangling toolkit that you can use for quick analysis and investigations, but is also robust enough for production data pipelines. Its many commands are targeted towards common data-wrangling tasks and can be combined/composed into complex data-wrangling scripts with its Luau-based DSL.  
Luau will also serve as the backbone of a whole library of **qsv recipes** - reusable scripts for common tasks (e.g. street-level geocoding, removing PII, data enrichment, etc.) that prompt for easily modifiable parameters.   
* **Composable/Interoperable** - qsv is designed to be composable, with a focus on interoperability with other common CLI tools like 'awk', 'xargs', 'ripgrep', 'sed', etc., and with well known ETL/ELT tools like Airbyte, Airflow, Pentaho Kettle, etc. Its commands can be combined with other tools via pipes, and it supports other common file formats like JSON/JSONL, Parquet, Arrow IPC, Avro, Excel, ODS, PostgreSQL, SQLite, etc. See [File Formats](#file-formats) for more info.
* **As Portable as Possible** - qsv is designed to be portable, with installers on several platforms with an integrated self-update mechanism. In preference order, it supports Linux, macOS and Windows. See [Installation Options](#installation-options) for more info.
* **As Easy to Use as Possible** - qsv is designed to be easy to use. As easy-to-use that is,
 as command line interfaces go :shrug:. Its commands have numerous options but have sensible defaults. The usage text is written for a data analyst audience, not developers; and there are numerous examples in the usage text, with the tests doubling as examples as well. With [qsv pro](https://qsvpro.dathere.com), it has much expanded functionality while being easier to use with its Graphical User Interface.
* **As Secure as Possible** - qsv is designed to be secure. It has no external runtime dependencies, is [written](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/) [in](https://msrc.microsoft.com/blog/2019/07/why-rust-for-safe-systems-programming/) [Rust](https://opensource.googleblog.com/2023/06/rust-fact-vs-fiction-5-insights-from-googles-rust-journey-2022.html), and it's codebase is automatically audited for security vulnerabilities with automated [DevSkim](https://github.com/microsoft/DevSkim#devskim), ["cargo audit"](https://rustsec.org) and [Codacy](https://app.codacy.com/gh/dathere/qsv/dashboard) Github Actions workflows.  
It uses the latest stable Rust version, with an aggressive MSRV policy and the latest version of all its dependencies.
It has an extensive test suite with ~1,650 tests, including several [property tests](https://medium.com/criteo-engineering/introduction-to-property-based-testing-f5236229d237) which [randomly generate](https://github.com/BurntSushi/quickcheck#quickcheck) parameters for oft-used commands.   
Its prebuilt binary archives are [zipsigned](https://github.com/Kijewski/zipsign#zipsign), so you can [verify their integrity](#verifying-the-integrity-of-the-prebuilt-binaries-zip-archives). Its self-update mechanism automatically verifies the integrity of the prebuilt binaries archive before applying an update.
See [Security](docs/SECURITY.md) for more info.
* **As Easy to Contribute to as Possible** - qsv is designed to be easy to contribute to, with a focus on maintainability. It's modular architecture allows the easy addition of self-contained commands gated by feature flags, the source code is heavily commented, the usage text is embedded, and there are helper functions that make it easy to create additional commands and supporting tests. See [Features](docs/FEATURES.md) and [Contributing](CONTRIBUTING.md) for more info.

QuickSilver's non-goals are to be:
* **As Small as Possible** - qsv is designed to be small, but not at the expense of performance, features, composability, portability, usability, security or maintainability. However, we do have a `qsvlite` variant that is ~13% of the size of `qsv` and a `qsvdp` variant that is ~12% of the size of `qsv`. Those variants, however, have reduced functionality.
Further, several commands are gated behind feature flags, so you can compile qsv with only the features you need.
* **Multi-lingual** - qsv's _usage text_ and _messages_ are English-only. There are no plans to support other languages. This does not mean it can only process English input files.  
It can process well-formed CSVs in _any_ language so long as its UTF-8 encoded. Further, it supports alternate delimiters/separators other than comma; the `apply whatlang` operation detects 69 languages; and its `apply thousands, currency and eudex` operations supports different languages and country conventions for number, currency and date parsing/formatting.  
Finally, though the default Geonames index of the `geocode` command is English-only, the index can be rebuilt with the `geocode index-update` subcommand with the `--languages` option to return place names in multiple languages ([with support for 254 languages](http://download.geonames.org/export/dump/alternatenames/)).

## Testing
qsv has ~1,650 tests in the [tests](https://github.com/dathere/qsv/tree/master/tests) directory.
Each command has its own test suite in a separate file with the convention `test_<COMMAND>.rs`.
Apart from preventing regressions, the tests also serve as good illustrative examples, and are often linked from the usage text of each corresponding command.

To test each binary variant:

```bash
# to test qsv
cargo test --features all_features

# to test qsvlite
cargo test --features lite
# to test all tests with "stats" in the name with qsvlite
cargo test stats --features lite

# to test qsvdp
cargo test --features datapusher_plus,luau,polars

# to test a specific command
# here we test only stats and use the
# t alias for test and the -F shortcut for --features
cargo t stats -F all_features

# to test a specific command with a specific feature
# here we test only luau command with the luau feature
cargo t luau -F feature_capable,luau

# to test the count command with multiple features
# we use "test_count" as we don't want to run other tests
# that have "count" in the testname - e.g. test_geocode_countryinfo
cargo t test_count -F feature_capable,luau,polars

# to test using an alternate allocator
# other than the default mimalloc allocator
cargo t --no-default-features -F all_features,jemallocator
```

## License

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjqnatividad%2Fqsv.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjqnatividad%2Fqsv?ref=badge_large)

## Origins

qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's May 2018 release](https://github.com/BurntSushi/xsv/issues/267). On top of xsv's 20 commands, it adds numerous new features; 43 additional commands; 4 `apply` subcommands & 38 operations; 4 `to` subcommands; 3 `cat` subcommands; 7 `geocode` subcommands & 4 index operations; and 4 `snappy` subcommands.
See [FAQ](https://github.com/dathere/qsv/discussions/categories/faq) for more details.

## Sponsor

<div align="center">

|qsv was made possible by|
:-------------------------:|
|[![datHere Logo](docs/images/datHere-logo-tagline.png)](https://datHere.com)<br>|
|Standards-based, best-of-breed, open source solutions<br>to make your **Data Useful, Usable & Used.**   |

</div>

## Naming Collision

This project is unrelated to [Intel's Quick Sync Video](https://www.intel.com/content/www/us/en/architecture-and-technology/quick-sync-video/quick-sync-video-general.html).
