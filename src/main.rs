#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate clap;

use clap::{App, Arg};

mod direction;
mod instruction;
mod interpreter;
mod program;
mod rule;
mod stack;
mod state;

fn main() {
    let matches = App::new("Befreak Interpreter")
        .version(crate_version!())
        .arg(
            Arg::with_name("FILE")
                .help("The program file to interpret")
                .required(true),
        )
        .get_matches();

    let file = matches.value_of("FILE").expect("FILE is required");
    let mut file = File::open(file).unwrap(); // TODO

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap(); // TODO

    let program = program::Program::from_str(&file_content).unwrap(); // TODO

    interpreter::run(program);
}
