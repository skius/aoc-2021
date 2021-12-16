use std::io::{BufReader, Read};
use std::time::Instant;

#[derive(Debug)]
enum Content {
    Literal(u64),
    Packets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    content: Content,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        match self.content {
            Content::Literal(_) => self.version as usize,
            Content::Packets(ref packets) => (self.version as usize) + packets.iter().map(|p| p.sum_versions()).sum::<usize>(),
        }
    }

    fn eval(&self) -> u64 {
        match &self.content {
            Content::Literal(literal) => *literal,
            Content::Packets(packets) => {
                match self.type_id {
                    0 => packets.iter().map(|p| p.eval()).sum(),
                    1 => packets.iter().map(|p| p.eval()).product(),
                    2 => packets.iter().map(|p| p.eval()).min().unwrap(),
                    3 => packets.iter().map(|p| p.eval()).max().unwrap(),
                    5 => {
                        (packets[0].eval() > packets[1].eval()) as u64
                    },
                    6 => {
                        (packets[0].eval() < packets[1].eval()) as u64
                    },
                    7 => {
                        (packets[0].eval() == packets[1].eval()) as u64
                    },
                    _ => panic!("Unknown type_id: {}", self.type_id),
                }
            }
        }
    }
}

fn bits_to_num(bits: &[u8]) -> u64 {
    let mut num = 0;
    for bit in bits {
        num = num << 1;
        num += *bit as u64;
    }
    num
}

fn parse_literal(mut bits: &[u8]) -> (u64, &[u8]) {
    let mut num = 0;

    while bits[0] == 1 {
        num = num << 4;
        num += bits_to_num(&bits[1..5]) as u64;
        bits = &bits[5..];
    }

    num = num << 4;
    num += bits_to_num(&bits[1..5]) as u64;
    bits = &bits[5..];

    // for chunk in bits.chunks(5) {
    //     num << 4;
    //     num += bits_to_num(&chunk[1..]);
    //
    //     if chunk[0] == 0 {
    //         return num;
    //     }
    // }

    (num, bits)

    // panic!("Invalid literal");
    // num
}

// returns packet and remaining bits
fn parse_packet(mut bits: &[u8]) -> (Packet, &[u8]) {
    let version = bits_to_num(&bits[0..3]) as u8;
    bits = &bits[3..];
    // println!("Parsing packet version {}", version);
    let type_id = bits_to_num(&bits[0..3]) as u8;
    bits = &bits[3..];
    let content = if type_id == 4 {
        let (lit, rem) = parse_literal(bits);
        bits = rem;
        Content::Literal(lit)
    } else {
        let length_type_id = bits[0];
        bits = &bits[1..];
        if length_type_id == 0 {
            let mut total_subpacket_length = bits_to_num(&bits[0..15]) as usize;
            bits = &bits[15..];
            let mut subpacket_bits = &bits[0..total_subpacket_length];
            bits = &bits[total_subpacket_length..];
            let mut packets = vec![];
            while subpacket_bits.len() > 0 {
                let (packet, remaining_bits) = parse_packet(subpacket_bits);
                packets.push(packet);
                subpacket_bits = remaining_bits;
            }

            Content::Packets(packets)
        } else {
            let total_subpacket_nums = bits_to_num(&bits[0..11]);
            // println!("Parsing packet with {} subpackets (length type id 1)", total_subpacket_nums);
            bits = &bits[11..];
            let mut subpacket_bits = &bits[..];
            let mut packets = vec![];
            while packets.len() < total_subpacket_nums as usize {
                let (packet, remaining_bits) = parse_packet(subpacket_bits);
                packets.push(packet);
                subpacket_bits = remaining_bits;
            }
            bits = subpacket_bits;

            Content::Packets(packets)
        }
    };

    (
        Packet {
            version,
            type_id,
            content,
        },
        bits
    )
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut res = String::new();
    buf.read_to_string(&mut res);

    let mut bits = vec![];

    for c in res.chars() {
        bits.extend(format!("{:04b}", c.to_digit(16).unwrap())
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8));
    }

    println!("{:?}", bits);
    println!("{:?}", bits.len());

    let (packet, rem) = parse_packet(&bits);
    // println!("{:#?}", packet);
    // println!("{:?}", rem);

    packet.sum_versions().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buf = BufReader::new(input);
    let mut res = Vec::new();
    buf.read_to_end(&mut res);

    let mut bits = vec![];

    let pre = Instant::now();

    for b in res {
        match b {
            b'0'..=b'9' => {
                let val = b - b'0';
                for i in (0..4).rev() {
                    bits.push(((val & (1 << i)) != 0) as u8);
                    // bits.push(((val & (1 << i)) == 1));
                }

            },
            b'A'..=b'F' => {
                let val = b - b'A' + 10;
                for i in (0..4).rev() {
                    bits.push(((val & (1 << i)) != 0) as u8);
                    // bits.push(((val & (1 << i)) == 1));
                }
            },
            _ => panic!(),
        }
        // bits.extend(format!("{:04b}", c.to_digit(16).unwrap())
        //     .chars()
        //     // .map(|c| c.to_digit(10).unwrap() as u8));
        //     .map(|c| c == '1'));
    }

    // println!("{:?}", pre.elapsed());

    // println!("{:?}", bits);
    // println!("{:?}", bits.len());

    let (packet, rem) = parse_packet(&bits);
    // println!("{:#?}", packet);
    // println!("{:?}", rem);

    packet.eval().to_string()
    // String::new()
}


#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/16.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/16.2.txt");
    const SAMPLE3: &[u8] = include_bytes!("samples/16.3.txt");
    const SAMPLE4: &[u8] = include_bytes!("samples/16.4.txt");
    const REAL: &[u8] = include_bytes!("../inputs/16.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 16);
    }

    #[test]
    fn sample2_part1() {
        test_implementation(part1, SAMPLE2, 12);
    }

    #[test]
    fn sample3_part1() {
        test_implementation(part1, SAMPLE3, 23);
    }

    #[test]
    fn sample4_part1() {
        test_implementation(part1, SAMPLE4, 31);
    }

    #[test]
    fn real_part1() {
        test_implementation(part1, REAL, 879);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 315);
    }

    #[test]
    fn real_part2() {
        test_implementation(part2, REAL, 539051801941);
    }
}