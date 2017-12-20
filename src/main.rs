extern crate clap;

use clap::{App, Arg, AppSettings};
use std::io;

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
                .help("Decode a plain-text message from ascii85.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .help("Encode a plain-text message to ascii85.")
                .takes_value(false)
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Read in message to decode/encode
    let mut msg = String::new();
    if io::stdin().read_line(&mut msg).is_err() {
        println!("Error reading input!");
    }

    // Remove any back slashes from special characters
    let mut skip = false; // dont skip first occurance
    for i in 0..msg.len() {
        if let Some('\\') = msg.chars().nth(i) {
            // Skip every other occurance
            if skip {
                skip = false;
            } else {
                msg.remove(i);
                skip = true;
            }
        }
    }

    // Strip off newline or carriage return characters from the end
    if let Some('\n') = msg.chars().next_back() {
        msg.pop();
    }
    if let Some('\r') = msg.chars().next_back() {
        msg.pop();
    }
    if let Some('\n') = msg.chars().next_back() {
        msg.pop();
    }

    // Decode or encode the message
    let res = if cli_args.is_present("decode") {
        ascii85::ascii85_decode(&msg)
    } else {
        ascii85::ascii85_encode(&msg)
    };

    println!("{}", res);
}
