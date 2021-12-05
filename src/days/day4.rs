use super::super::day::Day;
use std::panic;
use multimap::MultiMap;

pub struct Day4 {
    nums: Vec<usize>,
    boards: Vec<Board>
}

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
            cells,
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

impl Day4 {
    pub fn from_content(content: &str) -> Box<dyn Day> {

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

        Box::new(Day4 {nums, boards})
    }
}

impl Day for Day4 {

    fn part1(&mut self) -> isize {
        for n in self.nums.iter_mut() {
            for b in self.boards.iter_mut() {
                if b.mark_all(*n) {
                    return (*n * b.unhit_sum()).try_into().unwrap();
                }
            }
        }

        0
    }

    fn part2(&mut self) -> isize {
        let num_boards = self.boards.len();
        let mut completed = 0;
        for n in self.nums.iter_mut() {
            for b in self.boards.iter_mut() {
                if !b.done() && b.mark_all(*n) {
                    completed += 1;
                }
                if completed + 1 == num_boards {
                    return (*n * b.unhit_sum()).try_into().unwrap();         
                }
            }
        }

        0
    }
}