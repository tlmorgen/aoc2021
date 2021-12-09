use super::super::day::Day;
use std::collections::HashSet;
use itertools::Itertools;

pub struct Day9 {
    depths: Grid
}

impl Day9 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let rows: Vec<Vec<usize>> = content.lines()
            .map(|line| line.trim().chars()
                .map(|c| c as usize - 48)
                .collect())
            .collect();
        let min_width = rows.iter().map(|row| row.len()).min().unwrap();
        let max_width = rows.iter().map(|row| row.len()).max().unwrap();
        assert_eq!(min_width, max_width);
        Ok(Box::new(Day9 {
            depths: Grid {
                rows
            }
        }))
    }
}

impl Day for Day9 {
    fn part1(&mut self) -> isize {
        self.depths.all_pos().iter()
            .filter(|target| self.depths.is_min(*target))
            .flat_map(|target| self.depths.get(target))
            .map(|value| value + 1)
            .sum::<usize>() as isize
    }

    fn part2(&mut self) -> isize {
        self.depths.basins().into_iter()
            .map(|basin| basin.len())
            .sorted()
            .rev()
            .tuples()
            .map(|(a, b, c)| a * b * c)
            .next()
            .unwrap() as isize
    }
}

#[derive(Hash, Debug, Copy,  Clone, Eq, PartialEq)]
struct Pos {
    row: isize,
    col: isize
}

impl Pos {
    fn up(&self) -> Pos {
        Pos {
            row: self.row - 1,
            col: self.col
        }
    }

    fn right(&self) -> Pos {
        Pos {
            row: self.row,
            col: self.col + 1
        }
    }

    fn down(&self) -> Pos {
        Pos {
            row: self.row + 1,
            col: self.col
        }
    }

    fn left(&self) -> Pos {
        Pos {
            row: self.row,
            col: self.col - 1
        }
    }
}

struct Grid {
    rows: Vec<Vec<usize>>
}

impl Grid {

    fn all_pos(&self) -> Vec<Pos> {
        (0..self.rows.len()).into_iter()
            .map(|row| {
                (0..self.rows[0].len()).into_iter()
                    .map(|col| Pos {
                        row: row as isize,
                        col: col as isize
                    })
                    .collect::<Vec<Pos>>()
            })
            .flatten()
            .collect()
    }

    fn get(&self, target: &Pos) -> Option<usize> {
        if target.row < 0 || target.col < 0
            || target.row + 1 > self.rows.len() as isize
            || target.col + 1 > self.rows[0].len() as isize {
            None
        } else {
            Some(self.rows[target.row as usize][target.col as usize])
        }
    }

    fn neighbors(&self, target: &Pos) -> Vec<Pos> {
        [target.up(), target.right(), target.down(), target.left()].into_iter()
            .collect()
    }

    fn neighbors_get(&self, target: &Pos) -> Vec<usize> {
        self.neighbors(target).iter()
            .flat_map(|target| self.get(target))
            .collect()
    }

    fn is_min(&self, target: &Pos) -> bool {
        let value = self.get(target).unwrap();
        value < *self.neighbors_get(target).iter().min().unwrap()
    }

    fn basins(&self) -> Vec<Vec<Pos>> {
        let mut basins: Vec<HashSet<Pos>> = Vec::new();
        let mut visits: HashSet<Pos> = HashSet::new();

        for pos in self.all_pos() {
            if visits.contains(&pos) {
                continue;
            } else if self.get(&pos).unwrap() > 8 {
                visits.insert(pos);
                continue;
            }

            // new basin

            let mut basin: HashSet<Pos> = HashSet::new();
            let mut stack: Vec<Pos> = Vec::new();
            stack.push(pos);

            eprintln!("new basin: starting from {:?}", pos);

            while stack.len() > 0 {
                let target = stack.pop().unwrap();
                if visits.contains(&target) {
                    continue;
                } else {
                    visits.insert(target);
                }

                eprintln!("basin: adding {:?}", target);
                basin.insert(target);
                self.neighbors(&target).into_iter()
                    .filter(|pos| {
                        match self.get(pos) {
                            None => false,
                            Some(value) => value < 9
                        }
                    })
                    .for_each(|low_neighbor| stack.push(low_neighbor));
            }

            basins.push(basin);
        }

        basins.into_iter()
            .map(|basin| basin.into_iter().collect())
            .collect()
    }
}

