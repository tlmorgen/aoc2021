use std::fs;
use clap::{App, Arg};

const WIN_SZ: usize = 3;

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 1")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let mut incrs = 0;
    let mut last_window: isize = -1;
    let depths: Vec<isize> = fs::read_to_string(args.value_of("FILE").unwrap())
        .expect("unable to read file")
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<isize>().expect("not a num"))
        .collect();

    for i in 0..(depths.len() - WIN_SZ + 1) {
        let curr_window = depths[i..i+WIN_SZ].iter().sum();
        if i > 0 && last_window < curr_window {
            incrs += 1;
        }
        last_window = curr_window;
    }

    println!("{}", incrs)
}
