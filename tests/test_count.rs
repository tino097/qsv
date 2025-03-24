use crate::{CsvData, qcheck, workdir::Workdir};

#[test]
fn count_simple() {
    let wrk = Workdir::new("count_simple");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_empty() {
    let wrk = Workdir::new("count_empty");
    wrk.create_from_string("empty.csv", "");
    let mut cmd = wrk.command("count");
    cmd.arg("empty.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "0";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_simple_tsv() {
    let wrk = Workdir::new("count_simple_tsv");
    wrk.create_with_delim(
        "in.tsv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
        b'\t',
    );

    let mut cmd = wrk.command("count");
    cmd.arg("in.tsv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";

    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_simple_ssv() {
    let wrk = Workdir::new("count_simple_ssv");
    wrk.create_with_delim(
        "in.ssv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.arg("in.ssv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";

    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_simple_custom_delimiter() {
    let wrk = Workdir::new("count_simple_custom_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.env("QSV_CUSTOM_DELIMITER", ";").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";

    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_width() {
    let wrk = Workdir::new("count_width");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;16-15-15-13-1.5-1.2247-1";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_json() {
    let wrk = Workdir::new("count_width");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("--json").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"count":4,"max":16,"avg":15,"median":15,"min":13,"variance":1.5,"stddev":1.2247,"mad":1}"#;
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_no_delims() {
    let wrk = Workdir::new("count_width_no_delims");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width-no-delims").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;16-13.5-13-11-3.25-1.8028-1.5";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_no_delims_human_readable() {
    let wrk = Workdir::new("count_width_no_delims_human_readable");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width-no-delims").arg("-H").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;max:16 avg:13.5 median:13 min:11 variance:3.25 stddev:1.8028 mad:1.5";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_human_readable() {
    let wrk = Workdir::new("count_width_human_readable");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("-H").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;max:16 avg:15 median:15 min:13 variance:1.5 stddev:1.2247 mad:1";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_custom_delimiter() {
    let wrk = Workdir::new("count_width_custom_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.env("QSV_CUSTOM_DELIMITER", ";")
        .arg("--width")
        .arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;18-15.5-15-13-3.25-1.8028-1.5";

    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_flexible() {
    let wrk = Workdir::new("count_flexible");
    wrk.create_from_string(
        "in.csv",
        r#"letter,number,flag
alphabetic,13,true,extra column
beta,24,false
gamma,37.1
delta,42.5,false
"#,
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--flexible").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn count_comments() {
    let wrk = Workdir::new("count_comments");

    wrk.create(
        "in.csv",
        vec![
            svec!["# this is a comment", ""],
            svec!["# next comment", ""],
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["# comment here too!", "24"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("in.csv").env("QSV_COMMENT_CHAR", "#");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "2";
    similar_asserts::assert_eq!(got, expected.to_string());
}

/// This tests whether `qsv count` gets the right answer.
///
/// It does some simple case analysis to handle whether we want to test counts
/// in the presence of headers and/or indexes.
fn prop_count_len(
    name: &str,
    rows: CsvData,
    headers: bool,
    idx: bool,
    noheaders_env: bool,
    human_readable: bool,
) -> bool {
    let mut expected_count = rows.len();
    if headers && expected_count > 0 {
        expected_count -= 1;
    }

    let wrk = Workdir::new(name);
    if idx {
        wrk.create_indexed("in.csv", rows);
    } else {
        wrk.create("in.csv", rows);
    }

    let mut cmd = wrk.command("count");
    if !headers {
        cmd.arg("--no-headers");
    }
    if noheaders_env {
        cmd.env("QSV_NO_HEADERS", "1");
    }
    if human_readable {
        cmd.arg("--human-readable");
    }
    cmd.arg("in.csv");

    if human_readable {
        use indicatif::HumanCount;

        let got_count: String = wrk.stdout(&mut cmd);
        let expected_count_commas = HumanCount(expected_count as u64).to_string();

        rassert_eq!(got_count, expected_count_commas)
    } else {
        let got_count: usize = wrk.stdout(&mut cmd);
        rassert_eq!(got_count, expected_count)
    }
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count", rows, false, false, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_human_readable() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count", rows, false, false, false, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_headers() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_headers", rows, true, false, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_headers_human_readable() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_headers", rows, true, false, false, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_indexed", rows, false, true, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_indexed_headers() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_indexed_headers", rows, true, true, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_noheaders_env() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_noheaders_env", rows, false, false, true, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_noheaders_indexed_env() {
    fn p(rows: CsvData) -> bool {
        prop_count_len(
            "prop_count_noheaders_indexed_env",
            rows,
            false,
            true,
            true,
            false,
        )
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn count_custom_delimiter() {
    let wrk = Workdir::new("count_custom_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.arg("--delimiter").arg(";").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn show_version() {
    let wrk = Workdir::new("show_version");
    let mut cmd = wrk.command("");
    cmd.arg("--version");

    let got: String = wrk.stdout(&mut cmd);
    let expected = format!(" {}", env!("CARGO_PKG_VERSION"));
    assert!(got.contains(&expected));
}
