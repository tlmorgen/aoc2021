use super::super::day::Day;
use std::cmp;
use itertools::Itertools;


#[derive(Debug, Copy, Clone)]
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
    max_x: usize,
    max_y: usize,
}

impl Day5 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let lines: Vec<Line> = content.split_whitespace()
            .tuples()
            .map(|(a, _, b)| Line {
                    a: Point::from_comma_string(a),
                    b: Point::from_comma_string(b),
                })
            .collect();
        let (max_x, max_y) = lines.iter()
            .fold((0, 0), |(max_x, max_y), line| {
                (cmp::max(max_x, cmp::max(line.a.x, line.b.x)),
                 cmp::max(max_y, cmp::max(line.a.y, line.b.y)))
            });

        Ok(Box::new(Day5 {
            lines,
            max_x,
            max_y,
        }))
    }
}

impl Day for Day5 {
    fn part1(&mut self) -> isize {
        let mut grid = vec![vec![0; self.max_y + 1]; self.max_x + 1];
        for line in self.lines.iter() {
            if line.a.x == line.b.x || line.a.y == line.b.y {
                for point in line {
                    grid[point.x][point.y] += 1;
                }
            }
        }

        grid.iter()
            .flatten()
            .filter(|&&count| count > 1)
            .count() as isize
    }

    fn part2(&mut self) -> isize {
        let mut grid = vec![vec![0; self.max_y + 1]; self.max_x + 1];
        for line in self.lines.iter() {
            for point in line {
                grid[point.x][point.y] += 1;
            }
        }

        grid.iter()
            .flatten()
            .filter(|&&count| count > 1)
            .count() as isize
    }
}

impl Point {
    fn from_comma_string(s: &str) -> Point {
        s.splitn(2, ",")
            .tuples()
            .map(|(x, y)| Point {
                    x: x.parse().expect("not a num"),
                    y: y.parse().expect("not a num"),
                })
            .next()
            .expect("no points")
    }
}

struct LineIntoIterator<'a> {
    line: &'a Line,
    next_point: Point,
    x_incr: isize,
    y_incr: isize,
    done: bool,
}

impl<'a> IntoIterator for &'a Line {
    type Item = Point;
    type IntoIter = LineIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LineIntoIterator {
            line: self,
            next_point: self.a,
            x_incr: if self.a.x == self.b.x { 0 } else if self.a.x < self.b.x { 1 } else { -1 },
            y_incr: if self.a.y == self.b.y { 0 } else if self.a.y < self.b.y { 1 } else { -1 },
            done: false,
        }
    }
}

impl<'a> Iterator for LineIntoIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if self.next_point.x == self.line.b.x && self.next_point.y == self.line.b.y {
            self.done = true;
            Some(self.next_point)
        } else {
            let this_point = self.next_point;
            self.next_point = Point {
                x: (self.next_point.x as isize + self.x_incr) as usize,
                y: (self.next_point.y as isize + self.y_incr) as usize,
            };
            Some(this_point)
        }
    }
}