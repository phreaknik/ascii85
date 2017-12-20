extern crate clap;
use clap::{App, Arg, AppSettings};

mod ascii85;

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Encode/decode messages to/from ascii85 encoding.")
        .author("John B. <johnboydiv@gmail.com>")
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .value_name("msg")
                .help("Decode a plain-text message from ascii85.")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .value_name("msg")
                .help("Encode a plain-text message to ascii85.")
                .takes_value(true)
                .multiple(true),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Read decode/encode message as vector of words
    let v: Vec<&str> = if cli_args.is_present("decode") {
        cli_args.values_of("decode").unwrap().collect()
    } else {
        cli_args.values_of("encode").unwrap().collect()
    };

    // Parse vector of words into message string
    let mut msg = String::new();
    for word in v {
        msg.push_str(word); // Concatenate word into msg
        msg.push_str(" "); // Add a space before next word
    }
    msg.pop(); // Remove trailing space

    // Decode or encode the message
    let res = if cli_args.is_present("decode") {
        ascii85::ascii85_decode(&msg)
    } else {
        ascii85::ascii85_encode(&msg)
    };

    println!("{}", res);
}
