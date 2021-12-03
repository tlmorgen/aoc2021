use std::fs;
use clap::{App, Arg};
use std::panic;
use itertools::Itertools;

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 2")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let (x, y): (isize, isize) = fs::read_to_string(args.value_of("FILE").unwrap())
        .expect("unable to read file")
        .split_whitespace()
        .tuples()
        .fold((0, 0), |(x, y), (dir, sqty)| {
            let qty = sqty.parse::<isize>().unwrap_or_else(|_| panic!("not a num: {}", sqty));
            match dir {
                "forward" => (x + qty, y),
                "up" => (x, y + qty),
                "down" => (x, y - qty),
                _ => panic!("unsupported direction: {}", dir)
            }
        });
        
    println!("{}", x * -y);
}
