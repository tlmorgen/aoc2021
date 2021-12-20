use super::super::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use petgraph::prelude::*;
use rayon::prelude::*;

type Point = (isize, isize, isize);
type PointSet = HashSet<Point>;
type Translator = fn(&Point) -> Point;
type Converter = (Point, Translator);
type DirectedConverters = HashMap<(usize, usize), Converter>;
type FieldGetter = fn(&Point) -> isize;

pub struct Day19 {
    scanners: Vec<ScannerView>,
    graph: DiGraphMap<usize, ()>,
    converters: DirectedConverters
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

        let mut graph: DiGraphMap<usize, ()> = DiGraphMap::new();
        let mut converters: DirectedConverters = HashMap::new();

        let perms: Vec<Vec<ScannerView>> = scanners.clone().into_iter()
            .permutations(2)
            .collect();
        let results: Vec<(usize, usize, Converter)> = perms.par_iter()
            .map(|pair| {
                match pair[0].find_alignment(&pair[1]) {
                    None => None,
                    Some(converter) => {
                        Some((pair[0].id, pair[1].id, converter))
                    }
                }
            })
            .filter_map(|opt| opt)
            .collect();

        for (from_id, to_id, backward_converter) in results {
            graph.add_edge(from_id, to_id, ());
            converters.insert((to_id, from_id), backward_converter);
        }

        Ok(Box::new(Day19 {
            scanners,
            graph,
            converters
        }))
    }
}

impl Day for Day19 {
    fn part1(&mut self) -> isize {
        let mut visits: HashSet<usize> = HashSet::new();
        reverse_translate(
            &self.graph,
            &mut visits,
            0,
            None,
            &self.scanners,
            &self.converters,
            |id, sets| sets[id].beacons.clone()
        ).len() as isize
    }

    fn part2(&mut self) -> isize {
        let mut visits: HashSet<usize> = HashSet::new();
        let scanners = reverse_translate(
            &self.graph,
            &mut visits,
            0,
            None,
            &self.scanners,
            &self.converters,
            |_, _| HashSet::from([(0isize, 0isize, 0isize)])
        );
        scanners.iter().permutations(2)
            .map(|pair| (pair[0].0 - pair[1].0).abs() + (pair[0].1 - pair[1].1).abs() + (pair[0].2 - pair[1].2).abs())
            .max()
            .unwrap() as isize
    }
}

fn reverse_translate(
    graph: &DiGraphMap<usize, ()>,
    visits: &mut HashSet<usize>,
    id: usize,
    prev_id: Option<usize>,
    sets: &Vec<ScannerView>,
    converters: &DirectedConverters,
    scanner_view_getter: fn(usize, &Vec<ScannerView>) -> PointSet
) -> PointSet {
    visits.insert(id);
    let mut combined_points = scanner_view_getter(id, sets);
    for neighbor in graph.neighbors(id) {
        if !visits.contains(&neighbor) {
            for point in reverse_translate(graph, visits, neighbor, Some(id), sets, converters, scanner_view_getter) {
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
                    combined_points.iter()
                        .map(converter.1)
                        .map(|(x, y, z)| (x + converter.0.0, y + converter.0.1, z + converter.0.2))
                        .collect()
                }
            }
        }
    }
}

#[derive(Clone)]
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
            let translated: PointSet = self.beacons.iter()
                .map(forward)
                .collect();

            for x_diff in cartesian_diff_filter(get_pos_counts(&translated, get_x), get_pos_counts(&other.beacons, get_x)) {
                let (l_matches, r_matches) = field_intersect(&translated, &other.beacons, x_diff, get_x);

                if l_matches.len() < 12 {continue}

                for y_diff in cartesian_diff_filter(get_pos_counts(&l_matches, get_y), get_pos_counts(&r_matches, get_y)) {
                    let (l_matches, r_matches) = field_intersect(&l_matches, &r_matches, y_diff, get_y);

                    if l_matches.len() < 12 {continue}

                    for z_diff in cartesian_diff_filter(get_pos_counts(&l_matches, get_z), get_pos_counts(&r_matches, get_z)) {

                        if l_matches.len() < 12 {continue}

                        return Some((backward(&(x_diff, y_diff, z_diff)), *backward))
                    }
                }
            }
        }
        None
    }
}

// forward, backward
const TRANSLATORS: [Translator; 48] = [
    |&(x, y, z)| (x, y, z), |&(x, y, z)| (x, y, z), // original
    |&(x, y, z)| (x, z, -y), |&(x, y, z)| (x, -z, y), // 90 about x
    |&(x, y, z)| (x, -y, -z), |&(x, y, z)| (x, -y, -z), // 180 about x
    |&(x, y, z)| (x, -z, y), |&(x, y, z)| (x, z, -y), // 260 about x
    |&(x, y, z)| (z, y, -x), |&(x, y, z)| (-z, y, x), // 90 about y (new section)
    |&(x, y, z)| (-y, z, -x), |&(x, y, z)| (-z, -x, y), // and then 90 about old x
    |&(x, y, z)| (-z, -y, -x), |&(x, y, z)| (-z, -y, -x), // and then 180 about old x
    |&(x, y, z)| (y, -z, -x), |&(x, y, z)| (-z, x, -y), // and then 260 about old x
    |&(x, y, z)| (-x, y, -z), |&(x, y, z)| (-x, y, -z), // and then 180 about old y (new section)
    |&(x, y, z)| (-x, z, y), |&(x, y, z)| (-x, z, y), // and then 90 about old x
    |&(x, y, z)| (-x, -y, z), |&(x, y, z)| (-x, -y, z), // and then 180 about old x
    |&(x, y, z)| (-x, -z, -y), |&(x, y, z)| (-x, -z, -y), // and then 260 about old x
    |&(x, y, z)| (-z, y, x), |&(x, y, z)| (z, y, -x), // and then 260 about y (new section)
    |&(x, y, z)| (-y, -z, x), |&(x, y, z)| (z, -x, -y), // and then 90 about old x
    |&(x, y, z)| (z, -y, x), |&(x, y, z)| (z, -y, x), // and then 180 about old x
    |&(x, y, z)| (y, z, x), |&(x, y, z)| (z, x, y), // and then 260 about old x
    |&(x, y, z)| (-y, x, z), |&(x, y, z)| (y, -x, z), // (from original) 90 about z
    |&(x, y, z)| (z, x, y), |&(x, y, z)| (y, z, x), // and then 90 about old x
    |&(x, y, z)| (y, x, -z), |&(x, y, z)| (y, x, -z), // and then 180 about old x
    |&(x, y, z)| (-z, x, -y), |&(x, y, z)| (y, -z, -x), // and then 260 about old x
    // (from original) 180 about z - already covered
    |&(x, y, z)| (y, -x, z), |&(x, y, z)| (-y, x, z), // (from original) 260 about z
    |&(x, y, z)| (z, -x, -y), |&(x, y, z)| (-y, -z, x), // and then 90 about old x
    |&(x, y, z)| (-y, -x, -z), |&(x, y, z)| (-y, -x, -z), // and then 180 about old x
    |&(x, y, z)| (-z, -x, y), |&(x, y, z)| (-y, z, -x), // and then 260 about old x
];

fn get_x(p: &Point) -> isize {
    p.0
}

fn get_y(p: &Point) -> isize {
    p.1
}

fn get_z(p: &Point) -> isize {
    p.2
}

fn get_pos_counts(points: &PointSet, field: FieldGetter) -> HashMap<isize, usize> {
    let mut pos_counts: HashMap<isize, usize> = HashMap::new();
    for p in points.iter() {
        *pos_counts.entry(field(p)).or_insert(0) += 1;
    }
    pos_counts
}

fn cartesian_diff_filter(l: HashMap<isize, usize>, r: HashMap<isize, usize>) -> HashSet<isize> {
    let mut diffs: HashSet<isize> = HashSet::new();
    for ln in l.keys() {
        for rn in r.keys() {
            let diff = *ln - *rn;
            if diffs.contains(&diff) {continue}
            if l.iter()
                .fold(0usize, |sum, (ln, l_count)| {
                    sum + (*l_count * *r.get(&(*ln - diff)).unwrap_or(&0))
                }) > 11
            {
                diffs.insert(diff);
            }
        }
    }
    diffs
}

fn field_intersect(l: &PointSet, r: &PointSet, l_diff: isize, field: FieldGetter) -> (PointSet, PointSet) {
    (
        l.iter()
            .filter(|lp| r.iter().any(|rp| field(lp) - l_diff == field(rp)))
            .cloned()
            .collect(),
        r.iter()
            .filter(|rp| l.iter().any(|lp| field(lp) - l_diff == field(rp)))
            .cloned()
            .collect()
    )
}