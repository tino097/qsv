module completions {

  export extern qsv [
    --list
    --envlist
    --update
    --updatenow
    --version
    --help(-h)                # Print help
  ]

  export extern "qsv apply" [
    --new-column
    --rename
    --comparand
    --replacement
    --formatstr
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --new-column
    --rename
    --comparand
    --replacement
    --formatstr
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --new-column
    --rename
    --comparand
    --replacement
    --formatstr
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --new-column
    --rename
    --comparand
    --replacement
    --formatstr
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --new-column
    --rename
    --comparand
    --replacement
    --formatstr
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv apply help" [
  ]

  export extern "qsv apply help operations" [
  ]

  export extern "qsv apply help emptyreplace" [
  ]

  export extern "qsv apply help dynfmt" [
  ]

  export extern "qsv apply help calcconv" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv apply help help" [
  ]

  export extern "qsv behead" [
    --flexible
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --flexible
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --group
    --group-name
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --pad
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv cat help" [
  ]

  export extern "qsv cat help rows" [
  ]

  export extern "qsv cat help rowskey" [
  ]

  export extern "qsv cat help columns" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv cat help help" [
  ]

  export extern "qsv clipboard" [
    --save
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --human-readable
    --width
    --width-no-delims
    --json
    --no-polars
    --low-memory
    --flexible
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --formatstr
    --new-column
    --rename
    --prefer-dmy
    --keep-zero-time
    --input-tz
    --output-tz
    --default-tz
    --utc
    --zulu
    --ts-resolution
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --select
    --numeric
    --ignore-case
    --sorted
    --dupes-output
    --human-readable
    --jobs
    --output
    --no-headers
    --delimiter
    --quiet
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --all
    --description
    --dictionary
    --tags
    --api-key
    --max-tokens
    --json
    --jsonl
    --prompt
    --prompt-file
    --base-url
    --model
    --timeout
    --user-agent
    --output
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --no-headers-left
    --no-headers-right
    --no-headers-output
    --delimiter-left
    --delimiter-right
    --delimiter-output
    --key
    --sort-columns
    --drop-equal-fields
    --jobs
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv edit" [
    --output
    --no-headers
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --new-column
    --start
    --increment
    --constant
    --copy
    --uuid4
    --uuid7
    --hash
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --sheet
    --header-row
    --metadata
    --table
    --range
    --error-format
    --flexible
    --trim
    --date-format
    --keep-zero-time
    --jobs
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --ignore-case
    -v
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --select
    --no-output
    --dupes-output
    --human-readable
    --memory-limit
    --temp-dir
    --no-headers
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --select
    --reverse
    --memory-limit
    --tmp-dir
    --jobs
    --delimiter
    --no-headers
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --rename
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --url-template
    --new-column
    --jaq
    --jaqfile
    --pretty
    --rate-limit
    --timeout
    --http-header
    --max-retries
    --max-errors
    --store-error
    --cookies
    --user-agent
    --report
    --no-cache
    --mem-cache-size
    --disk-cache
    --disk-cache-dir
    --redis-cache
    --cache-error
    --flush-cache
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --payload-tpl
    --content-type
    --new-column
    --jaq
    --jaqfile
    --pretty
    --rate-limit
    --timeout
    --http-header
    --compress
    --max-retries
    --max-errors
    --store-error
    --cookies
    --user-agent
    --report
    --no-cache
    --mem-cache-size
    --disk-cache
    --disk-cache-dir
    --redis-cache
    --cache-error
    --flush-cache
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --groupby
    --first
    --backfill
    --default
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --length
    --remove-empty
    --insert
    --quote
    --escape
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --condense
    --field-separator
    --separator
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --out-delimiter
    --crlf
    --ascii
    --quote
    --quote-always
    --quote-never
    --escape
    --no-final-newline
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --unify
    --new-column
    --dry-run
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --select
    --limit
    --unq-limit
    --lmt-threshold
    --pct-dec-places
    --other-sorted
    --other-text
    --asc
    --no-trim
    --no-nulls
    --ignore-case
    --stats-mode
    --all-unique-text
    --jobs
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggest" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode suggestnow" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reverse" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode reversenow" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfo" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode countryinfonow" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-load" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-check" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-update" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv geocode index-reset" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv geocode help" [
  ]

  export extern "qsv geocode help suggest" [
  ]

  export extern "qsv geocode help suggestnow" [
  ]

  export extern "qsv geocode help reverse" [
  ]

  export extern "qsv geocode help reversenow" [
  ]

  export extern "qsv geocode help countryinfo" [
  ]

  export extern "qsv geocode help countryinfonow" [
  ]

  export extern "qsv geocode help index-load" [
  ]

  export extern "qsv geocode help index-check" [
  ]

  export extern "qsv geocode help index-update" [
  ]

  export extern "qsv geocode help index-reset" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv geocode help help" [
  ]

  export extern "qsv headers" [
    --just-names
    --just-count
    --intersect
    --trim
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --quote
    --escape
    --no-quoting
    --quote-style
    --skip-lines
    --auto-skip
    --skip-lastlines
    --trim-headers
    --trim-fields
    --comment
    --encoding-errors
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --left
    --left-anti
    --left-semi
    --right
    --right-anti
    --right-semi
    --full
    --cross
    --nulls
    --keys-output
    --ignore-case
    --ignore-leading-zeros
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --left
    --left-anti
    --left-semi
    --right
    --right-anti
    --right-semi
    --full
    --cross
    --non-equi
    --coalesce
    --filter-left
    --filter-right
    --validate
    --maintain-order
    --nulls
    --streaming
    --try-parsedates
    --infer-len
    --cache-schema
    --low-memory
    --no-optimizations
    --ignore-errors
    --decimal-comma
    --asof
    --no-sort
    --left_by
    --right_by
    --strategy
    --tolerance
    --allow-exact-matches
    --sql-filter
    --datetime-format
    --date-format
    --time-format
    --float-precision
    --null-value
    --ignore-case
    --ignore-leading-zeros
    --norm-unicode
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --jaq
    --select
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --ignore-errors
    --jobs
    --batch
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv lens" [
    --delimiter
    --tab-separated
    --no-headers
    --columns
    --filter
    --find
    --ignore-case
    --freeze-columns
    --echo-column
    --debug
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --no-globals
    --colindex
    --remap
    --begin
    --end
    --luau-path
    --max-errors
    --timeout
    --ckan-api
    --ckan-token
    --cache-dir
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv luau map" [
    --no-globals
    --colindex
    --remap
    --begin
    --end
    --luau-path
    --max-errors
    --timeout
    --ckan-api
    --ckan-token
    --cache-dir
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv luau filter" [
    --no-globals
    --colindex
    --remap
    --begin
    --end
    --luau-path
    --max-errors
    --timeout
    --ckan-api
    --ckan-token
    --cache-dir
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv luau help" [
  ]

  export extern "qsv luau help map" [
  ]

  export extern "qsv luau help filter" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv luau help help" [
  ]

  export extern "qsv partition" [
    --filename
    --prefix-length
    --drop
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv pro" [
    --help(-h)                # Print help
  ]

  export extern "qsv pro lens" [
    --help(-h)                # Print help
  ]

  export extern "qsv pro workflow" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv pro help" [
  ]

  export extern "qsv pro help lens" [
  ]

  export extern "qsv pro help workflow" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv pro help help" [
  ]

  export extern "qsv prompt" [
    --msg
    --filters
    --workdir
    --fd-output
    --save-fname
    --base-delay-ms
    --output
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --start
    --increment
    --formatstr
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --helper
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv py map" [
    --helper
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv py filter" [
    --helper
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv py help" [
  ]

  export extern "qsv py help map" [
  ]

  export extern "qsv py help filter" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv py help help" [
  ]

  export extern "qsv rename" [
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --ignore-case
    --literal
    --select
    --unicode
    --size-limit
    --dfa-size-limit
    --not-one
    --output
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --mode
    --reserved
    --prefix
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --seed
    --rng
    --user-agent
    --timeout
    --max-size
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --enum-threshold
    --ignore-case
    --strict-dates
    --pattern-columns
    --date-whitelist
    --prefer-dmy
    --force
    --stdout
    --jobs
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --ignore-case
    --literal
    --select
    --invert-match
    --unicode
    --flag
    --quick
    --preview-match
    --count
    --size-limit
    --dfa-size-limit
    --json
    --not-one
    --output
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --ignore-case
    --literal
    --select
    --invert-match
    --unicode
    --flag
    --flag-matches-only
    --unmatched-output
    --quick
    --count
    --json
    --size-limit
    --dfa-size-limit
    --not-one
    --output
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --random
    --seed
    --sort
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --start
    --end
    --len
    --index
    --json
    --invert
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --user-agent
    --timeout
    --output
    --jobs
    --quiet
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --user-agent
    --timeout
    --output
    --jobs
    --quiet
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --user-agent
    --timeout
    --output
    --jobs
    --quiet
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --user-agent
    --timeout
    --output
    --jobs
    --quiet
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --user-agent
    --timeout
    --output
    --jobs
    --quiet
    --progressbar
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv snappy help" [
  ]

  export extern "qsv snappy help compress" [
  ]

  export extern "qsv snappy help decompress" [
  ]

  export extern "qsv snappy help check" [
  ]

  export extern "qsv snappy help validate" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv snappy help help" [
  ]

  export extern "qsv sniff" [
    --sample
    --prefer-dmy
    --delimiter
    --quote
    --json
    --pretty-json
    --save-urlsample
    --timeout
    --user-agent
    --stats-types
    --no-infer
    --just-mime
    --quick
    --harvest-mode
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --select
    --numeric
    --reverse
    --ignore-case
    --unique
    --random
    --seed
    --rng
    --jobs
    --faster
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --select
    --ignore-case
    --all
    --json
    --pretty-json
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --size
    --chunks
    --kb-size
    --jobs
    --filename
    --pad
    --no-headers
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --format
    --try-parsedates
    --infer-len
    --cache-schema
    --streaming
    --low-memory
    --no-optimizations
    --truncate-ragged-lines
    --ignore-errors
    --rnull-values
    --decimal-comma
    --datetime-format
    --date-format
    --time-format
    --float-precision
    --wnull-value
    --compression
    --compress-level
    --statistics
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --select
    --everything
    --typesonly
    --infer-boolean
    --mode
    --cardinality
    --median
    --mad
    --quartiles
    --round
    --nulls
    --infer-dates
    --dates-whitelist
    --prefer-dmy
    --force
    --jobs
    --stats-jsonl
    --cache-threshold
    --vis-whitespace
    --dataset-stats
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --width
    --pad
    --align
    --condense
    --output
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv template" [
    --template
    --template-file
    --globals-json
    --outfilename
    --outsubdir-size
    --customfilter-error
    --jobs
    --batch
    --timeout
    --cache-dir
    --ckan-api
    --ckan-token
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --print-package
    --dump
    --stats
    --stats-csv
    --quiet
    --schema
    --drop
    --evolve
    --pipe
    --separator
    --jobs
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv to postgres" [
    --print-package
    --dump
    --stats
    --stats-csv
    --quiet
    --schema
    --drop
    --evolve
    --pipe
    --separator
    --jobs
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv to sqlite" [
    --print-package
    --dump
    --stats
    --stats-csv
    --quiet
    --schema
    --drop
    --evolve
    --pipe
    --separator
    --jobs
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv to xlsx" [
    --print-package
    --dump
    --stats
    --stats-csv
    --quiet
    --schema
    --drop
    --evolve
    --pipe
    --separator
    --jobs
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv to datapackage" [
    --print-package
    --dump
    --stats
    --stats-csv
    --quiet
    --schema
    --drop
    --evolve
    --pipe
    --separator
    --jobs
    --delimiter
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv to help" [
  ]

  export extern "qsv to help postgres" [
  ]

  export extern "qsv to help sqlite" [
  ]

  export extern "qsv to help xlsx" [
  ]

  export extern "qsv to help datapackage" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv to help help" [
  ]

  export extern "qsv tojsonl" [
    --trim
    --no-boolean
    --jobs
    --batch
    --delimiter
    --output
    --memcheck
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --multipass
    --output
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --trim
    --fail-fast
    --valid
    --invalid
    --json
    --pretty-json
    --valid-output
    --jobs
    --batch
    --timeout
    --cache-dir
    --ckan-api
    --ckan-token
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help" [
  ]

  export extern "qsv help apply" [
  ]

  export extern "qsv help apply operations" [
  ]

  export extern "qsv help apply emptyreplace" [
  ]

  export extern "qsv help apply dynfmt" [
  ]

  export extern "qsv help apply calcconv" [
  ]

  export extern "qsv help behead" [
  ]

  export extern "qsv help cat" [
  ]

  export extern "qsv help cat rows" [
  ]

  export extern "qsv help cat rowskey" [
  ]

  export extern "qsv help cat columns" [
  ]

  export extern "qsv help clipboard" [
  ]

  export extern "qsv help count" [
  ]

  export extern "qsv help datefmt" [
  ]

  export extern "qsv help dedup" [
  ]

  export extern "qsv help describegpt" [
  ]

  export extern "qsv help diff" [
  ]

  export extern "qsv help edit" [
  ]

  export extern "qsv help enum" [
  ]

  export extern "qsv help excel" [
  ]

  export extern "qsv help exclude" [
  ]

  export extern "qsv help extdedup" [
  ]

  export extern "qsv help extsort" [
  ]

  export extern "qsv help explode" [
  ]

  export extern "qsv help fetch" [
  ]

  export extern "qsv help fetchpost" [
  ]

  export extern "qsv help fill" [
  ]

  export extern "qsv help fixlengths" [
  ]

  export extern "qsv help flatten" [
  ]

  export extern "qsv help fmt" [
  ]

  export extern "qsv help foreach" [
  ]

  export extern "qsv help frequency" [
  ]

  export extern "qsv help geocode" [
  ]

  export extern "qsv help geocode suggest" [
  ]

  export extern "qsv help geocode suggestnow" [
  ]

  export extern "qsv help geocode reverse" [
  ]

  export extern "qsv help geocode reversenow" [
  ]

  export extern "qsv help geocode countryinfo" [
  ]

  export extern "qsv help geocode countryinfonow" [
  ]

  export extern "qsv help geocode index-load" [
  ]

  export extern "qsv help geocode index-check" [
  ]

  export extern "qsv help geocode index-update" [
  ]

  export extern "qsv help geocode index-reset" [
  ]

  export extern "qsv help headers" [
  ]

  export extern "qsv help index" [
  ]

  export extern "qsv help input" [
  ]

  export extern "qsv help join" [
  ]

  export extern "qsv help joinp" [
  ]

  export extern "qsv help json" [
  ]

  export extern "qsv help jsonl" [
  ]

  export extern "qsv help lens" [
  ]

  export extern "qsv help luau" [
  ]

  export extern "qsv help luau map" [
  ]

  export extern "qsv help luau filter" [
  ]

  export extern "qsv help partition" [
  ]

  export extern "qsv help pro" [
  ]

  export extern "qsv help pro lens" [
  ]

  export extern "qsv help pro workflow" [
  ]

  export extern "qsv help prompt" [
  ]

  export extern "qsv help pseudo" [
  ]

  export extern "qsv help py" [
  ]

  export extern "qsv help py map" [
  ]

  export extern "qsv help py filter" [
  ]

  export extern "qsv help rename" [
  ]

  export extern "qsv help replace" [
  ]

  export extern "qsv help reverse" [
  ]

  export extern "qsv help safenames" [
  ]

  export extern "qsv help sample" [
  ]

  export extern "qsv help schema" [
  ]

  export extern "qsv help search" [
  ]

  export extern "qsv help searchset" [
  ]

  export extern "qsv help select" [
  ]

  export extern "qsv help slice" [
  ]

  export extern "qsv help snappy" [
  ]

  export extern "qsv help snappy compress" [
  ]

  export extern "qsv help snappy decompress" [
  ]

  export extern "qsv help snappy check" [
  ]

  export extern "qsv help snappy validate" [
  ]

  export extern "qsv help sniff" [
  ]

  export extern "qsv help sort" [
  ]

  export extern "qsv help sortcheck" [
  ]

  export extern "qsv help split" [
  ]

  export extern "qsv help sqlp" [
  ]

  export extern "qsv help stats" [
  ]

  export extern "qsv help table" [
  ]

  export extern "qsv help template" [
  ]

  export extern "qsv help to" [
  ]

  export extern "qsv help to postgres" [
  ]

  export extern "qsv help to sqlite" [
  ]

  export extern "qsv help to xlsx" [
  ]

  export extern "qsv help to datapackage" [
  ]

  export extern "qsv help tojsonl" [
  ]

  export extern "qsv help transpose" [
  ]

  export extern "qsv help validate" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help help" [
  ]

}

export use completions *
