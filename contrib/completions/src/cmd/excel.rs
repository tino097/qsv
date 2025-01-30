use clap::{arg, Command};

pub fn excel_cmd() -> Command {
    Command::new("excel").args([
        arg!(--sheet),
        arg!(--"header-row"),
        arg!(--metadata),
        arg!(--table),
        arg!(--range),
        arg!(--"error-format"),
        arg!(--flexible),
        arg!(--trim),
        arg!(--"date-format"),
        arg!(--"keep-zero-time"),
        arg!(--jobs),
        arg!(--output),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
