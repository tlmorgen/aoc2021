use itertools::Itertools;
use super::super::day::Day;

const BOARD_LEN: usize = 10;
const DIE_ROLLS: usize = 3;

pub struct Day21 {
    pos: Vec<usize>
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
                pos[player] = (pos[player] + die.roll_sum(DIE_ROLLS)) % BOARD_LEN;
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
        0
    }
}

struct DetDie {
    sides: usize,
    face: usize,
    rolls: usize
}

impl DetDie {
    fn new(sides: usize) -> Self {
        DetDie {
            sides,
            face: 0,
            rolls: 0
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