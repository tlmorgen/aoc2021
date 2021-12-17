use bitvec::field::BitField;
use super::super::day::Day;
use bitvec::prelude::*;
use tap::conv::Conv;

type BitWord = u8;
type BitOwner = BitVec<Msb0, BitWord>;
type Bits = BitSlice<Msb0, BitWord>;

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
        let mut bits = BitSliceScanner::from_slice(self.bits.as_bitslice());
        let packet = Packet::from_bits(&mut bits);
        packet.add_versions() as isize
    }

    fn part2(&mut self) -> isize {
        let mut bits = BitSliceScanner::from_slice(self.bits.as_bitslice());
        let packet = Packet::from_bits(&mut bits);
        packet.execute()
    }
}

struct BitSliceScanner<'a> {
    bits: &'a Bits,
    curr_pos: usize
}

impl<'a> BitSliceScanner<'a> {
    fn from_slice(bits: &Bits) -> BitSliceScanner {
        BitSliceScanner {
            bits,
            curr_pos: 0
        }
    }
    fn take_slice(&mut self, len: usize) -> &Bits {
        let start = self.curr_pos;
        let end = start + len;
        self.curr_pos = end;
        &self.bits[start..end]
    }
    fn has_more(&self) -> bool {
        self.curr_pos < self.bits.len()
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

impl From<usize> for PacketType {
    fn from(type_num: usize) -> Self {
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
    version: usize,
    packet_type: PacketType
}

impl PacketHeader {
    fn from_bits(bits: &mut BitSliceScanner) -> PacketHeader {
        PacketHeader {
            version: bits.take_slice(3).load_be(),
            packet_type: bits.take_slice(3).load_be::<usize>().conv()
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
    fn from_bits(bits: &mut BitSliceScanner) -> Packet {
        let header = PacketHeader::from_bits(bits);
        let mut literal = None;
        let mut children: Vec<Packet> = Vec::new();
        match header.packet_type {
            PacketType::Literal => {
                literal = Some(take_literal(bits));
            }
            _ => {
                if *bits.take_slice(1).first().unwrap() {
                    let num_packets: usize = bits.take_slice(11).load_be();
                    for _ in 0..num_packets {
                        children.push(Packet::from_bits(bits))
                    }
                } else {
                    let packets_size: usize = bits.take_slice(15).load_be();
                    let mut next_bits = BitSliceScanner::from_slice(bits.take_slice(packets_size));
                    while next_bits.has_more() {
                        children.push(Packet::from_bits(&mut next_bits));
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

    fn add_versions(&self) -> usize {
        self.children.iter().fold(self.header.version as usize,
          |sum, packet| sum + packet.add_versions())
    }

    fn execute(&self) -> isize {
        match self.header.packet_type {
            PacketType::Literal => {
                self.literal.unwrap()
            }
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
                assert_eq!(self.children.len(), 2);
                let values: Vec<isize> = self.children.iter().map(Packet::execute).collect();
                (values[0] > values[1]) as isize
            }
            PacketType::Lt => {
                assert_eq!(self.children.len(), 2);
                let values: Vec<isize> = self.children.iter().map(Packet::execute).collect();
                (values[0] < values[1]) as isize
            }
            PacketType::Eq => {
                assert_eq!(self.children.len(), 2);
                let values: Vec<isize> = self.children.iter().map(Packet::execute).collect();
                (values[0] == values[1]) as isize
            }
        }
    }
}

fn take_literal(bits: &mut BitSliceScanner) -> isize {
    let mut lit_bits = BitOwner::new();
    loop {
        let chunk = bits.take_slice(5);
        let cont = *chunk.first().unwrap();
        lit_bits.extend_from_bitslice(&chunk[1..]);
        if !cont {break}
    }
    lit_bits.load_be::<usize>() as isize
}

