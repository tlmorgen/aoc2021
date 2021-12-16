use super::super::day::Day;
use bitvec::prelude::*;
use bitvec::ptr::Const;
use tap::conv::Conv;

type BitWord = u8;
type BitOwner = BitVec<Msb0, BitWord>;

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
        let mut iter = self.bits.iter();
        let packet = Packet::from_iter(&mut iter);
        packet.execute()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq
}

impl From<isize> for PacketType {
    fn from(type_num: isize) -> Self {
        match type_num {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Literal,
            5 => PacketType::Gt,
            6 => PacketType::Lt,
            7 => PacketType::Eq,
            _ => panic!("Not a packet type: {}", type_num)
        }
    }
}

#[derive(Debug)]
struct PacketHeader {
    version: isize,
    packet_type: PacketType
}

impl PacketHeader {
    fn from_iter(bits: &mut dyn Iterator<Item=BitRef<Const, Msb0, BitWord>>) -> PacketHeader {
        let version = collect_num(&mut bits.take(3));
        let type_num = collect_num(&mut bits.take(3));

        PacketHeader {
            version,
            packet_type: type_num.conv()
        }
    }
}

#[derive(Debug)]
struct Packet {
    header: PacketHeader,
    literal: Option<isize>,
    children: Vec<Packet>
}

impl Packet {
    fn from_iter(iter: &mut dyn Iterator<Item=BitRef<Const, Msb0, BitWord>>) -> Packet {
        let header = PacketHeader::from_iter(iter);
        let mut literal = None;
        let mut children: Vec<Packet> = Vec::new();
        match header.packet_type {
            PacketType::Literal => {
                literal = Some(take_literal(iter));
            }
            _ => {
                if *iter.next().unwrap() {
                    // 11 bits describing number of packets
                    let num_packets = collect_num(&mut iter.take(11));
                    for _ in 0..num_packets {
                        children.push(Packet::from_iter(iter))
                    }
                } else {
                    // 15 bits describing size of packets
                    let packets_size = collect_num(&mut iter.take(15));
                    let mut next_iter = iter.take(packets_size as usize).peekable();
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

    fn execute(&self) -> isize {
        match self.header.packet_type {
            PacketType::Literal => {self.literal.unwrap()}
            PacketType::Sum => {
                self.children.iter().fold(0isize, |sum, child| {
                    sum + child.execute()
                })
            }
            PacketType::Product => {
                self.children.iter().fold(1isize, |product, child| {
                    product * child.execute()
                })
            }
            PacketType::Min => {
                self.children.iter().map(Packet::execute).min().unwrap()
            }
            PacketType::Max => {
                self.children.iter().map(Packet::execute).max().unwrap()
            }
            PacketType::Gt => {
                let values: Vec<isize> = self.children.iter().map(Packet::execute).collect();
                (values[0] > values[1]) as isize
            }
            PacketType::Lt => {
                let values: Vec<isize> = self.children.iter().map(Packet::execute).collect();
                (values[0] < values[1]) as isize
            }
            PacketType::Eq => {
                let values: Vec<isize> = self.children.iter().map(Packet::execute).collect();
                (values[0] == values[1]) as isize
            }
        }
    }
}

fn collect_num(iter: &mut dyn Iterator<Item=BitRef<Const, Msb0, BitWord>>) -> isize {
    iter.fold(0isize, |num, bit| (num << 1) | (*bit as isize))
}

fn take_literal(iter: &mut dyn Iterator<Item=BitRef<Const, Msb0, BitWord>>) -> isize {
    let mut lit_bits = BitOwner::new();
    loop {
        let cont: bool = *iter.next().unwrap();
        lit_bits.extend(iter.take(4));
        if !cont {break}
    }
    collect_num(&mut lit_bits.iter())
}

fn add_versions(packet: &Packet) -> isize {
    packet.children.iter()
        .fold(packet.header.version, |sum, packet| sum + add_versions(&packet))
}

