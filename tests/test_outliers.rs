use crate::workdir::Workdir;

#[test]
fn test_outliers_basic() {
    let wrk = Workdir::new("outliers");
    wrk.create(
        "data.csv",
        vec![
            svec!["number", "value"],
            svec!["1", "10"],
            svec!["2", "12"],
            svec!["3", "15"],
            svec!["4", "100"], // Outlier
            svec!["5", "13"],
            svec!["6", "11"],
            svec!["7", "14"],
        ],
    );

    let mut cmd = wrk.command("outliers");
    cmd.arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Found 1 outlier"));
    assert!(got.contains("value: 100"));
}

#[test]
fn test_outliers_multiple_columns() {
    let wrk = Workdir::new("outliers_multiple");
    wrk.create(
        "data.csv",
        vec![
            svec!["temp", "pressure", "humidity"],
            svec!["20", "1013", "45"],
            svec!["22", "1014", "48"],
            svec!["21", "1012", "46"],
            svec!["50", "900", "99"], // All outliers
            svec!["23", "1015", "47"],
        ],
    );

    let mut cmd = wrk.command("outliers");
    cmd.arg("-s").arg("temp,pressure,humidity").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("temp: Found 1 outlier"));
    assert!(got.contains("pressure: Found 1 outlier"));
    assert!(got.contains("humidity: Found 1 outlier"));
}

#[test]
fn test_outliers_inner_fence() {
    let wrk = Workdir::new("outliers_inner");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["12"],
            svec!["15"],
            svec!["30"], // Outlier with inner fence
            svec!["13"],
            svec!["11"],
            svec!["14"],
        ],
    );

    let mut cmd = wrk.command("outliers");
    cmd.arg("-m").arg("inner").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Found 1 outlier"));
    assert!(got.contains("value: 30"));
}

#[test]
fn test_outliers_quiet_mode() {
    let wrk = Workdir::new("outliers_quiet");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["12"],
            svec!["15"],
            svec!["100"], // Outlier
            svec!["13"],
        ],
    );

    let mut cmd = wrk.command("outliers");
    cmd.arg("-q").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Found 1 outlier"));
    assert!(!got.contains("value: 100")); // Detailed output should be suppressed
}

#[test]
fn test_outliers_string_column() {
    let wrk = Workdir::new("outliers_string");
    wrk.create(
        "data.csv",
        vec![
            svec!["text"],
            svec!["normal"],
            svec!["typical"],
            svec!["regular"],
            svec!["very very very very long text"], // Length outlier
            svec!["usual"],
        ],
    );

    let mut cmd = wrk.command("outliers");
    cmd.arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Found 1 outlier"));
    assert!(got.contains("very very very very long text"));
}

#[test]
fn test_outliers_both_fences() {
    let wrk = Workdir::new("outliers_both");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["12"],
            svec!["15"],
            svec!["30"],  // Inner fence outlier
            svec!["100"], // Outer fence outlier
            svec!["13"],
            svec!["11"],
            svec!["14"],
        ],
    );

    let mut cmd = wrk.command("outliers");
    cmd.arg("-m").arg("both").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Inner fence outliers:"));
    assert!(got.contains("Outer fence outliers:"));
    assert!(got.contains("value: 30"));
    assert!(got.contains("value: 100"));
}
