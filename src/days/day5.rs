use super::super::day::Day;
use std::cmp;
use itertools::Itertools;


#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

pub struct Day5 {
    lines: Vec<Line>,
}

impl Day5 {
    pub fn from_content(content: &str) -> Box<dyn Day> {
        Box::new(Day5 {
            lines: content.split_whitespace()
                .tuples()
                .map(|(a, _, b)| {
                    Line {
                        a: Point::from_comma_string(a),
                        b: Point::from_comma_string(b),
                    }
                })
                .collect()
        })
    }
}

impl Day for Day5 {
    fn part1(&mut self) -> isize {
        let (max_x, max_y) = self.lines.iter()
            .fold((0, 0), |(max_x, max_y), line| {
                (cmp::max(max_x, cmp::max(line.a.x, line.b.x)),
                 cmp::max(max_y, cmp::max(line.a.y, line.b.y)))
            });

        let mut grid = vec![vec![0 as usize; (max_y + 1)]; (max_x + 1)];
        for line in self.lines.iter() {
            if line.a.x == line.b.x || line.a.y == line.b.y {
                let lx = cmp::min(line.a.x, line.b.x);
                let hx = cmp::max(line.a.x, line.b.x);
                for x in lx..=hx {
                    let ly = cmp::min(line.a.y, line.b.y);
                    let hy = cmp::max(line.a.y, line.b.y);
                    for y in ly..=hy {
                        grid[x][y] += 1;
                    }
                }
            }
        }

        grid.iter()
            .flatten()
            .filter(|&&count| count > 1)
            .count() as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

impl Point {
    fn from_comma_string(s: &str) -> Point {
        s.splitn(2, ",")
            .tuples()
            .map(|(x, y)| {
                Point {
                    x: x.parse().expect("not a num"),
                    y: y.parse().expect("not a num"),
                }
            })
            .next()
            .expect("no points")
    }
}

fn print(grid: &Vec<Vec<usize>>) {
    grid.iter().for_each(|row| eprintln!("{:?}", row));
    eprintln!();
}