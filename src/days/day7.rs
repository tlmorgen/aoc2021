use super::super::day::Day;
use std::cmp;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Day7 {
    crab_hpos: Vec<usize>,
    linear_costs: HashMap<usize, usize>
}

impl Day7 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day7 {
            crab_hpos: content
                .trim()
                .split(",")
                .filter_map(|n| n.parse().ok())
                .collect(),
            linear_costs: HashMap::new()
        }))
    }
}

impl Day for Day7 {

    fn part1(&mut self) -> isize {
        let min = self.crab_hpos.iter().min().unwrap();
        let max = self.crab_hpos.iter().max().unwrap();
        let mut min_cost = usize::MAX;
        for p in *min..*max {
            let mut cost = 0;
            for crab in &self.crab_hpos {
                cost += (p as isize - *crab as isize).abs();
            }
            min_cost = cmp::min(min_cost, cost as usize);
        }
        min_cost as isize
    }

    fn part2(&mut self) -> isize {
        let crabs = self.crab_hpos.clone();
        let min = self.crab_hpos.iter().min().unwrap();
        let max = self.crab_hpos.iter().max().unwrap();
        let mut min_cost = usize::MAX;
        for p in *min..*max {
            let mut cost: usize = 0;
            for crab in crabs.iter() {
                cost += self.mv_cost((p as isize - *crab as isize).abs() as usize);
            }
            min_cost = cmp::min(min_cost, cost);
        }
        min_cost as isize
    }
}

impl Day7 {
    fn mv_cost(&mut self, mut dist: usize) -> usize {
        match self.linear_costs.entry(dist) {
            Entry::Occupied(entry) => {
                *entry.get()
            }
            Entry::Vacant(slot) => {
                let mut cost = 0;
                while dist > 0 {
                    cost += dist;
                    dist -= 1;
                }
                slot.insert(cost);
                cost
            }
        }
    }
}