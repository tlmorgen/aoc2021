use super::super::day::Day;
use std::collections::{HashSet, HashMap};
use bimap::BiHashMap;
use itertools::Itertools;

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
        let peers: BiHashMap<char, char> = BiHashMap::from_iter(vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
        let point_vals: HashMap<char, isize> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

        let mut entry_points: Vec<isize> = Vec::new();
        for entry in &self.braces {
            let mut incomplete = true;
            let mut stack: Vec<char> = Vec::new();
            for c in entry {
                match peers.get_by_left(&c) {
                    None => {
                        // must be an open
                        stack.push(*c);
                    }
                    Some(proper_peer) => {
                        // must be a close
                        match stack.pop() {
                            None => {
                                // too many closes
                                incomplete = false;
                                break;
                            },
                            Some(found_peer) => {
                                if found_peer != *proper_peer { // wrong close
                                    incomplete = false;
                                    break;
                                } // else normal close
                            }
                        }
                    }
                }
            }

            let mut sum = 0isize;
            if incomplete {
                let points = stack.iter().rev()
                    .fold(0isize, |points, open_brace| {
                        (points * 5) + point_vals.get(peers.get_by_right(open_brace).unwrap()).unwrap()
                    });
                entry_points.push(points);
            }
        }
        let entry_points: Vec<isize> = entry_points.into_iter().sorted().collect();
        entry_points[entry_points.len() / 2]
    }
}