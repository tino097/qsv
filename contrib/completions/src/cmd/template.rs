use clap::{arg, Command};

pub fn template_cmd() -> Command {
    Command::new("template").args([
        arg!(--"template"),
        arg!(--"template-file"),
        arg!(--"globals-json"),
        arg!(--"outfilename"),
        arg!(--"outsubdir-size"),
        arg!(--"customfilter-error"),
        arg!(--jobs),
        arg!(--batch),
        arg!(--timeout),
        arg!(--"cache-dir"),
        arg!(--"ckan-api"),
        arg!(--"ckan-token"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
