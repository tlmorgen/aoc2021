use super::super::day::Day;
use std::str::Chars;
use itertools::Itertools;
use std::fmt::Formatter;
use std::{cmp, fmt};

pub struct Day18 {
    numbers: Vec<SnailFishNumber>
}

impl Day18 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day18 {
            numbers: content.lines()
                .map(|line| SnailFishNumber::from_chars(&mut line.chars()))
                .collect()
        }))
    }
}

impl Day for Day18 {
    fn part1(&mut self) -> isize {
        let mut numbers = self.numbers.clone();
        numbers.reverse();
        let mut sum = numbers.pop().unwrap();
        sum.reduce();
        while numbers.len() > 0 {
            sum = sum.add(numbers.pop().unwrap());
            sum.reduce();
        }
        sum.magnitude()
    }

    fn part2(&mut self) -> isize {
        self.numbers.iter().permutations(2).fold(0isize, |max, pair| {
            let mut sum = pair[0].add_refs(pair[1]);
            sum.reduce();
            let mag = sum.magnitude();
            cmp::max(max, mag)
        })
    }
}

#[derive(Clone)]
struct SnailFishNumber {
    left: Option<Box<SnailFishNumber>>,
    right: Option<Box<SnailFishNumber>>,
    literal: Option<isize>
}

impl SnailFishNumber {
    fn from_chars(iter: &mut Chars) -> SnailFishNumber {
        match iter.next() {
            None => {
                panic!("cannot parse nothing");
            }
            Some(c) => {
                if c == '[' {
                    // this is me
                } else if c >= '0' && c <= '9' {
                    // first char is a number
                    return SnailFishNumber::from_char_num(c);
                } else {
                    panic!("invalid beginning: {}", c);
                }
            }
        }

        let left = SnailFishNumber::from_chars(iter);
        match iter.next() {
            None => {
                panic!("EOF mid parse");
            }
            Some(c) => {
                if c == ',' {
                    // expected
                } else {
                    panic!("all snailfish numbers are pairs: {}", c);
                }
            }
        }
        let right = SnailFishNumber::from_chars(iter);

        match iter.next() {
            None => {
                panic!("EOF mid parse");
            }
            Some(c) => {
                if c == ']' {
                    // expected
                } else {
                    panic!("all snailfish numbers are pairs (ie 2): {}", c);
                }
            }
        }

        SnailFishNumber {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            literal: None
        }
    }

    fn from_char_num(c: char) -> SnailFishNumber {
        SnailFishNumber {
            left: None,
            right: None,
            literal: Some((c as usize - '0' as usize) as isize)
        }
    }

    fn from_num(num: isize) -> SnailFishNumber {
        SnailFishNumber {
            left: None,
            right: None,
            literal: Some(num)
        }
    }

    fn add_refs(&self, _rhs: &SnailFishNumber) -> SnailFishNumber {
        SnailFishNumber {
            left: Some(Box::new(self.clone())),
            right: Some(Box::new(_rhs.clone())),
            literal: None
        }
    }

    fn add(self, _rhs: SnailFishNumber) -> SnailFishNumber {
        SnailFishNumber {
            left: Some(Box::new(self)),
            right: Some(Box::new(_rhs)),
            literal: None
        }
    }

    fn incr_lit(&mut self, _rhs: isize) {
        if !self.is_lit() {
            panic!("cannot incr a non-lit");
        }
        self.literal = Some(self.literal.unwrap() + _rhs);
    }

    fn left_is_lit(&self) -> bool {
        self.left.is_some() && self.left.as_ref().unwrap().is_lit()
    }

    fn right_is_lit(&self) -> bool {
        self.right.is_some() && self.right.as_ref().unwrap().is_lit()
    }

    fn is_lit(&self) -> bool {
        self.literal.is_some()
    }

    fn is_lit_pair(&self) -> bool {
        self.left_is_lit() && self.right_is_lit()
    }

    fn reduce(&mut self) {
        loop {
            loop {
                let explode = self.explode(1);
                if !explode.is_some() {break}
            }
            let splitted = self.split();
            let mut any_explodes = false;
            loop {
                let explode = self.explode(1);
                any_explodes |= explode.is_some();
                if !explode.is_some() {break}
            }
            if !splitted && !any_explodes {break}
        }
    }

    fn split(&mut self) -> bool {
        if self.is_lit() {
            if self.literal.unwrap() >= 10 {
                let lit = self.literal.take().unwrap();
                self.left = Some(Box::new(SnailFishNumber::from_num(lit / 2)));
                self.right = Some(Box::new(SnailFishNumber::from_num((lit + 1) / 2)));
                true
            } else {
                false
            }
        } else {
            if !self.left.as_mut().unwrap().split() {
                self.right.as_mut().unwrap().split()
            } else {
                true
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Option<SnailFishNumber> {
        if depth > 4 && self.is_lit_pair() {
            let explode_bubble = SnailFishNumber {
                left: self.left.take(),
                right: self.right.take(),
                literal: None
            };
            self.literal = Some(0);
            Some(explode_bubble)
        } else if self.is_lit() {
            None
        } else {
            match self.left.as_mut().unwrap().explode(depth + 1) {
                Some(explode) => {
                    self.try_merge_up(explode, Side::Left)
                }
                None => {
                    match self.right.as_mut().unwrap().explode(depth + 1) {
                        None => None,
                        Some(explode) => {
                            self.try_merge_up(explode, Side::Right)
                        }
                    }
                }
            }
        }
    }

    fn try_merge_up(&mut self, explode: SnailFishNumber, source: Side) -> Option<SnailFishNumber> {
        match source {
            Side::Left => {
                self.right.as_mut().unwrap().try_merge_down_rightward(explode)
            }
            Side::Right => {
                self.left.as_mut().unwrap().try_merge_down_leftward(explode)
            }
        }
    }

    fn try_merge_down_leftward(&mut self, mut explode: SnailFishNumber) -> Option<SnailFishNumber> {
        if explode.left.is_some() {
            if self.is_lit() {
                self.incr_lit(explode.left.take().unwrap().literal.unwrap());
                Some(explode)
            } else {
                let result = self.right.as_mut().unwrap().try_merge_down_leftward(explode);
                self.left.as_mut().unwrap().try_merge_down_leftward(result.unwrap())
            }
        } else {
            Some(explode) // nothing to do
        }
    }

    fn try_merge_down_rightward(&mut self, mut explode: SnailFishNumber) -> Option<SnailFishNumber> {
        if explode.right.is_some() {
            if self.is_lit() {
                self.incr_lit(explode.right.take().unwrap().literal.unwrap());
                Some(explode)
            } else {
                let result = self.left.as_mut().unwrap().try_merge_down_rightward(explode);
                self.right.as_mut().unwrap().try_merge_down_rightward(result.unwrap())
            }
        } else {
            Some(explode) // nothing to do
        }
    }

    fn magnitude(&self) -> isize {
        if self.is_lit() {
            self.literal.unwrap()
        } else {
            3 * self.left.as_ref().unwrap().magnitude()
                + 2 * self.right.as_ref().unwrap().magnitude()
        }
    }
}

enum Side {
    Left,
    Right
}

impl fmt::Debug for SnailFishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_lit() {
            f.write_fmt(format_args!("{}", self.literal.unwrap()))
        } else {
            let mut list = f.debug_list();
            if self.left.is_some() {
                list.entry(self.left.as_ref().unwrap());
            }
            if self.right.is_some() {
                list.entry(self.right.as_ref().unwrap());
            }
            list.finish()
        }
    }
}