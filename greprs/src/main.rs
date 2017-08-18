extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        writeln!(std::io::stderr(), "Problem parsing arguments: {}", err)
            .expect("Could not write to stderr");
        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {
        writeln!(std::io::stderr(), "Application error: {}", e)
            .expect("Could not write to stderr");

        process::exit(1);
    }
}