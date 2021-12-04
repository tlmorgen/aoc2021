use std::fs;
use clap::{App, Arg};
use std::panic;
use array2d::Array2D;

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 3")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let file = args.value_of("FILE").unwrap();
    let content = fs::read_to_string(file).expect("io");
    let vecs: Vec<Vec<bool>> = content
        .split_whitespace()
        .map(|word| word
            .chars()
            .map(|char| {
                match char {
                    '0' => false,
                    '1' => true,
                    _ => panic!("not binary")
                }})
            .collect())
        .collect();
    let vals = Array2D::from_rows(&vecs);

    part1(&vals);
    part2(&vals);
}

fn counts(vals: &Array2D<bool>) -> Vec<usize> {
    vals.columns_iter()
        .map(|cell_iter| cell_iter.map(|b| *b as usize).sum())
        .collect()
}

fn filter(vals: &Array2D<bool>, popular: bool) -> Result<usize, &'static str> {

    let mut matches = vals.as_rows();
    for pos in 0..vals.row_len() {
        let amatches = Array2D::from_rows(&matches);
        let counts = counts(&amatches);
        let tval = if counts[pos] >= (amatches.column_len() - counts[pos]) {popular} else {!popular};
        matches = amatches
            .as_rows()
            .into_iter()
            .filter(|val| val[pos] == tval)
            .collect();
        if matches.len() < 2 {
            break;
        }
    }

    if matches.len() != 1 {
        Err("no matching word")
    } else {
        let dec = matches[0].iter()
            .fold(0, |dec, &bit| {
                (dec << 1) + bit as usize
            });
        Ok(dec)
    }
}

fn part1(vals: &Array2D<bool>) {

    let counts = counts(vals);

    let gamma: usize = counts.iter()
        .fold(0, |gamma, &count| {
            (gamma << 1) + if count > vals.column_len() / 2 {1} else {0}
        });
    let epsilon: usize = counts.iter()
        .fold(0, |epsilon, &count| {
            (epsilon << 1) + if count > vals.column_len() / 2 {0} else {1}
        });

    println!("{}", gamma * epsilon);
}

fn part2(vals: &Array2D<bool>) {

    let ox = filter(vals, true).expect("unable to find O2");
    let co2 = filter(vals, false).expect("unable to find CO2");

    println!("{}", ox * co2);
}
