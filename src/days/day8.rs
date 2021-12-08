use std::fmt::{Debug, Formatter};
use super::super::day::Day;
use itertools::Itertools;

const SEGMENTS: usize = 7;

struct Digit {
    states: [bool; SEGMENTS]
}

#[derive(Debug)]
struct Pair {
    cypher: Vec<Digit>,
    target: Vec<Digit>
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
                        cypher: cypher_words.split_whitespace().map(Digit::from_string).collect(),
                        target: target_words.split_whitespace().map(Digit::from_string).collect()
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
            .fold(0 as usize, |sum, pair| {
                sum + pair.target.iter()
                    .fold(0, |sum, digit| {
                        sum + match digit.states.iter().map(|b| *b as usize).sum() {
                            2 => 1,
                            3 => 1,
                            4 => 1,
                            7 => 1,
                            _ => 0
                        }
                    })
            }) as isize
    }

    fn part2(&mut self) -> isize {

        0
    }
}

impl Debug for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let states: Vec<Vec<usize>> = vec![
            vec![1, 2, 3, 4],
            vec![7, 14],
            vec![12, 19],
            vec![22, 23, 24, 25],
            vec![28, 35],
            vec![33, 40],
            vec![43, 44, 45, 46]
        ];
        let mut grid = [' '; 7 * 7];
        for segment in 0..SEGMENTS {
            if self.states[segment] {
                for node in states[segment].iter() {
                    grid[*node] = (('a' as usize) + segment) as u8 as char;
                }
            }
        }
        for node in (6..42).step_by(7) {
            grid[node] = '\n';
        }

        let fmt_string: String = grid.iter().collect();

        f.write_fmt(format_args!("\n{}\n", fmt_string))
    }
}

impl Digit {
    pub fn from_string(s: &str) -> Digit {
        Digit {
            states: s.chars()
                .fold([false; SEGMENTS], |mut states, c| {
                    match c {
                        'a' => states[0] = true,
                        'b' => states[1] = true,
                        'c' => states[2] = true,
                        'd' => states[3] = true,
                        'e' => states[4] = true,
                        'f' => states[5] = true,
                        'g' => states[6] = true,
                        _ => panic!("unsupported character: {}", c)
                    }
                    states
                })
        }
    }
}