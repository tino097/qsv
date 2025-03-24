use std::process;

use crate::workdir::Workdir;

fn setup(name: &str) -> (Workdir, process::Command) {
    let rows = vec![
        svec!["h1", "h2"],
        svec!["abcdef", "ghijkl"],
        svec!["mnopqr", "stuvwx"],
        svec!["ab\"cd\"ef", "gh,ij,kl"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fmt");
    cmd.arg("in.csv");

    (wrk, cmd)
}

#[test]
fn fmt_delimiter() {
    let (wrk, mut cmd) = setup("fmt_delimiter");
    cmd.args(["--out-delimiter", "\t"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1\th2
abcdef\tghijkl
mnopqr\tstuvwx
\"ab\"\"cd\"\"ef\"\tgh,ij,kl";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_weird_delimiter() {
    let (wrk, mut cmd) = setup("fmt_weird_delimiter");
    cmd.args(["--out-delimiter", "h"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
\"h1\"h\"h2\"
abcdefh\"ghijkl\"
mnopqrhstuvwx
\"ab\"\"cd\"\"ef\"h\"gh,ij,kl\"";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_crlf() {
    let (wrk, mut cmd) = setup("fmt_crlf");
    cmd.arg("--crlf");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1,h2\r
abcdef,ghijkl\r
mnopqr,stuvwx\r
\"ab\"\"cd\"\"ef\",\"gh,ij,kl\"";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_tab_delimiter() {
    let (wrk, mut cmd) = setup("fmt_tab_delimiter");
    cmd.args(["--out-delimiter", "T"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "h1\th2\nabcdef\tghijkl\nmnopqr\tstuvwx\n\"ab\"\"cd\"\"ef\"\tgh,ij,kl";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_nofinalnewline() {
    let (wrk, mut cmd) = setup("fmt_nofinalnewline");
    cmd.arg("--no-final-newline");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"h1,h2
abcdef,ghijkl
mnopqr,stuvwx
"ab""cd""ef","gh,ij,kl""#;
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_output() {
    let (wrk, mut cmd) = setup("fmt_output");

    let output_file = wrk.path("output.csv").to_string_lossy().to_string();

    cmd.args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();

    let expected = r#"h1,h2
abcdef,ghijkl
mnopqr,stuvwx
"ab""cd""ef","gh,ij,kl"
"#;
    similar_asserts::assert_eq!(got, expected);
}

#[test]
fn fmt_quote_always() {
    let (wrk, mut cmd) = setup("fmt_quote_always");
    cmd.arg("--quote-always");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
\"h1\",\"h2\"
\"abcdef\",\"ghijkl\"
\"mnopqr\",\"stuvwx\"
\"ab\"\"cd\"\"ef\",\"gh,ij,kl\"";
    similar_asserts::assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_quote_never() {
    let (wrk, mut cmd) = setup("fmt_quote_never");
    cmd.arg("--quote-never");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1,h2
abcdef,ghijkl
mnopqr,stuvwx
ab\"cd\"ef,gh,ij,kl";
    similar_asserts::assert_eq!(got, expected.to_string());
}
