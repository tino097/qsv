use crate::workdir::Workdir;

#[test]
fn sample_seed() {
    let wrk = Workdir::new("sample_seed");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("5").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["6", "e"],
        svec!["8", "h"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_delimiter() {
    let wrk = Workdir::new("sample_seed_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
        b'|',
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("5").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R|S"],
        svec!["6|e"],
        svec!["8|h"],
        svec!["3|d"],
        svec!["7|i"],
        svec!["5|f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_faster() {
    let wrk = Workdir::new("sample_seed_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "faster"])
        .args(["--seed", "42"])
        .arg("5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["8", "h"],
        svec!["2", "a"],
        svec!["7", "i"],
        svec!["4", "c"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_secure() {
    let wrk = Workdir::new("sample_seed_secure");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["6", "e"],
        svec!["3", "d"],
        svec!["4", "c"],
        svec!["8", "h"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_url() {
    let wrk = Workdir::new("sample_seed_url");

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"])
        .arg("5")
        .arg("https://github.com/dathere/qsv/raw/master/resources/test/aliases.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        ["position", "title"],
        ["Q107058011", "ambassador to Mauritania"],
        [
            "Q100797227",
            "Minister of Family and Social Services of the Government of the Balearic Islands",
        ],
        [
            "Q106968387",
            "Minister of Research and Universities of the Government of Catalonia",
        ],
        ["Q106918017", "conseller d'Obres Públiques i Urbanisme"],
        [
            "Q106162142",
            "Conseiller aux Infrastructures, au Territoire et à l'Environnement de la Généralité \
             valencienne",
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_no_index_percentage() {
    let wrk = Workdir::new("sample_percentage_seed_no_index_percentage");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("0.6").arg("in.csv");

    // no error since percentage sampling no longer requires an index
    // though note the results are different even with the same seed and
    // sample size. This is because we use sample_reservoir method, not
    // sample_random_access method
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["6", "e"],
        svec!["8", "h"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["8", "h"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed() {
    let wrk = Workdir::new("sample_indexed");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("0.4").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["8", "h"],
        svec!["7", "i"],
        svec!["4", "c"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed_faster() {
    let wrk = Workdir::new("sample_indexed_faster");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "faster"])
        .args(["--seed", "42"])
        .arg("0.4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["8", "h"],
        svec!["6", "e"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed_secure() {
    let wrk = Workdir::new("sample_indexed_secure");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("0.4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["6", "e"],
        svec!["3", "d"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access() {
    let wrk = Workdir::new("sample_indexed_random_access");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("4").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["18", "r"],
        svec!["17", "q"],
        svec!["14", "n"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access_faster() {
    let wrk = Workdir::new("sample_indexed_random_access_faster");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "faster"])
        .args(["--seed", "42"])
        .arg("4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["22", "v"],
        svec!["26", "z"],
        svec!["23", "w"],
        svec!["10", "j"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access_secure() {
    let wrk = Workdir::new("sample_indexed_random_access_secure");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--rng", "cryptosecure"])
        .args(["--seed", "42"])
        .arg("4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["22", "v"],
        svec!["23", "w"],
        svec!["5", "f"],
        svec!["12", "l"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_negative_sample_size_error() {
    let wrk = Workdir::new("sample_negative");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("-5").arg("in.csv");

    wrk.assert_err(&mut cmd);
}
