use super::super::day::Day;
use itertools::Itertools;

pub struct Day2 {
    instructions: Vec<Instruction>
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

impl Day2 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day2 {
            instructions: content.split_whitespace()
                .tuples()
                .map(|(sdir, sqty)| {
                    let quantity = sqty.parse().expect("not a num");
                    match sdir {
                        "forward" => Instruction {
                            direction: Direction::Forward,
                            quantity
                        },
                        "up" => Instruction {
                            direction: Direction::Up,
                            quantity
                        },
                        "down" => Instruction {
                            direction: Direction::Down,
                            quantity
                        },
                        _ => panic!("unsupported direction {}", sdir)
                    }
                })
                .collect()
        }))
    }
}

impl Day for Day2 {

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