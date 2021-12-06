use super::super::day::Day;

pub struct Day6 {
    fishes: Vec<usize>
}

impl Day6 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day6 {
            fishes: content.split(',')
                .filter_map(|n| n.parse().ok())
                .collect()
        }))
    }
}

const SZ_STATES: usize = 9;
const STATE_POSTPARTUM: usize = 6;
const STATES_NASCENT: usize = 8;

impl Day for Day6 {

    fn part1(&mut self) -> isize {
        let mut fishes = self.fishes.clone();
        for _ in 0..80 {
            let mut new_fish: Vec<usize> = Vec::new();
            for fish in fishes.iter_mut() {
                if *fish > 0 {
                    *fish -= 1;
                } else {
                    *fish = 6;
                    new_fish.push(8);
                }
            }
            fishes.append(&mut new_fish);
        }
        fishes.len() as isize
    }

    fn part2(&mut self) -> isize {
        let mut fish_states = [0 as usize; SZ_STATES];
        for fish in &self.fishes {
            fish_states[*fish] += 1;
        }

        for _ in 0..256 {
            let mut new_fish_states = [0 as usize; SZ_STATES];
            let births = fish_states[0];
            for state in 1..SZ_STATES {
                new_fish_states[state - 1] = fish_states[state];
            }
            new_fish_states[STATE_POSTPARTUM] += births;
            new_fish_states[STATES_NASCENT] += births;
            fish_states = new_fish_states;
        }

        fish_states.iter().sum::<usize>() as isize
    }
}