use std::cmp::{max, min};
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
use rayon::prelude::*;

use super::super::day::Day;

pub struct Day23 {
    burrow: Burrow,
}

impl Day23 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day23 {
            burrow: Burrow::from_content(content)
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

fn try_all_moves(burrow: Burrow, memo: &mut HashMap<Burrow, Option<usize>>, solutions: &mut BTreeSet<usize>) -> Option<usize> {
    match memo.get(&burrow) {
        Some(min_energy) => *min_energy,
        None => {
            if burrow.is_stable() {
                let cost = burrow.cost;
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
                        burrow.move_hallway_to_room(*hallway_idx, *room_idx)
                    ])
                    .filter_map(|opt| opt)
                    .collect::<Vec<Burrow>>().into_iter() // un-thread
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Amphipod {
    a_type: char,
    moves: usize,
}

impl Amphipod {
    fn new(type_char: char) -> Self {
        Amphipod {
            a_type: type_char,
            moves: 0,
        }
    }
    fn move_pod(&self, distance: usize) -> (Self, usize) {
        let move_cost = distance * match self.a_type {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("unknown amphipod {}", self.a_type)
        };
        (
            Self {
                a_type: self.a_type,
                moves: self.moves + 1,
            },
            move_cost
        )
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Burrow {
    hallway: Vec<Option<Amphipod>>,
    rooms: Vec<Option<Vec<Option<Amphipod>>>>,
    cost: usize
}

impl Burrow {
    fn from_content(content: &str) -> Self {
        let mut lines = content.lines();
        lines.next(); // header
        let hallway_len = lines.next().unwrap().chars().filter(|c| *c == '.').count();
        let hallway: Vec<Option<Amphipod>> = vec![None; hallway_len];
        let mut rooms: Vec<Option<Vec<Option<Amphipod>>>> = vec![None; hallway_len];

        for room_line in lines {
            for (i, c) in room_line.chars().enumerate() {
                if !(c >= 'A' && c <= 'D') { continue; }
                let room_loc = i - 1;
                if rooms[room_loc].is_none() {
                    rooms[room_loc] = Some(Vec::new());
                }
                rooms[room_loc].as_mut().unwrap().push(Some(Amphipod::new(c)));
            }
        }

        rooms.iter_mut()
            .for_each(|room_entry| {
                match room_entry {
                    None => {}
                    Some(room) => room.reverse()
                }
            });

        Burrow {
            hallway,
            rooms,
            cost: 0
        }
    }

    fn is_stable(&self) -> bool {
        self.rooms.iter()
            .all(|room| room.is_none() ||
                (
                    room.as_ref().unwrap().iter().all(|room_slot| room_slot.is_some()) &&
                        room.as_ref().unwrap().iter()
                            .tuple_windows()
                            .all(|(left, right)| left.as_ref().unwrap().a_type == right.as_ref().unwrap().a_type)
                )
            )
    }

    #[allow(unused_must_use)]
    fn move_hallway_to_room(&self, hallway_pos: usize, room_pos: usize) -> Option<Self> {
        let mut hallway = self.hallway.clone();
        let mut rooms = self.rooms.clone();
        match hallway.get_mut(hallway_pos) {
            None => None, // bad idx
            Some(hallway_entry) => {
                match hallway_entry.take() {
                    None => None, // no pod there
                    Some(pod) => {
                        if pod.moves > 1 {
                            None // out of moves
                        } else {
                            match rooms.get_mut(room_pos) {
                                None => None, // bad idx
                                Some(room_entry) => {
                                    match room_entry {
                                        None => None, // no room there
                                        Some(room) => {
                                            if room_pos / 2 != match pod.a_type {
                                                'A' => 1,
                                                'B' => 2,
                                                'C' => 3,
                                                'D' => 4,
                                                _ => panic!("invalid amphipod {}", pod.a_type)
                                            } {
                                                None // wrong pod type for that location
                                            } else if !room.iter().all(|room_slot|
                                                room_slot.is_none() || room_slot.as_ref().unwrap().a_type == pod.a_type
                                            ) {
                                                None // incompatible room
                                            } else {
                                                // all empty or the same type
                                                let room_len = room.len();
                                                match room.iter_mut()
                                                    .enumerate()
                                                    .filter(|(_, room_slot)| room_slot.is_none())
                                                    .min_by(|l, r| l.0.cmp(&r.0))
                                                {
                                                    None => None, // room full
                                                    Some((room_idx, room_slot)) => {
                                                        if room_pos < hallway_pos && hallway[room_pos..hallway_pos].iter()
                                                            .any(|hallway_entry| hallway_entry.is_some())
                                                        {
                                                            None // pods in the way
                                                        } else if room_pos > hallway_pos && hallway[hallway_pos..room_pos].iter()
                                                            .any(|hallway_entry| hallway_entry.is_some())
                                                        {
                                                            None // pods in the way
                                                        } else {
                                                            let hallway_distance = max(hallway_pos, room_pos) - min(hallway_pos, room_pos);
                                                            let room_distance = room_len - room_idx;
                                                            let (moved_pod, move_cost) = pod.move_pod(hallway_distance + room_distance);
                                                            room_slot.insert(moved_pod);
                                                            Some(Burrow { hallway, rooms, cost: self.cost + move_cost })
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(unused_must_use)]
    fn move_room_to_hallway(&self, room_pos: usize, hallway_pos: usize) -> Option<Burrow> {
        let mut hallway = self.hallway.clone();
        let mut rooms = self.rooms.clone();
        if rooms.get(hallway_pos).is_some() && rooms[hallway_pos].is_some() {
            None // can't land outside a door
        } else {
            match rooms.get_mut(room_pos) {
                None => None, // bad idx
                Some(room_entry) => {
                    match room_entry {
                        None => None, // no room there
                        Some(room) => {
                            let room_len = room.len();
                            match room.iter_mut().enumerate()
                                .filter(|(_, room_slot)| room_slot.is_some())
                                .max_by(|l, r| l.0.cmp(&r.0))
                            {
                                None => None, // empty room
                                Some((room_idx, room_slot)) => {
                                    if room_slot.as_ref().unwrap().moves > 1 {
                                        None // out of moves
                                    } else if room_pos < hallway_pos && hallway[room_pos..hallway_pos].iter()
                                        .any(|hallway_entry| hallway_entry.is_some())
                                    {
                                        None // pods in the way
                                    } else if room_pos > hallway_pos && hallway[hallway_pos..room_pos].iter()
                                        .any(|hallway_entry| hallway_entry.is_some())
                                    {
                                        None // pods in the way
                                    } else {
                                        match hallway.get_mut(hallway_pos) {
                                            None => None, // bad idx
                                            Some(hallway_entry) => {
                                                match hallway_entry {
                                                    Some(_) => None, // already occupied
                                                    None => {
                                                        let hallway_distance = max(hallway_pos, room_pos) - min(hallway_pos, room_pos);
                                                        let room_distance = room_len - room_idx;
                                                        let (moved_pod, move_cost) = room_slot.take().unwrap().move_pod(hallway_distance + room_distance);
                                                        hallway_entry.insert(moved_pod);
                                                        Some(Burrow { hallway, rooms, cost: self.cost + move_cost })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(self.hallway.iter()
            .map(|entry| {
                match entry {
                    None => '.',
                    Some(pod) => pod.a_type
                }
            }).join("")
        );
        let max_room_len = self.rooms.iter()
            .map(|room| {
                match room {
                    None => 0,
                    Some(room) => room.len()
                }
            })
            .max()
            .unwrap();
        (0..max_room_len).map(|row| {
            self.rooms.iter().map(|room_entry| {
                match room_entry {
                    None => ' ',
                    Some(room) => {
                        match room.get(row) {
                            None => '.',
                            Some(room_slot) => {
                                match room_slot {
                                    None => '.',
                                    Some(pod) => pod.a_type
                                }
                            }
                        }
                    }
                }
            }).join("")
        }).rev().for_each(|line| lines.push(line));

        lines.join("\n")
    }
}