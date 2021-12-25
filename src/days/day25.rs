use std::collections::HashSet;
use array2d::Array2D;
use super::super::day::Day;

type Loc = [usize; 2];

pub struct Day25 {
    herds: [HashSet<Loc>; 2],
    width: usize,
    height: usize
}

impl Day25 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut east: HashSet<Loc> = HashSet::new();
        let mut south: HashSet<Loc> = HashSet::new();
        for (i, line) in content.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '>' => east.insert([i, j]),
                    'v' => south.insert([i, j]),
                    _ => false
                };
            }
        }
        Ok(Box::new(Day25 {
            herds: [east, south],
            width: content.lines().next().unwrap().len(),
            height: content.lines().count()
        }))
    }

    fn move_herd(&mut self, herd: usize) -> usize
    {
        let (max, incr) = if herd == 0 {
            (self.width, 1)
        } else {
            (self.height, 0)
        };
        let moves: Vec<(Loc, Loc)> = self.herds[herd].iter()
            .map(|curr| {
                let mut next = curr.clone();
                next[incr] = (next[incr] + 1) % max;
                (*curr, next)
            })
            .filter(|(curr, next)| {
                !self.herds.iter().any(|herd| herd.contains(next))
            })
            .collect();
        let moves_len = moves.len();
        for (before, after) in moves {
            self.herds[herd].remove(&before);
            self.herds[herd].insert(after);
        }
        moves_len
    }
}

impl Day for Day25 {
    fn part1(&mut self) -> isize {
        let mut steps = 0;
        loop {
            let mut moves = 0;
            moves += self.move_herd(0);
            moves += self.move_herd(1);

            steps += 1;
            if moves < 1 {
                break;
            }
        }
        steps as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}