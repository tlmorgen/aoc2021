use super::super::day::Day;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Day14 {
    base: PolymerHist,
    insertions: HashMap<Pair, char>
}

impl Day14 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut words = content.split_whitespace();
        let base = PolymerHist::from_str(words.next().unwrap());
        let insertions: HashMap<Pair, char> = words.tuples()
            .map(|(pair_s, _, ins_s)| instruction_from_strs(pair_s, ins_s))
            .collect();
        Ok(Box::new(Day14 {
            base,
            insertions
        }))
    }
}

impl Day for Day14 {
    fn part1(&mut self) -> isize {
        let mut base = self.base.clone();
        for _ in 0..10 {
            base = base.mutate(&self.insertions);
        }
        base.min_max_count_diff() as isize
    }

    fn part2(&mut self) -> isize {
        let mut base = self.base.clone();
        for _ in 0..40 {
            base = base.mutate(&self.insertions);
        }
        base.min_max_count_diff() as isize
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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

#[derive(Clone, Debug)]
struct PolymerHist {
    pairs: HashMap<Pair, usize>,
    last_pair: Pair
}

impl PolymerHist {
    fn from_str(word: &str) -> PolymerHist {
        let mut pairs: HashMap<Pair, usize> = HashMap::new();
        let chars: Vec<char> = word.chars().collect();
        let mut last_pair: Option<Pair> = None;
        for i in 0..(chars.len() - 1) {
            *pairs.entry(Pair::from_chars(&chars[i], &chars[i+1])).or_insert(0) += 1;
            if i + 2 == chars.len() {
                last_pair = Some(Pair {
                    left: chars[i],
                    right: chars[i+1]
                })
            }
        }

        PolymerHist {
            pairs,
            last_pair: last_pair.unwrap()
        }
    }

    fn mutate(&self, insertions: &HashMap<Pair, char>) -> PolymerHist {
        let mut new_hist: HashMap<Pair, usize> = HashMap::new();
        let mut new_last_pair = self.last_pair;
        for (pair, count) in self.pairs.iter() {
            match insertions.get(&pair) {
                None => {
                    *new_hist.entry(*pair).or_insert(0) += count;
                }
                Some(insert) => {
                    let left_pair = Pair {
                        left: pair.left,
                        right: *insert
                    };
                    let right_pair = Pair {
                        left: *insert,
                        right: pair.right
                    };
                    *new_hist.entry(left_pair).or_insert(0) += count;
                    *new_hist.entry(right_pair).or_insert(0) += count;

                    if *pair == self.last_pair {
                        new_last_pair = right_pair;
                    }
                }
            }
        }
        PolymerHist {
            pairs: new_hist,
            last_pair: new_last_pair
        }
    }

    fn min_max_count_diff(&self) -> usize {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            *counts.entry(pair.left).or_insert(0) += count;
        }
        *counts.entry(self.last_pair.right).or_insert(0) += 1;
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        max - min
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
