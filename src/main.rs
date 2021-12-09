#![feature(process_exitcode_placeholder)]
extern crate chrono;
extern crate chrono_tz;

mod day;
mod days;

use clap::{App, Arg};
use std::fs;
use std::process::ExitCode;
use day::DayMaker;
use chrono::{Utc, TimeZone, Datelike};
use chrono_tz::US::Eastern;

const ARG_DAY: &'static str = "day";
const ARG_TEST: &'static str = "test";
const DAY_MAKERS: &'static [DayMaker] = &[
    days::day1::Day1::from_content,
    days::day2::Day2::from_content,
    days::day3::Day3::from_content,
    days::day4::Day4::from_content,
    days::day5::Day5::from_content,
    days::day6::Day6::from_content,
    days::day7::Day7::from_content,
    days::day8::Day8::from_content,
    days::day9::Day9::from_content
];

fn main() -> ExitCode {
    let app = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Advent of Code 2021")
        .arg(Arg::with_name(ARG_DAY)
            .help("which day to run (default: the current day of December ET)")
            .index(1))
        .arg(Arg::with_name(ARG_TEST)
            .help("test mode (the sample data)")
            .short("t"))
        .get_matches();

    let day_num: usize = match app.value_of(ARG_DAY) {
        Some(day) => day.parse().expect("day is not a num"),
        None => Eastern.from_utc_datetime(&Utc::now().naive_utc())
            .day() as usize
    };
    let day_idx = day_num - 1;

    if DAY_MAKERS.len() <= day_idx {
        eprintln!("Day {} is not registered yet.", day_num);
        return ExitCode::FAILURE;
    }

    let test: bool = app.occurrences_of(ARG_TEST) > 0;
    let content_path = format!("./day{:02}/{}.txt", day_num, if test {"test"} else {"input"});
    println!("Loading input from {}", content_path);
    let content = match fs::read_to_string(&content_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Unable to open input file {}: {}", content_path, error);
            return ExitCode::FAILURE;
        }
    };

    match DAY_MAKERS[day_idx](&content) {
        Ok(mut day) => {
            println!("part 1: {}", day.part1());
            println!("part 2: {}", day.part2());
        },
        Err(desc) => {
            eprintln!("Error creating day: {}", desc);
            return ExitCode::FAILURE;
        }
    }

    return ExitCode::SUCCESS;
}
