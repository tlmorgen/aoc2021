use std::fs;
use clap::{App, Arg};
use std::panic;

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 2")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let file = args.value_of("FILE").unwrap();
    let content = fs::read_to_string(file).expect("io");
    let words: Vec<&str> = content
        .split_whitespace()
        .collect();        
    let bwords: Vec<Vec<usize>> = words.iter()
        .map(|word| word
            .chars()
            .map(|char| {
                match char {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("not binary")
                }})
            .collect())
        .collect(); 
    let width = words[0].len();

    part1(width, &bwords);
    part2(width, &bwords);
}

fn counts(width: usize, bwords: &Vec<Vec<usize>>, idxs: &Vec<usize>) -> Vec<usize> {

    let mut counts = vec![0; width]; // little endian
    for idx in idxs {
        for (i, val) in bwords[*idx].iter().enumerate() {
            counts[i] += val;
        }
    }
    
    counts
}

fn filter(width: usize, bwords: &Vec<Vec<usize>>, popular: bool) -> Result<usize, &'static str> {

    let mut matching_words: Vec<usize> = (0..bwords.len()).collect();
    for pos in 0..width {
        let counts = counts(width, bwords, &matching_words);
        let tval = if counts[pos] >= (matching_words.len() - counts[pos]) {
            if popular {1} else {0}
        } else {
            if popular {0} else {1}
        };
        matching_words = matching_words
            .into_iter()
            .filter(|i| bwords[*i][pos] == tval)
            .collect();
        if matching_words.len() < 2 {
            break;
        }
    }

    if matching_words.len() != 1 {
        Err("no matching word")
    } else {
        Ok(matching_words[0])
    }
}

fn part1(width: usize, bwords: &Vec<Vec<usize>>) {

    let idxs: Vec<usize> = (0..bwords.len()).collect();
    let counts = counts(width, &bwords, &idxs);

    let gamma: usize = counts.iter()
        .fold(0, |gamma, &count| {
            (gamma << 1) + if count > bwords.len() / 2 {1} else {0}
        });
    let epsilon: usize = counts.iter()
        .fold(0, |epsilon, &count| {
            (epsilon << 1) + if count > bwords.len() / 2 {0} else {1}
        });

    println!("{}", gamma * epsilon);
}

fn part2(width: usize, bwords: &Vec<Vec<usize>>) {

    let ox_idx = filter(width, &bwords, true).expect("unable to find O2");
    let co2_idx = filter(width, &bwords, false).expect("unable to find CO2");

    let ox = bwords[ox_idx].iter()
        .fold(0, |ox, bit| {
            (ox << 1) + bit
        });
    let co2 = bwords[co2_idx].iter()
        .fold(0, |co2, bit| {
            (co2 << 1) + bit
        });

    println!("{}", ox * co2);
}
