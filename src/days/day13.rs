use super::super::day::Day;
use itertools::Itertools;
use array2d::Array2D;

pub struct Day13 {
    points: Vec<Pos>,
    folds: Vec<Fold>
}

impl Day13 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let (points, folds) = content.lines()
            .fold((Vec::new(), Vec::new()), |(mut points, mut folds), line| {
                if line.contains(',') {
                    points.push(Pos::from_str(line))
                } else if line.contains('=') {
                    line.split_whitespace().filter(|word| word.contains('='))
                        .for_each(|word| folds.push(Fold::from_str(word)))
                };
                (points, folds)
            });
        Ok(Box::new(Day13 {
            points,
            folds
        }))
    }
}

impl Day for Day13 {
    fn part1(&mut self) -> isize {
        let folded: Vec<Pos> = self.folds[0].fold_points(self.points.clone());
        folded.len() as isize
    }

    fn part2(&mut self) -> isize {
        let folded: Vec<Pos> = self.folds.iter().fold(self.points.clone(), |points, fold_instr| {
            fold_instr.fold_points(points)
        });
        let max_x = folded.iter().map(|pos| pos.x).max().unwrap();
        let max_y = folded.iter().map(|pos| pos.y).max().unwrap();
        let mut grid: Array2D<char> = Array2D::filled_with(' ', (max_y + 1) as usize, (max_x + 1) as usize);
        for pos in folded {
            grid.set(pos.y as usize, pos.x as usize, '#');
        }
        grid.rows_iter().for_each(|mut col_iter| {
            eprintln!("{}", col_iter.join(""));
        });
        0
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Debug)]
struct Pos {
    x: isize,
    y: isize
}

#[derive(Eq, PartialEq)]
enum FoldDir {
    Row,
    Col
}

struct Fold {
    location: isize,
    fold: FoldDir
}

impl Fold {
    fn from_str(word: &str) -> Fold {
        let mut iter = word.splitn(2, '=');
        let fold = match iter.next().unwrap() {
            "x" => FoldDir::Col,
            "y" => FoldDir::Row,
            _ => panic!("unsupported fold direction")
        };
        let location = iter.next().unwrap().parse::<isize>().unwrap();
        Fold {
            location,
            fold
        }
    }

    fn fold_points(&self, points: Vec<Pos>) -> Vec<Pos> {
        if self.fold == FoldDir::Row {
            points.into_iter().map(|pos| {
                if pos.y > self.location {
                    Pos {
                        x: pos.x,
                        y: self.location - (pos.y - self.location),
                    }
                } else {
                    pos
                }
            }).unique().collect()
        } else {
            points.into_iter().map(|pos| {
                if pos.x > self.location {
                    Pos {
                        x: self.location - (pos.x - self.location),
                        y: pos.y,
                    }
                } else {
                    pos
                }
            }).unique().collect()
        }
    }
}

impl Pos {
    fn from_str(word: &str) -> Pos {
        word.split(',')
            .flat_map(|n| n.parse().ok())
            .tuples()
            .map(|(row,  col)| Pos { x: row, y: col })
            .next()
            .expect("no pos on this line")
    }
}