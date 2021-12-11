use super::super::day::Day;

pub struct Day11 {
    octopuses: OctopusGrid
}

impl Day11 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day11 {
            octopuses: OctopusGrid::from_string(content)
        }))
    }
}

impl Day for Day11 {
    fn part1(&mut self) -> isize {
        self.octopuses.count_flashes(100)
    }

    fn part2(&mut self) -> isize {
        0
    }
}

#[derive(Debug)]
struct Pos {
    row: isize,
    col: isize
}

impl Pos {
    fn from_idxs(row: isize, col: isize) -> Pos {
        Pos {row, col}
    }
    fn delta(&self, row_d: isize, col_d: isize) -> Pos {
        Pos {
            row: self.row + row_d,
            col: self.col + col_d
        }
    }
    fn row(&self) -> usize {
        self.row as usize
    }
    fn col(&self) -> usize {
        self.col as usize
    }
}

struct OctopusGrid {
    energies: Vec<Vec<usize>>
}

impl OctopusGrid {
    fn from_string(content: &str) -> OctopusGrid {
        OctopusGrid {
            energies: content.lines()
                .map(|line| line.chars()
                    .map(|c| c as usize - '0' as usize)
                    .collect())
                .collect()
        }
    }

    fn all_pos(&self) -> Vec<Pos> {
        (0isize..self.energies.len() as isize).into_iter().map(|row| {
            (0isize..self.energies[0].len() as isize).into_iter().map(|col| {
                Pos::from_idxs(row, col)
            }).collect::<Vec<Pos>>()
        }).flatten().collect()
    }

    fn valid_pos(&self, pos: &Pos) -> bool {
        pos.row < (self.energies.len() as isize)
            && pos.col < (self.energies[0].len() as isize)
            && pos.row > -1
            && pos.col > -1
    }

    fn incr_all(&mut self) {
        self.all_pos().into_iter().for_each(|pos| self.incr(&pos));
    }

    fn incr(&mut self, pos: &Pos) {
        self.energies[pos.row()][pos.col()] += 1;
        if self.energies[pos.row()][pos.col()] == 10 {
            self.adjacent(pos).into_iter()
                .for_each(|neighbor| self.incr(&neighbor));
        }
    }

    fn prune_all(&mut self) -> isize {
        self.all_pos().into_iter().fold(0isize, |flashes, pos| {
            flashes + self.prune(&pos)
        })
    }

    fn prune(&mut self, pos: &Pos) -> isize {
        if self.energies[pos.row()][pos.col()] > 9 {
            self.energies[pos.row()][pos.col()] = 0;
            1
        } else {
            0
        }
    }

    fn adjacent(&self, target: &Pos) -> Vec<Pos> {
        [
            target.delta(-1, 0), // up
            target.delta(-1, 1), // upright
            target.delta(0, 1), // right
            target.delta(1, 1), // downright
            target.delta(1, 0), // down
            target.delta(1, -1), // downleft
            target.delta(0, -1), // left
            target.delta(-1, -1) // upleft
        ].into_iter()
            .filter(|pos| self.valid_pos(pos))
            .collect()
    }

    fn count_flashes(&mut self, cycles: usize) -> isize {
        (0..cycles).into_iter().fold(0isize, |flashes, _cycle| {
            self.incr_all();
            flashes + self.prune_all()
        })
    }
}