use super::super::day::Day;
use std::collections::{HashSet, HashMap};

pub struct Day10 {
    braces: Vec<Vec<char>>
}

impl Day10 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day10 {
            braces: content.lines().map(|line| line.chars().collect()).collect()
        }))
    }
}

impl Day for Day10 {
    fn part1(&mut self) -> isize {
        let close: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
        let points: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

        let mut sum = 0isize;
        for entry in &self.braces {
            let mut stack: Vec<char> = Vec::new();
            for c in entry {
                if close.contains_key(&c) {
                    match stack.pop() {
                        None => {break}
                        Some(peer) => {
                            if peer != *close.get(&c).unwrap() {
                                sum += *points.get(&c).unwrap() as isize;
                                break;
                            }
                        }
                    }
                } else {
                    stack.push(*c);
                }
            }
        }
        sum
    }

    fn part2(&mut self) -> isize {
        0
    }
}