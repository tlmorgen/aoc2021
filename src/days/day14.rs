use super::super::day::Day;
use std::collections::HashMap;
use itertools::Itertools;

type Pair = (char, char);

pub struct Day14 {
    base: PolymerHist,
    insertions: HashMap<Pair, char>
}

impl Day14 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let mut words = content.split_whitespace();
        let base = PolymerHist::from_str(words.next().unwrap());
        let insertions: HashMap<Pair, char> = words.tuples()
            .map(|(pair_s, _, ins_s)| (
                (pair_s.chars().tuples::<(_, _)>().next().unwrap()),
                ins_s.chars().next().unwrap()
            ))
            .collect();
        Ok(Box::new(Day14 {
            base,
            insertions
        }))
    }
}

impl Day for Day14 {
    fn part1(&mut self) -> isize {
        (0..10).into_iter()
            .fold(self.base.clone(), |hist, _| hist.mutate(&self.insertions))
            .min_max_count_diff() as isize
    }

    fn part2(&mut self) -> isize {
        (0..40).into_iter()
            .fold(self.base.clone(), |hist, _| hist.mutate(&self.insertions))
            .min_max_count_diff() as isize
    }
}

#[derive(Clone, Debug)]
struct PolymerHist {
    hist: HashMap<Pair, usize>,
    last_char: char
}

impl PolymerHist {
    fn from_str(word: &str) -> PolymerHist {
        let mut hist: HashMap<Pair, usize> = HashMap::new();
        let chars: Vec<char> = word.chars().collect();
        chars.windows(2)
            .map(|pair| (pair[0], pair[1]))
            .for_each(|pair| *hist.entry(pair).or_insert(0) += 1 );

        PolymerHist {
            hist,
            last_char: chars[chars.len() - 1]
        }
    }

    fn mutate(&self, insertions: &HashMap<Pair, char>) -> PolymerHist {
        let mut hist: HashMap<Pair, usize> = HashMap::new();
        for (pair, count) in self.hist.iter() {
            match insertions.get(&pair) {
                None => {
                    *hist.entry(*pair).or_insert(0) += count;
                }
                Some(insert) => {
                    *hist.entry((pair.0, *insert)).or_insert(0) += count;
                    *hist.entry((*insert, pair.1)).or_insert(0) += count;
                }
            }
        }
        PolymerHist {
            hist,
            last_char: self.last_char
        }
    }

    fn min_max_count_diff(&self) -> usize {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for (pair, count) in self.hist.iter() {
            *counts.entry(pair.0).or_insert(0) += count;
        }
        *counts.entry(self.last_char).or_insert(0) += 1;
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        max - min
    }
}
