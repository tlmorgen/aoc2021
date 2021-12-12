use super::super::day::Day;
use itertools::Itertools;
use petgraph::prelude::*;
use std::collections::HashMap;

const START_ID: &'static str = "start";
const END_ID: &'static str = "end";

pub struct Day12 {
    content: String
}

impl Day12 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day12 {
            content: String::from(content)
        }))
    }

    fn build_graph(&self) -> UnGraphMap<&str, ()> {
        self.content.split_whitespace()
            .flat_map(|word| word.split('-'))
            .tuples::<(_, _)>()
            .collect()
    }
}

impl Day for Day12 {
    fn part1(&mut self) -> isize {
        let graph = self.build_graph();
        all_paths_start(&graph, is_visitable_small_caves_once) as isize
    }

    fn part2(&mut self) -> isize {
        let graph = self.build_graph();
        all_paths_start(&graph, is_visitable_small_caves_once_except_one_small_cave_twice) as isize
    }
}

fn all_paths_start(
    graph: &UnGraphMap<&str, ()>,
    is_visitable: fn(id: &str, &HashMap<&str, usize>) -> bool
) -> usize {
    all_paths(graph, START_ID, HashMap::new(), is_visitable)
}


fn all_paths<'a, 'b: 'a>(
    graph: &UnGraphMap<&'b str, ()>,
    node: &'a str,
    mut visits: HashMap<&'a str, usize>,
    is_visitable: fn(id: &str, &HashMap<&str, usize>) -> bool
) -> usize {
    *visits.entry(node).or_insert(0) += 1;
    graph.neighbors(node).fold(0usize, |hits, next_node| {
        hits + if next_node == END_ID {
            1
        } else if is_visitable(next_node, &visits) {
            all_paths(&graph, next_node, visits.clone(), is_visitable)
        } else {
            0
        }
    })
}

fn is_visitable_small_caves_once(id: &str, visits: &HashMap<&str, usize>) -> bool {
    if id == START_ID || id == END_ID {
        false
    } else if is_big_cave(id) {
        true
    } else if is_small_cave(id) && *visits.get(id).unwrap_or(&0) < 1 {
        true
    } else {
        false
    }
}

fn is_visitable_small_caves_once_except_one_small_cave_twice(id: &str, visits: &HashMap<&str, usize>) -> bool {
    if id == START_ID || id == END_ID {
        false
    } else if is_big_cave(id) {
        true
    } else if is_small_cave(id) {
        let node_visits = *visits.get(id).unwrap_or(&0);
        if node_visits < 1 {
            true
        } else if node_visits == 1 && visits.iter()
                .filter(|(check_id, _)| is_small_cave(check_id) && *id != ***check_id)
                .map(|(_, val)| val)
                .all(|v| *v < 2) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn is_small_cave(id: &str) -> bool {
    let first_char = id.chars().next().unwrap();
    first_char >= 'a' && first_char <= 'z'
}

fn is_big_cave(id: &str) -> bool {
    let first_char = id.chars().next().unwrap();
    first_char >= 'A' && first_char <= 'Z'
}