#![feature(process_exitcode_placeholder)]
#![feature(linked_list_cursors)]
#![feature(map_first_last)]
#![feature(int_log)]
extern crate chrono;
extern crate chrono_tz;

mod day;
mod days;

use clap::{App, Arg};
use std::fs;
use std::process::ExitCode;
use std::time::Instant;
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
    days::day9::Day9::from_content,
    days::day10::Day10::from_content,
    days::day11::Day11::from_content,
    days::day12::Day12::from_content,
    days::day13::Day13::from_content,
    days::day14::Day14::from_content,
    days::day15::Day15::from_content,
    days::day16::Day16::from_content,
    days::day17::Day17::from_content,
    days::day18::Day18::from_content,
    days::day19::Day19::from_content,
    days::day20::Day20::from_content,
    days::day21::Day21::from_content,
    days::day22::Day22::from_content,
    days::day23::Day23::from_content,
    days::day24::Day24::from_content,
    days::day25::Day25::from_content
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

    let start = Instant::now();
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
    println!("runtime: {:?}", start.elapsed());

    return ExitCode::SUCCESS;
}
