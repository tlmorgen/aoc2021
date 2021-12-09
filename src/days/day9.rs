use super::super::day::Day;

pub struct Day9 {
    depths: Vec<Vec<usize>>,
    width: usize
}

impl Day9 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let rows: Vec<Vec<usize>> = content.lines()
            .map(|line| line.trim().chars()
                .map(|c| c as usize - 48)
                .collect())
            .collect();
        let min_width = rows.iter().map(|row| row.len()).min().unwrap();
        let max_width = rows.iter().map(|row| row.len()).max().unwrap();
        assert_eq!(min_width, max_width);
        Ok(Box::new(Day9 {
            depths: rows,
            width: max_width
        }))
    }
}

impl Day for Day9 {
    fn part1(&mut self) -> isize {
        let mut sum = 0isize;
        for row in 0..self.depths.len() {
            for col in 0..self.width {
                let target = self.depths[row][col];
                let mut neighbors: Vec<usize> = Vec::new();
                if row > 0 {
                    neighbors.push(self.depths[row - 1][col]); // up
                }
                if col > 0 {
                    neighbors.push(self.depths[row][col - 1]); // left
                }
                if col + 1 < self.width {
                    neighbors.push(self.depths[row][col + 1]); // right
                }
                if row + 1 < self.depths.len() {
                    neighbors.push(self.depths[row + 1][col]); // down
                }
                if target < *neighbors.iter().min().unwrap() {
                    sum += (1 + target) as isize;
                }
            }
        }
        sum
    }

    fn part2(&mut self) -> isize {
        0
    }
}
