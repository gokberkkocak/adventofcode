use itertools::Itertools;
use std::{iter::Peekable, str::Chars};

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 18);
    let v = parse(&input);
    let cloned_v = v.clone();
    let p1 = part1(v);
    println!("Part 1: {}", p1);
    let p2 = part2(cloned_v);
    println!("Part 2: {}", p2);
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Value(usize),
    Pair(Box<Element>, Box<Element>),
}

impl Element {
    fn new_pair(l: Element, r: Element) -> Element {
        Element::Pair(Box::new(l), Box::new(r))
    }

    fn add_left(&mut self, val: Option<usize>) {
        match val {
            None => (),
            Some(v) => match self {
                Element::Value(n) => *self = Element::Value(*n + v),
                Element::Pair(l, _r) => {
                    l.add_left(val);
                }
            },
        }
    }

    fn add_right(&mut self, val: Option<usize>) {
        match val {
            None => (),
            Some(v) => match self {
                Element::Value(n) => *self = Element::Value(*n + v),
                Element::Pair(_l, r) => {
                    r.add_right(val);
                }
            },
        }
    }

    fn explode(&mut self, depth: u8) -> ExplodeResult {
        match self {
            Element::Value(_) => ExplodeResult::new(None, None, false),
            Element::Pair(l, r) => {
                if depth >= 4 {
                    match (*l.clone(), *r.clone()) {
                        (Element::Value(v_l), Element::Value(v_r)) => {
                            *self = Element::Value(0);
                            return ExplodeResult::new(Some(v_l), Some(v_r), true);
                        }
                        _ => unreachable!(),
                    }
                }
                let l_res = l.explode(depth + 1);
                if l_res.exploded {
                    r.add_left(l_res.r_add);
                    ExplodeResult::new(l_res.l_add, None, true)
                } else {
                    let r_res = r.explode(depth + 1);
                    l.add_right(r_res.l_add);
                    ExplodeResult::new(None, r_res.r_add, r_res.exploded)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Value(n) => {
                if *n >= 10 {
                    *self =
                        Element::new_pair(Element::Value(*n / 2), Element::Value(*n / 2 + *n % 2));
                    true
                } else {
                    false
                }
            }

            Element::Pair(l, r) => {
                let l_res = l.split();
                if l_res {
                    true
                } else {
                    r.split()
                }
            }
        }
    }

    fn reduce(mut self) -> Element {
        loop {
            if self.explode(0).exploded {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
        self
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Value(n) => *n,
            Element::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

struct ExplodeResult {
    l_add: Option<usize>,
    r_add: Option<usize>,
    exploded: bool,
}

impl ExplodeResult {
    fn new(l_add: Option<usize>, r_add: Option<usize>, exploded: bool) -> Self {
        ExplodeResult {
            l_add,
            r_add,
            exploded,
        }
    }
}

fn parse(input: &str) -> Vec<Element> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars().peekable();
            parse_element(&mut chars)
        })
        .collect()
}

fn parse_element(iter: &mut Peekable<Chars>) -> Element {
    match iter.peek() {
        Some('[') => {
            iter.next();
            let left = Box::new(parse_element(iter));
            debug_assert_eq!(iter.peek(), Some(&','));
            iter.next();
            let right = Box::new(parse_element(iter));
            debug_assert_eq!(iter.peek(), Some(&']'));
            iter.next();
            Element::Pair(left, right)
        }
        Some(c) => {
            let value = c.to_digit(10).unwrap() as usize;
            iter.next();
            Element::Value(value)
        }
        None => panic!("Unexpected end of input"),
    }
}

fn part1(v: Vec<Element>) -> usize {
    let res_element = v
        .into_iter()
        .reduce(|acc, next| Element::new_pair(acc, next).reduce())
        .unwrap();
    res_element.magnitude()
}

fn part2(v: Vec<Element>) -> usize {
    let res_element = v
        .into_iter()
        .permutations(2)
        .map(|perm| Element::new_pair(perm[0].clone(), perm[1].clone()).reduce())
        .max_by_key(|el| el.magnitude())
        .unwrap();
    res_element.magnitude()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_0_1() {
        let input = "[[1,2],3]";
        let v = parse(&input);
        assert_eq!(v.len(), 1);
        assert_eq!(
            v[0],
            Element::Pair(
                Box::new(Element::Pair(
                    Box::new(Element::Value(1)),
                    Box::new(Element::Value(2))
                )),
                Box::new(Element::Value(3))
            )
        );
    }

    #[test]
    fn test_0_2() {
        let input = crate::util::read_file("inputs/2021_18_test.in");
        let v = parse(&input);
        assert_eq!(v.len(), 10);
        assert_eq!(
            v[7],
            Element::Pair(
                Box::new(Element::Pair(
                    Box::new(Element::Value(9)),
                    Box::new(Element::Value(3))
                )),
                Box::new(Element::Pair(
                    Box::new(Element::Pair(
                        Box::new(Element::Value(9)),
                        Box::new(Element::Value(9))
                    )),
                    Box::new(Element::Pair(
                        Box::new(Element::Value(6)),
                        Box::new(Element::Pair(
                            Box::new(Element::Value(4)),
                            Box::new(Element::Value(9))
                        ))
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_18_test.in");
        let v = parse(&input);
        let p1 = part1(v);
        assert_eq!(p1, 4140);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_18_test.in");
        let v = parse(&input);
        let p1 = part2(v);
        assert_eq!(p1, 3993);
    }
}
