use super::super::day::Day;
use std::cmp::{min ,max};
use std::fmt::Formatter;

pub struct Day22 {
    cubes: Vec<Cuboid>
}

impl Day22 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let day = Day22 {
            cubes: content.lines().map(|line| {
                let parts: Vec<&str> = line.split([' ', ',', '=', '.']).collect();
                Cuboid {
                    state: match parts[0] {
                        "on" => true,
                        "off" => false,
                        _ => panic!("invalid state {}", parts[0])
                    },
                    x_range: Range::from_strs(parts[2], parts[4]),
                    y_range: Range::from_strs(parts[6], parts[8]),
                    z_range: Range::from_strs(parts[10], parts[12])
                }
            }).collect()
        };
        assert!(day.cubes.iter().all(|cube| !cube.x_range.is_empty() && !cube.y_range.is_empty() && !cube.z_range.is_empty()));
        Ok(Box::new(day))
    }
}

impl Day for Day22 {
    fn part1(&mut self) -> isize {
        let on_cubes: Vec<Cuboid> = self.cubes.iter()
            .filter(|cube| {
                cube.x_range.start >= -50 && cube.x_range.end <= 50 &&
                    cube.y_range.start >= -50 && cube.y_range.end <= 50 &&
                    cube.z_range.start >= -50 && cube.z_range.end <= 50
            })
            .fold(Vec::new(), fold_cube_states);
        on_cubes.iter().map(Cuboid::count).sum()
    }

    fn part2(&mut self) -> isize {
        let on_cubes: Vec<Cuboid> = self.cubes.iter().fold(Vec::new(), fold_cube_states);
        on_cubes.iter().map(Cuboid::count).sum()
    }
}

fn fold_cube_states(current_on_cubes: Vec<Cuboid>, next_cube: &Cuboid) -> Vec<Cuboid> {
    let mut new_on_cubes: Vec<Cuboid> = Vec::new();
    for on_cube in current_on_cubes { // remove intersect from all previous cubes
        new_on_cubes.extend(on_cube.split_into_on_cubes(next_cube));
    }
    if next_cube.state {
        new_on_cubes.push(next_cube.clone());
    }
    new_on_cubes
}

#[derive(Clone)]
struct Range {
    start: isize,
    end: isize
}

impl Range {
    fn from_strs(start: &str, end: &str) -> Self {
        Range::new(start.parse().unwrap(), end.parse().unwrap())
    }
    fn new(start: isize, end: isize) -> Self {
        Range {
            start,
            end
        }
    }
    fn count(&self) -> isize {
        self.end - self.start + 1
    }
    fn start(&self) -> isize {
        self.start
    }
    fn end(&self) -> isize {
        self.end
    }
    fn is_empty(&self) -> bool {
        self.count() < 1
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.start)
            .field(&self.end)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    state: bool,
    x_range: Range,
    y_range: Range,
    z_range: Range
}

impl Cuboid {
    fn try_new(state: bool, x_range: Range, y_range: Range, z_range: Range) -> Option<Self> {
        if !x_range.is_empty() && !y_range.is_empty() && !z_range.is_empty() {
            Some(Cuboid {
                state,
                x_range,
                y_range,
                z_range,
            })
        } else {
            None
        }
    }

    fn sub_split(&self, intersect: &Cuboid) -> Vec<Cuboid> {
        if self.state {
            let x_below = Range::new(self.x_range.start(), intersect.x_range.start() - 1);
            let x_above = Range::new(intersect.x_range.end() + 1, self.x_range.end());
            let y_below = Range::new(self.y_range.start(), intersect.y_range.start() - 1);
            let y_above = Range::new(intersect.y_range.end() + 1, self.y_range.end());
            let z_below = Range::new(self.z_range.start(), intersect.z_range.start() - 1);
            let z_above = Range::new(intersect.z_range.end() + 1, self.z_range.end());
            [
                // corners
                Cuboid::try_new(true, x_below.clone(), y_below.clone(), z_below.clone()), // lower left back
                Cuboid::try_new(true, x_below.clone(), y_below.clone(), z_above.clone()), // lower left front
                Cuboid::try_new(true, x_below.clone(), y_above.clone(), z_above.clone()), // upper left front
                Cuboid::try_new(true, x_below.clone(), y_above.clone(), z_below.clone()), // upper left back
                Cuboid::try_new(true, x_above.clone(), y_below.clone(), z_below.clone()), // lower right back
                Cuboid::try_new(true, x_above.clone(), y_below.clone(), z_above.clone()), // lower right front
                Cuboid::try_new(true, x_above.clone(), y_above.clone(), z_above.clone()), // upper right front
                Cuboid::try_new(true, x_above.clone(), y_above.clone(), z_below.clone()), // upper right back
                // middle edges
                Cuboid::try_new(true, x_below.clone(), intersect.y_range.clone(), z_below.clone()), // mid left back
                Cuboid::try_new(true, x_below.clone(), intersect.y_range.clone(), z_above.clone()), // mid left front
                Cuboid::try_new(true, x_below.clone(), y_above.clone(), intersect.z_range.clone()), // upper left mid
                Cuboid::try_new(true, x_below.clone(), y_below.clone(), intersect.z_range.clone()), // lower left mid
                Cuboid::try_new(true, x_above.clone(), intersect.y_range.clone(), z_below.clone()), // mid right back
                Cuboid::try_new(true, x_above.clone(), intersect.y_range.clone(), z_above.clone()), // mid right front
                Cuboid::try_new(true, x_above.clone(), y_above.clone(), intersect.z_range.clone()), // upper right mid
                Cuboid::try_new(true, x_above.clone(), y_below.clone(), intersect.z_range.clone()), // lower right mid
                Cuboid::try_new(true, intersect.x_range.clone(), y_above.clone(), z_above.clone()), // upper mid front
                Cuboid::try_new(true, intersect.x_range.clone(), y_above.clone(), z_below.clone()), // upper mid back
                Cuboid::try_new(true, intersect.x_range.clone(), y_below.clone(), z_below.clone()), // lower mid back
                Cuboid::try_new(true, intersect.x_range.clone(), y_below.clone(), z_above.clone()), // lower mid front
                // middle middles
                Cuboid::try_new(true, x_below.clone(), intersect.y_range.clone(), intersect.z_range.clone()), // mid left mid
                Cuboid::try_new(true, x_above.clone(), intersect.y_range.clone(), intersect.z_range.clone()), // mid right mid
                Cuboid::try_new(true, intersect.x_range.clone(), y_above.clone(), intersect.z_range.clone()), // upper mid mid
                Cuboid::try_new(true, intersect.x_range.clone(), y_below.clone(), intersect.z_range.clone()), // lower mid mid
                Cuboid::try_new(true, intersect.x_range.clone(), intersect.y_range.clone(), z_above.clone()), // mid mid front
                Cuboid::try_new(true, intersect.x_range.clone(), intersect.y_range.clone(), z_below.clone()), // mid mid back
            ].into_iter().filter_map(|cube| cube).collect()
        } else {
            panic!("this method should not be called for off cubes")
        }
    }

    fn count(&self) -> isize {
        if self.state {
            if !self.x_range.is_empty() && !self.y_range.is_empty() && !self.z_range.is_empty() {
                self.x_range.count() * self.y_range.count() * self.z_range.count()
            } else {
                panic!("shouldn't be measuring negative cuboid");
            }
        } else {
            eprintln!("shouldn't be measuring off cubes");
            0
        }
    }

    fn split_into_on_cubes(&self, potential_intersect: &Cuboid) -> Vec<Cuboid> {
        if self.state {
            match intersect(self, potential_intersect) {
                None => {
                    vec![self.clone()]
                }
                Some(intersect) => {
                    self.sub_split(&intersect)
                }
            }
        } else {
            panic!("this method should not be called for off cubes")
        }
    }
}

fn intersect(l: &Cuboid, r: &Cuboid) -> Option<Cuboid> {
    let x_start = max(l.x_range.start(), r.x_range.start());
    let x_end = min(l.x_range.end(), r.x_range.end());
    let y_start = max(l.y_range.start(), r.y_range.start());
    let y_end = min(l.y_range.end(), r.y_range.end());
    let z_start = max(l.z_range.start(), r.z_range.start());
    let z_end = min(l.z_range.end(), r.z_range.end());

    Cuboid::try_new(
        l.state && r.state,
        Range::new(x_start, x_end),
        Range::new(y_start, y_end),
        Range::new(z_start, z_end)
    )
}