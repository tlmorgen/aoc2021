use super::super::day::Day;

pub struct Day7 {
    crab_hpos: Vec<usize>
}

impl Day7 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day7 {
            crab_hpos: content
                .trim()
                .split(",")
                .filter_map(|n| n.parse().ok())
                .collect()
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
            min_cost = std::cmp::min(min_cost, cost as usize);
        }
        min_cost as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}