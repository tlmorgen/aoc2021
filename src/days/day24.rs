use super::super::day::Day;
use rayon::prelude::*;

pub struct Day24 {
    ops: Vec<Op>
}

impl Day24 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day24 {
            ops: content.lines().map(Op::from_str).collect()
        }))
    }
}

impl Day for Day24 {
    fn part1(&mut self) -> isize {
        // analysis and algebra done by "hand"
        ALU::new().verify(&self.ops, "91897399498995".chars().collect()) as isize
    }

    fn part2(&mut self) -> isize {
        // analysis and algebra done by "hand"
        ALU::new().verify(&self.ops, "51121176121391".chars().collect()) as isize
    }
}

#[derive(Debug, Clone)]
struct Register {
    id: char
}

impl Register {
    fn from_str(word: &str) -> Self {
        Self {
            id: word.chars().next().unwrap()
        }
    }
    fn get_index(&self) -> usize {
        (self.id as usize) - ('w' as usize)
    }
}

#[derive(Debug, Clone)]
enum RValue {
    Register(Register),
    Literal(isize)
}

impl RValue {
    fn from_str(word: &str) -> Self {
        match word.parse::<isize>() {
            Ok(literal) => {
                RValue::Literal(literal)
            }
            Err(_) => {
                RValue::Register(Register::from_str(word))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Input(Register),
    Add(Register, RValue),
    Multiply(Register, RValue),
    Divide(Register, RValue),
    Mod(Register, RValue),
    Equal(Register, RValue)
}

impl Op {
    fn from_str(words: &str) -> Self {
        let parts: Vec<&str> = words.split(' ').collect();
        let reg = Register::from_str(parts[1]);
        match parts[0] {
            "inp" => Op::Input(reg),
            "add" => Op::Add(reg, RValue::from_str(parts[2])),
            "mul" => Op::Multiply(reg, RValue::from_str(parts[2])),
            "div" => Op::Divide(reg, RValue::from_str(parts[2])),
            "mod" => Op::Mod(reg, RValue::from_str(parts[2])),
            "eql" => Op::Equal(reg, RValue::from_str(parts[2])),
            _ => panic!("no such op {}", parts[0])
        }
    }
}

const REG_SZ: usize = 4;

struct ALU {
    registers: [isize; REG_SZ]
}

impl ALU {
    fn new() -> Self {
        Self {
            registers: [0; REG_SZ]
        }
    }

    fn verify(&mut self, ops: &Vec<Op>, input: Vec<char>) -> bool {
        if input.len() == 14 && input.iter().all(|c| *c != '0') {
            let mut iter = input.iter().map(|c| (*c as usize) - ('0' as usize));
            for op in ops {
                if !self.execute_op(op, &mut iter) {
                    return false;
                }
            }
            return self.registers[REG_SZ - 1] == 0
        } else {
            false
        }
    }

    fn execute_op<I>(&mut self, op: &Op, input: &mut I) -> bool
        where I: Iterator<Item = usize>
    {
        match op {
            Op::Input(reg) => {
                self.registers[reg.get_index()] = input.next().unwrap() as isize;
                true
            }
            Op::Add(reg, val) => {
                self.registers[reg.get_index()] += self.resolve(val);
                true
            }
            Op::Multiply(reg, val) => {
                self.registers[reg.get_index()] *= self.resolve(val);
                true
            }
            Op::Divide(reg, val) => {
                let lit = self.resolve(val);
                if lit != 0 {
                    self.registers[reg.get_index()] /= lit;
                    true
                } else {
                    eprintln!("attempt invalid div {:?}: {} /= {} - {:?}", op, self.registers[reg.get_index()], lit, self.registers);
                    false
                }
            }
            Op::Mod(reg, val) => {
                let lit = self.resolve(val);
                if lit > 0 && self.registers[reg.get_index()] >= 0 {
                    self.registers[reg.get_index()] %= lit;
                    true
                } else {
                    eprintln!("attempt invalid mod {:?}: {} %= {} - {:?}", op, self.registers[reg.get_index()], lit, self.registers);
                    false
                }
            }
            Op::Equal(reg, val) => {
                self.registers[reg.get_index()] = (self.registers[reg.get_index()] == self.resolve(val)) as isize;
                true
            }
        }
    }

    fn resolve(&self, r_val: &RValue) -> isize {
        match &r_val {
            RValue::Register(register) => self.registers[register.get_index()],
            RValue::Literal(literal) => *literal
        }
    }
}