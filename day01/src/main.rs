use clap::{App, Arg};
use itertools::Itertools;
use std::fs;
use std::io;

const MAX_INT: u16 = 1 << 15;

struct Executable {
    memory: Vec<u16>,
    pos: usize,
    stack: Vec<u16>,
    registers: Vec<u16>,
    curr_line: String
}

impl Executable {

    pub fn new(fpath: &str) -> Executable {

        let content: Vec<u8> = fs::read(fpath).expect("error reading file");
        assert_eq!(content.len() % 2, 0);

        let mut data: Vec<u16> = Vec::with_capacity(content.len() / 2);
        for (a, b) in content.iter().tuples() {
            data.push(((*b as u16) << 8) + (*a as u16));
        }

        Executable {
            memory: data,
            pos: 0,
            stack: Vec::new(),
            registers: vec![0; 8],
            curr_line: String::new()
        }
    }

    fn next(&mut self) -> u16 {
        let val = self.memory[self.pos];
        self.pos += 1;
        val
    }

    fn more(&self) -> bool {
        return self.memory.len() > self.pos;
    }

    pub fn execute(&mut self) {
        while self.more() {
            let instr = self.next();

            match instr {
                0 => return,
                1 => {
                    let (a, b) = (self.next(), self.next());
                    self.set(a, b)
                },
                2 => {
                    let a = self.next();
                    self.push(a)
                },
                3 => {
                    let a = self.next();
                    self.pop(a)
                },
                4 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.eq(a, b, c)
                },
                5 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.gt(a, b, c)
                },
                6 => {
                    let a = self.next();
                    self.jmp(a)
                },
                7 => {
                    let (a, b) = (self.next(), self.next());
                    self.jt(a, b)
                },
                8 => {
                    let (a, b) = (self.next(), self.next());
                    self.jf(a, b)
                },
                9 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.add(a, b, c)
                },
                10 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.mult(a, b, c)
                },
                11 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.modu(a, b, c)
                },
                12 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.and(a, b, c)
                },
                13 => {
                    let (a, b, c) = (self.next(), self.next(), self.next());
                    self.or(a, b, c)
                },
                14 => {
                    let (a, b) = (self.next(), self.next());
                    self.not(a, b)
                },
                15 => {
                    let (a, b) = (self.next(), self.next());
                    self.rmem(a, b)
                },
                16 => {
                    let (a, b) = (self.next(), self.next());
                    self.wmem(a, b)
                },
                17 => {
                    let a = self.next();
                    self.call(a)
                },
                18 => self.ret(),
                19 => {
                    let a = self.next();
                    self.io_out(a)
                },
                20 => {
                    let a = self.next();
                    self.io_in(a)
                },
                21 => continue,
                _ => return
            }
        }
    }

    fn lit_or_val(&self, a: u16) -> u16 {
        if a < MAX_INT { a } else {
            let reg = (a - MAX_INT) as usize;
            assert_eq!(reg < 8, true);
            self.registers[reg]
        }
    }

    fn addr(&self, a: u16) -> usize {
        let val = self.lit_or_val(a);
        let addr = val & (MAX_INT - 1);
        assert_eq!(addr, val);
        addr as usize
    }

    fn set(&mut self, a: u16, b: u16) {
        let reg = (a - MAX_INT) as usize;
        assert_eq!(reg < 8, true);
        self.registers[reg] = self.lit_or_val(b)
    }

    fn push(&mut self, a: u16) {
        self.stack.push(self.lit_or_val(a))
    }

    fn pop(&mut self, a: u16) {
        let b: u16 = self.stack.pop().expect("empty stack");
        self.set(a, b)
    }

    fn eq(&mut self, a: u16, b: u16, c: u16) {
        if self.lit_or_val(b) == self.lit_or_val(c) {
            self.set(a, 1)
        } else {
            self.set(a, 0)
        }
    }

    fn gt(&mut self, a: u16, b: u16, c: u16) {
        if self.lit_or_val(b) > self.lit_or_val(c) {
            self.set(a, 1)
        } else {
            self.set(a, 0)
        }
    }

    fn jmp(&mut self, a: u16) {
        self.pos = self.addr(a)
    }

    fn jt(&mut self, a: u16, b: u16) {
        if self.lit_or_val(a) > 0 {
            self.jmp(b)
        }
    }

    fn jf(&mut self, a: u16, b: u16) {
        if self.lit_or_val(a) == 0 {
            self.jmp(b)
        }
    }

    fn add(&mut self, a: u16, b: u16, c: u16) {
        let b: usize = self.lit_or_val(b) as usize;
        let c: usize = self.lit_or_val(c) as usize;
        let val: u16 = ((b + c) % (MAX_INT as usize)) as u16;
        self.set(a, val)
    }

    fn mult(&mut self, a: u16, b: u16, c: u16) {
        let b: usize = self.lit_or_val(b) as usize;
        let c: usize = self.lit_or_val(c) as usize;
        let val: u16 = ((b * c) % (MAX_INT as usize)) as u16;
        self.set(a, val)
    }

    fn modu(&mut self, a: u16, b: u16, c: u16) {
        let val = self.lit_or_val(b) % self.lit_or_val(c);
        self.set(a, val)
    }

    fn and(&mut self, a: u16, b: u16, c: u16) {
        let val = self.lit_or_val(b) & self.lit_or_val(c);
        self.set(a, val)
    }

    fn or(&mut self, a: u16, b: u16, c: u16) {
        let val = self.lit_or_val(b) | self.lit_or_val(c);
        self.set(a, val)
    }

    fn not(&mut self, a: u16, b: u16) {
        let val = (!self.lit_or_val(b)) & (MAX_INT - 1);
        self.set(a, val)
    }

    fn rmem(&mut self, a: u16, b: u16) {
        let addr = self.addr(b);
        self.set(a, self.memory[addr])
    }

    fn wmem(&mut self, a: u16, b: u16) {
        let addr = self.addr(a);
        self.memory[addr] = self.lit_or_val(b)
    }

    fn call(&mut self, a: u16) {
        self.push(self.pos as u16);
        self.jmp(a)
    }

    fn ret(&mut self) {
        let addr = self.stack.pop().expect("empty stack");
        let addr = self.addr(addr);
        self.pos = addr;
    }

    fn io_out(&mut self, a: u16) {
        let v = self.lit_or_val(a);
        assert_eq!(v < (1 << 8), true);
        let ascii = v as u8;
        print!("{}", ascii as char)
    }

    fn io_in(&mut self, a: u16) {
        let b: u16 = self.next_char();
        self.set(a, b)
    }

    fn next_char(&mut self) -> u16 {
        if self.curr_line.len() == 0 {
            io::stdin().read_line(&mut self.curr_line).expect("io error");
            let len = self.curr_line.len() - 2;
            self.curr_line.remove(len);
        }
        let char_val = self.curr_line.remove(0);
        let ascii_val = char_val as u32;
        assert_eq!(ascii_val < (1 << 8), true);
        ascii_val as u16
    }

}

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 1")
        .arg(Arg::with_name("FILE")
            .help("binary input file path")
            .required(true)
            .index(1))
        .get_matches();

    Executable::new(args.value_of("FILE").unwrap()).execute();
}
