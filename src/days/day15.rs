use super::super::day::Day;
use std::cmp::min;

pub struct Day15 {
    risks: Vec<Vec<usize>>
}

impl Day15 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day15 {
            risks: content.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c as usize - '0' as usize)
                        .collect()
                })
                .collect()
        }))
    }
}

impl Day for Day15 {
    fn part1(&mut self) -> isize {
        lowest_risk(self.risks.clone()) as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

fn lowest_risk(mut risks: Vec<Vec<usize>>) -> usize {
    risks[0][0] = 0; // you do not "enter" the first spot
    for i in 0..risks.len() {
        for j in 0..risks[0].len() {
            if i > 0 && j > 0 {
                risks[i][j] += min(risks[i][j-1], risks[i-1][j]);
            } else if i > 0 {
                risks[i][j] += risks[i-1][j];
            } else if j > 0 {
                risks[i][j] += risks[i][j-1];
            }
        }
    }
    risks[risks.len() - 1][risks[0].len() - 1]
}