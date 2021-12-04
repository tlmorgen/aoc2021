use std::fs;
use clap::{App, Arg};
use std::panic;
use multimap::MultiMap;

#[derive(Clone, Debug)]
struct Cell {
    row: usize,
    col: usize,
    num: usize,
    hit: bool
}

#[derive(Debug)]
struct Board {
    cells: MultiMap<usize, Cell>,
    row_hits: Vec<usize>,
    col_hits: Vec<usize>,
    done: bool
}

impl Board {

    pub fn from_row_major(rows: Vec<Vec<usize>>) -> Board {
        let mut cells: MultiMap<usize, Cell> = MultiMap::new();
        rows.iter().enumerate()
            .map(|(i, row)| row.iter().enumerate()
                .map(move |(j, &num)| Cell{
                    row: i,
                    col: j,
                    num: num,
                    hit: false
                }))
            .flatten()
            .for_each(|cell| cells.insert(cell.num, cell));
        Board {
            cells: cells,
            row_hits: vec![0; rows.len()],
            col_hits: vec![0; rows[0].len()],
            done: false
        }
    }

    pub fn mark_all(&mut self, val: usize) -> bool {

        for cell in self.cells.get_vec_mut(&val).into_iter().flatten() { // TODO better terse way to not care about Option
            if cell.hit {continue}
            
            cell.hit = true;
            self.row_hits[cell.row] += 1;
            self.col_hits[cell.col] += 1;
            if self.row_hits[cell.row] == self.col_hits.len() {
                self.done = true;
            }
            if self.col_hits[cell.col] == self.row_hits.len() {
                self.done = true;
            }

        }

        self.done
    }

    pub fn unhit_sum(&self) -> usize {
        self.cells.iter_all()
            .map(|(_, vec)| vec) // no iterator over all values :(
            .flatten()
            .filter(|cell| !cell.hit)
            .map(|cell| cell.num)
            .sum()
    }

    pub fn done(&self) -> bool {
        self.done
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
            let b = Board::from_row_major(next_board);
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
        let b = Board::from_row_major(next_board);
        boards.push(b);
    }

    (nums, boards)
}

fn part1(nums: &[usize], boards: &mut [Board]) {
    for &n in nums {
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
    for &n in nums {
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
