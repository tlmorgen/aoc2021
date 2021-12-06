pub trait Day {
    fn part1(&mut self) -> isize;
    fn part2(&mut self) -> isize;
}

pub type DayMaker = fn (content: &str) -> Result<Box<dyn Day>, &'static str>;