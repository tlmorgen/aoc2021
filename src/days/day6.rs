use super::super::day::Day;

pub struct Day6 {
    fish_states: Vec<usize>,
    days: usize
}

impl Day6 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day6 {
            fish_states: content.split(',')
                .filter_map(|n| n.parse().ok())
                .collect(),
            days: 80
        }))
    }
}

impl Day for Day6 {
    fn part1(&mut self) -> isize {
        eprintln!("{} {:?}", self.fish_states.len(), self.fish_states);
        for _ in 0..self.days {
            let mut new_fish: Vec<usize> = Vec::new();
            for fish in self.fish_states.iter_mut() {
                if *fish > 0 {
                    *fish -= 1;
                } else {
                    *fish = 6;
                    new_fish.push(8);
                }
            }
            self.fish_states.append(&mut new_fish);
        }
        self.fish_states.len() as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}