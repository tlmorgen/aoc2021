use super::super::day::Day;
use array2d::Array2D;
use petgraph::algo::dijkstra;
use petgraph::prelude::DiGraphMap;

type NodeId = (usize, usize);

pub struct Day15 {
    risks: Array2D<usize>
}

impl Day15 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        let row_major: Vec<Vec<usize>> = content.lines()
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.chars()
                    .map(|c| c as usize - '0' as usize)
                    .collect()
            })
            .collect();
        Ok(Box::new(Day15 {
            risks: Array2D::from_rows(&row_major)
        }))
    }
}

impl Day for Day15 {
    fn part1(&mut self) -> isize {
        cheapest_path(&self.risks) as isize
    }

    fn part2(&mut self) -> isize {
        let risks = expand(self.risks.clone(), 5);
        cheapest_path(&risks) as isize
    }
}

fn cheapest_path(grid: &Array2D<usize>) -> usize {
    let start = (0,0);
    let end = (grid.row_len() - 1, grid.column_len() - 1);
    let graph = to_graph(grid.clone());
    let costs = dijkstra(&graph, start, Some(end), |edge| *edge.2);
    *costs.get(&end).unwrap()
}

fn wrap(mut num: usize) -> usize {
    while num > 9 {
        num -= 9;
    }
    num
}

fn expand(risks: Array2D<usize>, times: usize) -> Array2D<usize> {
    let orig_row_num = risks.column_len();
    let orig_col_num = risks.row_len();
    let mut new_risks = Array2D::filled_with(10, orig_row_num * times, orig_col_num * times);

    // copy original data
    for i in 0..orig_row_num {
        for j in 0..orig_col_num {
            new_risks[(i, j)] = risks[(i, j)];
        }
    }

    // expand original rows to all columns
    for col_time in 1..times {
        for i in 0..orig_row_num {
            let j_orig_start = (col_time - 1) * orig_col_num;
            let j_orig_end = j_orig_start + orig_col_num;
            for j_orig in j_orig_start..j_orig_end {
                let j_target = j_orig + orig_col_num;
                new_risks[(i, j_target)] = wrap(new_risks[(i, j_orig)] + 1);
            }
        }
    }

    // expand expanded rows to all rows
    for row_time in 1..times {
        let i_orig_start = (row_time - 1) * orig_row_num;
        let i_orig_end = i_orig_start + orig_row_num;
        for i_orig in i_orig_start..i_orig_end {
            let i_target = i_orig + orig_row_num;
            for j in 0..new_risks.row_len() {
                new_risks[(i_target, j)] = wrap(new_risks[(i_orig, j)] + 1);
            }
        }
    }

    new_risks
}

fn to_graph(grid: Array2D<usize>) -> DiGraphMap<NodeId, usize> {
    let row_num = grid.column_len();
    let col_num = grid.row_len();
    let valid_loc= |(i, j)| i > -1 && j > -1 && i < (row_num as isize) && j < (col_num as isize);

    let mut graph: DiGraphMap<NodeId, usize> = DiGraphMap::new();
    for i in 0..row_num {
        for j in 0..col_num {
            let curr = (i, j);
            [
                ((i as isize) - 1, j as isize), // up
                (i as isize, (j as isize) + 1), // right
                ((i as isize) + 1, j as isize), // down
                (i as isize, (j as isize) - 1) // left
            ].into_iter()
                .filter(|loc| valid_loc(*loc))
                .map(|(i, j)| (i as usize, j as usize))
                .for_each(|neighbor| {
                    graph.add_edge(curr, neighbor, grid[neighbor]);
                });
        }
    }

    graph
}