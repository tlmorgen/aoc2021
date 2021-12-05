extern crate chrono;
extern crate chrono_tz;

mod day;
mod days;

use clap::{App, Arg};
use std::fs;
use day::Day;
use chrono::{Utc, TimeZone, Datelike};
use chrono_tz::US::Eastern;

const DAY_MAKERS: &'static [fn(&str) -> Box<dyn  Day>] = &[
    days::day1::Day1::from_content,
    days::day2::Day2::from_content,
    days::day3::Day3::from_content,
    days::day4::Day4::from_content
];

fn main() {
    let app = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Advent of Code 2021")
        .arg(Arg::with_name("day")
            .help("which day to run (default: the current day of December ET)")
            .index(1))
        .arg(Arg::with_name("test")
            .help("test mode (the sample data)")
            .short("t"))
        .get_matches();

    let day_num: usize = match app.value_of("day") {
        Some(day) => day.parse().expect("day is not a num"),
        None => Eastern.from_utc_datetime(&Utc::now().naive_utc())
            .day() as usize
    };
    let day_idx = day_num - 1;
    let test: bool = app.occurrences_of("test") > 0;
    let content_path = format!("./day{:02}/{}.txt", day_num, if test {"test"} else {"input"});
    let content = fs::read_to_string(&content_path).expect("io");

    let mut day_impl = DAY_MAKERS[day_idx](&content);
    println!("part 1: {}", day_impl.part1());
    println!("part 2: {}", day_impl.part2());
}
