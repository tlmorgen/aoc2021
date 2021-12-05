pub trait Day {
    fn load(&mut self, content: &str);
    fn part1(&mut self) -> isize;
    fn part2(&mut self) -> isize;
}