mod day;
mod days;

use clap::{App, Arg};
use std::fs;
use day::Day;

fn main() {
    let app = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Advent of Code 2021")
        .arg(Arg::with_name("day")
            .help("which day to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("test")
            .help("test mode (the sample data)")
            .short("t"))
        .get_matches();

    let mut days: Vec<Box<dyn Day>> = vec![
        Box::new(days::day1::Day1::new()),
        Box::new(days::day2::Day2::new()),
        Box::new(days::day3::Day3::new()),
        Box::new(days::day4::Day4::new())
    ];

    let day_num: usize = app.value_of("day").unwrap().parse().expect("day is not a num");
    let day_idx = day_num - 1;
    let test: bool = app.occurrences_of("test") > 0;
    let content_path = format!("./day{:02}/{}.txt", day_num, if test {"test"} else {"input"});
    let content = fs::read_to_string(&content_path).expect("io");

    days[day_idx].load(&content);
    println!("part 1: {}", days[day_idx].part1());
    println!("part 2: {}", days[day_idx].part2());
}
