use super::super::day::Day;
use itertools::Itertools;
use bimap::BiHashMap;

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
                count + match sum_bits(*word) {
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

            let mut plain_to_cypher: BiHashMap<u8, u8> = BiHashMap::new();
            let mut set069: Vec<u8> = Vec::new();
            let mut set235: Vec<u8> = Vec::new();
            for cypher_digit in entry.cypher.iter() {
                match sum_bits(*cypher_digit) {
                    2 => {
                        plain_to_cypher.insert(1, *cypher_digit);
                    },
                    3 => {
                        plain_to_cypher.insert(7, *cypher_digit);
                    },
                    4 => {
                        plain_to_cypher.insert(4, *cypher_digit);
                    },
                    5 => set235.push(*cypher_digit),
                    6 => set069.push(*cypher_digit),
                    7 => {
                        plain_to_cypher.insert(8, *cypher_digit);
                    },
                    _ => panic!("unexpected digit bits {}", sum_bits(*cypher_digit))
                }
            }
            let cypher_1 = *plain_to_cypher.get_by_left(&1).unwrap();
            let cypher_4 = *plain_to_cypher.get_by_left(&4).unwrap();
            let cypher_7 = *plain_to_cypher.get_by_left(&7).unwrap();

            // 7 ^ 1 => a
            let cypher_bit_a = cypher_7 ^ cypher_1;

            // (4 | a) ^ [069] == 1 => 9,g
            let bits_4a = cypher_4 | cypher_bit_a;
            set069 = set069.into_iter().filter(|cypher_digit| {
                if sum_bits(bits_4a ^ *cypher_digit) == 1 {
                    plain_to_cypher.insert(9, *cypher_digit);
                    false
                } else {true}
            }).collect();

            // (1 ^ 4) & [06] == 1 => 0,d
            let bits_1x4 = cypher_1 ^ cypher_4;
            set069 = set069.into_iter().filter(|cypher_digit| {
                    if sum_bits(cypher_digit & bits_1x4) == 1 {
                        plain_to_cypher.insert(0, *cypher_digit);
                        false
                    } else {true}
                }).collect();

            // [6]
            if set069.len() == 1 {
                plain_to_cypher.insert(6, *set069.get(0).unwrap());
            } else {
                panic!("algo not working: [069]");
            }

            let cypher_6 = *plain_to_cypher.get_by_left(&6).unwrap();
            // 6 ^ [235] == 1 => 5,e
            set235 = set235.into_iter().filter(|cypher_digit| {
                if sum_bits(cypher_6 ^ *cypher_digit) == 1 {
                    plain_to_cypher.insert(5, *cypher_digit);
                    false
                } else {true}
            }).collect();

            let cypher_9 = *plain_to_cypher.get_by_left(&9).unwrap();
            // 9 ^ [23] == 1 => 5,e
            set235 = set235.into_iter().filter(|cypher_digit| {
                if sum_bits(cypher_9 ^ *cypher_digit) == 1 {
                    plain_to_cypher.insert(3, *cypher_digit);
                    false
                } else {true}
            }).collect();

            // [2]
            if set235.len() == 1 {
                plain_to_cypher.insert(2, *set235.get(0).unwrap());
            } else {
                panic!("algo not working: [235]");
            }

            // check
            if !plain_to_cypher.len() == 10 {
                panic!("not enough keys found: {}", plain_to_cypher.len());
            }

            // decrypt
            let mut plain_num = 0isize;
            for cypher_digit in entry.target.iter() {
                let plain_digit = plain_to_cypher.get_by_right(cypher_digit).unwrap();
                plain_num = (plain_num * 10) + *plain_digit as isize;
            }
            sum += plain_num;
        }

        sum
    }
}

fn sum_bits(mut word: u8) -> usize {
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