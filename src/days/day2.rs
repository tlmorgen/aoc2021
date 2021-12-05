use super::super::day::Day;
use itertools::Itertools;

pub struct Day2 {
    instructions: Vec<Instruction>
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            instructions: Vec::new()
        }
    }
}

enum Direction {
    Forward,
    Up,
    Down
}

struct Instruction {
    direction: Direction,
    quantity: isize
}

impl Day for Day2 {
    fn load(&mut self, content: &str) {
        self.instructions = content.split_whitespace()
            .tuples()
            .map(|(sdir, sqty)| {
                let quantity = sqty.parse().expect("not a num");
                match sdir {
                    "forward" => Instruction {
                        direction: Direction::Forward,
                        quantity: quantity
                    },
                    "up" => Instruction {
                        direction: Direction::Up,
                        quantity: quantity
                    },
                    "down" => Instruction {
                        direction: Direction::Down,
                        quantity: quantity
                    },
                    _ => panic!("unsupported direction {}", sdir)
                }
            })
            .collect();
    }

    fn part1(&mut self) -> isize {
        let (x, y): (isize, isize) = self.instructions.iter().fold((0, 0), |(x, y), instr| {
            match instr.direction {
                Direction::Forward => (x + instr.quantity, y),
                Direction::Up => (x, y + instr.quantity),
                Direction::Down => (x, y - instr.quantity)
            }
        });
        x * -y
    }

    fn part2(&mut self) -> isize {
        let (h, d, _) = self.instructions.iter().fold((0, 0, 0), |(h, d, dfac), instr| {
            match instr.direction {
                Direction::Forward => (h + instr.quantity, d + instr.quantity * dfac, dfac),
                Direction::Up => (h, d, dfac - instr.quantity),
                Direction::Down => (h, d, dfac + instr.quantity)
            }
        });
        h * d
    }
}