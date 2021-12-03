use std::fs;
use clap::{App, Arg};
use std::panic;

const WIDTH: usize = 12;

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 2")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let content = fs::read_to_string(args.value_of("FILE").unwrap()).expect("io");
    let words: Vec<&str> = content
        .split_whitespace()
        .collect();
    let mut counts = vec![0; WIDTH]; // little endian
    for word in &words {
        for (i, val) in word.chars().enumerate() {
            counts[i] += match val {
                '0' => 0,
                '1' => 1,
                _ => panic!("not binary")
            }
        }
    }

    let pops: Vec<usize> = counts.iter()
        .map(|count| if count >= &(words.len() / 2) {1} else {0})
        .collect();

    let gamma: usize = pops.iter()
        .fold(0, |gamma, &pop| {
            (gamma << 1) + pop
        });
    let epsilon: usize = pops.iter()
        .fold(0, |epsilon, &pop| {
            (epsilon << 1) + (1 - pop)
        });

    println!("{}", gamma * epsilon);
}
