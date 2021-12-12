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
// eprintln!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

impl Day for Day12 {
    fn part1(&mut self) -> isize {
        let graph = self.build_graph();
        all_paths_start(&graph) as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

fn all_paths_start(graph: &UnGraphMap<&str, ()>) -> usize {
    all_paths(graph, START_ID, HashMap::new())
}


fn all_paths<'a, 'b: 'a>(graph: &UnGraphMap<&'b str, ()>, node: &'a str, mut visits: HashMap<&'a str, usize>) -> usize {
    *visits.entry(node).or_insert(0) += 1;
    graph.neighbors(node).fold(0usize, |hits, next_node| {
        hits + if next_node == END_ID {
            1
        } else if is_visitable(next_node, *visits.get(&next_node).unwrap_or(&0)) {
            all_paths(&graph, next_node, visits.clone())
        } else {
            0
        }
    })
}

fn is_visitable(id: &str, visits: usize) -> bool {
    if id == START_ID || id == END_ID {
        false
    } else if id.chars().all(|c| c >= 'A' && c <= 'Z') {
        true
    } else if id.chars().all(|c| c >= 'a' && c <= 'z') && visits < 1 {
        true
    } else {
        false
    }
}