use clap::{arg, Command};

pub fn join_cmd() -> Command {
    Command::new("join").args([
        arg!(--"left"),
        arg!(--"left-anti"),
        arg!(--"left-semi"),
        arg!(--right),
        arg!(--"right-anti"),
        arg!(--"right-semi"),
        arg!(--full),
        arg!(--cross),
        arg!(--nulls),
        arg!(--"keys-output"),
        arg!(--"ignore-case"),
        arg!(--"ignore-leading-zeros"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
