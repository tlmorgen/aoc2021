use super::super::day::Day;
use bitvec::prelude::*;
use bitvec::ptr::{Const, Mutability};
use bitvec::slice::Iter;
use itertools::Itertools;
use tap::conv::Conv;

type BitOwner = BitVec<Msb0, u8>;
type Bits = BitSlice<Msb0, u8>;

pub struct Day16 {
    bits: BitOwner
}

impl Day16 {
    pub fn from_content(content: &str) -> Result<Box<dyn Day>, &'static str> {
        Ok(Box::new(Day16 {
            bits: hex::decode(content.trim()).unwrap().into_iter().collect()
        }))
    }
}

impl Day for Day16 {
    fn part1(&mut self) -> isize {
        let mut iter = self.bits.iter();
        let packet = Packet::from_iter(&mut iter);
        add_versions(&packet) as isize
    }

    fn part2(&mut self) -> isize {
        0
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PacketType {
    Literal,
    Operation
}

impl PacketType {
    fn from_num(num: usize) -> PacketType {
        if num == 4 {
            PacketType::Literal
        } else {
            PacketType::Operation
        }
    }
}

#[derive(Debug)]
struct PacketHeader {
    version: usize,
    packet_type: PacketType
}

impl PacketHeader {
    fn from_iter(bits: &mut dyn Iterator<Item=BitRef<Const, Msb0, u8>>) -> PacketHeader {
        let version = to_num(&mut bits.take(3));
        let type_num = to_num(&mut bits.take(3));

        PacketHeader {
            version,
            packet_type: PacketType::from_num(type_num)
        }
    }
}

#[derive(Debug)]
struct Packet {
    header: PacketHeader,
    literal: Option<usize>,
    children: Vec<Packet>
}

impl Packet {
    fn from_iter(iter: &mut dyn Iterator<Item=BitRef<Const, Msb0, u8>>) -> Packet {
        let header = PacketHeader::from_iter(iter);
        let mut literal = None;
        let mut children: Vec<Packet> = Vec::new();
        match header.packet_type {
            PacketType::Literal => {
                literal = Some(to_literal(iter));
            }
            PacketType::Operation => {
                if *iter.next().unwrap() {
                    // 11 bits describing number of packets
                    let num_packets = to_num(&mut iter.take(11));
                    for _ in 0..num_packets {
                        children.push(Packet::from_iter(iter))
                    }
                } else {
                    // 15 bits describing size of packets
                    let packets_size = to_num(&mut iter.take(15));
                    let mut next_iter = iter.take(packets_size).peekable();
                    while next_iter.peek().is_some() {
                        children.push(Packet::from_iter(&mut next_iter));
                    }
                }
            }
        }

        Packet {
            header,
            literal,
            children
        }
    }
}

fn to_num(iter: &mut dyn Iterator<Item=BitRef<Const, Msb0, u8>>) -> usize {
    iter.fold(0usize, |num, bit| (num << 1) | (*bit as usize))
}

fn to_literal(iter: &mut dyn Iterator<Item=BitRef<Const, Msb0, u8>>) -> usize {
    let mut lit_bits = BitOwner::new();
    loop {
        let cont: bool = *iter.next().unwrap();
        let mut num_bits: BitOwner = iter.take(4).collect();
        lit_bits.append(&mut num_bits);
        if !cont {break}
    }
    to_num(&mut lit_bits.iter())
}

fn add_versions(packet: &Packet) -> usize {
    packet.children.iter()
        .fold(packet.header.version, |sum, packet| sum + add_versions(&packet))
}

