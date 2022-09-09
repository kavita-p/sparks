// Cut this line when debugging dead code.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::{env, process};

use sparksrs::Command;

mod interpreter;

fn main() {
    // Sparks!
    let command = Command::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(0_0);
    });

    if let Err(e) = sparksrs::run(command) {
        eprintln!("App error: {e}");

        process::exit(1);
    }
}
