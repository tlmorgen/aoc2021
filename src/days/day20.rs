use itertools::Itertools;
use super::super::day::Day;
use array2d::Array2D;

type Idx = (isize, isize);

pub struct Day20 {
    image: Image,
}

impl Day20 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut lines = content.lines();
        let enhancer: Vec<char> = lines.next().unwrap().chars().collect();
        lines.next();
        let base_rows = lines.map(|line| {
            line.chars().collect()
        }).collect::<Vec<Vec<char>>>();
        Ok(Box::new(Day20 {
            image: Image::from(Array2D::from_rows(&base_rows), enhancer)
        }))
    }
}

impl Day for Day20 {
    fn part1(&mut self) -> isize {
        self.image.enhance().enhance().pixel_count() as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

#[derive(Clone)]
struct Image {
    cells: Array2D<char>,
    enhancement_data: Vec<char>,
    infinity: char
}

impl Image {
    fn from(cells: Array2D<char>, enhancement_data: Vec<char>) -> Self {
        let infinity = enhancement_data[0];
        Image {
            cells,
            enhancement_data,
            infinity
        }
    }

    fn display(&self) -> String {
        self.cells.rows_iter()
            .map(|mut row| row.join(""))
            .join("\n")
    }

    fn enhance(&self) -> Image {
        let expand_radius = 1usize;
        let new_infinity = if self.infinity == '.' {
            self.enhancement_data[0]
        } else {
            self.enhancement_data[self.enhancement_data.len() - 1]
        };
        let mut new_cells = Array2D::filled_with(new_infinity,
                                                 self.cells.num_rows() + 2 * expand_radius,
                                                 self.cells.num_columns() + 2 * expand_radius);
        for i in 0..new_cells.num_rows() {
            for j in 0..new_cells.num_columns() {
                let old_i = (i as isize) - expand_radius as isize;
                let old_j = (j as isize) - expand_radius as isize;
                let values: Vec<char> = self.build_group(&(old_i, old_j)).iter()
                    .map(|p| self.get_cell_or_default(p, new_infinity))
                    .collect();
                if values.len() > 0 {
                    new_cells[(i, j)] = self.get_enhancement_cell(values);
                }
            }
        }
        Image {
            cells: new_cells,
            enhancement_data: self.enhancement_data.clone(),
            infinity: new_infinity
        }
    }

    fn validate_idx(&self, p: &Idx) -> Option<(usize, usize)> {
        if p.0 > -1 && p.1 > -1
            && (p.0 as usize) < self.cells.num_rows()
            && (p.1 as usize) < self.cells.num_columns()
        {
            Some((p.0 as usize, p.1 as usize))
        } else {
            None
        }
    }

    fn get_cell_or_default(&self, p: &Idx, default: char) -> char {
        match self.validate_idx(p) {
            None => default,
            Some(p) => self.cells[p]
        }
    }

    fn get_enhancement_cell(&self, chars: Vec<char>) -> char {
        let b_str = chars.iter().map(|c| match *c {
            '.' => "0",
            '#' => "1",
            _ => panic!("invalid char {}", c)
        }).join("");
        let idx = usize::from_str_radix(&b_str, 2).unwrap();
        self.enhancement_data[idx]
    }

    fn pixel_count(&self) -> usize {
        if self.infinity == '#' {
            usize::MAX
        } else {
            self.cells.elements_row_major_iter()
                .map(|c| match *c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("invalid char {}", c)
                }).sum()
        }
    }

    fn build_group(&self, p: &Idx) -> Vec<Idx> {
        let idxs: Vec<Idx> = [
            (p.0 - 1, p.1 - 1), // up left
            (p.0 - 1, p.1), // up
            (p.0 - 1, p.1 + 1), // up right
            (p.0, p.1 - 1), // left
            (p.0, p.1), // current
            (p.0, p.1 + 1), // right
            (p.0 + 1, p.1 - 1), // down left
            (p.0 + 1, p.1), // down
            (p.0 + 1, p.1 + 1) // down right
        ].into_iter().collect();
        if idxs.iter().filter_map(|p| self.validate_idx(p)).count() > 0 {
            idxs
        } else {
            Vec::new()
        }
    }
}

