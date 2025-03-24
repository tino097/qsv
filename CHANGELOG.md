# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.3.0] - 2025-03-23

## Highlights:
* `stats` got another round of improvements:
  * __boolean inferencing is now configurable!__<br />Before, it was limited to a simple, English-centric heuristic:
    - When a column's cardinality is 2; and the 2 values's first characters are `0/1`. `t/f` or `y/n` case-insensitive, the data type of the column is inferred as boolean
    - With the new `--boolean-patterns <arg>` option, we can now specify arbitrary `true_pattern:false_pattern` pattern pairs. Each pattern can be a string of length > 1 and are case-insensitive. If a pattern ends with "*", it is treated as a prefix.<br />For example, `t*:f*` matches "true", "Truthy", "T" as boolean true so long as the corresponding false pattern (e.g. "Fake, False, f") is also matched and the cardinality is 2.<br />
    For backwards compatibility, the default true/false pairs are `1:0,t*:f*,y*:n*`
  * __percentiles can now be computed!__<br />By enabling the `--percentiles` flag, `stats` will now return the 5th, 10th, 40th, 60th, 90th and 95th percentile by default using the [nearest-rank method](https://en.wikipedia.org/wiki/Percentile#The_nearest-rank_method) for all numeric and date/datetime columns. The returned percentiles can be configured to return different percentiles using the `--percentile-list <arg>` option.<br />
  Note that [Method 3 for computing quartiles](https://en.wikipedia.org/wiki/Quartile#Method_3), is basically a specialized implementation of the nearest rank method for q1 (25th), q2 (50th or median) and q3 (75th percentile), thus the choice of defaults for `--percentile-list`.
* `frequency`: got a performance boost now that we're using `qsv-stats` 0.32.0, which now uses the faster `foldhash` crate
* in the same vein, by replacing `ahash` with `foldhash` suite-wide, qsv got a tad faster when doing hash lookups
* `sample`: "streaming" bernoulli sampling now works for any remotely hosted CSVs with servers that support chunked downloads, without requiring range request support.
* we're now using the [latest Polars engine - v0.46.0 at the py-1.26.0 tag](https://github.com/pola-rs/polars/releases/tag/py-1.26.0).

### Added
* `stats`: add configurable boolean inferencing https://github.com/dathere/qsv/pull/2595
* `stats`: add `--percentiles` option https://github.com/dathere/qsv/pull/2617

### Changed
* refactor: replace ahash with faster foldhash https://github.com/dathere/qsv/pull/2619
* replace std `assert_eq!` macro with `similar_asserts::assert_eq!` macro for easier debugging https://github.com/dathere/qsv/pull/2605
* deps: bump polars to 0.46.0 at py-1.25.2 tag https://github.com/dathere/qsv/pull/2604
* deps: bump Polars to v0.46.0 at py-1.26.0 tag https://github.com/dathere/qsv/pull/2621
* build(deps): bump actix-web from 4.9.0 to 4.10.2 by @dependabot in https://github.com/dathere/qsv/pull/2591
* build(deps): bump indexmap from 2.7.1 to 2.8.0 by @dependabot in https://github.com/dathere/qsv/pull/2592
* build(deps): bump mimalloc from 0.1.43 to 0.1.44 by @dependabot in https://github.com/dathere/qsv/pull/2608
* build(deps): bump qsv-stats from 0.30.0 to 0.31.0 by @dependabot in https://github.com/dathere/qsv/pull/2603
* build(deps): bump qsv-stats from 0.31.0 to 0.32.0 by @dependabot in https://github.com/dathere/qsv/pull/2620
* build(deps): bump reqwest from 0.12.12 to 0.12.13 by @dependabot in https://github.com/dathere/qsv/pull/2593
* build(deps): bump reqwest from 0.12.13 to 0.12.14 by @dependabot in https://github.com/dathere/qsv/pull/2596
* build(deps): bump reqwest from 0.12.14 to 0.12.15 by @dependabot in https://github.com/dathere/qsv/pull/2609
* build(deps): bump rfd from 0.15.2 to 0.15.3 by @dependabot in https://github.com/dathere/qsv/pull/2597
* build(deps): bump rust_decimal from 1.37.0 to 1.37.1 by @dependabot in https://github.com/dathere/qsv/pull/2616
* build(deps): bump simd-json from 0.14.3 to 0.15.0 by @dependabot in https://github.com/dathere/qsv/pull/2615
* build(deps): bump tempfile from 3.18.0 to 3.19.0 by @dependabot in https://github.com/dathere/qsv/pull/2602
* build(deps): bump tempfile from 3.19.0 to 3.19.1 by @dependabot in https://github.com/dathere/qsv/pull/2612
* build(deps): bump uuid from 1.15.1 to 1.16.0 by @dependabot in https://github.com/dathere/qsv/pull/2601
* build(deps): bump zip from 2.2.3 to 2.4.1 by @dependabot in https://github.com/dathere/qsv/pull/2607
* apply select clippy lint suggestions
* bumped indirect dependencies to latest version
* set Rust nightly to 2025-03-07, the same version Polars uses https://github.com/dathere/qsv/commit/17f6bdb3f80c5798d154a133428f0ca6ff59fc79

### Fixed
* updated lock file, primarily to fix [CVE-2025-29787](https://github.com/advisories/GHSA-94vh-gphv-8pm8) https://github.com/dathere/qsv/commit/e44e5df3fd296fcf85293d46a7afe08f40b86693
* `luau`: fix flaky register_lookup_table CI test that only intermittently fails in Windows by using buffered writer in lookup `write_cache_file` helper https://github.com/dathere/qsv/commit/f494b46d334259d370c92cd8cc6b211bc81c244a
* `sample`: refactor "streaming" Bernoulli sampling, so it actually works without requiring range requests support https://github.com/dathere/qsv/pull/2600

**Full Changelog**: https://github.com/dathere/qsv/compare/3.2.0...3.3.0

## [3.2.0] - 2025-03-09

### Added
* `sample`: "streaming" bernoulli sampling of remote files when hosted on servers with range requests support  https://github.com/dathere/qsv/pull/2588

### Changed
* Updated benchmarks.sh to add Homebrew installation prompt by @ondohotola in https://github.com/dathere/qsv/pull/2575
* feat: migrate to Rust 2024 edition https://github.com/dathere/qsv/pull/2587
* deps: bump `luau` from 0.660 to 0.663 https://github.com/dathere/qsv/pull/2567
* deps: bump polars to 0.46.0 at py-1.24.0 tag https://github.com/dathere/qsv/commit/f70ce71ffa2d822aaa511e66bd11a2789786c82e
* deps: replace deprecated `simple-home-dir` with `directories` crate https://github.com/dathere/qsv/commit/6768cd59baa20b23ac9152cc8a9ce176d9e2c362
* deps: bump arrow from 54.2.0 to 54.2.1 https://github.com/dathere/qsv/commit/fc479b2b87843a370e072248e9b6331de690f0a2
* build(deps): bump bytemuck from 1.21.0 to 1.22.0 by @dependabot in https://github.com/dathere/qsv/pull/2570
* build(deps): bump console from 0.15.10 to 0.15.11 by @dependabot in https://github.com/dathere/qsv/pull/2569
* build(deps): bump governor from 0.8.0 to 0.8.1 by @dependabot in https://github.com/dathere/qsv/pull/2562
* build(deps): bump minijinja from 2.7.0 to 2.8.0 by @dependabot in https://github.com/dathere/qsv/pull/2573
* build(deps): bump minijinja-contrib from 2.7.0 to 2.8.0 by @dependabot in https://github.com/dathere/qsv/pull/2571
* build(deps): bump pyo3 from 0.23.4 to 0.23.5 by @dependabot in https://github.com/dathere/qsv/pull/2558
* build(deps): bump pyo3 from 0.23.5 to 0.24.0 by @dependabot in https://github.com/dathere/qsv/pull/2590
* build(deps): bump redis from 0.29.0 to 0.29.1 by @dependabot in https://github.com/dathere/qsv/pull/2568
* build(deps): bump robinraju/release-downloader from 1.11 to 1.12 by @dependabot in https://github.com/dathere/qsv/pull/2580
* build(deps): bump serde_json from 1.0.139 to 1.0.140 by @dependabot in https://github.com/dathere/qsv/pull/2572
* build(deps): bump tempfile from 3.17.1 to 3.18.0 by @dependabot in https://github.com/dathere/qsv/pull/2581
* build(deps): bump uuid from 1.14.0 to 1.15.0 by @dependabot in https://github.com/dathere/qsv/pull/2563
* build(deps): bump uuid from 1.15.0 to 1.15.1 by @dependabot in https://github.com/dathere/qsv/pull/2566
* applied select clippy lint suggestions
* bumped indirect dependencies to latest versions

### Fixed
* `apply`: fix `currencytonum` handling of "0.00" value by adding parsing strictness control with  `--formatstr` option https://github.com/dathere/qsv/pull/2586
* `describegpt`: fix panic by adding error handling when LLM API response format is not in expected format https://github.com/dathere/qsv/pull/2577
* `tojsonl`: fix display of floats as per the JSON spec https://github.com/dathere/qsv/pull/2583

## New Contributors
* @ondohotola made their first contribution in https://github.com/dathere/qsv/pull/2575

**Full Changelog**: https://github.com/dathere/qsv/compare/3.1.1...3.2.0

## [3.1.1] - 2025-02-24

## Highlights:
* `sample`: is now a "smart" command now uses the stats cache to validate and make sampling faster.
* With the QSV_STATSCACHE_MODE env var, you can now control the stats cache behavior suite-wide, making sure "smart" commands use it when appropriate.
* `luau` command's capabilities have been significantly expanded with:
  - New accumulate helper function for aggregating values across rows
  - Optional naming for cumulative helper functions
  - More robust error handling and improved docstrings
  - Enhanced scripting performance with fast-float parsing
  - new [Wiki section](https://github.com/dathere/qsv/wiki/Luau-Helper-Functions-Examples) with examples of using its helper functions
* `schema`: now does type-aware sorting of enum lists, making JSON Schema enum list customization easier when fine-tuning it for JSON Schema validation with `validate`.
* `lens`: adds `--freeze-columns` option with a default of 1, improving navigation of wide CSVs
* `stats`: adds `--dataset-stats` option to explicitly compute dataset-level statistics. Starting with qsv 2.0.0, it was computed automatically to support Datapusher+ and the DRUF workflow, but it was causing confusion with some command-line users.

---

### Added
* `lens`: added `--freeze-columns` option https://github.com/dathere/qsv/pull/2552
* `luau`: added accumulate helper function https://github.com/dathere/qsv/pull/2537 https://github.com/dathere/qsv/pull/2539
* `luau`: added a new section in the Wiki with examples of using the new helper functions https://github.com/dathere/qsv/wiki/Luau-Helper-Functions-Examples
* `sample`: is now "smart" - using the stats cache to validate and make sampling faster https://github.com/dathere/qsv/pull/2529 https://github.com/dathere/qsv/pull/2530 https://github.com/dathere/qsv/commit/71ec7ede121ef1e09fb19af9bac3f52aa67a7f54
* `schema`: added type-aware sort of JSON Schema enum list https://github.com/dathere/qsv/pull/2551
* `stats`: added `--dataset-stats` option https://github.com/dathere/qsv/pull/2555
* `python`: added precompiled qsvpy binary for Python 3.13 https://github.com/dathere/qsv/commit/c4087788b6fee64f358047ea8ef44a5450604ec1
* added QSV_STATSCACHE_MODE env var to control stats cache suite-wide https://github.com/dathere/qsv/commit/4afb98d8729fa4c3c5f61e0a26347dad5aa1e9f8 https://github.com/dathere/qsv/commit/2adc313937ec8aa292976f8e5acf3a4e7756fd93 https://github.com/dathere/qsv/commit/ba75f0897e5a7e6579380a8a4c073a1af436648a
* docs: updated PERFORMANCE docs and added a TLDR version https://github.com/dathere/qsv/commit/77ed167aef8f7307ec295616a8b96af2f3bb81fd https://github.com/dathere/qsv/commit/c61c249a8354ee7f4ab0d03464624f3dd3249d2b https://github.com/dathere/qsv/commit/db0bb3f147599ece48ca2e8ad1d54db83d7b897c
* chore: added *.tab & *.ssv to typos config https://github.com/dathere/qsv/commits/523667520ac06a1c96942897aa9288fe7a9d1f5d/

### Changed
* `frequency`: made error handling more robust https://github.com/dathere/qsv/commit/b195519ec04efcba7cfa7f99e153818d03f419d0
* `luau`: refactored all cumulative helper functions (cum_) now have name as an optional argument https://github.com/dathere/qsv/pull/2540
* `schema`: refactored to use QSV_STATSCACHE_MODE env var https://github.com/dathere/qsv/commit/5771ff4892ab89f8ca7d6940aa02baaa0c9b1fa5
* `select`: refactored select helper https://github.com/dathere/qsv/commit/bfbe64cc64a20006e4c93d8a3f6be3f326411fec
* `stats`: optimized memory layout of central Stats struct https://github.com/dathere/qsv/commit/52f697e5828a5c3e059d7f25254e4aef840d8598
* `stats`: optimized record_count functionality https://github.com/dathere/qsv/commit/0e3114a54a8340639c381a19251d03ab94496b04 https://github.com/dathere/qsv/commit/18791da0cc2972de2f5909fe1556d83c8b7e8f9f
* `contrib(completions)`: update qsv completions for qsv 3.1 by @rzmk in https://github.com/dathere/qsv/pull/2556
* deps: bump arrow and tempfile https://github.com/dathere/qsv/commit/4cc267972622dfb703779b3d18b084006369b449
* deps: bump cached and redis crates https://github.com/dathere/qsv/commit/e622d1447a9a8ff4ecdb22d000335fb2d129683a
* deps: bump csvlens from 0.11 to 0.12 https://github.com/dathere/qsv/commit/b2fd985bf51fac4ec224b4664cc2fe91d8676101
* deps: use our patched fork of csvlens with ability to freeze columns https://github.com/dathere/qsv/commit/d66ec6df0e768f29b1102108152f28028da0ec8b
* deps: bump polars to 0.46.0 at py-1.23.0 tag https://github.com/dathere/qsv/commit/6072aa22bed211cafa2fe90be58386acd8869415
* deps: bump flate2 from 1.0.35 to 1.1.0 https://github.com/dathere/qsv/commit/eed471a441f031d0311849a13ac3efb116baa33d
* deps: bump gzp from 0.11 to 1.0.0 https://github.com/dathere/qsv/commit/43c8a4a414484b9a3d573cb41a713ce838a2d425
* build(deps): bump jaq-json from 1.1.0 to 1.1.1 by @dependabot in https://github.com/dathere/qsv/pull/2547
* build(deps): bump jaq-core from 2.1.0 to 2.1.1 by @dependabot in https://github.com/dathere/qsv/pull/2546
* build(deps): bump log from 0.4.25 to 0.4.26 by @dependabot in https://github.com/dathere/qsv/pull/2545
* build(deps): bump tempfile from 3.16.0 to 3.17.0 by @dependabot in https://github.com/dathere/qsv/pull/2532
* build(deps): bump tempfile from 3.17.0 to 3.17.1 by @dependabot in https://github.com/dathere/qsv/pull/2535
* build(deps): bump serde_json from 1.0.138 to 1.0.139 by @dependabot in https://github.com/dathere/qsv/pull/2541
* build(deps): bump serde from 1.0.217 to 1.0.218 by @dependabot in https://github.com/dathere/qsv/pull/2542
* build(deps): bump smallvec from 1.13.2 to 1.14.0 by @dependabot in https://github.com/dathere/qsv/pull/2528
* build(deps): bump strum from 0.27.0 to 0.27.1 by @dependabot in https://github.com/dathere/qsv/pull/2533
* build(deps): bump strum_macros from 0.27.0 to 0.27.1 by @dependabot in https://github.com/dathere/qsv/pull/2534
* build(deps): bump uuid from 1.13.1 to 1.13.2 by @dependabot in https://github.com/dathere/qsv/pull/2538
* build(deps): bump uuid from 1.13.2 to 1.14.0 by @dependabot in https://github.com/dathere/qsv/pull/2544
* chore: we now have ~1,800 tests! https://github.com/dathere/qsv/commit/f5d09ed76d8e0acb9052f89b6688a047c756b053
* applied select clippy lint suggestions
* bumped indirect dependencies to latest versions
* bumped MSRV to latest Rust stable - v1.85

### Fixed
* `count`: refactored to fall back to "regular" CSV reader when Polars counting returns a zero count https://github.com/dathere/qsv/commit/fd39bcbd9574d8d5ef1ddc5025eda4748f2a8652
* `schema`: fixed off-by-one error https://github.com/dathere/qsv/commit/60de090bdf727dd0eaf79ba7058745fdacef07ef
* ensured get_stats_record helper returns field/stats correctly https://github.com/dathere/qsv/commit/ad86a373d01ea45902d764a46c19f26ad5b01029
* Fixed RUSTSEC-2025-0007: *ring* is unmaintained https://github.com/dathere/qsv/issues/2548
* `stats`: only add `qsv__value` column when `--dataset-stats` is enabled https://github.com/dathere/qsv/commit/64267d38c4161b8591a6f81e36bea6c7fdbddc70
* skip format check when path starts with temp dir or is a snappy file https://github.com/dathere/qsv/commit/ff8957e77ae4c28a24f323328c58a2549ff43c0c

### Removed
* `frequency`: removed `--stats-mode` option now that we have a suite-wide QSV_STATSCACHE_MODE env var https://github.com/dathere/qsv/commit/ba75f0897e5a7e6579380a8a4c073a1af436648a https://github.com/dathere/qsv/commit/416abb7ce73f406c2a605cdca87d50c12723698a
* chore: removed simdutf8 conditional directive for aarch64 architecture, now that its no longer needed https://github.com/dathere/qsv/commit/ec1e16c7a20a7458b560e3c78dfbd83fba82de29
* removed publish-linux-qsvpy-glibc-231-musl-123.yml workflow as it was getting cross compilation errors and we have another musl workflow that works https://github.com/dathere/qsv/commit/7c08617132e8d7df069b7b3be160d3b348f44d53

**Full Changelog**: https://github.com/dathere/qsv/compare/3.0.0...3.1.1

## [3.0.0] - 2025-02-13

## Highlights:
* `sample`: Four new sampling methods! In addition to [reservoir](https://en.wikipedia.org/wiki/Reservoir_sampling) & [indexed](https://en.wikipedia.org/wiki/Random_access) - added [bernoulli](https://en.wikipedia.org/wiki/Bernoulli_sampling), [systematic](https://en.wikipedia.org/wiki/Systematic_sampling), [stratified](https://en.wikipedia.org/wiki/Stratified_sampling), [weighted](https://doi.org/10.1016/j.ipl.2005.11.003) & [cluster](https://en.wikipedia.org/wiki/Cluster_sampling) sampling. And they're all memory efficient so you should be able to sample arbitrarily large datasets!
* `stats`: Added "sortiness" (-1 (Descending) to 1 (Ascending)) and "uniqueness_ratio" (0 (many repeated values) to 1 (All unique values)) stats. The [qsv-stats](https://github.com/dathere/qsv-stats) engine has also been optimized to squeeze out more performance.
* `diff`: make it a "smart" command, so that it uses the stats cache to short-circuit the diff if the files are identical per their fingerprint hashes, and to validate that the diff key column is all unique.

### Added
* `joinp`: additional `joinp` `asof` join sort and match options https://github.com/dathere/qsv/pull/2486
* `stats`: add "sortiness" statistic https://github.com/dathere/qsv/pull/2499
* `stats` add uniqueness_ratio https://github.com/dathere/qsv/pull/2521
* `stats` & `frequency`: add `--vis-whitespace` option. Fulfills #2501 https://github.com/dathere/qsv/pull/2503
* `sample`: add more sampling methods (in addition to indexed and reservoir - added bernoulli, systematic, stratified, weighted & cluster sampling) and made them all memory efficient so we can sample arbitrarily large datasets: https://github.com/dathere/qsv/pull/2507 & https://github.com/dathere/qsv/pull/2511
* `diff`: make `diff` a "smart" command. Fulfills #2493 and #2509 https://github.com/dathere/qsv/pull/2518
* `benchmarks` : added new benchmarks for `sample` for new sampling methods https://github.com/dathere/qsv/commit/d758c54effcef31dbc1c1eb40e0c1789050eeb34

### Changed
* `luau`: bump from 0.653 to 0.657 and optimize for performance https://github.com/dathere/qsv/commit/4402df6788205341552b4f4e43220ea49924a28e https://github.com/dathere/qsv/commit/de429b4bb858a7872e30eccbdb3e526ad0ea322b https://github.com/dathere/qsv/commit/07ff8b8458a042987c9d11cae5b5b1dfaa934097 https://github.com/dathere/qsv/commit/3211f5c84fc23b652e4d7da83098e7db46829081
* `stats`: compute string len stats only for string columns https://github.com/dathere/qsv/pull/2495
* `contrib(completions)`: update qsv completions for qsv 2.2.1 by @rzmk in https://github.com/dathere/qsv/pull/2494
* deps: bump polars to latest upstream after its py-1.22.0 release
* deps: backported csv-core 0.1.12 fix to our qsv-optimized csv-core fork https://github.com/dathere/rust-csv/commit/5d0916e243f365a377b1b0e7c84bcf9585e83f2d
* build(deps): bump actions/setup-python from 5.3.0 to 5.4.0 by @dependabot in https://github.com/dathere/qsv/pull/2488
* build(deps): bump bytes from 1.9.0 to 1.10.0 by @dependabot in https://github.com/dathere/qsv/pull/2497
* build(deps): bump data-encoding from 2.7.0 to 2.8.0 by @dependabot in https://github.com/dathere/qsv/pull/2512
* build(deps): bump geosuggest-core from 0.6.5 to 0.6.6 by @dependabot in https://github.com/dathere/qsv/pull/2520
* build(deps): bump geosuggest-utils from 0.6.5 to 0.6.6 by @dependabot in https://github.com/dathere/qsv/pull/2519
* build(deps): bump jsonschema from 0.28.3 to 0.29.0 by @dependabot in https://github.com/dathere/qsv/pull/2510
* build(deps): bump minijinja from 2.6.0 to 2.7.0 by @dependabot in https://github.com/dathere/qsv/pull/2489
* build(deps): bump mlua from 0.10.2 to 0.10.3 by @dependabot in https://github.com/dathere/qsv/pull/2485
* build(deps): bump qsv-stats from 0.27.0 to 0.28.0 by @dependabot in https://github.com/dathere/qsv/pull/2496
* build(deps): bump qsv-stats from 0.28.0 to 0.29.0 by @dependabot in https://github.com/dathere/qsv/pull/2498
* build(deps): bump qsv-stats from 0.29.0 to 0.30.0 by @dependabot in https://github.com/dathere/qsv/pull/2505
* chore: Bump rand to 0.9 https://github.com/dathere/qsv/pull/2504
* build(deps): bump simple-home-dir from 0.4.6 to 0.4.7 by @dependabot in https://github.com/dathere/qsv/pull/2515
* build(deps): bump uuid from 1.12.1 to 1.13.1 by @dependabot in https://github.com/dathere/qsv/pull/2500
* bumped numerous indirect dependencies to latest versions
* applied select clippy lint suggestions
* bumped MSRV to latest Rust stable - v1.84.1

### Fixed
* docs: QSV_AUTOINDEX => QSV_AUTOINDEX_SIZE typo. Fixes #2479 https://github.com/dathere/qsv/pull/2484
* fix: `search` & `searchset` off by 1 when using `--flag` option. Fixes #2508 https://github.com/dathere/qsv/pull/2513


**Full Changelog**: https://github.com/dathere/qsv/compare/2.2.1...3.0.0

## [2.2.1] - 2025-01-26

### Changed
* deps: bumped polars to 0.46.0. This will allow us to publish qsv to crates.io as qsv was using features that were not enabled in polars 0.45.1 https://github.com/dathere/qsv/commit/275b2b8bd3cb41d9ddf30ba721d393d446bd2b48

### Fixed
* `stats`: fix cache json processing bug. Fixes #2476  https://github.com/dathere/qsv/pull/2477
* benchmarks: v6.1.0 - ensured all `stats` cache benchmarks actually used the stats cache even if the `--cache-threshold` is 5 seconds - too high to trigger stats cache creation https://github.com/dathere/qsv/commit/ac33010260bf55c3424f8baa195f359f10ffe088

**Full Changelog**: https://github.com/dathere/qsv/compare/2.2.0...2.2.1

## [2.2.0] - 2025-01-26

## Highlights:
* `stats` - the :heart: of qsv, got a little tune-up:
  * It got a tad faster now that we only compute string length stats for string types. Previously, we were also computing length for numbers, thinking it'll be useful for storage sizing purposes (as everything is stored as string with CSV). But as [performance is goal number 1](https://github.com/dathere/qsv?tab=readme-ov-file#goals--non-goals), we're no longer doing so. Besides, this sizing info can be derived using other stats.
  * Fixed the problem with the stats cache being deleted/ignored even when not necessary.<br/>This bug snuck in while implementing the `--cache-threshold` cache suppression option. With `stats` getting its cache mojo back - expect near-instant cache-backed response not only for `stats` but also other ["automagical" smart commands 🪄](https://github.com/dathere/qsv?tab=readme-ov-file#legend_deeplink).
* `diff` - @janriemer squashed some bugs without sacrificing `diff`'s _ludicrous speed_! :wink:
* `validate`: The `dynamicEnum` custom JSON Schema keyword column specifier support.<br/>You can now specify which column to validate against (by name or by 0-based column index), instead of always using the first column. This works for local & remote lookup files using the `http/s://`, `ckan://` and `dathere://` URL schemes.
* `extdedup` now actually uses a proper memory-mapped backed on-disk hash table.<br/>Previously, it was only deduping in-memory as the odht crate was not properly wired to a memory mapped file :facepalm: (I took the name of the odht crate literally and thought it was handling it :shrug:). Thanks for the [detailed bug report](https://github.com/dathere/qsv/issues/2462) @Svenskunganka!
* JSON query parsing overhaul.<br/>The `fetch`, `fetchpost` & `json` commands now use the latest [`jaq`](https://github.com/01mf02/jaq?tab=readme-ov-file#jaq) engine, making for faster performance especially now that we're precompiling and caching the jaq filter.
* Polars engine upgraded. :polar_bear:<br/>By two versions - py-polars [1.20.0](https://github.com/pola-rs/polars/releases/tag/py-1.20.0) and [1.21.0](https://github.com/pola-rs/polars/releases/tag/py-1.21.0) - giving the `sqlp`, `joinp`, `pivotp` and `count` commands a little boost. :rocket:

---

### Added
* `diff`: add `--delimiter` "convenience" option. Fulfills #2447 https://github.com/dathere/qsv/pull/2464
* `slice`: add stdin and snappy compressed file support https://github.com/dathere/qsv/commit/ab34a623f32bd25d9ff761972f66faa85f510a5d
* `validate`: add dynamicEnum column specifier support. Fulfills #2470 https://github.com/dathere/qsv/pull/2472

### What's Changed
* `fetch`, `fetchpost` & `json`: `jaq` dependency upgrade - from `jaq-interpret` & `jaq-parse` to `jaq-core`/`jaq-json`/`jaq-std` https://github.com/dathere/qsv/pull/2458
* `fetch` & `fetchpost`: cache compiled jaq filter https://github.com/dathere/qsv/pull/2467
* `joinp`: adjust asofby test to reflect Polars py-1.20.0 behavior https://github.com/dathere/qsv/commit/853a266c866aa54598b6b1a3faa253d151a6b472
* `stats`: compute string length stats for string type only https://github.com/dathere/qsv/pull/2471
* `sqlp`: wordsmith fastpath explanation https://github.com/dathere/qsv/commit/4e3f85397f67cbe20562e8a84c228b7dc61e4bd7
* refactor: standardize -q and -Q shortcut options. Fulfills #2466 https://github.com/dathere/qsv/pull/2468
* deps: bump polars to 0.45.1 at py-polars-1.20.0 tag https://github.com/dathere/qsv/pull/2448
* deps: bump polars to 0.45.1 at py-polars-1.21.0 tag https://github.com/dathere/qsv/commit/4525d00ecd4845feaac2062d40bb7bc64c13688f
* deps: Bump csv-diff to 0.1.1 by @janriemer in https://github.com/dathere/qsv/pull/2456
* deps: Bump csvlens to latest upstream https://github.com/dathere/qsv/commit/27a723eee4af046920a022605ad6c3476c0962e4
* deps: use latest strum upstream https://github.com/dathere/qsv/commit/2ca1b0d476a20b93c786d0839cc5077e26fd6d88
* build(deps): bump base62 from 2.2.0 to 2.2.1 by @dependabot in https://github.com/dathere/qsv/pull/2440
* build(deps): bump chrono-tz from 0.10.0 to 0.10.1 by @dependabot in https://github.com/dathere/qsv/pull/2449
* build(deps): bump data-encoding from 2.6.0 to 2.7.0 by @dependabot in https://github.com/dathere/qsv/pull/2444
* build(deps): bump indexmap from 2.7.0 to 2.7.1 by @dependabot in https://github.com/dathere/qsv/pull/2461
* build(deps): bump jsonschema from 0.28.1 to 0.28.2 by @dependabot in https://github.com/dathere/qsv/pull/2469
* build(deps): bump jsonschema from 0.28.2 to 0.28.3 by @dependabot in https://github.com/dathere/qsv/pull/2473
* build(deps): bump log from 0.4.22 to 0.4.25 by @dependabot in https://github.com/dathere/qsv/pull/2439
* build(deps): bump semver from 1.0.24 to 1.0.25 by @dependabot in https://github.com/dathere/qsv/pull/2459
* build(deps): bump serde_json from 1.0.135 to 1.0.136 by @dependabot in https://github.com/dathere/qsv/pull/2455
* build(deps): bump serde_json from 1.0.136 to 1.0.137 by @dependabot in https://github.com/dathere/qsv/pull/2460
* build(deps): bump simple-home-dir from 0.4.5 to 0.4.6 by @dependabot in https://github.com/dathere/qsv/pull/2445
* build(deps): bump uuid from 1.11.1 to 1.12.0 by @dependabot in https://github.com/dathere/qsv/pull/2441
* build(deps): bump uuid from 1.12.0 to 1.12.1 by @dependabot in https://github.com/dathere/qsv/pull/2465
* tests: enabled Windows CI caching for faster CI tests 
* bumped numerous indirect dependencies to latest versions
* applied select clippy lint suggestions

### Fixed
* `count`: Sometimes, polars count returns zero even if there are rows. Fixed by doing a regular csv reader count when polars count returns zero https://github.com/dathere/qsv/commit/abcd36524d6c26a17a2ecfac54498ecab58fe87c 
* `diff`: Fix name to index conversion by @janriemer. Fixes #2443 https://github.com/dathere/qsv/pull/2457
* `extdedup`: refactor/fix to actually have on-disk hash table backed by a mem-mapped file. Fixes #2462 https://github.com/dathere/qsv/pull/2475
* `stats`: fix stats caching as it was inadvertently deleting the stats cache even when not necessary https://github.com/dathere/qsv/commit/96e6d289d31a2b22345524fb5cc71eca0d6ffae9

### Removed
* `foreach`: refactored to remove unmaintained `local-encoding` dependency https://github.com/dathere/qsv/pull/2454
* remove `polars` feature from qsvdp binary variant. We'll use py-polars from DP+ directly.

**Full Changelog**: https://github.com/dathere/qsv/compare/2.1.0...2.2.0

## [2.1.0] - 2025-01-12

### Added
* `join`: add `--ignore-leading-zeros` option https://github.com/dathere/qsv/pull/2430
* `joinp` add `--norm-unicode` option to unicode normalize join keys https://github.com/dathere/qsv/pull/2436
* `pivotp` added more smart aggregation suggestions https://github.com/dathere/qsv/pull/2428
* `template`: added to qsvdp binary variant https://github.com/dathere/qsv/commit/9df85e65dedf130981ab430764b4a4cdc9382dc8
* `benchmarks`: added `pivotp` benchmark https://github.com/dathere/qsv/commit/92e4c51cb17e5511f668b4a2cc96d9cab28a4758

### Changed
* `joinp`: refactored `--ignore-leading-zeros` handling https://github.com/dathere/qsv/pull/2433
* Migrate from unmaintained dynfmt to dynfmt2 https://github.com/dathere/qsv/pull/2421
* deps: bump csvlens to latest upstream https://github.com/dathere/qsv/commit/52c766da43642c2eef6f35819d8e9fb0966700a3
* deps: bump to latest csv qsv-optimized fork https://github.com/dathere/qsv/commit/58ac650abfa51b7b8deb23d1a8917b3983515755
* deps: bumped MiniJinja to 2.6.0 https://github.com/dathere/qsv/commit/8176368434982ba6bd206762c524a3dc47370039
* deps: bump to latest Polars upstream
* deps: bump qsv-stats to 0.26.0
* build(deps): bump azure/trusted-signing-action from 0.5.0 to 0.5.1 by @dependabot in https://github.com/dathere/qsv/pull/2420
* build(deps): bump base62 from 2.0.3 to 2.1.0 by @dependabot in https://github.com/dathere/qsv/pull/2419
* build(deps): bump base62 from 2.1.0 to 2.2.0 by @dependabot in https://github.com/dathere/qsv/pull/2426
* build(deps): bump phf from 0.11.2 to 0.11.3 by @dependabot in https://github.com/dathere/qsv/pull/2417
* build(deps): bump pyo3 from 0.23.3 to 0.23.4 by @dependabot in https://github.com/dathere/qsv/pull/2431
* build(deps): bump serde_json from 1.0.134 to 1.0.135 by @dependabot in https://github.com/dathere/qsv/pull/2416
* build(deps): bump tokio from 1.42.0 to 1.43.0 by @dependabot in https://github.com/dathere/qsv/pull/2423
* build(deps): bump uuid from 1.11.0 to 1.11.1 by @dependabot in https://github.com/dathere/qsv/pull/2427
* apply several clippy suggestions
* bumped numerous indirect dependencies to latest versions
* bumped Rust nightly from 2024-12-19 to 2025-01-05 (same version used by Polars)
* bump MSRV to latest Rust stable - v1.84.0

### Fixed
* `join`: revert optimization that actually resulted in a performance regression https://github.com/dathere/qsv/commit/e42af2b4e9ab9ef4eed43b97e343e253c50a35a1
* `join`: `--right-anti` and `--right-semi` joins didn't swap headers properly https://github.com/dathere/qsv/pull/2435
* `count`: polars-powered `count` didn't use the right data type SQL count(*) https://github.com/dathere/qsv/commit/d8c1524ca0dff4ac19164ccb8090b01fd740b571

**Full Changelog**: https://github.com/dathere/qsv/compare/2.0.0...2.1.0

## [2.0.0] - 2025-01-06

## qsv v2.0.0 is here! 🎉
It took 193 releases to get to v1.0.0, and we're already at v2.0.0 a month later!?!

Yes! We wanted a running start for 2025, and qsv 2.0.0 marks qsv's biggest release yet!

* It fully enables the "Data Resource Upload First (DRUF)" workflow, allowing Datapusher+ to infer ["automagical metadata"](https://dathere.com/2023/11/automagical-metadata/) from the data itself. It exposes two Domain Specific Language (DSL) options - [Luau](https://luau.org) and [MiniJinja](https://docs.rs/minijinja/latest/minijinja/) - to enable powerful data transformation and validation capabilities. This allows data stewards to upload data first, then use qsv's DSL capabilities inside DP+ to automatically generate rich metadata - including data dictionaries, field descriptions, data quality rules, and data validation schemas. This "automagical metadata" approach dramatically reduces the friction in compiling high-quality, high-resolution metadata (using the [DCAT-US 3.0 specification](https://doi-do.github.io/dcat-us/) as a reference) that would otherwise be a manual, laborious, and error-prone process.  
Under the hood, the `fetchpost`, `template`, `stats`, and `luau` commands now have the necessary scaffolding to fully support this workflow inside Datapusher+ and ckanext-scheming.
* It adds a new `pivotp` command, powered by Polars, to enable fast pivot operations on large datasets. You can now pivot your data in seconds by simply specifying the columns to pivot on while blowing past [Excel's PivotTable limitations](https://support.microsoft.com/en-us/office/excel-specifications-and-limits-1672b34d-7043-467e-8e27-269d656771c3).
* `stats` now computes geometric mean and harmonic mean and adds string length stats, all while getting a performance boost.
* `join` and `joinp` got a lot of love in this release, with several new options:
  * `joinp`: non-equi join support! 🎉💯🥳<br/> See ["Lightning Fast and Space Efficient Inequality Joins" paper](https://vldb.org/pvldb/vol8/p2074-khayyat.pdf) and this [Polars non-equi join tracking issue](https://github.com/pola-rs/polars/issues/10068).
  * `join` & `joinp`: `--right-anti` and `--right-semi` joins
  * `joinp`: `--ignore-leading-zeros` option for join keys
  * `joinp`: `--maintain-order` option to maintain the order of the either the left or right dataset in the output
  * `joinp`: expanded `--cache-schema` options to make `joinp` smarter/faster by leveraging the stats cache
  * `join`: `--keys-output` option to write successfully joined keys to a separate output file.

This release lays the groundwork for the [`outliers` "smart" command](https://github.com/dathere/qsv/issues/107) to quickly identify outliers using stats/frequency info.

It also sets the stage for an initial implementation of our ["Data Concierge"](https://dathere.com/2024/12/the-golden-rule-of-pragmatic-data-governance/) that leverages all the high-quality, high-res metadata we automagically compile with DRUF to enable *Metadata Gardening Agents* to proactively link seemingly unrelated data and glean insights as it constantly grooms the Data Catalog - effectively making it a ___[FAIR Data](https://www.go-fair.org/fair-principles/) Factory___.

---

### Added
* `fetchpost`: add `--globals-json` option  https://github.com/dathere/qsv/pull/2357
* `fixlengths`: add `--remove-empty` option; refactored for performance. Fulfills #2391. https://github.com/dathere/qsv/pull/2411
* `join`: add `--keys-output` option. Fulfills #2407. https://github.com/dathere/qsv/pull/2408
* `join`: add `--right-anti` and `--right-semi` options. Fulfills #2379. https://github.com/dathere/qsv/pull/2380
* `joinp`: add non-equi join support! 🎉💯🥳   https://github.com/dathere/qsv/pull/2409
* `joinp`: add `--ignore-leading-zeros` option. Fulfills #2398.  https://github.com/dathere/qsv/pull/2400
* `joinp`: add `--maintain-order` option  https://github.com/dathere/qsv/pull/2338
* `joinp`: add `--right-anti` and `--right-semi` options. Fulfills #2377. https://github.com/dathere/qsv/pull/2378
* `luau`: addl helper functions. Fulfills #1782. https://github.com/dathere/qsv/pull/2362
* `luau`: add `qsv_writejson` helper  https://github.com/dathere/qsv/pull/2375
* `pivotp`: new polars polars-powered command. Fulfills #799.  https://github.com/dathere/qsv/pull/2364
* `pivotp`: "smart" pivotp. https://github.com/dathere/qsv/pull/2367
* `stats`: add geometric mean and harmonic mean. Fulfills #2227. https://github.com/dathere/qsv/pull/2342
* `stats`: add string length stats to set stage for upcoming `outliers` "smart"  command to quickly identify outliers using stats/frequency info  https://github.com/dathere/qsv/pull/2390
* `template`: add `--globals-json` option  https://github.com/dathere/qsv/pull/2356
* `tojsonl`: add `--quiet` option. Fulfills #2335. https://github.com/dathere/qsv/pull/2336
* `validate`: add `--validate-schema` option to check if the JSON Schema itself is valid  https://github.com/dathere/qsv/pull/2393
* `contrib(completions)`: add joinp `--ignore-case` and slice `--invert` by @rzmk in https://github.com/dathere/qsv/pull/2322
* `contrib(completions)`: add `--quiet` to `tojsonl` by @rzmk in https://github.com/dathere/qsv/pull/2337
* `ci`: add qsv_glibc_2.31-headless to action by @rzmk in https://github.com/dathere/qsv/pull/2330
* Add license to MSI installer by @rzmk in https://github.com/dathere/qsv/pull/2321

### Changed
* `lens`: optimized csvlens library usage, dropping clap dependency  https://github.com/dathere/qsv/pull/2403
* `pivotp`: an even smarter `pivotp`  https://github.com/dathere/qsv/pull/2368
* `stats`: performance boost https://github.com/dathere/qsv/commit/51349ba8f0121804a1a6766371f1e17c0da800b6
* Update deb package by @tino097 in https://github.com/dathere/qsv/pull/2226
* `ci`: attempt using files-folder instead of files by @rzmk in https://github.com/dathere/qsv/pull/2320
* Setting QSV_FREEMEMORY_HEADROOM_PCT to 0 disables memory availability check  https://github.com/dathere/qsv/pull/2353
* build(deps): bump actix-governor from 0.7.0 to 0.8.0 by @dependabot in https://github.com/dathere/qsv/pull/2351
* build(deps): bump bytemuck from 1.20.0 to 1.21.0 by @dependabot in https://github.com/dathere/qsv/pull/2361
* build(deps): bump chrono from 0.4.38 to 0.4.39 by @dependabot in https://github.com/dathere/qsv/pull/2345
* build(deps): bump crossbeam-channel from 0.5.13 to 0.5.14 by @dependabot in https://github.com/dathere/qsv/pull/2354
* build(deps): bump flexi_logger from 0.29.6 to 0.29.7 by @dependabot in https://github.com/dathere/qsv/pull/2348
* build(deps): bump governor from 0.7.0 to 0.8.0 by @dependabot in https://github.com/dathere/qsv/pull/2347
* build(deps): bump itertools from 0.13.0 to 0.14.0 by @dependabot in https://github.com/dathere/qsv/pull/2413
* build(deps): bump jsonschema from 0.26.1 to 0.26.2 by @dependabot in https://github.com/dathere/qsv/pull/2355
* build(deps): bump jsonschema from 0.26.2 to 0.27.0 by @dependabot in https://github.com/dathere/qsv/pull/2371
* build(deps): bump jsonschema from 0.27.1 to 0.28.0 by @dependabot in https://github.com/dathere/qsv/pull/2389
* build(deps): bump jsonschema from 0.28.0 to 0.28.1 by @dependabot in https://github.com/dathere/qsv/pull/2396
* bump polars from 0.44.2 to 0.45  https://github.com/dathere/qsv/pull/2340
* build(deps): bump polars from 0.45.0 to 0.45.1 by @dependabot in https://github.com/dathere/qsv/pull/2344
* bump pyo3 from 0.22 to 0.23 now that Polars supports it  https://github.com/dathere/qsv/pull/2352
* build(deps): bump redis from 0.27.5 to 0.27.6 by @dependabot in https://github.com/dathere/qsv/pull/2331
* build(deps): bump reqwest from 0.12.9 to 0.12.11 by @dependabot in https://github.com/dathere/qsv/pull/2385
* build(deps): bump reqwest from 0.12.11 to 0.12.12 by @dependabot in https://github.com/dathere/qsv/pull/2395
* build(deps): bump rfd from 0.15.1 to 0.15.2 by @dependabot in https://github.com/dathere/qsv/pull/2404
* build(deps): bump serde from 1.0.215 to 1.0.216 by @dependabot in https://github.com/dathere/qsv/pull/2349
* build(deps): bump serde from 1.0.216 to 1.0.217 by @dependabot in https://github.com/dathere/qsv/pull/2384
* build(deps): bump serde_json from 1.0.133 to 1.0.134 by @dependabot in https://github.com/dathere/qsv/pull/2365
* build(deps): bump sysinfo from 0.32.1 to 0.33.0 by @dependabot in https://github.com/dathere/qsv/pull/2334
* build(deps): bump sysinfo from 0.33.0 to 0.33.1 by @dependabot in https://github.com/dathere/qsv/pull/2383
* deps: bump tabwriter to 1.4.1  https://github.com/dathere/qsv/commit/bbcbeba193b7b1808bcd359c460fb688b49107f0
* build(deps): bump tokio from 1.41.1 to 1.42.0 by @dependabot in https://github.com/dathere/qsv/pull/2333
* build(deps): bump xxhash-rust from 0.8.12 to 0.8.13 by @dependabot in https://github.com/dathere/qsv/pull/2359
* build(deps): bump xxhash-rust from 0.8.13 to 0.8.14 by @dependabot in https://github.com/dathere/qsv/pull/2372
* build(deps): bump xxhash-rust from 0.8.14 to 0.8.15 by @dependabot in https://github.com/dathere/qsv/pull/2392
* apply several clippy suggestions
* bumped numerous indirect dependencies to latest versions
* bumped Rust nightly from 2024-11-28 to 2024-12-19 (same version used by Polars)

### Fixed
* `joinp`: refactor `--cache-schema` option. Resolves #2369. https://github.com/dathere/qsv/pull/2370
* `extsort` underflow in CSV mode. Resolves #2391. https://github.com/dathere/qsv/pull/2412
* instantiate logger properly https://github.com/dathere/qsv/commit/9c0c1a7a63ef3773e599f6fa91e6fa3b734936df
* fix `util::get_stats_records()`  to no longer infer boolean in `StatsMode::PolarsSchema`. Resolves #2369. https://github.com/dathere/qsv/commit/cebb6642daf8b528ed8c95be9fc47709abe1c50a

**Full Changelog**: https://github.com/dathere/qsv/compare/1.0.0...2.0.0

## [1.0.0] - 2024-12-02

## qsv v1.0.0 is here! 🎉
After over 3 years of development, nearly 200 releases, and 11,000+ commits, qsv has finally reached v1.0.0!

What started as a hobby project to learn Rust during COVID has evolved into a powerful data wrangling tool used in multiple datHere products, open source projects, and even in several mission-critical production environments!

To mark this major milestone, this larger than usual release includes major performance improvements, new features, and various optimizations!

---

### Added
* `joinp`: add `--ignore-case` option https://github.com/dathere/qsv/pull/2287
* `py`: add ability to load python expression from file https://github.com/dathere/qsv/pull/2295
* `replace`: add `--not-one` flag (resolves #2305) by @rzmk in https://github.com/dathere/qsv/pull/2307
* `slice`: add `--invert` option https://github.com/dathere/qsv/pull/2298
* `stats`: add dataset-level stats https://github.com/dathere/qsv/pull/2297
* `sqlp`: auto-decompression of gzip, zstd & zlib compressed csv files with `read_csv` table function (implements suggestion from @wardi in #2301) https://github.com/dathere/qsv/pull/2315
* `template`: add lookup support https://github.com/dathere/qsv/pull/2313
* added `ui` feature to make it easier to make a headless build of qsv https://github.com/dathere/qsv/pull/2289
* added better panic handling https://github.com/dathere/qsv/pull/2304
* added new benchmark for `template` command https://github.com/dathere/qsv/commit/cd7e480de5ff1e2766a16b8d21767b76fbf10d35
* added 📚 `lookup support` legend https://github.com/dathere/qsv/commit/b46de73f57ba35ee08581a4f20809a5f581d461b

### Changed
* move qsv from personal Github repo to datHere GitHub org https://github.com/dathere/qsv/pull/2317
* `template`: parallelized template rendering for significant speedups https://github.com/dathere/qsv/pull/2273
* simplify input format check https://github.com/dathere/qsv/pull/2309
* bump embedded `luau` from 0.650 to 0.653 https://github.com/dathere/qsv/commit/986a1d3b4e60f15c25ef8a157c7e9e205ae8e7a9
* deps: Switch back to `simple-home-dir` from `simple-expand-tilde` https://github.com/dathere/qsv/pull/2319
* deps: Add minijinja contrib https://github.com/dathere/qsv/pull/2276
* deps: bump pyo3 down to 0.21.2 because polars-mem-engine is not compatible with pyo3 0.23.x yet https://github.com/dathere/qsv/commit/7f9fc8a6cfe94a104d33e895ecae11e2f40274ee
* build(deps): bump base62 from 2.0.2 to 2.0.3 by @dependabot in https://github.com/dathere/qsv/pull/2281
* build(deps): bump bytemuck from 1.19.0 to 1.20.0 by @dependabot in https://github.com/dathere/qsv/pull/2299
* build(deps): bump bytes from 1.8.0 to 1.9.0 by @dependabot in https://github.com/dathere/qsv/pull/2314
* build(deps): bump file-format from 0.25.0 to 0.26.0 by @dependabot in https://github.com/dathere/qsv/pull/2277
* build(deps): bump hashbrown from 0.15.1 to 0.15.2 by @dependabot in https://github.com/dathere/qsv/pull/2310
* build(deps): bump itoa from 1.0.11 to 1.0.12 by @dependabot in https://github.com/dathere/qsv/pull/2300
* build(deps): bump itoa from 1.0.12 to 1.0.13 by @dependabot in https://github.com/dathere/qsv/pull/2302
* build(deps): bump itoa from 1.0.13 to 1.0.14 by @dependabot in https://github.com/dathere/qsv/pull/2311
* build(deps): bump mlua from 0.10.0 to 0.10.1 by @dependabot in https://github.com/dathere/qsv/pull/2280
* build(deps): bump mlua from 0.10.1 to 0.10.2 by @dependabot in https://github.com/dathere/qsv/pull/2316
* build(deps): bump serial_test from 3.1.1 to 3.2.0 by @dependabot in https://github.com/dathere/qsv/pull/2279
* build(deps): bump minijinja from 2.4.0 to 2.5.0 by @dependabot in https://github.com/dathere/qsv/pull/2284
* build(deps): bump minijinja-contrib from 2.3.1 to 2.5.0 by @dependabot in https://github.com/dathere/qsv/pull/2283
* build(deps): bump rfd from 0.15.0 to 0.15.1 by @dependabot in https://github.com/dathere/qsv/pull/2291
* build(deps): bump sanitize-filename from 0.5.0 to 0.6.0 by @dependabot in https://github.com/dathere/qsv/pull/2275
* build(deps): bump serde from 1.0.214 to 1.0.215 by @dependabot in https://github.com/dathere/qsv/pull/2286
* build(deps): bump serde_json from 1.0.132 to 1.0.133 by @dependabot in https://github.com/dathere/qsv/pull/2292
* build(deps): bump tempfile from 3.13.0 to 3.14.0 by @dependabot in https://github.com/dathere/qsv/pull/2278
* build(deps): bump tokio from 1.41.0 to 1.41.1 by @dependabot in https://github.com/dathere/qsv/pull/2274
* build(deps): bump url from 2.5.3 to 2.5.4 by @dependabot in https://github.com/dathere/qsv/pull/2306
* applied several clippy suggestions
* bumped numerous indirect dependencies to latest versions
* bumped MSRV to latest Rust stable (1.83.0)
* bumped Rust nightly from 2024-11-01 to 2024-11-28, the same version used by Polars

### Fixed
* fix `get_stats_records()` helper to handle input files with embedded spaces (fixes #2294) https://github.com/dathere/qsv/pull/2296
* added better panic handling (fixes #2301) https://github.com/dathere/qsv/pull/2304
* implement simple format check for input files (fixes #2308) https://github.com/dathere/qsv/pull/2308

### Removed
* removed `simple-expand-tilde` dependency in favor of `simple-home-dir` https://github.com/dathere/qsv/pull/2318
* removed patched fork of `indicatif` now that 0.17.9 is released, fixing GH unmaintained advisory for `instant` https://github.com/dathere/qsv/commit/33fa54a1651ce29d286c0e1ff4f3d77bbbd2ffd5
* removed `clipboard` command from `qsvlite` binary variant https://github.com/dathere/qsv/commit/9c663d84da49cbbe53d7c9df6bd747cad0d9ba24

**Full Changelog**: https://github.com/dathere/qsv/compare/0.138.0...1.0.0

## [0.138.0] - 2024-11-05

## Highlights:
* __:star: New `template` command for rendering templates with CSV data.__  
This should allow users to generate very complex documents (Form letters, JSON/XML files, etc.) with the powerful [MiniJinja template engine](https://docs.rs/minijinja/latest/minijinja/) ([Example template](https://github.com/jqnatividad/qsv/blob/master/scripts/template.tpl)).   

* __:star: New `lookup` module for fetching reference data from remote and local files.__  
In addition to the typical `http`/`https` schemes for remote files, qsv adds two additional schemes - `CKAN://` and `datHere://`, fetching lookup data from a CKAN site or [datHere maintained](https://data.dathere.com) [reference data](https://github.com/dathere/qsv-lookup-tables) respectively. The lookup module has simple file-based caching as well to minimize repeated fetching of typically static reference data (default cache age: 600 seconds).  
The `lookup` module is now being used by the `luau` (for its [`qsv_register_lookup`](https://github.com/jqnatividad/qsv/blob/9036430b1902701eaf60058afce7823810968099/src/cmd/luau.rs#L2034-L2070) helper) and `validate` (for its [`dynamicEnum`](https://github.com/jqnatividad/qsv/blob/9036430b1902701eaf60058afce7823810968099/src/cmd/validate.rs#L35-L72) custom JSON Schema keyword) commands. More commands will take advantage of this module over time (e.g. `apply`, `geocode`, `template`, `sqlp`, etc.) to do extended lookups (e.g. lookup Census information given spatiotemporal data - like demographic info of a Census tract).
* __:sparkles: Enhanced `fetchpost` with MiniJinja templating for payload construction.__  
Previously, `fetchpost` was limited to posting url-encoded HTML Form data. Now with the `--payload-tpl` and `--content-type` options, users can render and post request bodies using MiniJinja using other content types as well (typically `application/json`, `text/plain`, `multipart/form-data`).
* __:sparkles: Improved Polars integration with automatic schema detection__  
The `joinp` and `sqlp` commands now use qsv's stats cache to automatically determine column data types, rather than having Polars scan a sample of rows. This provides two key benefits:
  1. Faster execution by skipping Polars' schema inference step
  2. More accurate data type detection since the stats cache analyzes the entire dataset, not just a sample
* __:running: `fast-float2` crate for faster float parsing__  
Casting string/bytes to float is now much faster ([2 to 8x faster than Rust's standard library](https://github.com/Alexhuszagh/fast-float-rust?tab=readme-ov-file#performance)) with `fast-float2`.
* __:muscle: Major dependency updates including [Polars 0.44.2](https://github.com/pola-rs/polars/releases/tag/rs-0.44.2), [Luau 0.650](https://github.com/luau-lang/luau/releases/tag/0.650), [mlua 0.10.0](https://github.com/mlua-rs/mlua/releases/tag/v0.10.0) and [jsonschema 0.26.1](https://github.com/Stranger6667/jsonschema/releases/tag/rust-v0.26.1)__  
These core crates underpin much of qsv's functionality. Using the latest version of these crates allow qsv to stay true to its goal of being the [fastest and most comprehensive data-wrangling toolkit](https://github.com/jqnatividad/qsv?tab=readme-ov-file#goals--non-goals).

---

### Added
* added lookup module - enabling fetching and caching of reference data from remote and local files https://github.com/jqnatividad/qsv/pull/2262
* `fetchpost`: add `--payload-tpl <file>` and `--content-type` options to construct payload using MiniJinja with the appropriate content-type https://github.com/jqnatividad/qsv/pull/2268 https://github.com/jqnatividad/qsv/commit/592149867997da6ac56d20a7e7f84252b2baeb2a
* `joinp`: derive polars schema from stats cache https://github.com/jqnatividad/qsv/commit/86fe22ee4e3677dc702eaf21175c60ceb8166001
* `sqlp`: derive polars schema from stats cache https://github.com/jqnatividad/qsv/pull/2256
* `template`: new command to render MiniJinja templates with CSV data https://github.com/jqnatividad/qsv/pull/2267
* `validate`: add `dynamicEnum` lookup support https://github.com/jqnatividad/qsv/pull/2265
* `contrib(completions)`: add template command and update fetchpost by @rzmk in https://github.com/jqnatividad/qsv/pull/2269
* add `fast-float2` dependency for faster bytes to float conversion https://github.com/jqnatividad/qsv/commit/7590e4ed171eeb6804845e1b54bec0fa26cca706 https://github.com/jqnatividad/qsv/commit/3ca30aa878ed3c4dc58944d46f53fb0c4b955356
* added more benchmarks for new/updated commands https://github.com/jqnatividad/qsv/commit/f8a1d4fff11d78860c102c1375653822ee95ca58 https://github.com/jqnatividad/qsv/commit/cd7e480de5ff1e2766a16b8d21767b76fbf10d35

### Changed
* `luau`: adapt to mlua 0.10 API changes https://github.com/jqnatividad/qsv/commit/268cb45a04a49360befb81af76cc1cddd6307286
* `luau`: refactored stage management https://github.com/jqnatividad/qsv/commit/31ef58a82b8f80fe0b29260f9170f10220c73714
* `luau`: now uses the lookup module https://github.com/jqnatividad/qsv/commit/2f4be3473a90252df4fd559a5f3b38246a3da696
* `stats`: minor perf refactoring https://github.com/jqnatividad/qsv/commit/6cdd6ea94adbae063e7fb6d9da71dac0c86adc12
* build(deps): bump actions/setup-python from 5.2.0 to 5.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2243
* build(deps): bump azure/trusted-signing-action from 0.4.0 to 0.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2239
* build(deps): bump bytes from 1.7.2 to 1.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2231
* build(deps): bump cached from 0.53.1 to 0.54.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2272
* build(deps): bump flexi_logger from 0.29.3 to 0.29.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/2229
* build(deps): bump flexi_logger from 0.29.4 to 0.29.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/2261
* build(deps): bump flexi_logger from 0.29.5 to 0.29.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/2266
* build(deps): bump hashbrown from 0.15.0 to 0.15.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2270
* build(deps): bump jsonschema from 0.24.0 to 0.24.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2234
* build(deps): bump jsonschema from 0.24.1 to 0.24.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2238
* build(deps): bump jsonschema from 0.24.2 to 0.24.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2240
* build(deps): bump jsonschema from 0.25.0 to 0.25.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2244
* build(deps): bump jsonschema from 0.26.0 to 0.26.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2260
* build(deps): bump regex from 1.11.0 to 1.11.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2242
* build(deps): bump reqwest from 0.12.8 to 0.12.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/2258
* build(deps): bump serde from 1.0.210 to 1.0.211 by @dependabot in https://github.com/jqnatividad/qsv/pull/2232
* build(deps): bump serde from 1.0.211 to 1.0.213 by @dependabot in https://github.com/jqnatividad/qsv/pull/2236
* build(deps): bump serde from 1.0.213 to 1.0.214 by @dependabot in https://github.com/jqnatividad/qsv/pull/2259
* build(deps): bump simd-json from 0.14.1 to 0.14.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2235
* build(deps): bump tokio from 1.40.0 to 1.41.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2237
* `deps`: updated our fork of the csv crate with more perf optimizations https://github.com/jqnatividad/qsv/commit/eae7d764bd31d717bdf123646ea85c81ed829829
* `deps`: use calamine upstream with unreleased fixes https://github.com/jqnatividad/qsv/commit/4cc7f37e9c34b712ae2c5f43c018b2d6a6655ebb
* `deps`: use our csvlens fork untl PR removing unneeded arboard features is merged https://github.com/jqnatividad/qsv/commit/bb3232205b7a948848c2949bcaf3b54e54f3d49b
* `deps`: bump jsonschema from 0.25 to 0.26 https://github.com/jqnatividad/qsv/pull/2251
* `deps`: bump embedded Luau from 0.640 to 0.650 https://github.com/jqnatividad/qsv/commit/8c54b875bf8768849b128ab15d96c33b02be180b https://github.com/jqnatividad/qsv/commit/aca30b072ecb6bb22d7edbe8ddef348649a5d699
* `deps`: bump mlua from 0.9 to 0.10  https://github.com/jqnatividad/qsv/pull/2249
* `deps`: bump Polars from 0.43.1 at py-1.11.0 tag to latest 0.44.2 upstream  https://github.com/jqnatividad/qsv/pull/2255 https://github.com/jqnatividad/qsv/commit/0e40a4429b4ef219ab7a11c91767e95778470ef2
* apply select clippy lint suggestions
* updated indirect dependencies
* aligned Rust nightly to Polars nightly - 2024-10-28 - https://github.com/jqnatividad/qsv/commit/245bcb55af416960aa603c05de960289f6125c5c

### Fixed
* fix documentation typo: it's → its by @tmtmtmtm in https://github.com/jqnatividad/qsv/pull/2254

### Removed
* removed need to set RAYON_NUM_THREADS env var and just call the Rayon API directly https://github.com/jqnatividad/qsv/commit/aa6ef89eceac89c3d1ed19068e0e23a451c4402d
* removed unneeded `create_dir_all_threadsafe` helper now that std::create_dir_all is threadsafe https://github.com/jqnatividad/qsv/commit/d0af83bfbd0430fa22f039bd00615380110f456e

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.137.0...0.138.0

## [0.137.0] - 2024-10-20

### Highlights:
* `extdedup` and `extsort` now support two modes - LINE mode and CSV mode. Previously, both commands only sorted on a line-by-line basis (now called LINE MODE).<br/>
With the addition of CSV MODE, you can now deduplicate or sort CSV files on a column-by-column basis, with the powerful `--select` option to specify which columns to deduplicate or sort on. This is especially useful for large CSV files with many columns, where you only want to deduplicate or sort on a subset of columns.
And since both commands use the disk and are streaming, they can handle files of any size.
* `sqlp` now has a `--cache-schema` option that caches the schema of the input CSV file, which can significantly speed up subsequent queries on the same file.
* `fetch` and `fetchpost` have been updated to use the [`jaq`](https://github.com/01mf02/jaq?tab=readme-ov-file#jaq) (a [jq](https://jqlang.github.io/jq/)-like tool for parsing JSON) crate instead of the `jql` crate. This change was made to improve performance and to make the commands more consistent with the `json` command which also uses `jaq`. Furthermore, `jaq` is a clone of `jq` - which is widely used and has a large community, so it should be more familiar to users.
* `stats` is a tad faster as we keep squeezing more performance from this central command.
* `validate` is now faster and more memory efficient due to optimizations in the `jsonschema` crate and minor performance improvements in the `validate` command itself.

---

### Added
* `extdedup`: now supports two modes - LINE mode and CSV mode https://github.com/jqnatividad/qsv/pull/2208
* `extsort`: now also has two modes - CSV mode and LINE mode https://github.com/jqnatividad/qsv/pull/2210
* `sqlp`: add `--cache-schema` option https://github.com/jqnatividad/qsv/pull/2224
* added `sqlp --cache-schema` benchmarks

### Changed
* `apply` & `applydp`: use smallvec for operations vector & other minor performance optimizations https://github.com/jqnatividad/qsv/pull/2219 & https://github.com/jqnatividad/qsv/commit/bc837ae698f3aee06ea9b846b98ea0c75820a22d
* `apply` & `applydp`: specify min_length for parallel iterators https://github.com/jqnatividad/qsv/commit/7d6ce5ec9675755abd5942a5e9e731592961700d
* `fetch` & `fetchpost`: replace jql with jaq https://github.com/jqnatividad/qsv/pull/2222
* `stats`: performance optimizations https://github.com/jqnatividad/qsv/commit/f205809549ac275078a95bc2821a583611955ad0 https://github.com/jqnatividad/qsv/commit/e26c27f58df688d7bfb2185ad54d4fe010b1fccf https://github.com/jqnatividad/qsv/commit/4579c1bfba4eca21d7480694780e39f6966a88a0
* `validate`: specify min_length for parallel iterators https://github.com/jqnatividad/qsv/commit/a5b818562d5db7d65f00e5acd2c8bf7d44bd869a
* build(deps): bump calamine from 0.26.0 to 0.26.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2204
* build(deps): bump csvs_convert from 0.8.14 to 0.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2215
* build(deps): bump flexi_logger from 0.29.2 to 0.29.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2209
* build(deps): bump jsonschema from 0.23.0 to 0.24.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2223
* build(deps): bump pyo3 from 0.22.3 to 0.22.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/2207
* build(deps): bump pyo3 from 0.22.4 to 0.22.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/2212
* build(deps): bump redis from 0.27.3 to 0.27.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/2202
* build(deps): bump redis from 0.27.4 to 0.27.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/2217
* build(deps): bump serde_json from 1.0.129 to 1.0.130 by @dependabot in https://github.com/jqnatividad/qsv/pull/2218
* build(deps): bump serde_json from 1.0.131 to 1.0.132 by @dependabot in https://github.com/jqnatividad/qsv/pull/2220
* build(deps): bump uuid from 1.10.0 to 1.11.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2213
* apply select clippy lints
* bumped indirect dependencies
* bumped MSRV to 1.82

### Fixed:
* fix performance regression in batched commands by refactoring `optimal_batch_size` to require indexed CSV files https://github.com/jqnatividad/qsv/pull/2206

### Removed:
* `fetch` & `fetchpost`: removed jql options; replaced with jaq https://github.com/jqnatividad/qsv/pull/2222

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.136.0...0.137.0

## [0.136.0] - 2024-10-08

## Highlights

# :tada: qsv pro is now available in the Microsoft Store! :tada:
It's ***Data Wrangling Democratized*** on the Desktop, featuring:

  - __:bar_chart: Familiar Spreadsheet Interface__<br/>tap the power of qsv to query, analyze, enrich, scrub and transform huge Excel files and multi-gigabyte CSV files in seconds, without having to deal with the command-line.
  - __![CKAN](docs/images/ckan.png) CKAN desktop client__<br/>designed to make data publishing easier for portal operators and data stewards using the ![CKAN](docs/images/ckan.png) [CKAN](https://ckan.org) platform.
  - __:inbox_tray: Flow__<br/>allows you to build custom node-based flows and data pipelines using a visual interface.
  - __:wrench: Toolbox__<br/>features an ever-expanding library of reusable scripts for common data-wrangling use cases.
  - __:star: and more!__<br/>Natural Language Interface ([RAG](https://docs.google.com/presentation/d/10T_3MyIqS5UsKxJaOY7Ktrd-GfhJelQImlE_qYmtuis/edit#slide=id.g2e10e05624b_0_124)), [Polars](https://pola.rs) SQL query support, an API, Python/Luau support, automatic Data Dictionaries, [DCAT 3 metadata profile inferencing](https://github.com/jqnatividad/qsv/issues/1705), along with a retinue of other cloud-based services (e.g. customizable street-level geocoding, data feeds, reference data lookups, geo-ip lookups, cloud storage support, [`.qsv` file format](https://github.com/jqnatividad/qsv/issues/1982), etc.) that will be unveiled in future versions.

Like qsv, we're iterating rapidly with qsv pro, so your feedback is essential. Give it a try!
<div dir="rtl">Get it from https://qsvpro.dathere.com or<br/><a href="https://apps.microsoft.com/detail/xpffdj3f1jsztf?mode=full">
<img
src="https://get.microsoft.com/images/en-us%20light.svg"
width="200"  /></a></div>

__Other highlights:__
* `excel`: new `--table` option for XLSX files; new `--header-row` option;  expanded `--range` option, adding support for Named Ranges and absolute ranges (e.g. `Sheet2!$A$1:$J$10`); and expanded metadata export now including Named Ranges and Tables (for XLSX files)
* Improved performance for several commands (`apply`, `datefmt`, `tojsonl` and `validate`) through automatic batch size optimization
* `validate`: `dynamicEnum` custom JSON Schema keyword in validate command (renamed from `dynenum`) and enhanced email validation
* `schema`: automatic JSON Schema `const` inferencing for columns with just one value
* Significant dependency updates, including latest upstream versions of Polars, jsonschema, and serde_json with unreleased performance upgrades, new features and fixes

> __NOTE:__ You can also see __qsv__ & __qsv pro__ in action in our ["The Problem with Data Portals" webinar](https://us06web.zoom.us/webinar/register/5317284045017/WN_wTe4l6nlTWa6C0HDs8R2PA) Oct 23, 2024. 1-2pm EDT
---

### Added
* :tada: [__qsv pro is now in the Microsoft Store!!!__](https://apps.microsoft.com/detail/xpffdj3f1jsztf?mode=full) :tada:
* `apply`, `datefmt`, `tojsonl`, `validate`: added logic to automatically determine optimal batch size for better parallelization https://github.com/jqnatividad/qsv/pull/2178
* `enum`: added `--new-column` support for all enum modes, not just `--increment` https://github.com/jqnatividad/qsv/pull/2173
* `excel`: new `--table` option for XLSX files https://github.com/jqnatividad/qsv/pull/2194
* `excel`: new `--header-row` option https://github.com/jqnatividad/qsv/commit/458f79ad9f4da504c68d73b48e83ad53b9634027
* `excel`: expanded range and metadata options https://github.com/jqnatividad/qsv/pull/2195
* `schema`: added JSON Schema automatic `const` inferencing https://github.com/jqnatividad/qsv/pull/2180
* Add signing step to qsv MSI installer GitHub Action by @rzmk in https://github.com/jqnatividad/qsv/pull/2182
* `contrib(completions)`: add `--table` option to `qsv excel` by @rzmk in https://github.com/jqnatividad/qsv/pull/2197
* `completions`: add `--header-row` option to `qsv excel` https://github.com/jqnatividad/qsv/commit/e8794d569185245f857659cdc299ea86029dd841
* added new `apply operations sentiment` benchmark https://github.com/jqnatividad/qsv/commit/b745e6438b64686810e4d1df4fa2e6748ba93ff8
* `docs`: added indexing section to PERFORMANCE.md https://github.com/jqnatividad/qsv/commit/804145a5304091c36728a8cdde4d56f879f71c15

### Changed
* `stats`: various minor micro-optimizations https://github.com/jqnatividad/qsv/commit/62d95fc6db2c34916160db88e4235719749a5f23 https://github.com/jqnatividad/qsv/commit/2c2862a75d6c0b2651516da30a7e6207a0043670
* `validate`: renamed custom keyword `dynenum` to `dynamicEnum` to be more consistent with JSON schema naming conventions https://github.com/jqnatividad/qsv/compare/0.135.0...master#diff-9783631cdad9e1f47f60266303dc2d56a6e7a486784b61c40961601e8192f7cf
* `validate`: optimizations for increased performance; replace serde_json with simd_json https://github.com/jqnatividad/qsv/compare/0.135.0...master#diff-9783631cdad9e1f47f60266303dc2d56a6e7a486784b61c40961601e8192f7cf
* apply new `clippy::ref_option` lint to Config::new API https://github.com/jqnatividad/qsv/pull/2192
* Update debian package readme by @tino097 in https://github.com/jqnatividad/qsv/pull/2187
* `deps`: bump `calamine` from 0.25 to 0.26 https://github.com/jqnatividad/qsv/commit/b42279a66144264bde9333068c47c530e3945f8c
* `deps`: `jsonschema` use [latest 0.22.3 upstream with unreleased features/fixes](https://github.com/jqnatividad/qsv/blob/f44d4c95db034d0770a5ee7df42a472aba7f4dd5/Cargo.toml#L300)
* `deps`: `polars` use [latest 0.43.1 upstream with unreleased features/fixes](https://github.com/jqnatividad/qsv/blob/1c1174b3b8b65d9dfd9c841597366fb09d0a047c/Cargo.toml#L311-L322)
* `deps`: created our own fork of unmaintained vader_sentiment crate https://github.com/jqnatividad/qsv/commit/b4267610f39d13eb8939c86f3b5e70033aa95a0c
* `deps`: use `serde_json` upstream with unreleased perf improvement/fixes https://github.com/jqnatividad/qsv/blob/1c1174b3b8b65d9dfd9c841597366fb09d0a047c/Cargo.toml#L221
* build(deps): bump flate2 from 1.0.33 to 1.0.34 by @dependabot in https://github.com/jqnatividad/qsv/pull/2171
* build(deps): bump flexi_logger from 0.29.0 to 0.29.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2189
* build(deps): bump flexi_logger from 0.29.1 to 0.29.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2196
* build(deps): bump hashbrown from 0.14.5 to 0.15.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2186
* build(deps): bump jsonschema from 0.20.0 to 0.21.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2177
* build(deps): bump jsonschema from 0.22.1 to 0.22.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2191
* build(deps): bump regex from 1.10.6 to 1.11.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2176
* build(deps): bump reqwest from 0.12.7 to 0.12.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/2183
* build(deps): bump simd-json from 0.14.0 to 0.14.1 https://github.com/jqnatividad/qsv/pull/2199
* build(deps): bump simple-expand-tilde from 0.4.2 to 0.4.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2190
* build(deps): bump sysinfo from 0.31.4 to 0.32.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2193
* build(deps): bump tempfile from 3.12.0 to 3.13.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2175
* apply select clippy lints
* bumped indirect dependencies
* aligned Rust nightly to Polars nightly - 2024-09-29 https://github.com/jqnatividad/qsv/commit/7cd2de1151b2299d9b75a9c8b1a3e21dc9c992e2

### Fixed
* `schema`: fix `enum` so it only adds a list when the number of unique values > `--enum-threshold` https://github.com/jqnatividad/qsv/pull/2180
* Upload artifact fix for Debian package publishing by @tino097 in https://github.com/jqnatividad/qsv/pull/2168
* fixed typos configuration https://github.com/jqnatividad/qsv/commit/627de891d8fd358aadf8c302552e8a99c54ed959
* fixed various GitHub Actions publishing workflow issues

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.135.0...0.136.0

## [0.135.0] - 2024-09-24

### Highlights
JSON Schema validation just got a whole lot more powerful with the introduction of the `dynenum` keyword!
With `dynenum`, you can now dynamically lookup valid enum values from a CSV (on the filesystem or on a URL), allowing for more flexible and responsive data validation.  

Unlike the standard[`enum` keyword](https://json-schema.org/draft/2020-12/draft-bhutton-json-schema-validation-01#name-enum), `dynenum` does not require hardcoding valid values at schema definition time, and can be used to validate data against a changing set of valid values.  

In an upcoming qsv pro release, we're making `dynenum` even more powerful by allowing you to specify  high-value reference data (e.g. US Census data, World Bank data, etc.) that is maintained at [data.dathere.com](https://data.dathere.com) and other CKAN instances.

This release also add the custom [`currency` JSON Schema format](https://github.com/jqnatividad/qsv/blob/90257bbba6d0b1c59c7a6c104b05beae35ae97e1/src/cmd/validate.rs#L23-L31), which enables currency validation according to the [ISO 4217 standard](https://en.wikipedia.org/wiki/ISO_4217).

The Polars engine was also updated to [0.43.1](https://github.com/pola-rs/polars/releases/tag/rs-0.43.1) at the [py-1.81.1 tag](https://github.com/pola-rs/polars/releases/tag/py-1.81.1) - making for various under-the-hood improvements for the `sqlp`, `joinp` and `count` commands, as we set the stage for more [Polars-powered features in future releases](https://github.com/jqnatividad/qsv/issues?q=is%3Aissue+is%3Aopen+label%3Apolars).

---

### Added
* `foreach`: enabled `foreach` command on Windows prebuilt binaries https://github.com/jqnatividad/qsv/commit/def9c8fa98cd214f0db839b64bcd12764dcfba43
* `lens`: added support for QSV_SNIFF_DELIMITER env var and snappy auto-decompression https://github.com/jqnatividad/qsv/commit/8340e8949c4b60669bc95c432c661a8c374ca422
* `sample`: add `--max-size` option https://github.com/jqnatividad/qsv/commit/e845a3cc1dcbbceda86bb7fe132c5040d23ce78b
* `validate`: added `dynenum` custom JSON Schema keyword for dynamic validation lookups https://github.com/jqnatividad/qsv/pull/2166
* `tests`: add tests for https://100.dathere.com/lessons/2 by @rzmk in https://github.com/jqnatividad/qsv/pull/2141
* added `stats_sorted` and `frequency_sorted` benchmarks
* added `validate_dynenum` benchmarks

### Changed
* `json`: add error for empty key and update usage text by @rzmk in https://github.com/jqnatividad/qsv/pull/2167
* `prompt`: gate `prompt` command behind `prompt` feature https://github.com/jqnatividad/qsv/pull/2163
* `validate`: expanded `currency` JSON Schema custom format to support ISO 4217 currency codes and alternate formats https://github.com/jqnatividad/qsv/commit/5202508e5c3969b279c20cf80bb1e37d89afd826
* `validate`: migrate to new `jsonschema` crate api https://github.com/jqnatividad/qsv/commit/5d6505426c652e7db4bb602c1bf9d302e6a09214
* Update ubuntu version for deb package by @tino097 in https://github.com/jqnatividad/qsv/pull/2126
* move --help output from stderr to stdout https://github.com/jqnatividad/qsv/pull/2138
* `contrib(completions)`: update completions for qsv v0.134.0 and fix subcommand options by @rzmk in https://github.com/jqnatividad/qsv/pull/2135
* `contrib(completions)`: add `--max-size` completion for `sample` by @rzmk in https://github.com/jqnatividad/qsv/pull/2142
* `deps`: bump to polars 0.43.1 at py-1.81.1 https://github.com/jqnatividad/qsv/pull/2130
* `deps`: switch back to calamine upstream instead of our fork https://github.com/jqnatividad/qsv/commit/677458faa4439b1b34c8a3556687a031ed184e4e
* build(deps): bump actix-governor from 0.5.0 to 0.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2146
* build(deps): bump anyhow from 1.0.87 to 1.0.88 by @dependabot in https://github.com/jqnatividad/qsv/pull/2132
* build(deps): bump arboard from 3.4.0 to 3.4.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2137
* build(deps): bump bytes from 1.7.1 to 1.7.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2148
* build(deps): bump geosuggest-core from 0.6.3 to 0.6.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/2153
* build(deps): bump geosuggest-utils from 0.6.3 to 0.6.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/2154
* build(deps): bump jql-runner from 7.1.13 to 7.2.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2165
* build(deps): bump jsonschema from 0.18.1 to 0.18.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2127
* build(deps): bump jsonschema from 0.18.2 to 0.18.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2134
* build(deps): bump jsonschema from 0.18.3 to 0.19.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2144
* build(deps): bump jsonschema from 0.19.1 to 0.20.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2152
* build(deps): bump pyo3 from 0.22.2 to 0.22.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2143
* build(deps): bump rfd from 0.14.1 to 0.15.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2151
* build(deps): bump simple-expand-tilde from 0.4.0 to 0.4.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2129
* build(deps): bump qsv_currency from 0.6.0 to 0.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2159
* build(deps): bump qsv_docopt from 1.7.0 to 1.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2136
* build(deps): bump redis from 0.26.1 to 0.27.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2133
* build(deps): bump simdutf8 from 0.1.4 to 0.1.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/2164
* bump indirect dependencies
* apply select clippy lint suggestions
* several usage text/documentation improvements
* bump MSRV to 1.81.0

### Fixed
* `validate`: correct `fail_validation_error!` macro; reformat error messages to use hyphens as the JSONschema error message already starts with "error:" https://github.com/jqnatividad/qsv/commit/9a2552481a07759847efe6025b402297ecba7e19
* moved `--help` output from stderr to stdout as per [GNU CLI guidelines](https://www.gnu.org/prep/standards/standards.html#g_t_002d_002dhelp) https://github.com/jqnatividad/qsv/commit/2b7dbdc68d49b67fb80c58cc7678cd3f2c112bd9
* `lens`: fixed parsing of lens options https://github.com/jqnatividad/qsv/commit/1cdd1bcac29fd2411521ac95fa87595de74cbb1b
* `searchset`: fixed usage text for <regexset-file> https://github.com/jqnatividad/qsv/commit/9a60fb088a326ee97ed1b147c4c3686b6b8aaeeb
* [used patched forks of `arrow`, `csvlens` and `xlsxwriter` crates](https://github.com/jqnatividad/qsv/blob/90257bbba6d0b1c59c7a6c104b05beae35ae97e1/Cargo.toml#L270-L315) that replaces a dependency on an old version of `lexical-core` with known soundness issues - https://rustsec.org/advisories/RUSTSEC-2023-0086. Once those crates have updated their `lexical-core`dependency, we will revert to the original crates.

### Removed
* removed `prompt` command from qsvlite https://github.com/jqnatividad/qsv/pull/2163
* publish: remove `lens` feature from i686 targets as it does not compile https://github.com/jqnatividad/qsv/commit/959ca7686f8656c98de9257d11f1f762852bdf9d
* `deps`: remove anyhow dependency https://github.com/jqnatividad/qsv/pull/2150

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.134.0...0.135.0

## [0.134.0] - 2024-09-10

## qsv pro v1 is here! 🎉
If you've been using qsv for a while, even if you're a command-line ninja, you'll find a lot of new capabilities in qsv pro that can make your data wrangling experience even better!

Apart from making qsv easier to use, qsv pro has a multitude of features including: view interactive data tables; browse stats/frequency/metadata; run recipes and tools (scripts); run Polars SQL queries; an interface using Retrieval Augmented Generation (RAG) techniques to attempt converting Natural Language queries to Polars SQL queries; regular expression search; export to multiple file formats; download/upload from/to compatible CKAN instances; design custom node-based flows and data pipelines; interact with a local API from external programs including the qsv pro command; run various qsv commands in a graphical user interface; and the list goes on!

That's just the beginning, there's more to come! You just have to try it!

Download qsv pro v1 now at [qsvpro.dathere.com](https://qsvpro.dathere.com/).

Other highlights include:

- `pro`: new command to allow qsv to interact with the qsv pro API to tap into qsv pro exclusive features.
- `lens`: new command to interactively view CSVs using the [csvlens](https://github.com/YS-L/csvlens) crate.
- The ludicrously fast `diff` command is now easier to use with its `--drop-equal-fields` option. @janriemer continues to work on his `csv-diff` crate, and there's more `diff` UX improvements coming soon!
- `stats` adds `sum_length` and `avg_length` "streaming" statistics in addition to the existing `min_length` and `max_length` metrics. These are especially useful for datasets with a lot of "free text" columns.
- `stats` also got "smarter" and "faster" by [dog-fooding](https://en.wikipedia.org/wiki/Eating_your_own_dog_food) its own statistics to make it run faster!  
&nbsp;  
It's a little complicated, but the way `stats` works is that it compiles the "streaming" statistics on the fly first, and the more expensive advanced statistics are "lazily" computed at the end.  
Since we now compile "sort order" in a streaming manner, we use this info when deriving cardinality at the end to see if we can skip sorting - an otherwise necessary step to get cardinality which is done by "scanning" all the sorted values of a column. Everytime two neighboring values differ in a sortedcolumn, it increments the cardinality count.  
Apart from this "sort order" optimization, we also improved the "cardinality scan" algorithm - halving its memory footprint and making it faster still for larger datasets by parallelizing the computation!  
This in turn, makes the `frequency` command faster and more memory efficient!
- we now also use our own fork of the `csv` crate, featuring SIMD-accelerated UTF-8 validation and other minor perf tweaks, making the *entire qsv suite* faster still!

---

### Added
* `pro`: add `qsv pro` command to interact with qsv pro API by @rzmk in https://github.com/jqnatividad/qsv/pull/2039
* `lens`: new command to interactively view CSVs using the [csvlens](https://github.com/YS-L/csvlens) crate https://github.com/jqnatividad/qsv/pull/2117
* `apply`: add crc32 operation https://github.com/jqnatividad/qsv/pull/2121
* `count`: add --delimiter option https://github.com/jqnatividad/qsv/pull/2120
* `diff`: add flag `--drop-equal-fields` by @janriemer in https://github.com/jqnatividad/qsv/pull/2114
* `stats`: add `sum_length` and `avg_length` columns https://github.com/jqnatividad/qsv/pull/2113
* `stats`: smarter cardinality computation - added new parallel algorithm for large datasets (10,000+ rows) and updated sequential algorithm for smaller datasets https://github.com/jqnatividad/qsv/commit/4e63fec61a394ef2ddfa499c0cdd0958e677ad17

### Changed
* `count`: added comment to justify magic number https://github.com/jqnatividad/qsv/commit/5241e3972c05f024a0791be04632d03a06b2f9ce
* `stats`: use simdjson for faster JSONL parsing; micro-optimize `compute` hot loop https://github.com/jqnatividad/qsv/commit/0e8b73451999a3e95bfd52246b1088aecd64b88f
* `stats`: standardized OVERFLOW and UNDERFLOW messages https://github.com/jqnatividad/qsv/commit/38c61285704e5064a63c9dbb1ac866f18fa130fd
* `sort`: renamed symbol so eliminate devskim lint false positive warning https://github.com/jqnatividad/qsv/commit/12db7397f68d3199e3311f402d5c7afed586b88c
* enable `lens` feature in GH workflows https://github.com/jqnatividad/qsv/pull/2122
* `deps`: bump polars 0.42.0 to latest upstream at time of release https://github.com/jqnatividad/qsv/commit/3c17ed12c3c763d644d9713afcc8442964f22de3
* `deps`: use our own optimized fork of csv crate, with simdutf8 validation and other minor perf tweaks https://github.com/jqnatividad/qsv/commit/e4bcd7123172fa8d8094c305d7780e151c120db1
* build(deps): bump serde from 1.0.209 to 1.0.210 by @dependabot in https://github.com/jqnatividad/qsv/pull/2111
* build(deps): bump serde_json from 1.0.127 to 1.0.128 by @dependabot in https://github.com/jqnatividad/qsv/pull/2106
* build(deps): bump qsv-stats from 0.19.0 to 0.22.0 https://github.com/jqnatividad/qsv/pull/2107 https://github.com/jqnatividad/qsv/pull/2112 https://github.com/jqnatividad/qsv/commit/cb1eb60a0a9fb3b9ba381183a2c29909f82efa42
* apply select clippy lint suggestions
* updated several indirect dependencies
* made various doc and usage text improvements

### Fixed
* `schema`: Print an error if the `qsv stats` invocation fails by @abrauchli in https://github.com/jqnatividad/qsv/pull/2110

## New Contributors
* @abrauchli made their first contribution in https://github.com/jqnatividad/qsv/pull/2110

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.133.1...0.134.0

## [0.133.1] - 2024-09-03

### Highlights
This release doubles down on Polars' capabilities, as we now, as a matter of [policy track the latest polars upstream](https://github.com/jqnatividad/qsv/blob/0801f678fd55af01ff53f80ee6b22b508e7c3dfb/Cargo.toml#L283-L294). If you think qsv has a torrid release schedule, you should [see Polars](https://github.com/pola-rs/polars/releases). They're constantly fixing bugs, adding new features and optimizations!  
To keep up, we've added Polars revision info to the `--version` output, and the `--envlist` option now includes Polars relevant env vars. We've also added a new `POLARS_BACKTRACE_IN_ERR` env var to control whether Polars backtraces are included in error messages.  
We also removed the `to parquet` subcommand as its redundant with the Polars-powered `sqlp`'s ability to create parquet files. This also removes the HUGE duckdb dependency, which should markedly make compile times shorter and binaries much smaller.

Other highlights include:
- New `edit` command that allows you to edit CSV files.
- The `count` command's `--width` option now includes record width stats beyond max length (avg, median, min, variance, stddev & MAD).
- The `fixlengths` command now has `--quote` and `--escape` options.
- The `stats` command adds a `sort_order` streaming statistic.

---

### Added
* `count`: expanded `--width` options, adding record width stats beyond max length (avg, median, min, variance, stddev & MAD). Also added `--json` output when using `--width`  https://github.com/jqnatividad/qsv/pull/2099
* `edit`: add `qsv edit` command by @rzmk in https://github.com/jqnatividad/qsv/pull/2074
* `fixlengths`: added `--quote` and `--escape` options https://github.com/jqnatividad/qsv/pull/2104
* `stats`: add `sort_order` streaming statistic https://github.com/jqnatividad/qsv/pull/2101
* `polars`: add polars revision info to `--version` output https://github.com/jqnatividad/qsv/commit/e60e44f99061c37758bd53dfa8511c16d49ceed5
* `polars`: added Polars relevant env vars to `--envlist` option https://github.com/jqnatividad/qsv/commit/0ad68fed94f7b5059cca6cf96cec4a3b55638e60
* `polars`: add & document `POLARS_BACKTRACE_IN_ERR` env var https://github.com/jqnatividad/qsv/commit/f9cc5595664d4665f0b610fcbac93c30fa445056

### Changed
* Optimize polars optflags https://github.com/jqnatividad/qsv/pull/2089
* `deps`: bump polars 0.42.0 to latest upstream at time of release https://github.com/jqnatividad/qsv/commit/3b7af519343f08919f114c7307f0f561d04f93e8
* bump polars to latest upstream, removing smartstring https://github.com/jqnatividad/qsv/pull/2091
* build(deps): bump actions/setup-python from 5.1.1 to 5.2.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2094
* build(deps): bump flate2 from 1.0.32 to 1.0.33 by @dependabot in https://github.com/jqnatividad/qsv/pull/2085
* build(deps): bump flexi_logger from 0.28.5 to 0.29.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2086
* build(deps): bump indexmap from 2.4.0 to 2.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2096
* build(deps): bump jsonschema from 0.18.0 to 0.18.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2084
* build(deps): bump serde from 1.0.208 to 1.0.209 by @dependabot in https://github.com/jqnatividad/qsv/pull/2082
* build(deps): bump serde_json from 1.0.125 to 1.0.127 by @dependabot in https://github.com/jqnatividad/qsv/pull/2079
* build(deps): bump sysinfo from 0.31.2 to 0.31.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2077
* build(deps): bump qsv-stats from 0.18.0 to 0.19.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2100
* build(deps): bump tokio from 1.39.3 to 1.40.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2095
* apply select clippy lint suggestions
* updated several indirect dependencies
* made various doc and usage text improvements
* pin Rust nightly to 2024-08-26 from 2024-07-26, aligning with Polars pinned nightly

### Fixed
* Ensure portable binaries are "added" to the publish zip archive, instead of replacing all the binaries with just the portable version. Fixes #2083. https://github.com/jqnatividad/qsv/commit/34ad2067007a86ffad6355f7244163c4105a98f2

### Removed
* removed `to parquet` subcommand as its redundant with `sqlp`'s ability to create parquet files. This also removes the HUGE duckdb dependency, which should markedly make compile times shorter and binaries much smaller https://github.com/jqnatividad/qsv/pull/2088
* removed `smartstring` dependency now that Polars has its own compact inlined string type https://github.com/jqnatividad/qsv/commit/47f047e6ee10916b5caa19ee829471e9fb6f4bea
* remove `to parquet` benchmark

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.132.0...0.133.1

## [0.133.0] - 2024-09-03
SKIPPED because `cargo publish` was not publishing to crates.io because of a dev dependency issue with `csvs_convert` crate.

## [0.132.0] - 2024-08-21

### Highlights
With this release, we finally finish the `stats` caching refactor started in 0.131.0, replacing the binary encoded stats cache with a simpler JSONL cache. The `stats` cache stores the necessary statistical metadata to make several key commands smarter & faster. Per the [benchmarks](https://qsv.dathere.com/benchmarks):

- `frequency` is 6x faster (`frequency_index_stats_mode_auto`).   
Not only is it faster, it now doesn't need to compile a hashmap for columns with ALL unique values (e.g. ID columns) - practically, making it able to handle "real-world" datasets of any size (that is, unless all the columns have ALL unique cardinalities. In that case, the entire CSV will have to fit into memory).
- `tojsonl` is 2.67x faster (`tojsonl_index`)
- `schema` is two orders of magnitude (100x) faster!!! (`schema_index`)

The stats cache also provides the foundation for even more "smart" features and commands in the future.  It also has the side-benefit of adding a way to produce stats in JSONL  format that can be used for other purposes beyond qsv.

The `search`, `searchset`, and `replace` commands now also have a `--literal` option that allows you to search for and replace strings with regex special/reserved characters. This makes it easier to search for and replace strings that contain special characters without having to escape them.

---

### Added
* `search`, `searchset` & `replace`: add `--literal` option https://github.com/jqnatividad/qsv/pull/2060 & https://github.com/jqnatividad/qsv/commit/7196053b36c8886092fe25fd030ccf1cf765ed6a
* `slice`: added usage text examples https://github.com/jqnatividad/qsv/commit/04afaa3d5a6e51c75f3f9041515c1d7986c45777
* `publish`: added workflow to build "portable" binaries with CPU features disabled
* `contrib(completions)`: add `--literal` for `search` and `searchset` by @rzmk in https://github.com/jqnatividad/qsv/pull/2061
* `contrib(completions)`: add `--literal` completion to `replace` by @rzmk in https://github.com/jqnatividad/qsv/pull/2062
* add more polars metadata in `--version` info https://github.com/jqnatividad/qsv/pull/2073
* `docs`: added more info to SECURITY.md https://github.com/jqnatividad/qsv/commit/609d4df61c93de6959f07e8d972009ae6cd12b78
* `docs`: expanded Goals/Non-Goals https://github.com/jqnatividad/qsv/commit/54998e36eb4608a1fba7938836e5985b699e32ff
* `docs`: added Installation "Option 0" quick start https://github.com/jqnatividad/qsv/commit/bf5bf82105397436d901de247398fce3e808b122
* added `search --literal` benchmark

### Changed
* `stats`, `schema`, `frequency` & `tojsonl`: stats caching refactor, replacing binary encoded stats cache with a simpler JSONL cache https://github.com/jqnatividad/qsv/pull/2055
* rename `stats --stats-json` option to `stats --stats-jsonl` https://github.com/jqnatividad/qsv/pull/2063
* changed "broken pipe" error to a warning https://github.com/jqnatividad/qsv/commit/73532759a8dad2d643f283296aa402950765b648
* `docs`: update multithreading and caching sections of PERFORMANCE.md https://github.com/jqnatividad/qsv/commit/5e6bc455bc544003535e18f99493cc1a20c4a2ce
* `deps`: switch to our qsv-optimized fork of csv crate https://github.com/jqnatividad/qsv/commit/3fc1e82c83b5dec23d3ba610e3d0f9bbd2924788
* `deps`: bump polars from 0.41.3 to 0.42.0 https://github.com/jqnatividad/qsv/pull/2051
* build(deps): bump actix-web from 4.8.0 to 4.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2041
* build(deps): bump flate2 from 1.0.31 to 1.0.32 by @dependabot in https://github.com/jqnatividad/qsv/pull/2071

* build(deps): bump indexmap from 2.3.0 to 2.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2049
* build(deps): bump reqwest from 0.12.6 to 0.12.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/2070
* build(deps): bump rust_decimal from 1.35.0 to 1.36.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2068
* build(deps): bump serde from 1.0.205 to 1.0.206 by @dependabot in https://github.com/jqnatividad/qsv/pull/2043
* build(deps): bump serde from 1.0.206 to 1.0.207 by @dependabot in https://github.com/jqnatividad/qsv/pull/2047
* build(deps): bump serde from 1.0.207 to 1.0.208 by @dependabot in https://github.com/jqnatividad/qsv/pull/2054
* build(deps): bump serde_json from 1.0.122 to 1.0.124 by @dependabot in https://github.com/jqnatividad/qsv/pull/2045
* build(deps): bump serde_json from 1.0.124 to 1.0.125 by @dependabot in https://github.com/jqnatividad/qsv/pull/2052
* apply select clippy lint suggestions
* updated several indirect dependencies
* made various usage text improvements

### Fixed
* `stats`: fix `--output` delimiter inferencing based on file extension https://github.com/jqnatividad/qsv/pull/2065
* make process_input helper handle stdin better https://github.com/jqnatividad/qsv/pull/2058
* `docs`: fix completions for `--stats-jsonl` and qsv pro installation text update by @rzmk in https://github.com/jqnatividad/qsv/pull/2072
* `docs`: added Note about why `luau` feature is disabled in musl binaries - https://github.com/jqnatividad/qsv/commit/ffa2bc5a3f397b406347d14d0d4fb4ead49cb470 & https://github.com/jqnatividad/qsv/commit/27d0f8e1c2e43c00b99abf98dfa01a4758cf9bad

### Removed
* Removed bincode dependency now that we're using JSONL stats cache https://github.com/jqnatividad/qsv/pull/2055 https://github.com/jqnatividad/qsv/commit/babd92bbae473ed63f44f593bc1ab0ad1bc17761

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.131.1...0.132.0

## [0.131.1] - 2024-08-09

### Changed
* deps: bump polars to latest upstream post py-1.41.1 release at the time of this release
* build(deps): bump filetime from 0.2.23 to 0.2.24 by @dependabot in https://github.com/jqnatividad/qsv/pull/2038

### Fixed
* `frequency`: change `--stats-mode` default to `none` from `auto`.   
This is because of a big performance regression when using `--stats-mode auto` on datasets with columns with ALL unique values. 
See https://github.com/jqnatividad/qsv/issues/2040 for more info.

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.131.0...0.131.1

## [0.131.0] - 2024-08-08

### Highlights
* __Refactored `frequency` to make it smarter and faster.__   
`frequency`'s core algorithm essentially compiles an in-memory hashmap to determine the frequency of each unique value for each column. It does this using multi-threaded, multi-I/O techniques to make it blazing fast.   
However, for columns with ALL unique values (e.g. ID columns), this takes a comparatively long time and consumes a lot of memory as it essentially compiles a hashmap of the entire column.  
Now, with the new `--stats-mode` option (enabled by default), `frequency` can compile the dataset in a more intelligent way by looking up a column's cardinality in the stats cache.  
If the cardinality of a column is equal to the CSV's rowcount (indicating a column with ALL unique values), it short-circuits frequency calculations for that column - dramatically reducing the time and memory requirements for the ID column as it eliminates the need to maintain a hashmap for it.  
Practically speaking, this makes `frequency` able to handle "real-world" datasets of any size.  
To ensure `frequency` is as fast as possible, be sure to `index` and compute `stats` for your datasets beforehand.
* __Setting the stage for Datapusher+ v1 and...__  
The "[itches we've been scratching](https://en.wikipedia.org/wiki/The_Cathedral_and_the_Bazaar#Lessons_for_creating_good_open_source_software)" the past few months have been informed by our work at several clients towards the release of Datapusher+ 1.0 and qsv pro 1.0 (more info below) - both targeted for release this month.  
[DP+](https://github.com/dathere/datapusher-plus) is our third-gen, high-speed data ingestion/registration tool for CKAN that uses qsv as its data wrangling/analysis engine. It will enable us to reinvent the way data is ingested into CKAN - with exponentially faster data ingestion, metadata inferencing, data validation, computed metadata fields, and more!  
We're particularly excited how qsv will allow us to compute and infer high-quality metadata for datasets (with a focus on inferring optional recommended [DCAT-US v3](https://doi-do.github.io/dcat-us/) metadata fields) in "near real-time", while dataset publishers are still entering metadata. This will be a game-changer for CKAN administrators and data publishers!
* __...qsv pro 1.0__  
[qsv pro](https://qsvpro.dathere.com) is [datHere](https://dathere.com)'s enterprise-grade data wrangling/curation workbench that’s planned for v1.0 release this month.
Building the core functionality of qsv pro's Workflow feature is one of the primary reasons for a v1.0 release.  
We feel qsv pro may be a game-changer for data wranglers and data curators who need to work with spreadsheets and large datasets to view statistical data and metadata while also performing complex data wrangling operations in a user-friendly way without having to write code.

---

### Added
* `docs`: added Shell Completion section https://github.com/jqnatividad/qsv/commit/556a2ff48660d05f8e9a865ec427e98114f49b43
* `docs:` add 🪄 emoji in legend to indicate "automagical" commands https://github.com/jqnatividad/qsv/commit/2753c90fcbd1cc1b41dae0a51d26bfe704029ee8
* Add building deb package (WIP) by @tino097 in https://github.com/jqnatividad/qsv/pull/2029
* Added GitHub workflow to test debian package (WIP) by @tino097 in https://github.com/jqnatividad/qsv/pull/2032
* `tests`: added false positive to _typos.toml configuration https://github.com/jqnatividad/qsv/commit/d576af229bf76b7d0e1f40eb37b578a6b6691ed4
* added more benchmarks
* added more tests

### Changed
* `fetch` & `fetchpost`: remove expired diskcache entries on startup https://github.com/jqnatividad/qsv/commit/9b6ab5db91416f71577b8a1fc91e2e3189a1bd4b
* `frequency`: smarter frequency compilation with new `--stats-mode` option https://github.com/jqnatividad/qsv/pull/2030
* `json`: refactored for maintainability & performance https://github.com/jqnatividad/qsv/commit/62e92162a4aa446097736ec76834cf0e06d195b8 and https://github.com/jqnatividad/qsv/commit/4e44b1878b952c455c1922a66795b8c86a1b1dba
* improved `self-update` messages https://github.com/jqnatividad/qsv/commit/5c874e09e15a274dccd8f83a322002032e65c2d0 and https://github.com/jqnatividad/qsv/commit/0aa0b13cf34103cfb75befc6480f31714d806aa2
* `contrib(completions)`: `frequency` updates & remove bashly/fish by @rzmk in https://github.com/jqnatividad/qsv/pull/2031
* Debian package update by @tino097 in https://github.com/jqnatividad/qsv/pull/2017
* `publish`: optimized enabled CPU features when building release binaries in all GitHub Actions "publishing" workflows
* `publish`: ensure latest Python patch release is used when building `qsvpy` binary variants https://github.com/jqnatividad/qsv/commit/2ab03a097645a95b0d390f546ad9735c9a7e72b2 and https://github.com/jqnatividad/qsv/commit/ec6f486ef112cf942b2263b84b97d90cba1beb12
* `tests`: also enabled CPU features in CI tests
* `docs`: wordsmith qsv "elevator pitch" https://github.com/jqnatividad/qsv/commit/cc47fe688eeeb13b4deb3f3bf48d954924eee22e
* `docs`: point to https://100.dathere.com in Whirlwind tour https://github.com/jqnatividad/qsv/commit/fc49aef826c1b1933ea1508cb687476936a147ff
* `deps`: bump polars to latest upstream post py-1.41.1 release at the time of this release
* build(deps): bump bytes from 1.6.1 to 1.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2018
* build(deps): bump bytes from 1.7.0 to 1.7.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2021
* build(deps): bump flate2 from 1.0.30 to 1.0.31 by @dependabot in https://github.com/jqnatividad/qsv/pull/2027
* build(deps): bump indexmap from 2.2.6 to 2.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2020
* build(deps): bump jaq-parse from 1.0.2 to 1.0.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/2016
* build(deps): bump redis from 0.26.0 to 0.26.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/2023
* build(deps): bump regex from 1.10.5 to 1.10.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/2025
* build(deps): bump serde_json from 1.0.121 to 1.0.122 by @dependabot in https://github.com/jqnatividad/qsv/pull/2022
* build(deps): bump sysinfo from 0.30.13 to 0.31.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2019
* build(deps): bump sysinfo from 0.31.0 to 0.31.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/2024
* build(deps): bump tempfile from 3.11.0 to 3.12.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/2033
* build(deps): bump serde from 1.0.204 to 1.0.205 by @dependabot in https://github.com/jqnatividad/qsv/pull/2036
* apply select clippy suggestions
* updated several indirect dependencies
* made various usage text improvements
* bumped MSRV to 1.80.1

### Fixed
* `sqlp` & `joinp`: fixed `.ssv.sz` output auto-compression support https://github.com/jqnatividad/qsv/commit/5397f6c7a3b083872bbb97d90db3a2fd2f8521e6 & https://github.com/jqnatividad/qsv/commit/d86ba6376d5819898187d5fa88eae19373022e5b
* `docs`: fix link by @uncenter in https://github.com/jqnatividad/qsv/pull/2026
* `tests`: correct misnamed test https://github.com/jqnatividad/qsv/commit/8ae600011ddb109e7993e54dae9b933d15eccd38
* `tests`: fix flaky `reverse` property tests https://github.com/jqnatividad/qsv/commit/d86ba6376d5819898187d5fa88eae19373022e5b

### Removed
* `docs`: "Quicksilver" is the name of the logo horse, not how you pronounce "qsv" https://github.com/jqnatividad/qsv/commit/e4551ae4b62a3a635b7c351c5f28aa2a7d374958

## New Contributors
* @uncenter made their first contribution in https://github.com/jqnatividad/qsv/pull/2026

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.130.0...0.131.0

## [0.130.0] - 2024-07-29

Following the [0.129.0 release - the largest release ever](https://github.com/jqnatividad/qsv/releases/tag/0.129.0), 0.130.0 continues to polish qsv as a data-wrangling engine, packing new features, fixes, and improvements, previewing upcoming features in qsv pro 1.0. Here are a few highlights:

### Highlights
- Added `.ssv` (semicolon separated values) automatic support. Semicolon separated values are now automatically detected and supported by qsv. Though not as common as CSV, SSV is used in some regions and industries, so qsv now supports it.
- Added cargo deb compatibility. In preparation for the release of [DataPusher+ 1.0](https://github.com/dathere/datapusher-plus/tree/master), we're now making it easier to upgrade `qsvdp` so [CKAN](https://ckan.org) administrators can install and upgrade it more easily, using `apt-get install qsvdp` or `apt-get upgrade qsvdp`.
DP+ is our next-gen, high-speed data ingestion tool for CKAN. Its not only a robust, fast, validating data pump that guarantees high quality data, it also does extended analysis to infer and derive high-quality metadata - what we call "[automagical metadata](https://dathere.com/2023/11/automagical-metadata/)".
- Upgraded to the latest Polars upstream at the [py-polars-1.3.0](https://github.com/pola-rs/polars/releases/tag/py-1.3.0) tag. [Polars tops the TPC-H Benchmark](https://pola.rs/posts/benchmarks/) and is several orders of magnitude faster than traditional dataframe libraries (cough - 🐼 pandas). qsv proudly rides the 🐻‍❄️ Polars bear to get subsecond response times even with very large datasets!
- qsv v0.130.0 shell completions files are available for download [here](https://github.com/jqnatividad/qsv/tree/master/contrib/completions/examples). With shell completions, pressing tab in a compatible shell may provide suggestions for various qsv commands, subcommands, and options that you may choose from. Supported shells include bash, zsh, powershell, fish, nushell, fig, and elvish. You may view tips on how to install completions for the bash shell [here](https://100.dathere.com/exercises-setup.html#optional-set-up-qsv-completions).

### Added
* `apply`: add base62 encode/decode operations https://github.com/jqnatividad/qsv/pull/2013
* `headers`: add `--just-count` option https://github.com/jqnatividad/qsv/pull/2004
* `json`: add `--select` option https://github.com/jqnatividad/qsv/pull/1990
* `searchset`: add `--not-one` flag by @rzmk in https://github.com/jqnatividad/qsv/pull/1994
* Added `.ssv` (semicolon separated values) automatic support https://github.com/jqnatividad/qsv/pull/1987
* Added cargo deb compatibility by @tino097 in https://github.com/jqnatividad/qsv/pull/1991
* `contrib(completions)`: add `--just-count` for `headers` by @rzmk in https://github.com/jqnatividad/qsv/pull/2006
* `contrib(completions)`: add `--select` for `json` by @rzmk in https://github.com/jqnatividad/qsv/pull/1992
* added several benchmarks
* added more tests

### Changed
* `diff`: allow selection of `--key` and `--sort-columns` by name, not just by index https://github.com/jqnatividad/qsv/pull/2010
* `fetch` & `fetchpost`: replace deprecated Redis execute command https://github.com/jqnatividad/qsv/commit/75cbe2b76426591e4658fdcb7d29287a40a7db36
* `stats`: more intelligent `--infer-len`option https://github.com/jqnatividad/qsv/commit/c6a0e641cd4c6ef87c070c8944f32a962a11c7e3
* `validate`: return delimiter detected upon successful CSV validation https://github.com/jqnatividad/qsv/pull/1977
* bump polars to latest upstream at py-polars-1.3.0 tag https://github.com/jqnatividad/qsv/pull/2009
* deps: bump csvs_convert from 0.8.12 to 0.8.13 https://github.com/jqnatividad/qsv/commit/d1d08009deb0579fd4d6fe305097e00e92da4191
* build(deps): bump cached from 0.52.0 to 0.53.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1983
* build(deps): bump cached from 0.53.0 to 0.53.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1986
* build(deps): bump postgres from 0.19.7 to 0.19.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1985
* build(deps): bump pyo3 from 0.22.1 to 0.22.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1979
* build(deps): bump redis from 0.25.4 to 0.26.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1995
* build(deps): bump serde_json from 1.0.120 to 1.0.121 by @dependabot in https://github.com/jqnatividad/qsv/pull/2011
* build(deps): bump simple-expand-tilde from 0.1.7 to 0.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1984
* build(deps): bump tokio from 1.38.0 to 1.38.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1973
* build(deps): bump tokio from 1.38.1 to 1.39.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1988
* build(deps): bump xxhash-rust from 0.8.11 to 0.8.12 by @dependabot in https://github.com/jqnatividad/qsv/pull/1997
* apply select clippy suggestions
* updated several indirect dependencies
* made various usage text improvements
* pin Rust nightly to 2024-07-26

### Fixed
* `diff`: clarify `--key` usage examples, resolves #1998 by @rzmk in https://github.com/jqnatividad/qsv/pull/2001
* `json`: refactored so it didn't need to use threads to spawn `qsv select` to order the columns. Had to do this as sometimes intermediate output was sent to stdout before the final output was ready https://github.com/jqnatividad/qsv/commit/0f25deff98139b574dfd61c6e9bf58d36ea16618
* `py`: replace row with col in usage text by @allen-chin in https://github.com/jqnatividad/qsv/pull/2008
* `reverse`: fix indexed bug https://github.com/jqnatividad/qsv/pull/2007
* `validate`: properly auto-detect tab delimiter when file extension is TSV or TAB https://github.com/jqnatividad/qsv/pull/1975
* fix panic when process_input helper fn receives unexpected input from stdin https://github.com/jqnatividad/qsv/commit/152fec486c0e7b16242f3967930e9654ff2bdf3c

### Removed
* `docs`: remove *nix only message for `foreach` by @rzmk in https://github.com/jqnatividad/qsv/pull/1972

## New Contributors
* @tino097 made their first contribution in https://github.com/jqnatividad/qsv/pull/1991
* @allen-chin made their first contribution in https://github.com/jqnatividad/qsv/pull/2008

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.129.1...0.130.0

---

To stay updated with datHere's latest news and updates (including [qsv pro](https://qsvpro.dathere.com), [datHere's CKAN DMS](https://dathere.com/ckan-dms/), and [analyze.dathere.com](https://analyze.dathere.com)), subscribe to the newsletter here: [dathere.com/newsletter](https://dathere.com/newsletter/)

## [0.129.0] - 2024-07-14

This release is the biggest one ever!

Packed with new features, improvements, and previews of upcoming qsv pro features, here are a few highlights:

## 📌 Highlights (click each dropdown for more info)

<details><summary><strong>Meet @rzmk - qsv pro's software engineer now also co-maintains qsv!</strong></summary>

@rzmk has contributed to projects in the qsv ecosystem including qsv's [`describegpt`](https://github.com/jqnatividad/qsv/tree/master/src/main/describegpt.rs), [`prompt`](https://github.com/jqnatividad/qsv/tree/master/src/main/prompt.rs), [`json`](https://github.com/jqnatividad/qsv/tree/master/src/main/json.rs), and [`clipboard`](https://github.com/jqnatividad/qsv/tree/master/src/main/clipboard.rs) commands; qsv's tab completion support; [qsv.dathere.com](https://qsv.dathere.com) including its online configurator and benchmarks page; [100.dathere.com](https://100.dathere.com) with its qsv lessons and exercises; and [qsv pro](https://qsvpro.dathere.com) the spreadsheet data wrangling desktop app (along with its promo site). @rzmk now also co-maintains qsv!

With @rzmk now also co-maintaining qsv, our data-wrangling portfolio's roadmap may get more intriguing as @rzmk's work on qsv pro, 100.dathere.com, and other initiatives can result in contributions to qsv as we've seen in this release. Perhaps some aims may be put towards AI; "[automagical](https://dathere.com/2023/11/automagical-metadata/)" metadata inferencing; DCAT 3; and expanded recipe support with the accelerated evolution of qsv pro as an enterprise-grade Data-Wrangling/Data Curation Workbench.

</details>

<details><summary><strong>Polars v0.41.3</strong> - numerous <a href="https://github.com/jqnatividad/qsv/tree/master/src/cmd/sqlp.rs"><code>sqlp</code></a> and <a href="https://github.com/jqnatividad/qsv/tree/master/src/cmd/joinp.rs"><code>joinp</code></a> improvements</summary>

* `sqlp`: expanded SQL support 
  - Natural Join support
  - DuckDB-like `COLUMNS` SQL function to select columns that match a pattern
  - ORDER BY ALL support
  - Support POSTGRESQL `^@` ("starts with"), `~~`,`~~*`,`!~~`,`!~~*` ("like", "ilike") string-matching operators
  - Support for SQL `SELECT * ILIKE` wildcard syntax
  - Support SQL temporal functions `STRFTIME` and `STRPTIME`
* `sqlp`: added `--streaming` option

</details>

<details style="margin-bottom: 0;"><summary><strong>New command <code><a href="https://github.com/jqnatividad/qsv/tree/master/src/cmd/prompt.rs">qsv prompt</a></code></strong> - Use a file dialog for qsv file input and output</summary>

Be more interactive with qsv by using a file dialog to select a file for input and output.

![qsv-prompt-0.129.0-demo](https://github.com/jqnatividad/qsv/assets/30333942/4ec1f6ef-3a82-41fb-91ab-a0ab15360d21)

Here are a few key highlights:

- Start with `qsv prompt` when piping commands to provide a file as input from an open file dialog and pipe it into another command, for example: `qsv prompt | qsv stats`.
- End with `qsv prompt -f` when piping commands to save the output to a file you choose with a save file dialog.

There are other options too, so feel free to explore more with `qsv prompt --help`.

This will allow you to create qsv pipelines that are more "user-friendly" and distribute them to non-technical users. It's not as flexible as qsv pro's full-blown GUI, but it's a start!

</details>

<details><summary><strong>New command <a href="https://github.com/jqnatividad/qsv/tree/master/src/cmd/json.rs"><code>qsv json</code></a></strong> - Convert JSON data to CSV and optionally provide a jq-like filter</summary>

The new `json` command allows you to convert non-nested JSON data to CSV. If your data is not in the expected format, try using the `--jaq` option to provide a jq-like filter. See `qsv json --help` for more information and examples.

![qsv-json-demo](https://github.com/jqnatividad/qsv/assets/30333942/e8e5e39d-dc2a-45a5-895a-5ec4ec5b6e01)

Here are a few key highlights:

- Specify the path to a JSON file to attempt conversion to CSV with `qsv json <filepath>`.
- Attempt conversion of JSON to CSV data from `stdin`, for example: `qsv slice <filepath.csv> --json | qsv json`.
- Write the output to a file with the `--output <filepath>` (or `-o` for short) option.
- Use the `--jaq <filter>` option to try converting nested or complex JSON data into the intended format before parsing to CSV.

You may learn more by running `qsv json --help`.

Along with the `jsonl` command, we now have more options to convert JSON to CSV with qsv!

</details>

<details style="margin-bottom: 0;"><summary><strong>New command <code><a href="https://github.com/jqnatividad/qsv/tree/master/src/cmd/prompt.rs">qsv clipboard</a></code></strong> - Provide input from your clipboard and save output to your clipboard</summary>

Provide your clipboard content using `qsv clipboard` and save output to your clipboard by piping into `qsv clipboard --save` (or `-s` for short).

![qsv-clipboard-demo](https://github.com/jqnatividad/qsv/assets/30333942/c3e3754a-8db0-4a28-84bd-ba88054cf9a6)

</details>

<details><summary><strong><a href="https://100.dathere.com">100.dathere.com</a></strong> - Try out lessons and exercises with qsv from your browser!</summary>

You may run qsv commands from your browser without having to install it locally at [100.dathere.com](https://100.dathere.com).

| Within the lesson (in-page) using Thebe                            | In a Jupyter Lab environment                            |
| ----------------------------------- | ----------------------------------- |
| ![qsv Thebe demo](https://github.com/jqnatividad/qsv/assets/30333942/f5315ad4-e73a-4fe1-b868-b2f950412ecc) | ![qsv Jupyter Lab demo](https://github.com/jqnatividad/qsv/assets/30333942/9acca4b1-3117-4222-8198-c751a74e6378) |

Thanks to [Jupyter Book](https://jupyterbook.org), [datHere](https://dathere.com) has released a website available at [100.dathere.com](https://100.dathere.com) where you may explore lessons and exercises with qsv by running them within the web page, in a Jupyter Lab environment, or locally after following the provided installation instructions. There are multiple exercises planned, but feel free to try out the first few available lessons/exercises by visiting [100.dathere.com](https://100.dathere.com) and star the source code's repository [here](https://github.com/dathere/100.dathere.com).

</details>

<details><summary><strong>New <a href="https://github.com/jqnatividad/qsv/tree/master/contrib/completions">multi-shell completions draft</a></strong> (bash, zsh, powershell, fish, nushell, fig, elvish)</summary>

There's a draft of more qsv shell completion support including 7 different shells! The plan is to add the rest of the commands in this implementation since we can use one codebase to generate the 7 shell completion script files. Feel free to try out the various shell completions in the `examples` folder from [`contrib/completions`](https://github.com/jqnatividad/qsv/tree/master/contrib/completions) to verify if the examples work (as of today's release date only `qsv count` and `qsv clipboard` may be available) and also contribute to adding the rest of the completions if you know a bit of Rust.

The existing <a href="https://github.com/jqnatividad/qsv/tree/master/contrib/bashly">Bash shell completions for v0.129.0</a> and <a href="https://github.com/jqnatividad/qsv/tree/master/contrib/fish">fish shell completions draft</a> are available for now as the multi-shell completions draft is being developed.

| Bash completions demo                            | Fish completions demo                            |
| ----------------------------------- | ----------------------------------- |
| ![qsv Bash completions demo](https://github.com/jqnatividad/qsv/assets/30333942/bec4b9ae-584a-49ad-8ced-c765174e8113) | ![qsv Fish completions demo](https://github.com/jqnatividad/qsv/assets/30333942/fafbf40b-9ea3-4ec4-ae22-9ae3319ce400) |

With shell completions enabled, you may identify qsv commands more easily when pressing the `tab` key on your keyboard in certain positions using the relevant Bash or fish shell from your terminal. You may follow the instructions from 100.dathere.com [here](https://100.dathere.com/exercises-setup.html#bash) to learn how to install the Bash completions and under the Usage section [here](https://github.com/jqnatividad/qsv/tree/master/contrib/fish#usage) for fish shell completions. Note that the fish shell completions are incomplete and both of the implementations may be replaced by the multi-shell completions implementation once complete.

</details>

<details><summary><strong><a href="https://qsvpro.dathere.com">qsvpro.dathere.com</a></strong> - Preview: Download spreadsheets from a compatible CKAN instance into the qsv pro Workflow</summary>

> This is a preview of a feature, meaning it is planned for an upcoming release but may change by the time it is released.

![qsv-pro-ckan-download-demo](https://github.com/jqnatividad/qsv/assets/30333942/9f4931ce-f51e-4266-9c22-e568d10ed811)

In addition to importing local spreadsheet files and uploading to a CKAN instance, this new feature allows users to select a locally registered CKAN instance where they have the `create_dataset` permission to download a spreadsheet file from their CKAN instance and load the new local spreadsheet file into the Workflow. qsv pro's Workflow would therefore have both upload and download capability to and from a compatible CKAN instance.

</details>
<details><summary><strong><a href="https://qsvpro.dathere.com">qsvpro.dathere.com</a></strong> - Preview: Attempt SQL query generation from natural language with a compatible LLM API instance</summary>

> This is a preview of a feature, meaning it is planned for an upcoming release but may change by the time it is released.
> Also note that this video is sped up as you may see by the notes that pop up (you may pause the video to read them).

https://github.com/jqnatividad/qsv/assets/30333942/e90893e6-3196-4fa6-bce0-f69a9f6347f2

Leveraging [`qsv describegpt`](https://github.com/jqnatividad/qsv/tree/master/src/cmd/describegpt.rs)'s AI integration capabilities along with multiple other qsv commands, qsv pro's Workflow's existing SQL query tab now has a generator that may ***attempt*** to generate a SQL query natural language using an LLM API compatible with OpenAI's API specification  such as running an [Ollama](https://ollama.com/) (v0.2.0 or above) server locally and  ***attempt*** to generate a SQL query by asking a question related to your spreadsheet data. Results may vary depending on your configuration and you may need to fix the generated output. For example in the demo we asked for ***who*** has the highest salary but extra information and only the highest salary was provided, though this does give a query we can modify and work with.

<details><summary>Note on Ask and <code>qsv describegpt</code></summary>

We mention ***attempt*** since LLMs can produce incorrect output, even output that *seems* correct but is not. We mention that "inaccurate information" may be produced within `qsv describegpt`'s usage text too along with AI-generated output potentially being incorrect within qsv pro, so make sure the output is fixed and verified before using it in production use cases.

</details>
</details>

<details><summary><h2>🔁 Changelog</h2></summary>

### Added

* `clipboard`: add `qsv clipboard` command for clipboard input/output by @rzmk in https://github.com/jqnatividad/qsv/pull/1953
* `describegpt`: add `--prompt` for custom prompt & update prompt file + docs by @rzmk in https://github.com/jqnatividad/qsv/pull/1862
* `describegpt`: add base_url, model, ollama, & timeout to prompt file by @rzmk in https://github.com/jqnatividad/qsv/pull/1859
* `enum`: add  `--hash` option to create a platform-independent deterministic id https://github.com/jqnatividad/qsv/pull/1902
* `enum`: add `--uuid7` option to create UUID v7 identifiers https://github.com/jqnatividad/qsv/pull/1914
* `freq`: add  `--no-trim` option  https://github.com/jqnatividad/qsv/pull/1944
* `foreach`: add sample Windows implementation by @rzmk in https://github.com/jqnatividad/qsv/pull/1847
* `joinp`: add `--right` outer join option https://github.com/jqnatividad/qsv/pull/1945
* `json`: change jsonp to json using new implementation by @rzmk in https://github.com/jqnatividad/qsv/pull/1924
* `json`: add `--jaq` option to allow jq-like filtering & test by @rzmk in https://github.com/jqnatividad/qsv/pull/1959
* `jsonp`: add `jsonp` command allowing non-nested JSON to CSV conversion with Polars by @rzmk in https://github.com/jqnatividad/qsv/pull/1880
* `prompt`: add `qsv prompt` to pick a file with a file dialog & write to stdout by @rzmk in https://github.com/jqnatividad/qsv/pull/1860
* `prompt`: add `--fd-output` (`-f`) & `--output` (`-o`) options by @rzmk in https://github.com/jqnatividad/qsv/pull/1861
* `select`: add `--sort`, `--random` & `--seed` options; also add 9999 sentinel value to indicate last column https://github.com/jqnatividad/qsv/pull/1867
* `select`: use underscore char (_) to indicate last column, replacing 9999 sentinel value https://github.com/jqnatividad/qsv/pull/1873
* `sqlp`: add `--streaming` option https://github.com/jqnatividad/qsv/commit/e8bee9a60dccc6ec5b5a43b91cb6f558915faa0e
* `stats`: add Standard Error of the Mean (SEM) & Coefficient of Variation (CV) https://github.com/jqnatividad/qsv/pull/1857
* `validate`: added custom JSONschema format "currency" (decimal with 2 decimal places). Also, added check that only ascii characters are allowed in keys in JSONschema files.
* added `--batch` zero option to all commands with batch processing. This sentinel value is used to indicate that the entire input should be processed in one batch https://github.com/jqnatividad/qsv/commit/feedbda4a3be9f8835eba0626e5fe01147831186
* added typos check to CI https://github.com/jqnatividad/qsv/commit/9fdf0662b6dc4fa6ebfed592a177d8539f264041
* `contrib(fish)`: add fish completions prototype with `qsv.fish` and docs by @rzmk in https://github.com/jqnatividad/qsv/pull/1884
* contrib(bashly): add `--hash <columns>` option to `enum` by @rzmk in https://github.com/jqnatividad/qsv/pull/1905
* contrib(bashly): add `--uuid4` & `--uuid7` for `qsv enum` by @rzmk in https://github.com/jqnatividad/qsv/pull/1915
* `contrib(bashly)`: remove `--ollama` from `qsv describegpt` by @rzmk in https://github.com/jqnatividad/qsv/pull/1951
* `contrib(bashly)`: add `--no-trim` to `frequency` & `--right` to `joinp` by @rzmk in https://github.com/jqnatividad/qsv/pull/1952
* `tests`: add tests for 100.dathere.com/lessons/1 by @rzmk in https://github.com/jqnatividad/qsv/pull/1876
* `tests`: add test_100 for 100.dathere.com & tests for lesson/exercise 0 by @rzmk in https://github.com/jqnatividad/qsv/pull/1848
* `docs`: add 👆 emoji to indicate commands with column selector support https://github.com/jqnatividad/qsv/commit/40ac8a7602315857ca529f43dd4fc45bec65c703
* Incorporate typos check in CI https://github.com/jqnatividad/qsv/pull/1930

### Changed
* `stats`: made several microoptimizations to Field Data Type inferencing https://github.com/jqnatividad/qsv/commit/35004541d25eb29d564ec60824da63d9f32344dd https://github.com/jqnatividad/qsv/commit/f829e0cfbc8a390570f85371e3d661ec33211405 
* `select`: `--sort` & `--random` options now work with the initial selection, not just the entire CSV https://github.com/jqnatividad/qsv/pull/1875
* `contrib(bashly)`: update `contrib/bashly/completions.bash` (prep for qsv v0.129.0) by @rzmk in https://github.com/jqnatividad/qsv/pull/1885
* `jsonp`: use `print!` instead of `println!` & add `House.csv` + tests by @rzmk in https://github.com/jqnatividad/qsv/pull/1897
* `docs`: add column selector emoji - 👆 https://github.com/jqnatividad/qsv/pull/1906
* upgrade to polars 0.41.0 https://github.com/jqnatividad/qsv/pull/1907
* `describegpt`: update `dotenv.template` variable with `QSV_LLM_APIKEY` by @rzmk in https://github.com/jqnatividad/qsv/pull/1908
* `describegpt`: change min Ollama version from 0.1.49 to 0.2.0 by @rzmk in https://github.com/jqnatividad/qsv/pull/1954
* `describegpt`: add `{headers}` replaced by `qsv slice ... --len 1 -n` by @rzmk in https://github.com/jqnatividad/qsv/pull/1941
* `validate`: validating against a JSONschema requires headers https://github.com/jqnatividad/qsv/pull/1931
* setting `--batch` to 0 loads all rows at once before parallel processing https://github.com/jqnatividad/qsv/pull/1928
* `deps`: add polars timezones support https://github.com/jqnatividad/qsv/pull/1898
* `tests`: update `test_100/exercise_0.rs` setup file data by @rzmk in https://github.com/jqnatividad/qsv/pull/1858
* build(deps): bump actions/setup-python from 5.1.0 to 5.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1961
* build(deps): bump actix-web from 4.6.0 to 4.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1866
* build(deps): bump actix-web from 4.7.0 to 4.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1901
* build(deps): bump atoi_simd from 0.15.6 to 0.16.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1844
* build(deps): bump cached from 0.51.3 to 0.51.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1874
* build(deps): bump cached from 0.51.4 to 0.52.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1938
* build(deps): bump csvs_convert from 0.8.10 to 0.8.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1891
* build(deps): bump csvs_convert from 0.8.11 to 0.8.12 by @dependabot in https://github.com/jqnatividad/qsv/pull/1948
* build(deps): bump curve25519-dalek from 4.1.2 to 4.1.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1893
* build(deps): bump flexi_logger from 0.28.0 to 0.28.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1853
* build(deps): bump flexi_logger from 0.28.1 to 0.28.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1868
* build(deps): bump flexi_logger from 0.28.2 to 0.28.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1870
* build(deps): bump flexi_logger from 0.28.3 to 0.28.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1881
* build(deps): bump flexi_logger from 0.28.4 to 0.28.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1904
* build(deps): bump geosuggest-core from 0.6.2 to 0.6.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1883
* build(deps): bump geosuggest-utils from 0.6.2 to 0.6.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1882
* build(deps): bump jql-runner from 7.1.9 to 7.1.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1845
* build(deps): bump jql-runner from 7.1.10 to 7.1.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1856
* build(deps): bump jql-runner from 7.1.11 to 7.1.12 by @dependabot in https://github.com/jqnatividad/qsv/pull/1903
* build(deps): bump jql-runner from 7.1.12 to 7.1.13 by @dependabot in https://github.com/jqnatividad/qsv/pull/1960
* build(deps): bump log from 0.4.21 to 0.4.22 by @dependabot in https://github.com/jqnatividad/qsv/pull/1925
* build(deps): bump mimalloc from 0.1.42 to 0.1.43 by @dependabot in https://github.com/jqnatividad/qsv/pull/1911
* build(deps): bump mlua from 0.9.8 to 0.9.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1894
* `deps`: apply latest polars upstream with unreleased fixes https://github.com/jqnatividad/qsv/commit/261ede59058a123c4cba62c0945a1fc4e1c77861
* `deps`: we now track py-polars release, instead of rust-polars https://github.com/jqnatividad/qsv/pull/1854
* `deps`: update polars engine to use py-polars-1.0.0-beta1 https://github.com/jqnatividad/qsv/pull/1896
* build(deps): bump polars from 0.41.0 to 0.41.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1909
* build(deps): bump polars from 0.41.1 to 0.41.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1916
* deps: bump polars from 0.41.2 to 0.41.3 https://github.com/jqnatividad/qsv/commit/dc0492ffe2669ddf8a7ff3f82fcd2db8daad83f9
* build(deps): bump pyo3 from 0.21.2 to 0.22.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1918
* build(deps): bump pyo3 from 0.22.0 to 0.22.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1950
* build(deps): bump regex from 1.10.4 to 1.10.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1865
* build(deps): bump redis from 0.25.3 to 0.25.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1846
* build(deps): bump reqwest from 0.12.4 to 0.12.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1889
* build(deps): bump self_update from 0.40.0 to 0.41.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1939

* build(deps): bump serde from 1.0.203 to 1.0.204 by @dependabot in https://github.com/jqnatividad/qsv/pull/1949
* build(deps): bump serde_json from 1.0.117 to 1.0.118 by @dependabot in https://github.com/jqnatividad/qsv/pull/1920
* build(deps): bump serde_json from 1.0.118 to 1.0.119 by @dependabot in https://github.com/jqnatividad/qsv/pull/1932
* build(deps): bump serde_json from 1.0.119 to 1.0.120 by @dependabot in https://github.com/jqnatividad/qsv/pull/1935
* build(deps): bump simple-expand-tilde from 0.1.6 to 0.1.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1886
* build(deps): bump strum from 0.26.2 to 0.26.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1913
* build(deps): bump strum_macros from 0.26.2 to 0.26.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1855
* build(deps): bump strum_macros from 0.26.3 to 0.26.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1863
* build(deps): bump sysinfo from 0.30.12 to 0.30.13 by @dependabot in https://github.com/jqnatividad/qsv/pull/1957
* build(deps): bump sysinfo from 0.30.12 to 0.30.13 by @dependabot in https://github.com/jqnatividad/qsv/pull/1965
* build(deps): bump titlecase from 3.2.0 to 3.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1963
* build(deps): bump tokio from 1.37.0 to 1.38.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1850
* build(deps): bump url from 2.5.0 to 2.5.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1869
* build(deps): bump url from 2.5.1 to 2.5.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1895
* build(deps): bump uuid from 1.8.0 to 1.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1912
* build(deps): bump uuid from 1.9.0 to 1.9.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1919
* build(deps): bump uuid from 1.9.1 to 1.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1964
* build(deps): bump xxhash-rust from 0.8.10 to 0.8.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1942

* apply select clippy suggestions
* updated several indirect dependencies
* made various usage text improvements
* added several benchmarks
* pin Rust nightly to 2024-06-23

### Fixed

* `frequency`: fix unique identifiers column detection https://github.com/jqnatividad/qsv/pull/1966
* `json`: add empty single JSON object logic & empty tests by @rzmk in https://github.com/jqnatividad/qsv/pull/1958
* `json`: fix typo in error message by @rzmk in https://github.com/jqnatividad/qsv/pull/1929
* `sniff`: fix doc typo by @rzmk in https://github.com/jqnatividad/qsv/pull/1947
* `validate`: validating with a JSONSchema requires headers https://github.com/jqnatividad/qsv/commit/616438213de44e4377a98ea81a676a7900bd4ae9
* Fixed several typos https://github.com/jqnatividad/qsv/commit/9fdf0662b6dc4fa6ebfed592a177d8539f264041

### Removed
* `describegpt`: remove `--ollama` since Ollama v0.1.49 has endpoints by @rzmk in https://github.com/jqnatividad/qsv/pull/1946
* `json`: remove necessity for `polars` feature & fix `--list` formatting by @rzmk in https://github.com/jqnatividad/qsv/pull/1936
* `jsonp`: remove `jsonp` command in favor of `json` by @rzmk in https://github.com/jqnatividad/qsv/pull/1924
* `deps`: fine tune polars features and remove explicit polars-ops dependency https://github.com/jqnatividad/qsv/commit/ccfd000d129799f5a106a7d4c8edab88af37367b


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.128.0...0.129.0
</details>

---

To stay updated with datHere's latest news and updates (including [qsv pro](https://qsvpro.dathere.com), [datHere's CKAN DMS](https://dathere.com/ckan-dms/), and [analyze.dathere.com](https://analyze.dathere.com)), subscribe to the newsletter here: [dathere.com/newsletter](https://dathere.com/newsletter/)


## [0.128.0] - 2024-05-25

# ❤️ csv,conf,v8 Edition - [_¡Ándale! ¡Ándale! ¡Arriba! ¡Arriba!_](https://www.youtube.com/watch?v=5bmiDLH5htU) 🎉 #

Yii-hah! We're Mexico bound as we head to [csv,conf,v8](https://csvconf.com) to present and share qsv with fellow data-makers and wranglers from all over!

And we've packed a lot into this release for the occcasion:
* `search` got a lot of love as it now powers `qsv-pro`'s new `search` feature to get near-instant search results even on large datasets.
* `stats` - the ❤️ of qsv, now has cache fine-tuning options with the `--cache-threshold` option. It now also computes `max_precision` for floats and `is_ascii` for strings. It also has a new `--round` 9999 sentinel value to suppress rounding of statistics.
* `schema` & `tojsonl` are now faster thanks to `stats --cache-threshold` autoindex creation/deletion logic.
* We [upgraded Polars to 0.40.0](https://github.com/pola-rs/polars/releases/tag/rs-0.40.0) for even more speed and stability for the `count`, `joinp` & `sqlp` commands.
* `count` now has an additional blazing fast counting mode using Polars' `read_csv()` table function.
* `frequency` gets some micro-optimizations for even faster frequency analysis.
* `luau` is bundled with luau [0.625](https://github.com/luau-lang/luau/releases/tag/0.625) from [0.622](https://github.com/luau-lang/luau/releases/tag/0.622). We also upgraded the bundled LuaDate library [from 2.2.0 to 2.2.1](https://github.com/Tieske/date?tab=readme-ov-file#changes).

Overall, qsv manages to keep its performance edge despite the addition of new capabilities and features, and we'll give a whirlwind tour in [our talk at csv,conf,v8](https://csvconf.com/schedule/). 

We'll also preview what we've been calling the __People's APPI__ - our _"Answering People/Policymaker Interface"_ in [qsv pro](https://qsvpro.dathere.com). This is a new way to interact with qsv that's more conversational and less command-line-y using a natural language interface. It's a way to make qsv more accessible to more people, especially those who are not comfortable with the command line.

We're excited to share these with the csv,conf,v8 community and the wider world! Nos vemos en Puebla!

[_¡Ándele! ¡Ándele! ¡Epa! ¡Epa! ¡Epa!_](https://www.youtube.com/watch?v=cc-3wVQuD7k)

---

### Added
* `count`: additional Polars-powered counting mode using `read_csv()` SQL table function https://github.com/jqnatividad/qsv/commit/05c580912365356e9c5383654f351e0cc6ebaab6
* `input`: add `--quote-style` option https://github.com/jqnatividad/qsv/commit/df3c8f14a4eaa2fba7237dfe30df2fef8c98eccd
* `joinp`: add `--coalesce` option https://github.com/jqnatividad/qsv/commit/8d142e51d683ab425fc53b2dddfdeeff6a814ffa
* `search`: add `--preview-match` option https://github.com/jqnatividad/qsv/pull/1785
* `search`: add `--json` output option https://github.com/jqnatividad/qsv/pull/1790
* `search`: add "match-only" `--flag` option mode https://github.com/jqnatividad/qsv/pull/1799
* `search`: add `--not-one` flag for not using exit code 1 when no match by @rzmk in https://github.com/jqnatividad/qsv/pull/1810
* `sqlp`: add `--decimal-comma` option https://github.com/jqnatividad/qsv/pull/1832
* `stats`: add `--cache-threshold` option https://github.com/jqnatividad/qsv/pull/1795
* `stats`: add `--cache-threshold` autoindex creation/deletion  logic https://github.com/jqnatividad/qsv/pull/1809
* `stats`: add additional mode to `--cache-threshold` https://github.com/jqnatividad/qsv/commit/63fdc55828ec55bf7545c37bd56a4d537aa0cf71
* `stats`: now computes max_precision for floats https://github.com/jqnatividad/qsv/pull/1815
* `stats`: add `--round` 9999 sentinel value support to suppress rounding https://github.com/jqnatividad/qsv/pull/1818
* `stats`: add `is_ascii` column https://github.com/jqnatividad/qsv/pull/1824
* added new benchmarks for `search` command https://github.com/jqnatividad/qsv/commit/58d73c3beb41071d6cd8532768f0991f0554b717

### Changed
* `count`: document three count modes https://github.com/jqnatividad/qsv/commit/3d5a333ca8aef3aeaf74ff9e153b5118eb6a605b
* `describegpt`: update `--max-tokens` type for LLMs with larger context sizes by @rzmk https://github.com/jqnatividad/qsv/pull/1841
* `excel`: use simpler `range::headers()` to get headers https://github.com/jqnatividad/qsv/commit/069acbf5a6e86132214521324720608f4258c20f
* `frequency`: ensure `--other-sorted` works with `--other-text` https://github.com/jqnatividad/qsv/commit/7430ad76bda869be7729ea5000ad4d85a875433b
* `frequency`: microoptimize hot loop https://github.com/jqnatividad/qsv/commit/d9c01e17fa6c4f853a501fe75c6a6b8a30c269d2, https://github.com/jqnatividad/qsv/commit/7c9f925184100f89f6f3a77ae4f7b93448103f38 and 
* `luau`: improve usage text https://github.com/jqnatividad/qsv/commit/cb6b4d9b7bfb60a10385057ca093453e3549e424
* `luau`: we now bundle luau 0.625 from 0.622 https://github.com/jqnatividad/qsv/commit/40609751950a852f998fba41edb35aab31c74c20
* `luau`: update vendored LuaDate library from 2.2.0 to 2.2.1 https://github.com/jqnatividad/qsv/pull/1840
* `schema`: adjust to reflect `stats --cache-threshold` option https://github.com/jqnatividad/qsv/commit/92fed8696fd885d3721f07eeedcf67732febed4c
* `slice`: move json output helpers to util https://github.com/jqnatividad/qsv/commit/1f44b488784fd0c1ef22786ab7aeacbf2f8cf976
* `tojsonl`: refactor boolcheck helper https://github.com/jqnatividad/qsv/commit/74d5f5a8c934254e11ee611973cc10524a288a9e
* `docs`: cross-reference `split` & `partition` commands https://github.com/jqnatividad/qsv/pull/1828
* contrib(bashly): update completions.bash for qsv v0.127.0 by @rzmk in https://github.com/jqnatividad/qsv/pull/1776
* contrib(bashly): update completions.bash for qsv v0.128.0 by @rzmk in https://github.com/jqnatividad/qsv/pull/1838
* `deps`: upgrade to polars 0.40.0 https://github.com/jqnatividad/qsv/pull/1831
* build(deps): bump actix-web from 4.5.1 to 4.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1825
* build(deps): bump anyhow from 1.0.82 to 1.0.83 by @dependabot in https://github.com/jqnatividad/qsv/pull/1798
* build(deps): bump anyhow from 1.0.83 to 1.0.85 by @dependabot in https://github.com/jqnatividad/qsv/pull/1823
* build(deps): bump anyhow from 1.0.85 to 1.0.86 by @dependabot in https://github.com/jqnatividad/qsv/pull/1826
* build(deps): bump cached from 0.50.0 to 0.51.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1789
* build(deps): bump cached from 0.51.0 to 0.51.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1793
* build(deps): bump cached from 0.51.1 to 0.51.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1802
* build(deps): bump cached from 0.51.2 to 0.51.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1805
* build(deps): bump crossbeam-channel from 0.5.12 to 0.5.13 by @dependabot in https://github.com/jqnatividad/qsv/pull/1827
* build(deps): bump csvs_convert from 0.8.9 to 0.8.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1808
* build(deps): bump data-encoding from 2.5.0 to 2.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1780
* build(deps): bump file-format from 0.24.0 to 0.25.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1807
* build(deps): bump flate2 from 1.0.28 to 1.0.29 by @dependabot in https://github.com/jqnatividad/qsv/pull/1778
* build(deps): bump flate2 from 1.0.29 to 1.0.30 by @dependabot in https://github.com/jqnatividad/qsv/pull/1784
* build(deps): bump hashbrown from 0.14.3 to 0.14.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1781
* build(deps): bump itertools from 0.12.1 to 0.13.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1822
* deps: bump forked jsonschema from 0.17.1 to 0.18.0 https://github.com/jqnatividad/qsv/commit/f02620fd170804b1995b070e8133522b98a8c443
* build(deps): bump mimalloc from 0.1.41 to 0.1.42 by @dependabot in https://github.com/jqnatividad/qsv/pull/1829
* build(deps): bump mlua from 0.9.7 to 0.9.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1821
* build(deps): bump qsv-stats from 0.16.0 to 0.17.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1813
* build(deps): bump qsv-stats from 0.17.1 to 0.17.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1814
* build(deps): bump qsv-stats from 0.17.2 to 0.18.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1816
* build(deps): bump ryu from 1.0.17 to 1.0.18 by @dependabot in https://github.com/jqnatividad/qsv/pull/1801
* build(deps): bump semver from 1.0.22 to 1.0.23 by @dependabot in https://github.com/jqnatividad/qsv/pull/1800
* build(deps): bump serde from 1.0.198 to 1.0.199 by @dependabot in https://github.com/jqnatividad/qsv/pull/1777
* build(deps): bump serde from 1.0.199 to 1.0.200 by @dependabot in https://github.com/jqnatividad/qsv/pull/1787
* build(deps): bump serde from 1.0.200 to 1.0.201 by @dependabot in https://github.com/jqnatividad/qsv/pull/1804
* build(deps): bump serde from 1.0.201 to 1.0.202 by @dependabot in https://github.com/jqnatividad/qsv/pull/1817
* build(deps): bump serde_json from 1.0.116 to 1.0.117 by @dependabot in https://github.com/jqnatividad/qsv/pull/1806
* build(deps): bump serial_test from 3.1.0 to 3.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1779
* build(deps): bump simple-expand-tilde from 0.1.5 to 0.1.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1811
* build(deps): bump sysinfo from 0.30.11 to 0.30.12 by @dependabot in https://github.com/jqnatividad/qsv/pull/1797
* build(deps): bump titlecase from 3.0.0 to 3.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1791
* build(deps): bump jql-runner from 7.1.8 to 7.1.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1839
* apply select clippy suggestions
* updated several indirect dependencies
* pin Rust nightly to 2024-05-14
* bump MSRV to 1.78

### Fixed
* `luau`: correct example when using `--colindex` https://github.com/jqnatividad/qsv/commit/cbbed21718324346031a3201407f274abfec5ee6
* `search`: fix `--json` output https://github.com/jqnatividad/qsv/pull/1792
* pass through docopt messages without a prefix https://github.com/jqnatividad/qsv/pull/1835
* apply Polars SQL `count(*) group by` fix https://github.com/jqnatividad/qsv/pull/1837

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.127.0...0.128.0

## [0.127.0] - 2024-04-25

# 📊 Enhanced Frequency Analysis 📊 #
This a quick release adding several `frequency` enhancements for more detailed frequency analysis. The `frequency` command now includes a percentage column, calculates `other` values, and supports limiting unique counts and negative limits.
These options provides additional context for qsv-pro and `describegpt` so their metadata inferences are more accurate and comprehensive.

Previously, for a 775-row CSV file containing one column named `state` with entries for all 50 states, `frequency` only showed:

```
field,value,count
state,NY,100
state,NJ,70
state,CA,60
state,MA,55
state,FL,45
state,TX,43
state,NM,40
state,AZ,39
state,NV,38
state,MI,35
```

Now, there's a new `percentage` column and `other` values calculation, both of which have configurable options:

```
field, value, count, percentage
state, NY, 100, 12.90
state, NJ, 70, 9.03
state, CA, 60, 7.74
state, MA, 55, 7.10
state, FL, 45, 5.81
state, TX, 43, 5.55
state, NM, 40, 5.16
state, AZ, 39, 5.03
state, NV, 38, 4.90
state, MI, 35, 4.52
state, Other (40), 250, 32.26
```

This release is also out of cycle to address a big performance regression in the `excel` command caused by unnecessary formula info retrieval for the `--error-format` option introduced in 0.126.0. This has been fixed, and the `excel` command is now back to its speedy self.

---

### Added
* `frequency`: added percentage column;  `other` values calculation, implementing https://github.com/jqnatividad/qsv/issues/1774 https://github.com/jqnatividad/qsv/pull/1775
* `benchmarks`: added new `frequency` and `excel` benchmarks https://github.com/jqnatividad/qsv/commit/b83ad3aae1cdf9a1750201cbf9b3ccd4ac3a4192

### Changed
* contrib(bashly): update completions.bash for qsv v0.126.0 by @rzmk in https://github.com/jqnatividad/qsv/pull/1771
* build(deps): bump mimalloc from 0.1.39 to 0.1.41 by @dependabot in https://github.com/jqnatividad/qsv/pull/1772
* build(deps): bump qsv-stats from 0.14.0 to 0.15.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1773
* updated several indirect dependencies
* applied select clippy recommendations

### Fixed
* `excel`: fixed performance regression because qsv was unnecessarily getting formula info (an expensive operation) for `--error-format` option even when not required https://github.com/jqnatividad/qsv/commit/772af3420c44c864e06cd2cb61606900bff17947
* renamed 0.126.0 sqlp_vs_duckdb benchmark results so they're not to each other for easy direct comparison https://github.com/jqnatividad/qsv/commit/7bcd59e301965b9e8737a9230d1236e8d34ab4bf

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.126.0...0.127.0

## [0.126.0] - 2024-04-22

# 🤖 Expanded Metadata Inferencing 🤖 #

`describegpt` headlines this release, with its new ability to support other local Large Language Models (LLMs) such as [Ollama](https://ollama.com) and [Jan](https://jan.ai). This broadens the tool's utility in diverse AI environments and unlocks expanded metadata inferencing capabilities in qsv-pro.

Several commands got additional options: `cat` with `--no-headers` support in the `rowskey` subcommand; `excel` with new options like `--error-format` and short `--metadata` mode; and `foreach` with a `--dry-run` option. `frequency` also got new options, including `--unq-limit` for limiting unique counts, support for negative limits, and a `--lmt-threshold` option for compiling comprehensive frequencies below a threshold. `slice` now supports negative indices and new JSON output options, providing more flexibility in data slicing.

This is all rounded out with `sqlp` improvements, including support for single-line comments in SQL scripts and a special SKIP_INPUT value for more efficient data loading - all while increasing performance thanks to the Polars engine being upgraded to 0.39.2. 

---

### New Features
* `cat`: Added `--no-headers` support to the `rowskey` subcommand, allowing for more versatile manipulation of CSV data.
* `describegpt`: Added compatibility for other local Large Language Models (LLMs) such as [Ollama](https://ollama.com) and [Jan](https://jan.ai), broadening the tool's utility in diverse AI environments.
* `excel`: Introduced new options in the excel command: `--error-format` for better error handling and `--metadata` for short JSON mode, enhancing data parsing and metadata management capabilities.
* `foreach`: added a `--dry-run` option, allowing users to preview the results of scripts without executing them.
* `frequency`: New options added such as `--unq-limit` for limiting unique counts; support for negative limits to only show frequencies >= abs(negative limit); and a `--lmt-threshold` option to allow the compilation of comprehensive frequencies below the threshold - all providing more detailed control over frequency analysis.
* `slice`: Support for negative indices to slice from the end and new JSON output options, offering more flexibility in data slicing.
* `sqlp`: sqlp now supports single-line comments and includes a special SKIP_INPUT value for more efficient data loading. The Polars engine has also been upgraded to [0.39.2](https://github.com/pola-rs/polars/releases/tag/rs-0.39.2), providing enhanced performance and stability.

### Changes and Optimizations
* __Performance Enhancements__: Microoptimizations in datefmt and validate functions, and increased default length for --infer-len in sqlp for improved performance.
* __Dependency Updates__: Numerous updates including bumping Luau, jql-runner, pyo3, and other dependencies to enhance stability and security.
* __Benchmarks Added__: New performance benchmarks for sqlp vs duckdb included, showcasing the efficiency gains through Polars integration.

### Security and Robustness
* __Security Fixes__: Updated rustls to fix a specific CVE, and other minor fixes to enhance the security and robustness of network and data processing features.
* __Bug Fixes__: Various bug fixes including improvements in error formatting in excel and robustness in fetch and fetchpost commands.

### Deprecated Features
* `fetch` & `fetchpost`: Removal of the jsonxf crate from these commands to streamline JSON processing
* `reverse`: Eliminate kludgy buffer expansions.

This release not only enhances existing functionalities with added options and support for additional models and formats but also emphasizes performance improvements and robustness with critical updates and optimizations.

---

### Added
* `cat`: add `--no-headers` support to rowskey subcommand https://github.com/jqnatividad/qsv/pull/1762
* `describegpt`: add compatibility for other (local) LLMs (Ollama, Jan, etc.) by @rzmk in https://github.com/jqnatividad/qsv/pull/1761
* `excel`: add `--error-format` option https://github.com/jqnatividad/qsv/pull/1721
* `excel`: add `--metadata` short JSON mode https://github.com/jqnatividad/qsv/pull/1738
* `foreach`: add `--dry-run` option https://github.com/jqnatividad/qsv/pull/1740
* `frequency`: add `--unq-limit` option https://github.com/jqnatividad/qsv/pull/1763
* `frequency`: add support for negative `--limit`s https://github.com/jqnatividad/qsv/pull/1765
* `frequency`: add `--lmt-threshold` option https://github.com/jqnatividad/qsv/pull/1766
* `slice`: add support for negative `--index` option values https://github.com/jqnatividad/qsv/pull/1726
* `slice`: implement `--json` output option https://github.com/jqnatividad/qsv/pull/1729
* `sqlp`: added support for single-line comments in SQL scripts https://github.com/jqnatividad/qsv/commit/bb52bcee61d8ea980a2ab093315ead0c153517a5
* `sqlp`: added SKIP_INPUT special value to short-circuit input processing if the user wants to
load input files directly using table functions (e.g. read_csv(), read_parquet(), etc.) https://github.com/jqnatividad/qsv/commit/fe850adb47f1d7aa7f6c3981e350646e7b0c7476
* `validate`: add `--valid-output` option https://github.com/jqnatividad/qsv/pull/1730
* contrib: add sample Bashly completions implementation by @rzmk in https://github.com/jqnatividad/qsv/pull/1731
* `benchmarks`: added `sqlp` vs `duckdb` benchmarks. Right now, `sqlp` is faster than `duckdb` in most cases (thanks to Polars - see the [latest TPC-H benchmarks](https://pola.rs/posts/benchmarks/)), but we want to make sure that we keep it that way.

### Changed
* `datefmt`: microoptimize formatting https://github.com/jqnatividad/qsv/commit/0ee27e768fdc08b7381094842d22b45940fd0a26
* `joinp`: adapt to breaking change in Polars 0.39 for lazyframe sort https://github.com/jqnatividad/qsv/commit/c625ca9f5aef59c736a837aaa4eeda7688403c37
* `sqlp`: change `--infer-len` option default from 250 to 1000 for increased performance https://github.com/jqnatividad/qsv/commit/da1d215d803f8bfe400a7202feeecb8ae14239e9
* `validate`: microoptimize `to_json_instance()` https://github.com/jqnatividad/qsv/commit/c2e4a1c696300eea04cccacca33f6872622ec086
* bump Luau from 0.616 to 0.622 https://github.com/jqnatividad/qsv/commit/9216ec3a53767379662657f69c0076f4a52caaff
* build(deps): bump jql-runner from 7.1.6 to 7.1.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1711
* build(deps): bump pyo3 from 0.21.0 to 0.21.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1712
* build(deps): bump pyo3 from 0.21.1 to 0.21.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1750
* build(deps): bump strsim from 0.11.0 to 0.11.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1715
* build(deps): bump sysinfo from 0.30.7 to 0.30.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1716
* build(deps): bump sysinfo from 0.30.8 to 0.30.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1732
* build(deps): bump sysinfo from 0.30.9 to 0.30.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1735
* build(deps): bump sysinfo from 0.30.10 to 0.30.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1755
* build(deps): bump redis from 0.25.2 to 0.25.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1720
* build(deps): bump mlua from 0.9.6 to 0.9.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1724
* build(deps): bump reqwest from 0.12.2 to 0.12.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1725
* build(deps): bump reqwest from 0.12.3 to 0.12.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1759
* build(deps): bump anyhow from 1.0.81 to 1.0.82 by @dependabot in https://github.com/jqnatividad/qsv/pull/1733
* build(deps): bump robinraju/release-downloader from 1.9 to 1.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1734
* build(deps): bump chrono from 0.4.37 to 0.4.38 by @dependabot in https://github.com/jqnatividad/qsv/pull/1744
* bump polars from 0.38 to 0.39 https://github.com/jqnatividad/qsv/pull/1745
* build(deps): bump polars from 0.39.0 to 0.39.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1746
* build(deps): bump polars from 0.39.1 to 0.39.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1752
* build(deps): bump qsv-dateparser from 0.12.0 to 0.12.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1747
* build(deps): bump serde_json from 1.0.115 to 1.0.116 by @dependabot in https://github.com/jqnatividad/qsv/pull/1749
* build(deps): bump serde from 1.0.197 to 1.0.198 by @dependabot in https://github.com/jqnatividad/qsv/pull/1751
* build(deps): bump rustls from 0.22.3 to 0.22.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1758
* build(deps): bump simple-expand-tilde from 0.1.4 to 0.1.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1767
* build(deps): bump serial_test from 3.0.0 to 3.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1768
* build(deps): bump actions/setup-python from 5.0.0 to 5.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1769
* applied select clippy recommendations
* updated several indirect dependencies
* added several benchmarks for new/changed commands
* pin Rust nightly to 2024-04-15 - the same nightly that Polars 0.39 is pinned to
* bumped MSRV to 1.77.2

### Fixed
* Make init_logger more robust https://github.com/jqnatividad/qsv/pull/1717
* `count`: empty CSVs count as zero also for polars. Fixes #1741 https://github.com/jqnatividad/qsv/pull/1742
* `excel`: fix $1682 by adding `--error-format` option https://github.com/jqnatividad/qsv/issues/1689
* `fetch` & `fetchpost`: more robust JSON response validation https://github.com/jqnatividad/qsv/commit/ebc7287cd929cc23629ee53c7d82e0b8984bc2b0
* `slice`: use `write!` macro to get rid of GH Advanced Security lint https://github.com/jqnatividad/qsv/commit/c739097e20d526cb6f49ca69d76fed8b28adc029
* `sqlp`: fixed docopt defaults that were not being parsed correctly https://github.com/jqnatividad/qsv/commit/fe850adb47f1d7aa7f6c3981e350646e7b0c7476
* `deps`: bump h2 from 0.4.3 to 0.4.4 to fix HTTP2 Continuation Flood vulnerability https://github.com/jqnatividad/qsv/commit/6af0da27f4e4a0bb6d5563701c07c89ad00f76b8
* `deps`: bump rustls from 0.22.3 to 0.22.4 to fix https://nvd.nist.gov/vuln/detail/CVE-2024-32650 https://github.com/jqnatividad/qsv/pull/1758 

### Removed
* `fetch` & `fetch post`: remove jsonxf crate; use serde_json to prettify JSON strings https://github.com/jqnatividad/qsv/pull/1727
* `reverse`: remove kludgy expansion of read/write buffers https://github.com/jqnatividad/qsv/commit/46095cdf57f65c5380251c5d59317053ae1f80c3

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.125.0...0.126.0

## [0.125.0] - 2024-04-01

In this release, we focused on the need for even more speed. 

This was done primarily by tweaking several supporting qsv crates. `qsv-docopt` now parses command-line arguments slightly faster. `qsv-stats`, the crate behind commands like `stats`, `schema`, `tojsonl`, and `frequency`, has been further optimized for speed. `qsv-dateparser` has been updated to support new timezone handling options in `datefmt`. `qsv-sniffer` also got a speed boost.

The `count` command has been refactored to utilize Polars' SQLContext, which leverages LazyFrames evaluation to automagically count even very large files in just a few seconds. Previously, `count` was already using Polars, but it mistakenly fell back to a slower counting mode. Now, it consistently delivers fast performance, even without an index.

The `datefmt` command also got a tad faster while also being enhanced with new timezone and timestamp options.

Lastly, we are excited to announce that qsv will be attending the [CSV,Conf,V8](https://csvconf.com) conference in Puebla, Mexico on May 28-29. I'll be presenting a talk titled "qsv: A Blazing Fast CSV Data-Wrangling Toolkit". [Hope to see you there!](https://www.eventbrite.com/e/csvconfv8-tickets-808081201627?aff=oddtdtcreator).

---

## Added
* `excel`: added short mode to `--metadata` option https://github.com/jqnatividad/qsv/pull/1699
* `datefmt`: added `ts-resolution` option to specify resolution to use when parsing unix timestamps https://github.com/jqnatividad/qsv/pull/1704
* `datefmt`: added timezone handling options  https://github.com/jqnatividad/qsv/pull/1706 https://github.com/jqnatividad/qsv/pull/1707 https://github.com/jqnatividad/qsv/pull/1642

## Changed
* `count`: refactored to use Polars SQLContext https://github.com/jqnatividad/qsv/commit/43a236f6a45c890d2bb6b4c43eb469bd627f82e1
* `stats`: refactored stats_path helper function https://github.com/jqnatividad/qsv/commit/174c30e3b87470613ff34a98617d44e477a4296a
* `apply`, `applydp`, `datefmt`, `excel`, `geocode`, `py`, `validate`: use std::mem::take to avoid clone https://github.com/jqnatividad/qsv/commit/1fd187f23262b51e0f431664895d49fd930d011a https://github.com/jqnatividad/qsv/commit/8402d3a8063ef161fc9ec68dd7f0f0601802d21d https://github.com/jqnatividad/qsv/commit/849615775505a25888a50b255ba0d544e878aeaf
* `excel`: optimized workbook opening operation https://github.com/jqnatividad/qsv/commit/67f662eba501e543ec44e5daf5eb175f8a8ae7b1
* build(deps): bump flexi_logger from 0.27.4 to 0.28.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1673
* build(deps): bump polars from 0.38.2 to 0.38.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1674
* build(deps): bump uuid from 1.7.0 to 1.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1675
* build(deps): bump hashbrown from 0.14.3 to 0.14.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1680
* build(deps): bump reqwest from 0.11.26 to 0.11.27 by @dependabot in https://github.com/jqnatividad/qsv/pull/1679
* build(deps): bump bytes from 1.5.0 to 1.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1685
* build(deps): bump regex from 1.10.3 to 1.10.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1686
* build(deps): bump indexmap from 2.2.5 to 2.2.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1687
* build(deps): bump rayon from 1.9.0 to 1.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1688
* build(deps): bump qsv_docopt from 1.6.0 to 1.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1691
* build(deps): bump reqwest from 0.12.1 to 0.12.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1693
* build(deps): bump serde_json from 1.0.114 to 1.0.115 by @dependabot in https://github.com/jqnatividad/qsv/pull/1694
* build(deps): bump itoa from 1.0.10 to 1.0.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1695
* build(deps): bump actions/setup-python from 5.0.0 to 5.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1700
* build(deps): bump rust_decimal from 1.34.3 to 1.35.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1701
* build(deps): bump chrono from 0.4.35 to 0.4.37 by @dependabot in https://github.com/jqnatividad/qsv/pull/1702
* build(deps): bump tokio from 1.36.0 to 1.37.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1703
* build(deps): bump qsv-sniffer from 0.10.2 to 0.10.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1708
* build(deps): bump titlecase from 2.2.1 to 3.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1709
* build(deps): bump qsv-stats from 0.13.0 to 0.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1710
* applied select clippy recommendations
* updated several indirect dependencies
* added several benchmarks for new/changed commands
* bumped MSRV to 1.77.1
* use `#[cfg(debug_assertions)]` conditional compilation to avoid compiling debug code in release mode
* use patched forks of `jsonschema`, `cached`, `self_update` and `localzone` crates to avoid old dependencies
which was causing dependency bloat

## Fixed
* `count`: fixed polars_count_input helper, as it was always falling back to "slow" counting mode https://github.com/jqnatividad/qsv/commit/3484c89080d41d2e39457c918a893189aee64753

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.124.1...0.125.0

## [0.124.1] - 2024-03-15

# [Datapusher+](https://github.com/dathere/datapusher-plushttps://github.com/dathere/datapusher-plus) "_[Speed of Insight](https://dathere.com/2024/03/the-speed-of-insight/)_" Release! 🚀🚀🚀

This release is all about speed, speed, speed! We've made qsv even faster by leveraging Polars' multithreaded, mem-mapped CSV reader to get near-instant row counts of large CSV files, and near instant SQL queries and aggregations with Datapusher+ - automagically inferring metadata and giving you quick insights into your data in seconds!

We're demoing our qsv-powered Datapusher+ at the [March 2024 installment of CKAN Monthly Live](https://ckan.org/events/ckan-datapusher-plus-automagical-metadata) on March 20, 2024, 13:00-14:00 UTC. [Join us](https://ckan.us4.list-manage.com/subscribe?u=91e21b1d5004f15a8fb3d3276&id=0b261bc4ca)!

Beyond pushing data reliably at speed into your CKAN Datastore ([it pushes real good! 😉](https://github.com/dathere/datapusher-plus/discussions/23)), DP+ does some extended analysis, processing and enrichment of the data so it can be readily Used.

Both `fetch` and `fetchpost` commands now have a `--disk-cache` option and are fully synched - forming the foundation for high-speed data enrichment from Web Services - including datHere's forthcoming, fully-integrated Data Enrichment Service.

## 🏇🏽 Hi-ho Quicksilver, away! 🏇🏽

---

## Added
* `count`: automatically use Polars multithreaded, mem-mapped CSV reader when `polars` feature is enabled to get near-instant row counts of large CSV files even without an index  https://github.com/jqnatividad/qsv/pull/1656
* `qsvdp`: added polars support to Datapusher+-optimized binary variant, so we can do near instant SQL queries and aggregations during DP+ processing https://github.com/jqnatividad/qsv/pull/1664
* `fetchpost`: added `--disk-cache` options and synced usage options with `fetch` https://github.com/jqnatividad/qsv/pull/1671
* extended `.infile-list` to skip empty and commented lines, and to validate file paths
https://github.com/jqnatividad/qsv/commit/20a45c80fa32ef8a8060bb32cc94b7934da23229 and 
https://github.com/jqnatividad/qsv/commit/26509303719ce29e900cb73b5000671a78db6b4a

## Changed
* `sqlp`: automatically disable `read_csv()` fast path optimization when a custom delimiter is specified https://github.com/jqnatividad/qsv/pull/1648
* refactored util::count_rows() helper to also use polars if available https://github.com/jqnatividad/qsv/commit/1e09e17e440d3cdc11237d9d9e45cefb82da5a42 and https://github.com/jqnatividad/qsv/commit/8d321fe8ad4c288b72edc7e8d082fcd6ec304a32
* publish: updated Windows MSI publish GH Action workflow to use Wix 3.14 from 3.11 https://github.com/jqnatividad/qsv/commit/75894ef4e894f521056a93b4f0a14d7469bac022
* deps: bump polars from 0.38.1 to 0.38.2 https://github.com/jqnatividad/qsv/commit/5faf90ed830541a724768e808c7f07f0a418e2ab
* deps: update Luau from 0.614 to 0.616 https://github.com/jqnatividad/qsv/commit/eb197fe81738b4ed15352f5f89d5d5d1b0fad604 and https://github.com/jqnatividad/qsv/commit/52331da939a3cd278c6a1f474179bef2207364a8
* build(deps): bump sysinfo from 0.30.6 to 0.30.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1650
* build(deps): bump chrono from 0.4.34 to 0.4.35 by @dependabot in https://github.com/jqnatividad/qsv/pull/1651
* build(deps): bump strum from 0.26.1 to 0.26.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1658
* build(deps): bump qsv-stats from 0.12.0 to 0.13.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1663
* build(deps): bump anyhow from 1.0.80 to 1.0.81 by @dependabot in https://github.com/jqnatividad/qsv/pull/1662
* build(deps): bump reqwest from 0.11.25 to 0.11.26 by @dependabot in https://github.com/jqnatividad/qsv/pull/1667
* applied select clippy recommendations
* updated several indirect dependencies
* added several benchmarks for new/changed commands

## Fixed
* `dedup`: fixed #1665 dedup not handling numeric values properly by adding a --numeric option  https://github.com/jqnatividad/qsv/pull/1666
* `joinp`: reenable join validation tests now that Polars 0.38.2 join validation is working again https://github.com/jqnatividad/qsv/commit/5faf90ed830541a724768e808c7f07f0a418e2ab and https://github.com/jqnatividad/qsv/commit/fcfc75b855c615effb50f23c09a1d66ce70505e8
* `count`: broken in unreleased 0.124.0. Polars-powered count require a "clean" CSV file as it infers the schema based on the first 1000 rows of a CSV. This will sometimes result in an invalid "error" (e.g. it infers a column is a number column, when its not). 0.124.1 fixes this by adding a fallback to the "regular" CSV reader if a Polars error occurs https://github.com/jqnatividad/qsv/commit/a2c086900d1c1f1ba8ed2b2d1eaf8e547e3ef740

## Removed
* `gender_guesser` 0.2.0 has been released. Remove patch.crates-io entry
https://github.com/jqnatividad/qsv/commit/97873a5c496bfd559d7a7804db4d28b94915d536

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.123.0...0.124.1

## [0.123.0] - 2024-03-05

# [OPEN DATA DAY 2024](https://opendataday.org) Release! 🎉🎉🎉

In celebration of [Open Data Day](https://en.wikipedia.org/wiki/International_Open_Data_Day), we're releasing qsv 0.123.0 - the biggest release ever with [330+ commits](https://github.com/jqnatividad/qsv/compare/0.122.0...0.123.0)! qsv 0.123.0 continues to focus on performance, stability and reliability as we continue setting the stage for qsv's big brother - qsv pro.

We've been baking qsv pro for a while now, and it's almost ready for release. qsv pro is a cross-platform Desktop Data Wrangling tool marrying an Excel-like UI with the power of qsv, backed by cloud-based data cleaning, enrichment and enhancement service that's easy to use for casual Excel users and Data Publishers, yet powerful enough for data scientists and data engineers.

Stay tuned!

## Highlights:

* `sqlp` now has automatic `read_csv()` fast path optimization, often making optimized queries run [dramatically faster](https://github.com/jqnatividad/qsv/discussions/1620) - e.g what took 6.09 seconds for a non-trivial SQL aggregation on an [18 column, 657mb CSV with 7.43 million rows](https://github.com/DataTalksClub/nyc-tlc-data/releases/download/yellow/yellow_tripdata_2019-04.csv.gz)  now takes just 0.14 seconds with the optimization - 🚀 **43.5x FASTER** 🚀 ! [^1]
[^1]: measurements taken on an Apple Mac Mini 2023 model with an M2 Pro chip with 12 CPU cores & 32GB of RAM, running macOS Sonoma 14.4
```bash
# with fast path optimization turned off
/usr/bin/time qsv sqlp taxi.csv --no-optimizations "select VendorID,sum(total_amount) from taxi group by VendorID order by VendorID"
VendorID,total_amount
1,52377417.52985942
2,89959869.13054822
4,600584.610000027
(3, 2)
        6.09 real         6.82 user         0.16 sys

# with fast path optimization, fully exploiting Polars' multithreaded, mem-mapped CSV reader!
 /usr/bin/time qsv sqlp taxi.csv "select VendorID,sum(total_amount) from taxi group by VendorID order by VendorID"
VendorID,total_amount
1,52377417.52985942
2,89959869.13054822
4,600584.610000027
(3, 2)
        0.14 real         1.09 user         0.09 sys

# in contrast, csvq takes 72.46 seconds - 517.57x slower
/usr/bin/time csvq "select VendorID,sum(total_amount) from taxi group by VendorID order by VendorID"
+----------+---------------------+
| VendorID |  SUM(total_amount)  |
+----------+---------------------+
| 1        |  52377417.529256366 |
| 2        |    89959869.1264675 |
| 4        |   600584.6099999828 |
+----------+---------------------+
       72.46 real        65.15 user        75.17 sys
```

### "Traditional" SQL engines
qsv and csvq both operate on "bare" CSVs. For comparison, let's contrast qsv's performance against "traditional" SQL engines
that require setup and import (aka ETL).   Not counting setup and import time (which alone, takes several minutes), we get:

#### **sqlite3.43.2** takes 2.910 seconds - 20.79x slower
```sql
sqlite> .timer on
sqlite> select VendorID,sum(total_amount) from taxi group by VendorID order by VendorID;
1,52377417.53
2,89959869.13
4,600584.61
Run Time: real 2.910 user 2.569494 sys 0.272972
```
#### **PostgreSQL 15.6** using PgAdmin 4 v6.12 takes 18.527 seconds - 132.34x slower
 
![Screenshot 2024-03-06 at 10 14 04 AM](https://github.com/jqnatividad/qsv/assets/1980690/5f7a0eca-d035-46b7-b4df-15991f92c00f)

#### even with an index, qsv sqlp is still 5.96x faster

<img width="996" alt="Screenshot 2024-03-08 at 7 57 57 AM" src="https://github.com/jqnatividad/qsv/assets/1980690/e2919dc6-68fd-4ad9-a56d-9a4dc105b59f">


* `sqlp` now supports JSONL output format and adds compression support for Avro and Arrow output formats.
* `fetch` now has a `--disk-cache` option, so you can cache web service responses to disk, complete with cache control and expiry handling!
* `jsonl` is now multithreaded with additional `--batch` and `--job` options.
* `split` now has three modes: split by record count, split by number of chunks and split by file size.
* `datefmt` is a new top-level command for date formatting. We extracted it from `apply` to make it easier to use, and to set the stage for expanded date and timezone handling.
* `enum` now has a `--start` option.
* `excel` now has a `--keep-zero-time` option and now has improved datetime/duration parsing/handling with upgrade of calamine from 0.23 to 0.24.
* `tojsonl` now has `--trim` and `--no-boolean` options and eliminated false positive boolean inferences. 

---

### Added
* `apply`: add `gender_guess` operation https://github.com/jqnatividad/qsv/pull/1569
* `datefmt`: new top-level command for date formatting.  https://github.com/jqnatividad/qsv/pull/1638
* `enum`: add `--start` option https://github.com/jqnatividad/qsv/pull/1631
* `excel`: added `--keep-zero-time` option; improved datetime/duration parsing/handling with upgrade of calamine from 0.23 to 0.24 https://github.com/jqnatividad/qsv/pull/1595
* `fetch`: add `--disk-cache` option https://github.com/jqnatividad/qsv/pull/1621
* `jsonl`: major performance refactor! Now multithreaded with addl `--batch` and `--job` options https://github.com/jqnatividad/qsv/pull/1553
* `sniff`: added addl mimetype/file formats detected by bumping `file-format` from 0.23 to 0.24 https://github.com/jqnatividad/qsv/pull/1589
* `split`: add `<outdir>` error handling and add usage text examples https://github.com/jqnatividad/qsv/pull/1585
* `split`: added `--chunks` option https://github.com/jqnatividad/qsv/pull/1587
* `split`: add `--kb-size` option https://github.com/jqnatividad/qsv/pull/1613
* `sqlp`: added JSONL output format and compression support for AVRO and Arrow output formats in https://github.com/jqnatividad/qsv/pull/1635
* `tojsonl`: add  `--trim` option https://github.com/jqnatividad/qsv/pull/1554
* Add QSV_DOTENV_PATH env var https://github.com/jqnatividad/qsv/pull/1562
* Add license scan report and status by @fossabot in https://github.com/jqnatividad/qsv/pull/1550
* Added several benchmarks for new/changed commands

### Changed
* `luau`: bumped Luau from 0.606 to 0.614
* `freq`: major performance refactor - https://github.com/jqnatividad/qsv/commit/1a3a4b4f54f7459ce120c2bc907385ad72d34d8e
* `split`: migrate to rayon from threadpool https://github.com/jqnatividad/qsv/pull/1555
* `split`: refactored to actually create chunks <= desired `--kb-size`, obviating need for hacky `--sep-factor` option https://github.com/jqnatividad/qsv/pull/1615
* `tojsonl`: improved true/false boolean inferencing  false positive handling https://github.com/jqnatividad/qsv/pull/1641
* `tojsonl`: fine-tune boolean inferencing https://github.com/jqnatividad/qsv/pull/1643
* `schema`: use parallel sort when sorting enums for fields https://github.com/jqnatividad/qsv/commit/523c60a36bf45b4df5e66f3951a91948c22d5261
* Use array for rustflags to avoid conflicts with user flags by @clarfonthey in https://github.com/jqnatividad/qsv/pull/1548
* Make it easier and more consistent to package for distros by @alerque in https://github.com/jqnatividad/qsv/pull/1549
* Replace `simple_home_dir` with `simple_expand_tilde` crate https://github.com/jqnatividad/qsv/pull/1578
* build(deps): bump rayon from 1.8.0 to 1.8.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1547
* build(deps): bump rayon from 1.8.1 to 1.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1623
* build(deps): bump uuid from 1.6.1 to 1.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1551
* build(deps): bump jql-runner from 7.1.2 to 7.1.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1552
* build(deps): bump jql-runner from 7.1.3 to 7.1.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1602
* build(deps): bump jql-runner from 7.1.5 to 7.1.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1637
* build(deps): bump flexi_logger from 0.27.3 to 0.27.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1556
* build(deps): bump regex from 1.10.2 to 1.10.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1557
* build(deps): bump cached from 0.47.0 to 0.48.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1558
* build(deps): bump cached from 0.48.0 to 0.48.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1560
* build(deps): bump cached from 0.48.1 to 0.49.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1618
* build(deps): bump chrono from 0.4.31 to 0.4.32 by @dependabot in https://github.com/jqnatividad/qsv/pull/1559
* build(deps): bump chrono from 0.4.32 to 0.4.33 by @dependabot in https://github.com/jqnatividad/qsv/pull/1566
* build(deps): bump mlua from 0.9.4 to 0.9.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1565
* build(deps): bump mlua from 0.9.5 to 0.9.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1632
* build(deps): bump serde from 1.0.195 to 1.0.196 by @dependabot in https://github.com/jqnatividad/qsv/pull/1568
* build(deps): bump serde from 1.0.196 to 1.0.197 by @dependabot in https://github.com/jqnatividad/qsv/pull/1612
* build(deps): bump serde_json from 1.0.111 to 1.0.112 by @dependabot in https://github.com/jqnatividad/qsv/pull/1567
* build(deps): bump serde_json from 1.0.112 to 1.0.113 by @dependabot in https://github.com/jqnatividad/qsv/pull/1576
* build(deps): bump serde_json from 1.0.113 to 1.0.114 by @dependabot in https://github.com/jqnatividad/qsv/pull/1610
* bump Polars from 0.36 to 0.37 https://github.com/jqnatividad/qsv/pull/1570
* build(deps): bump polars from 0.37.0 to 0.38.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1629
* build(deps): bump polars from 0.38.0 to 0.38.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1634
* build(deps): bump strum from 0.25.0 to 0.26.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1572
* build(deps): bump indexmap from 2.1.0 to 2.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1575
* build(deps): bump indexmap from 2.2.1 to 2.2.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1579
* build(deps): bump indexmap from 2.2.2 to 2.2.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1601
* build(deps): bump indexmap from 2.2.4 to 2.2.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1633
* build(deps): bump robinraju/release-downloader from 1.8 to 1.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1574
* build(deps): bump itertools from 0.12.0 to 0.12.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1577
* build(deps): bump rust_decimal from 1.33.1 to 1.34.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1580
* build(deps): bump rust_decimal from 1.34.0 to 1.34.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1582
* build(deps): bump rust_decimal from 1.34.2 to 1.34.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1597
* build(deps): bump reqwest from 0.11.23 to 0.11.24 by @dependabot in https://github.com/jqnatividad/qsv/pull/1581
* build(deps): bump tokio from 1.35.1 to 1.36.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1583
* build(deps): bump tempfile from 3.9.0 to 3.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1590
* build(deps): bump tempfile from 3.10.0 to 3.10.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1622
* build(deps): bump indicatif from 0.17.7 to 0.17.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1598
* build(deps): bump csvs_convert from 0.8.8 to 0.8.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1596
* build(deps): bump ahash from 0.8.7 to 0.8.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1599
* build(deps): bump ahash from 0.8.8 to 0.8.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1611
* build(deps): bump ahash from 0.8.9 to 0.8.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1624
* build(deps): bump ahash from 0.8.10 to 0.8.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1640
* build(deps): bump governor from 0.6.0 to 0.6.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1603
* build(deps): bump semver from 1.0.21 to 1.0.22 by @dependabot in https://github.com/jqnatividad/qsv/pull/1606
* build(deps): bump ryu from 1.0.16 to 1.0.17 by @dependabot in https://github.com/jqnatividad/qsv/pull/1605
* build(deps): bump anyhow from 1.0.79 to 1.0.80 by @dependabot in https://github.com/jqnatividad/qsv/pull/1604
* build(deps): bump geosuggest-core from 0.6.0 to 0.6.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1607
* build(deps): bump geosuggest-utils from 0.6.0 to 0.6.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1608
* build(deps): bump pyo3 from 0.20.2 to 0.20.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1616
* build(deps): bump crossbeam-channel from 0.5.11 to 0.5.12 by @dependabot in https://github.com/jqnatividad/qsv/pull/1627
* build(deps): bump log from 0.4.20 to 0.4.21 by @dependabot in https://github.com/jqnatividad/qsv/pull/1628
* build(deps): bump sysinfo from 0.30.5 to 0.30.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1636
* build(deps): bump qsv-sniffer from 0.10.1 to 0.10.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1644
* deps: bump halfbrown from 0.24 to 0.25 https://github.com/jqnatividad/qsv/commit/b32fc7161715fc0d3cc96b1566f89354bea36abf
* apply select clippy suggestions
* update several indirect dependencies
* pin Rust nightly to 2024-02-23 - the nightly that Polars 0.38 can be built with

### Fixed
* fix: fix feature = "cargo-clippy" deprecation by @rex4539 in https://github.com/jqnatividad/qsv/pull/1626
* `stats`: fixed cache.json file not being updated properly https://github.com/jqnatividad/qsv/commit/b9c43713b0943baf2d70eb7089e1d8f05b848b9d

### Removed
* Removed `datefmt` subcommand from `apply` https://github.com/jqnatividad/qsv/pull/1638

## New Contributors
* @clarfonthey made their first contribution in https://github.com/jqnatividad/qsv/pull/1548
* @alerque made their first contribution in https://github.com/jqnatividad/qsv/pull/1549
* @fossabot made their first contribution in https://github.com/jqnatividad/qsv/pull/1550
* @rex4539 made their first contribution in https://github.com/jqnatividad/qsv/pull/1626

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.122.0...0.123.0

## [0.122.0] - 2024-01-17

## 👉  **REQUEST FOR USE CASES**: 👈 
Please help define the future of qsv.
Add what you're currently using qsv for here - https://github.com/jqnatividad/qsv/discussions/1529

Not only does it help us catalog what use cases we should optimize for, posters will get higher priority access to the qsv pro preview.

## Highlights:
* `qsvpy` is now available in the prebuilt binaries for select platforms! It's a new qsv binary variant with the python feature, enabling the `py` command. Three subvariants are available - qsvpy310, qsvpy311 and qsvpy312, corresponding to Python 3.10, 3.11 and 3.12 respectively.
* Removed `generate` command as `generate`'s main dependency is unmaintained and has old dependencies. `generate` was also not used much, as the test data it generated was not well suited for training models and it was too slow so we decided to remove it even before the `fake` (#235) command is ready.
* `reverse` now has index support and can work in "streaming" mode.
* `sort` and `sample` now have faster and cryptosecure Random Number Generators (RNG) with the `--rng` option.
* `pseudo` now has `--start`, `--increment` & `--formatstr` options.
* `fmt` now has a `--no-final-newline` option to suppress the final newline for better interoperability with other tools, specifically Excel. It also treats "T" as special value for tab character for the `--out-delimiter` option.

---

### Added
* `reverse`: now has index support and can work in "streaming" mode https://github.com/jqnatividad/qsv/pull/1531
* `sort`: added `--rng <kind>` for different kinds of RNGs - standard, faster & cryptosecure https://github.com/jqnatividad/qsv/pull/1535
* `sample`: added `--rng <kind>` option (standard, faster & cryptosecure) https://github.com/jqnatividad/qsv/pull/1532
* `pseudo`: major refactor. Added `--start`, `--increment` & `--formatstr` options https://github.com/jqnatividad/qsv/pull/1541
* `fmt`:  add `--no-final-newline` option https://github.com/jqnatividad/qsv/pull/1545
* added additional benchmarks
* added additional test for new options.  We now have ~1,300 tests!

### Changed
* `fmt`: `--out-delimiter` now treats "T" as special value for tab character https://github.com/jqnatividad/qsv/pull/1546
* build(deps): bump whatlang from 0.16.3 to 0.16.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1525
* build(deps): bump serde_json from 1.0.110 to 1.0.111 by @dependabot in https://github.com/jqnatividad/qsv/pull/1524
* build(deps): bump pyo3 from 0.20.1 to 0.20.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1526
* build(deps): bump sysinfo from 0.30.3 to 0.30.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1523
* build(deps): bump sysinfo from 0.30.4 to 0.30.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1530
* build(deps): bump serial_test from 2.0.0 to 3.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1534
* build(deps): bump mlua from 0.9.2 to 0.9.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1540
* build(deps): bump mlua from 0.9.3 to 0.9.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1542
* build(deps): bump simple-home-dir from 0.2.1 to 0.2.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1544
* apply select clippy suggestions
* update several indirect dependencies

### Removed
* removed `generate` command https://github.com/jqnatividad/qsv/pull/1527
* removed `generate` feature from GitHub Action workflows https://github.com/jqnatividad/qsv/pull/1528
* `sample`: removed `--faster` RNG sampling option https://github.com/jqnatividad/qsv/pull/1532

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.121.0...0.122.0

## [0.121.0] - 2024-01-03

We just released 0.120.0 two days ago, and hours later, Polars 0.36 was released, Homebrew released support for Rust 1.75.0 and our [pull request for cached](https://github.com/jaemk/cached/pull/176) was merged!

So we're releasing 0.121.0 out of cycle to take advantage of the latest features and performance improvements of these critical components that underpin qsv!

---

### Added
* `sqlp`: with Polars 0.36, it now supports:
  * subqueries for JOIN and FROM
  * REGEXP and RLIKE pattern matching
  * common variant spelling STDEV in the SQL engine (in addition to STDDEV)
  * and more [under the hood improvements](https://github.com/pola-rs/polars/releases)!
* `sqlp`: now supports writing to Apache Avro format https://github.com/jqnatividad/qsv/commit/32f2fbb1b06dfbee4e7823521e9991a306e7eb44
* `sqlp`: when writing to CSV format, if a file has a TSV or TAB extension, it will automatically use the tab delimiter https://github.com/jqnatividad/qsv/commit/c97048cfc8c0fed01d7b32d3173be508135b9769

### Changed
* Bump polars from 0.35 to 0.36 https://github.com/jqnatividad/qsv/pull/1521
* build(deps): bump serde from 1.0.193 to 1.0.194 by @dependabot in https://github.com/jqnatividad/qsv/pull/1520
* build(deps): bump serde_json from 1.0.109 to 1.0.110 by @dependabot in https://github.com/jqnatividad/qsv/pull/1519
* build(deps): bump semver from 1.0.20 to 1.0.21 by @dependabot in https://github.com/jqnatividad/qsv/pull/1518
* build(deps): bump serde_stacker from 0.1.10 to 0.1.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1517
* build(deps): bump cached from 0.46.1 to 0.47.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1522
* bumped MSRV to 1.75.0

### Fixed
* `cat`: fixed performance regression in `rowskey` by moving unchanging variables out of hot loop - https://github.com/jqnatividad/qsv/commit/96a40e93b5ab09655aa90f8653014c96d3da652b
* `sqlp`: Polars 0.36 fixed the SQL SUBSTR() function

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.120.0...0.121.0

## [0.120.0] - 2024-01-01

Happy New Year! 🎉🎉🎉
Here's the first release of 2024, the biggest ever with 280+ commits! qsv 0.120.0 continues to focus on performance, stability and reliability as we continue setting the stage for qsv's big brother - qsv pro.

Apart from wrapping qsv with a User Interface, qsv pro also comes with a retinue of related cloud-based data cleaning, enrichment and enhancement services along with expanded metadata inferencing to make your _*Data Useful, Usable and Used!*_

qsv pro draws inspiration from [OpenRefine](https://openrefine.org), but reimagined without its file size and speed limitations, with qsv pro having ability to process multi-gigabyte files in seconds.

It incorporates hard lessons we learned in the past 12 years deploying Data Portals and Data Pipelines to create a new Data/Metadata Wrangling and AI-assisted Data Publishing service that's easy to use for casual Excel users and Data Publishers, yet powerful enough for data scientists and data engineers.

But it's not quite ready for release yet, so stay tuned!

We're now taking signups for a preview release however, so if you're interested, please [sign up here](https://dathere.com/qsv-feedback-form/)!

Excitingly, qsv was also mentioned on Hacker News in [this thread](https://news.ycombinator.com/item?id=38733617) Dec 23, 2023! As a result, we're now almost at 2,000+ stars on GitHub from 900 stars on Dec 22! 🎉🎉🎉

Stay tuned for more advancements in 2024 – it's set to be a landmark year for qsv! 🦄🦄🦄

---

### Added
* `cat`: add rowskey --group options; increased perf of rowskey https://github.com/jqnatividad/qsv/pull/1508
* `validate`: add --trim and --quiet options https://github.com/jqnatividad/qsv/pull/1452
* `apply` & `applydp`: `operations regex_replace` now supports  empty `--replacement` with the "<NULL>" special value https://github.com/jqnatividad/qsv/pull/1470 and https://github.com/jqnatividad/qsv/pull/1471
* `exclude`: also consider rows with empty fields https://github.com/jqnatividad/qsv/pull/1498
* `extsort`: add `--tmp-dir` option https://github.com/jqnatividad/qsv/commit/ca1f46145cf6a06ad4401e2bf30f82415cc2ef82

### Changed
* `validate`: Faster RFC4180 validation with byterecords and SIMD-accelerated utf8 validation https://github.com/jqnatividad/qsv/pull/1440
* `excel`: minor performance tweaks https://github.com/jqnatividad/qsv/pull/1446
* `apply`, `applydp`, `explode`, `geocode`, `pseudo`: consolidate redundant code and use one `replace_column_value` helper fn in util.rs https://github.com/jqnatividad/qsv/pull/1456
* `excel`: bump calamine from 0.22 to 0.23 https://github.com/jqnatividad/qsv/pull/1473
* `excel` & `joinp`: use atoi_simd for faster &[u8] to int conversion https://github.com/jqnatividad/qsv/commit/9521f3e3fb73f600e6691188a65e19eda6cd317e
* `cat`, `describegpt`, `headers`, `sqlp`, `to`, `tojsonl`: refactor commands that accept multiple input files to use improved process_input helper https://github.com/jqnatividad/qsv/pull/1496
* `fetch` & `fetchpost`: get_response refactor for maintainability and performance https://github.com/jqnatividad/qsv/pull/1507
* `luau`: replaced --no-colindex option with --colindex option. --col-index slows down processing and is not often used, so make it an option, not the default. https://github.com/jqnatividad/qsv/commit/a0c856807c47f00f531837ae124d412fca834cd2
* make thousands crate optional with apply feature in https://github.com/jqnatividad/qsv/pull/1453
* build(deps): bump uuid from 1.6.0 to 1.6.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1430
* build(deps): bump serde from 1.0.192 to 1.0.193 by @dependabot in https://github.com/jqnatividad/qsv/pull/1432
* build(deps): bump data-encoding from 2.4.0 to 2.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1435
* build(deps): bump mlua from 0.9.1 to 0.9.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1436
* build(deps): bump url from 2.4.1 to 2.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1437
* build(deps): bump jql-runner from 7.0.6 to 7.0.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1439
* build(deps): bump jql-runner from 7.0.7 to 7.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1447
* build(deps): bump jql-runner from 7.1.0 to 7.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1457
* build(deps): bump jql-runner from 7.1.1 to 7.1.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1486
* build(deps): bump hashbrown from 0.14.2 to 0.14.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1441
* build(deps): bump redis from 0.23.3 to 0.23.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1442
* build(deps): bump redis from 0.23.3 to 0.24.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1455
* build(deps): bump atoi_simd from 0.15.3 to 0.15.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1444
* build(deps): bump atoi_simd from 0.15.4 to 0.15.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1445
* build(deps): bump atoi_simd from 0.15.5 to 0.15.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1512
* build(deps): bump actions/setup-python from 4.7.1 to 4.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1454
* build(deps): bump actions/setup-python from 4.8.0 to 5.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1459
* build(deps): bump actions/stale from 8 to 9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1463
* build(deps): bump itoa from 1.0.9 to 1.0.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1464
* build(deps): bump tokio from 1.34.0 to 1.35.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1465
* build(deps): bump tokio from 1.35.0 to 1.35.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1483
* build(deps): bump ryu from 1.0.15 to 1.0.16 by @dependabot in https://github.com/jqnatividad/qsv/pull/1466
* build(deps): bump file-format from 0.22.0 to 0.23.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1468
* build(deps): bump github/codeql-action from 2 to 3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1476
* build(deps): bump geosuggest-utils from 0.5.1 to 0.5.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1479
* build(deps): bump geosuggest-core from 0.5.1 to 0.5.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1478
* build(deps): bump reqwest from 0.11.22 to 0.11.23 by @dependabot in https://github.com/jqnatividad/qsv/pull/1480
* build(deps): bump calamine from 0.23.0 to 0.23.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1481
* build(deps): bump qsv-sniffer from 0.10.0 to 0.10.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1484
* build(deps): bump anyhow from 1.0.75 to 1.0.76 by @dependabot in https://github.com/jqnatividad/qsv/pull/1485
* build(deps): bump futures from 0.3.29 to 0.3.30 by @dependabot in https://github.com/jqnatividad/qsv/pull/1492
* build(deps): bump futures-util from 0.3.29 to 0.3.30 by @dependabot in https://github.com/jqnatividad/qsv/pull/1491
* build(deps): bump crossbeam-channel from 0.5.9 to 0.5.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1490
* build(deps): bump sysinfo from 0.29.10 to 0.29.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/1443
* Bump sysinfo from 0.29.11 to 0.30 https://github.com/jqnatividad/qsv/pull/1489
* build(deps): bump sysinfo from 0.30.0 to 0.30.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1495
* build(deps): bump sysinfo from 0.30.1 to 0.30.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1504
* build(deps): bump sysinfo from 0.30.2 to 0.30.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1509
* build(deps): bump tabwriter from 1.3.0 to 1.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1500
* build(deps): bump tempfile from 3.8.1 to 3.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1502
* build(deps): bump qsv_docopt from 1.4.0 to 1.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1503
* build(deps): bump ahash from 0.8.6 to 0.8.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1510
* build(deps): bump serde_json from 1.0.108 to 1.0.109 by @dependabot in https://github.com/jqnatividad/qsv/pull/1511
* apply select clippy suggestions
* update several indirect dependencies
* pin Rust nightly to 2023-12-23

### Fixed
* `apply`: Fix for `dynfmt` and `calcconv` subcommands not working in release mode https://github.com/jqnatividad/qsv/pull/1467
* `luau`:  fix check for excess mapped columns earlier. Otherwise, we'll get a CSV different field count error https://github.com/jqnatividad/qsv/commit/db1581159590205af9befaade5c047d316c9c8b3

### Removed
* `luau`: remove unneeded `--jit` option as we precompile luau scripts to bytecode https://github.com/jqnatividad/qsv/pull/1438

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.119.0...0.120.0

## [0.119.0] - 2023-11-19

## Highlights:
As we prepare for version 1.0, we're focusing on performance, stability and reliability as we set the stage for qsv pro - a cloud-backed UI version of qsv powered by [Tauri](https://tauri.app), set to be released in 2024. Stay tuned!

* `diff` is now out of beta and blazingly fast! Give _"the fastest CSV-diff in the world"_ a try :wink:!
* `joinp` now supports snappy automatic compression/decompression!
* `sqlp` & `joinp` now recognize the `QSV_COMMENT_CHAR` environment variable, allowing you to skip comment lines in your input CSV files. They're also faster with the upgrade to Polars 0.35.4.
* `sqlp`now supports subqueries, table aliases, and more!
* `luau`: upgraded embedded Luau from 0.599 to 0.604; refactored code to reduce unneeded allocations and increase performance as we prepare for [extended recipe support](https://github.com/jqnatividad/qsv/issues/1419).
* `cat` is now even faster with the `--flexible` option. If you know your CSV files are valid, you can use this option to skip CSV validation and make `cat` run twice as fast!
* qsv can now add a [Byte Order Mark](https://en.wikipedia.org/wiki/Byte_order_mark) (BOM) header sequence for Excel-friendly CSVs on Windows with the `QSV_OUTPUT_BOM` environment variable.
* `stats`, `sort`, `schema` & `validate` are now faster with the use of `atoi_simd` to directly convert &[u8] to integer, skipping unnecessary utf8 validation, while also using SIMD CPU instructions for noticeably faster performance.

### Added
* `diff`: added option/flag for headers in output by @janriemer in https://github.com/jqnatividad/qsv/pull/1395
* `diff`: added option/flag `--delimiter-output` by @janriemer in https://github.com/jqnatividad/qsv/pull/1402
* `cat`: added `--flexible` option to make `cat rows` faster still https://github.com/jqnatividad/qsv/pull/1408
* `sqlp` & `joinp`: both commands now recognize QSV_COMMENT_CHAR env var https://github.com/jqnatividad/qsv/pull/1412
* `joinp`: added snappy compression/decompression support https://github.com/jqnatividad/qsv/pull/1413
* `geocode`: now automatically decompresses snappy-compressed index files https://github.com/jqnatividad/qsv/pull/1429
* Add Byte Order Mark (BOM) output support https://github.com/jqnatividad/qsv/pull/1424
* Added Codacy code quality badge https://github.com/jqnatividad/qsv/commit/99591297d59b3c45363592516d5ecb7d4d98d5c8

### Changed
* `stats`, `sort`, `schema` & `validate`: use atoi_simd to directly convert &[u8] to integer skipping unnecessary utf8 validation, while also using SIMD instructions for noticeably faster performance
* `cat`: faster `cat rows` https://github.com/jqnatividad/qsv/pull/1407
* `count`: optimize `--width` option https://github.com/jqnatividad/qsv/pull/1411
* `luau`: upgrade embedded Luau from 0.603 to 0.604 https://github.com/jqnatividad/qsv/pull/1426
* use `ato_simd` for fast &[u8] to int conversion https://github.com/jqnatividad/qsv/pull/1423
* `luau`: performance refactor https://github.com/jqnatividad/qsv/commit/4cebd7c9a4b3f9f754fd2746484c24fa61ee1286
* build(deps): bump csv-diff from 0.1.0-beta.4 to 0.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1394
* build(deps): bump serde_json from 1.0.107 to 1.0.108 by @dependabot in https://github.com/jqnatividad/qsv/pull/1393
* build(deps): bump indexmap from 2.0.2 to 2.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1397
* build(deps): bump jql-runner from 7.0.4 to 7.0.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1399
* build(deps): bump jql-runner from 7.0.5 to 7.0.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1400
* build(deps): bump file-format from 0.21.0 to 0.22.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1401
* build(deps): bump cached from 0.46.0 to 0.46.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1403
* build(deps): bump serde from 1.0.190 to 1.0.192 by @dependabot in https://github.com/jqnatividad/qsv/pull/1404
* build(deps): bump tokio from 1.33.0 to 1.34.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1409
* build(deps): bump flexi_logger from 0.27.2 to 0.27.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1410
* build(deps): bump qsv-stats from 0.11.0 to 0.12.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1415
* build(deps): bump itertools from 0.11.0 to 0.12.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1418
* build(deps): bump rust_decimal from 1.33.0 to 1.33.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1420
* build(deps): bump polars from 0.35.2 to 0.35.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1425
* build(deps): bump uuid from 1.5.0 to 1.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1428
* bump MSRV to 1.74.0
* apply select clippy suggestions
* update several indirect dependencies
* pin Rust nightly to 2023-11-18

### Fixed
* `pseudo`: detect when more than one column is selected for pseudonymization https://github.com/jqnatividad/qsv/commit/0b093726bb964c2a4a6eec15c0e30ed3660fdf97
* dotenv (.env) tweaks/fixes https://github.com/jqnatividad/qsv/pull/1427
* fix several typos https://github.com/jqnatividad/qsv/commit/723443eed4ac0f692cdd6ac4a1af4d82e22fda8b
* fix several markdown lints

### Removed
* remove fast-float as std float parse is now also using Eisel-Lemire algorithm https://github.com/jqnatividad/qsv/pull/1414

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.118.0...0.119.0

## [0.118.0] - 2023-10-27

## Highlights:
* With the Polars upgrade to [0.34.2](https://github.com/pola-rs/polars/releases/tag/rs-0.34.0), the `sqlp` and `joinp` enjoy [expanded](https://github.com/pola-rs/polars/blob/rs-0.34.0/crates/polars-sql/src/functions.rs
) [capabilities](https://github.com/pola-rs/polars/blob/rs-0.34.0/crates/polars-sql/src/keywords.rs) and a noticeable performance boost. 🦄🏇
* We now publish the 500, 1000, 5000 and 15000 Geonames cities indices for the `geocode` command, with users able to easily switch indices with the `index-load` subcommand. As the name implies, the 500 index contains cities with populations of 500 or more, the 1000 index contains cities with populations of 1000 or more, and so on.   
The 15000 index (default) is the smallest (13mb) and fastest with ~26k cities. The 500 index is the largest(56mb) and slowest, with ~200k cities.  The 5000 index is 21mb with ~53k cities. The 1000 index is 44mb with ~140k cities. 🎠
* The `geocode` command now returns US Census FIPS codes for US places with the `%json` and `%pretty-json` formats, returning both US State and US County FIPS codes, with upcoming support for Cities and other US Census geographies (School Districts, Voting Districts, Congressional Districts, etc.) 🎠
* Improved performance for `stats`, `schema` and `tojsonl` commands with the stats cache bincode refactor. This is especially noticeable for large CSV files as `stats`  previously created large bincode cache files by default.   
The bincode cache allows other commands (currently, only `schema` and `tojsonl`) to skip recomputing statistics and deserialize the saved stats data structures directly into memory. Now, it will only create a bincode file if the `--stats-binout` option is specified (typically, before using the `schema` an `tojsonl` commands). `stats` will still continue to create a stats CSV cache file by default, but it will be much smaller than the bincode file, and is universally applicable, unlike the bincode cache. 🏇
* self-update will now verify updates. This is done by verifying the [zipsign](https://crates.io/crates/zipsign) signature of the release zip archive before applying it. This should make it harder for malicious actors to compromise the self-update process. Version 0.118.0 has the verification code, and future releases will use this new verification process.
Regardless, we will zipsign all zip archives starting with this release.
Users can manually verify the signatures by downloading the zipsign public key and running the `zipsign` command line tool. See [Verifying the Integrity of the Prebuilt Binaries Zip Archive](README.md#verifying-the-integrity-of-the-prebuilt-binaries-zip-archives) for more info. 🦄
* The `frequency` command now supports the `--ignore-case` option for case-insensitive frequency counts. 🦄🎠
* The `schema` command can now compile case-insensitive enum constraints. 🦄
* Improved performance for `apply` and `applydp` commands with faster compile-time perfect hash functions for operations lookups. 🏇
* Several minor performance improvements and bug fixes with `snappy`, `sniff` & `cat` commands. 🏇

---

### Added
* `frequency`: added `--ignore-case` option https://github.com/jqnatividad/qsv/pull/1386
* `geocode`: added 500, 1000, 5000, 15000 Geonames cities convenience shortcuts to `index` subcommands https://github.com/jqnatividad/qsv/commit/bd9f4c34b0a88cc6a446872ed4cda41e8a1ca102
* `schema`: added `--ignore-case` option when compiling enum constraints; replaced Hashset with faster AHashset https://github.com/jqnatividad/qsv/commit/a16a1ca25f93699a5ee27327f4257e8e559bc5e8
* `snappy`: added `buf_size` param to compress helper fn https://github.com/jqnatividad/qsv/commit/e0c0d1f7eb22917d43f638121babe23e366c9dd8
* `sniff` added `--just-mime` option https://github.com/jqnatividad/qsv/pull/1372
* added zipsign signature verification to self-update https://github.com/jqnatividad/qsv/pull/1389

### Changed
* `apply` & `applydp`: replaced binary_search with faster compile-time perfect hash functions for operations lookups https://github.com/jqnatividad/qsv/pull/1371
* `stats`, `schema` and `tojsonl`:  stats cache bincode refactor https://github.com/jqnatividad/qsv/pull/1377
* `luau`: replaced sanitise-file-name with more popular sanitize-filename crate https://github.com/jqnatividad/qsv/commit/8927cb70bc92e9e1360547e96d1ac10e6037e9e3
* `cat`: minor optimization by preallocating with capacity https://github.com/jqnatividad/qsv/commit/c13c34120c47bb7ab603a97a0a7cae7f0de7b146
* `sqlp` & `joinp`: expanded speed/functionality with upgrade to Polars 0.34.2 https://github.com/jqnatividad/qsv/pull/1385
* `tojsonl`: improved boolean inferencing. Now correctly infers booleans, even if the enum domain range is more than 2, but has cardinality 2 case-insensitive https://github.com/jqnatividad/qsv/commit/6345f2dc01f6451075ba7f23c35d8ba8cced9293
* build(deps): bump strum_macros from 0.25.2 to 0.25.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1368
* build(deps): bump regex from 1.10.1 to 1.10.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1369
* build(deps): bump uuid from 1.4.1 to 1.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1373
* build(deps): bump hashbrown from 0.14.1 to 0.14.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1376
* build(deps): bump self_update from 0.38.0 to 0.39.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1378
* build(deps): bump ahash from 0.8.5 to 0.8.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1383
* build(deps): bump serde from 1.0.189 to 1.0.190 by @dependabot in https://github.com/jqnatividad/qsv/pull/1388
* build(deps): bump futures from 0.3.28 to 0.3.29 by @dependabot in https://github.com/jqnatividad/qsv/pull/1390
* build(deps): bump futures-util from 0.3.28 to 0.3.29 by @dependabot in https://github.com/jqnatividad/qsv/pull/1391
* build(deps): bump tempfile from 3.8.0 to 3.8.1 by @dependabot in https://github.com/jqnatividad/qsv/commit/4f6200cb57fdeb612aeb74d796b4b0c1fde7c243
* apply select clippy suggestions
* update several indirect dependencies
* pin Rust nightly to 2023-10-26

### Fixed
* `dedup`: fixed --ignore-case not being honored during internal sort option https://github.com/jqnatividad/qsv/pull/1387
* `applydp`: fixed wrong usage text using `apply` and not `aaplydp` https://github.com/jqnatividad/qsv/commit/c47ba86f305508a41e19ce39f2bd6323a0a60e1e
* `geocode`: fixed `index-update` not honoring `--timeout` parameter https://github.com/jqnatividad/qsv/commit/3272a9e3ac75e8b8f2d9f13b0cec81a0c41c7ed4
* `geocode` : fixed `index-load` to work properly with convenience shortcuts https://github.com/jqnatividad/qsv/commit/5097326ee41d39787b472b4eea95ddec76bb06b5

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.117.0...0.118.0

## [0.117.0] - 2023-10-15

## Highlights:
* `geocode`: added Federal Information Processing Standards (FIPS) codes to results for US places, so we can derive [GEOIDs](https://www.census.gov/programs-surveys/geography/guidance/geo-identifiers.html#:~:text=FIPS%20codes%20are%20assigned%20alphabetically,Native%20Hawaiian%20(AIANNH)%20areas.).  This paves the way to doing data enrichment lookups (starting with the US Census) in an upcoming release.
* Added [Goal/Non-goals](https://github.com/jqnatividad/qsv#goals--non-goals), explicitly codifying what qsv is and isn't, and what we're trying to achieve with the toolkit.
* `excel`: CSV output processing is now multithreaded, making it a bit faster. The bottleneck is still the Excel/ODS library we're using ([calamine](https://github.com/tafia/calamine)), which is single-threaded. But there are [active](https://github.com/tafia/calamine/issues/346) [discussions](https://github.com/tafia/calamine/issues/362) underway to make it much faster in the future.
* Upgrading the MSRV to 1.73.0 has allowed us to use LLVM 17, which has resulted in an overall performance boost.

---

### Added:
* `geocode`: added Federal Information Processing Standards (FIPS) codes to results for US places.
* Added Goals/Non-goals to README.md

### Changed
* `cat` : minor optimization https://github.com/jqnatividad/qsv/commit/343bb668ae84fcf862883245382e7d8015da88c2
* `excel`: CSV output processing is now multithreaded https://github.com/jqnatividad/qsv/pull/1360
* `geocode`: more efficient dynfmt ptocessing https://github.com/jqnatividad/qsv/pull/1367
* `frequency`: optimize allocations before hot loop https://github.com/jqnatividad/qsv/commit/655bebcdec6d89f0ffa33d794069ee5eee0df3e5
* `luau`: upgraded embedded Luau from 0.596 to 0.599
* `deps`: bump calamine from 0.22.0 to 0.22.1 https://github.com/jqnatividad/qsv/commit/4c4ed7e25614bbfe4d7b16fe7619a5a874ef7591
* `docs`: reorganized README, moving FEATURES and INTERPRETERS to their own markdown files.
* build(deps): bump byteorder from 1.4.3 to 1.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1347
* build(deps): bump tokio from 1.32.0 to 1.33.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1354
* build(deps): bump regex from 1.9.6 to 1.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1356
* build(deps): bump semver from 1.0.19 to 1.0.20 by @dependabot in https://github.com/jqnatividad/qsv/pull/1358
* build(deps): bump pyo3 from 0.19.2 to 0.20.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1359
* build(deps): bump serde from 1.0.188 to 1.0.189 by @dependabot in https://github.com/jqnatividad/qsv/pull/1361
* build(deps): bump flate2 from 1.0.27 to 1.0.28 by @dependabot in https://github.com/jqnatividad/qsv/pull/1363
* build(deps): bump regex from 1.10.0 to 1.10.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1366
* `deps`: update several indirect dependencies
* pin Rust nightly to 2023-10-14
* bump MSRV to 1.73.0

### Removed
* `excel`: removed `--progressbar` option as Excel/ODS maximum sheet size is just too small (1,048,576 rows) to make it useful. 

### Fixed
* Fixed Jupyter Notebook Viewer Link  by @a5dur in https://github.com/jqnatividad/qsv/pull/1349

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.116.0...0.117.0

## [0.116.0] - 2023-10-05

## Highlights: :tada: :rocket:
* Benchmarks refinements galore with more benchmarks and more comprehensive benchmarking instructions. 🎠
* `geocode`: The Geonames index's configuration metadata is now available with the `geocode index-check` command.  No need to maintain a separate metadata JSON file.  This should make it even easier to maintain multiple Geonames index files with different configurations without having to worry you're looking at the right metadata JSON file. 🎠
* `tojsonl`: parallelized with rayon, making it much faster. 🏇🏽
* smaller qsv binary size and faster compile times with the `to_parquet` feature.  If you're good enough with `sqlp`'s ability to create a parquet file from a SQL query, qsv's binary size and compile time will be markedly smaller/faster. 🏇🏽
* minor perf tweaks to `cat`, `count` and `luau` commands 🏇🏽

---

### Added
* `geocode`: added Geonames index file metadata to `index-check` command
* `tojsonl`: parallelized with rayon https://github.com/jqnatividad/qsv/pull/1338
* `to`: added `to_parquet` feature. https://github.com/jqnatividad/qsv/pull/1341
* `benchmarks`: upgraded from 3.0.0 to 3.3.1
  * you can now specify a separate benchmarking binary as we dogfood qsv for the benchmarks and some features are required that may not be in the qsv binary variant being benchmarked
  * added additional `count` benchmarks with `--width` option
  * added additional `luau` benchmarks with single/multi filter options
  * added additional `search` benchmark with `--unicode` option
  * show absolute path of qsv binaries used (both the one we're dogfooding and the one being benchmarked) and their version info before running the benchmarks proper
  * ensured `schema` benchmark was not using the stats cache with the `--force` option

### Changed
* `cat`: use an empty byte_record var instead of repeatedly allocating a new one https://github.com/jqnatividad/qsv/commit/eddafd11acb8e8d9d8587f952ba8cd02d450b08e
* `count`: minor optimization https://github.com/jqnatividad/qsv/commit/bb113c0f348d4903ebfdc893c09517e5a4b145ad
* `luau`: minor perf tweaks https://github.com/jqnatividad/qsv/commit/c71cd16a22f729a074a2a8d59020eba4cc8d7281 and https://github.com/jqnatividad/qsv/commit/f9c1e3c755fdb847be8f7f54d21622fb0c8c747f
* (deps): bump Geosuggest from 0.4.5 to 5.1 https://github.com/jqnatividad/qsv/pull/1333
* (deps): use patched version of calamine which has unreleased fixes since 0.22.0
* build(deps): bump flexi_logger from 0.27.0 to 0.27.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1328
* build(deps): bump indexmap from 2.0.0 to 2.0.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1329
* build(deps): bump hashbrown from 0.14.0 to 0.14.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1334
* build(deps): bump file-format from 0.20.0 to 0.21.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1335
* build(deps): bump indexmap from 2.0.1 to 2.0.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1336
* build(deps): bump regex from 1.9.5 to 1.9.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1337
* build(deps): bump jql-runner from 7.0.3 to 7.0.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1340
* build(deps): bump csvs_convert from 0.8.7 to 0.8.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1339
* build(deps): bump actions/setup-python from 4.7.0 to 4.7.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1342
* build(deps): bump reqwest from 0.11.21 to 0.11.22 by @dependabot in https://github.com/jqnatividad/qsv/pull/1343
* build(deps): bump csv from 1.2.2 to 1.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1344
* build(deps): bump actix-governor from 0.4.1 to 0.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1346
* applied select clippy suggestions
* update several indirect dependencies
* pin Rust nightly to 2023-10-04

### Removed
* `geocode`: removed separate metadata JSON file for Geonames index files. The metadata is now embedded in the index file itself and can be viewed with the `index-check` command. 🎠
* removed redundant setting from profile.release-samply in Cargo.toml https://github.com/jqnatividad/qsv/commit/2a35be5bbae2fc6994c103acac37ea3559854a0a

### Fixed
* `geocode`: when producing JSON output with the now subcommands, we now produced valid JSON. We previously generated JSON with extra quotes. https://github.com/jqnatividad/qsv/pull/1345
* `schema`: fixed `--force` flag not being honored


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.115.0...0.116.0

## [0.115.0] - 2023-09-26 -🏇🏽🎠
We continue to refine the benchmark suite, and have added a new `setup` argument to setup and install the required tools for the benchmark suite.  We've also added more comprehensive checks to ensure that the required tools are installed before running the benchmarks. 🎠

For `geocode`, we've added a JSON file describing the Geonames index file configuration. This should help users maintain several Geonames index files with different configurations. 🎠

`geocode` should also be a tad faster now, thanks to `cached` crate making ahash its default hashing algorithm and upgrading `hashbrown` - microbenchmarks show a 33% performance improvement. 🏇🏽

We also added a `release-samply` profile so we can make it easier to squeeze more performance out of the toolkit with [`samply`](https://github.com/mstange/samply/#samply). 🏇🏽

---

### Added
* `geocode`: added a JSON file describing the Geonames index file configuration in https://github.com/jqnatividad/qsv/pull/1324
* `benchmarks`: v3.0.0 release
  * added `setup` argument to setup and install required tools for the benchmark suite
  * added more comprehensive required tools check
  * added more realistic luau benchmarks, using helper luau scripts
    (dt_format.luau and turnaround_time.luau)
  * added stats with_cache and create_cache benchmarks
  * added benchmark_aggregations.luau script for benchmark analysis
  * captured `binary`, `total_mean` and `qsv_env` columns to benchmark results
    `binary` is the qsv binary variant used
    `total_mean` is the sum of all the mean run times of the benchmarks
    `qsv_env` are the qsv-relevant environment variables active while running the benchmarks
  * expanded README.md and benchmark suite usage instructions
* added `release-samply` profile to Cargo.toml to facilitate continued performance optimization with [`samply`](https://github.com/mstange/samply/#samply)

### Changed
* `readme`: move tab completion instructions/script to scripts/misc
* `geocode`: updated bundled Geonames index to 2021-09-25
* bump embedded luau from 0.594 to 0.596
* build(deps): bump flexi_logger from 0.26.1 to 0.27.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1317
* build(deps): bump indicatif from 0.17.6 to 0.17.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1318
* build(deps): bump semver from 1.0.18 to 1.0.19 by @dependabot in https://github.com/jqnatividad/qsv/pull/1320
* build(deps): bump cached from 0.45.1 to 0.46.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1322
* build(deps): bump geosuggest-core from 0.4.3 to 0.4.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1323
* build(deps): bump geosuggest-utils from 0.4.3 to 0.4.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1321
* build(deps): bump fastrand from 2.0.0 to 2.0.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1325
* bump MSRV from Rust 1.72.0 to 1.72.1
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-09-25

### Fixed
* `benchmarks`: fixed invalid luau benchmark that had invalid luau command

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.114.0...0.115.0

## [0.114.0] - 2023-09-21 🏇🏽🎠
The long-overdue Benchmarks revamp is finally here! 🎉

The benchmarks have been completely rewritten to be more reproducible, and now use [hyperfine](https://github.com/sharkdp/hyperfine#hyperfine) instead of `time`.  The new benchmarks are now run as part of the release process, and the results are compiled into a single page that is published on the new [Quicksilver website](https://qsv.dathere.com/benchmarks/).

The new benchmarks are also more comprehensive, and designed to be run on a variety of hardware and operating systems. This allows users to adapt the benchmarks to their own workloads and environments.

Other release highlights include:
* [`geocode`](https://github.com/jqnatividad/qsv/blob/master/src/cmd/geocode.rs#L2) is now fully-featured and ready for production use! 🎉 Though it only currently features Geonames city-level lookup support, it provides a solid foundation on top of which we'll add more geocoding providers in the future (next up - [OpenCage support](https://github.com/jqnatividad/qsv/issues/1295) with street-level geocoding).
* [Polars](https://www.pola.rs) has been bumped from 0.32.1 to [0.33.2](https://github.com/pola-rs/polars/releases/tag/rs-0.33.0), which includes a number of performance improvements for the `sqlp` and `joinp` commands.
* major performance increase on several `regex`/`aho-corasick` powered commands on Apple Silicon thanks to various under-the-hood improvements in the [`aho-corasick`](https://www.reddit.com/r/rust/comments/16lvyyg/ahocorasick_and_thus_the_regex_crate_too_now_uses/) crate.

---

### Added
* Added autoindex size threshold, replacing `QSV_AUTOINDEX` env var with `QSV_AUTOINDEX_SIZE`. Resolves #1300. in https://github.com/jqnatividad/qsv/pull/1301 https://github.com/jqnatividad/qsv/commit/69e25aceb25d3bb20d8fdeeadf5504d8fe75fe37
* `diff`: Added test for different delimiters by @janriemer in https://github.com/jqnatividad/qsv/pull/1297
* `benchmarks`: Added qsv benchmark notebook. by @a5dur in https://github.com/jqnatividad/qsv/pull/1309
* `geocode`: Added `countryinfo/now` subcommand made available in geosuggest 0.4.3 https://github.com/jqnatividad/qsv/pull/1311
* `geocode`: Added `--language` option so users can specify the language of the geocoding results. This requires running the `index-update` subcommand with the `--languages` option to rebuild the index with the desired languages.
* `sqlp`: add example of using columns with embedded spaces in SQL queries https://github.com/jqnatividad/qsv/commit/f7bf4f65edc2068f42712808aec7096ef7122dfe

### Changed
* `benchmarks`: Benchmarks revamped https://github.com/jqnatividad/qsv/pull/1298, https://github.com/jqnatividad/qsv/pull/1310 https://github.com/jqnatividad/qsv/commit/d8eeb949b8c846793941eb9c343e8598784b6207
* build(deps): bump serde_json from 1.0.106 to 1.0.107 by @dependabot in https://github.com/jqnatividad/qsv/pull/1302
* build(deps): bump mimalloc from 0.1.38 to 0.1.39 by @dependabot in https://github.com/jqnatividad/qsv/pull/1303
* build(deps): bump simple-home-dir from 0.1.4 to 0.2.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1304
* build(deps): bump chrono from 0.4.30 to 0.4.31 by @dependabot in https://github.com/jqnatividad/qsv/pull/1305
* (deps): bump Polars from 0.32.1 to Polars 0.33.2 https://github.com/jqnatividad/qsv/pull/1308
* build(deps): bump cpc from 1.9.2 to 1.9.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1313
* build(deps): bump rayon from 1.7.0 to 1.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1315
* (deps): update several indirect dependencies
* pin Rust nightly to 2023-09-21

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.113.0...0.114.0

## [0.113.0] - 2023-09-08 🦄🏇🏽🎠
This is the first "[Unicorn](https://7esl.com/unicorn/)" 🦄 release, adding MAJOR new features to the toolkit!

* `geocode`: adds high-speed, cache-backed, multithreaded geocoding using a local, updateable copy of the [GeoNames](https://www.geonames.org/) database.  This is a major improvement over the previous `geocode` subcommand in the `apply` command thanks to the wonderful [geosuggest](https://github.com/estin/geosuggest) crate.
* guaranteed non-UTF8 input detection with the `validate` and `input` commands. Quicksilver [_REQUIRES_ UTF-8 encoded input](https://github.com/jqnatividad/qsv/tree/master#utf-8-encoding). You can now use these commands to ensure you have valid UTF-8 input before using the rest of the toolkit.
* New/expanded whirlwind tour & quick-start notebooks by @a5dur and @rzmk 🎠
* Various performance improvements all-around: 🏇🏽
  * overall increase of ~5% now that `mimalloc` - the default allocator for qsv, is built without secure mode unnecessarily enabled.
  * `flatten` command is now ~10% faster
  * faster regex performance thanks to various under-the-hood improvements in the [`regex`](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md#195-2023-09-02) crate
  * and the benchmark scripts have been updated by @minhajuddin2510 to use [hyperfine](https://github.com/sharkdp/hyperfine#hyperfine) instead of time, and to use the same input file for all benchmarks to make them more reproducible. In upcoming releases, we'll start compiling the benchmark results into a single page as part of the release process, so we can track our progress over time.

and last but not least - Quicksilver now has a website! - https://qsv.dathere.com/ :unicorn: :tada: :rocket:

And its not just a static site with a few links - its a full-blown web app that lets you try out qsv commands in your browser!  It's not just a demo site - you can use it as a configurator and save your commands to a gist and share them with others!

It's the first Beta release of the Quicksilver website, so there's still a lot of work to do, but we're excited to share it with you and get your [feedback](https://dathere.com/qsv-feedback-form/)!

We have more exciting features planned for Quicksilver and the website, but we require your help to make it happen! For qsv, use [GitHub issues](https://github.com/jqnatividad/qsv/issues). For the website, use the [feedback form](https://dathere.com/qsv-feedback-form/).  And if you want to help out, please check out the [contributing guide](https://github.com/jqnatividad/qsv/blob/master/CONTRIBUTING.md)

Big thanks to @rzmk for all the work on the website! To @a5dur for all the QA work on this release! And to @minhajuddin2510 for revamping the benchmark script!

---

### Added
* `geocode`: new high-speed geocoding command  https://github.com/jqnatividad/qsv/pull/1231
  * major improvements using geosuggest upstream  https://github.com/jqnatividad/qsv/pull/1269
  * add  suggest `--country` filter  https://github.com/jqnatividad/qsv/pull/1275
  * add `--admin1` filter  https://github.com/jqnatividad/qsv/pull/1276
  * automatic `--country` inferencing from `--admin1` code  https://github.com/jqnatividad/qsv/pull/1277    
  * add `--suggestnow` and `--reversenow` subcommands  https://github.com/jqnatividad/qsv/pull/1280
  * add `"%dyncols:"` special formatter to dynamically add geocoded columns to the output CSV https://github.com/jqnatividad/qsv/pull/1286
* `excel`: add SheetType (Worksheet, DialogSheet, MacroSheet, ChartSheet, VBA) in metadata mode; log.info! headers; wordsmith comments  https://github.com/jqnatividad/qsv/pull/1225
* `excel`: moar metadata! moar examples!  https://github.com/jqnatividad/qsv/pull/1271
* add support ALL_PROXY env var  https://github.com/jqnatividad/qsv/pull/1233
* `input`: add `--encoding-errors` handling option  https://github.com/jqnatividad/qsv/pull/1235
* `fixlengths`: add `--insert` option  https://github.com/jqnatividad/qsv/pull/1247
* `joinp`: add `--sql-filter` option  https://github.com/jqnatividad/qsv/pull/1287
* `luau`: we now embed [Luau 0.594](https://github.com/Roblox/luau/releases/tag/0.594) from 0.592
* `notebooks`: add qsv-colab-quickstart by @rzmk in https://github.com/jqnatividad/qsv/pull/1253
* `notebooks`: Added Whirlwindtour.ipynb by @a5dur in https://github.com/jqnatividad/qsv/pull/1223

### Changed
* `flatten`: refactor for performance  https://github.com/jqnatividad/qsv/pull/1227
* `validate`: improved utf8 error messages  https://github.com/jqnatividad/qsv/pull/1256
* `apply` & `applydp`: improve usage text in relation to multi-column capabilities  https://github.com/jqnatividad/qsv/pull/1257
* qsv-cache now set to ~/.qsv-cache by default  https://github.com/jqnatividad/qsv/pull/1265
* Download file helper refactor  https://github.com/jqnatividad/qsv/pull/1267
* Benchmark Update by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/1237
* Improved error handling  https://github.com/jqnatividad/qsv/pull/1238
* Improved error handling - incorrect usage errors are now differentiated from other errors as well  https://github.com/jqnatividad/qsv/pull/1239
* build(deps): bump whatlang from 0.16.2 to 0.16.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1221
* build(deps): bump serde_json from 1.0.104 to 1.0.105 by @dependabot in https://github.com/jqnatividad/qsv/pull/1220
* build(deps): bump tokio from 1.31.0 to 1.32.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1222
* build(deps): bump mlua from 0.9.0-rc.3 to 0.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1224
* build(deps): bump tempfile from 3.7.1 to 3.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1226
* build(deps): bump postgres from 0.19.5 to 0.19.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1229
* build(deps): bump file-format from 0.18.0 to 0.19.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1228
* build(deps): bump reqwest from 0.11.18 to 0.11.19 by @dependabot in https://github.com/jqnatividad/qsv/pull/1232
* build(deps): bump rustls-webpki from 0.101.3 to 0.101.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1236
* build(deps): bump reqwest from 0.11.19 to 0.11.20 by @dependabot in https://github.com/jqnatividad/qsv/pull/1241
* build(deps): bump rust_decimal from 1.31.0 to 1.32.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1242
* build(deps): bump serde from 1.0.185 to 1.0.186 by @dependabot in https://github.com/jqnatividad/qsv/pull/1243
* build(deps): bump jql-runner from 7.0.2 to 7.0.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1246
* build(deps): bump grex from 1.4.2 to 1.4.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1245
* build(deps): bump mlua from 0.9.0 to 0.9.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1244
* build(deps): bump mimalloc from 0.1.37 to 0.1.38 by @dependabot in https://github.com/jqnatividad/qsv/pull/1249
* build(deps): bump postgres from 0.19.6 to 0.19.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1251
* build(deps): bump serde from 1.0.186 to 1.0.187 by @dependabot in https://github.com/jqnatividad/qsv/pull/1250
* build(deps): bump serde from 1.0.187 to 1.0.188 by @dependabot in https://github.com/jqnatividad/qsv/pull/1252
* build(deps): bump regex from 1.9.3 to 1.9.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1254
* build(deps): bump url from 2.4.0 to 2.4.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1261
* build(deps): bump tabwriter from 1.2.1 to 1.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1259
* build(deps): bump sysinfo from 0.29.8 to 0.29.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1260
* build(deps): bump actix-web from 4.3.1 to 4.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1262
* build(deps): bump chrono from 0.4.26 to 0.4.27 by @dependabot in https://github.com/jqnatividad/qsv/pull/1264
* build(deps): bump chrono from 0.4.27 to 0.4.28 by @dependabot in https://github.com/jqnatividad/qsv/pull/1266
* build(deps): bump redis from 0.23.2 to 0.23.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1268
* build(deps): bump regex from 1.9.4 to 1.9.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1272
* build(deps): bump flexi_logger from 0.25.6 to 0.26.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1273
* build(deps): bump geosuggest-core from 0.4.0 to 0.4.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1279
* build(deps): bump geosuggest-utils from 0.4.0 to 0.4.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1278
* build(deps): bump cached from 0.44.0 to 0.45.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1282
* build(deps): bump self_update from 0.37.0 to 0.38.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1281
* build(deps): bump actions/checkout from 3 to 4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1283
* build(deps): bump chrono from 0.4.28 to 0.4.29 by @dependabot in https://github.com/jqnatividad/qsv/pull/1284
* build(deps): bump cached from 0.45.0 to 0.45.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1285
* build(deps): bump sysinfo from 0.29.9 to 0.29.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1288
* build(deps): bump chrono from 0.4.29 to 0.4.30 by @dependabot in https://github.com/jqnatividad/qsv/pull/1290
* build(deps): bump bytes from 1.4.0 to 1.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1289
* build(deps): bump file-format from 0.19.0 to 0.20.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1291
* cargo update bump several indirect dependencies
* apply select clippy suggestions
* pin Rust nightly to 2023-09-06
  
### Removed
* `apply`: remove geocode subcmd now that we have a dedicated `geocode` command  https://github.com/jqnatividad/qsv/pull/1263

### Fixed
* `excel`: we can now open workbooks with formulas set to an empty string value  https://github.com/jqnatividad/qsv/pull/1274
* `notebooks`: fix qsv colab quickstart link by @rzmk in https://github.com/jqnatividad/qsv/pull/1255
  
**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.112.0...0.113.0

## [0.112.0] - 2023-08-15 🏇🏽🎠
This is the second in a series of "[Giddy-up](https://7esl.com/giddy-up/)" 🏇🏽 releases, improving the performance of the following commands:

* `stats`: by refactoring the code to detect empty cells more efficiently, and by removing
unnecessary bounds checks in the main compute loop. (~10% performance improvement)
* `sample`: by refactoring the code to use an index more effectively when available - not only making it faster, but also eliminating the need to load the entire dataset into memory. Also added a `--faster` option to use a faster random number generator. (~15% performance improvement)
* `frequency`, `schema`, `search` & `validate` by amortizing/reducing allocations in hot loops
* `excel`: by refactoring the main hot loop to convert Excel cells more efficiently

The prebuilt binaries are also built with CPU optimizations enabled for x86_64 and Apple Silicon (arm64) architectures.

0.112.0 is also a "Carousel" (i.e. increased usability) 🎠 release featuring new Jupyter notebooks in the `contrib/notebooks` directory to help users get started with qsv.

* [intro-to-count.ipynb](https://github.com/jqnatividad/qsv/blob/master/contrib/notebooks/intro-to-count.ipynb) by @rzmk
* [qsv-describegpt-qa.ipynb](https://github.com/jqnatividad/qsv/blob/master/contrib/notebooks/qsv-describegpt-qa.ipynb) by @a5dur

---

### Added
* `sqlp`: added `CASE` expression support with Polars 0.32 https://github.com/jqnatividad/qsv/commit/9d508e69cc4165b7adbe4b44b15c4c07001cf76b
* `sample`: added `--faster` option to use a faster random number generator https://github.com/jqnatividad/qsv/pull/1210
* `jsonl`: add `--delimiter` option https://github.com/jqnatividad/qsv/pull/1205
* `excel`: add `--delimiter` option https://github.com/jqnatividad/qsv/commit/ab73067da1f498c7c64de9b87586d6998d36d042
* `notebook/describegpt`: added describegpt QA Jupyter notebook by @a5dur in https://github.com/jqnatividad/qsv/pull/1215
* `notebook/count`: add intro-to-count.ipynb by @rzmk in https://github.com/jqnatividad/qsv/pull/1207

### Changed
* `stats`: refactor hot compute function - https://github.com/jqnatividad/qsv/commit/35999c5dad996edcafe6094ff4b717f96d657832
* `stats`: faster detection of empty samples https://github.com/jqnatividad/qsv/commit/b0548159ca8c8a35bab1dd196c72414f739c2fd8 and https://github.com/jqnatividad/qsv/commit/a7f0836bcebf947efb3cc7e7f6a884cc649196b5
* `sample`: major refactor making it faster, but also eliminating need to load the entire dataset into memory when an index is available. https://github.com/jqnatividad/qsv/pull/1210
* `frequency`: refactor primary ftables function https://github.com/jqnatividad/qsv/commit/57d660d6cf48be4b8845b5c09a46b16582f612c0
* `fmt`: match_block_trailing_comma https://github.com/jqnatividad/qsv/pull/1206
* bump MSRV to 1.71.1 https://github.com/jqnatividad/qsv/commit/1c993644992d1cf4d0985d100045821cb027c17d
* apply clippy suggestions https://github.com/jqnatividad/qsv/pull/1209
* build(deps): bump tokio from 1.29.1 to 1.30.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1204
* build(deps): bump log from 0.4.19 to 0.4.20 by @dependabot in https://github.com/jqnatividad/qsv/pull/1211
* build(deps): bump redis from 0.23.1 to 0.23.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1213
* build(deps): bump tokio from 1.30.0 to 1.31.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1212
* build(deps): bump sysinfo from 0.29.7 to 0.29.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1214
* upgrade to Polars 0.32.0 https://github.com/jqnatividad/qsv/pull/1217
* build(deps): bump flate2 from 1.0.26 to 1.0.27 by @dependabot in https://github.com/jqnatividad/qsv/pull/1218
* build(deps): bump polars from 0.32.0 to 0.32.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1219
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-08-13

### Removed
* `stats`: removed Debug derives from structs - https://github.com/jqnatividad/qsv/commit/2def136230ed2e9af727168d3a6329d660b65d4d

### Fixed
* `notebook/count`: fix Google Colab link by @rzmk in https://github.com/jqnatividad/qsv/pull/1208

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.111.0...0.112.0

## [0.111.0] - 2023-08-07
This is the first in a series of "[Giddy-up](https://7esl.com/giddy-up/)" 🏇🏽 releases. 

As Quicksilver matures, we will continue to tweak it in our goal to be the 🚀 fastest general purpose CSV data-wrangling CLI toolkit available.

"Giddy-up" 🏇🏽 releases will do this by:
* taking advantage of new [Rust features as they become available](https://releases.rs/)
* using new libraries that are faster than the ones we currently use
* optimizing our code to take advantage of new features in the libraries we use
* using new algorithms that are faster than the ones we currently use
* taking advantage of more hardware features (SIMD, multi-core, etc.)
* adding reproducible benchmarks that are automatically updated on release to track our progress

As it is, Quicksilver has an aggressive release tempo - with more than 160 releases since its initial release in December 2020.  This was made possible by the solid foundation of Rust and the [xsv](https://github.com/BurntSushi/xsv) project from which qsv was forked.  We will continue to build on this foundation by adding more CI tests and starting to track code coverage so we can continue to iterate aggressively with confidence.

Apart from "giddy-up" releases, Quicksilver will also have "carousel" 🎠 releases that will focus on making the toolkit more accessible to non-technical users.

"Carousel" 🎠 releases will include:
* more documentation
* more examples
* more tutorials
* more recipes in the Cookbook
* multiple GUI wrappers around the CLI
* integrations with common desktop tools like Excel, Google Sheets, Open Office, etc.

Hopefully, this will make qsv more accessible to non-technical users, and help them get more value out of their data.

Every now and then, we'll also have "Unicorn" 🦄 releases that will add MAJOR new features to the toolkit (e.g. 10x type features like the integration of [Pola.rs](https://pola.rs) into qsv).

We will also add a new Technical Documentation section to the [wiki](https://github.com/jqnatividad/qsv/wiki) to document qsv's architecture and how each command works.  The hope is doing so will [lower the barrier to contributions](https://github.com/jqnatividad/qsv/blob/master/CONTRIBUTING.md) and help us grow the community of qsv contributors.

### Added
* `sort`: add --faster option https://github.com/jqnatividad/qsv/pull/1190
* `describegpt`: add -Q, --quiet option by @rzmk in https://github.com/jqnatividad/qsv/pull/1179

### Changed
* `stats`: refactor init_date_inference https://github.com/jqnatividad/qsv/pull/1187
* `join`: cache has_headers result in hot loop https://github.com/jqnatividad/qsv/commit/e53edafdc91493c61e9889c8004177f147483a45
* `search` &  `searchset`: amortize allocs https://github.com/jqnatividad/qsv/pull/1188
* `stats`: use `fast-float` to convert string to float https://github.com/jqnatividad/qsv/pull/1191
* `sqlp`: more examples, apply clippy::needless_borrow lint https://github.com/jqnatividad/qsv/commit/ff37a041da246101db03c51d22b498127a5d7ba7 and https://github.com/jqnatividad/qsv/commit/b8e1f7784cc6906745cdd43b61194e897a3666c4
* use `fast-float` project-wide (`apply`, `applydp`, `schema`, `sort`, `validate`) https://github.com/jqnatividad/qsv/pull/1192
* fine tune publishing workflows to enable universally available CPU features https://github.com/jqnatividad/qsv/commit/a1dccc74b480477acaa17e21dde706c159c56b48
* build(deps): bump serde from 1.0.179 to 1.0.180 by @dependabot in https://github.com/jqnatividad/qsv/pull/1176
* build(deps): bump pyo3 from 0.19.1 to 0.19.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1177
* build(deps): bump qsv-dateparser from 0.9.0 to 0.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1178
* build(deps): bump qsv-sniffer from 0.9.4 to 0.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1180
* build(deps): bump indicatif from 0.17.5 to 0.17.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1182
* Bump to qsv stats 0.11  https://github.com/jqnatividad/qsv/pull/1184
* build(deps): bump serde from 1.0.180 to 1.0.181 by @dependabot in https://github.com/jqnatividad/qsv/pull/1185
* build(deps): bump qsv_docopt from 1.3.0 to 1.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1186
* build(deps): bump filetime from 0.2.21 to 0.2.22 by @dependabot in https://github.com/jqnatividad/qsv/pull/1193
* build(deps): bump regex from 1.9.1 to 1.9.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1194
* build(deps): bump regex from 1.9.2 to 1.9.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1195
* build(deps): bump serde from 1.0.181 to 1.0.182 by @dependabot in https://github.com/jqnatividad/qsv/pull/1196
* build(deps): bump tempfile from 3.7.0 to 3.7.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1199
* build(deps): bump strum_macros from 0.25.1 to 0.25.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1200
* build(deps): bump serde from 1.0.182 to 1.0.183 by @dependabot in https://github.com/jqnatividad/qsv/pull/1201
* cargo update bump several indirect dependencies
* apply select clippy lint suggestions
* pin Rust nightly to 2023-08-07

### Removed
* temporarily remove rand/simd_support feature when building nightly as its causing the nightly build to fail https://github.com/jqnatividad/qsv/commit/0a66fdb454941052857f6458df38abe7730e0b4b

### Fixed
* fixed typos from documentation by @a5dur in https://github.com/jqnatividad/qsv/pull/1203

## New Contributors
* @a5dur made their first contribution in https://github.com/jqnatividad/qsv/pull/1203

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.110.0...0.111.0

## [0.110.0] - 2023-07-30
### Added
* `describegpt`: Add jsonl to prompt file doc section & more clarification by @rzmk in https://github.com/jqnatividad/qsv/pull/1149
* `luau`: add `--no-jit` option  https://github.com/jqnatividad/qsv/pull/1170
* `sqlp`: add CTE examples https://github.com/jqnatividad/qsv/commit/33f0218c6a78b9cef15e9bed6e227e5f17ef747a

### Changed
* `frequency`: minor optimizations https://github.com/jqnatividad/qsv/commit/ecac0be5777a50cef2bfe7937d80c5ffe071e4cd
* `join`: performance optimizations https://github.com/jqnatividad/qsv/commit/4cb593783efc4e7c2026d632b8dc741cc2edc778 and https://github.com/jqnatividad/qsv/commit/4cb593783efc4e7c2026d632b8dc741cc2edc778
* `sqlp`: reduce allocs in loop https://github.com/jqnatividad/qsv/commit/ae164b570c300845e75ce0fac3272221bdebfa66
* Apple Silicon build now uses mimalloc allocator by default https://github.com/jqnatividad/qsv/commit/bfab24aba2d3b3f70f08ea407572d20feeda725d
* build(deps): bump jql-runner from 7.0.1 to 7.0.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1151
* build(deps): bump serde from 1.0.171 to 1.0.173 by @dependabot in https://github.com/jqnatividad/qsv/pull/1154
* build(deps): bump tempfile from 3.6.0 to 3.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1155
* build(deps): bump serde from 1.0.174 to 1.0.175 by @dependabot in https://github.com/jqnatividad/qsv/pull/1157
* build(deps): bump redis from 0.23.0 to 0.23.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1164
* build(deps): bump serde from 1.0.175 to 1.0.177 by @dependabot in https://github.com/jqnatividad/qsv/pull/1163
* build(deps): bump serde_json from 1.0.103 to 1.0.104 by @dependabot in https://github.com/jqnatividad/qsv/pull/1160
* build(deps): bump grex from 1.4.1 to 1.4.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1159
* build(deps): bump sysinfo from 0.29.6 to 0.29.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1158
* build(deps): bump mlua from 0.9.0-rc.1 to 0.9.0-rc.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1169
* build(deps): bump flexi_logger from 0.25.5 to 0.25.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1168
* build(deps): bump jemallocator from 0.5.0 to 0.5.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1167
* build(deps): bump serde from 1.0.177 to 1.0.178 by @dependabot in https://github.com/jqnatividad/qsv/pull/1166
* build(deps): bump rust_decimal from 1.30.0 to 1.31.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1172
* build(deps): bump csvs_convert from 0.8.6 to 0.8.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1174
* apply `clippy:needless_pass_by_ref_mut` lint in `select` and `frequency` https://github.com/jqnatividad/qsv/commit/ba6566e5ea73a1042d33c02035ed1736947b60d8 and https://github.com/jqnatividad/qsv/commit/83add7b30c6e32a49b412629acf60c4c7057df37
* cargo update bump indirect dependencies
* pin Rust nightly to 2023-07-29

### Removed
* `excel`: remove defunct dates-whitelist comments https://github.com/jqnatividad/qsv/commit/2a24d2dcd23c2ccd24dfef1055bf265085f10146

### Fixed
* `join`: fix left-semi join. Fixes #1150. https://github.com/jqnatividad/qsv/pull/1153
* `foreach`: fix command argument token splitter pattern. Fixes #1171 https://github.com/jqnatividad/qsv/pull/1173


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.109.0...0.110.0

## [0.109.0] - 2023-07-17
This is a monstrous👹 release with lots of new features and improvements!

The biggest new feature is the `describegpt` command which allows you to use OpenAI's Large Language Models to generate extended metadata from a CSV. We created this command primarily for [CKAN](https://ckan.org) and [Datapusher+](https://github.com/dathere/datapusher-plus#datapusher) so we can infer descriptions, tags and to automatically created annotated data dictionaries using the CSV's summary statistics and frequency tables. In that way, it works even for very large CSV files without consuming too many Open AI tokens.
This is a very powerful feature and we are looking forward to seeing what people do with it. Thanks @rzmk for all the work on this!

This release also features major improvements in the `sqlp` and `joinp` commands thanks to all the new capabilities of [Polars 0.31.1](https://github.com/pola-rs/polars/releases/tag/rs-0.31.1). 

Polars SQL's capabilities have been vastly improved in 0.31.1 with numerous new SQL functions and operators, and they're all available using the `sqlp` command.

The `joinp` command has several new options for CSV parsing, for pre-join filtering (`--filter-left` and `--filter-right`), and pre-join validation with the `--validate` option. Two new asof join variants (`--left_by` and `right_by`) were also added.

### Added
* `describegpt` command by @rzmk in https://github.com/jqnatividad/qsv/pull/1036
* `describegpt`: minor refactoring in https://github.com/jqnatividad/qsv/pull/1104
* `describegpt`: `--key` & QSV_OPENAI_API_KEY by @rzmk in https://github.com/jqnatividad/qsv/pull/1105
* `describegpt`: add `--user-agent` in help message by @rzmk in https://github.com/jqnatividad/qsv/pull/1095
* `describegpt`: json output format for redirection by @rzmk in https://github.com/jqnatividad/qsv/pull/1107
* `describegpt`: add testing (resolves #1114) by @rzmk in https://github.com/jqnatividad/qsv/pull/1115
* `describegpt`: add `--model` option (resolves #1101) by @rzmk in https://github.com/jqnatividad/qsv/pull/1117
* `describegpt`: polishing https://github.com/jqnatividad/qsv/pull/1122
* `describegpt`: add `--jsonl` option (resolves #1086) by @rzmk in https://github.com/jqnatividad/qsv/pull/1127
* `describegpt`: add `--prompt-file` option (resolves #1085) by @rzmk in https://github.com/jqnatividad/qsv/pull/1120
* `joinp`: added  `asof_by` join variant; added CSV formatting options consistent with sqlp CSV format options https://github.com/jqnatividad/qsv/pull/1090
* `joinp`: add `--filter-left` and `--filter-right` options https://github.com/jqnatividad/qsv/pull/1146
* `joinp`: add `--validate` option https://github.com/jqnatividad/qsv/pull/1147
* `fetch` & `fetchpost`: add `--no-cache` option https://github.com/jqnatividad/qsv/pull/1112
* `sniff`: detect file kind along with mime type https://github.com/jqnatividad/qsv/pull/1137
* user-agent metadata now contains the current command's name https://github.com/jqnatividad/qsv/pull/1093

### Changed
* `fetch` & `fetchpost`: --redis and --no-cache are mutually exclusive https://github.com/jqnatividad/qsv/pull/1113
* `luau`: adapt to mlua 0.9.0-rc.1 API changes https://github.com/jqnatividad/qsv/pull/1129
* upgrade to Polars 0.31.1 https://github.com/jqnatividad/qsv/pull/1139
* Bump MSRV to latest Rust stable (1.71.0)
* pin Rust nightly to 2023-07-15
* Bump uuid from 1.3.4 to 1.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1073
* Bump tokio from 1.28.2 to 1.29.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1077
* Bump tokio from 1.29.0 to 1.29.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1087
* Bump sysinfo from 0.29.2 to 0.29.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1088
* build(deps): bump sysinfo from 0.29.4 to 0.29.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1148
* Bump jql-runner from 6.0.9 to 7.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1092
* build(deps): bump jql-runner from 7.0.0 to 7.0.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1132
* Bump itoa from 1.0.6 to 1.0.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/1091
* Bump itoa from 1.0.7 to 1.0.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/1098
* build(deps): bump itoa from 1.0.8 to 1.0.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1142
* Bump serde from 1.0.164 to 1.0.165 by @dependabot in https://github.com/jqnatividad/qsv/pull/1094
* Bump serde from 1.0.165 to 1.0.166 by @dependabot in https://github.com/jqnatividad/qsv/pull/1100
* Bump serde from 1.0.166 to 1.0.167 by @dependabot in https://github.com/jqnatividad/qsv/pull/1116
* build(deps): bump serde from 1.0.167 to 1.0.171 by @dependabot in https://github.com/jqnatividad/qsv/pull/1118
* Bump pyo3 from 0.19.0 to 0.19.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1099
* Bump ryu from 1.0.13 to 1.0.14 by @dependabot in https://github.com/jqnatividad/qsv/pull/1096
* build(deps): bump ryu from 1.0.14 to 1.0.15 by @dependabot in https://github.com/jqnatividad/qsv/pull/1144
* Bump strum_macros from 0.25.0 to 0.25.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1097
* Bump serde_json from 1.0.99 to 1.0.100 by @dependabot in https://github.com/jqnatividad/qsv/pull/1103
* build(deps): bump serde_json from 1.0.100 to 1.0.101 by @dependabot in https://github.com/jqnatividad/qsv/pull/1123
* build(deps): bump serde_json from 1.0.101 to 1.0.102 by @dependabot in https://github.com/jqnatividad/qsv/pull/1125
* build(deps): bump serde_json from 1.0.102 to 1.0.103 by @dependabot in https://github.com/jqnatividad/qsv/pull/1143
* Bump serde_stacker from 0.1.8 to 0.1.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1110
* Bump regex from 1.8.4 to 1.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1109
* build(deps): bump regex from 1.9.0 to 1.9.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1119
* Bump jsonschema from 0.17.0 to 0.17.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1108
* build(deps): bump cpc from 1.9.1 to 1.9.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1121
* build(deps): bump governor from 0.5.1 to 0.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1128
* build(deps): bump actions/setup-python from 4.6.1 to 4.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1134
* build(deps): bump file-format from 0.17.3 to 0.18.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1136
* build(deps): bump serde_stacker from 0.1.9 to 0.1.10 by @dependabot in https://github.com/jqnatividad/qsv/pull/1141
* build(deps): bump semver from 1.0.17 to 1.0.18 by @dependabot in https://github.com/jqnatividad/qsv/pull/1140
* cargo update bump several indirect dependencies

### Fixed
* `fmt`: Quote ASCII format differently by @LemmingAvalanche in https://github.com/jqnatividad/qsv/pull/1075
* `apply`: make `dynfmt` subcommand case sensitive. Fixes #1126 https://github.com/jqnatividad/qsv/pull/1130
* `applydp`: make `dynfmt` case-sensitive  https://github.com/jqnatividad/qsv/pull/1131
* `describegpt`: docs/Describegpt.md: typo 'a' --> 'an' by @rzmk in https://github.com/jqnatividad/qsv/pull/1135
* `tojsonl`: support snappy-compressed input. Fixes #1133 https://github.com/jqnatividad/qsv/pull/1145
* security.md: fix mailto text by @rzmk in https://github.com/jqnatividad/qsv/pull/1079

## New Contributors
* @LemmingAvalanche made their first contribution in https://github.com/jqnatividad/qsv/pull/1075

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.108.0...0.109.0

## [0.108.0] - 2023-06-25
Another big Quicksilver release with lots of new features and improvements!

The two [Polars](https://www.pola.rs)-powered commands - `joinp` and `sqlp` - have received significant attention. `joinp` now supports [asof joins](https://pola-rs.github.io/polars/py-polars/html/reference/dataframe/api/polars.DataFrame.join_asof.html) and the `--try-parsedates` option. `sqlp` now has several Parquet format options, along with a `--low-memory` option.

Other new features include:

* A new `cat rowskey --group` option that [emulates csvkit's `csvstack` command](https://github.com/jqnatividad/qsv/discussions/1053).
* SIMD-accelerated UTF-8 validation for the `input` command.
* A `--field-separator` option for the `flatten` command.
* The `sniff` command now uses the excellent [`file-format`](https://github.com/mmalecot/file-format#file-format) crate for mime-type detection on __ALL__ platforms, not just Linux, as was the case when we were using the libmagic library.

Also, QuickSilver now has optimized builds for Apple Silicon. These builds are created using native Apple Silicon self-hosted Action Runners, which means we can enable all qsv features without being constrained by cross-compilation limitations and GitHub’s Action Runner’s disk/memory constraints. Additionally, we compile Apple Silicon builds with M1/M2 chip optimizations enabled to maximize performance.

Finally, qsv startup should be noticeably faster, thanks to @vi’s [PR to avoid sysinfo::System::new_all](https://github.com/jqnatividad/qsv/pull/1064).

### Added
* `joinp`: added asof join & --try-parsedates option https://github.com/jqnatividad/qsv/pull/1059
* `cat`: emulate csvkit's csvstack https://github.com/jqnatividad/qsv/pull/1067
* `input`: SIMD-accelerated utf8 validation https://github.com/jqnatividad/qsv/commit/88e1df2757b4a9a6f9dbaf55a99b87fc15b18a65
* `sniff`: replace magic with file-format crate, enabling mime-type detection on all platforms https://github.com/jqnatividad/qsv/pull/1069
* `sqlp`: add --low-memory option https://github.com/jqnatividad/qsv/commit/d95048e7be1a9d34cc7a22feebbd792a5c27c604
* `sqlp`: added parquet format options https://github.com/jqnatividad/qsv/commit/c179cf49e02343138b058d02783332394029a050 https://github.com/jqnatividad/qsv/commit/a861ebf246d22db0f4bcbce1b76788413cfdd1e7
* `flatten`: add --field-separator option https://github.com/jqnatividad/qsv/pull/1068
* Apple Silicon binaries built on native Apple Silicon self-hosted Action Runners, enabling all features and optimized for M1/M2 chips

### Changed
* `input`: minor improvements https://github.com/jqnatividad/qsv/commit/62cff74b4679e2ba207916392cab5de573ce0a59
* `joinp`: align option names with `join` command https://github.com/jqnatividad/qsv/pull/1058
* `sqlp`: minor improvements
* changed all GitHub action workflows to account for the new Apple Silicon builds
* Bump rust_decimal from 1.29.1 to 1.30.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1049
* Bump serde_json from 1.0.96 to 1.0.97 by @dependabot in https://github.com/jqnatividad/qsv/pull/1051
* Bump calamine from 0.21.0 to 0.21.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1052
* Bump strum from 0.24.1 to 0.25.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1055
* Bump actix-governor from 0.4.0 to 0.4.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1060
* Bump csvs_convert from 0.8.5 to 0.8.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1061
* Bump itertools from 0.10.5 to 0.11.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1062
* Bump serde_json from 1.0.97 to 1.0.99 by @dependabot in https://github.com/jqnatividad/qsv/pull/1065
* Bump indexmap from 1.9.3 to 2.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1066
* Bump calamine from 0.21.1 to 0.21.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1071
* cargo update bump various indirect dependencies
* pin Rust nightly to 2021-06-23

### Fixed
* Avoid sysinfo::System::new_all by @vi in https://github.com/jqnatividad/qsv/pull/1064
* correct typos project-wide https://github.com/jqnatividad/qsv/pull/1072

### Removed
* removed libmagic dependency from all GitHub action workflows

## New Contributors
* @vi made their first contribution in https://github.com/jqnatividad/qsv/pull/1064

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.107.0...0.108.0

## [0.107.0] - 2023-06-14
We continue to improve the new [`sqlp`](https://github.com/jqnatividad/qsv/blob/master/src/cmd/sqlp.rs#L2) command. It now supports scripts, Polars CSV parsing and CSV format options. We also added a new special value for the `rename` command which allows you to rename all columns in a CSV. This was done to make it easier to prepare CSVs with no headers for use with `sqlp`.

This release also features a Windows MSI installer. This is a big step forward for qsv and we hope to make it easier for Windows users to install and use qsv. Thanks @minhajuddin2510 for all the work on pulling this together!

### Added
* `sqlp`: added script support https://github.com/jqnatividad/qsv/pull/1037
* `sqlp`: added CSV format options https://github.com/jqnatividad/qsv/pull/1048
* `rename`: add `"_all generic"` special value for headers https://github.com/jqnatividad/qsv/pull/1031

### Changed
* `excel`: now supports Duration type with calamine upgrade to 0.21.0 https://github.com/jqnatividad/qsv/pull/1045
* Update publish-wix-installer.yml by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/1032
* Bump mlua from 0.9.0-beta.2 to 0.9.0-beta.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/1030
* Bump serde from 1.0.163 to 1.0.164 by @dependabot in https://github.com/jqnatividad/qsv/pull/1029
* Bump csvs_convert from 0.8.4 to 0.8.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1028
* Bump sysinfo from 0.29.1 to 0.29.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1027
* Bump log from 0.4.18 to 0.4.19 by @dependabot in https://github.com/jqnatividad/qsv/pull/1039
* Bump uuid from 1.3.3 to 1.3.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1041
* Bump jql-runner from 6.0.8 to 6.0.9 by @dependabot in https://github.com/jqnatividad/qsv/pull/1043
* cargo update bump several indirect dependencies
* pin Rust nightly to 2021-06-13

### Fixed
* Remove redundant registries protocol by @icp1994 in https://github.com/jqnatividad/qsv/pull/1034
* fix typo in tojsonl.rs (optionns -> options) by @rzmk in https://github.com/jqnatividad/qsv/pull/1035
* Fix eula by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/1046

## New Contributors
* @icp1994 made their first contribution in https://github.com/jqnatividad/qsv/pull/1034
* @rzmk made their first contribution in https://github.com/jqnatividad/qsv/pull/1035

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.106.0...0.107.0

## [0.106.0] - 2023-06-07
This release features the new [Polars](https://www.pola.rs/)-powered `sqlp` command which allows you to run SQL queries against CSVs.

Initial tests show that its competitive with [DuckDB](https://duckdb.org/) and faster than [DataFusion](https://arrow.apache.org/datafusion/) on identical SQL queries, and it just runs rings around [pandasql](https://github.com/yhat/pandasql/#pandasql).

It converts Polars SQL (a subset of ANSI SQL) queries to multithreaded LazyFrames expressions and then executes them. This is a very powerful feature and allows you to do things like joins, aggregations, group bys, etc. on larger than memory CSVs. The `sqlp` command is still experimental and we are looking for feedback on it. Please try it out and let us know what you think.

### Added
* `sqlp`: new command to allow Polars SQL queries against CSVs https://github.com/jqnatividad/qsv/pull/1015

### Changed
* Bump csv from 1.2.1 to 1.2.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1008
* Bump pyo3 from 0.18.3 to 0.19.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1007
* workflow for creating msi for qsv by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/1009
* migrate from once_cell to std::sync::oncelock https://github.com/jqnatividad/qsv/pull/1010
* Bump qsv_docopt from 1.2.2 to 1.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1011
* Bump self_update from 0.36.0 to 0.37.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1014
* Bump indicatif from 0.17.4 to 0.17.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/1013
* Bump cached from 0.43.0 to 0.44.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1012
* Bump url from 2.3.1 to 2.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1016
* Wix changes by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/1017
* Bump actions/github-script from 5 to 6 by @dependabot in https://github.com/jqnatividad/qsv/pull/1018
* Bump regex from 1.8.3 to 1.8.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1019
* Bump hashbrown from 0.13.2 to 0.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1020
* Bump tempfile from 3.5.0 to 3.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1021
* Bump sysinfo from 0.29.0 to 0.29.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/1023
* Bump qsv-dateparser from 0.8.2 to 0.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/1022
* Bump qsv-sniffer from 0.9.3 to 0.9.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1024
* Bump qsv-stats from 0.9.0 to 0.10.0 https://github.com/jqnatividad/qsv/commit/38035793d2bb3bf4bee1d3e4cbfc62a6f0235fb6
* Bump embedded luau from 0.577 to 0.579
* Bump data-encoding from 2.3.3 to 2.4.0 https://github.com/jqnatividad/qsv/commit/2285a12eab6a7997f97cb39f908684c3adae3ec9
* cargo update bump several indirect dependencies
* change MSRV to 1.70.0
* pin Rust nightly to 2023-06-06

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.105.1...0.106.0

## [0.105.1] - 2023-05-30

### Changed
* `stats`: remove all unsafes https://github.com/jqnatividad/qsv/commit/4a4c0107f98dcd3a2fac7a793101624ec46762df
* `fetch` & `fetchpost`: remove unsafe https://github.com/jqnatividad/qsv/commit/1826bb3cbe24f731973d2e2ce8edc1927dc87d4b
* `validate`: remove unsafe https://github.com/jqnatividad/qsv/commit/742ccb3b36fd6a0fb9690d9150bec5b2e4d44b0a
* normalize `--user-agent` option across all of qsv https://github.com/jqnatividad/qsv/commit/feff90bba4d6840f7d2aa2100897cfaad7efe08f & https://github.com/jqnatividad/qsv/commit/feff90bba4d6840f7d2aa2100897cfaad7efe08f
* bump qsv-dateparser from 0.8.1 to 0.8.2 which also uses chrono 0.4.26
* pin sventaro/upload-release-action to v2.5 as v2.6 was overwriting release body text https://github.com/jqnatividad/qsv/commit/4e6bb702d2de7457b559bc6dad69b4071f745289
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-05-29

### Fixed
* remove chrono pin to 0.4.24 and upgrade to 0.4.26 which fixed 0.4.25 CI test failures https://github.com/jqnatividad/qsv/commit/7636d82bdcf3428e59b800b6ff9f53dcd52cddd9

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.105.0...0.105.1

## [0.105.0] - 2023-05-30

### Added
* `sniff`: add --harvest-mode convenience option https://github.com/jqnatividad/qsv/pull/997
* `sniff`: added --quick option on Linux https://github.com/jqnatividad/qsv/commit/e16df6fbbad9318cc4efeb500409f80b76cd50e2
* qsv (pronounced "Quicksilver") now has a tagline - [_"Hi ho, QuickSilver! Away!"_](https://www.youtube.com/watch?v=p9lf76xOA5k) :smile: https://github.com/jqnatividad/qsv/commit/d32aeb1afe7a90c4887b00a0c2a20481a91722fe

### Changed
* `sniff`: if --no-infer is enabled when sniffing a snappy file, just return the snappy mime type https://github.com/jqnatividad/qsv/pull/996
* `sniff`: now returns filesize and last-modified date in errors. https://github.com/jqnatividad/qsv/commit/2162659bd574122e93e204cb14b5114bd7ca5344
* `stats`: minor performance tweaks in hot compute loop https://github.com/jqnatividad/qsv/commit/f61198c2057545fb76a9b30bd12adfd3a3bbf8ba
* qsv binary variants built using older glibc/musl libraries are now published with their respective glibc/musl version suffixes (glibc-2.31/musl-1.1.24) in the filename, instead of just the "older" suffix.
* pin chrono to 0.4.24 as the new 0.4.25 is breaking CI tests https://github.com/jqnatividad/qsv/commit/cde3623b27fcb583a1248fc736aaf11f569f5085
* Bump calamine from 0.19.1 to 0.20.0 https://github.com/jqnatividad/qsv/commit/ec7e2df70e33756d4ef49567bf4f5acba3eb19d4
* Bump actions/setup-python from 4.6.0 to 4.6.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/991
* Bump flexi_logger from 0.25.4 to 0.25.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/992
* Bump regex from 1.8.2 to 1.8.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/993
* Bump csvs_convert from 0.8.3 to 0.8.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/994
* Bump log from 0.4.17 to 0.4.18 by @dependabot in https://github.com/jqnatividad/qsv/pull/998
* Bump polars from 0.29.0 to 0.30.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/999
* Bump tokio from 1.28.1 to 1.28.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1000
* Bump once_cell from 1.17.1 to 1.17.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/1003
* Bump indicatif from 0.17.3 to 0.17.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/1001
* cargo bump update several indirect dependencies
* pin Rust nightly to 2023-05-28

### Removed
* `excel`: removed kludgy --dates-whitelist option https://github.com/jqnatividad/qsv/pull/1005

### Fixed
* `sniff`: fix inconsistent mime type detection https://github.com/jqnatividad/qsv/pull/995

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.104.1...0.105.0

## [0.104.1] - 2023-05-23

### Added
* added new publishing workflow to build binary variants using older glibc 2.31 instead of glibc 2.35 and musl 1.1.24 instead of musl 1.2.2. This will allow users running on older Linux distros (e.g. Debian, Ubuntu 20.04) to run qsv prebuilt binaries with  "older" glibc/musl versions. https://github.com/jqnatividad/qsv/commit/1a08b920240b39ff57282645cc92686b42e3c278

### Changed
* `sniff`: improved usage text https://github.com/jqnatividad/qsv/commit/d2b32ac6631589230484cb84506b5113c8f75192
* `sniff`: if sniffing a URL, and server does not return content-length or last-modified headers, set filesize and last-modified to "Unknown" https://github.com/jqnatividad/qsv/commit/d4a64ac2e7147e7ab5452864fe6063a97f37f76b
* `frequency`: use simdutf8 validation in hot loop https://github.com/jqnatividad/qsv/commit/33406a15f554d03ca117e0196efa6362f104e3cc
* `foreach`: use simdut8 validation https://github.com/jqnatividad/qsv/commit/df6b4f8ae967bde8ca22bc6dd217938ae5238add
* `apply`: tweak decode operation to avoid panics (however unlikely) https://github.com/jqnatividad/qsv/commit/adf7052db39a08aeda2401774892a884be98223c
* update install & build instructions with magic
* Bump regex from 1.8.1 to 1.8.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/990
* Bump bumpalo from 3.12.2 to 3.13.0
* pin Rust nightly to 2021-05-22

### Removed
* `sniff`: disabled --progressbar option on qsvdp binary variant.

### Fixed
* updated publishing workflows to properly enable magic feature (for sniff mime type detection) https://github.com/jqnatividad/qsv/commit/136211fcd9134f3421223979a5272ff53d77f03b

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.104.0...0.104.1

## [0.104.0] - 2023-05-22

### Added
* `sniff`: add --no-infer option only available on Linux. Using this option makes `sniff` work acts as a general mime type detector, retrieving detected mime type, file size (content-length when sniffing a URL), and last modified date.   
When sniffing a URL with --no-infer, it only sniff's the first downloaded chunk, making it very fast even for very large remote files. This option was designed to facilitate accelerated harvesting and broken/stale link checking on CKAN. https://github.com/jqnatividad/qsv/pull/987
* `excel`: add canonical_filename to metadata https://github.com/jqnatividad/qsv/pull/985
* `snappy`: now accepts url input https://github.com/jqnatividad/qsv/pull/986
* `sample`: support url input https://github.com/jqnatividad/qsv/pull/989

### Changed
* Bump qsv-sniffer from 0.9.2 to 0.9.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/979
* Bump console from 0.15.5 to 0.15.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/980
* Bump jql-runner from 6.0.7 to 6.0.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/981
* Bump console from 0.15.6 to 0.15.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/988
* Bump embedded Luau from 0.576 to 0.577
* apply select clippy recommendations
* tweaked emojis used in Available Commands legend - 🗜️ to 🤯 to denote memory-intensive commands that load the entire CSV into memory; 🪗 to 😣 to denote commands that need addl memory proportional to the cardinality of the columns being processed; 🌐 to denote commands that have web-aware options
* cargo update bump several indirect dependencies
* pin Rust nightly to 2021-05-21

### Fixed
* `excel`: Handle ranges larger than the sheet by @bluepython508 in https://github.com/jqnatividad/qsv/pull/984

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.103.1...0.104.0

## [0.103.1] - 2023-05-17

### Changed
* Bump reqwest from 0.11.17 to 0.11.18 by @dependabot in https://github.com/jqnatividad/qsv/pull/978
* cargo update bump indirect dependencies

### Fixed
* fix `cargo install` failing as it is trying to fetch cargo environment variables that are only set for `cargo build`, but not `cargo install` #977 

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.103.0...0.103.1

## [0.103.0] - 2023-05-15

### Added
* `sniff`: On Linux, short-circuit sniffing a remote file when we already know its not a CSV https://github.com/jqnatividad/qsv/pull/976
* `stats`: now computes variance for dates https://github.com/jqnatividad/qsv/commit/e3e678298de59f2485d5e70f622218d849a2e2c9
* `stats`: now automatically invalidates cached stats across qsv releases https://github.com/jqnatividad/qsv/commit/6e929dd1feac692be3f7e1883ad88f99b3abc5b2
* add magic version to --version option https://github.com/jqnatividad/qsv/commit/455c0f26e237c812bf9d88d6a7906e34c5a9cbeb
* added CKAN-aware (![CKAN](https://github.com/jqnatividad/qsv/blob/master/docs/images/ckan.png?raw=true)) legend to List of Available Commands

### Changed
* `stats`: improve usage text
* `stats`: use extend_from_slice for readability https://github.com/jqnatividad/qsv/commit/23275e2e8ef30bdc101293084bce71e651b3222a
* `validate`: do not panic if the input is not UTF-8 https://github.com/jqnatividad/qsv/commit/532cd012de0866250be2dc19b6e02ffa27b3c9fb
* `sniff`: simplify getting stdin last_modified property; return detected mime type in JSON error response https://github.com/jqnatividad/qsv/commit/01975912ae99fe0a7b38cf741f3dfbcf2b9dc486
* `luau`: update embedded Luau from 0.573 to 0.576
* Update nightly build instructions
* Bump qsv-sniffer from 0.9.1 to 0.9.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/972
* Bump tokio from 1.28.0 to 1.28.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/973
* Bump serde from 1.0.162 to 1.0.163 by @dependabot in https://github.com/jqnatividad/qsv/pull/974
* cargo update bump several indirect dependencies
* pin Rust nightly to 2021-05-13

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.102.1...0.103.0

## [0.102.1] - 2023-05-09
0.102.1 is a small patch release to fix issues in publishing the pre-built binary variants with magic for `sniff` when cross-compiling.

### Changed
* `stats`: refine `--infer-boolean` option info & update test count https://github.com/jqnatividad/qsv/commit/de6390b21a21b67ae0dd3f3f6d0153f2c0736cff
* `tojsonl`: refine boolcheck_first_lower_char() fn https://github.com/jqnatividad/qsv/commit/241115e4718c67cd8e701c435b91e02556875eac

### Fixed
* tweaked GitHub Actions publishing workflows to enable building magic-enabled `sniff` on Linux. Disabled magic when cross-compiling for non-x86_64 Linux targets.

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.102.0...0.102.1

## [0.102.0] - 2023-05-08
A lot of work was done on `sniff` to make it not just a CSV dialect detector, but a general purpose file type detector leveraging :magic_wand: [magic](https://www.darwinsys.com/file/) :sparkles: - able to detect mime types even for files on URLs.  

`sniff` can now also use the same data types as `stats` with the `--stats-types` option. This was primarily done to support metadata collection when registering CKAN resources not only during data entry, but also when checking resource links for bitrot, and when harvesting metadata from other systems, so `stats` & `sniff` can be used interchangeably based on the response time requirement and the data quality of the data source.

For example, `sniff` can be used for quickly inferring metadata by just downloading a small sample from a very large data file DURING data entry (["Resource-first upload workflow"](https://github.com/dathere/datapusher-plus/blob/master/docs/RESOURCE_FIRST_WORKFLOW.md#Resource-first-Upload-Workflow)), with `stats` being used later on, when the data is actually being pushed to the Datastore with [Datapusher+](https://github.com/dathere/datapusher-plus#datapusher), when data type inferences need to be guaranteed, and the entire file will need to be scanned.

### Added
* `stats`: add `--infer-boolean` option https://github.com/jqnatividad/qsv/pull/967
* `sniff`: add `--stats-types` option https://github.com/jqnatividad/qsv/pull/968
* `sniff`: add magic mime-type detection on Linux https://github.com/jqnatividad/qsv/pull/970
* `sniff`: add `--user-agent` option https://github.com/jqnatividad/qsv/commit/bd0bf788609c7dd5220cdab6061067170acf1ca2
* `sniff`: add last_modified info https://github.com/jqnatividad/qsv/commit/ef68bff177ee7c9ce6bd45868488287c8114a91e

### Changed
* make `--envlist` option allocator-aware https://github.com/jqnatividad/qsv/commit/f3566dc0c4ab7c7236374cce936f5db7200e39de
* Bump serde from 1.0.160 to 1.0.162 by @dependabot in https://github.com/jqnatividad/qsv/pull/962
* Bump robinraju/release-downloader from 1.7 to 1.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/960
* Bump flexi_logger from 0.25.3 to 0.25.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/965
* Bump sysinfo from 0.28.4 to 0.29.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/966
* Bump jql-runner from 6.0.6 to 6.0.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/969
* Bump polars from 0.28.0 to 0.29.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/971
* apply select clippy recommendations
* cargo update bump indirect dependencies
* change MSRV to 1.69.0
* pin Rust nightly to 2023-05-07

### Fixed
* `sniff`: make sniff give more consistent results https://github.com/jqnatividad/qsv/pull/958. Fixes #956
* Bump qsv-sniffer from 0.8.3 to 0.9.1. Replaced all assert with proper error-handling. https://github.com/jqnatividad/qsv/pull/961 https://github.com/jqnatividad/qsv/commit/a7c607a55be9bebca13148f5a0dddf1fea909df7 https://github.com/jqnatividad/qsv/commit/43d7eaf9201c72016682096e84400dba59b7cd95 
* `sniff`: fixed rowcount calculation when sniffing a URL and the entire file was actually downloaded - https://github.com/jqnatividad/qsv/commit/ef68bff177ee7c9ce6bd45868488287c8114a91e


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.101.0...0.102.0

## [0.101.0] - 2023-05-01
We're back to the future! The qsv release train is back on track, as we jump to 0.101.0 over the yanked
0.100.0 release now that self-update logic has been fixed.

### Added
* `stats`: added more metadata to stats arg cache json - https://github.com/jqnatividad/qsv/commit/5767e5650690a8f39d537ccdc428a6688762cd77
* added target-triple to user-agent string, and changed agent name to qsv binary variant https://github.com/jqnatividad/qsv/commit/063b08031e361b5c1f26ed504870f0bc1bfd7678, https://github.com/jqnatividad/qsv/commit/70f4ea3b2d0d88b54358c470dd8e964e89adf16d

### Changed
* `excel`: performance, safety & documentation refinements https://github.com/jqnatividad/qsv/commit/e9a283d51fe84cc4c4e004c0e7b9b2ef12db683d, https://github.com/jqnatividad/qsv/commit/3800d250223619963bc9072ade9c43200ca1bdaf, https://github.com/jqnatividad/qsv/commit/252b01e2207bb995d09154af546a12174d532d6a, https://github.com/jqnatividad/qsv/commit/6a6df0f045cb4f1e58d07433e73a41579ca1262f, https://github.com/jqnatividad/qsv/commit/6a6df0f045cb4f1e58d07433e73a41579ca1262f, https://github.com/jqnatividad/qsv/commit/67ccd85cbe5441b1ad0188ae524b3e832c817d30, https://github.com/jqnatividad/qsv/commit/f2908ce020316087ed756d614c357373727f2664, https://github.com/jqnatividad/qsv/commit/6d5105deaa00f3b8e350d522b196ef4ed3676fc4, https://github.com/jqnatividad/qsv/commit/dbcea393cfba08b4ffe3b6b6d0acd364a59cb342, https://github.com/jqnatividad/qsv/commit/faa8ef9b3f9d6de6af47ddced0d80a5ad5b4e763
* `replace`: clarify that it works on a field-by-field basis https://github.com/jqnatividad/qsv/commit/c0e2012dc011a6269359ed0ff2c7dc157bae5cd0
* `stats`: use extend_from_slice when possible - https://github.com/jqnatividad/qsv/commit/c71ad4ee3d7992f4ef1cdc37e32d740756340ba9
* `fetch` & `fetchpost`: replace multiple push_fields with a csv from vec - https://github.com/jqnatividad/qsv/commit/f4e0479e508c845f49d320967af443fe5a247327
* `fetch` & `fetchpost`: Migrate to jql 6 https://github.com/jqnatividad/qsv/pull/955
* `schema`: made bincode reader buffer bigger - https://github.com/jqnatividad/qsv/commit/39b4bb5f89bab7ada2dda40d66d1e40bb51cbe0a
* `index`: * use increased default buffer size when creating index https://github.com/jqnatividad/qsv/commit/60fe7d64b7eeb322625d2cc44d196bd5633bd79c
* standard  ized user_agent processing https://github.com/jqnatividad/qsv/commit/4c063015a8d664b9ef105243b2ea6541b3cc6b59, https://github.com/jqnatividad/qsv/commit/010c565912c6ae5ba09620cee7f90aeb294c4d14
* User agent environment variable; standardized user agent processing https://github.com/jqnatividad/qsv/pull/951
* more robust Environment Variables processing https://github.com/jqnatividad/qsv/pull/946
* move Environment Variables to its own markdown file https://github.com/jqnatividad/qsv/commit/77c167fe3942ce464bc5a675b76b3371cf75e84b
* Bump tokio from 1.27.0 to 1.28.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/945
* Bump mimalloc from 0.1.36 to 0.1.37 by @dependabot in https://github.com/jqnatividad/qsv/pull/944
* Bump mlua from 0.9.0-beta.1 to 0.9.0-beta.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/952
* Bump flate2 from 1.0.25 to 1.0.26 by @dependabot in https://github.com/jqnatividad/qsv/pull/954
* Bump reqwest from 0.11.16 to 0.11.17 by @dependabot in https://github.com/jqnatividad/qsv/pull/953
* cargo update bump indirect dependencies
* pin Rust nightly to 2023-04-30

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.99.1...0.101.0

## [0.99.1] - 2023-04-24
Even though this is a patch release, it actually contains a lot of new features and improvements.
This was done so that qsv version 0.99.0 and below can upgrade to this release, as the self-update logic
in older versions compared versions as strings, and not as semvers, preventing the older versions from
updating as the yanked 0.100.0 is less than anything 0.99.0 and below when compared as strings.

The changelog below is a combination of the changelog of the yanked 0.100.0 and the changes since 0.99.0.

### Added
* `snappy`: add validate subcommand https://github.com/jqnatividad/qsv/pull/920
* `sniff`: can now sniff snappy-compressed files - on the local file system and on URLs https://github.com/jqnatividad/qsv/pull/925
* `schema` & `stats`: stats now has a `--stats-binout` option which `schema` takes advantage of https://github.com/jqnatividad/qsv/pull/931
* `schema`: added NYC 311 JSON schema validation file generated by `qsv schema` https://github.com/jqnatividad/qsv/commit/c956212574ad0d800c3cf3bb1caa4e5722f0a393
* `to`: added snappy auto-compression/decompression support https://github.com/jqnatividad/qsv/commit/09a7afd38fdf59703edf76fa492eed9747586b6c
* `to`: added dirs as input source https://github.com/jqnatividad/qsv/commit/a31fb3b7499e1ed05136b32b3179d5713bec2106 and https://github.com/jqnatividad/qsv/commit/4d4dd548c44967c61493f1e1c2403f352dcfba34
* `to`: added unit tests for sqlite, postgres, xslx and datapackage https://github.com/jqnatividad/qsv/commit/16f2b7ec35bc44093b90d4673e8c20a61f6263bb https://github.com/jqnatividad/qsv/commit/808b018d1f5b7f815897979e1bd67d663fe31c9c https://github.com/jqnatividad/qsv/commit/10739c55bdf66494e5f76028fb1bc67dbeb706cf
* add dotenv file support https://github.com/jqnatividad/qsv/pull/936 and https://github.com/jqnatividad/qsv/pull/937


### Changed
* `stats` & `schema`: major performance improvement (30x faster) with stats binary format serialization/deserialization https://github.com/jqnatividad/qsv/commit/73b4b2075a7d9013f8b71a9109073e6d9b8ad9b4
* `snappy`: misc improvements in https://github.com/jqnatividad/qsv/pull/921
* `stats`: Refine stats binary format caching in https://github.com/jqnatividad/qsv/pull/932
* bump embedded Luau from [0.5.71](https://github.com/Roblox/luau/releases/tag/0.571) to [0.5.73](https://github.com/Roblox/luau/releases/tag/0.573) https://github.com/jqnatividad/qsv/commit/d0ea7c8f926299c5d201609e4f3f11e11e3462d7
* Better OOM checks. It now has two distinct modes - NORMAL and CONSERVATIVE, with NORMAL being the default. Previously, it was using the CONSERVATIVE heuristic and it was causing too many false positives https://github.com/jqnatividad/qsv/pull/935
* Bump actions/setup-python from 4.5.0 to 4.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/934
* Bump emdedded Luau from 0.5.67 to 0.5.71 https://github.com/jqnatividad/qsv/commit/a67bd3e274b1f73d64bb93e03c817cce583a8b02
* Bump qsv-stats from 0.7 to 0.8 https://github.com/jqnatividad/qsv/commit/9a6812abff719b11e5b0c7e25009dfc81231757a
* Bump serde from 1.0.159 to 1.0.160 by @dependabot in https://github.com/jqnatividad/qsv/pull/918
* Bump cached from 0.42.0 to 0.43.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/919
* Bump serde_json from 1.0.95 to 1.0.96 by @dependabot in https://github.com/jqnatividad/qsv/pull/922
* Bump pyo3 from 0.18.2 to 0.18.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/923
* Bump ext-sort from 0.1.3 to 0.1.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/929
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-04-23

### Removed
* `snappy` is even snappier when we removed 8-cpu cap for even faster compression - going from 1.75 gb/sec to 2.25 gb/sec for the NYC 311 test data :rocket: https://github.com/jqnatividad/qsv/commit/19acf2f23187dee5fd104e9e6eceb8fdc74d7a08

### Fixed
* `excel`: Float serialization correctness by @bluepython508 in https://github.com/jqnatividad/qsv/pull/933
* `luau`: only create qsv_cache directory when needed https://github.com/jqnatividad/qsv/pull/930
* `luau`: make `qsv_shellcmd()` helper function work with Windows https://github.com/jqnatividad/qsv/commit/f867158c4c7eaf10c18092b2a4c88ff67cc3a487 and https://github.com/jqnatividad/qsv/commit/cc24acba3c916184059e7e9d776dce9e35294d44
* Self update semver parsing fixed so versions are compared as semvers, not as strings. This prevented self-update from updating from 0.99.0 to 0.100.0 as 0.99.0 > 0.100.0 when compared as string. https://github.com/jqnatividad/qsv/pull/940
* fixed werr macro to also format! messages https://github.com/jqnatividad/qsv/commit/c3ceaf713683ddb70e40a293f494f15144cc78fb

## New Contributors
* @bluepython508 made their first contribution in https://github.com/jqnatividad/qsv/pull/933

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.99.0...0.99.1

## [0.100.0] - 2023-04-18 (YANKED)
0.100.0 was yanked as it was comparing versions as strings instead of semver.
So "0.100.0" is less than "0.99.0", and self-update won't work.

### Added
* `snappy`: add validate subcommand https://github.com/jqnatividad/qsv/pull/920
* `sniff`: can now sniff snappy-compressed files - on the local file system and on URLs https://github.com/jqnatividad/qsv/pull/925
* `schema` & `stats`: stats now has a `--stats-binout` option which `schema` takes advantage of https://github.com/jqnatividad/qsv/pull/931
* `schema`: added NYC 311 JSON schema validation file generated by `qsv schema` https://github.com/jqnatividad/qsv/commit/c956212574ad0d800c3cf3bb1caa4e5722f0a393
* `to`: added snappy auto-compression/decompression support https://github.com/jqnatividad/qsv/commit/09a7afd38fdf59703edf76fa492eed9747586b6c
* `to`: added dirs as input source https://github.com/jqnatividad/qsv/commit/a31fb3b7499e1ed05136b32b3179d5713bec2106 and https://github.com/jqnatividad/qsv/commit/4d4dd548c44967c61493f1e1c2403f352dcfba34
* `to`: added unit tests for sqlite, postgres, xslx and datapackage https://github.com/jqnatividad/qsv/commit/16f2b7ec35bc44093b90d4673e8c20a61f6263bb https://github.com/jqnatividad/qsv/commit/808b018d1f5b7f815897979e1bd67d663fe31c9c https://github.com/jqnatividad/qsv/commit/10739c55bdf66494e5f76028fb1bc67dbeb706cf

### Changed
* `snappy`: misc improvements in https://github.com/jqnatividad/qsv/pull/921
* `stats`: Refine stats binary format caching in https://github.com/jqnatividad/qsv/pull/932
* Bump emdedded Luau from 0.5.67 to 0.5.71 https://github.com/jqnatividad/qsv/commit/a67bd3e274b1f73d64bb93e03c817cce583a8b02
* Bump qsv-stats from 0.7 to 0.8 https://github.com/jqnatividad/qsv/commit/9a6812abff719b11e5b0c7e25009dfc81231757a
* Bump serde from 1.0.159 to 1.0.160 by @dependabot in https://github.com/jqnatividad/qsv/pull/918
* Bump cached from 0.42.0 to 0.43.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/919
* Bump serde_json from 1.0.95 to 1.0.96 by @dependabot in https://github.com/jqnatividad/qsv/pull/922
* Bump pyo3 from 0.18.2 to 0.18.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/923
* Bump ext-sort from 0.1.3 to 0.1.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/929
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-04-17

### Removed
* `snappy` is even snappier when we removed 8-cpu cap for even faster compression - going from 1.75 gb/sec to 2.25 gb/sec for the NYC 311 test data :rocket: https://github.com/jqnatividad/qsv/commit/19acf2f23187dee5fd104e9e6eceb8fdc74d7a08

### Fixed
* only create qsv_cache directory when needed https://github.com/jqnatividad/qsv/pull/930
* fixed werr macro to also formmat! messages https://github.com/jqnatividad/qsv/commit/c3ceaf713683ddb70e40a293f494f15144cc78fb

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.99.0...0.100.0

## [0.99.0] - 2023-04-10
### Added
* added [Snappy](https://google.github.io/snappy/) auto-compression/decompression support. The Snappy format was chosen primarily
because it supported streaming compression/decompression and is designed for performance. https://github.com/jqnatividad/qsv/pull/911
* added `snappy` command. Although files ending with the ".sz" extension are automatically compressed/decompressed, the `snappy` command offers 4-5x faster multithreaded compression. It can also be used to check if a file is Snappy-compressed or not, and can be used to compress/decompress any file. https://github.com/jqnatividad/qsv/pull/911 and https://github.com/jqnatividad/qsv/pull/916
* `diff` command added to `qsvlite` and `qsvdp` binary variants https://github.com/jqnatividad/qsv/pull/910
* `to`: added stdin support https://github.com/jqnatividad/qsv/pull/913

### Changed
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-04-09

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.98.0...0.99.0

## [0.98.0] - 2023-04-07

### Added
* `stats`: added stats caching and storing the [computed stats as metadata](https://github.com/jqnatividad/qsv/issues/713). Doing so not only prevents unnecessary recomputation of stats, especially for very large files, it also sets the foundation for summary statistics to be used more widely across qsv to support new commands that leverages these stats - e.g. [`fixdata`](https://github.com/jqnatividad/qsv/issues/613), [`outliers`](https://github.com/jqnatividad/qsv/issues/107), [`describegpt`](https://github.com/jqnatividad/qsv/issues/896), [`fake`](https://github.com/jqnatividad/qsv/issues/235), [`statsviz`](https://github.com/jqnatividad/qsv/issues/302) and [multi-pass stats](https://github.com/jqnatividad/qsv/issues/895), etc. https://github.com/jqnatividad/qsv/pull/902
* `stats`: added `--force` option to force recomputation of stats https://github.com/jqnatividad/qsv/commit/2f91d0cd981ce9be6c36424cd946f3bcce42b909
* `luau`: add qsv_loadcsv helper function https://github.com/jqnatividad/qsv/pull/908
* added more info about regular expression syntax and link to https://regex101.com which now supports the Rust flavor of regex

### Changed
* logging is now buffered by default https://github.com/jqnatividad/qsv/pull/903
* renamed features to be more easily understandable: "full" -> "feature_capable", "all_full" -> "all_features" https://github.com/jqnatividad/qsv/pull/906
* changed GitHub Actions workflows to use the new feature names
* Bump redis from 0.22.3 to 0.23.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/901
* Bump filetime from 0.2.20 to 0.2.21 by @dependabot in https://github.com/jqnatividad/qsv/pull/904
* reenabled `fetch` and `fetchpost` CI tests
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-04-06

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.97.0...0.98.0

## [0.97.0] - 2023-04-04

Since 0.96.x was not published, 0.97.0 contains the changes from 0.96.x after fixing the mimalloc build errors on non-Windows platforms.

### Added
* `excel`: add --date-format option in https://github.com/jqnatividad/qsv/pull/897 and https://github.com/jqnatividad/qsv/commit/6a7db997c8d150854405a2cb2ac392479c3534b9
* `luau`: add qsv_fileexists() helper fn https://github.com/jqnatividad/qsv/commit/f4cc60f87c3c7c85a7736260356daa3051d2a879

### Changed
* `excel`: speed up float conversion by using ryu and itoa together rather than going thru core::fmt::Formatter https://github.com/jqnatividad/qsv/commit/e722753c377e385ebdffca199557ab3cf848ce7b
* `joinp`: --cross option does not require columns; added CI tests https://github.com/jqnatividad/qsv/pull/894
* `schema`: better, more human-readable regex patterns are generated when inferring pattern attribute; more interactive messages https://github.com/jqnatividad/qsv/commit/1620477b752e64b6b2844aafeee4adf9256d4de8
* `schema` & `validate`: improve usage text; added JSON Schema Validation info https://github.com/jqnatividad/qsv/commit/3da68474d0fa4b6ec2170bf69dbfb27ab0d5f8a3
* Bump tokio from 1.26.0 to 1.27.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/887
* Bump reqwest from 0.11.15 to 0.11.16 by @dependabot in https://github.com/jqnatividad/qsv/pull/888
* Bump serde_json from 1.0.94 to 1.0.95 by @dependabot in https://github.com/jqnatividad/qsv/pull/889
* Bump serde from 1.0.158 to 1.0.159 by @dependabot in https://github.com/jqnatividad/qsv/pull/890
* Bump tempfile from 3.4.0 to 3.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/891
* Bump polars from 0.27.2 to 0.28.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/893
* Bump mimalloc from 0.1.34 to 0.1.35 by @dependabot in https://github.com/jqnatividad/qsv/pull/899
* Bump mlua from 0.8 to 0.9.0-beta.1 https://github.com/jqnatividad/qsv/commit/9b7e984cba4079f8e826f7e74209a90ce7856bc7
* bump MSRV to Rust 1.68.2
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-04-02

### Removed
* `luau`: removed unnecessary --exec option https://github.com/jqnatividad/qsv/commit/0d4ccdaab95ab5471bb71d99aa7f9056dabf48c3

### Fixed
* Fixed build errors on non-Windows platforms #900 by bumping mimalloc from 0.1.34 to 0.1.36

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.95.1...0.97.0

## [0.96.1] - 2023-04-03 [NOT PUBLISHED]

### Fixed
* bump mimalloc down from 0.1.35 to 0.1.34 due to build errors on non-Windows platforms

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.96.0...0.96.1

## [0.96.0] - 2023-04-03 [NOT PUBLISHED]

### Added
* `excel`: add --date-format option in https://github.com/jqnatividad/qsv/pull/897 and https://github.com/jqnatividad/qsv/commit/6a7db997c8d150854405a2cb2ac392479c3534b9
* `luau`: add qsv_fileexists() helper fn https://github.com/jqnatividad/qsv/commit/f4cc60f87c3c7c85a7736260356daa3051d2a879

### Changed
* `excel`: speed up float conversion by using ryu and itoa together rather than going thru core::fmt::Formatter https://github.com/jqnatividad/qsv/commit/e722753c377e385ebdffca199557ab3cf848ce7b
* `joinp`: --cross option does not require columns; added CI tests https://github.com/jqnatividad/qsv/pull/894
* `schema`: better, more human-readable regex patterns are generated when inferring pattern attribute; more interactive messages https://github.com/jqnatividad/qsv/commit/1620477b752e64b6b2844aafeee4adf9256d4de8
* `schema` & `validate`: improve usage text; added JSON Schema Validation info https://github.com/jqnatividad/qsv/commit/3da68474d0fa4b6ec2170bf69dbfb27ab0d5f8a3
* Bump tokio from 1.26.0 to 1.27.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/887
* Bump reqwest from 0.11.15 to 0.11.16 by @dependabot in https://github.com/jqnatividad/qsv/pull/888
* Bump serde_json from 1.0.94 to 1.0.95 by @dependabot in https://github.com/jqnatividad/qsv/pull/889
* Bump serde from 1.0.158 to 1.0.159 by @dependabot in https://github.com/jqnatividad/qsv/pull/890
* Bump tempfile from 3.4.0 to 3.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/891
* Bump polars from 0.27.2 to 0.28.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/893
* Bump mimalloc from 0.1.34 to 0.1.35 by @dependabot in https://github.com/jqnatividad/qsv/pull/899
* Bump mlua from 0.8 to 0.9.0-beta.1 https://github.com/jqnatividad/qsv/commit/9b7e984cba4079f8e826f7e74209a90ce7856bc7
* bump MSRV to Rust 1.68.2
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-04-02

### Removed
* `luau`: removed unnecessary --exec option https://github.com/jqnatividad/qsv/commit/0d4ccdaab95ab5471bb71d99aa7f9056dabf48c3

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.95.1...0.96.0

## [0.95.1] - 2023-03-27

### Changed
* `count`: add example/test add link from usage text https://github.com/jqnatividad/qsv/commit/9cd3c293eef0344c27693949f415850881211adf
* `diff`: add examples link from usage text https://github.com/jqnatividad/qsv/commit/4250811d0d20284342ccd7efcc58cd7562d16636
* Standardize --timeout option handling and exposed it with QSV_TIMEOUT env var https://github.com/jqnatividad/qsv/pull/886
* improved self-update messages https://github.com/jqnatividad/qsv/commit/4027306f08aeca3b2ebe1e4243628a65c1307a9e
* Bump qsv-dateparser from 0.6 to 0.7
* Bump qsv-sniffer from 0.7 to 0.8
* Bump actions/stale from 7 to 8 by @dependabot in https://github.com/jqnatividad/qsv/pull/876
* Bump newline-converter from 0.2.2 to 0.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/877
* Bump rust_decimal from 1.29.0 to 1.29.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/882
* Bump regex from 1.7.2 to 1.7.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/881
* Bump sysinfo from 0.28.3 to 0.28.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/883
* Bump pyo3 from 0.18.1 to 0.18.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/885
* Bump indexmap from 1.9.2 to 1.9.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/884
* change MSRV to Rust 1.68.1
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-03-26

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.95.0...0.95.1

## [0.95.0] - 2023-03-23

### Added
* `luau`: added qsv_cmd() and qsv_shellcmd() helpers, detailed map error messages to help with script development https://github.com/jqnatividad/qsv/pull/869
* `luau`: added environment variable set/get helper functions - qsv_setenv() and qsv_getenv() https://github.com/jqnatividad/qsv/pull/872
* `luau`: added smart qsv_register_lookup() caching so lookup tables need not be repeatedly downloaded and can be persisted/expired as required https://github.com/jqnatividad/qsv/pull/874
* `luau`: added QSV_CKAN_API, QSV_CKAN_TOKEN and QSV_CACHE_DIR env vars https://github.com/jqnatividad/qsv/commit/9b7269e98fe004c6d2268d626777628af65dd45d

### Changed
* `apply` & `applydp`: expanded usage text to have arguments section; emptyreplace subcommand now supports column selectors https://github.com/jqnatividad/qsv/pull/868
* `luau`: smarter script file processing. In addition to recognizing "file:" prefix, if the script argument ends with ".lua/luau" file extensions, its automatically processed as a file https://github.com/jqnatividad/qsv/pull/875
* `luau`: qsv_sleep() and qsv_writefile() improvements https://github.com/jqnatividad/qsv/commit/27358a26411f95f57acfd62aad8b92906fe82ced
* `partition`: added arguments section to usage text; added NYC 311 example https://github.com/jqnatividad/qsv/commit/74aa37b1c138f1c010d338fb4f6c9b48a381532a
* Bump reqwest from 0.11.14 to 0.11.15 by @dependabot in https://github.com/jqnatividad/qsv/pull/870
* Bump regex from 1.7.1 to 1.7.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/873
* apply select clippy lint recommendations
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-03-22

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.94.0...0.95.0

## [0.94.0] - 2023-03-17

### Added
* `luau`: qsv_register_lookup now supports "ckan://" scheme. This allows the luau script developer to fetch lookup table resources from CKAN instances. https://github.com/jqnatividad/qsv/pull/864
* `luau`: added detailed example for "dathere://" lookup scheme in https://github.com/dathere/qsv-lookup-tables repo. https://github.com/jqnatividad/qsv/commit/3074538a9ac1071ba6d6b6e85fdc0ca3c833ce4e
* `luau`:  added `qsv_writefile` helper function. This allows the luau script developer to write text files to the current working directory. Filenames are sanitized for safety.  https://github.com/jqnatividad/qsv/pull/867
* `luau`: random access mode now supports progressbars. The progressbar indicates the current record and the total number of records in the CSV file https://github.com/jqnatividad/qsv/commit/63150a0a0d885f5bd5b118524d802ff59b18f621
* `input`: added  --comment option which allows the user to specify the comment character.
CSV rows that start with the comment character are skipped. https://github.com/jqnatividad/qsv/pull/866


### Changed
* `luau`: added additional logging messages to help with script debugging https://github.com/jqnatividad/qsv/commit/bcff8adc03ad398829f4874e948f5152bca04783
* `schema` & `tojsonl`: refactor stdin handling https://github.com/jqnatividad/qsv/commit/6c923b19bfa3fbed918335b70b793a6d6011a960
* bump jsonschema from 0.16 to 0.17
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-03-17

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.93.1...0.94.0

## [0.93.1] - 2023-03-15

### Fixed
* Fixed publishing workflow so qsvdp `luau` is only enabled on platforms that support it

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.93.0...0.93.1

## [0.93.0] - 2023-03-15

### Added
* `luau`: qsv_register_lookup helper function now works with CSVs on URLs https://github.com/jqnatividad/qsv/pull/860
* `luau`: added support for "dathere://" lookup scheme, allowing users to conveniently load oft-used lookup tables from https://github.com/dathere/qsv-lookup-tables https://github.com/jqnatividad/qsv/pull/861
* `luau`: added detailed API definitions for Luau Helper Functions https://github.com/jqnatividad/qsv/blob/605b38b5636382d45f96d3d9d3c404bb20efaf15/src/cmd/luau.rs#L1156-L1497
* `validate`: added --timeout option when downloading JSON Schemas https://github.com/jqnatividad/qsv/commit/605b38b5636382d45f96d3d9d3c404bb20efaf15

### Changed
* remove all glob imports https://github.com/jqnatividad/qsv/pull/857 and https://github.com/jqnatividad/qsv/pull/858
* qsvdp ([Datapusher+](https://github.com/dathere/datapusher-plus#datapusher)-optimized qsv binary variant) now has an embedded `luau` interpreter https://github.com/jqnatividad/qsv/pull/859
* `validate`: JSON Schema url now case-insensitive https://github.com/jqnatividad/qsv/commit/3123dc6da30370cae88c9e4bb9d387fed3d36507
* Bump serde from 1.0.155 to 1.0.156 by @dependabot in https://github.com/jqnatividad/qsv/pull/862
* applied select clippy lint recommendations
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-03-14

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.92.0...0.93.0

## [0.92.0] - 2023-03-13

### Added
* `excel`: added option to specify range to extract by @EricSoroos in https://github.com/jqnatividad/qsv/pull/843
* `luau`: added --remap option. This allows the user to only map specified columns to the output CSV https://github.com/jqnatividad/qsv/pull/841
* `luau`: added several new helper functions:
  * `qsv_skip`: skips writing the current record to the output CSV https://github.com/jqnatividad/qsv/pull/854
  * `qsv_break`: stops processing the current CSV file https://github.com/jqnatividad/qsv/pull/846
  * `qsv_insertrecord`: inserts a new record to the output CSV https://github.com/jqnatividad/qsv/pull/845
  * `qsv_register_lookup`: loads a CSV that can be used as a lookup table in Luau https://github.com/jqnatividad/qsv/commit/38e7b7eb264d4b43b7f3039696ad918238f0a4c6

### Changed
* `luau`: reorganized code for readability/maintainability https://github.com/jqnatividad/qsv/pull/846
* `foreach`: tweak usage text to say it works with shell commands, not just the bash shell https://github.com/jqnatividad/qsv/commit/78851b33e8482c1961e97c17c95ea316950355fd
* `split`: added deeplink to examples/tests https://github.com/jqnatividad/qsv/commit/6f293b853b74505b7769e2972e7bc358506db34e
* `select`: added deeplink to examples/tests https://github.com/jqnatividad/qsv/commit/72fa0942c5954b48236b6d137a8347e89e2f097c
* Switch to qsv-optimized fork of docopt.rs - [qsv_docopt](https://github.com/jqnatividad/docopt.rs#qsv_docopt). As [docopt.rs](https://github.com/docopt/docopt.rs) is unmaintained and docopt parsing is an integral part of qsv as we embed each command's usage text in a way that cannot be done by either [clap](http://docs.rs/clap/) or [structopt](http://docs.rs/structopt/) https://github.com/jqnatividad/qsv/pull/852
* Bump embedded Luau from [0.566](https://github.com/Roblox/luau/releases/tag/0.566) to [0.567](https://github.com/Roblox/luau/releases/tag/0.567) https://github.com/jqnatividad/qsv/commit/d624e840802b51aae59cf5db0923f8f2605426c5
* Bump csv from 1.2.0 to 1.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/839
* Bump serde from 1.0.152 to 1.0.153 by @dependabot in https://github.com/jqnatividad/qsv/pull/842
* Bump serde from 1.0.153 to 1.0.154 by @dependabot in https://github.com/jqnatividad/qsv/pull/844
* Bump rust_decimal from 1.28.1 to 1.29.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/853
* start using new crates.io sparse protocol
* applied select clippy lint recommendations
* cargo update bump several other dependencies
* pin Rust nightly to 2021-03-12

### Fixed
* `stats`: fix stdin regression https://github.com/jqnatividad/qsv/pull/851
* `excel`: Fix missing integer headers in excel transform. by @EricSoroos in https://github.com/jqnatividad/qsv/pull/840
* `luau`: fix & improve comment remover https://github.com/jqnatividad/qsv/pull/845


## New Contributors
* @EricSoroos made their first contribution in https://github.com/jqnatividad/qsv/pull/840

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.91.0...0.92.0

## [0.91.0] - 2023-03-05

### Added
* `luau`: map multiple new computed columns in one call https://github.com/jqnatividad/qsv/pull/829
* `luau`: added `qsv_autoindex()` helper function https://github.com/jqnatividad/qsv/pull/834
* `luau`: added `qsv_coalesce()` helper function https://github.com/jqnatividad/qsv/commit/3064ba2116ce5c96f3bd7e789616a3b0ffe9f63b
* `luau`: added `_LASTROW` special variable to facilitate random access mode

### Changed
* `diff`: rename --primary-key-idx -> --key by @janriemer in https://github.com/jqnatividad/qsv/pull/826
* `diff`: implement option to sort by columns by @janriemer in https://github.com/jqnatividad/qsv/pull/827
* `luau`: parsing improvements https://github.com/jqnatividad/qsv/pull/835
* `luau`: bump embedded luau version from 0.562 to 0.566 https://github.com/jqnatividad/qsv/commit/f4a08b4980201015dcba31dfae74d8b1045c0455
* `sniff`: major refactoring. https://github.com/jqnatividad/qsv/pull/836
* enable polars nightly feature when building nightly https://github.com/jqnatividad/qsv/pull/816
* bump qsv-sniffer from 0.6.1 to 0.7.0 https://github.com/jqnatividad/qsv/commit/5027a64576f19792f917550f9146792d5c9e351a
* Bump crossbeam-channel from 0.5.6 to 0.5.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/818
* Bump flexi_logger from 0.25.1 to 0.25.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/824
* Bump rayon from 1.6.1 to 1.7.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/831
* Bump ryu from 1.0.12 to 1.0.13 by @dependabot in https://github.com/jqnatividad/qsv/pull/830
* Bump itoa from 1.0.5 to 1.0.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/832
* cargo update bump dependencies
* pin Rust nightly to 2023-03-04

### Fixed
* `stats`: use utf8-aware truncate https://github.com/jqnatividad/qsv/pull/819
* `sniff`: fix URL sniffing https://github.com/jqnatividad/qsv/commit/8d2c514fa2a173be626b5c36dbfb70d60335b81e
* show polars version in `qsv --version` https://github.com/jqnatividad/qsv/commit/586a1ed987fa2efbfbc233bd82f84a52fa4c3859

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.90.1...0.91.0

## [0.90.1] - 2023-02-28

### Changed
* `joinp`: Refactor to use LazyFrames instead of DataFrames for performance and ability to do streaming. https://github.com/jqnatividad/qsv/pull/814 and https://github.com/jqnatividad/qsv/pull/815
* `luau`: expanded example using `qsv_log` helper https://github.com/jqnatividad/qsv/commit/5c198e4bcb243005dace25d8aecbc58bb211cadc
* handled new clippy lints https://github.com/jqnatividad/qsv/commit/e81a391bd675a2f4fb07169c1d6848340104b9fe
* adjust publishing workflows to build binaries with as many features enabled. On some platforms, the `to` and `polars`(for `joinp`) features cannot be built. 
* cargo update bump indirect dependencies, notably arrow and duckdb
* pin Rust nightly to 2023-02-27

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.90.0...0.90.1

## [0.90.0] - 2023-02-27

### Added
* `joinp`:  new join command powered by [Pola.rs](https://pola.rs). This is just the first of more commands that will leverage the Pola.rs engine. https://github.com/jqnatividad/qsv/pull/798
* `luau`: added random access mode; major refactor as we prepare to use `luau` as qsv's [DSL](https://en.wikipedia.org/wiki/Domain-specific_language); added `qsv_log` helper that can be called from Luau scripts to facilitate development of [full-fledged data-wrangling scripts](https://github.com/jqnatividad/qsv/blob/9cad3396a8f56d2c2136c843078d5635324539a5/tests/test_luau.rs#L224-L247).  https://github.com/jqnatividad/qsv/pull/805 and https://github.com/jqnatividad/qsv/pull/806
* `sniff`: added URL & re-enabled stdin support; URL support features sampling only the required number of rows to sniff the metadata without downloading the entire file; expanded sniff metadata returned; added `--progressbar` option for URL sniffing https://github.com/jqnatividad/qsv/pull/812
* `sniff`: added `--timeout` option for URL inputs; now runs async from all the binary variants  https://github.com/jqnatividad/qsv/pull/813

### Changed
* `diff`: sort by line when no other sort option is given by @janriemer in https://github.com/jqnatividad/qsv/pull/808
* `luau`: rename `--prologue`/`--epilogue` options to `--begin`/`--end`; add  embedded BEGIN/END block handling https://github.com/jqnatividad/qsv/pull/801
* Update to csvs_convert 0.8 by @kindly in https://github.com/jqnatividad/qsv/pull/800
* use simdutf8 when possible https://github.com/jqnatividad/qsv/commit/ae466cbffbc924cc5c1cc09509dd963c56dfc259
* Bump self_update from 0.35.0 to 0.36.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/797
* Bump sysinfo from 0.28.0 to 0.28.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/809
* Bump actix-web from 4.3.0 to 4.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/811
* improved conditional compilation of different variants https://github.com/jqnatividad/qsv/commit/9e636946504a09a1edeea4b0533d42a0bb658b7f
* temporarily skip CI tests that use httpbin.org as it was causing intermittent failures https://github.com/jqnatividad/qsv/commit/bee160228794c26326baf569e5e7239206ae4314
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-02-26

### Removed
* Python 3.6 support removed https://github.com/jqnatividad/qsv/commit/86b29d487261fda7670072bfd5977dd9508ac0aa

### Fixed
* `sniff`: does not work with stdin which fixes #803; https://github.com/jqnatividad/qsv/pull/807   
Note that stdin support was shortly re-enabled in https://github.com/jqnatividad/qsv/pull/812  

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.89.0...0.90.0

## [0.89.0] - 2023-02-20

### Added
* `cat`: added new `rowskey` subcommand. Unlike the existing `rows` subcommand, it allows far more flexible concatenation of CSV files by row, even if the files have different number of columns and column order. https://github.com/jqnatividad/qsv/pull/795
* added jemalloc support. As the current default mimalloc allocator is not supported in some platforms. Also, for certain workloads, jemalloc may be faster. See [Memory Allocator](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#memory-allocator) for more info https://github.com/jqnatividad/qsv/pull/796
* added `--no-memcheck` and related `QSV_NO_MEMORY_CHECK` env var. This relaxes the conservative Out-of-Memory prevention heuristic of qsv. See [Memory Management](https://github.com/jqnatividad/qsv#memory-management) for more info https://github.com/jqnatividad/qsv/pull/792

### Changed
* `--version` now returns max input file size when running in "non-streaming" mode, and detailed memory info. See [Version details](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#version-details) for more info https://github.com/jqnatividad/qsv/pull/780
* `exclude`: expanded usage text and added 'input parameters' help by @tmtmtmtm in https://github.com/jqnatividad/qsv/pull/783
* `stats`: performance tweaks in https://github.com/jqnatividad/qsv/commit/96e8168e6064469ab4489ed19c36aa595d5d119d, https://github.com/jqnatividad/qsv/commit/634d42a646dfb3bed2d34842bb3fa484cf641c7e and https://github.com/jqnatividad/qsv/commit/7e148cf78753aa60ef60f8efd6f1c7fea246b703
* Use [simdutf8](https://github.com/rusticstuff/simdutf8#simdutf8--high-speed-utf-8-validation) to do SIMD accelerated utf8 validation, replacing problematic utf8 screening. Together with https://github.com/jqnatividad/qsv/pull/782, completes utf8 validation revamp. https://github.com/jqnatividad/qsv/pull/784
* Bump sysinfo from 0.27.7 to 0.28.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/786
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-02-18

### Removed
* Removed patched versions of csv crate optimized for performance, using Rust 2021 edition. With the release of csv 1.2,switched back to csv crate upstream. https://github.com/jqnatividad/qsv/pull/794
* removed utf8 first 8k screening. It was increasing code complexity and not very reliable. https://github.com/jqnatividad/qsv/pull/782

### Fixed
* `dedup`: refactored to use iterators to avoid out of bounds errors. https://github.com/jqnatividad/qsv/commit/f5e547b68410407851f217c706ad303bdbc5a583
* `exclude`: don't screen for utf8. This bugfix spurred the utf8 validation revamp, where I realized, I just needed to pull out utf8 screening https://github.com/jqnatividad/qsv/pull/781
* `py`:  `col`, not `row` https://github.com/jqnatividad/qsv/pull/793

## New Contributors
* @tmtmtmtm made their first contribution in https://github.com/jqnatividad/qsv/pull/783

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.88.2...0.89.0

## [0.88.2] - 2023-02-16

### Changed
* also show `--update` and `--updatenow` errors on stderr https://github.com/jqnatividad/qsv/pull/770
* `sortcheck`: when a file is not sorted, dupecount is invalid. Set dupecount to -1 to make it plainly evident when file is not sorted. https://github.com/jqnatividad/qsv/pull/771
* `excel`: added `--quiet` option https://github.com/jqnatividad/qsv/commit/99d88499df573f9f46992346f394d9372ceeffcc
* `extdedup`: minimize allocations in hot loop https://github.com/jqnatividad/qsv/commit/62096fa84505b6de2c108d1f07707008e1c2d170
* improved mem_file_check OOM-prevention helper function. Better error messages; clamp free memory headroom percentage between 10 and 90 percent https://github.com/jqnatividad/qsv/commit/6701ebfae58e942117378996ec6679544f620cbf and https://github.com/jqnatividad/qsv/commit/5cd8a95e7b36819f75f0d3bb8172dcff601b649b
* improved utf8 check error messages to give more detail, and not just say there is an encoding error https://github.com/jqnatividad/qsv/commit/c9b5b075d31b9639958193db919683475c3e3ba5
* improved README, adding Regular Expression Syntax section; reordered sections
* modified CI workflows to also check qsvlite
* Bump once_cell from 1.17.0 to 1.17.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/775
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-02-14

### Fixed
* `dedup` unnecessarily doing utf8 check; improve `input` usage text https://github.com/jqnatividad/qsv/pull/773
* `dedup`: fix unstable dedup results caused by using `par_sort_unstable_by` https://github.com/jqnatividad/qsv/pull/776
* `sort`: fix unstable sort results caused by using `par_sort_unstable_by` https://github.com/jqnatividad/qsv/commit/9f01df41a77dece75e434ee24b3ea0178d58deaf
* removed mispublished 0.88.1 release

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.88.0...088.2

## [0.88.0] - 2023-02-13

### Added
* `extdedup`: new command to deduplicate arbitrarily large CSV/text files using a memory-buffered, on-disk hash table. Not only does it dedup very large files using constant memory, it does so while retaining the file's original sort order, unlike `dedup` which loads the entire file into memory to sort it first before deduping by comparing neighboring rows https://github.com/jqnatividad/qsv/pull/762
* Added Out-of-Memory (OOM) handling for "non-streaming" commands (i.e. commands that load the entire file into memory) using a heuristic that if an input file's size is lower than the free memory available minus a default headroom of 20 percent, qsv processing stops gracefully with a detailed message about the potential OOM condition. This headroom can be adjusted using the `QSV_FREEMEMORY_HEADROOM_PCT` environment variable, which has a minimum value of 10 percent https://github.com/jqnatividad/qsv/pull/767
* add `-Q, --quiet` option to all commands that return counts to stderr (`dedup`, `extdedup`, `search`, `searchset` and `replace`) in https://github.com/jqnatividad/qsv/pull/768

### Changed
* `sort` & `sortcheck`: separate test suites and link from usage text https://github.com/jqnatividad/qsv/pull/756
* `frequency`: amortize allocations, preallocate with_capacity. Informal benchmarking shows an improvement of ~30%! :rocket: https://github.com/jqnatividad/qsv/pull/761
* `extsort`: refactor. Aligned options with `extdedup`; now also support stdin/stdout; added `--memory-limit` option  https://github.com/jqnatividad/qsv/pull/763
* `safenames`: minor optimization https://github.com/jqnatividad/qsv/commit/a7df378e0a755300e541dec0fef0b12d39b215f2
* `excel`: minor optimization https://github.com/jqnatividad/qsv/commit/75eac7875e276b45e668cbe91271ad86cec8db49
* `stats`: add date inferencing false positive warning, with a recommendation how to prevent false positives https://github.com/jqnatividad/qsv/commit/a84a4e614b5c14dd2e0d523bec4c6d9dbeb7c3ba
* `sortcheck`: added note to usage text that dupe_count is only valid if file is sorted https://github.com/jqnatividad/qsv/commit/ab69f144fa2ac375255bf9fbd6dd08bf538c1dfa
* reorganized Installation section to differentiate different options https://github.com/jqnatividad/qsv/commit/9ef8bfc0b90574b41629c7c7bd463289dc1dcb62
* bump MSRV to 1.67.1
* applied select clippy recommendations
* Bump flexi_logger from 0.25.0 to 0.25.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/755
* Bump pyo3 from 0.18.0 to 0.18.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/757
* Bump serde_json from 1.0.92 to 1.0.93 by @dependabot in https://github.com/jqnatividad/qsv/pull/760
* Bump filetime from 0.2.19 to 0.2.20 by @dependabot in https://github.com/jqnatividad/qsv/pull/759
* Bump self_update from 0.34.0 to 0.35.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/765
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-02-12

### Fixed
* `sortcheck`: correct wrong progress message showing invalid dupe_count (as dupe count is only valid if the file is sorted) https://github.com/jqnatividad/qsv/commit/8eaa8240249c5c7eb1ece068764a8caa7e804414
* `py` & `luau`: correct usage text about stderr https://github.com/jqnatividad/qsv/commit/1b56e72988e2dee1502517f8e2dbf036416efb8d


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.87.1...0.88.0

## [0.87.1] - 2023-02-02

### Changed
* `safenames`: refactor in https://github.com/jqnatividad/qsv/pull/754
   - better handling of headers that start with a digit, instead of replacing the digit with a _, prepend the unsafe prefix
   - quoted identifiers are also considered unsafe, unless conditional mode is used
   - verbose modes now also return a list of duplicate header names
* update MSRV to 1.67.0
* cargo update bump dependencies
* disable optimization on test profile for faster CI compilation, which was taking much longer than test run time
* optimize prebuilt nightlies to compile with target-cpu=native
* pin Rust nightly to 2023-02-01

### Fixed
^ `safenames`: fixed mode behavior inconsistencies https://github.com/jqnatividad/qsv/pull/754
all modes now use the same safenames algorithm. Before, the verbose modes used a simpler one leading to inconsistencies between modes (resolves safenames handling inconsistent between modes #753)

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.87.0...0.88.0

## [0.87.0] - 2023-01-29

### Added
* `apply`: add decimal separator --replacement option to thousands operation. This fully rounds out `thousands` formatting, as it will allow formatting numbers to support "euro-style" formats (e.g. 1.234.567,89 instead of 1,234,567.89) https://github.com/jqnatividad/qsv/pull/749
* `apply`: add round operation; also refactored thousands operation to use more appropriate `--formatstr` option instead of `--comparand` option to specify "format" of thousands separator policy https://github.com/jqnatividad/qsv/pull/751
* `applydp`: add round operation  https://github.com/jqnatividad/qsv/pull/752

### Changed
* changed MSRV policy to track latest Rust version in Homebrew, instead of latest Rust stable
* removed excess trailing whitespace in `apply` & `applydp` usage text
* moved `round_num` function from `stats.rs` to `util.rs` so it can be used in round operation in `apply` and `applydp`
* cargo update bump dependencies, notably tokio from 1.24.2 to 1.25.0
* pin Rust nightly to 2023-01-28

### Fixed
* `apply`: corrected thousands operation usage text - `hexfour` not `hex_four` https://github.com/jqnatividad/qsv/commit/6545aa2b3ce470b5f6c039c998e9f6fc21a6ad84


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.86.0...0.87.0


## [0.86.0] - 2023-01-29

### Added
* `apply`: added `thousands` operation which adds thousands separators to numeric values.
Specify the separator policy with --comparand (default: comma). The valid policies are:
comma, dot, space, underscore, hexfour (place a space every four hex digits) and
indiancomma (place a comma every two digits, except the last three digits). https://github.com/jqnatividad/qsv/pull/748
* `searchset`: added `--unmatched-output` option. This was done to allow Datapusher+ to screen for PIIs more efficiently. Writing PII candidate records in one CSV file, and the "clean" records in another CSV in just one pass.  https://github.com/jqnatividad/qsv/pull/742


### Changed
* `fetch` & `fetchpost`: expanded usage text info on HTTP2 Adaptive Flow Control support
* `fetchpost`: added more detail about `--compress` option
* `stats`: added more tests
* updated prebuilt zip archive READMEs https://github.com/jqnatividad/qsv/commit/072973efd7947a93773b2783d098eeace17d963d
* Bump redis from 0.22.2 to 0.22.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/741
* Bump ahash from 0.8.2 to 0.8.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/743
* Bump jql from 5.1.4 to 5.1.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/747
* applied select clippy recommendations
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-01-27


### Fixed
* `stats`: fixed antimodes null display. Use the literal `NULL` instead of just "" when listing NULL as an antimode. https://github.com/jqnatividad/qsv/pull/745
* `tojsonl`: fixed invalid escaping of JSON values https://github.com/jqnatividad/qsv/pull/746

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.85.0...0.86.0

## [0.85.0] - 2023-01-22

### Added
* Update csvs_convert by @kindly in https://github.com/jqnatividad/qsv/pull/736
* `sniff`: added `--delimiter` option https://github.com/jqnatividad/qsv/pull/732
* `fetchpost`: add `--compress` option in https://github.com/jqnatividad/qsv/pull/737
* `searchset`: several tweaks for PII screening requirement of Datapusher+. `--flag` option now shows regex labels instead of just row number; new `--flag-matches-only` option sends only matching rows to output when used with `--flag`; `--json` option returns rows_with_matches, total_matches and rowcount as json to stderr. https://github.com/jqnatividad/qsv/pull/738

### Changed
* `luau`: minor tweaks to increase code readability https://github.com/jqnatividad/qsv/commit/31d01c8b9eb1fe85262e9bf5fd237ae4493d562c
* `stats`: now normalize after rounding. Normalizing strips trailing zeroes and converts -0.0 to 0.0. https://github.com/jqnatividad/qsv/commit/f838272b4deb79d25ca5704cf3c89652c0b9a3bb
* `safenames`: mention CKAN-specific options https://github.com/jqnatividad/qsv/commit/f371ac25ba0c27e48b7b9b14a37dc47913cf0095
* `fetch` & `fetchpost`: document decompression priority https://github.com/jqnatividad/qsv/commit/43ce13c4bf7eb23dc5d051d522d6d52d3cc255aa
* Bump actix-governor from 0.3.2 to 0.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/728
* Bump sysinfo from 0.27.6 to 0.27.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/730
* Bump serial_test from 0.10.0 to 1.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/729
* Bump pyo3 from 0.17.3 to 0.18.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/731
* Bump reqwest from 0.11.13 to 0.11.14 by @dependabot in https://github.com/jqnatividad/qsv/pull/734
* cargo update bump for other dependencies
* pin Rust nightly to 2023-01-21

### Fixed
* `sniff`: now checks that `--sample` size is greater than zero https://github.com/jqnatividad/qsv/commit/cd4c390ce4322d7076866be27025d67800bc60e2

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.84.0...0.85.0

## [0.84.0] - 2023-01-14

### Added
* `headers`: added `--trim` option to trim quote and spaces from headers https://github.com/jqnatividad/qsv/pull/726


### Changed
* `input`: `--trim-headers` option also removes excess quotes https://github.com/jqnatividad/qsv/pull/727
* `safenames`: trim quotes and spaces from headers https://github.com/jqnatividad/qsv/commit/0260833bc8b36ea6e6ccb9e79687c76470a8a6b0
* cargo update bump dependencies
* pin Rust nightly to 2022-01-13


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.83.0...0.84.0

## [0.83.0] - 2023-01-13

### Added
* `stats`: add sparsity to "streaming" statistics https://github.com/jqnatividad/qsv/pull/719
* `schema`: also infer enum constraints for integer fields. Not only good for validation, this is also required by `tojsonl` for smarter boolean inferencing https://github.com/jqnatividad/qsv/pull/721

### Changed
* `stats`: change `--typesonly` so it will not automatically `--infer-dates`. Let the user decide. https://github.com/jqnatividad/qsv/pull/718
* `stats`: if median is already known, use it to calculate Median Absolute Deviation https://github.com/jqnatividad/qsv/commit/08ed08da4651a96bf05372b34b670063fbcec14f
* `tojsonl`: smarter boolean inferencing. It will infer a column as boolean if it only has a domain of two values,
and the first character of the values are one of the following case-insensitive "truthy/falsy"
combinations: t/f; t/null; 1/0; 1/null; y/n & y/null are treated as true/false. https://github.com/jqnatividad/qsv/pull/722 and https://github.com/jqnatividad/qsv/pull/723
* `safenames`: process `--reserved` option before `--prefix` option. https://github.com/jqnatividad/qsv/commit/b333549199726a3e92b95fb1d501fbdbbeede34a
* `strum` and `strum-macros` are no longer optional dependencies as we use it with all the binary variants now https://github.com/jqnatividad/qsv/commit/bea6e00fc400e8fafa2938832f8654d97c45fe34
* Bump qsv-stats from 0.6.0 to 0.7.0
* Bump sysinfo from 0.27.3 to 0.27.6
* Bump hashbrown from 0.13.1 to 0.13.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/720
* Bump actions/setup-python from 4.4.0 to 4.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/724
* change MSRV from 1.66.0 to 1.66.1

* cargo update bump indirect dependencies
* pin Rust nightly to 2023-01-12

### Fixed
* `safenames`: fixed `--prefix` option. When checking for invalid underscore prefix, it was checking for hyphen, not underscore, causing a problem with Datapusher+ https://github.com/jqnatividad/qsv/commit/4fbbfd3a479b6678fa9d4c823fd00b592b326c7a


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.82.0...0.83.0


## [0.82.0] - 2023-01-09

### Added
* `diff`: Find the difference between two CSVs ludicrously fast! by @janriemer in https://github.com/jqnatividad/qsv/pull/711
* `stats`: added [Median Absolute Deviation](https://en.wikipedia.org/wiki/Median_absolute_deviation) (MAD) https://github.com/jqnatividad/qsv/pull/715
* added Testing section to README https://github.com/jqnatividad/qsv/commit/517d69b496aaa9535a2b23b05e44a5999d8ef994

### Changed
* `validate`: schema less validation error improvements https://github.com/jqnatividad/qsv/pull/703
* `stats`: faster date inferencing https://github.com/jqnatividad/qsv/pull/706
* `stats`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/15e6284c20cccf4a6b74498336d31b0d7ba03285 https://github.com/jqnatividad/qsv/commit/3f0ed2b314765a546e28b534d5e82bff892592c3
* `stats`: refactored modes compilation https://github.com/jqnatividad/qsv/commit/6e448b041a2c78b3ce1cc89aadaff4a8d1081472
* `stats`: simplify if condition https://github.com/jqnatividad/qsv/commit/ae7cc85afe1dc4c3f87cbefe3b14dc93b28d94e9
* `luau`: show luau version when invoking --version https://github.com/jqnatividad/qsv/commit/f7f9c4297fb3dea685b5d0f631932b6b2ca4a99a
* `excel`: add "sheet" suffix to end msg for readability https://github.com/jqnatividad/qsv/commit/ae3a8e31784a24c8492de76c5074e477cc474063
* cache `util::count_rows` result, so if a CSV without an index is queried, it caches the result and future calls to count_rows in the same session will be instantaneous https://github.com/jqnatividad/qsv/commit/e805dedf5674cfbc56d9948791419ac6fd51f2fd
* Bump console from 0.15.3 to 0.15.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/704
* Bump cached from 0.41.0 to 0.42.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/709
* Bump mlua from 0.8.6 to 0.8.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/712
* Bump qsv-stats from 0.5.2 to 0.6.0 with the new MAD statistic support
* cargo update bump dependencies - notably mimalloc from 0.1.32 to 0.1.34, luau0-src from 0.4.1_luau553 to 0.5.0_luau555, csvs_convert from 0.7.9 to 0.7.11 and regex from 1.7.0 to 1.7.1
* pin Rust nightly to 2023-01-08

### Fixed
* `tojsonl`: fix escaping of unicode string. Replace hand-rolled escape fn with built-in escape_default fn https://github.com/jqnatividad/qsv/pull/707. Fixes https://github.com/jqnatividad/qsv/issues/705
* `tojsonl`: more robust boolean inferencing https://github.com/jqnatividad/qsv/pull/710. Fixes https://github.com/jqnatividad/qsv/issues/708


## New Contributors
* @janriemer made their first contribution in https://github.com/jqnatividad/qsv/pull/711

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.81.0...0.82.0

## [0.81.0] - 2023-01-02

### Added
* `stats`: added range statistic https://github.com/jqnatividad/qsv/pull/691
* `stats`: added additional mode stats. For mode, added mode_count and mode_occurrences. Added "antimode" (opposite of mode - least frequently occurring non-zero value), antimode_count and antimode_occurrences. https://github.com/jqnatividad/qsv/pull/694
* qsv-dateparser now recognizes unix timestamp values with fractional seconds to nanosecond precision as dates. `stats`, `sniff`, `apply datefmt` and `schema`, which all use qsv-dateparser, now infer unix timestamps as dates - https://github.com/jqnatividad/qsv/commit/a29ff8ea255d5aed9992556a0a23ab76117c8340 https://github.com/jqnatividad/qsv/pull/702
> USAGE NOTE: As timestamps can be float or integer, and data type inferencing will guess dates last, preprocess timestamp columns with apply datefmt first to more date-like, non-timestamp formats, so they are recognized as dates by other qsv commands.

### Changed
* `apply`: document numtocurrency --comparand & --replacement behavior https://github.com/jqnatividad/qsv/commit/cc88fe921d8cdf7eedcb0008e16ebb5c46744f33
* `index`: explicitly flush buffers after creating index https://github.com/jqnatividad/qsv/commit/ee5d790af1cde73dfc57b028bf52fa88e83cdaa4
* `sample`: no longer requires an index to do percentage sampling https://github.com/jqnatividad/qsv/commit/45d4657713ebe2ae8388ce55f4cb1a733e727024
* `slice`: removed unneeded utf8 check https://github.com/jqnatividad/qsv/commit/5a199f4442bd025cec31309bee44ac71bacbdfaa
* `schema`: expand usage text regarding `--strict-dates` https://github.com/jqnatividad/qsv/commit/3d22829f3cf0441961e854555cd0c333bcb3ffb1 
* `stats`: date stats refactor. Date stats are returned in rfc3339 format. Dates are converted to timestamps with millisecond precision while calculating date stats. https://github.com/jqnatividad/qsv/pull/690 https://github.com/jqnatividad/qsv/commit/e7c297795ff5e82cf1dc242090be11ecced6da9a
* filter out variance/stddev in tests as float precision issues are causing flaky CI tests  https://github.com/jqnatividad/qsv/pull/696
* Bump qsv-dateparser from 0.4.4 to 0.6.0
* Bump qsv-stats from 0.4.6 to 0.5.2
* Bump qsv-sniffer from 0.5.0 to 0.6.0
* Bump serde from 1.0.151 to 1.0.152 by @dependabot in https://github.com/jqnatividad/qsv/pull/692
* Bump csvs_convert from 0.7.7 to 0.7.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/693
* Bump once_cell from 0.16.0 to 0.17.0 https://github.com/jqnatividad/qsv/commit/d3ac2556c74e2ddd66dcee00e5e836d284b662a7
* Bump self-update from 0.32.0 to 0.34.0 https://github.com/jqnatividad/qsv/commit/5f95933f01e2e0c592b52d7424b6a832aafd3591
* Bump cpc from 1.8 to 1.9; set csvs_convert dependency to minor version https://github.com/jqnatividad/qsv/commit/ee9164810559f5496dfafba0e789b9cd84000a17
* applied select clippy recommendations
* deeplink to Cookbook from Table of Contents
* pin Rust nightly to 2023-01-01
* implementation comments on `stats`, `sample`, `sort` & Python distribution

### Fixed
* `stats`: prevent premature rounding, and make sum statistic use the same rounding method https://github.com/jqnatividad/qsv/commit/879214a1f3032f140f0207fe8807e1bb641110d7 https://github.com/jqnatividad/qsv/commit/1a1362031de8973b623598748bea4bc5fc6e08d3
* fix autoindex so we return the index path properly https://github.com/jqnatividad/qsv/commit/d3ce6a3918683d66bf0f3246c7d6e8518eead392
* `fetch` & `fetchpost`: corrected typo https://github.com/jqnatividad/qsv/commit/684036bbc237d5b80ea060f9ee8b8d46c1a2ad88


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.80.0...0.81.0

## [0.80.0] - 2022-12-23

### Added
* new `to` command. Converts CSVs "to" PostgreSQL, SQLite, XLSX, Parquet and Data Package by @kindly in https://github.com/jqnatividad/qsv/pull/656
* `apply`: add numtocurrency operation https://github.com/jqnatividad/qsv/pull/670
* `sort`: add --ignore-case option https://github.com/jqnatividad/qsv/pull/673
* `stats`: now computes descriptive statistics for dates as well https://github.com/jqnatividad/qsv/pull/684
* added --updatenow option, resolves https://github.com/jqnatividad/qsv/issues/661 https://github.com/jqnatividad/qsv/pull/662
* replace footnotes in Available Commands list with emojis :smile:


### Changed
* `apply` & `applydp`: expose --batch size option https://github.com/jqnatividad/qsv/pull/679
* `validate`: add last valid row to validation error https://github.com/jqnatividad/qsv/commit/7680011a2fcc459aa621414122ecaa869e98ae83
* `input`: add last valid row to error message https://github.com/jqnatividad/qsv/commit/492e51f85ab5a0637c201d7020d7ac2fdb72be96
* upgrade to csvs-convert 0.7.5 by @kindly in https://github.com/jqnatividad/qsv/pull/668
* Bump serial_test from 0.9.0 to 0.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/671
* Bump csvs_convert from 0.7.5 to 0.7.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/674
* Bump num_cpus from 1.14.0 to 1.15.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/678
* Bump robinraju/release-downloader from 1.6 to 1.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/677
* Bump actions/stale from 6 to 7 by @dependabot in https://github.com/jqnatividad/qsv/pull/676
* Bump actions/setup-python from 4.3.1 to 4.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/683
* added concurrency check to CI tests so that redundant CI test are canceled when new ones are launched
* instead of saying "descriptive statistics", use more understandable "summary statistics"
* changed publishing workflows to enable `to` feature for applicable target platforms
* cargo update bump dependencies, notably qsv-stats from 0.4.5 to 0.4.6 and qsv_currency from 0.5.0 to 0.6.0
* pin Rust nightly to 2022-12-22

### Fixed
* `stats`: fix leading zero handling https://github.com/jqnatividad/qsv/pull/667
* `apply`: fix currencytonum bug https://github.com/jqnatividad/qsv/pull/669


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.79.0...0.80.0


## [0.79.0] - 2022-12-16
### Added
* `safenames`: add --reserved option, allowing user to specify additional "unsafe" names https://github.com/jqnatividad/qsv/pull/657
* `safenames`: add --prefix option https://github.com/jqnatividad/qsv/pull/658
* `fetch` & `fetchpost`: added simple retry backoff multiplier - https://github.com/jqnatividad/qsv/commit/e343398ddd9c804237e73bbc652cc9e51c657b78

### Changed
* `excel`: refactored --metadata processing; added more debug messages; minor perf tweaks https://github.com/jqnatividad/qsv/commit/f137bab42f81518acd3ef825cd223b9970d70b02
* set MSRV to Rust 1.6.6
* cargo update bump several dependencies, notably qsv-dateparser
* pin Rust nightly to 2022-12-15

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.78.2...0.79.0


## [0.78.2] - 2022-12-13

### Changed
* cargo update bump paste 1.0.9 to 1.0.10
* pin Rust nightly to 2022-12-12

### Removed
* `excel`: remove --safenames option. If you need safenames, use the `safenames` command https://github.com/jqnatividad/qsv/commit/e5da73bcc64ef3a8c66c611fd6247fa331117544


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.78.1...0.78.2

## [0.78.1] - 2022-12-12

### Changed
* `qsvdp`: `apply` now available in qsvdp as`applydp` - removing the geocode and calconv subcommands, and removing all operations that require third-party crates EXCEPT dynfmt and datefmt which is needed for Datapusher+ https://github.com/jqnatividad/qsv/pull/652
* `excel`: fine-tune --metadata processing https://github.com/jqnatividad/qsv/commit/09530d4f65b06060d24b7ed3948aeab25b2aa7c8
* bump serde from 1.0.149 to 1.0.150
* `qsvdp` in now included in CI tests


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.78.0...0.78.1

## [0.78.0] - 2022-12-11

### Added
* `stats`: added leading zero handling when inferring types (e.g. zipcodes like "07094" are strings not integers) https://github.com/jqnatividad/qsv/pull/648
* `stats`: added --typesonly option, which infers only data types with date inferencing enabled for all columns  https://github.com/jqnatividad/qsv/pull/650
* `stats`: added underflow handing to sum statistic https://github.com/jqnatividad/qsv/commit/1b5e5451f929ad1c7dc5fb7f17b2a3261809ab05
* `excel`: expanded --metadata functionality, with the option to return workbook metadata as JSON as well https://github.com/jqnatividad/qsv/pull/651
* added platform-specific README for prebuilt zip archives https://github.com/jqnatividad/qsv/commit/15e247e523dbc22a50ebff1b15d7d0c4eb668bd5

### Changed
* `safenames`: improved usage text
* `stats`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/88be38b542fc61470a7b0331e7be3a3cad62a7bb and https://github.com/jqnatividad/qsv/commit/8aa58c5ad733116d246e171bcea622c1378b8e48
* `join`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/92d41910077148f769ccf2c8a283be2c30d68bbf
* `exclude`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/f3cc0ac29c5f3e6cec5a08d3aac3371d32b5eb0f
* `sniff`: minor performance tweak https://github.com/jqnatividad/qsv/commit/d2a4676fcb5189fc9232538e68854cfcf4ef808b
* `sortcheck`: minor performance tweak https://github.com/jqnatividad/qsv/commit/83c22ae5a623a8b0740f7024aac9448ee809eabd
* switch GitHub Actions to use ubuntu-20.04 so as not to link to too new glibc libraries, preventing older distros from running the linux-gnu prebuilts.
* switch GitHub Actions to use macos-12 to minimize flaky CI tests
* expanded `qsvdp` description in README
* Bump actions/setup-python from 4.3.0 to 4.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/645
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-12-10


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.77.0...0.78.0

## [0.77.0] - 2022-12-08

### Added
* `safenames`: added Verbose JSON options https://github.com/jqnatividad/qsv/pull/644

### Changed
* `py` & `luau`: improved usage text
* opt-in self-update in https://github.com/jqnatividad/qsv/pull/640 and https://github.com/jqnatividad/qsv/pull/641
* Create README in prebuilt zip archive with platform specific notes https://github.com/jqnatividad/qsv/pull/642
* Simplify python map_datetime test so it works on older Python versions https://github.com/jqnatividad/qsv/commit/e85e4e7bf9bf379f8478b066a9f6dea21afbf0e8
* include date.lua in qsv package so `cargo install` works https://github.com/jqnatividad/qsv/commit/11a0ff8edc5405afd9cc6637de026bf2138a7df0
* Bump data-encoding from 2.3.2 to 2.3.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/638
* cargo update bump several dependencies
* pin Rust nightly to 2022-12-07

### Fixed:
* `safenames`: fixed calculation of unsafe headers as it was dupe-counting some unsafe headers - https://github.com/jqnatividad/qsv/pull/644


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.3...0.77.0

## [0.76.3] - 2022-12-05

### Changed
* cargo update bump serde from 1.0.148 to 1.0.149
* simplify python datetime test so it runs on Python 3.6 and above

### Fixed
* reverted `not_luau_compatible` introduced in 0.76.2 and 0.76.3. Adjusted Github Action publish workflow instead to properly build `luau` in qsvdp when the platform supports it.

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.2...0.76.3

## [0.76.2] - 2022-12-04

### Fixed
* tweak `not_luau_compatible` feature so we can more easily disable `luau` feature when cross-compiling for some platforms where we cannot properly build luau.

NOTE: Not published on crates.io due to problems creating prebuilt binaries

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.1...0.76.2

## [0.76.1] - 2022-12-04

### Fixed
* added `not_luau_compatible` feature so we can more easily disable `luau` feature when cross-compiling for some platforms where we cannot properly build luau.

NOTE: Not published on crates.io due to problems creating prebuilt binaries

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.0...0.76.1

## [0.76.0] - 2022-12-04

### Added
* `qsvdp`: add `luau` in anticipation of Datapusher+ optional preprocessing https://github.com/jqnatividad/qsv/pull/634
* `luau`: added ability to load libraries using "require"; preload LuaDate library https://github.com/jqnatividad/qsv/pull/633
* `luau`: added more extensive debug logging support, adding _idx to debug log messages; trace log level support showing global vars and record values when an error occurs https://github.com/jqnatividad/qsv/pull/636 and https://github.com/jqnatividad/qsv/pull/637

### Changed
* `py` and `luau`: when errors encountered, return non-zero exit code, along with error count to stderr https://github.com/jqnatividad/qsv/pull/631
* `safenames` and `excel`: Unsafe empty column/header names are replaced with "\_blank" instead of "\_" https://github.com/jqnatividad/qsv/pull/632
* `frequency`: replace foreach iterator with regular for; remove unneeded assert https://github.com/jqnatividad/qsv/commit/74eb321defbf294675872a7dd891e8a7aedd31f1
* bumped qsv-stats from 0.4.1 to 0.4.5 - fixing sum rounding and variance precision errors. 
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-12-03

### Fixed
* `stats`: fix sum rounding and variance precision errors https://github.com/jqnatividad/qsv/pull/635

NOTE: Not published on crates.io due to problems creating prebuilt binaries

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.75.0...0.76.0

## [0.75.0] - 2022-12-01

### Added:
* `py`: added python datetime module by default in https://github.com/jqnatividad/qsv/pull/629
* `qsvdp` ([Datapusher+](https://github.com/dathere/datapusher-plus) optimized binary variant): added self-update. However, unlike `qsv` and `qsvlite` binary variants, `qsvdp` will not automatically prompt for a self-update, and will only inform the user if there is a new release. The user will need to invoke the `--update` option explicitly. https://github.com/jqnatividad/qsv/pull/622

### Changed:
* `stats`: Speedup type checking by @kindly in https://github.com/jqnatividad/qsv/pull/625
* `validate`: Added a useful note about validate output by @aborruso in https://github.com/jqnatividad/qsv/pull/624
* `luau`: Now precompiles all scripts, including the `--prologue` & `--epilogue` scripts, into bytecode https://github.com/jqnatividad/qsv/commit/e97c2caf81316bcf655875a9bee4c78dac5a8b70
* `frequency`: remove unsafe from_utf8_unchecked https://github.com/jqnatividad/qsv/commit/16642e8ee3364309c1a774142976f6207ba5c594
* More robust autoindexing in https://github.com/jqnatividad/qsv/pull/623
* minor clippy performance tweaks to [rust-csv fork](https://github.com/jqnatividad/rust-csv/tree/perf-tweaks)
* Bump serde from 1.0.147 to 1.0.148 by @dependabot in https://github.com/jqnatividad/qsv/pull/620
* cargo update bump several indirect dependencies
* improved README; use :sparkle: to indicate commands behind a feature flag
* pin Rust nightly to 2022-11-30

## New Contributors
* @aborruso made their first contribution in https://github.com/jqnatividad/qsv/pull/624
* @kindly made their first contribution in https://github.com/jqnatividad/qsv/pull/625

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.74.0...0.75.0

## [0.74.0] - 2022-11-27

### Added:
* `safenames`: added --verify and --verbose modes in https://github.com/jqnatividad/qsv/pull/610 and https://github.com/jqnatividad/qsv/pull/615

### Changed:
* `excel`: align --safenames option to `safenames` command in https://github.com/jqnatividad/qsv/pull/611 and https://github.com/jqnatividad/qsv/pull/616
* `luau`: Now precompiles main script to bytecode; now allow loading luau script from file for main, prologue and epilogue scripts in https://github.com/jqnatividad/qsv/pull/619
* `sniff`: increase default sample size from 100 to 1000 in https://github.com/jqnatividad/qsv/commit/40d52cf0c67e39d645a1c76a26ae234999317b0b
* `validate`: applied various optimizations in https://github.com/jqnatividad/qsv/commit/bfed127f28c4ccf6e9a18a5998588396594831d2 and https://github.com/jqnatividad/qsv/commit/06c109a0335326f57d903211334b4f2fb1ab7ccc
* updated Github Actions workflows to reflect removal of luajit feature
* Bump sysinfo from 0.26.7 to 0.26.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/614
* Bump rust_decimal from 1.26.1 to 1.27.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/617
* cargo bump update several indirect dependencies
* applied various clippy recommendations
* pin Rust nightly to 2022-11-25

### Removed:
* `luajit`: removed as its been deprecated by optimized `luau` command which now support precompiling to bytecode, largely obviating the main feature of LuaJIT - Just-in-Time compilation in https://github.com/jqnatividad/qsv/pull/619

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.73.2...0.74.0

## [0.73.2] - 2022-11-22

### Changed:
* Link to tests as examples from usage text in https://github.com/jqnatividad/qsv/pull/608
* Bump serde_json from 1.0.88 to 1.0.89 by @dependabot in https://github.com/jqnatividad/qsv/pull/607
* cargo update bump to get latest crossbeam crates to replace yanked crates https://github.com/jqnatividad/qsv/commit/5108a87b0f5e2d5a7cfef3f60f4cd6b3659bce7d 

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.73.1...0.73.2

## [0.73.1] - 2022-11-21
### Changed:
* rename `safename` command to `safenames` for consistency
* cargo update bump indirect dependencies

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.73.0...0.73.1

## [0.73.0] - 2022-11-21
### Added
* `safenames`: new command to modify header names to db-safe names in https://github.com/jqnatividad/qsv/pull/606
* `apply`: added `censor-count` operation in https://github.com/jqnatividad/qsv/pull/599
* `apply`: added `escape` operation in https://github.com/jqnatividad/qsv/pull/600
* `excel`: added `--safe-names` option in https://github.com/jqnatividad/qsv/pull/598

### Changed
* `apply`: refactored to use enums instead of strings for operations in https://github.com/jqnatividad/qsv/pull/601
* `fetch` & `fetchpost`: --http-header -H shortcut in https://github.com/jqnatividad/qsv/pull/596
* `excel`: smarter date parsing for XLSX files; rename --safe-column-names to --safe-names in https://github.com/jqnatividad/qsv/pull/603
* Smarter safe names in https://github.com/jqnatividad/qsv/pull/605
* Bump uuid from 1.2.1 to 1.2.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/594
* Bump mimalloc from 0.1.31 to 0.1.32 by @dependabot in https://github.com/jqnatividad/qsv/pull/595
* Bump censor from 0.2.0 to 0.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/597
* Bump Swatinem/rust-cache from 1 to 2 by @dependabot in https://github.com/jqnatividad/qsv/pull/602
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-11-19

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.72.0...0.73.0

## [0.72.0] - 2022-11-14

### Added
* `apply`: added --keep-zero-time option in https://github.com/jqnatividad/qsv/pull/590
* `lua` and `luajit`: added  --prologue & --epilogue options in https://github.com/jqnatividad/qsv/pull/592
* `luau` & `luajit`: switched from Lua to Luau; added special vars _idx and _rowcount in https://github.com/jqnatividad/qsv/pull/593
* `luau` & `luajit`: return exitcode 1 if interpretation error is encountered https://github.com/jqnatividad/qsv/commit/655041b86c86c3ce0024d1e20599c98dfab28658

### Changed
* `schema` & `validate`: expand description/usage text in https://github.com/jqnatividad/qsv/commit/60dfebc9f401045467417b2065481b657ff82c92
* `validate`: return exitcode 0 if CSV is valid; exitcode 1 otherwise in https://github.com/jqnatividad/qsv/pull/591
* Bump hashbrown from 0.12.3 to 0.13.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/586
* cargo bump update indirect dependencies, notably chrono from 0.4.22 to 0.4.23
* Shortened command descriptions for `luau` & `luajit` and added salient notes to new interpreter section
* adjust GitHub Actions workflows to use `luau` feature
* pin Rust nightly to 2022-11-14


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.71.1...0.72.0

## [0.71.1] - 2022-11-09

### Changed
* `python` feature is no longer enabled in the prebuilt binaries to avoid distribution issues and qsv panicking if the exact python version it was statically linked against
is not available. If you require the `python` feature, you'll have to install/build for source.

### Fixed
* whirlwind tour: `join`'s `--no-case` option has been replaced by `--ignore-case` by @alperyilmaz in https://github.com/jqnatividad/qsv/pull/585

## New Contributors
* @alperyilmaz made their first contribution in https://github.com/jqnatividad/qsv/pull/585

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.71.0...0.71.1

## [0.71.0] - 2022-11-08
### Added
* `apply`: new `encode` and  `decode` operations in https://github.com/jqnatividad/qsv/pull/569
* `apply`: add ability to show confidence to whatlang language detection. in https://github.com/jqnatividad/qsv/pull/579
* `count`: add --width option in https://github.com/jqnatividad/qsv/pull/582
* `fetch` & `fetchpost`: Added --user_agent option by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/565 and https://github.com/jqnatividad/qsv/commit/f59bd8591079f22df3c40e5f036c5e2ff83e77f8
* Documented Homebrew installer :rocket: created by @FnControlOption

### Changed
* `apply`: refactor operations validation in https://github.com/jqnatividad/qsv/pull/564 and https://github.com/jqnatividad/qsv/commit/f83ec6f7e7fa7bed9bcc2b5e55516a61e5154b52
* `sortcheck`: expand usage text and use fail_clierror macro https://github.com/jqnatividad/qsv/commit/8513b53eaac594d20106b3f77f73f3d1b63e227d
* `stats`: minor refactoring https://github.com/jqnatividad/qsv/commit/38795134e3ed66bf0816eeee2a68aa9b557c4908
* `tojsonl`: it does "smart" conversion of CSV to JSONL https://github.com/jqnatividad/qsv/commit/af98290bf1803ae5ab3e01df5f20f5b007912e02
* `validate`: also show --progressbar when doing schemaless validation https://github.com/jqnatividad/qsv/commit/aae550aa0b1042e205689ae40d19c0532e7ae584
* only show enabled commands in command list in https://github.com/jqnatividad/qsv/pull/583
* Updated the benchmark script by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/581
* Switch back to using num_cpus for detecting parallelism https://github.com/jqnatividad/qsv/commit/b7dbed88f7d931e03a835ca4a929328c2c4a34b6
* qsv now links against Python 3.11 for the `py` command in https://github.com/jqnatividad/qsv/pull/576
* Bump robinraju/release-downloader from 1.5 to 1.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/568
* Bump newline-converter from 0.2.0 to 0.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/567
* Bump sysinfo from 0.26.5 to 0.26.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/566 and https://github.com/jqnatividad/qsv/pull/572
* Bump ahash from 0.8.0 to 0.8.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/575
* Bump flexi_logger from 0.24.0 to 0.24.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/574
* Bump pyo3 from 0.17.2 to 0.17.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/573
* Bump jql from 5.1.1 to 5.1.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/577
* Bump num_cpus from 1.13.1 to 1.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/580
* Bump mimalloc from 0.1.30 to 0.1.31
* Bump indicatif from 0.17.1 to 0.17.2
* cargo update bump several indirect dependencies
* updated rustfmt.toml with comment and string formatting options
* bump MSRV to 1.65.0
* pin Rust Nightly to 2022-11-07

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.70.0...0.71.0

## [0.70.0] - 2022-10-24

### Added
* `apply`: additional operations - `squeeze0`, `strip_prefix` and `strip_suffix` https://github.com/jqnatividad/qsv/pull/518 & https://github.com/jqnatividad/qsv/pull/519
* `apply`: add `calcconv` subcommand, which parses & evaluate math expressions, with support for units & conversions. https://github.com/jqnatividad/qsv/pull/560

### Changed
* `search` & `searchset`: make match count optional https://github.com/jqnatividad/qsv/pull/526
* `jsonl`: remove panic and do proper error handling; add  --ignore-errors option https://github.com/jqnatividad/qsv/pull/531
* `py`: py command does not do aggregations (reduce) operations https://github.com/jqnatividad/qsv/pull/548
* `lua` & `luajit` can do aggregations across CSV rows and `py` cannot https://github.com/jqnatividad/qsv/pull/549
* `py`: add more complex f-string formatting example https://github.com/jqnatividad/qsv/pull/556
* Standardize ignore case option https://github.com/jqnatividad/qsv/pull/535
* Use rustfmt nightly to take advantage of advanced features like StdExternalCrate https://github.com/jqnatividad/qsv/pull/514 & https://github.com/jqnatividad/qsv/pull/517
* Update benchmark-basic.sh by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/542
* Use fail macros more consistently https://github.com/jqnatividad/qsv/pull/545
* Use Redis `ahash` feature for performance
* Added wix file for future Windows Installer by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/546
* Bump console from 0.15.1 to 0.15.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/512
* Bump pyo3 from 0.17.1 to 0.17.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/522
* Bump jql from 5.0.2 to 5.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/521
* Bump titlecase from 2.2.0 to 2.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/534
* Bump itoa from 1.0.3 to 1.0.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/533
* Bump sysinfo from 0.26.4 to 0.26.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/554
* Bump mlua from 0.8.3 to 0.8.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/540
* Bump uuid from 1.1.2 to 1.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/539
* Bump flexi_logger from 0.23.3 to 0.24.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/538
* Bump serde_json from 1.0.85 to 1.0.86 by @dependabot in https://github.com/jqnatividad/qsv/pull/537
* Bump actions/setup-python from 4.2.0 to 4.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/541
* Bump filetime from 0.2.17 to 0.2.18 by @dependabot in https://github.com/jqnatividad/qsv/pull/559
* Bump redis from 0.21.6 to 0.22.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/555
* Bump cached from 0.39.0 to 0.40.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/561
* Bump whatlang from 0.16.1 to 0.16.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/562
* cargo update bump several indirect dependencies
* Pin Rust nightly to 2022-10-22


### Fixed
* `excel`: xls float temporary workaround for #516 that was ultimately fixed in PR 558 https://github.com/jqnatividad/qsv/pull/520
* `tojsonl`: escape newlines and double quotes. Fixes #552 https://github.com/jqnatividad/qsv/pull/553
* `tojsonl`: better error handling; when checking stdin for utf8, make sure its not empty. Fixes #536 https://github.com/jqnatividad/qsv/pull/536

### Removed
* `excel`: removed xls float workaround now that calamine crate has been fixed. Fixes #516 removing need for PR 520 workaround. https://github.com/jqnatividad/qsv/pull/558
* removed obsolete Rust Nightly workflow https://github.com/jqnatividad/qsv/commit/2a99318242040300130c323dc3e7df504a6e3b2e


## New Contributors
* @minhajuddin2510 made their first contribution in https://github.com/jqnatividad/qsv/pull/542

## [0.69.0] - 2022-09-28

### Added
* `luajit`: new command using LuaJIT, which is much faster than Lua https://github.com/jqnatividad/qsv/pull/500

### Changed
* `python`: tweaks. Expanded usage text. Only show python version when logging is on.  https://github.com/jqnatividad/qsv/pull/507
* `fetch` & `fetchpost`: apply clippy recommendation https://github.com/jqnatividad/qsv/commit/dd7220bce2811d9e8248c379af5d5c38da3b02d5
* `excel`: use `winfo!` macro https://github.com/jqnatividad/qsv/commit/7211ff214a58394d68c8c7484e8ef4505d75b482
* Removed anyhow dependency https://github.com/jqnatividad/qsv/pull/508
* Bump actions/stale from 5 to 6 by @dependabot in https://github.com/jqnatividad/qsv/pull/505
* Bump sysinfo from 0.26.3 to 0.26.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/510
* Cargo update bump several indirect dependencies
* include Python 3.10 shared libraries when publishing for select platforms
* bump MSRV to Rust 1.64.0
* Pin Rust nightly to 2022-09-26

### Fixed
* `python`: corrected erroneous --helper example. Included hashhelper.py example.
* `extsort`: fixed --help bug (https://github.com/jqnatividad/qsv/issues/506)

## [0.68.0] - 2022-09-16
### Changed
* Simplify python support. For prebuilt binaries, Python 3.10 is now required and the python 3.10 shared libraries are bundled for select platforms.
If you require an earlier version of Python (3.6 and up), you'll have to install/compile from source. https://github.com/jqnatividad/qsv/pull/492
* Smarter self update. --update can still be explicitly invoked even when self-update feature has been disabled. Further, if you compiled qsv from source,
self-update will only notify you of new releases, instead of proceeding with self-update. https://github.com/jqnatividad/qsv/pull/490 and https://github.com/jqnatividad/qsv/pull/493
* `lua`: switch from Lua 5.4 to LuaJIT 2.1, primarily for performance https://github.com/jqnatividad/qsv/pull/495
* `lua`: when filtering using floats, "0.0" is false
* `join`: removed unneeded utf8 check
* `search`: simplify regex_unicode check
* `fetch` & `fetchpost`: optimize imports; remove unneeded utf8 check
* Bump anyhow from 1.0.64 to 1.0.65 by @dependabot in https://github.com/jqnatividad/qsv/pull/498
* Bump self_update from 0.31.0 to 0.32.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/499
* add additional copyright holder to MIT License
* Improved publishing workflow for prebuilt binaries
* cargo update bumped several dependencies
* pin Rust nightly to 2022-09-14

### Fixed
* fix typos by @kianmeng in https://github.com/jqnatividad/qsv/pull/491
* `python`: better error handling. When mapping/filtering, python expression errors no longer cause a panic, but instead fail to map/filter as expected (when mapping, "\<ERROR\>" is returned, when filtering, the filter is not applied), and continue processing. Also, other errors are properly propagated instead of panicking. https://github.com/jqnatividad/qsv/pull/496
* `lua`: better error handling. When mapping/filtering, Lua errors no longer cause a panic, but instead fail to map/filter as expected (when mapping, "\<ERROR\>" is returned, when filtering, the filter is not applied), and continue processing. https://github.com/jqnatividad/qsv/pull/497

## [0.67.0] - 2022-09-09
### Added
* added `self_update` feature, so users can build qsv without self-update engine https://github.com/jqnatividad/qsv/pull/483 and https://github.com/jqnatividad/qsv/pull/484

### Changed
* `search` & `searchset`: --quick option returns first match row to stderr https://github.com/jqnatividad/qsv/pull/475
* `python`: make --batch size configurable https://github.com/jqnatividad/qsv/pull/485
* `stats`: added more implementation comments; standardize string creation
* `replace`: add conditional compilation to eliminate dead_code warning
* `lua`: when filtering, non-zero integers are true
* refactored `workdir.rs` test helpers
* refactored `util:init_logger()` to log command-line arguments
* Bump url from 2.3.0 to 2.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/489
* Bump anyhow from 1.0.63 to 1.0.64 by @dependabot in https://github.com/jqnatividad/qsv/pull/478
* Bump sysinfo from 0.26.1 to 0.26.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/477
* Bump robinraju/release-downloader from 1.4 to 1.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/481
* cargo update bump indirect dependencies
* pin Rust nightly to 2022-09-07

## [0.66.0] - 2022-09-01

### Added
* `apply`: added Multi-column subcommands by @udsamani in https://github.com/jqnatividad/qsv/pull/462
* `stats`:  added --round option https://github.com/jqnatividad/qsv/pull/474
* created `fail_format!` macro for more concise error handling in https://github.com/jqnatividad/qsv/pull/471

### Changed
* Move command usage text to beginning of cmd source code, so we don't need to move around deeplinks to usage texts from README https://github.com/jqnatividad/qsv/pull/467
* Optimize conditional compilation of various qsv binary variants, removing dead code https://github.com/jqnatividad/qsv/pull/473
* `fetch` & `fetchpost`: removed initial burst of requests, making the commands "friendlier" to rate-limited APIs
* `search`, `searchset` & `replace`: minor performance optimizations
* created dedicated rustfmt GitHub action workflow to ensure code is always rust formatted. Previously, rustfmt check was in Linux workflow.
* applied some clippy recommendations
* Bump actix-governor from 0.3.1 to 0.3.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/461
* cargo update bumped several dependencies
* pin Rust nightly to 2022-08-31
* set RUSTFLAGS to emit=asm when compiling pre-built binaries for performance
  see http://likebike.com/posts/How_To_Write_Fast_Rust_Code.html#emit-asm

### Fixed
* `extsort` code was being compiled for qsvdp even if it was not enabled
* bump sysinfo from 0.25.2 to 0.26.0, fixing segfault on Apple Silicon
* fixed qsvnp on Windows so it doesn't look for python shared libraries even if python is not enabled
* fixed CliError::Other so it returns bad exitcode (exitcode 1) instead of incorrect_usage (exit code 2)

## New Contributors
* @udsamani made their first contribution in https://github.com/jqnatividad/qsv/pull/462

## [0.65.0] - 2022-08-28

### Added
* Major refactoring of main variants - removing redundant code and moving them to a new module - clitypes.rs. Added custom exit codes. 
  Removed need to have --exitcode option in several commands as qsv now returns exit codes for ALL commands in a standard way. https://github.com/jqnatividad/qsv/pull/460
* Major refactoring of CI test helpers in workdir.rs

### Changed
* `py`: use python interning to amortize allocs https://github.com/jqnatividad/qsv/pull/457
* `search` & `searchset`: return num of matches to stderr; add --quick option; remove --exitcode option https://github.com/jqnatividad/qsv/pull/458
* `extsort`: improved error handling
* `fetch` & `fetchpost`: better --report option handling https://github.com/jqnatividad/qsv/pull/451
* `lua`: faster number to string conversion using itoa and ryu
* `replace`: removed --exitcode option
* `sortcheck`: --json options now always cause full scan of CSV
* `stats`: expanded usage text, explicitly listing stats that require loading the entire CSV into memory. Mentioned data type inferences are guaranteed.
* cargo update bumped several dependencies
* pin Rust nightly to 2022-08-27

### Fixed
* `py`: batched python processing refactor. Instead of using one GILpool for one session, `py` now processes in batches of 30,000 rows, releasing memory after each batch.  This resulted in memory consumption levelling out, instead of increasing to gigabytes of memory with very large files. As an added bonus, this made the `py` command ~30% faster in testing. :smile:  https://github.com/jqnatividad/qsv/pull/456

## [0.64.0] - 2022-08-23
### Added
* added `sortcheck` command https://github.com/jqnatividad/qsv/pull/445
* `replace`: added --exitcode and --progressbar options 

### Changed
* `apply`: improved usage text
* `excel`: replace --list-sheets option with expanded --metadata option https://github.com/jqnatividad/qsv/pull/448
* `sortcheck` improvements https://github.com/jqnatividad/qsv/pull/447
* `extsort`: improved error handling
* progressbar messages are now logged
* bump pyo3 from 0.16 to 0.17
* bump reqwest & redis "patches" further upstream
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-08-22

### Fixed
* `extsort`: fixed sysinfo segfault on Apple Silicon by pinning sysinfo to 0.25.2 https://github.com/jqnatividad/qsv/pull/446
* `tojsonl`: fixed panic with stdin input

## [0.63.2] - 2022-08-18
### Added
* `fetchpost`: added formdata to report https://github.com/jqnatividad/qsv/pull/434
* `search` & `searchset`: added Custom exit codes; --exitcode option https://github.com/jqnatividad/qsv/pull/439
* `search` & `searchset`: added --progressbar option
* progressbars are now optional by default; added QSV_PROGRESSBAR env var to override setting
* `search`, `searchset` & `replace`: added mem-limit options for regex-powered commands https://github.com/jqnatividad/qsv/pull/440
### Changed
* Bump jql from 4.0.7 to 5.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/436
* progressbars are now off by default, and are disabled with stdin input https://github.com/jqnatividad/qsv/pull/438
* `lua` & `py`: improved error-handling when loading script files
* `stats`: changed to using AtomicBool instead of OnceCell, use with_capacity in hot compute loop to minize allocs - hyperfine shows 18% perf increase with these changes
* self-update now gives a proper error message when GitHub is rate-limiting updates
* cargo update bump several dependencies
* document MSRV policy
* pin Rust Nightly to 2022-08-16

### Fixed
* fixed stdin input causing an error when progressbars are enabled https://github.com/jqnatividad/qsv/pull/438

## [0.62.0] - 2022-08-12
### Added
* `fetchpost`: new command that uses HTTP POST, as opposed to `fetch` - which uses HTTP GET ([difference between HTTP GET & POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)) https://github.com/jqnatividad/qsv/pull/431
* Added `qsvnp` binary variant to prebuilt binaries - qsv with all the features EXCEPT python

### Changed
* `fetch`: refactor report parameter processing https://github.com/jqnatividad/qsv/pull/426
* Bump serde from 1.0.142 to 1.0.143 by @dependabot in https://github.com/jqnatividad/qsv/pull/423
* Bump ahash from 0.7.6 to 0.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/425
* Bump serial_test from 0.8.0 to 0.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/428
* Bump anyhow from 1.0.60 to 1.0.61 by @dependabot in https://github.com/jqnatividad/qsv/pull/427
* Bump sysinfo from 0.25.1 to 0.25.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/429
* Bump actix-governor from 0.3.0 to 0.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/430
* cargo update bump various indirect dependencies
* pin Rust nightly to 2022-08-11
* change MSRV to 1.63

### Fixed
* `excel`: fixed empty sheet handling https://github.com/jqnatividad/qsv/pull/422

## [0.61.4] - 2022-08-07
### Changed
* `py`: qsv uses the present working directory to find python shared library
* `py`: show python version info on startup
* publish qsvnp - another binary variant with all features except python
* bumped once_cell from 1.12 to 1.13
* use reqwest upstream with MSRV from 1.49 to 1.56; lazy_static to once_cell
* update calamine fork with chrono time feature disabled
* BetterTOML reformat cargo.toml
* pin Rust nightly to 2022-08-06

### Fixed
* `excel`: remove unneeded checkutf8 for writer

## [0.61.2] - 2022-08-04
### Changed
* `fetch`: Reformatted report so response is the last column; do not allow --timeout to be zero; progressbar refresh set at 5 times/sec; show name of generated report at the end. https://github.com/jqnatividad/qsv/pull/404
* `fetch`: report improvements. Remove `qsv_fetch_` column prefix in short report; change progressbar format to default characters https://github.com/jqnatividad/qsv/pull/406
* `excel`: make --sheet case-insensitive; better error-handling  https://github.com/jqnatividad/qsv/pull/416
* `py`: add detected python version to --version option
* Only do input utf8-encoding check for commands that need it. https://github.com/jqnatividad/qsv/pull/419
* Bump cached from 0.37.0 to 0.38.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/407
* Bump anyhow from 1.0.58 to 1.0.59 by @dependabot in https://github.com/jqnatividad/qsv/pull/408
* Bump serde from 1.0.140 to 1.0.141 by @dependabot in https://github.com/jqnatividad/qsv/pull/409
* Bump ryu from 1.0.10 to 1.0.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/414
* Bump anyhow from 1.0.59 to 1.0.60 by @dependabot in https://github.com/jqnatividad/qsv/pull/413
* Bump mlua from 0.8.2 to 0.8.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/412
* Bump actions/setup-python from 4.1.0 to 4.2.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/411
* Bump flexi_logger from 0.22.5 to 0.22.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/417
* Bump indicatif from 0.16.2 to 0.17.0
* Bump chrono from 0.4.19 to 0.4.20
* Bump qsv-dateparser from 0.4.2 to 0.4.3
* pin Rust nightly to 2022-08-03

### Fixed
* fixed double progressbars https://github.com/jqnatividad/qsv/pull/405
* fix utf8 encoding check to resolve [#410](https://github.com/jqnatividad/qsv/issues/410) https://github.com/jqnatividad/qsv/pull/418

## [0.61.1] - 2022-07-30
### Added
* `fetch`: add elapsed time, retries to reports; add --max-retries option https://github.com/jqnatividad/qsv/pull/395

### Changed
* `lua`: better error messages https://github.com/jqnatividad/qsv/pull/399
* `python`: better error messages https://github.com/jqnatividad/qsv/pull/400
* `fetch`: improved error handling https://github.com/jqnatividad/qsv/pull/402
* `stats`: improve performance by using `unwrap_unchecked` in hot compute loop
* Bump indicatif from 0.16.2 to 0.17.0 https://github.com/jqnatividad/qsv/pull/403
* Bump mlua from 0.8.1 to 0.8.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/394
* Bump console from 0.15.0 to 0.15.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/398
* Bump grex from 1.3 to 1.4
* Cargo update bump various dependencies
* pin Rust nightly to 2022-07-29

### Fixed
* `excel`:  fixed --sheet option bounds checking https://github.com/jqnatividad/qsv/pull/401

## [0.60.0] - 2022-07-24
### Added
* `fetch`: add redis --flushdb option https://github.com/jqnatividad/qsv/pull/387
* `fetch`: add --report & --cache-error options. --report creates a separate report file, detailing the URL used,
the response, the HTTP status code, and if its a cache hit.
--cache-error is used to also cache errors - i.e. identical fetches will return the cached error. Otherwise, fetch will
request the URL again. https://github.com/jqnatividad/qsv/pull/393

### Changed
* `fetch`: fast defaults. Now tries to go as fast as possible, leveraging dynamic throttling (using RateLimit and Retry-After headers) 
but aborting after 100 errors. Also added a separate error progress bar. https://github.com/jqnatividad/qsv/pull/388
* Smarter `tojsonl`. Now scans CSV file and infers data types and uses the appropriate JSON data type https://github.com/jqnatividad/qsv/pull/389
* `tojsonl` is also multithreaded https://github.com/jqnatividad/qsv/pull/392
* `stats`: use unwrap_unchecked for even more performance https://github.com/jqnatividad/qsv/pull/390
* `fetch`: refactor dynamic throttling https://github.com/jqnatividad/qsv/pull/391
* Bump sysinfo from 0.24.6 to 0.24.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/384
* cargo bump update several dependencies
* pin Rust nightly to 2022-07-23

### Fixed
* `fetch`: fix --http-header parsing bug https://github.com/jqnatividad/qsv/pull/386

## [0.59.0] - 2022-07-18
### Added
* added `tojsonl` command - CSV to JSONL https://github.com/jqnatividad/qsv/pull/380
* `excel`: additional --date-whitelist modes https://github.com/jqnatividad/qsv/pull/368
* `fetch`: added Redis connection pooling https://github.com/jqnatividad/qsv/pull/373

### Changed
* `python`: remove unneeded python3.dll generation https://github.com/jqnatividad/qsv/pull/379
* `stats`: minor performance tweaks
* `fetch`: minor performance tweaks - larger/faster in-mem cache
* Bump cached from 0.34.1 to 0.37.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/367 and https://github.com/jqnatividad/qsv/pull/381
* Bump regex from 1.5.6 to 1.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/369
* Bump reverse_geocoder from 3.0.0 to 3.0.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/377
* Bump actions/setup-python from 4.0.0 to 4.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/376
* Bump serde from 1.0.138 to 1.0.139 by @dependabot in https://github.com/jqnatividad/qsv/pull/374
* cargo update bump several dependencies
* larger logfiles (from 1mb to 10mb) before rotating
* apply select clippy recommendations
* pin Rust nightly to 2022-07-13

### Fixed
* Use option_env! macro to trap errors https://github.com/jqnatividad/qsv/pull/378

## [0.58.2] - 2022-07-02
### Changed
* Pin Rust nightly to 2022-07-02

### Fixed
* fixed redis dev-dependency which mistakenly added a non-existent ahash feature. This prevented publishing of qsv 0.58.1 to crates.io.

## [0.58.1] - 2022-07-02
### Changed
* Universal clippy handling. Added allow clippy hint section in main for clippy lints we allow/ignore, and added exceptions as needed throughout the codebase.
This means clippy, even in pedantic/nursery/perf mode will have no warnings. https://github.com/jqnatividad/qsv/pull/365
* reqwest deflate compression support https://github.com/jqnatividad/qsv/pull/366
* `fetch`: expanded --http-header explanation/example
* `fetch`: refactored --timeout processing https://github.com/jqnatividad/qsv/commit/3454ed068f0f243473a0f66520f90f55ece4bf49
* `fetch`: prioritized ACCEPT-ENCODING to prioritize brotli first, gzip second, and deflate last for compression https://github.com/jqnatividad/qsv/commit/c540d22b630df424a8516bb07af9bbf80150d67b
* updated patched crates, particularly our rust-csv fork with more clippy recommendations applied
* cargo update bump actix-http from 3.2.0 to 3.2.1

### Fixed
* `excel`: fixed docopt usage text which prevents --help from working
* `extsort`: better parsing/error-handling, instead of generic panic when no input/output is specified. This also allows --help to be displayed.

## [0.58.0] - 2022-07-02
### Added
* `excel`: add --list-sheets option https://github.com/jqnatividad/qsv/pull/364
* `fetch`: added 0 option to --rate-limit to go as fast as possible.  
**CAUTION:** Only use this with APIs that have [RateLimit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html) headers so qsv can automatically down-throttle as required. Otherwise, the fetch job will look like a Denial-of-Service attack. https://github.com/jqnatividad/qsv/commit/e4ece60aea3720b872119ca7a8ad3666dad033e7
* `fetch`: added --max-errors option. Maximum number of errors before aborting

### Changed
* progress bars now display per_sec throughput while running jobs, not just at the end of a job
* `fetch`: for long-running fetch jobs, the progress bar will update at least every three seconds, so it doesn't look like the job is frozen/stuck.
* `fetch`: added additional verbiage to usage text on how to pass multiple key-value pairs to the HTTP header
* `fetch`: made RateLimit jitters (required to avoid [thundering herd](https://en.wikipedia.org/wiki/Thundering_herd_problem) issues as per the [RateLimit spec](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html#resource-exhaustion-and-clock-skew)) shorter, as they were too long.
* pin Rust nightly to 2022-07-01
* applied various clippy recommendations
* bumped serde from 1.0.137 to 1.0.138
* added stale warning to benchmarks. The benchmarks have not been updated since qsv 0.20.0.
* cargo update bumped several other dependencies

### Fixed
* remove unneeded sleep pause before fetch ratelimit test

## [0.57.1] - 2022-06-31
### Changed
* `fetch`: higher default settings which makes fetch much faster

## [0.57.0] - 2022-06-30
### Added
* `excel`: date support https://github.com/jqnatividad/qsv/pull/357
* added hardware survey reminiscent of [Steam's Hardware Survey](https://store.steampowered.com/hwsurvey). Only sent when checking for updates with no personally identifiable information. https://github.com/jqnatividad/qsv/pull/358
* `fetch`: ensure URLs are properly encoded https://github.com/jqnatividad/qsv/pull/359

### Changed
* Bump jql from 4.0.4 to 4.0.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/356
* cargo bump update several dependencies
* change MSRV to Rust 1.62.0
* pin Rust Nightly to 2022-06-29

### Fixed
* `fetch`: is single-threaded again. It turns out it was more complicated than I hoped. Will revisit making it multithreaded once I sort out the sync issues.

## [0.56.0] - 2022-06-20
### Added
* `fetch` is now multithreaded! 🚀🚀🚀 - with threadsafe memoized caching, dynamic throttling & http2 adaptive flow control https://github.com/jqnatividad/qsv/pull/354

### Changed
* `fetch`: do more expensive ops behind cache https://github.com/jqnatividad/qsv/pull/355
* applied BetterTOML formatting to Cargo.toml
* `exclude`, `flatten` & `join`: applied clippy recommendation for borrow_deref_ref https://github.com/jqnatividad/qsv/commit/bf1ac90185947a6d923613f17c4af616631dc149
* `utils`: minor cleanup of version fn https://github.com/jqnatividad/qsv/commit/217702b51785f51d6924608a5122c405ff384fef
* `validate`: perf tweak - use collect_into_vec to reduce allocations
* `apply`: perf tweak - use collect_into_vec to reduce allocations
* removed `thiserror` dependency
* pin Rust Nightly to 2022-06-19
* Bump robinraju/release-downloader from 1.3 to 1.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/351
* Bump crossbeam-channel from 0.5.4 to 0.5.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/352
* Bump redis patch
* cargo update bump several other dependencies

### Fixed
* `fetch`: better error handling https://github.com/jqnatividad/qsv/pull/353


###
## [0.55.5] - 2022-06-16
### Changed
* `fetch`: performance tweaks https://github.com/jqnatividad/qsv/pull/350
* Bump titlecase from 1.1.0 to 2.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/349
* Bump sysinfo from 0.24.3 to 0.24.4

### Fixed
* `fetch`: convert non-persistent cache from an Unbound cache to a Sized LRU cache, 
so we don't run out of memory if the file being processed is very large and cache hits are low.
https://github.com/jqnatividad/qsv/commit/4349fc9389a32c0d9544be824d1f42b1af65974d

## [0.55.4] - 2022-06-15
### Changed
* `fetch`: preemptively throttle down before we hit the ratelimit quota

## [0.55.3] - 2022-06-15
### Added
* `fetch`: add "dynamic throttling". If response header has [rate-limit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html) or [retry-after](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After) fields, fetch will dynamically throttle itself as needed. https://github.com/jqnatividad/qsv/pull/348

### Changed
* cargo update bump dependencies
* Pin Rust nightly to 2022-06-14

## [0.55.2] - 2022-06-14
### Changed
* `fetch`: more robust/consistent error handling https://github.com/jqnatividad/qsv/pull/347
* removed reqwest 0.11.10 patch and used reqwest 0.11.11
* Pin Rust nightly to 2022-06-13

## [0.55.1] - 2022-06-13
### Changed
* Pin Rust nightly to 2022-06-12

### Fixed
* `fetch`: fix invalid jsonl response https://github.com/jqnatividad/qsv/pull/346

## [0.55.0] - 2022-06-12
### Added
* `apply`: now multithreaded with rayon (up to 10x 🚀🚀🚀 faster!) https://github.com/jqnatividad/qsv/pull/342

### Changed
* `apply`: refactor hot loop to use enums instead of nested if https://github.com/jqnatividad/qsv/pull/343
* `sniff`: more idiomatic vec loop https://github.com/jqnatividad/qsv/commit/2a70134bf45f9485bcbb27579f92f89abb7b6bb1
* `validate`: optimizations (up to 20% 🚀 faster) https://github.com/jqnatividad/qsv/commit/0f0be0aba0a6d0cd10f5c96fd17ffd726d3231d1
* `excel`: optimize trimming https://github.com/jqnatividad/qsv/commit/780206a575d40cf759abd295aa91da640e5febed
* various whirlwind tour improvements (more timings, flows/reads better, removed non-sequiturs)
* improved progress bar prep (unstyled progress bar is not momentarily displayed, standardized across cmds)
* bumped reqwest patch to latest upstream https://github.com/jqnatividad/qsv/commit/cb0eb1717f07d8481211e289e6762d9b994fac18
* Bump actions/setup-python from 3.1.2 to 4.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/339
* Bump mlua from 0.7.4 to 0.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/340

### Fixed
* fixed error-handling in util::count_rows()  https://github.com/jqnatividad/qsv/pull/341
* do not panic when index is stale https://github.com/jqnatividad/qsv/commit/36dbd79591e3ae1e9c271ec3c0272599cc8695de
* `fetch`: fixed docopt arg processing so --help text is displayed properly https://github.com/jqnatividad/qsv/commit/0cbf7017ebc7f28fa67951133e3bac7d2c7a1368
* `excel`: more robust error handling https://github.com/jqnatividad/qsv/commit/413c693320653d085b5cca48ca32b0d371ccd240

## [0.54.0] - 2022-06-08

### Added
* `stats`: added [outer fences](https://www.statisticshowto.com/upper-and-lower-fences/) to help identify extreme and mild outliers  https://github.com/jqnatividad/qsv/pull/337

### Changed
* `stats`: change skewness algorithm to use [quantile-based measures](https://en.wikipedia.org/wiki/Skewness#Quantile-based_measures)
* whirlwind tour: added more stats about stats command; updated stats output with the additional columns
* pin nightly to 2022-06-07
* cargo update bump several dependencies

### Fixed
* fixed stats quartile tests, as the results were being prematurely truncated, causing in false negative test results

## [0.53.0] - 2022-06-05

### Changed
* `stats`: changed `--dates-whitelist` option to use "all" instead of "\<null\>"; better usage text; more perf tweaks; more tests https://github.com/jqnatividad/qsv/pull/334
* `stats`: mem alloc tweaks & date-inferencing optimization https://github.com/jqnatividad/qsv/pull/333
* `apply`: improved usage text about --formatstr https://github.com/jqnatividad/qsv/commit/2f18565caec6c6e900f776c5f6f3e1adf4c9b6e1
* `sample`: added note about why we don't need crypto secure random number generators https://github.com/jqnatividad/qsv/commit/3384d1a9630bc1033ff67db5dcbf48c067e97728
* `excel` & `slice`: avoid panic by replacing `abs` with `unsigned_abs` https://github.com/jqnatividad/qsv/commit/7e2b14a5de67e70ee0b26ea0eff83462dbc77a0a
* turn on once_cell `parking_lot` feature for storage efficiency/performance https://github.com/jqnatividad/qsv/commit/849548cde8bc9c2d96ddf464f2578faf63d6e9cf
* applied various cargo +nightly clippy optimizations
* pin nightly build to Rust Nightly 2022-06-04
* made various optimizations to our csv fork https://github.com/BurntSushi/rust-csv/compare/master...jqnatividad:perf-tweaks
* cargo bump updated several dependencies

## [0.52.2] - 2022-06-01
### Added
* added `QSV_PREFER_DMY` environment variable. https://github.com/jqnatividad/qsv/pull/331

### Changed
* reorganized Environment Variables section in README https://github.com/jqnatividad/qsv/commit/f25bbf0361fcb7b960d45590ca35b2e676a4497d
* logging: longer END snippet to make it easier to match START/END pairs
* added Boston 311 sample data to tests
* Bump uuid from 1.1.0 to 1.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/332
* cargo update bumped packed_simd_2 from 0.3.7 to 0.3.8

### Fixed
* Instead of panicking, do proper error-handling on IO errors when checking utf-8 encoding https://github.com/jqnatividad/qsv/pull/331/commits/37b4482aae77563995f13a15f73ca8849df6a27d

## [0.52.1] - 2022-05-31
### Added
* added qsv GitHub social media image which

### Changed
* `stats`: added sum integer overflow handling. If sum overflows, instead of panicking, the value 'OVERFLOW' is returned
* upgraded to faster qsv_dateparser 0.4.2, which parses the slash_dmy/slash_mdy date formats earlier in the parse tree, which has more prevalent usage.
* nightly builds are now bundled into the main distribution zip archive.
* renamed qsv_rust_version_info.txt to qsv_nightly_rust_version.info.txt in the distribution zip archive to make it clearer that it only pertains to nightly builds
* cargo bump update several dependencies

### Removed
* nightly distribution zip archives have been removed, now that the nightly builds are in the main zip archive.

### Fixed
* `stats`: prefer_dmy date-parsing preference was not used when computing date min/max
* `stats`: prefer_dmy setting was not initialized properly the first time its called
* nightly build self-update now works properly, now that they are bundled into the main distribution zip archive

## [0.52.0] - 2022-05-29
### Added
* `apply`: DATEFMT subcommand now has a `--prefer-dmy` option. https://github.com/jqnatividad/qsv/pull/328
* `stats` and `schema`: add `--prefer-dmy` option. https://github.com/jqnatividad/qsv/pull/329
* `sniff`: can now sniff Date and Datetime data types.  https://github.com/jqnatividad/qsv/pull/330
* `sniff`: added to `qsvdp` - [DataPusher+](https://github.com/dathere/datapusher-plus)-optimized qsv binary
* added DevSkim security linter Github Action to CI

### Changed
* applied various clippy pedantic and nursery recommendations
* cargo bump updated several dependencies, notably [qsv-dateparser](https://docs.rs/qsv-dateparser/0.4.1/qsv_dateparser/) with its new DMY format parsing capability and
  [qsv-sniffer](https://github.com/jqnatividad/qsv-sniffer) with its Date and Datetime data type detection

### Fixed
* Closed all cargo-audit findings(https://github.com/jqnatividad/qsv/issues/167), as the latest `qsv-dateparser` eliminated qsv's `chrono` dependency.
* Properly create `qsv_rust_version_info.txt` in nightly builds
* Fixed multithreading link in Features Flag section

## [0.51.0] - 2022-05-27
### Added
* `sniff`: sniff field names as well in addition to field data types in https://github.com/jqnatividad/qsv/pull/317
* `sniff`: intelligent sampling. In addition to specifying the number of first n rows to sample, when `--sample`
is between 0 and 1 exclusive, its treated as a percentage of the CSV to sample (e.g. 0.20 is 20 percent).
If its zero, the entire file is sampled. https://github.com/jqnatividad/qsv/pull/318
* `schema`: add --stdout option in https://github.com/jqnatividad/qsv/pull/321
* `stats`: smart date inferencing with field-name date whitelist. Also did some minor tweaks for a little more performance in https://github.com/jqnatividad/qsv/pull/327
* `rename`: added to `qsvdp` - [DataPusher+](https://github.com/dathere/datapusher-plus)-optimized qsv binary 

### Changed
* Switch to qsv_sniffer fork of csv_sniffer. [qsv_sniffer](https://github.com/jqnatividad/qsv-sniffer) has several optimizations (field name sniffing, utf-8 encoding detection, 
SIMD speedups, [etc.](https://github.com/jqnatividad/qsv-sniffer/releases/tag/0.4.0)) that enabled the added `sniff` features above. https://github.com/jqnatividad/qsv/pull/320
* Bump uuid from 1.0.0 to 1.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/323
* Improved Performance Tuning section with more details about UTF-8 encoding, and Nightly builds
* Updated list of commands that use an index
* cargo update bump dependencies, notably jql 4.0.3 to 4.0.4, and cookie_store from 0.16.0 to 0.16.1

### Fixed
* pinned Rust Nightly to 2022-05-23. Later Rust Nightly releases "broke" packed-simd dependency
which prevented us from building qsv's nightly build. (see https://github.com/apache/arrow-rs/issues/1734)

## [0.50.1] - 2022-05-23
### Changed
* disable simd acceleration feature on our csv-sniffer fork so we can publish on crates.io

## [0.50.0] - 2022-05-23
### Added
* `input`:  added `--auto-skip` CSV preamble option in https://github.com/jqnatividad/qsv/pull/313
* `sniff`: support non-utf8 files; flexible detection now works; rename --len to --sample in https://github.com/jqnatividad/qsv/pull/315
* `sniff`: added `is_utf8` property in https://github.com/jqnatividad/qsv/pull/316
* added RFC4180 section to README

### Changed
* `validate`: improve RFC4180 validation messages in https://github.com/jqnatividad/qsv/pull/309
* `stats`: nullcount is a "streaming" statistic and is now on by default in https://github.com/jqnatividad/qsv/pull/311
* `schema`: refactored stdin processing 
* Made logging more consistent in https://github.com/jqnatividad/qsv/pull/314
* bumped MSRV to Rust 1.61.0
* use a qsv-optimized fork of csv-sniffer (https://github.com/jqnatividad/csv-sniffer/tree/non-utf8-qsv), that fixes flexible detection,
  reads non-utf8 encoded files, reports if a file is utf8-encoded, and uses SIMD/CPU features to accelerate performance.
* applied select pedantic clippy recommendations
* bumped several dependencies, notably regex from 1.5.5 to 1.5.6

### Fixed
* `py`: enabled `abi3` feature properly, so qsv now works with higher versions of python over v3.8

## [0.49.0] - 2022-05-17
### Added
* `validate`: add `--json` & `--pretty-json` options for RFC4180 check in https://github.com/jqnatividad/qsv/pull/303
* `qsvdp`: add `validate` command in https://github.com/jqnatividad/qsv/pull/306
* added rust nightly version info to nightly builds

### Changed
* apply select clippy::pedantic recommendations in https://github.com/jqnatividad/qsv/pull/305
* Bump actions/checkout from 2 to 3 by @dependabot in https://github.com/jqnatividad/qsv/pull/300
* `sniff` and `validate` json errors are now JSONAPI compliant
* cargo update bump several dependencies

### Removed
* removed unused debian package publishing workflow 

### Fixed
* `sniff`: preamble and rowcount fixes in https://github.com/jqnatividad/qsv/pull/301
* `schema`: fixed stdin bug in https://github.com/jqnatividad/qsv/pull/304

## [0.48.1] - 2022-05-16
### Fixed:
* Fixed conditional compilation directives that caused qsvdp build to fail.

## [0.48.0] - 2022-05-15
### Added
* `dedup`: add `--sorted` option in https://github.com/jqnatividad/qsv/pull/286
* `sniff`: add `--json` and `--pretty-json` options in https://github.com/jqnatividad/qsv/pull/297
* added rust version info to nightly build zip files so users can see which Rust nightly version was used to build the nightly binaries

### Changed:
* `stats`: added more `--infer-dates` tests
* number of processors used now logged when logging is on
* `python`: nightly build optimization in https://github.com/jqnatividad/qsv/pull/296
* moved Performance Tuning to its own markdown file, and included it in the TOC
* bumped several dependencies, notably `rayon`, `jsonschema` and `pyo3`
* moved FAQ from Wiki to Discussions
* added clone count badge

### Fixed:
* `python`: should now work with python 3.8, 3.9.or 3.10

## [0.47.0] - 2022-05-12
### Added
* `dedup` and `sort` are now multithreaded with rayon in https://github.com/jqnatividad/qsv/pull/283
* add `--jobs` option to `schema` and `validate` in https://github.com/jqnatividad/qsv/pull/284

### Changed
* `--jobs` and `QSV_MAX_JOBS` settings also now work with rayon
* cargo update bump several dependencies
* upgrade `calamine` fork patch that enables `excel` command
* removed `target-cpu=native` in nightly builds so they are more portable

### Fixed
* fixed `publish-nightly` workflow bugs so nightly builds are built properly
* corrected several build instructions errors in README
* fixed `workdir:output_stderr()` helper so it also returns std_err message
* fixed `Rust Beta` workflow so we can also manually test against Rust Beta

## [0.46.1] - 2022-05-08
### Changed
* `extsort`: increased performance. Use 10% of total memory or if total mem is not detectable, 100 mb for in-mem sorting. Increased R/W buffer size to 1mb [e2f013f](https://github.com/jqnatividad/qsv/commit/e2f013f267ce0add457a3a64bc16b9924c142a05)
* `searchset`: more idiomatic rust [fa1f340](https://github.com/jqnatividad/qsv/commit/fa1f340c3084cea548008ec204ec12bc67c60ad7)
* added "Nightly Release Builds" section in README Performance Tuning
* cargo update bump several dependencies

### Fixed
* `excel`: fixed off by +1 row count (we were counting the header as well); added column count to final message and removed useless human-readable option. [c99df2533b5c112d90c6e04068227b7f873459c2](https://github.com/jqnatividad/qsv/commit/c99df2533b5c112d90c6e04068227b7f873459c2)
* fixed various bugs in Publish Nightly GitHub Action that automatically built nightly binaries

## [0.46.0] - 2022-05-07
### Added
* Added release nightly binaries, optimized for size and speed
   * uses Rust nightly
   * also compiles stdlib, so build-time optimizations also apply, instead of using pre-built stdlib
   * set `panic=abort` - removing panic-handling, formatting and backtrace code from binaries
   * set `RUSTFLAGS= -C target-cpu=native` to enable use of additional CPU-level features
   * enables unstable/nightly features on `regex` and `rand` crates
* Added testing on nightly to CI

### Changed
* `dedup`: reduced memory footprint by half by writing directly to disk, rather than storing in working mem, before writing
* `excel`: show sheet name in message along with row count; let docopt take care of validating mandatory arguments
* More whirlwind tour improvements - how timings were collected, footnotes, etc.
* Bump github/codeql-action from 1 to 2 by @dependabot in https://github.com/jqnatividad/qsv/pull/277
* Bump log from 0.4.16 to 0.4.17 by @dependabot in https://github.com/jqnatividad/qsv/pull/278
* Bump whatlang from 0.15 to 0.16
* Make file extension processing case-insensitive in https://github.com/jqnatividad/qsv/pull/280
* Added Caching section to Performance Tuning
* Added UTF-8 section to Performance Tuning

### Removed
* removed unneeded header file for wcp.csv used in Whirlwind Tour, now that we have a well-formed wcp.csv

## [0.45.2] - 2022-05-01
### Added
* added `headers` command to qsvdp binary

### Changed
* cargo update bump semver from 1.0.7 to 1.0.8

## [0.45.1] - 2022-05-01
### Added
* added rust-clippy GH action workflow
* added security policy

### Changed:
* `extsort`: use util::njobs to process --jobs option
* various improvements on Whirlwind tour to help users follow along
* `extsort`: add link to "External Sorting" wikipedia article
* `extsort`: made <input> and <output> mandatory docopt arguments 
* `sort`: mention `extsort` in usage text
* added markdownlint.json config to suppress noisy markdown lints in VSC
* reformatted README to apply some markdown lints
* bump whatlang from 0.14 to 0.15
* bump qsv-stats from 0.3.6 to 0.3.7 for some minor perf improvements

## [0.45.0] - 2022-04-30
### Added
* Added `extsort` command - sort arbitrarily large text files\CSVs using a multithreaded external sort algorithm.

### Changed
* Updated whirlwind tour with simple `stats` step
* `py`: Automatically create python3.dll import libraries on Windows targets
* Updated build instructions to include `full` feature
* `index`: mention QSV_AUTOINDEX env var in usage text
* Corrected minor typos
* Bump jql from 4.0.1 to 4.0.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/276
* cargo update bump several dependencies - notably mimalloc

## [0.44.0] - 2022-04-27
### Added
* Created new binary - qsvdp - binary optimized for [DataPusher+](https://github.com/dathere/datapusher-plus) in https://github.com/jqnatividad/qsv/pull/273
  qsvdp only has DataPusher+ relevant commands, with the self-update engine removed. This results in a binary that's
  3x smaller than qsvlite, and 6x smaller than qsv will all features enabled.

### Changed
* `dedup`: send dupe count to stderr in https://github.com/jqnatividad/qsv/pull/272
* `dedup`: improve usage text
* cargo update bump several crates

### Fixed
* `count`: corrected usage text typo

## [0.43.0] - 2022-04-26
### Added
* `input` can now effectively transcode non-utf-8 encoded files to utf-8 in https://github.com/jqnatividad/qsv/pull/271

### Changed
* `table`: made it flexible - i.e. each row can have varying number of columns
* `excel`: remove unneeded closure

## [0.42.2] - 2022-04-25
### Changed
* use our grex fork, as the upstream fork has an unpublished version number that prevents us from publishing on crates.io.

## [0.42.1] - 2022-04-25
### Changed
* use `[patch.crates-io]` to use crate forks, rather than using the git directive in the dependencies section.
This has the added benefit of making the dependency tree smaller, as other crates that depend on the patched crates also
use the patches. This should also result in smaller binaries.

## [0.42.0] - 2022-04-24
### Added
* `input` refactor. Added trimming and epilog skiplines option. https://github.com/jqnatividad/qsv/pull/270
* `sniff`: added note about sniff limitations
* also publish x86_64-unknown-linux-musl binary

### Changed
* Bump anyhow from 1.0.56 to 1.0.57 by @dependabot in https://github.com/jqnatividad/qsv/pull/268
* Bump jsonschema from 0.15.2 to 0.16.0
* use optimized fork of rust-csv, with non-allocating, in-place trimming and various perf tweaks
* use optimized fork of docopt.rs, with various perf & memory allocation tweaks
* use reqwest fork with unreleased changes that remove unneeded crates
* `validate`: use `from_utf8_unchecked` in creating json instances for performance

### Fixed
* `input`: Fixed line-skipping logic so CSV parsing is flexible - i.e. column count can change between records

## [0.41.0] - 2022-04-21
### Added
* `input`: add `--skip-lines` option in https://github.com/jqnatividad/qsv/pull/266

### Changed
* More verbose, matching START/END logging messages when `QSV_LOG_LEVEL` is enabled.
* Bump whatlang from 0.13.0 to 0.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/264
* Bump filetime from 0.2.15 to 0.2.16 by @dependabot in https://github.com/jqnatividad/qsv/pull/263
* Bump uuid from 0.8 to 1 in https://github.com/jqnatividad/qsv/pull/267
* Minor documentation improvements
* `cargo update` bumped several other second-level dependencies

## [0.40.3] - 2022-04-14
### Changed
* Bump pyo3 from 0.16.3 to 0.16.4
* `stats`: renamed `--dates` option to `--infer-dates`
### Fixed
* `stats`: fixed panic caused by wrong type inference when `--infer-dates` option is on in https://github.com/jqnatividad/qsv/pull/256

## [0.40.2] - 2022-04-14
### Changed
* Datapusher tweaks, primarily to help with datapusher error-handling in https://github.com/jqnatividad/qsv/pull/255
* `excel`: exported count with `--human-readable` option
* use calamine fork to bump dependencies, and reduce binary size
* Bump rayon from 1.5.1 to 1.5.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/254
* Bump jql from 4.0.0 to 4.0.1

### Fixed
* removed unnecessary *.d dependency files from published binaries zip

## [0.40.1] - 2022-04-13
### Changed
* use performance tweaked forks of csv crate
* Made `this_error` dependency optional with `fetch` feature 
* Made `once_cell` dependency optional  with `apply` and `fetch` features

### Fixed
* Fixed qsv binary publishing. qsv binary was not built properly, it was built using a qsvlite profile

## [0.40.0] - 2022-04-12
### Added
* `excel` command in https://github.com/jqnatividad/qsv/pull/249 and https://github.com/jqnatividad/qsv/pull/252

### Changed
* Bump jql from 3.3.0 to 4.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/251
* Bump actions/setup-python from 3.1.1 to 3.1.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/250

## [0.39.1] - 2022-04-11
### Fixed
* added version to grex dependency as its required by crates.io, though we're still using the grex fork without the CLI components.

## [0.39.0] - 2022-04-10
### Added
* `QSV_AUTOINDEX` environment variable. When set, autoindexes csv files, autoupdates stale indices 
* `replace`: \<NULL\> `--replacement` option (https://github.com/jqnatividad/qsv/pull/244)
* qsv now automatically screens files for utf-8 encoding. Set `QSV_SKIPUTF8_CHECK` env var to skip encoding check. (https://github.com/jqnatividad/qsv/pull/245 and https://github.com/jqnatividad/qsv/pull/248)

### Changed
* `foreach`: refactored. (https://github.com/jqnatividad/qsv/pull/247)
* Bump jql from 3.2.3 to 3.3.0
* Bump actions/setup-python from 3.1.0 to 3.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/246
* use grex fork to remove unneeded CLI dependencies

## [0.38.0] - 2022-04-05
### Changed
* qsv **requires** UTF-8/ASCII encoded files. Doing so allows us to squeeze more performance by removing UTF-8 validation in https://github.com/jqnatividad/qsv/pull/239 and https://github.com/jqnatividad/qsv/pull/240
### Fixed
* fixed `--jobs` parameter parsing for multithreaded commands in https://github.com/jqnatividad/qsv/pull/236 and https://github.com/jqnatividad/qsv/pull/237

## [0.37.2] - 2022-04-03
### Fixed
* Handle/log self-update errors in https://github.com/jqnatividad/qsv/pull/233

## [0.37.1] - 2022-04-03
### Changed
* `fetch` and `apply`: use cheaper, faster lookup tables for dynamic formatting in https://github.com/jqnatividad/qsv/pull/231
* Cleanup - remove commented code; convert `match` to `if let`; more pedantic clippy recommendations, etc. in https://github.com/jqnatividad/qsv/pull/232

## [0.37.0] - 2022-04-02
### Added
* `enumerate`: added `--constant` <NULL> sentinel value in https://github.com/jqnatividad/qsv/pull/219
* `fetch`: added `--jqlfile` option in https://github.com/jqnatividad/qsv/pull/220
* `stats`: more perf tweaks in https://github.com/jqnatividad/qsv/pull/223

### Changed
* `fetch`: argument parsing refactor, removing need for dummy argument in https://github.com/jqnatividad/qsv/pull/222
* applied select pedantic clippy recommendations in https://github.com/jqnatividad/qsv/pull/224
* simplified multithreading - removed jobs div by three heuristic in https://github.com/jqnatividad/qsv/pull/225
* use qsv-dateparser fork of dateparser for increased performance of `stats`, `schema` and `apply` in https://github.com/jqnatividad/qsv/pull/230
* Bump actions/checkout from 2.3.3 to 3 by @dependabot in https://github.com/jqnatividad/qsv/pull/228
* Bump actions/stale from 3 to 5 by @dependabot in https://github.com/jqnatividad/qsv/pull/227
* Bump actions/setup-python from 2 to 3.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/226

## [0.36.1] - 2022-03-26
### Changed
* `validate`: use user agent & compression settings when fetching jsonschema from a URL in https://github.com/jqnatividad/qsv/pull/207
* Build and publish smaller qsvlite binary in https://github.com/jqnatividad/qsv/pull/208, https://github.com/jqnatividad/qsv/pull/210 & https://github.com/jqnatividad/qsv/pull/213
* `sniff`: now works with stdin in https://github.com/jqnatividad/qsv/pull/211 and https://github.com/jqnatividad/qsv/pull/212
* `stats`: remove smartstring in https://github.com/jqnatividad/qsv/pull/214
* various performance tweaks in `stats` and `select`
### Fixed
* README: Installation - git:// is no longer supported by GitHub  by @harrybiddle in https://github.com/jqnatividad/qsv/pull/205
* README: Fixed wrong footnote for feature flags
* Silent error when an index file is not found is now logged (https://github.com/jqnatividad/qsv/commit/7f2fe7f3259fb74a8d76396dcc2aa71585967b9b)
* bumped self-update to 0.29. This partly addresses #167, as self-update had an indirect dependency to `time` 0.1.43.

## [0.36.0] - 2022-03-20
### Added
* `sniff`: new command to quickly detect CSV metadata in https://github.com/jqnatividad/qsv/pull/202
* auto-delimiter setting with `QSV_SNIFF_DELIMITER` environment variable in https://github.com/jqnatividad/qsv/pull/203
* `apply`: new `dynfmt` multi-column, dynamic formatting subcommand in https://github.com/jqnatividad/qsv/pull/200
* `fetch`: new multi-column dynamic formatting with --url-template option in https://github.com/jqnatividad/qsv/pull/196
### Changed
* `fetch`: --url-template safety tweaks in https://github.com/jqnatividad/qsv/pull/197
* `fetch`: automatically minify JSON responses. JSON can still be pretty-printed with --pretty option in https://github.com/jqnatividad/qsv/pull/198
* `fetch` is now an optional feature in https://github.com/jqnatividad/qsv/pull/201
* `sniff`: improved display in https://github.com/jqnatividad/qsv/pull/204
* slim down dev-dependencies
### Fixed:
* `py`: now checks if first character of a column is a digit, and replaces it with an underscore

## [0.35.2] - 2022-03-13
### Added
* README: Added datHere logo
### Changed
* `py`: ensure valid python variable names https://github.com/jqnatividad/qsv/pull/192
* `fetch`: dev-dependency actix upgrade (actix-governor from 0.2->0.3; actix-web from 3.3->4.0) https://github.com/jqnatividad/qsv/pull/193
* `lua`: replace hlua with mlua  https://github.com/jqnatividad/qsv/pull/194
* `stats`: refactor for performance - skip from_utf8 check as input is utf8 transcoded as necessary; smartstring https://github.com/jqnatividad/qsv/pull/195
* Whirlwind Tour: show country-continent.csv file with comment handling
* cargo bump update several dependencies

### Fixed
* `stats`: only compute quartiles/median for int/float fields - https://github.com/jqnatividad/qsv/pull/195

## [0.35.1] - 2022-03-08

### Changed
- README: note about `--output` option changing delimiter automatically based on file extension and UTF-8 encoding the file
- README: Windows usage note about UTF16-LE encoding and `--output` workaround
### Fixed
* upgraded regex to 1.5.5 which resolves the [GHSA-m5pq-gvj9-9vr8 security advisory](https://github.com/rust-lang/regex/security/advisories/GHSA-m5pq-gvj9-9vr8)

## [0.35.0] - 2022-03-08
### Added
* `count`: `--human-readable` option in https://github.com/jqnatividad/qsv/pull/184
* Automatic utf8 transcoding in https://github.com/jqnatividad/qsv/pull/187
* Added NYC School of Data 2022 presentation
* Added ahash 0.7 and encoding_rs_io 0.1 dependencies

### Changed
* Use ahash::AHashMap instead of std::collections::HashMap for performance in https://github.com/jqnatividad/qsv/pull/186
* Revamped Whirlwind Tour
* bumped several dependencies 
  * anyhow 1.0.55 to 1.0.56
  * ipnet 2.3.1 to 2.4.0
  * pyo3 0.16.0 to 0.16.1

### Fixed
* `py`: convert spaces to underscores for valid python variable names when Column names have embedded spaces in https://github.com/jqnatividad/qsv/pull/183
* docs: CSV Kit got a 10x improvement by @jpmckinney in https://github.com/jqnatividad/qsv/pull/180
* `fetch`: added jql selector to cache key
* Corrected README mixup re `join` hashmap indices and qsv indices

## New Contributors
* @jpmckinney made their first contribution in https://github.com/jqnatividad/qsv/pull/180

## [0.34.1] - 2022.03-04
### Added
* `stats`: added `--dates` option. This option turns on date/datetime data type inferencing, which is 
a very expensive operation. Only use this option when you have date/datetime fields and you want to 
compile the proper statistics for them (otherwise, they will be treated as "String" fields.)

## [0.34.0] - 2022.03-03
### Added
* added intentionally kitschy qsv logo :grin:
* `stats`: added `datetime` data type inferencing
* `fetch`: added optional Redis response caching
* `schema`: added `--strict-dates` option by @mhuang74 in https://github.com/jqnatividad/qsv/pull/177 
* `validate`: added more robust [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180)-compliance checking when no jsonschema is provided
* added Redis to CI

### Changed
* bumped reverse-geocoder crate from 2.0.1 to 3.0.0 to modernize geonames reverse geocoder
* bumped cached crate from 0.30.0 to 0.33.0 to enable Redis response caching
* bumped various other dependencies to latest release

### Fixed
* removed invalid `--path` cargo install option in README
* `workdir.rs` was not properly cleaning up test files

## [0.33.0] - 2022.02-27
### Added
* `fetch`: add `--url-template` and `--redis` options in https://github.com/jqnatividad/qsv/pull/175
* `stats`: add `DateTime` data type (RFC3339 format) in https://github.com/jqnatividad/qsv/pull/176
* added Rust Beta to Github Actions CI

### Changed
* `validate`: improve performance and simplify error report format by @mhuang74 in https://github.com/jqnatividad/qsv/pull/172
* Addl `validate` performance tweaks in https://github.com/jqnatividad/qsv/pull/173
* changed MSRV to latest Rust stable - 1.59.0
* removed `num_cpus` crate and use new `std::thread::available_parallelism` stabilized in Rust 1.59.0
* use new cargo.toml `strip` option to strip binaries
* refactored GitHub Actions CI to make it faster

## [0.32.2] - 2022-02-20
### Changed
* `schema` (#60): pattern constraint for string types by @mhuang74 in https://github.com/jqnatividad/qsv/pull/168
* `validate`: improve performance by @mhuang74 in https://github.com/jqnatividad/qsv/pull/170
* `fetch`: Spell out k:v -> key:value in docopt usage text
* cargo update bump several dependencies

### Fixed
* `validate`: bug fix and refactor by @mhuang74 in https://github.com/jqnatividad/qsv/pull/171

## [0.32.1] - 2022-02-14
### Changed
* `fetch`: upgrade to jql 3.1.0 by @mhuang74 in https://github.com/jqnatividad/qsv/pull/160
* `schema`: refactor tests by @mhuang74 in https://github.com/jqnatividad/qsv/pull/161
* `schema`: support Enum constraint by @mhuang74 in https://github.com/jqnatividad/qsv/pull/162
* `schema`: default to include value constraints  by @mhuang74 in https://github.com/jqnatividad/qsv/pull/166
* bumped `qsv-stats` to 0.3.6 for `stats` & `frequency` performance tweaks
* specify that `apply geocode` expects WGS84 coordinate system
* cargo update bump several dependencies
* changed CI to run clippy and rustfmt automatically

### Fixed
* `schema`: Fix bug with enum by @mhuang74 in https://github.com/jqnatividad/qsv/pull/163

## [0.32.0] - 2022-02-06
### Added
* `schema` POC by @mhuang74 in https://github.com/jqnatividad/qsv/pull/155
* `schema`: add value constraints via stats  by @mhuang74 in https://github.com/jqnatividad/qsv/pull/158
* `schema`: update command description by @mhuang74 in https://github.com/jqnatividad/qsv/pull/159

### Changed
* `stats` data type inference changed to more straightforward "String" from "Unicode"
* changed CI/CD to use rust-cache GitHub Actions making it ~3x faster.
* always build and test with `--locked` flag. This allows us to use rust-cache and guarantee that
  builds are using the exact dependency versions qsv requires.
* bumped `qsv-stats` to 0.3.5 for `stats` performance tweaks  

### Fixed
* Validate: bug fixes by @mhuang74 in https://github.com/jqnatividad/qsv/pull/154

## [0.31.0] - 2022-01-31
### Changed
* Validate: bug fixes by @mhuang74 in https://github.com/jqnatividad/qsv/pull/151
* Python 3.8 (current stable version) is now required for the `py` command. Changed from Python 3.7.
* bumped jsonschema dependency to to 0.15.
* always build/publish with `--locked` flag in CI/CD.
* enclose environment variable values with double quotes when using `--envlist` option
* use more captured identifiers in format strings.

### Added
* added `--helper` option to `py` command. This allows users to load a python user helper script as a module named `qsv_uh`. [Example](https://github.com/jqnatividad/qsv/blob/78046d922e9a530c0887f18065fc325049b58687/tests/test_py.rs#L93) 
* added support for last N records in `slice` command by allowing negative values for the `slice --start` option.
* added progress bar to `py` command.

## [0.30.1] - 2022-01-23
### Changed
* convert more format strings to use captured identifiers
* bump jsonschema to 0.14.0. This will allow cross-compilation to work again as 
  we can explicitly use rustls for reqwest. This is required as cross no longer bundles openssl.

### Fixed
* fixed broken self-update ([#150](https://github.com/jqnatividad/qsv/issues/150))

## [0.30.0] - 2022-01-22
### Added
* `validate` command by @mhuang74 in https://github.com/jqnatividad/qsv/pull/145
* README: additional information on xsv fork differences

### Changed
* bumped MSRV to 1.58.1
* `validate` tweaks in https://github.com/jqnatividad/qsv/pull/148
* `validate` buffered jsonl error report in https://github.com/jqnatividad/qsv/pull/149

### Fixed
* fix `fetch` bugs by @mhuang74 in https://github.com/jqnatividad/qsv/pull/146
* README: added missing `--path` option in `cargo install`

## [0.29.1] - 2022-01-17
### Changed
* refactored `--update` to give update progress messages; run on `--help` as well
* updated README
  - remove bold formatting of commands
  - expanded descriptions of
      - fixlengths
      - foreach
      - jsonl
      - py
    - searchset
  - added reason why pre-built binaries on some platforms do not have the python feature installed.
  - drop use of "parallelism", just say "multithreading"
  - expanded Feature Flag section
* bump cached from 0.26 to 0.29
* added `update_cache_info!` macro to util.rs, replacing redundant code for progress indicators with cache info
* bump MSRV to Rust 1.58
* use new Rust 1.58 captured identifiers for format strings
* added `output_stderr` test helper to test for expected errors in CI
* added tests for invalid delimiter length; truncated comment char and unknown apply operators
* pointed documentation to Github README instead of doc.rs
* added `rustup update` to Github Actions publish workflow as Github's runners are still on Rust 1.57
* added Debian package build to publish workflow for `x86_64-unknown-linux-musl`

### Fixed
* corrected help text on job divisor is 3 not 4 for multithreaded commands (`frequency`, `split` and `stats`)
* corrected `stats` help text to state that multithreading requires an index

## [0.29.0] - 2022-01-08
### Changed
* `fetch`: enable cookies and storing error messages by @mhuang74 in https://github.com/jqnatividad/qsv/pull/141
* `fetch`: improve jql integration by @mhuang74 in https://github.com/jqnatividad/qsv/pull/139
* `--envlist` option now returns all qsv-relevant environment variables in https://github.com/jqnatividad/qsv/pull/140
* Move logging and update utility functions to util.rs in https://github.com/jqnatividad/qsv/pull/142
* `fetch`: support custom http headers by @mhuang74 in https://github.com/jqnatividad/qsv/pull/143
* bumped whatlang to 13.0 which supports Tagalog detection
* improved documentation of feature flags, environment variables & `stats` command

### Added
* added JSONL/NDJSON to Recognized File Formats (thru `jsonl` command)
* added CODE_OF_CONDUCT.md

### Deleted
* removed WIP indicator from `fetch` in README

## [0.28.0] - 2021-12-31
### Changed
* Fetch: support rate limiting by @mhuang74 in https://github.com/jqnatividad/qsv/pull/133
* Runtime minimum version check for Python 3.7 if `python` feature is enabled  https://github.com/jqnatividad/qsv/pull/138
* Fine-tuned GitHub Actions publish workflow for pre-built binaries
   * removed upx compression, as it was creating invalid binaries on certain platforms
   * enabled `python` feature on x86_64 platforms as we have access to the Python interpreter on GitHub's Action runners
   * include both `qsv` and `qsvlite` in the distribution zip file
* Formatted Cargo.toml with Even Better TOML VS code extension
* changed Cargo.toml categories and keywords
* removed patch version number from Cargo.toml dependencies. Let cargo do its semver dependency magic, and we include the Cargo.lock file anyway.

### Added
* added example of Python f-string formatting to `py` help text
* added Python f-string formatting test
* Added note in README about enabled features in pre-built binaries

### Deleted
* Removed _**NEW**_ and _**EXTENDED**_ indicators in README

## [0.27.1] - 2021-12-28
### Changed
* changed publish workflow for apple targets to use Xcode 12.5.1 from 12.4
* `jsonl` command now recognize and process JSON arrays
* `--version` option now shows binary name and enabled features
* Use upgraded [`qsv_currency`](https://crates.io/crates/qsv_currency) fork to power `apply currencytonum` operation. Now supports currency strings
  (e.g. USD, EUR, JPY, etc) in addition to currency symbols (e.g. $, €, ¥, etc)
* renamed `QSV_COMMENTS` environment variable to `QSV_COMMENT_CHAR` to make it clear that it clear that we're expecting
  a single character, not a boolean as the old name implies.

### Added
* added `create_from_string` helper function in workdir.rs
* compress select pre-built binaries with [UPX](https://upx.github.io/)
* `qsvlite` binary target, with all features disabled.
* `py` command. Evaluates a Python expression over CSV lines to transform, aggregate or filter them.

### Deleted
* removed Debian package publishing workflow, as the GH action for it
  does not support Rust 2021 edition

## [0.26.2] - 2021-12-21
## Added
* automatic self-update version check when the `--list` option is invoked.
* `QSV_NO_UPDATE` environment variable to prohibit self-update checks.
### Fixed
* explicitly include `deflate` compression method for self_update. Otherwise, `--update` unzipping doesn't work.
## [0.26.1] - 2021-12-21
### Fixed
* explicitly include `deflate` compression method for self_update. Otherwise, `--update` unzipping doesn't work.
## [0.26.0] - 2021-12-21
### Changed
* `fetch` refinements. Still WIP, but usable (See [#77](https://github.com/jqnatividad/qsv/issues/77))
  - add default user agent
  - `fetch` progress bar
  - `--jobs`, `--throttle`, `--header`, `--store-error` and `cookies` options still not functional.
* cargo update bump several crates to their latest releases. Of note are `test-data-generation`, 
`self_update` and `jql` where we worked with the crate maintainers directly with the update.

### Fixed
* `--update` bug fixed. It was not finding the binary to self update properly.


## [0.25.2-beta] - 2021-12-13
## Added
* `fetch` command by [@mhuang74](https://github.com/mhuang74). Note that the command is functional but still WIP, that's why this is a beta release.
* Download badge for GitHub pre-built binaries
* Compute hashes for pre-built binaries for verification

## Changed
* Additional helptext for `apply` NLP functions
* standardized on canonical way to suppress progress bars with `--quiet` option
* README: Mentioned `--frozen` option when installing/building qsv; wordsmithing
* rustfmt; clippy

## Deleted
* remove obsolete Makefile and .gitsubmodules
## [0.24.1] - 2021-12-06
### Changed
- changed selfupdate dependency to use pure Rust TLS implementation as cross no longer bundles OpenSSL, causing some binary builds using cross to fail.
## [0.24.0] - 2021-12-06
### Added
* Add logging by @mhuang74 in https://github.com/jqnatividad/qsv/pull/116
* Environment variables for logging - `QSV_LOG_LEVEL` and `QSV_LOG_DIR` - see [Logging](https://github.com/jqnatividad/qsv/blob/master/docs/Logging.md#logging) for more details.
* `sentiment` analysis `apply` operation  https://github.com/jqnatividad/qsv/pull/121
* `whatlang` language detection `apply` operation  https://github.com/jqnatividad/qsv/pull/122
* aarch64-apple-darwin prebuilt binary (Apple Silicon AKA M1)
* `--envlist` convenience option to list all environment variables with the `QSV_` prefix

### Changed
* changed `MAX_JOBS` heuristic logical processor divisor from 4 to 3
* `selfupdate` is no longer an optional feature

## New Contributors
* @mhuang74 made their first contribution in https://github.com/jqnatividad/qsv/pull/116
## [0.23.0] - 2021-11-29
### Added
- added `--update` option. This allows qsv to check and update itself if there are new release binaries published on GitHub.
- added `--envlist` option to show all environment variables with the `QSV_` prefix.
- `apply`, `generate`, `lua`, `foreach` and `selfupdate` are now optional features. `apply` and `generate` are marked optional since they have
large dependency trees; `lua` and `foreach` are very powerful commands that can be abused to issue system commands. Users now have the option exclude these features from their local builds.  Published binaries on GitHub still have `-all-features` enabled.
- added `QSV_COMMENTS` environment variable (contributed by [@jbertovic](https://github.com/jbertovic)). This allows qsv to ignore lines in the CSV (including headers) that start with the set character. [EXAMPLES](https://github.com/jqnatividad/qsv/blob/feae8cf5750530318b83c4b3c7bf0f72d2332079/tests/test_comments.rs#L3)
- catch input empty condition when qsv's input is empty when using `select`.   
(e.g. `cat /dev/null | qsv select 1` will now show the error "Input is empty." instead of "Selector index 1 is out of bounds. Index must be >= 1 and <= 0.")
- added `--pad <arg>` option to `split` command to zero-pad the generated filename by the number of `<arg>` places. [EXAMPLES](https://github.com/jqnatividad/qsv/blob/feae8cf5750530318b83c4b3c7bf0f72d2332079/tests/test_split.rs#L81)
- tests for `QSV_COMMENTS`, `split --pad`, `select` input empty condition, 
### Changed
- set Cargo.toml to Rust 2021 edition
- added "command-line-utilities" category to crates.io metadata
- cargo update bumped `mimalloc`, `serde_json`, `syn`, `anyhow` and `ryu`.
- GitHub Actions CI tests runs with `--all-features` enabled.
- published binaries on GitHub have `--all-features` enabled by default.
- made geocode caching a tad faster by making the transitional cache unbounded, and simplifying the key.
- `--version` now also shows the number of logical CPUs detected.
- project-wide rustfmt
- documentation for features, `QSV_COMMENTS` and `apply`
### Removed
- removed greetings.yml workflow from GitHub Actions.

## [0.22.1] - 2021-11-22
### Added
- added `lua` and `foreach` feature flags. These commands are very powerful and can be easily abused or get into "foot-shooting" scenarios.
They are now only enabled when these features are enabled during install/build.
- `censor` and `censor_check` now support the addition of custom profanities to screen for with the --comparand option.
### Changed
- removed `lazy_static` and used `once_cell` instead
- smaller stripped binaries for `x86_64-unknown-linux-gnu`, `i686-unknown-linux-gnu`, `x86_64-apple-darwin` targets
- expanded `apply` help text
- added more tests (currencytonum, censor, censor_check)

## [0.22.0] - 2021-11-15
### Added
- `generate` command. Generate test data by profiling a CSV using a [Markov decision process](https://docs.rs/test-data-generation).
- add `--no-headers` option to `rename` command (see [discussion #81](https://github.com/jqnatividad/qsv/discussions/81#discussioncomment-1599027))
- Auto-publish binaries for more platforms on release
- added combo-test for sort-dedup-sort (see [discussion #80](https://github.com/jqnatividad/qsv/discussions/80#discussioncomment-1610190))
- New environment variables galore
  - `QSV_DEFAULT_DELIMITER` - single ascii character to use as delimiter.  Overrides `--delimiter` option. Defaults to "," (comma) for CSV files and "\t" (tab) for TSV files, when not set. Note that this will also set the delimiter for qsv's output. Adapted from [xsv PR](https://github.com/BurntSushi/xsv/pull/94) by [@camerondavison](https://github.com/camerondavison).
  - `QSV_NO_HEADERS` - when set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`.
  - `QSV_MAX_JOBS` - number of jobs to use for parallelized commands (currently `frequency`, `split` and `stats`). If not set, max_jobs is set
to number of logical processors divided by four.  See [Parallelization](#parallelization) for more info.
  - `QSV_REGEX_UNICODE` - if set, makes `search`, `searchset` and `replace` commands unicode-aware. For increased performance, these
commands are not unicode-aware and will ignore unicode values when matching and will panic when unicode characters are used in the regex.
- Added parallelization heuristic (num_cpus/4), in connection with `QSV_MAX_JOBS`.
- Added more tests
  - `apply` (test for regex_replace, eudex, and lat/long parsing)
  - combo-test (see above) - for testing qsv command combinations
  - tests for `QSV_NO_HEADERS` environment variable
  - tests for `QSV_REGEX_UNICODE` environment variable in `search`, `searchset` and `replace` commands
  - tests for `QSV_DEFAULT_DELIMITER` environment variable
### Changed
- MSRV of Rust 1.56
- expanded `apply` help-text examples
- progress bar now only updates every 1% progress by default
- replaced English-specific soundex with multi-lingual eudex algorithm (see https://docs.rs/crate/eudex/0.1.1)
- refactored `apply geocode` subcommand to improve cache performance
- improved lat/long parsing - can now recognize embedded coordinates in text
- changed `apply operations regex_replace` behavior to do all matches in a field, instead of just the left-most one, to be consistent with the behavior of `apply operations replace`

## [0.21.0] - 2021-11-07
### Added
- added `apply geocode` caching, more than doubling performance in the geocode benchmark.
- added `--random` and `--seed` options to `sort` command from [@pjsier](https://github.com/pjsier).
- added qsv tab completion section to README.
- additional `apply operations` subcommands:
  * Match Trim operations - enables trimming of more than just whitespace, but also of multiple trim characters in one pass ([Example](https://github.com/jqnatividad/qsv/blob/9569dd7c2a897e0a47b97e1abfd1c3efab920990/tests/test_apply.rs#L214)):
    * mtrim: Trims `--comparand` matches left & right of the string ([trim_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_matches) wrapper)
    * mltrim: Left trim `--comparand` matches ([trim_start_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_start_matches) wrapper)
    * mrtrim: Right trim `--comparand` matches ([trim_end_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_end_matches) wrapper)
  * replace: Replace all matches of a pattern (using `--comparand`)
      with a string (using `--replacement`) (Std::String [replace](https://doc.rust-lang.org/std/string/struct.String.html#method.replace) wrapper).
  * regex_replace: Replace the leftmost-first regex match with `--replacement` (regex [replace](https://docs.rs/regex/1.1.0/regex/struct.Regex.html#method.replace) wrapper).
  * titlecase - capitalizes English text using Daring Fireball titlecase style
      https://daringfireball.net/2008/05/title_case
  * censor_check: check if profanity is detected (boolean) [Examples](https://github.com/jqnatividad/qsv/blob/9569dd7c2a897e0a47b97e1abfd1c3efab920990/tests/test_apply.rs#L66)
  * censor: profanity filter
- added parameter validation to `apply operations` subcommands
- added more robust parameter validation to `apply` command by leveraging docopt
- added more tests
- added `rust-version` in Cargo.toml to specify MSRV of rust 1.56

### Changed
- revamped benchmark script:
  * allow binary to be changed, so users can benchmark xsv and other xsv forks by simply replacing the $bin shell variable
  * now uses a much larger data file - a 1M row, 512 mb, 41 column sampling of NYC's 311 data
  * simplified and cleaned-up script now that it's just using 1 data file
- Upgrade rand and quickcheck crates to latest releases (0.8.4 and 1.0.3 respectively), and modified code accordingly.
- `cargo update` bumped addr2line (0.16.0->0.17.0), backtrace (0.3.62->0.3.63), gimli (0.25.0->0.26.1) and anyhow (1.0.44->1.0.45)

### Removed
- removed `scramble` command as its function is now subsumed by the `sort` command with the `--random` and `--seed` options
- removed `num-format` crate which has a large dependency tree with several old crates; replaced with much smaller `thousands` crate.
- removed 1M row, 48mb, 7 column world_cities_pop_mil.csv as its no longer used by the revamped benchmark script.
- removed `build.rs` build dependency that was checking for MSRV of Rust >= "1.50". Instead, took advantage of new [`rust-version`](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html#cargo-rust-version) Cargo.toml option
introduced in Rust 1.56.

## [0.20.0] - 2021-10-31
### Added
- added string similarity operations to `apply` command:
  * simdl: Damerau-Levenshtein similarity
  * simdln: Normalized Damerau-Levenshtein similarity (between 0.0 & 1.0)
  * simjw: Jaro-Winkler similarity (between 0.0 & 1.0)
  * simsd: Sørensen-Dice similarity (between 0.0 & 1.0)
  * simhm: Hamming distance. Number of positions where characters differ.
  * simod: OSA Distance.
  * soundex: sounds like (boolean)
- added progress bars to commands that may spawn long-running jobs - for this release,
`apply`, `foreach`, and `lua`. Progress bars can be suppressed with `--quiet` option.
- added progress bar helper functions to utils.rs.
- added `apply` to benchmarks.
- added sample NYC 311 data to benchmarks.
- added records per second (RECS_PER_SEC) to benchmarks

### Changed
- major refactoring of `apply` command:
  - to take advantage of docopt parsing/validation.
  - instead of one big command, broke down apply to several subcommands:
    - operations
    - emptyreplace
    - datefmt
    - geocode
- simplified lat/long regex validator to no longer validate range, as the underlying 
geocoder function validates it already - 18% geocode speedup.
- bumped docopt back up to 1.1.1.
- improved error message when specifying an invalid apply operation.

## [0.19.0] - 2021-10-24
### Added
- new `scramble` command. Randomly scrambles a CSV's records.
- read/write buffer capacity can now be set using environment variables
`QSV_RDR_BUFFER_CAPACITY` and `QSV_WTR_BUFFER_CAPACITY` (in bytes).
- added additional test for `apply datefmt`.

### Changed
- default read buffer doubled from 8k to 16k.
- default write buffer doubled from 32k to 64k.
- benchmark script revamped. Now produces aligned output onscreen,
while also creating a benchmark TSV file; downloads the sample file from GitHub;
benchmark more commands.
- version info now also returns memory allocator being used, and number of cpus detected.
- minor refactor of `enumerate`, `explode`, `fill` and `foreach` commands.

### Removed
- removed benchmark data from repository. Moved to GitHub wiki instead.

## [0.18.2] - 2021-10-21
## Changed
- use docopt v1.1.0 instead of docopt v.1.1.1 for docopt to support all regex features

## [0.18.1] - 2021-10-20
### Added
- added `mimalloc` feature flag. [mimalloc](https://github.com/microsoft/mimalloc) is Microsoft's performance-oriented memory allocator.
Earlier versions of qsv used mimalloc by default. Now it is only used when the feature is set.
- README: Added Performance section.
- README: Document how to enable `mimalloc` feature.

### Changed
- README: Explicitly show how to set environment variables on different platforms.
## [0.18.0] - 2021-10-18
### Added
- `stats` `mode` is now also multi-modal -i.e. returns multiples modes when detected. 
e.g. mode[1,1,2,2,3,4,6,6] will return [1,2,6].
It will continue to return one mode if there is only one detected.
- `stats` `quartile` now also computes IQR, lower/upper fences and skew ([using Pearson's median skewness](https://en.wikipedia.org/wiki/Skewness#Pearson's_second_skewness_coefficient_(median_skewness))). For code simplicity, calculated skew with quartile.
- `join` now also support `left-semi` and `left-anti` joins, the same way [Spark does](https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-join.html#semi-join).
- `search` `--flag` option now returns row number, not just '1'.
- `searchset` `--flag` option now returns row number, followed by a semi-colon, and a list of matching regexes.
- README: Added badges for Security Audit, Discussion & Docs
- README: Added FAQ link in fork note.

### Changed
- point to https://docs.rs/crate/qsv for documentation.
- README: `stats` and `join` section updated with new features.
- README: wordsmithing - replaced "CSV data" and "CSV file/s" with just "CSV".
- in `stats` changed `q2` column name to `q2_median`.
- removed debug symbols in release build for smaller binaries.
- minor refactoring of `search`, `searchset` & `stats`.

### Fixed
- README: fixed `flatten` example.

### Removed
- removed Rust badge.

## [0.17.3] - 2021-10-12
### Added
- added [sample regexset file](https://github.com/jqnatividad/qsv/commit/d209436b588b88b0f92cc133ebcada726f72a2bd) for PII-screening.

### Changed
- `apply geocode --formatstr` now accepts less US-centric format selectors.
- `searchset --flag` now shows which regexes match as a list (e.g. "[1, 3, 5]"), not just "1" or "0".
### Fixed
- `foreach` command now returns error message on Windows. `foreach` still doesn't work on 
Windows (yet), but at least it returns "foreach command does not work on Windows.".
- `apply geocode` was not accepting valid lat/longs below the equator. Fixed regex validator.
- more robust `searchset` error handling when attempting to load regexset files.
- `apply` link on README was off by one. 
## [0.17.2] - 2021-10-10

### Changed
- bumped `dateparser` to 0.1.6. This now allows `apply datefmt` to properly reformat
dates without a time component. Before, when reformatting a date like "July 4, 2020", 
qsv returns "2020-07-04T00:00:00+00:00". It now returns "2020-07-04".
- minor clippy refactoring
### Removed
- removed rust-stats submodule introduced in 0.17.1. It turns out
crates.io does not allow publishing of crates with local dependencies on submodules. 
Published the modified rust-stats fork as qsv-stats instead. This allows us to publish
qsv on crates.io
- removed unused `textwrap` dependency
## [0.17.1] - 2021-10-10
### Fixed
- explicitly specified embedded modified rust-stats version in Cargo.toml. 
## [0.17.0] - 2021-10-10
### Added
- added `searchset` command. Run **multiple regexes** over CSV data in a **single pass**.
- added `--unicode` flag to `search`, `searchset` and `replace` commands.
Previously, regex unicode support was on by default, which comes at the cost of performance.
And since `qsv` optimizes for performance ("q is for quick"), it is now off by default.
- added quartiles calculation to `stats`. Pulled in upstream
[pending](https://github.com/BurntSushi/rust-stats/pull/15) [PRs](https://github.com/BurntSushi/xsv/pull/273) 
from [@m15a](https://github.com/m15a) to implement.

### Changed
- changed variance algorithm. For some reason, the previous variance algorithm was causing
intermittent test failures on macOS. Pulled in [pending upstream PR](https://github.com/BurntSushi/rust-stats/pull/11)
from [@ruppertmillard](https://github.com/ruppertmillard).
- embedded [rust-stats fork](https://github.com/jqnatividad/rust-stats) submodule which implements 
quartile and new variance algorithm.
- changed GitHub Actions to pull in submodules.

### Fixed
- the project was not following semver properly, as several new features were released 
in the 0.16.x series that should have been MINOR version bumps, not PATCH bumps.

## [0.16.4] - 2021-10-08
### Added
- added `geocode` operation to `apply` command. It geocodes to the closest city given a column   
with coordinates in Location format ('latitude, longitude') using a static geonames lookup file.   
(see https://docs.rs/reverse_geocoder)
- added `currencytonum` operation to `apply` command.
- added `getquarter.lua` helper script to support `lua` example in [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook).
- added `turnaroundtime.lua` helper script to compute turnaround time.
- added `nyc311samp.csv` to provide sample data for recipes.
- added several Date Enrichment and Geocoding recipes to [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook).

### Fixed
- fixed `publish.yml` Github Action workflow to properly create platform specific binaries.
- fixed variance test to eliminate false positives in macOS.

## [0.16.3] - 2021-10-06
### Added
- added `docs` directory. For README reorg, and to add detailed examples per command in the future.
- added `emptyreplace` operation to `apply` command.
- added `datefmt` operation to `apply` command.
- added support for reading from stdin to `join` command.
- setup GitHub wiki to host [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook) and sundry docs to encourage collaborative editing.
- added footnotes to commands table in README.

### Changed
- changed GitHub Actions publish workflow so it adds the version to binary zip filename.
- changed GitHub Actions publish workflow so binary is no longer in `target/release` directory.
- reorganized README.
- moved whirlwind tour and benchmarks to `docs` directory.
- use zipped repo copy of worldcitiespop_mil.csv for benchmarks.

### Fixed
- fixed links to help text in README for `fixlengths` and `slice` cmds
- `exclude` not listed in commands table. Added to README.

### Removed
- Removed `empty0` and `emptyNA` operations in `apply` command.
Replaced with `emptyreplace`.

## [0.16.2] - 2021-09-30
### Changed
- changed Makefile to remove github recipe as we are now using GitHub Actions.
- Applied rustfmt to entire project [#56](https://github.com/jqnatividad/qsv/issues/56)
- Changed stats variance test as it was causing false positive test failures on macOS ([details](https://github.com/jqnatividad/qsv/commit/8c45c60de7598c7dc4cedd10ce7cb281ee34db46))
- removed `-amd64` suffix from binaries built by GitHub Actions.

### Fixed
- fixed publish Github Actions workflow to zip binaries before uploading.

### Removed 
- removed `.travis.yml` as we are now using GitHub Actions.
- removed scripts `build-release`, `github-release` and `github-upload` as we are now
 using GitHub Actions.
- removed `ci` folder as we are now using GitHub Actions.
- removed `py` command. [#58](https://github.com/jqnatividad/qsv/issues/58)

## [0.16.1] - 2021-09-28
### Fixed
- Bumped qsv version to 0.16.1. Inadvertently released 0.16.0 with qsv version still at 0.15.0.

## [0.16.0] - 2021-09-28
### Added
- Added a CHANGELOG.
- Added additional commands/options from [@Yomguithereal](https://github.com/Yomguithereal) 
[xsv fork](https://github.com/Yomguithereal/xsv).
  * `apply` - Apply series of string transformations to a CSV column.
  * `behead` - Drop headers from CSV file.
  * `enum` - Add a new column enumerating rows by adding a column of incremental or 
  uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.
  * `explode` - Explode rows into multiple ones by splitting a column value based on the given separator.
  * `foreach` - Loop over a CSV file to execute bash commands.
  * `jsonl` - Convert newline-delimited JSON to CSV.
  * `lua` - Execute a Lua script over CSV lines to transform, aggregate or filter them.
  * `pseudo` -  Pseudonymise the value of the given column by replacing them by an incremental identifier.
  * `py` - Evaluate a Python expression over CSV lines to transform, aggregate or filter them.
  * `replace` - Replace CSV data using a regex.
  * `sort` --uniq option - When set, identical consecutive lines will be dropped to keep only one line 
  per sorted value.
  * `search` --flag `column` option -  If given, the command will not filter rows but will instead flag 
  the found rows in a new column named `column`.

- Added conditional compilation logic for `foreach` command to only 
compile on `target_family=unix` as it has a dependency on 
`std::os::unix::ffi::OsStrExt` which only works in unix-like OSes.
- Added `empty0` and `emptyNA` operations to `apply` command with 
corresponding test cases.
- Added GitHub Actions to check builds on `ubuntu-latest`, 
`windows-latest` and `macos-latest`.
- Added GitHub Action to publish binaries on release.
- Added `build.rs` build-dependency to check that Rust is at least 
at version 1.50.0 and above.

### Changed
- reformatted README listing of commands to use a table, and to link to
corresponding help text.

### Removed
- Removed appveyor.yml as qsv now uses GitHub Actions.

## [0.15.0] - 2021-09-22
### Added
- `dedup` cmd from [@ronohm](https://github.com/ronohm).
- `table` cmd `--align` option from [@alex-ozdemir](https://github.com/alex-ozdemir).
- `fmt` cmd `--quote-never` option from [@niladic](https://github.com/niladic).
- `exclude` cmd from [@lalaithion](https://github.com/lalaithion)
- Added `--dupes-output` option to `dedup` cmd.
- Added datetime type detection to `stats` cmd.
- Added datetime `min/max` calculation to `stats` cmd.
- es-ES translation from [@ZeliosAriex](https://github.com/ZeliosAriex).

### Changed
- Updated benchmarks script.
- Updated whirlwind tour to include additional commands.
- Made whirlwind tour reproducible by using `sample` `--seed` option.

### Fixed
- Fixed `sample` percentage sampling to be always reproducible even if
sample size < 10% when using `--seed` option.
- Fixed BOM issue with tests, leveraging [unreleased xsv fix](https://github.com/BurntSushi/xsv/commit/a1165e0fe58e6e39f6ed8b1a67ca87dd966c0df3).
- Fixed count help text typo.

### Removed
- Removed `session.vim` file.

## [0.14.1] - 2021-09-15
### Changed
- Performance: enabled link-time optimization (`LTO="fat"`).
- Performance: used code generation units.
- Performance: used [mimalloc](https://docs.rs/mimalloc/0.1.26/mimalloc/) allocator.
- Changed benchmark to compare xsv 0.13.0 and qsv.
- Changed chart from png to svg.
- Performance: Added note in README on how to optimize local compile 
by setting `target-cpu=native`.

## [0.14.0] - 2021-09-14
### Changed
- Renamed fork to qsv.
- Revised highlight note explaining reason for qsv renamed fork in README.
- Added **(NEW)** and **(EXPANDED)** notations to command listing.
- Adapted to Rust 2018 edition.
- used serde derive feature.

## [0.13.1] - 2020-12-27
Initial fork from xsv.
### Added
- `rename` cmd from [@Kerollmops](https://github.com/Kerollmops).
- `fill` cmd from [@alexrudy](https://github.com/alexrudy).
- `transpose` cmd from [@mintyplanet](https://github.com/mintyplanet).
- `select` cmd regex support from [@sd2k](https://github.com/sd2k).
- `stats` cmd `--nullcount` option from [@scpike](https://github.com/scpike).
- added percentage sampling to `sample` cmd.

### Changed
- Updated README with additional commands.
