use std::cmp::max;
use std::ops::RangeInclusive;
use itertools::Itertools;
use super::super::day::Day;

pub struct Day22 {
    instructions: Vec<Switch>
}

impl Day22 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day22 {
            instructions: content.lines().map(|line| {
                let parts: Vec<&str> = line.split([' ', ',', '=', '.']).collect();
                Switch {
                    state: parts[0].into(),
                    x_range: RangeInclusive::new(parts[2].parse().unwrap(), parts[4].parse().unwrap()),
                    y_range: RangeInclusive::new(parts[6].parse().unwrap(), parts[8].parse().unwrap()),
                    z_range: RangeInclusive::new(parts[10].parse().unwrap(), parts[12].parse().unwrap())
                }
            }).collect()
        }))
    }
}

impl Day for Day22 {
    fn part1(&mut self) -> isize {
        let frame: usize = 101;
        let mut reactor: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; frame]; frame]; frame];
        for switch in &self.instructions {
            if max(*switch.x_range.start(), *switch.x_range.end()).abs() > 50
                || max(*switch.y_range.start(), *switch.y_range.end()).abs() > 50
                || max(*switch.z_range.start(), *switch.z_range.end()).abs() > 50 {
                continue
            }
            let b = match switch.state {
                State::On => true,
                State::Off => false
            };
            for mut x in switch.x_range.clone() {
                x += (frame as isize) / 2;
                for mut y in switch.y_range.clone() {
                    y += (frame as isize) / 2;
                    for mut z in switch.z_range.clone() {
                        z += (frame as isize) / 2;
                        reactor[x as usize][y as usize][z as usize] = b;
                    }
                }
            }
        }
        reactor.iter().flatten().flatten().map(|b| *b as usize).sum::<usize>() as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

#[derive(Debug)]
enum State {
    On,
    Off
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        match s {
            "on" => State::On,
            "off" => State::Off,
            _ => panic!("invalid state: {}", s)
        }
    }
}

#[derive(Debug)]
struct Switch {
    state: State,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>
}