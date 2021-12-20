use super::super::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use petgraph::algo::dijkstra;
use petgraph::graphmap::Neighbors;

type Point = (isize, isize, isize);
type PointSet = HashSet<Point>;

pub struct Day19 {
    scanners: Vec<ScannerView>,
}

impl Day19 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut scanners = Vec::new();
        let mut scanner_idx = 0;
        let mut curr_view = ScannerView::new(scanner_idx);
        scanner_idx += 1;
        for line in content.lines() {
            if line.contains("---") {
                if curr_view.beacons.len() > 0 {
                    scanners.push(curr_view);
                    curr_view = ScannerView::new(scanner_idx);
                    scanner_idx += 1;
                }
            } else if line.len() > 0 {
                curr_view.parse_line_add(line);
            }
        }
        if curr_view.beacons.len() > 0 {
            scanners.push(curr_view);
        }
        Ok(Box::new(Day19 {
            scanners
        }))
    }
}

type DirectedConverters = HashMap<(usize, usize), Converter>;

impl Day for Day19 {
    fn part1(&mut self) -> isize {

        let mut graph: DiGraphMap<usize, ()> = DiGraphMap::new();
        let mut converters: DirectedConverters = HashMap::new();

        for pair in self.scanners.iter().permutations(2) {
            match pair[0].find_alignment(pair[1]) {
                None => {}
                Some(converter) => {
                    graph.add_edge(pair[0].id, pair[1].id, ());
                    converters.insert((pair[1].id, pair[0].id), converter);
                }
            }
        }

        let mut visits: HashSet<usize> = HashSet::new();
        reverse_translate(&graph, &mut visits, 0, None, &self.scanners, &converters).len() as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

fn reverse_translate(graph: &DiGraphMap<usize, ()>, visits: &mut HashSet<usize>, id: usize, prev_id: Option<usize>, sets: &Vec<ScannerView>, converters: &DirectedConverters) -> PointSet {
    visits.insert(id);
    let mut combined_points: PointSet = sets[id].beacons.clone();
    for neighbor in graph.neighbors(id) {
        if !visits.contains(&neighbor) {
            for point in reverse_translate(graph, visits, neighbor, Some(id), sets, converters) {
                combined_points.insert(point);
            }
        }
    }

    match prev_id {
        None => combined_points,
        Some(to_id) => {
            match converters.get(&(id, to_id)) {
                None => panic!("no converter for {}->{}", id, to_id),
                Some(converter) => {
                    combined_points.into_iter()
                        .map(converter.1)
                        .map(|(x, y, z)| (x + converter.0.0, y + converter.0.1, z + converter.0.2))
                        .collect()

                }
            }
        }
    }
}

type Translator = fn(Point) -> Point;
type Converter = (Point, Translator);

struct ScannerView {
    id: usize,
    beacons: PointSet,
}

impl ScannerView {
    fn new(id: usize) -> ScannerView {
        ScannerView {
            id,
            beacons: HashSet::new()
        }
    }
    fn parse_line_add(&mut self, line: &str) {
        self.beacons.insert(line.split(',')
            .map(|word| word.parse().unwrap())
            .tuples()
            .next()
            .unwrap());
    }
    fn find_alignment(&self, other: &ScannerView) -> Option<Converter> {
        for (forward, backward) in TRANSLATORS.iter().tuples() {
            let translated: PointSet = self.beacons.clone().into_iter()
                .map(forward)
                .collect();

            for x_diff in cartesian_diff(&get_field_set(&translated, get_x), &get_field_set(&other.beacons, get_x)) {
                let (l_matches, r_matches) = field_intersect(&translated, &other.beacons, x_diff, get_x);

                if l_matches.len() < 12 { continue; }

                for y_diff in cartesian_diff(&get_field_set(&l_matches, get_y), &get_field_set(&r_matches, get_y)) {
                    let (l_matches, r_matches) = field_intersect(&l_matches, &r_matches, y_diff, get_y);

                    if l_matches.len() < 12 { continue; }

                    for z_diff in cartesian_diff(&get_field_set(&l_matches, get_z), &get_field_set(&r_matches, get_z)) {
                        let (l_matches, r_matches) = field_intersect(&l_matches, &r_matches, z_diff, get_z);

                        if l_matches.len() > 11 {
                            return Some((backward((x_diff, y_diff, z_diff)), *backward))
                        }
                    }
                }
            }
        }
        None
    }
}

const TRANSLATORS: [Translator; 48] = [
    |(x, y, z)| (x, y, z), |(x, y, z)| (x, y, z), // original
    |(x, y, z)| (x, z, -y), |(x, y, z)| (x, -z, y), // 90 about x
    |(x, y, z)| (x, -y, -z), |(x, y, z)| (x, -y, -z), // 180 about x
    |(x, y, z)| (x, -z, y), |(x, y, z)| (x, z, -y), // 260 about x
    |(x, y, z)| (z, y, -x), |(x, y, z)| (-z, y, x), // 90 about y (new section)
    |(x, y, z)| (-y, z, -x), |(x, y, z)| (-z, -x, y), // and then 90 about old x
    |(x, y, z)| (-z, -y, -x), |(x, y, z)| (-z, -y, -x), // and then 180 about old x
    |(x, y, z)| (y, -z, -x), |(x, y, z)| (-z, x, -y), // and then 260 about old x
    |(x, y, z)| (-x, y, -z), |(x, y, z)| (-x, y, -z), // and then 180 about old y (new section)
    |(x, y, z)| (-x, z, y), |(x, y, z)| (-x, z, y), // and then 90 about old x
    |(x, y, z)| (-x, -y, z), |(x, y, z)| (-x, -y, z), // and then 180 about old x
    |(x, y, z)| (-x, -z, -y), |(x, y, z)| (-x, -z, -y), // and then 260 about old x
    |(x, y, z)| (-z, y, x), |(x, y, z)| (z, y, -x), // and then 260 about y (new section)
    |(x, y, z)| (-y, -z, x), |(x, y, z)| (z, -x, -y), // and then 90 about old x
    |(x, y, z)| (z, -y, x), |(x, y, z)| (z, -y, x), // and then 180 about old x
    |(x, y, z)| (y, z, x), |(x, y, z)| (z, x, y), // and then 260 about old x
    |(x, y, z)| (-y, x, z), |(x, y, z)| (y, -x, z), // (from original) 90 about z
    |(x, y, z)| (z, x, y), |(x, y, z)| (y, z, x), // and then 90 about old x
    |(x, y, z)| (y, x, -z), |(x, y, z)| (y, x, -z), // and then 180 about old x
    |(x, y, z)| (-z, x, -y), |(x, y, z)| (y, -z, -x), // and then 260 about old x
    // (from original) 180 about z - already covered
    |(x, y, z)| (y, -x, z), |(x, y, z)| (-y, x, z), // (from original) 260 about z
    |(x, y, z)| (z, -x, -y), |(x, y, z)| (-y, -z, x), // and then 90 about old x
    |(x, y, z)| (-y, -x, -z), |(x, y, z)| (-y, -x, -z), // and then 180 about old x
    |(x, y, z)| (-z, -x, y), |(x, y, z)| (-y, z, -x), // and then 260 about old x
];

type FieldGetter = fn(&Point) -> isize;

fn get_x(p: &Point) -> isize {
    p.0
}

fn get_y(p: &Point) -> isize {
    p.1
}

fn get_z(p: &Point) -> isize {
    p.2
}

fn get_field_set(points: &PointSet, field: FieldGetter) -> HashSet<isize> {
    points.clone().iter().map(field).collect()
}

fn field_intersect(l: &PointSet, r: &PointSet, l_diff: isize, field: FieldGetter) -> (PointSet, PointSet) {
    (
        l.clone().into_iter()
            .filter(|lp| r.iter().any(|rp| field(lp) - l_diff == field(rp)))
            .collect(),
        r.clone().into_iter()
            .filter(|rp| l.iter().any(|lp| field(lp) - l_diff == field(rp)))
            .collect()
    )
}

fn cartesian_diff(l: &HashSet<isize>, r: &HashSet<isize>) -> HashSet<isize> {
    l.iter().flat_map(|ln| r.iter().map(|rn| *ln - *rn))
        .collect()
}