use std::cmp;
use std::collections::{HashMap, HashSet};
use super::super::day::Day;

#[derive(Debug)]
pub struct Day17 {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize
}

impl Day17 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let words: Vec<&str> = content.trim().split([' ', '=', ':', '.', ','])
            .filter(|word| word.len() > 0)
            .collect();
        Ok(Box::new(Day17 {
            x_min: words[3].parse().unwrap(),
            x_max: words[4].parse().unwrap(),
            y_min: words[6].parse().unwrap(),
            y_max: words[7].parse().unwrap(),
        }))
    }

    fn hit(&self, x: isize, y: isize) -> bool {
        x >= self.x_min && x <= self.x_max
            && y >= self.y_min && y <= self.y_max
    }

    fn miss(&self, x: isize, y: isize) -> bool {
        x > self.x_max || y < self.y_min
    }

    fn step(&self, x: &mut isize, y: &mut isize, xv: &mut isize, yv: &mut isize) {
        *x += *xv;
        *y += *yv;
        if *xv < 0 {
            *xv += 1;
        } else if *xv > 0 {
            *xv -= 1;
        }
        *yv -= 1;
    }

    fn launch(&self, mut xv: isize, mut yv: isize) -> Option<isize> {
        let mut y_epoch = 0isize;
        let mut x = 0isize;
        let mut y = 0isize;
        while !self.hit(x, y) && !self.miss(x, y) {
            self.step(&mut x, &mut y, &mut xv, &mut yv);
            y_epoch = cmp::max(y_epoch, y);
        }
        if self.hit(x, y) {
            Some(y_epoch)
        } else {
            None
        }
    }
}

impl Day for Day17 {
    fn part1(&mut self) -> isize {
        let mut y_epoch = 0isize;
        let xv_min = 1isize; // really slow
        let xv_max = self.x_max; // immediate overshoot
        let yv_min = self.y_min; // immediate overshoot
        let yv_max = 1000; // math
        for xv in xv_min..=xv_max {
            for yv in yv_min..=yv_max {
                match self.launch(xv, yv) {
                    None => {}
                    Some(this_y_epoch) => {
                        y_epoch = cmp::max(y_epoch, this_y_epoch);
                    }
                }
            }
        }
        y_epoch
    }

    fn part2(&mut self) -> isize {
        let mut success: HashSet<(isize, isize)> = HashSet::new();
        let xv_min = 1isize; // really slow
        let xv_max = self.x_max; // immediate overshoot
        let yv_min = self.y_min; // immediate overshoot
        let yv_max = 1000; // math
        for xv in xv_min..=xv_max {
            for yv in yv_min..=yv_max {
                match self.launch(xv, yv) {
                    None => {}
                    Some(_) => {
                        success.insert((xv, yv));
                    }
                }
            }
        }
        success.len() as isize
    }
}