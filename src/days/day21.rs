use std::cmp::max;
use std::collections::HashMap;

use itertools::Itertools;

use super::super::day::Day;

const DIE_ROLLS: usize = 3;

pub struct Day21 {
    pos: Vec<usize>,
}

impl Day21 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day21 {
            pos: content.split([':', '\n'])
                .tuples()
                .filter_map(|(_, n)| n.trim().parse().ok())
                .map(|p: usize| p - 1) // make mod nice
                .collect()
        }))
    }
}

impl Day for Day21 {
    fn part1(&mut self) -> isize {
        let end_score = 1000usize;
        let mut die = DetDie::new(100);
        let mut pos = self.pos.clone();
        let mut scores = vec![0usize; pos.len()];
        'outer: loop {
            for player in 0..pos.len() {
                pos[player] = (pos[player] + die.roll_sum(DIE_ROLLS)) % 10;
                let score = scores[player] + pos[player] + 1;
                if score >= end_score {
                    break 'outer;
                } else {
                    scores[player] = score;
                }
            }
        }
        (scores.iter().min().unwrap() * die.rolls()) as isize
    }

    fn part2(&mut self) -> isize {
        let wins = dirac_wins((self.pos[0], 0, self.pos[1], 0, true), &mut HashMap::new());
        max(wins.0, wins.1) as isize
    }
}

type DiracState = (usize, usize, usize, usize, bool);
type WinCounts = (usize, usize);

const THREE_SUM: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
const END_SCORE: usize = 21;
const BOARD_SIZE: usize = 10;

fn dirac_wins(state: DiracState, memo: &mut HashMap<DiracState, WinCounts>) -> WinCounts {
    if memo.contains_key(&state) {
        memo[&state]
    } else if state.1 >= END_SCORE {
        (1, 0)
    } else if state.3 >= END_SCORE {
        (0, 1)
    } else {
        let wins = if state.4 { // player 1's turn
            THREE_SUM.iter().fold((0, 0), |wins, (sum, count)| {
                let new_pos = (state.0 + sum) % BOARD_SIZE;
                let sub_wins = dirac_wins((
                    new_pos,
                    state.1 + new_pos + 1,
                    state.2,
                    state.3,
                    !state.4
                ), memo);
                (wins.0 + (sub_wins.0 * count), wins.1 + (sub_wins.1 * count))
            })
        } else { // player 2's turn
            THREE_SUM.iter().fold((0, 0), |wins, (sum, count)| {
                let new_pos = (state.2 + sum) % BOARD_SIZE;
                let sub_wins = dirac_wins((
                    state.0,
                    state.1,
                    new_pos,
                    state.3 + new_pos + 1,
                    !state.4
                ), memo);
                (wins.0 + (sub_wins.0 * count), wins.1 + (sub_wins.1 * count))
            })
        };
        memo.insert(state, wins);
        wins
    }
}

struct DetDie {
    sides: usize,
    face: usize,
    rolls: usize,
}

impl DetDie {
    fn new(sides: usize) -> Self {
        DetDie {
            sides,
            face: 0,
            rolls: 0,
        }
    }
    fn roll(&mut self) -> usize {
        (
            self.face + 1,
            self.face = (self.face + 1) % self.sides,
            self.rolls += 1
        ).0
    }
    fn roll_sum(&mut self, count: usize) -> usize {
        (0..count).fold(0usize, |sum, _| sum + self.roll())
    }
    fn rolls(&self) -> usize {
        self.rolls
    }
}

