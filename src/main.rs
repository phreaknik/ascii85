extern crate clap;
use clap::{App, AppSettings};

mod ascii85;

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Encode/decode messages to/from ascii85 encoding.")
        .author("John B. <johnboydiv@gmail.com>")
        .setting(AppSettings::TrailingVarArg)
        .arg_from_usage(
            "-d, --decode=[msg] 'Decode a plain-text message from ascii85.'",
        )
        .arg_from_usage(
            "-e, --encode=[msg] 'Encode a plain-text message to ascii85.'",
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let msg: Vec<&str> = cli_args
        .values_of("decode")
        .unwrap_or(cli_args.values_of("encode").unwrap())
        .collect();

    println!("{:?}", msg);
}
