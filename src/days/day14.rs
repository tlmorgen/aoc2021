use super::super::day::Day;
use std::collections::{HashMap, LinkedList};
use itertools::Itertools;

pub struct Day14 {
    base: LinkedList<char>,
    insertions: HashMap<Pair, char>
}

impl Day14 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut words = content.split_whitespace();
        let base: LinkedList<char> = words.next().unwrap().chars().collect();
        let insertions: HashMap<Pair, char> = words.tuples()
            .map(|(pair_s, _, ins_s)| instruction_from_strs(pair_s, ins_s))
            .collect();
        Ok(Box::new(Day14 {
            base,
            insertions
        }))
    }

    fn single_pass(&mut self) {
        let mut cursor = self.base.cursor_front_mut();
        loop {
            let left = match cursor.current() {
                None => {break;}
                Some(left) => {*left}
            };
            let right = match cursor.peek_next() {
                None => {break;}
                Some(right) => {*right}
            };
            match self.insertions.get(&Pair::from_chars(&left, &right)) {
                None => {}
                Some(insertion) => {
                    cursor.insert_after(*insertion);
                    cursor.move_next();
                }
            }
            cursor.move_next();
        }
    }
}

impl Day for Day14 {
    fn part1(&mut self) -> isize {
        for _ in 0..10 {
            self.single_pass();
        }
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in self.base.iter() {
            *counts.entry(*c).or_insert(0) += 1;
        }
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        (max - min) as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pair {
    left: char,
    right: char
}

impl Pair {
    fn from_chars(left: &char, right: &char) -> Pair {
        Pair {
            left: *left,
            right: *right
        }
    }
}

fn instruction_from_strs(pair: &str, ins: &str) -> (Pair, char) {
    let mut chars = pair.chars().chain(ins.chars());
    (
        Pair {
            left: chars.next().unwrap(),
            right: chars.next().unwrap()
        },
        chars.next().unwrap()
    )
}
