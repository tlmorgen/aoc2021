use super::super::day::Day;
use array2d::Array2D;

pub struct Day3 {
    values: Array2D<bool>
}

impl Day3 {
    pub fn from_content(content: &str) -> Box<dyn Day> {
        let vecs: Vec<Vec<bool>> = content
            .split_whitespace()
            .map(|word| word
                .chars()
                .map(|char| {
                    match char {
                        '0' => false,
                        '1' => true,
                        _ => panic!("not binary")
                    }})
                .collect())
            .collect();

        Box::new(Day3 {
            values: Array2D::from_rows(&vecs)
        })
    }
}

impl Day for Day3 {

    fn part1(&mut self) -> isize {
        let counts = counts(&self.values);
    
        let gamma: usize = counts.iter()
            .fold(0, |gamma, &count| {
                (gamma << 1) + if count > self.values.column_len() / 2 {1} else {0}
            });
        let epsilon: usize = counts.iter()
            .fold(0, |epsilon, &count| {
                (epsilon << 1) + if count > self.values.column_len() / 2 {0} else {1}
            });
    
        (gamma * epsilon).try_into().unwrap()
    }

    fn part2(&mut self) -> isize {
        let ox = filter(&self.values, true).expect("unable to find O2");
        let co2 = filter(&self.values, false).expect("unable to find CO2");

        (ox * co2).try_into().unwrap()
    }
}


fn counts(vals: &Array2D<bool>) -> Vec<usize> {
    vals.columns_iter()
        .map(|cell_iter| cell_iter.map(|b| *b as usize).sum())
        .collect()
}

fn filter(vals: &Array2D<bool>, popular: bool) -> Result<usize, &'static str> {

    let mut matches = vals.as_rows();
    for pos in 0..vals.row_len() {
        let amatches = Array2D::from_rows(&matches);
        let counts = counts(&amatches);
        let tval = if counts[pos] >= (amatches.column_len() - counts[pos]) {popular} else {!popular};
        matches = amatches
            .as_rows()
            .into_iter()
            .filter(|val| val[pos] == tval)
            .collect();
        if matches.len() < 2 {
            break;
        }
    }

    if matches.len() != 1 {
        Err("no matching word")
    } else {
        let dec = matches[0].iter()
            .fold(0, |dec, &bit| {
                (dec << 1) + bit as usize
            });
        Ok(dec)
    }
}