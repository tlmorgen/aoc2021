use std::fmt::{Debug};
use super::super::day::Day;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Pair {
    cypher: Vec<u8>,
    target: Vec<u8>
}

pub struct Day8 {
    entries: Vec<Pair>
}

impl Day8 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day8 {
            entries: content.lines()
                .map(|line| line.splitn(2, '|')
                    .tuples()
                    .map(|(cypher_words, target_words)| Pair {
                        cypher: cypher_words.split_whitespace().map(alpha_to_bits).collect(),
                        target: target_words.split_whitespace().map(alpha_to_bits).collect()
                    })
                    .collect::<Vec<Pair>>()
                )
                .flatten()
                .collect()
        }))
    }
}

impl Day for Day8 {
    fn part1(&mut self) -> isize {
        self.entries.iter()
            .map(|entry| &entry.target)
            .flatten()
            .fold(0isize, |count, word| {
                count + match sumbits(*word) {
                    2 => 1,
                    3 => 1,
                    4 => 1,
                    7 => 1,
                    _ => 0
                }
            })
    }

    fn part2(&mut self) -> isize {
        let mut sum = 0isize;
        for entry in self.entries.iter() {
            let mut cypher_map: HashMap<u8, u8> = HashMap::new();
            let mut crypt_map: HashMap<u8, u8> = HashMap::new();
            let mut set069: Vec<u8> = Vec::new();
            let mut set235: Vec<u8> = Vec::new();
            for word in entry.cypher.iter() {
                match sumbits(*word) {
                    2 => {
                        cypher_map.insert(*word, 1);
                        crypt_map.insert(1, *word);
                    },
                    3 => {
                        cypher_map.insert(*word, 7);
                        crypt_map.insert(7, *word);
                    },
                    4 => {
                        cypher_map.insert(*word, 4);
                        crypt_map.insert(4, *word);
                    },
                    5 => set235.push(*word),
                    6 => set069.push(*word),
                    7 => {
                        cypher_map.insert(*word, 8);
                        crypt_map.insert(8, *word);
                    },
                    _ => {}
                }
            }

            let a_bit = crypt_map.get(&7).unwrap() ^ crypt_map.get(&1).unwrap();
            let bits_4a = crypt_map.get(&4).unwrap() | a_bit;
            set069 = set069.into_iter().filter(|word| {
                    if sumbits(bits_4a ^ *word) == 1 {
                        crypt_map.insert(9, *word);
                        cypher_map.insert(*word, 9);
                        false
                    } else {true}
                })
                .collect();
            let bits_1x4 = crypt_map.get(&1).unwrap() ^ crypt_map.get(&4).unwrap();
            set069 = set069.into_iter().filter(|word| {
                    if sumbits(word & bits_1x4) == 1 {
                        crypt_map.insert(0, *word);
                        cypher_map.insert(*word, 0);
                        false
                    } else {true}
                }).collect();
            if set069.len() == 1 {
                crypt_map.insert(6, *set069.get(0).unwrap());
                cypher_map.insert(*set069.get(0).unwrap(), 6);
            } else {
                panic!("algo not working");
            }
            let bits_6 = *crypt_map.get(&6).unwrap();
            set235 = set235.into_iter().filter(|word| {
                if sumbits(bits_6 ^ *word) == 1 {
                    crypt_map.insert(5, *word);
                    cypher_map.insert(*word, 5);
                    false
                } else {true}
            }).collect();
            let bits_9 = *crypt_map.get(&9).unwrap();
            set235 = set235.into_iter().filter(|word| {
                if sumbits(bits_9 ^ *word) == 1 {
                    crypt_map.insert(3, *word);
                    cypher_map.insert(*word, 3);
                    false
                } else {true}
            }).collect();
            if set235.len() == 1 {
                crypt_map.insert(2, *set235.get(0).unwrap());
                cypher_map.insert(*set235.get(0).unwrap(), 2);
            } else {
                panic!("algo not working");
            }
            if !crypt_map.len() == 10 {
                panic!("not enough found: {}", crypt_map.len());
            }

            let mut plain_num = 0isize;
            for word in entry.target.iter() {
                let plain_digit = cypher_map.get(word).unwrap();
                plain_num = (plain_num * 10) + *plain_digit as isize;
            }
            sum += plain_num;
        }

        sum
    }
}

fn sumbits(mut word: u8) -> usize {
    let mut sum = 0usize;
    for _ in 0..8 {
        sum += (word & 1) as usize;
        word = word >> 1;
    }
    sum
}

fn alpha_to_bits(word: &str) -> u8 {
    word.chars().fold(0u8, |bits, c| bits | match c {
            'a' => 1 << 0,
            'b' => 1 << 1,
            'c' => 1 << 2,
            'd' => 1 << 3,
            'e' => 1 << 4,
            'f' => 1 << 5,
            'g' => 1 << 6,
            _ => panic!("unsupported character {}", c)
        })
}