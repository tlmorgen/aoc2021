use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use rayon::prelude::*;

use super::super::day::Day;

pub struct Day23 {
    burrow: Burrow2,
}

impl Day23 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day23 {
            burrow: Burrow2::from_content(content)
        }))
    }
}

impl Day for Day23 {
    fn part1(&mut self) -> isize {
        try_all_moves(self.burrow.clone(), &mut HashMap::new(), &mut BTreeSet::new()).unwrap() as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

fn min_opt(l: Option<usize>, r: Option<usize>) -> Option<usize> {
    match l {
        None => {
            match r {
                None => None,
                Some(rv) => Some(rv)
            }
        }
        Some(lv) => {
            match r {
                None => Some(lv),
                Some(rv) => {
                    Some(min(lv, rv))
                }
            }
        }
    }
}

fn try_all_moves(burrow: Burrow2, memo: &mut HashMap<Burrow2, Option<usize>>, solutions: &mut BTreeSet<usize>) -> Option<usize> {
    // eprintln!("try\n{}", burrow.display());
    match memo.get(&burrow) {
        Some(min_energy) => *min_energy,
        None => {
            if burrow.is_solved() {
                let cost = burrow.cost;
                eprintln!("solution: {}", cost);
                memo.insert(burrow, Some(cost));
                solutions.insert(cost);
                Some(cost)
            } else {
                let mut min_energy: Option<usize> = None; // all moves could fail

                for next_burrow in (0..burrow.rooms.len()).map(|room_idx| {
                    (0..burrow.hallway.len()).map(move |hallway_idx| (room_idx, hallway_idx))
                })
                    .flatten()
                    .collect::<Vec<(usize, usize)>>()
                    .par_iter() // thread the valid move calculations
                    .flat_map(|(room_idx, hallway_idx)| vec![
                        burrow.move_room_to_hallway(*room_idx, *hallway_idx),
                        burrow.move_hallway_to_room(*hallway_idx, *room_idx),
                    ])
                    .filter_map(|opt| opt)
                    .collect::<Vec<Burrow2>>().into_iter() // un-thread
                {
                    if solutions.is_empty() || next_burrow.cost < *solutions.first().unwrap() {
                        min_energy = min_opt(min_energy, try_all_moves(next_burrow, memo, solutions));
                    }
                }

                memo.insert(burrow, min_energy);
                min_energy
            }
        }
    }
}

type Slots2 = Vec<usize>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Room2 {
    location: usize,
    slots: Slots2,
}

impl Room2 {
    fn new(location: usize) -> Self {
        Self {
            location,
            slots: Slots2::new(),
        }
    }
    fn accepts(&self, pod: usize) -> bool {
        self.slots.iter().all(|my_pod| *my_pod == 0 || *my_pod == pod) &&
            *self.slots.last().unwrap() == 0
    }
    fn is_solved(&self) -> bool {
        self.slots.iter().sum::<usize>() != 0 &&
            *self.slots.iter()
                .filter(|pod| **pod != 0)
                .min()
                .unwrap()
                == *self.slots.iter()
                .filter(|pod| **pod != 0)
                .max()
                .unwrap()
    }

    fn can_remove(&self, target_pod: usize) -> bool {
        self.slots.iter().any(|pod| *pod != 0 && *pod != target_pod)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Burrow2 {
    hallway: Slots2,
    rooms: Vec<Room2>,
    cost: usize,
}

impl Burrow2 {
    fn from_content(content: &str) -> Self {
        let mut lines = content.lines();
        lines.next(); // header
        let hallway_len = lines.next().unwrap().chars().filter(|c| *c == '.').count();

        let mut rooms: Vec<Room2> = Vec::new();
        for room_line in lines {
            let mut room_i = 0;
            for (i, c) in room_line.chars().enumerate() {
                let pod = match c {
                    'A' => 1,
                    'B' => 10,
                    'C' => 100,
                    'D' => 1000,
                    _ => continue
                };
                if room_i >= rooms.len() {
                    rooms.push(Room2::new(i - 1));
                }
                rooms[room_i].slots.push(pod);
                room_i += 1;
            }
        }

        rooms.iter_mut().for_each(|room| room.slots.reverse());

        Self {
            hallway: vec![0; hallway_len],
            rooms,
            cost: 0,
        }
    }

    fn is_solved(&self) -> bool {
        for room in &self.rooms {
            if !room.is_solved() {
                return false;
            }
        }
        return self.hallway.iter().sum::<usize>() == 0;
    }

    fn move_hallway_to_room(&self, hallway_idx: usize, room_idx: usize) -> Option<Self> {
        let pod = self.hallway[hallway_idx];
        if pod == 0 { return None; }
        if pod.log10() != (room_idx as u32) { return None; }
        let room = &self.rooms[room_idx];
        if !room.accepts(pod) { return None; }
        let (l, r) = if room.location < hallway_idx {
            min_max(room.location, hallway_idx - 1)
        } else {
            min_max(room.location, hallway_idx + 1)
        };
        if self.hallway[l..=r].iter().any(|entry| *entry != 0) { return None; }

        let mut hallway = self.hallway.clone();
        let mut rooms = self.rooms.clone();
        hallway[hallway_idx] = 0;
        let mut room_dist = 0;
        for (room_idx, slot) in rooms[room_idx].slots.iter_mut().enumerate() {
            if *slot == 0 {
                *slot = pod;
                room_dist = room.slots.len() - room_idx;
                break;
            }
        }
        let move_cost = (r - l + 1 + room_dist) * pod;
        Some(Self {
            hallway,
            rooms,
            cost: self.cost + move_cost,
        })
    }

    fn move_room_to_hallway(&self, room_idx: usize, hallway_idx: usize) -> Option<Self> {
        if self.rooms.iter().any(|room| room.location == hallway_idx) { return None; }
        let room = &self.rooms[room_idx];
        if !room.can_remove(10usize.pow(room_idx as u32)) { return None; } // TODO too aggressive an optimization?
        if room.slots.iter().sum::<usize>() == 0 { return None; }
        if room.location == hallway_idx { return None; }
        let (l, r) = min_max(room.location, hallway_idx);
        if self.hallway[l..=r].iter().any(|entry| *entry != 0) { return None; }

        let mut hallway = self.hallway.clone();
        let mut rooms = self.rooms.clone();
        let mut room_dist = 0;
        let mut pod = 0;
        for (i, room_slot) in rooms[room_idx].slots.iter_mut().enumerate().rev() {
            if *room_slot != 0 {
                pod = *room_slot;
                *room_slot = 0;
                room_dist = room.slots.len() - i;
                break;
            }
        }
        hallway[hallway_idx] = pod;
        let move_cost = (r - l + room_dist) * pod;
        Some(Self {
            hallway,
            rooms,
            cost: self.cost + move_cost,
        })
    }

    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(self.hallway.iter()
            .map(|entry| {
                match entry {
                    0 => '.',
                    1 => 'A',
                    10 => 'B',
                    100 => 'C',
                    1000 => 'D',
                    _ => panic!("bad pod")
                }
            }).join("")
        );
        let max_room_len = self.rooms.iter()
            .map(|room| room.slots.len())
            .max()
            .unwrap();
        (0..max_room_len).map(|room_idx| {
            (0..self.hallway.len()).map(|hallway_idx| {
                match self.rooms.iter().find(|room| room.location == hallway_idx) {
                    None => ' ',
                    Some(room) => match room.slots[room_idx] {
                        0 => '.',
                        1 => 'A',
                        10 => 'B',
                        100 => 'C',
                        1000 => 'D',
                        _ => panic!("bad pod")
                    }
                }
            }).join("")
        }).rev().for_each(|line| lines.push(line));

        lines.join("\n")
    }
}

fn min_max<T: Ord + Copy>(l: T, r: T) -> (T, T) {
    (l.min(r), l.max(r))
}