use super::super::day::Day;
use itertools::Itertools;
use array2d::Array2D;

pub struct Day13 {
    points: Vec<Pos>,
    folds: Vec<Fold>,
}

impl Day13 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut today = Day13 {
            points: Vec::new(),
            folds: Vec::new()
        };
        for word in content.split_whitespace() {
            if word.contains(',') {
                today.points.push(Pos::from_str(word).unwrap());
            } else if word.contains('=') {
                today.folds.push(Fold::from_str(word).unwrap())
            }
        }
        Ok(Box::new(today))
    }
}

impl Day for Day13 {
    fn part1(&mut self) -> isize {
        let folded: Vec<Pos> = self.folds[0].fold_points(self.points.clone());
        folded.len() as isize
    }

    fn part2(&mut self) -> isize {
        let folded: Vec<Pos> = self.folds.iter()
            .fold(self.points.clone(),
                  |points, fold_instr| fold_instr.fold_points(points));
        let max_x = folded.iter().map(|pos| pos.x).max().unwrap();
        let max_y = folded.iter().map(|pos| pos.y).max().unwrap();
        let mut grid: Array2D<char> = Array2D::filled_with(' ', max_y + 1, max_x + 1);
        for pos in folded {
            grid.set(pos.y, pos.x, '#').unwrap();
        }
        grid.rows_iter().for_each(|mut cell_iter| {
            eprintln!("{}", cell_iter.join(""));
        });
        0
    }
}

#[derive(PartialEq)]
enum FoldDir {
    Row,
    Col,
}

struct Fold {
    location: usize,
    dir: FoldDir,
}

impl Fold {
    fn from_str(word: &str) -> Option<Fold> {
        word.split('=')
            .tuples()
            .map(|(s_dir, s_loc)| Fold {
                dir: match s_dir {
                    "x" => FoldDir::Col,
                    "y" => FoldDir::Row,
                    _ => panic!("unsupported fold direction")
                },
                location: s_loc.parse().unwrap()
            })
            .next()
    }

    fn fold_point(&self, pos: Pos) -> Pos {
        let mut pos: Pos = pos.clone();
        if self.dir == FoldDir::Row {
            if pos.y > self.location {
                pos.y = (self.location * 2) - pos.y;
            }
        } else {
            if pos.x > self.location {
                pos.x = (self.location * 2) - pos.x;
            }
        }
        pos
    }

    fn fold_points(&self, points: Vec<Pos>) -> Vec<Pos> {
        points.into_iter()
            .map(|pos| self.fold_point(pos))
            .unique()
            .collect()
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn from_str(word: &str) -> Option<Pos> {
        word.split(',')
            .flat_map(|n| n.parse().ok())
            .tuples()
            .map(|(x, y)| Pos { x, y })
            .next()
    }
}