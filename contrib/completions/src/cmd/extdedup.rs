use clap::{arg, Command};

pub fn extdedup_cmd() -> Command {
    Command::new("extdedup").args([
        arg!(--select),
        arg!(--"no-output"),
        arg!(--"dupes-output"),
        arg!(--"human-readable"),
        arg!(--"memory-limit"),
        arg!(--"temp-dir"),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
