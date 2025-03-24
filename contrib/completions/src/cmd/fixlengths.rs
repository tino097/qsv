use clap::{arg, Command};

pub fn fixlengths_cmd() -> Command {
    Command::new("fixlengths").args([
        arg!(--length),
        arg!(--"remove-empty"),
        arg!(--insert),
        arg!(--quote),
        arg!(--escape),
        arg!(--output),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
