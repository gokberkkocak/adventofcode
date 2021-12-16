use bitvec::vec::BitVec;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 16);
    let package = parse(&input);
    let p1 = sum_versions(&package);
    println!("Part 1: {}", p1);
    let p2 = evaluate(&package);
    println!("Part 1: {}", p2);
}

fn sum_versions(package: &Package) -> usize {
    let sub = match &package.inner_package {
        InnerPackage::Literal(_) => 0,
        InnerPackage::Operator(op) => op.sub_packages.iter().map(sum_versions).sum::<usize>(),
    };
    sub + package.version
}

fn evaluate(package: &Package) -> usize {
    match &package.inner_package {
        InnerPackage::Literal(l) => l.value,
        InnerPackage::Operator(op) => {
            let mut sub_packages_values = op.sub_packages.iter().map(evaluate);
            match op.type_id {
                0 => sub_packages_values.sum(),
                1 => sub_packages_values.product(),
                2 => sub_packages_values.min().unwrap(),
                3 => sub_packages_values.max().unwrap(),
                5 => {
                    (sub_packages_values.next().unwrap() > sub_packages_values.next().unwrap())
                        as usize
                }
                6 => {
                    (sub_packages_values.next().unwrap() < sub_packages_values.next().unwrap())
                        as usize
                }
                7 => {
                    (sub_packages_values.next().unwrap() == sub_packages_values.next().unwrap())
                        as usize
                }
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Package {
    version: usize,
    inner_package: InnerPackage,
}

impl Package {
    fn new(version: usize, inner_package: InnerPackage) -> Package {
        Package {
            version,
            inner_package,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPackage {
    value: usize,
}

impl LiteralPackage {
    fn new(bv: &BitVec, i: &mut usize) -> LiteralPackage {
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
        LiteralPackage { value }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorPackage {
    type_id: usize,
    length_type_id: LengthTypeId,
    sub_packages: Vec<Package>,
}

impl OperatorPackage {
    fn new(bv: &BitVec, type_id: usize, i: &mut usize) -> OperatorPackage {
        let length_type = bv[*i];
        *i += 1;
        let length_type_id = match length_type {
            false => LengthTypeId::LengthOfSub(read_number(bv, i, 15)),
            true => LengthTypeId::NumberOfSub(read_number(bv, i, 11)),
        };
        let mut sub_packages = Vec::new();
        match length_type_id {
            LengthTypeId::LengthOfSub(total_length) => {
                let starting_bit_location = *i; // for subpackages
                while total_length != *i - starting_bit_location {
                    sub_packages.push(parse_package(bv, i));
                }
            }
            LengthTypeId::NumberOfSub(number) => {
                for _ in 0..number {
                    sub_packages.push(parse_package(bv, i));
                }
            }
        }

        OperatorPackage {
            type_id,
            length_type_id,
            sub_packages,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LengthTypeId {
    LengthOfSub(usize),
    NumberOfSub(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum InnerPackage {
    Literal(LiteralPackage),
    Operator(OperatorPackage),
}

impl InnerPackage {
    #[allow(dead_code)]
    fn is_inner_package(&self) -> bool {
        match self {
            InnerPackage::Literal(_) => true,
            InnerPackage::Operator(_) => false,
        }
    }
    #[allow(dead_code)]
    fn is_operator_package(&self) -> bool {
        match self {
            InnerPackage::Literal(_) => false,
            InnerPackage::Operator(_) => true,
        }
    }
}

fn parse(input: &str) -> Package {
    let mut bv = BitVec::new();
    input
        .chars()
        .flat_map(|c| {
            (0..4)
                .rev()
                .map(move |i| (c.to_digit(16).unwrap() as u8 >> i) & 1 == 1)
        })
        .for_each(|b| bv.push(b));
    let mut i = 0;
    parse_package(&bv, &mut i)
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

fn parse_package(bv: &BitVec, i: &mut usize) -> Package {
    debug_assert!(bv.len() - *i >= 11);
    let version = read_number(bv, i, 3);
    let type_id = read_number(bv, i, 3);
    match type_id {
        4 => Package::new(version, InnerPackage::Literal(LiteralPackage::new(bv, i))),
        _ => Package::new(
            version,
            InnerPackage::Operator(OperatorPackage::new(bv, type_id, i)),
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
        assert_eq!(v.inner_package.is_inner_package(), true);
        assert_eq!(
            v.inner_package,
            InnerPackage::Literal(LiteralPackage { value: 2021 })
        );
    }
    #[test]
    fn test_1_2() {
        let input = "38006F45291200";
        let v = parse(&input);
        assert_eq!(v.inner_package.is_operator_package(), true);
        match &v.inner_package {
            InnerPackage::Literal(_) => panic!("should be operator"),
            InnerPackage::Operator(op) => {
                assert!(op.sub_packages.len() == 2);
                assert_eq!(op.sub_packages[0].inner_package.is_inner_package(), true);
                assert_eq!(
                    op.sub_packages[0].inner_package,
                    InnerPackage::Literal(LiteralPackage { value: 10 })
                );
                assert_eq!(op.sub_packages[1].inner_package.is_inner_package(), true);
                assert_eq!(
                    op.sub_packages[1].inner_package,
                    InnerPackage::Literal(LiteralPackage { value: 20 })
                );
            }
        }
    }

    #[test]
    fn test_1_3() {
        let input = "EE00D40C823060";
        let v = parse(&input);
        assert_eq!(v.inner_package.is_operator_package(), true);
        match &v.inner_package {
            InnerPackage::Literal(_) => panic!("should be operator"),
            InnerPackage::Operator(op) => {
                assert!(op.sub_packages.len() == 3);
                assert_eq!(op.sub_packages[0].inner_package.is_inner_package(), true);
                assert_eq!(
                    op.sub_packages[0].inner_package,
                    InnerPackage::Literal(LiteralPackage { value: 1 })
                );
                assert_eq!(op.sub_packages[1].inner_package.is_inner_package(), true);
                assert_eq!(
                    op.sub_packages[1].inner_package,
                    InnerPackage::Literal(LiteralPackage { value: 2 })
                );
                assert_eq!(op.sub_packages[2].inner_package.is_inner_package(), true);
                assert_eq!(
                    op.sub_packages[2].inner_package,
                    InnerPackage::Literal(LiteralPackage { value: 3 })
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
