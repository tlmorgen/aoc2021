use super::super::day::Day;

const WIN_SZ: usize = 3;

pub struct Day1 {
    depths: Vec<isize>
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            depths: Vec::new()
        }
    }
}

impl Day for Day1 {
    fn load(&mut self, content: &str) {
        self.depths = content
            .split_whitespace()
            .map(|s| s.parse::<isize>().expect("not a num"))
            .collect();
    }

    fn part1(&mut self) -> isize {
        let mut incrs = 0;
        let mut last_depth = -1;
        
        for (i, depth) in self.depths.iter().enumerate() {
            if i > 0 && *depth > last_depth {
                incrs += 1;
            }
            last_depth = *depth;
        }

        incrs
    }

    fn part2(&mut self) -> isize {
        let mut incrs = 0;
        let mut last_window: isize = -1;

        for i in 0..(self.depths.len() - WIN_SZ + 1) {
            let curr_window = self.depths[i..i+WIN_SZ].iter().sum();
            if i > 0 && last_window < curr_window {
                incrs += 1;
            }
            last_window = curr_window;
        }

        incrs
    }
}