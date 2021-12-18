use bitvec::vec::BitVec;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 16);
    let packet = parse(&input);
    let p1 = sum_versions(&packet);
    println!("Part 1: {}", p1);
    let p2 = evaluate(&packet);
    println!("Part 2: {}", p2);
}

fn sum_versions(packet: &Packet) -> usize {
    let sub = match &packet.inner_packet {
        InnerPacket::Literal(_) => 0,
        InnerPacket::Operator(op) => op.sub_packets.iter().map(sum_versions).sum::<usize>(),
    };
    sub + packet.version
}

fn evaluate(packet: &Packet) -> usize {
    match &packet.inner_packet {
        InnerPacket::Literal(l) => l.value,
        InnerPacket::Operator(op) => {
            let mut sub_packet_values = op.sub_packets.iter().map(evaluate);
            match &op.op_type {
                OperationType::Sum => sub_packet_values.sum(),
                OperationType::Product => sub_packet_values.product(),
                OperationType::Min => sub_packet_values.min().unwrap(),
                OperationType::Max => sub_packet_values.max().unwrap(),
                OperationType::GreaterThan => {
                    (sub_packet_values.next().unwrap() > sub_packet_values.next().unwrap()) as usize
                }
                OperationType::LessThan => {
                    (sub_packet_values.next().unwrap() < sub_packet_values.next().unwrap()) as usize
                }
                OperationType::Equal => {
                    (sub_packet_values.next().unwrap() == sub_packet_values.next().unwrap())
                        as usize
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    inner_packet: InnerPacket,
}

impl Packet {
    fn new(version: usize, inner_packet: InnerPacket) -> Packet {
        Packet {
            version,
            inner_packet,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPacket {
    value: usize,
}

impl LiteralPacket {
    fn new(bv: &BitVec, i: &mut usize) -> LiteralPacket {
        let mut value_bv = BitVec::new();
        let mut read_more = true;
        while read_more {
            read_more &= bv[*i];
            *i += 1;
            for _ in 0..4 {
                value_bv.push(bv[*i]);
                *i += 1;
            }
        }
        let mut value_i = 0;
        let value_len = value_bv.len();
        let value = read_number(&value_bv, &mut value_i, value_len);
        LiteralPacket { value }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorPacket {
    op_type: OperationType,
    sub_packets: Vec<Packet>,
}

impl OperatorPacket {
    fn new(bv: &BitVec, type_id: usize, i: &mut usize) -> OperatorPacket {
        let length_type = bv[*i];
        *i += 1;
        let length_type_id = match length_type {
            false => LengthTypeId::LengthOfSub(read_number(bv, i, 15)),
            true => LengthTypeId::NumberOfSub(read_number(bv, i, 11)),
        };
        let mut sub_packets = Vec::new();
        match length_type_id {
            LengthTypeId::LengthOfSub(total_length) => {
                let starting_bit_location = *i;
                while total_length != *i - starting_bit_location {
                    sub_packets.push(parse_packet(bv, i));
                }
            }
            LengthTypeId::NumberOfSub(number) => {
                for _ in 0..number {
                    sub_packets.push(parse_packet(bv, i));
                }
            }
        }
        let op_type = OperationType::from(type_id);
        OperatorPacket {
            op_type,
            sub_packets,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum OperationType {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

impl From<usize> for OperationType {
    fn from(i: usize) -> Self {
        match i {
            0 => OperationType::Sum,
            1 => OperationType::Product,
            2 => OperationType::Min,
            3 => OperationType::Max,
            5 => OperationType::GreaterThan,
            6 => OperationType::LessThan,
            7 => OperationType::Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LengthTypeId {
    LengthOfSub(usize),
    NumberOfSub(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum InnerPacket {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl InnerPacket {
    #[allow(dead_code)]
    fn is_inner_packet(&self) -> bool {
        match self {
            InnerPacket::Literal(_) => true,
            InnerPacket::Operator(_) => false,
        }
    }
    #[allow(dead_code)]
    fn is_operator_packet(&self) -> bool {
        match self {
            InnerPacket::Literal(_) => false,
            InnerPacket::Operator(_) => true,
        }
    }
}

fn parse(input: &str) -> Packet {
    let bv = input
        .chars()
        .flat_map(|c| {
            let num = c.to_digit(16).unwrap() as u8;
            (0..4).rev().map(move |i| (num >> i) & 1 == 1)
        })
        .collect();
    let mut i = 0;
    parse_packet(&bv, &mut i)
}

fn read_number(bv: &BitVec, i: &mut usize, nb_bits: usize) -> usize {
    let value = (0..nb_bits)
        .map(|j| bv[*i + j])
        .rev()
        .enumerate()
        .map(|(i, bit)| if bit { 1 << i } else { 0 })
        .sum();
    *i += nb_bits;
    value
}

fn parse_packet(bv: &BitVec, i: &mut usize) -> Packet {
    debug_assert!(bv.len() - *i >= 11);
    let version = read_number(bv, i, 3);
    let type_id = read_number(bv, i, 3);
    match type_id {
        4 => Packet::new(version, InnerPacket::Literal(LiteralPacket::new(bv, i))),
        _ => Packet::new(
            version,
            InnerPacket::Operator(OperatorPacket::new(bv, type_id, i)),
        ),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1_1() {
        let input = "D2FE28";
        let v = parse(&input);
        assert_eq!(v.version, 6);
        assert_eq!(v.inner_packet.is_inner_packet(), true);
        assert_eq!(
            v.inner_packet,
            InnerPacket::Literal(LiteralPacket { value: 2021 })
        );
    }
    #[test]
    fn test_1_2() {
        let input = "38006F45291200";
        let v = parse(&input);
        assert_eq!(v.inner_packet.is_operator_packet(), true);
        match &v.inner_packet {
            InnerPacket::Literal(_) => panic!("should be operator"),
            InnerPacket::Operator(op) => {
                assert!(op.sub_packets.len() == 2);
                assert_eq!(op.sub_packets[0].inner_packet.is_inner_packet(), true);
                assert_eq!(
                    op.sub_packets[0].inner_packet,
                    InnerPacket::Literal(LiteralPacket { value: 10 })
                );
                assert_eq!(op.sub_packets[1].inner_packet.is_inner_packet(), true);
                assert_eq!(
                    op.sub_packets[1].inner_packet,
                    InnerPacket::Literal(LiteralPacket { value: 20 })
                );
            }
        }
    }

    #[test]
    fn test_1_3() {
        let input = "EE00D40C823060";
        let v = parse(&input);
        assert_eq!(v.inner_packet.is_operator_packet(), true);
        match &v.inner_packet {
            InnerPacket::Literal(_) => panic!("should be operator"),
            InnerPacket::Operator(op) => {
                assert!(op.sub_packets.len() == 3);
                assert_eq!(op.sub_packets[0].inner_packet.is_inner_packet(), true);
                assert_eq!(
                    op.sub_packets[0].inner_packet,
                    InnerPacket::Literal(LiteralPacket { value: 1 })
                );
                assert_eq!(op.sub_packets[1].inner_packet.is_inner_packet(), true);
                assert_eq!(
                    op.sub_packets[1].inner_packet,
                    InnerPacket::Literal(LiteralPacket { value: 2 })
                );
                assert_eq!(op.sub_packets[2].inner_packet.is_inner_packet(), true);
                assert_eq!(
                    op.sub_packets[2].inner_packet,
                    InnerPacket::Literal(LiteralPacket { value: 3 })
                );
            }
        }
    }

    #[test]
    fn test_2_1() {
        let input = "C200B40A82";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 3);
    }

    #[test]
    fn test_2_2() {
        let input = "04005AC33890";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 54);
    }

    #[test]
    fn test_2_3() {
        let input = "880086C3E88112";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 7);
    }

    #[test]
    fn test_2_4() {
        let input = "CE00C43D881120";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 9);
    }

    #[test]
    fn test_2_5() {
        let input = "D8005AC2A8F0";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 1);
    }

    #[test]
    fn test_2_6() {
        let input = "F600BC2D8F";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 0);
    }

    #[test]
    fn test_2_7() {
        let input = "9C005AC2F8F0";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 0);
    }

    #[test]
    fn test_2_8() {
        let input = "9C0141080250320F1802104A08";
        let p = parse(&input);
        assert_eq!(evaluate(&p), 1);
    }
}
