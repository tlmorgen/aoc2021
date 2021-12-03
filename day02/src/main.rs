use std::fs;
use clap::{App, Arg};

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 2")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();
}
