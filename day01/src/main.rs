use std::fs;
use clap::{App, Arg};

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 1")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let lines = fs::read_to_string(args.value_of("FILE").unwrap()).expect("unable to read file");
    let mut incrs = 0;
    let mut last_depth = -1;
    let depths: Vec<&str> = lines.split("\n").collect();
    for (i, depth) in depths.iter().enumerate() {
        let curr_depth = depth.parse::<isize>().expect("not a num");
        if i > 0 && curr_depth > last_depth {
            incrs += 1;
        }
        last_depth = curr_depth;
    }

    println!("{}", incrs)
}
