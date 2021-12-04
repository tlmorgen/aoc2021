use std::fs;
use clap::{App, Arg};
use std::panic;
use array2d::Array2D;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Cell {
    num: usize,
    hit: bool
}

#[derive(Debug)]
struct Board {
    cells: Array2D<Box<Cell>>,
    test: HashSet<usize>,
    done: bool
}

impl Board {

    pub fn new(rows: Vec<Vec<usize>>) -> Board {
        let num_rows = rows.len();
        let num_cols = rows[0].len();
        let elements: Vec<usize> = rows.concat();
        let el_iter = elements.iter()
            .map(|n| Box::new(Cell {
                num: *n,
                hit: false
            }));
        Board {
            cells: Array2D::from_iter_row_major(el_iter, num_rows, num_cols),
            test: elements.into_iter().collect(),
            done: false
        }
    }

    pub fn mark_all(&mut self, val: &usize) -> bool {
        if !self.test.contains(&val) {
            return false;
        }

        let mut completion: usize = 0;
        for i in 0..self.cells.num_rows() {
            for j in 0..self.cells.num_columns() {
                match self.cells.get_mut(i, j) {
                    Some(cell) => {
                        if cell.num == *val {
                            cell.hit = true;
                            self.done = self.check(i, j);
                            completion += self.done as usize
                        }
                    },
                    None => {}
                }
            }
        }

        completion > 0
    }

    pub fn unhit_sum(&self) -> usize {
        self.cells.elements_column_major_iter()
            .filter(|cell| !cell.hit)
            .map(|cell| cell.num)
            .sum()
    }

    pub fn done(&self) -> bool {
        self.done
    }

    fn check(&self, row: usize, col: usize) -> bool {
        self.cells.row_iter(row)
            .map(|cell| cell.hit)
            .all(|b| b)
        ||
        self.cells.column_iter(col)
            .map(|cell| cell.hit)
            .all(|b| b)
    }
}

fn parse(content: &str) -> (Vec<usize>, Vec<Board>) {
    
    let mut iter = content.lines();
    let conv_num = |n: &str| n.parse::<usize>().unwrap_or_else(|_| panic!("not a num: {}", n));
    let nums: Vec<usize> = iter.next()
        .expect("no first line")
        .split(",")
        .map(conv_num)
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut next_board: Vec<Vec<usize>> = Vec::new();
    for line in iter {
        if line.len() < 1 && next_board.len() > 0 {
            let b = Board::new(next_board);
            boards.push(b);
            next_board = Vec::new();
        } else if line.len() > 0 {
            let row: Vec<usize> = line.split_whitespace()
                .map(conv_num)
                .collect();
            next_board.push(row);
        }
    }

    if next_board.len() > 0 {
        let b = Board::new(next_board);
        boards.push(b);
    }

    (nums, boards)
}

fn part1(nums: &[usize], boards: &mut [Board]) {
    for n in nums {
        for b in boards.iter_mut() {
            if b.mark_all(n) {
                println!("{} {} {}", n, b.unhit_sum(), n * b.unhit_sum());
                return;
            }
        }
    }
}

fn part2(nums: &[usize], boards: &mut [Board]) {
    let num_boards = boards.len();
    let mut completed = 0;
    for n in nums {
        for b in boards.iter_mut() {
            if !b.done() && b.mark_all(n) {
                completed += 1;
            }
            if completed + 1 == num_boards {
                println!("{} {} {}", n, b.unhit_sum(), n * b.unhit_sum());
                return;                
            }
        }
    }
}

fn main() {
    let args = App::new("AoC-2021-01")
        .version("0.0.1")
        .about("Day 4")
        .arg(Arg::with_name("FILE")
            .help("input file path")
            .required(true)
            .index(1))
        .get_matches();

    let content = fs::read_to_string(args.value_of("FILE").unwrap()).expect("io");
    let (nums, mut boards): (Vec<usize>, Vec<Board>) = parse(&content);

    part1(&nums, &mut boards);
    part2(&nums, &mut boards);
}
