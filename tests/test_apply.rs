use crate::workdir::Workdir;

#[test]
fn apply_ops_unknown_operation() {
    let wrk = Workdir::new("unknown_op");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("obfuscate")
        .arg("letter")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(&*got, "usage error: Unknown 'obfuscate' operation\n")
}

#[test]
fn apply_ops_upper() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_gender_guess() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["Peter"],
            svec!["Michael"],
            svec!["Joel"],
            svec!["Hussein"],
            svec!["Ian"],
            svec!["Enrique"],
            svec!["Ana"],
            svec!["Olivia"],
            svec!["Mackenzie"],
            svec!["Adair"],
            svec!["Aaf"],
            svec!["Voldemort"],
            svec!["Sami"],
            svec!["Minhaj"],
            svec!["Abdurrahman"],
            svec!["Abbe"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("gender_guess")
        .arg("name")
        .args(["--new-column", "Gender"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "Gender"],
        svec!["Peter", "Male"],
        svec!["Michael", "Male"],
        svec!["Joel", "Male"],
        svec!["Hussein", "Male"],
        svec!["Ian", "Male"],
        svec!["Enrique", "Male"],
        svec!["Ana", "Female"],
        svec!["Olivia", "Female"],
        svec!["Mackenzie", "NotSure"],
        svec!["Adair", "MayBeMale"],
        svec!["Aaf", "MayBeFemale"],
        svec!["Voldemort", "NotFound"],
        svec!["Sami", "Male"],
        svec!["Minhaj", "NotFound"],
        svec!["Abdurrahman", "Male"],
        svec!["Abbe", "NotSure"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_escape() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John😁", "😡Cena"],
            svec!["Mary☎", "Janë"],
            svec!["Sue", "Bird🐦"],
            svec!["Hopękins", "Jæde"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("escape")
        .arg("name,surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname"],
        svec!["John\\u{1f601}", "\\u{1f621}Cena"],
        svec!["Mary\\u{260e}", "Jan\\u{eb}"],
        svec!["Sue", "Bird\\u{1f426}"],
        svec!["Hop\\u{119}kins", "J\\u{e6}de"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_upper_rename() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("--rename")
        .arg("uname,usurname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["uname", "usurname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_upper_rename_invalid() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("--rename")
        .arg("uname")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "usage error: Number of new columns does not match input column selection.\n"
    );
}

#[test]
fn apply_ops_upper_index_params() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("1,2")
        .arg("--rename")
        .arg("uname,usurname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["uname", "usurname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_encode62() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash", "encoded_hash"],
            svec!["3824660653605227303", "4YWz9bdwJXT"],
            svec!["1851770582521928574", "2Cn7zXlsIM2"],
            svec!["7916590694040213670", "9Qo4IeFuLyQ"],
            svec!["10903434754618017012", "CzRqVIpxoUG"],
            svec!["7671262618974725285", "98gSmJMD1XB"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("decode62")
        .arg("2")
        .arg("--new-column")
        .arg("decoded_hash")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["hash", "encoded_hash", "decoded_hash"],
        svec!["3824660653605227303", "4YWz9bdwJXT", "3824660653605227303"],
        svec!["1851770582521928574", "2Cn7zXlsIM2", "1851770582521928574"],
        svec!["7916590694040213670", "9Qo4IeFuLyQ", "7916590694040213670"],
        svec![
            "10903434754618017012",
            "CzRqVIpxoUG",
            "10903434754618017012"
        ],
        svec!["7671262618974725285", "98gSmJMD1XB", "7671262618974725285"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_decode62() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["hash"],
            svec!["3824660653605227303"],
            svec!["1851770582521928574"],
            svec!["7916590694040213670"],
            svec!["10903434754618017012"],
            svec!["7671262618974725285"],
            svec!["this should cause an error"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("encode62")
        .arg("1")
        .arg("--new-column")
        .arg("encoded_hash")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["hash", "encoded_hash"],
        svec!["3824660653605227303", "4YWz9bdwJXT"],
        svec!["1851770582521928574", "2Cn7zXlsIM2"],
        svec!["7916590694040213670", "9Qo4IeFuLyQ"],
        svec!["10903434754618017012", "CzRqVIpxoUG"],
        svec!["7671262618974725285", "98gSmJMD1XB"],
        svec![
            "this should cause an error",
            "encode62 error: ParseIntError { kind: InvalidDigit }"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_encode64() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
            svec![
                "Long",
                "the quick brown fox jumped over the lazy by the zigzag quarry site."
            ],
            svec![
                "With extended characters",
                "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
                 competentes al escalar por las ramas."
            ],
            svec![
                "Japanese",
                "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である"
            ],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("encode64")
        .arg("surname")
        .arg("--new-column")
        .arg("encoded_surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname", "encoded_surname"],
        svec!["John", "Cena", "Q2VuYQ=="],
        svec!["Mary", "Jane", "SmFuZQ=="],
        svec!["Sue", "Bird", "QmlyZA=="],
        svec!["Hopkins", "Jade", "SmFkZQ=="],
        svec!["Long", "the quick brown fox jumped over the lazy by the zigzag quarry site.", 
            "dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wZWQgb3ZlciB0aGUgbGF6eSBieSB0aGUgemlnemFnIHF1YXJyeSBzaXRlLg=="],
        svec!["With extended characters", "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy competentes al escalar por las ramas.", 
            "WSBhc8OtIG1pc21vLCBhdW5xdWUgbm8gc29uIHRhbiDDoWdpbGVzIGVuIGVsIHN1ZWxvIGNvbW8gZWwgdmFtcGlybyBjb23Dum4sIHNvbiBtdXkgY29tcGV0ZW50ZXMgYWwgZXNjYWxhciBwb3IgbGFzIHJhbWFzLg=="],
        svec!["Japanese", "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である", 
            "UnVzdO+8iOODqeOCueODiO+8ieOBr+S4puWIl+OBi+OBpOODnuODq+ODgeODkeODqeODgOOCpOODoOOBruODl+ODreOCsOODqeODn+ODs+OCsOiogOiqnuOBp+OBguOCiw=="],

    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_decode64() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname", "encoded_surname"],
            svec!["John", "Cena", "Q2VuYQ=="],
            svec!["Mary", "Jane", "SmFuZQ=="],
            svec!["Sue", "Bird", "QmlyZA=="],
            svec!["Hopkins", "Jade", "SmFkZQ=="],
            svec!["Long", "the quick brown fox jumped over the lazy by the zigzag quarry site.", 
                "dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wZWQgb3ZlciB0aGUgbGF6eSBieSB0aGUgemlnemFnIHF1YXJyeSBzaXRlLg=="],
            svec!["With extended characters", "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy competentes al escalar por las ramas.", 
                "WSBhc8OtIG1pc21vLCBhdW5xdWUgbm8gc29uIHRhbiDDoWdpbGVzIGVuIGVsIHN1ZWxvIGNvbW8gZWwgdmFtcGlybyBjb23Dum4sIHNvbiBtdXkgY29tcGV0ZW50ZXMgYWwgZXNjYWxhciBwb3IgbGFzIHJhbWFzLg=="],
            svec!["Japanese", "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である", 
                "UnVzdO+8iOODqeOCueODiO+8ieOBr+S4puWIl+OBi+OBpOODnuODq+ODgeODkeODqeODgOOCpOODoOOBruODl+ODreOCsOODqeODn+ODs+OCsOiogOiqnuOBp+OBguOCiw=="],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("decode64")
        .arg("encoded_surname")
        .arg("--new-column")
        .arg("decoded_surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname", "encoded_surname", "decoded_surname"],
        svec!["John", "Cena", "Q2VuYQ==", "Cena"],
        svec!["Mary", "Jane", "SmFuZQ==", "Jane"],
        svec!["Sue", "Bird", "QmlyZA==", "Bird"],
        svec!["Hopkins", "Jade", "SmFkZQ==", "Jade"],
        svec!["Long", "the quick brown fox jumped over the lazy by the zigzag quarry site.", 
            "dGhlIHF1aWNrIGJyb3duIGZveCBqdW1wZWQgb3ZlciB0aGUgbGF6eSBieSB0aGUgemlnemFnIHF1YXJyeSBzaXRlLg==",
            "the quick brown fox jumped over the lazy by the zigzag quarry site."],
        svec!["With extended characters", "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy competentes al escalar por las ramas.", 
            "WSBhc8OtIG1pc21vLCBhdW5xdWUgbm8gc29uIHRhbiDDoWdpbGVzIGVuIGVsIHN1ZWxvIGNvbW8gZWwgdmFtcGlybyBjb23Dum4sIHNvbiBtdXkgY29tcGV0ZW50ZXMgYWwgZXNjYWxhciBwb3IgbGFzIHJhbWFzLg==",
            "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy competentes al escalar por las ramas."],
        svec!["Japanese", "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である", 
            "UnVzdO+8iOODqeOCueODiO+8ieOBr+S4puWIl+OBi+OBpOODnuODq+ODgeODkeODqeODgOOCpOODoOOBruODl+ODreOCsOODqeODn+ODs+OCsOiogOiqnuOBp+OBguOCiw==",
            "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_dynfmt() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec![
                "qty-fruit/day",
                "1fruit",
                "another col",
                "unit cost usd",
                "and another one"
            ],
            svec!["20.5", "mangoes", "a", "5", "z"],
            svec!["10", "bananas", "b", "20", "y"],
            svec!["3", "strawberries", "c", "3.50", "x"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("dynfmt")
        .arg("--formatstr")
        .arg(
            "{qty_fruit_day} helpings of {1fruit} is good for you, even if it costs \
             ${unit_cost_usd} each. {1fruit}, all {qty_fruit_day} - is just worth it!",
        )
        .arg("--new-column")
        .arg("saying")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-fruit/day",
            "1fruit",
            "another col",
            "unit cost usd",
            "and another one",
            "saying"
        ],
        svec![
            "20.5",
            "mangoes",
            "a",
            "5",
            "z",
            "20.5 helpings of mangoes is good for you, even if it costs $5 each. mangoes, all \
             20.5 - is just worth it!"
        ],
        svec![
            "10",
            "bananas",
            "b",
            "20",
            "y",
            "10 helpings of bananas is good for you, even if it costs $20 each. bananas, all 10 - \
             is just worth it!"
        ],
        svec![
            "3",
            "strawberries",
            "c",
            "3.50",
            "x",
            "3 helpings of strawberries is good for you, even if it costs $3.50 each. \
             strawberries, all 3 - is just worth it!"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_dynfmt_keepcase() {
    let wrk = Workdir::new("apply_dynfmt_keepcase");
    wrk.create(
        "data.csv",
        vec![
            svec![
                "qty-FRUIT/day",
                "1fruit",
                "another Col",
                "unit cost USD",
                "and another one"
            ],
            svec!["20.5", "mangoes", "a", "5", "z"],
            svec!["10", "bananas", "b", "20", "y"],
            svec!["3", "strawberries", "c", "3.50", "x"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("dynfmt")
        .arg("--formatstr")
        .arg(
            "{qty_FRUIT_day} helpings of {1fruit} is good for you, even if it costs \
             ${unit_cost_USD} each. {1fruit}, all {qty_FRUIT_day} - is just worth it!",
        )
        .arg("--new-column")
        .arg("saying")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-FRUIT/day",
            "1fruit",
            "another Col",
            "unit cost USD",
            "and another one",
            "saying"
        ],
        svec![
            "20.5",
            "mangoes",
            "a",
            "5",
            "z",
            "20.5 helpings of mangoes is good for you, even if it costs $5 each. mangoes, all \
             20.5 - is just worth it!"
        ],
        svec![
            "10",
            "bananas",
            "b",
            "20",
            "y",
            "10 helpings of bananas is good for you, even if it costs $20 each. bananas, all 10 - \
             is just worth it!"
        ],
        svec![
            "3",
            "strawberries",
            "c",
            "3.50",
            "x",
            "3 helpings of strawberries is good for you, even if it costs $3.50 each. \
             strawberries, all 3 - is just worth it!"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_dynfmt_issue1458() {
    let wrk = Workdir::new("apply_dynfmt_issue1458");
    wrk.create(
        "data.csv",
        vec![
            svec!["FirstName", "MI", "LastName",],
            svec!["Adam", "B", "Case"],
            svec!["Derek", "E", "Foster"],
            svec!["Gordon", "H", "Irvin"],
            svec!["Jack", "K", "Lynch"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("dynfmt")
        .arg("--formatstr")
        .arg("Sir/Madam {FirstName} {MI}. {LastName}")
        .arg("-c")
        .arg("FullName")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["FirstName", "MI", "LastName", "FullName"],
        svec!["Adam", "B", "Case", "Sir/Madam Adam B. Case"],
        svec!["Derek", "E", "Foster", "Sir/Madam Derek E. Foster"],
        svec!["Gordon", "H", "Irvin", "Sir/Madam Gordon H. Irvin"],
        svec!["Jack", "K", "Lynch", "Sir/Madam Jack K. Lynch"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_regex_replace_issue1469() {
    let wrk = Workdir::new("apply_regex_replace_issue1469");
    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3",],
            svec!["(Adam)", "B", "Case(hello)Name "],
            svec!["Derek(foo)", "(bar)E", "Fos(this needs to go)ter"],
            svec!["Gordon", "H", "(cmon)Irvin"],
            svec!["Jack(ie)", "K", "Lynch(-Chan)"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("col1,col2,col3")
        .args(["--comparand", r"\([^)]+\)"])
        .args(["--replacement", "<Null>"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["col1", "col2", "col3"],
        svec!["", "B", "CaseName "],
        svec!["Derek", "E", "Foster"],
        svec!["Gordon", "H", "Irvin"],
        svec!["Jack", "K", "Lynch"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_calcconv() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["qty-fruit/day", "fruit", "calories", "unit cost usd"],
            svec!["20.5", "mangoes", "200", "2"],
            svec!["10", "bananas", "120", "0.50"],
            svec!["3", "strawberries", "20", "0.10"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("calcconv")
        .arg("--formatstr")
        .arg("{qty_fruit_day} * {calories} calories to watt hours")
        .arg("--new-column")
        .arg("watthours")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-fruit/day",
            "fruit",
            "calories",
            "unit cost usd",
            "watthours",
        ],
        svec!["20.5", "mangoes", "200", "2", "4.7683000"],
        svec!["10", "bananas", "120", "0.50", "1.395600"],
        svec!["3", "strawberries", "20", "0.10", "0.069780"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_calcconv_invalid() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["qty-fruit/day", "fruit", "calories", "unit cost usd"],
            svec!["20.5", "mangoes", "200", "2"],
            svec!["10", "bananas", "120", "0.50"],
            svec!["3", "strawberries", "20", "0.10"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("calcconv")
        .arg("--formatstr")
        .arg("{qty_fruit_day} * {calories} calories to bitcoins per sec")
        .arg("--new-column")
        .arg("watthours")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-fruit/day",
            "fruit",
            "calories",
            "unit cost usd",
            "watthours",
        ],
        svec![
            "20.5",
            "mangoes",
            "200",
            "2",
            "ERROR: Lexing error: Invalid string: bitcoins"
        ],
        svec![
            "10",
            "bananas",
            "120",
            "0.50",
            "ERROR: Lexing error: Invalid string: bitcoins"
        ],
        svec![
            "3",
            "strawberries",
            "20",
            "0.10",
            "ERROR: Lexing error: Invalid string: bitcoins"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_calcconv_units() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["qty-fruit/day", "fruit", "calories", "unit cost usd"],
            svec!["20.5", "mangoes", "200", "2"],
            svec!["10", "bananas", "120", "0.50"],
            svec!["3", "strawberries", "20", "0.10"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("calcconv")
        .arg("--formatstr")
        .arg("{qty_fruit_day} * {calories} calories to watt hours <UNIT>")
        .arg("--new-column")
        .arg("watthours")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-fruit/day",
            "fruit",
            "calories",
            "unit cost usd",
            "watthours",
        ],
        svec!["20.5", "mangoes", "200", "2", "4.7683000 WattHour"],
        svec!["10", "bananas", "120", "0.50", "1.395600 WattHour"],
        svec!["3", "strawberries", "20", "0.10", "0.069780 WattHour"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enum_apply_calconv_issue1458() {
    let wrk = Workdir::new("enum_apply_calconv_issue1458");
    wrk.create(
        "data.csv",
        vec![
            svec!["FirstName", "MI", "LastName",],
            svec!["Adam", "B", "Case"],
            svec!["Derek", "E", "Foster"],
            svec!["Gordon", "H", "Irvin"],
            svec!["Jack", "K", "Lynch"],
        ],
    );

    // qsv enum file.csv | qsv apply calcconv --formatstr '{index} * {index}' -c result

    let mut cmd = wrk.command("enum");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["FirstName", "MI", "LastName", "index"],
        svec!["Adam", "B", "Case", "0"],
        svec!["Derek", "E", "Foster", "1"],
        svec!["Gordon", "H", "Irvin", "2"],
        svec!["Jack", "K", "Lynch", "3"],
    ];

    assert_eq!(got, expected);

    wrk.create("enum.csv", got);

    let mut cmd_2 = wrk.command("apply");
    cmd_2
        .arg("calcconv")
        .arg("--formatstr")
        .arg("{index} * {index}")
        .arg("-c")
        .arg("result")
        .arg("enum.csv");

    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_2);
    let expected2 = vec![
        svec!["FirstName", "MI", "LastName", "index", "result"],
        svec!["Adam", "B", "Case", "0", "0"],
        svec!["Derek", "E", "Foster", "1", "1"],
        svec!["Gordon", "H", "Irvin", "2", "4"],
        svec!["Jack", "K", "Lynch", "3", "9"],
    ];

    assert_eq!(got2, expected2);
}

#[test]
fn apply_ops_empty_shortcircuit() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![""],
            svec![""],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations").arg("len").arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["4"],
        svec!["0"],
        svec!["0"],
        svec!["7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_titlecase() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle little star, how I wonder what you are"],
            svec!["a simple title to capitalize: an example"],
            svec!["new york city police department - NYPD"],
            svec!["department of human services"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("titlecase")
        .arg("description")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["The Quick Brown Fox Jumped Over the Lazy Dog."],
        svec!["Twinkle, Twinkle Little Star, How I Wonder What You Are"],
        svec!["A Simple Title to Capitalize: An Example"],
        svec!["New York City Police Department - NYPD"],
        svec!["Department of Human Services"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_check() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ you!"],
            svec!["F_u c_K"],
            svec!["fuuuuuuuck"],
            svec!["fluff truck"],
            svec!["fukushima"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_check")
        .arg("description")
        .arg("--new-column")
        .arg("profanity_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_flag"],
        svec!["fuck", "true"],
        svec!["FUCK", "true"],
        svec!["fμ¢κ you!", "true"],
        svec!["F_u c_K", "true"],
        svec!["fuuuuuuuck", "true"],
        svec!["fluff truck", "false"],
        svec!["fukushima", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_count() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ you!"],
            svec!["F_u c_K"],
            svec!["fuuuuuuuck"],
            svec!["fluff truck"],
            svec!["fukushima"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_count")
        .arg("description")
        .arg("--new-column")
        .arg("profanity_count")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_count"],
        svec!["fuck", "1"],
        svec!["FUCK", "1"],
        svec!["fμ¢κ you!", "1"],
        svec!["F_u c_K", "4"],
        svec!["fuuuuuuuck", "1"],
        svec!["fluff truck", "0"],
        svec!["fukushima", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ that shit, faggot!"],
            svec!["F_u c_K that blowjoboobies"],
            svec!["fuuuuuuuck yooooouuuu"],
            svec!["kiss my ass!"],
            svec!["shittitties"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor")
        .arg("description")
        .arg("--new-column")
        .arg("censored_text")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "censored_text"],
        svec!["fuck", "****"],
        svec!["FUCK", "****"],
        svec!["fμ¢κ that shit, faggot!", "**** that ****, ******!"],
        svec!["F_u c_K that blowjoboobies", "*_* *_* that *************"],
        svec!["fuuuuuuuck yooooouuuu", "********** yooooouuuu"],
        svec!["kiss my ass!", "kiss my ***!"],
        svec!["shittitties", "***********"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_check_addlwords() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ you!"],
            svec!["F_u c_K"],
            svec!["fuuuuuuuck"],
            svec!["fluff truck"],
            svec!["fukushima"],
            svec!["shlong dong ding"],
            svec!["long john silver's shlong"],
            svec!["Whoa! I see her cameltoe thru her athleisure!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_check")
        .arg("description")
        .arg("--comparand")
        .arg("shlong,dong,cameltoe")
        .arg("--new-column")
        .arg("profanity_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_flag"],
        svec!["fuck", "true"],
        svec!["FUCK", "true"],
        svec!["fμ¢κ you!", "true"],
        svec!["F_u c_K", "true"],
        svec!["fuuuuuuuck", "true"],
        svec!["fluff truck", "false"],
        svec!["fukushima", "false"],
        svec!["shlong dong ding", "true"],
        svec!["long john silver's shlong", "true"],
        svec!["Whoa! I see her cameltoe thru her athleisure!", "true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_addlwords() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ that shit, faggot!"],
            svec!["F_u c_K that blowjoboobies"],
            svec!["fuuuuuuuck yooooouuuu"],
            svec!["kiss my ass!"],
            svec!["shittitties"],
            svec!["move your shlllooooonng!!!"],
            svec!["that cameltoe is so penistracting!"],
            svec!["ding dong the bitch is dead!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor")
        .arg("description")
        .arg("--comparand")
        .arg("shlong, dong, cameltoe, bitch")
        .arg("--new-column")
        .arg("censored_text")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "censored_text"],
        svec!["fuck", "****"],
        svec!["FUCK", "****"],
        svec!["fμ¢κ that shit, faggot!", "**** that ****, ******!"],
        svec!["F_u c_K that blowjoboobies", "*_* *_* that *************"],
        svec!["fuuuuuuuck yooooouuuu", "********** yooooouuuu"],
        svec!["kiss my ass!", "kiss my ***!"],
        svec!["shittitties", "***********"],
        svec!["move your shlllooooonng!!!", "move your *************!!!"],
        svec![
            "that cameltoe is so penistracting!",
            "that ******** is so *****tracting!"
        ],
        svec![
            "ding dong the bitch is dead!",
            "ding **** the ***** is dead!"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_count_addlwords() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ that shit, faggot!"],
            svec!["F_u c_K that blowjoboobies"],
            svec!["fuuuuuuuck yooooouuuu"],
            svec!["kiss my ass!"],
            svec!["shittitties"],
            svec!["move your shlllooooonng!!!"],
            svec!["that cameltoe is so penistracting!"],
            svec!["ding dong the bitch is dead!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_count")
        .arg("description")
        .arg("--comparand")
        .arg("shlong, dong, cameltoe, bitch")
        .arg("--new-column")
        .arg("profanity_count")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_count"],
        svec!["fuck", "1"],
        svec!["FUCK", "1"],
        svec!["fμ¢κ that shit, faggot!", "3"],
        svec!["F_u c_K that blowjoboobies", "5"],
        svec!["fuuuuuuuck yooooouuuu", "1"],
        svec!["kiss my ass!", "1"],
        svec!["shittitties", "1"],
        svec!["move your shlllooooonng!!!", "1"],
        svec!["that cameltoe is so penistracting!", "2"],
        svec!["ding dong the bitch is dead!", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_replace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
            svec!["a simple title to capitalize: an example"],
            svec!["Mr. Brown is not pleased."],
            svec!["this is a brownado car"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("replace")
        .arg("description")
        .arg("--comparand")
        .arg("brown")
        .arg("--replacement")
        .arg("silver")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["THE quick silver fox jumped over the lazy dog."],
        svec!["twinkle, twinkle silverie star, how I wonder what you are"],
        svec!["a simple title to capitalize: an example"],
        svec!["Mr. Brown is not pleased."],
        svec!["this is a silverado car"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_replace_validation_error() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("replace")
        .arg("description")
        .arg("--replacement")
        .arg("silver")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "usage error: --comparand (-C) and --replacement (-R) are required for replace \
         operation.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn apply_ops_regex_replace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
            svec!["Somebody from Nigeria called asked for my ssn - 987-65-4320."],
            svec!["Won't fall for that scam!"],
            svec!["Just enter 987-65-4329 when prompted. Also try 987-65-1234 if it doesn't work."],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:\\d{3}-\\d{2}-\\d{4})")
        .arg("--replacement")
        .arg("SSN")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["My SSN is SSN. Please do not share it."],
        svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        svec!["Somebody from Nigeria called asked for my ssn - SSN."],
        svec!["Won't fall for that scam!"],
        svec!["Just enter SSN when prompted. Also try SSN if it doesn't work."],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_regex_replace_validation_error() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:\\d{3}-\\d{2}-\\d{4})")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "usage error: --comparand (-C) and --replacement (-R) are required for regex_replace \
         operation.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn apply_ops_regex_replace_error() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:?)") // invalid regular expression
        .arg("--replacement")
        .arg("SSN")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("regex_replace expression error"));
}

#[test]
fn apply_ops_mtrim() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["(This is in parentheses)"],
            svec!["(This is in parentheses, but with a period)."],
            svec!["(Only left paren"],
            svec!["Only right paren)"],
            svec!["(((multiple parens)))"],
            svec!["Embedded (((multiple parens)))"],
            svec![")))reverse parens((("],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("mtrim")
        .arg("description")
        .arg("--comparand")
        .arg("()")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["This is in parentheses"],
        svec!["This is in parentheses, but with a period)."],
        svec!["Only left paren"],
        svec!["Only right paren"],
        svec!["multiple parens"],
        svec!["Embedded (((multiple parens"],
        svec!["reverse parens"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_chain() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper,squeeze")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_chain_validation_error() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["   John       Paul   "], svec!["Mary"]],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper,squeeze,simdl,simod")
        .arg("name")
        .arg("--comparand")
        .arg("Joe")
        .arg("-c")
        .arg("new_column")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "usage error: you can only use censor(0), copy(0), eudex(0), regex_replace(0), \
         replace(0), sentiment(0), similarity(2), strip(0), and whatlang(0) ONCE per operation \
         series.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn apply_ops_chain_validation_error_missing_comparand() {
    let wrk = Workdir::new("apply_ops_chain_validation_error_missing_comparand");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["   John       Paul   "], svec!["Mary"]],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper,squeeze,simdl,simod")
        .arg("name")
        .arg("-c")
        .arg("new_column")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "usage error: --comparand (-C) and --new_column (-c) is required for similarity \
         operations.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn apply_ops_chain_squeeze0() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper,squeeze0")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHNPAUL"],
        svec!["MARY"],
        svec!["MARYSUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_squeeze0() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John   \t    Paul   "],
            svec!["    Mary \t   "],
            svec!["  Mary    \n  Sue"],
            svec!["John\r\nHopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("squeeze0")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JohnPaul"],
        svec!["Mary"],
        svec!["MarySue"],
        svec!["JohnHopkins"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_chain_strip() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["Doctor   John       Paul   "],
            svec!["DocTor Mary"],
            svec!["  Mary    Sue"],
            svec!["doctor Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("squeeze,upper,strip_prefix,trim")
        .arg("name")
        .arg("--comparand")
        .arg("DOCTOR")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_mixed_case_chain() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("Trim,UPPER,squEeZe")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_no_headers() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["John   "],
            svec!["Mary"],
            svec!["  Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("trim,upper")
        .arg("1")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["JOHN"], svec!["MARY"], svec!["SUE"], svec!["HOPKINS"]];
    assert_eq!(got, expected);
}

#[test]
fn apply_rename() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name")
        .arg("--rename")
        .arg("upper_name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["upper_name"],
        svec!["JOHN"],
        svec!["MARY"],
        svec!["SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_new_column() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("upper")
        .arg("name")
        .arg("--new-column")
        .arg("upper_name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "upper_name"],
        svec!["John", "JOHN"],
        svec!["Mary", "MARY"],
        svec!["Sue", "SUE"],
        svec!["Hopkins", "HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_thousands() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["0"],
            svec!["5"],
            svec!["-123456789"],
            svec!["-123456789.12345678"],
            svec!["-123456789.0"],
            svec!["-123456789.123"],
            svec!["0"],
            svec!["-5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("thousands")
        .arg("number")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123,456,789"],
        svec!["123,456,789.12345678"],
        svec!["123,456,789"],
        svec!["123,456,789.123"],
        svec!["0"],
        svec!["5"],
        svec!["-123,456,789"],
        svec!["-123,456,789.12345678"],
        svec!["-123,456,789"],
        svec!["-123,456,789.123"],
        svec!["0"],
        svec!["-5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_thousands_space() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("thousands")
        .arg("number")
        .args(["--formatstr", "space"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123 456 789"],
        svec!["123 456 789.12345678"],
        svec!["123 456 789"],
        svec!["123 456 789.123"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_thousands_indiancomma() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("thousands")
        .arg("number")
        .args(["--formatstr", "indiancomma"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["12,34,56,789"],
        svec!["12,34,56,789.12345678"],
        svec!["12,34,56,789"],
        svec!["12,34,56,789.123"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_thousands_eurostyle() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["12345123456789.123"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("thousands")
        .arg("number")
        .args(["--formatstr", "dot"])
        .args(["--replacement", ","])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123.456.789"],
        svec!["123.456.789,12345678"],
        svec!["123.456.789"],
        svec!["123.456.789,123"],
        svec!["12.345.123.456.789,123"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_thousands_hexfour() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["12345123456789.123"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("thousands")
        .arg("number")
        .args(["--formatstr", "hexfour"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["1 2345 6789"],
        svec!["1 2345 6789.12345678"],
        svec!["1 2345 6789"],
        svec!["1 2345 6789.123"],
        svec!["12 3451 2345 6789.123"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_round() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["123456789.12398"],
            svec!["0"],
            svec!["5"],
            svec!["-123456789"],
            svec!["-123456789.12345678"],
            svec!["-123456789.0"],
            svec!["-123456789.123"],
            svec!["-123456789.12398"],
            svec!["-0"],
            svec!["-5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("round")
        .arg("number")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123456789"],
        svec!["123456789.123"],
        svec!["123456789"],
        svec!["123456789.123"],
        svec!["123456789.124"],
        svec!["0"],
        svec!["5"],
        svec!["-123456789"],
        svec!["-123456789.123"],
        svec!["-123456789"],
        svec!["-123456789.123"],
        svec!["-123456789.124"],
        svec!["0"],
        svec!["-5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_round_5places() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["123456789.1239876"],
            svec!["123456789.1239844"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("round")
        .args(["--formatstr", "5"])
        .arg("number")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123456789"],
        svec!["123456789.12346"],
        svec!["123456789"],
        svec!["123456789.123"],
        svec!["123456789.12399"],
        svec!["123456789.12398"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_currencytonum() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["money"],
            svec!["$10.00"],
            svec!["$-10.00"],
            svec!["$ 12 500.00"],
            svec!["$5"],
            svec!["0"],
            svec!["5"],
            svec!["$0.25"],
            svec!["$ 10.05"],
            svec!["¥10,000,000.00"],
            svec!["£423.56"],
            svec!["€120.00"],
            svec!["֏99,999.50"],
            svec!["€300 999,55"],
            svec!["This is not money. Leave untouched."],
            svec!["₱1,234,567.89"],
            svec!["₽234,567.89"],
            svec!["₪ 567.89"],
            svec!["₩ 567.89"],
            svec!["₩ 89,123.0"],
            svec!["ƒ 123,456.00"],
            svec!["฿ 789,123"],
            svec!["₫ 456"],
            svec!["123,456.00 $"],
            svec!["USD 10,000"],
            svec!["EUR 1234.50"],
            svec!["JPY 9,999,999.99"],
            svec!["RMB 6543.21"],
            svec!["$10.0099"],
            svec!["$10.0777"],
            svec!["$10.0723"],
            svec!["$10.8723"],
            svec!["$10.77777"],
            svec!["$10.777"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("currencytonum")
        .arg("money")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["10.00"],
        svec!["-10.00"],
        svec!["12500.00"],
        svec!["5.00"],
        svec!["0"],
        svec!["5.00"],
        svec!["0.25"],
        svec!["10.05"],
        svec!["10000000.00"],
        svec!["423.56"],
        svec!["120.00"],
        svec!["99999.50"],
        svec!["300999.55"],
        svec!["This is not money. Leave untouched."],
        svec!["1234567.89"],
        svec!["234567.89"],
        svec!["567.89"],
        svec!["567.89"],
        svec!["89123.00"],
        svec!["123456.00"],
        svec!["789123.00"],
        svec!["456.00"],
        svec!["123456.00"],
        svec!["10000.00"],
        svec!["1234.50"],
        svec!["9999999.99"],
        svec!["6543.21"],
        svec!["10.01"],
        svec!["10.08"],
        svec!["10.07"],
        svec!["10.87"],
        svec!["10.78"],
        svec!["10.78"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_numtocurrency() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["money"],
            svec!["$10.00"],
            svec!["$-10.00"],
            svec!["$ 12 500.00"],
            svec!["$5"],
            svec!["0"],
            svec!["5"],
            svec!["$0.25"],
            svec!["$ 10.05"],
            svec!["¥10,000,000.00"],
            svec!["£423.56"],
            svec!["€120.00"],
            svec!["֏99,999.50"],
            svec!["€300 999,55"],
            svec!["This is not money. Set to zero."],
            svec!["₱1,234,567.89"],
            svec!["₽234,567.89"],
            svec!["₪ 567.89"],
            svec!["₩ 567.89"],
            svec!["₩ 89,123.0"],
            svec!["ƒ 123,456.00"],
            svec!["฿ 789,123"],
            svec!["₫ 456"],
            svec!["123,456.00 $"],
            svec!["USD 10,000"],
            svec!["EUR 1234.50"],
            svec!["JPY 9,999,999.99"],
            svec!["RMB 6543.21"],
            svec!["$10.0099"],
            svec!["$10.0777"],
            svec!["$10.0723"],
            svec!["$10.8723"],
            svec!["$10.77777"],
            svec!["$10.777"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("numtocurrency")
        .arg("--comparand")
        .arg("$")
        .arg("money")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["$10.00"],
        svec!["-$10.00"],
        svec!["$12,500.00"],
        svec!["$5.00"],
        svec!["$0.00"],
        svec!["$5.00"],
        svec!["$0.25"],
        svec!["$10.05"],
        svec!["$10,000,000.00"],
        svec!["$423.56"],
        svec!["$120.00"],
        svec!["$99,999.50"],
        svec!["$300,999.55"],
        svec!["$0.00"],
        svec!["$1,234,567.89"],
        svec!["$234,567.89"],
        svec!["$567.89"],
        svec!["$567.89"],
        svec!["$89,123.00"],
        svec!["$123,456.00"],
        svec!["$789,123.00"],
        svec!["$456.00"],
        svec!["$123,456.00"],
        svec!["$10,000.00"],
        svec!["$1,234.50"],
        svec!["$9,999,999.99"],
        svec!["$6,543.21"],
        svec!["$10.01"],
        svec!["$10.08"],
        svec!["$10.07"],
        svec!["$10.87"],
        svec!["$10.78"],
        svec!["$10.78"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_numtocurrency_convert() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["money"],
            svec!["$10.00"],
            svec!["$-10.00"],
            svec!["$ 12 500.00"],
            svec!["$5"],
            svec!["0"],
            svec!["5"],
            svec!["$0.25"],
            svec!["$ 10.05"],
            svec!["¥10,000,000.00"],
            svec!["£423.56"],
            svec!["€120.00"],
            svec!["֏99,999.50"],
            svec!["€300 999,55"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("numtocurrency")
        .arg("--comparand")
        .arg("EUR ")
        .arg("--replacement")
        .arg("1.5")
        .arg("money")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["EUR 15.00"],
        svec!["-EUR 15.00"],
        svec!["EUR 18,750.00"],
        svec!["EUR 7.50"],
        svec!["EUR 0.00"],
        svec!["EUR 7.50"],
        svec!["EUR 0.37"],
        svec!["EUR 15.07"],
        svec!["EUR 15,000,000.00"],
        svec!["EUR 635.34"],
        svec!["EUR 180.00"],
        svec!["EUR 149,999.25"],
        svec!["EUR 451,499.32"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_numtocurrency_convert_euro_format() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["money"],
            svec!["$10.00"],
            svec!["$-10.00"],
            svec!["$ 12 500.00"],
            svec!["$5"],
            svec!["0"],
            svec!["5"],
            svec!["$0.25"],
            svec!["$ 10.05"],
            svec!["¥10,000,000.00"],
            svec!["£423.56"],
            svec!["€120.00"],
            svec!["֏99,999.50"],
            svec!["€300 999,55"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("numtocurrency")
        .arg("-C")
        .arg("EUR ")
        .arg("--replacement")
        .arg("1.5")
        .arg("--formatstr")
        .arg("euro")
        .arg("money")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["EUR 15,00"],
        svec!["-EUR 15,00"],
        svec!["EUR 18.750,00"],
        svec!["EUR 7,50"],
        svec!["EUR 0,00"],
        svec!["EUR 7,50"],
        svec!["EUR 0,37"],
        svec!["EUR 15,07"],
        svec!["EUR 15.000.000,00"],
        svec!["EUR 635,34"],
        svec!["EUR 180,00"],
        svec!["EUR 149.999,25"],
        svec!["EUR 451.499,32"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_similarity() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Jonathan"],
            svec!["Edna"],
            svec!["Larry"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("simdln")
        .arg("name")
        .arg("--comparand")
        .arg("Joe")
        .arg("--new-column")
        .arg("name_sim_score")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "name_sim_score"],
        svec!["John", "0.5"],
        svec!["Jonathan", "0.25"],
        svec!["Edna", "0.0"],
        svec!["Larry", "0.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_similarity_eudex() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Jonathan"],
            svec!["Michelle"],
            svec!["Larry"],
            svec!["Joel"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("lower,eudex")
        .arg("name")
        .arg("--comparand")
        .arg("michael")
        .arg("--new-column")
        .arg("eudex_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "eudex_flag"],
        svec!["John", "false"],
        svec!["Jonathan", "false"],
        svec!["Michelle", "true"],
        svec!["Larry", "false"],
        svec!["Joel", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_similarity_more_eudex() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["Jeuses"],
            svec!["Josephina"],
            svec!["Juan"],
            svec!["Juanita"],
            svec!["Michael"],
            svec!["Jingjing"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("lower,eudex")
        .arg("name")
        .arg("--comparand")
        .arg("Jesus")
        .arg("--new-column")
        .arg("eudex_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "eudex_flag"],
        svec!["Jeuses", "true"],
        svec!["Josephina", "false"],
        svec!["Juan", "true"],
        svec!["Juanita", "true"],
        svec!["Michael", "false"],
        svec!["Jingjing", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_sentiment() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["customer comment"],
            svec!["This is ridiculous! I will never buy from this company again!"],
            svec![
                "Josephina was awesome! She was very helpful and patient. I wish more customer \
                 service folks are like her!"
            ],
            svec!["I can't believe that garbage is still out there. That is so false!"],
            svec!["5 stars! Highly recommended!"],
            svec!["What were they thinking!?!"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("sentiment")
        .arg("customer comment")
        .arg("--new-column")
        .arg("sentiment_score")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["customer comment", "sentiment_score"],
        svec![
            "This is ridiculous! I will never buy from this company again!",
            "-0.47384376462380107"
        ],
        svec![
            "Josephina was awesome! She was very helpful and patient. I wish more customer \
             service folks are like her!",
            "0.9227060290926788"
        ],
        svec![
            "I can't believe that garbage is still out there. That is so false!",
            "-0.07518070500292766"
        ],
        svec!["5 stars! Highly recommended!", "0.3973495344831422"],
        svec!["What were they thinking!?!", "-0.19353437967075598"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_whatlang() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec![
                "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
                 competentes al escalar por las ramas."
            ],
            svec!["See notes."],
            svec![
                "Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre \
                 la meva més gran preocupació."
            ],
            svec![""],
            svec![
                "Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated \
                 fortifications still had defensive usefulness."
            ],
            svec![
                "民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 \
                 ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。"
            ],
            svec!["Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である"],
            svec![
                "Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя \
                 заставил И лучше выдумать не мог."
            ],
            svec![
                "Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high \
                 school?!"
            ],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("whatlang")
        .arg("description")
        .arg("--new-column")
        .arg("language")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "language"],
        svec![
            "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
             competentes al escalar por las ramas.",
            "Spa"
        ],
        svec!["See notes.", "Cat(0.031)?"],
        svec![
            "Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre la \
             meva més gran preocupació.",
            "Cat"
        ],
        svec!["", ""],
        svec![
            "Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated \
             fortifications still had defensive usefulness.",
            "Eng"
        ],
        svec![
            "民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 \
             ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。",
            "Cmn"
        ],
        svec![
            "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である",
            "Jpn"
        ],
        svec![
            "Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя заставил И \
             лучше выдумать не мог.",
            "Rus"
        ],
        svec![
            "Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high school?!",
            "Tgl"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_whatlang_high_confidence_threshold() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec![
                "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
                 competentes al escalar por las ramas."
            ],
            svec!["See notes."],
            svec![
                "Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre \
                 la meva més gran preocupació."
            ],
            svec!["amikor a Fafnir "],
            svec![
                "Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated \
                 fortifications still had defensive usefulness."
            ],
            svec![
                "民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 \
                 ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。"
            ],
            svec!["Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である"],
            svec![
                "Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя \
                 заставил И лучше выдумать не мог."
            ],
            svec![
                "Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high \
                 school?!"
            ],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("whatlang")
        .arg("description")
        .arg("--comparand")
        .arg("0.95?")
        .arg("--new-column")
        .arg("language")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "language"],
        svec![
            "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
             competentes al escalar por las ramas.",
            "Spa(1.000)"
        ],
        svec!["See notes.", "Cat(0.031)?"],
        svec![
            "Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre la \
             meva més gran preocupació.",
            "Cat(1.000)"
        ],
        svec!["amikor a Fafnir ", "Por(0.011)?"],
        svec![
            "Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated \
             fortifications still had defensive usefulness.",
            "Eng(1.000)"
        ],
        svec![
            "民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 \
             ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。",
            "Cmn(1.000)"
        ],
        svec![
            "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である",
            "Jpn(1.000)"
        ],
        svec![
            "Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя заставил И \
             лучше выдумать не мог.",
            "Rus(1.000)"
        ],
        svec![
            "Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high school?!",
            "Tgl(1.000)"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_whatlang_low_confidence_threshold() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec![
                "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
                 competentes al escalar por las ramas."
            ],
            svec!["See notes."],
            svec![
                "Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre \
                 la meva més gran preocupació."
            ],
            svec!["amikor a Fafnir "],
            svec![
                "Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated \
                 fortifications still had defensive usefulness."
            ],
            svec![
                "民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 \
                 ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。"
            ],
            svec!["Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である"],
            svec![
                "Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя \
                 заставил И лучше выдумать не мог."
            ],
            svec![
                "Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high \
                 school?!"
            ],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("whatlang")
        .arg("description")
        .arg("--comparand")
        .arg("0.03")
        .arg("--new-column")
        .arg("language")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "language"],
        svec![
            "Y así mismo, aunque no son tan ágiles en el suelo como el vampiro común, son muy \
             competentes al escalar por las ramas.",
            "Spa"
        ],
        svec!["See notes.", "Cat"],
        svec![
            "Aquest és l’honor més gran que he rebut a la meva vida. La pau ha estat sempre la \
             meva més gran preocupació.",
            "Cat"
        ],
        svec!["amikor a Fafnir ", "Por(0.011)?"],
        svec![
            "Showing that even in the modern warfare of the 1930s and 1940s, the dilapidated \
             fortifications still had defensive usefulness.",
            "Eng"
        ],
        svec![
            "民國卅八年（ 1949年 ）， 從南京經 廣州 、 香港返回 香日德。 1950年6月 \
             ，受十世班禪派遣， 前往西安代表班禪向彭德懷投誠 。",
            "Cmn"
        ],
        svec![
            "Rust（ラスト）は並列かつマルチパラダイムのプログラミング言語である",
            "Jpn"
        ],
        svec![
            "Мой дядя самых честных правил, Когда не в шутку занемог, Он уважать себя заставил И \
             лучше выдумать не мог.",
            "Rus"
        ],
        svec![
            "Kamusta na, pare!?! Matagal na tayong di nagkita! Ilang taon na since high school?!",
            "Tgl"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![" "],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["John"],
        svec!["NA"],
        svec!["Sue"],
        svec!["Hopkins"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace_multiple_columns() {
    let wrk = Workdir::new("apply_emptyreplace_multiple_columns");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age", "city"],
            svec!["John", "30", "New York"],
            svec![" ", " ", "      "],
            svec!["Sue", " ", "Boston"],
            svec!["Hopkins", "40", ""],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("name,age,city")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age", "city"],
        svec!["John", "30", "New York"],
        svec!["NA", "NA", "NA"],
        svec!["Sue", "NA", "Boston"],
        svec!["Hopkins", "40", "NA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace_all_columns() {
    let wrk = Workdir::new("apply_emptyreplace_all_columns");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age", "city"],
            svec!["John", "30", "New York"],
            svec![" ", " ", "      "],
            svec!["Sue", " ", "Boston"],
            svec!["Hopkins", "40", ""],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("1-")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age", "city"],
        svec!["John", "30", "New York"],
        svec!["NA", "NA", "NA"],
        svec!["Sue", "NA", "Boston"],
        svec!["Hopkins", "40", "NA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_crc32() {
    let wrk = Workdir::new("apply_crc32");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Sue"],
            svec!["Hopkins"],
            svec!["TheQuickBrownFoxJumpedOverTheLazyDogByTheZigzagQuarrySite 1234567890 &^#@09"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("crc32")
        .arg("name")
        .arg("--new-column")
        .arg("crc32_hash")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "crc32_hash"],
        svec!["John", "2437433000"],
        svec!["Sue", "4264251807"],
        svec!["Hopkins", "1940610850"],
        svec![
            "TheQuickBrownFoxJumpedOverTheLazyDogByTheZigzagQuarrySite 1234567890 &^#@09",
            "4056627688"
        ],
    ];

    assert_eq!(got, expected);
}
