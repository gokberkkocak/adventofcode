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

    fn add_left(self, val: Option<usize>) -> Element {
        match val {
            None => self,
            Some(v) => match self {
                Element::Value(n) => Element::Value(n + v),
                Element::Pair(a, b) => Element::new_pair(a.add_left(val), *b),
            },
        }
    }

    fn add_right(self, val: Option<usize>) -> Element {
        match val {
            None => self,
            Some(v) => match self {
                Element::Value(n) => Element::Value(n + v),
                Element::Pair(a, b) => Element::new_pair(*a, b.add_right(val)),
            },
        }
    }

    fn explode(self, depth: u8) -> ExplodeResult {
        match self {
            Element::Value(_) => ExplodeResult::new(self, None, None, false),
            Element::Pair(l, r) => {
                if depth >= 4 {
                    match (*l, *r) {
                        (Element::Value(v_l), Element::Value(v_r)) => {
                            return ExplodeResult::new(
                                Element::Value(0),
                                Some(v_l),
                                Some(v_r),
                                true,
                            )
                        }
                        _ => unreachable!(),
                    }
                }
                let l_res = l.explode(depth + 1);
                if l_res.exploded {
                    ExplodeResult::new(
                        Element::new_pair(l_res.el, r.add_left(l_res.r_add)),
                        l_res.l_add,
                        None,
                        true,
                    )
                } else {
                    let r_res = r.explode(depth + 1);
                    ExplodeResult::new(
                        Element::new_pair(l_res.el.add_right(r_res.l_add), r_res.el),
                        None,
                        r_res.r_add,
                        r_res.exploded,
                    )
                }
            }
        }
    }

    fn split(self) -> (Element, bool) {
        match self {
            Element::Value(n) => {
                if n >= 10 {
                    (
                        Element::new_pair(Element::Value(n / 2), Element::Value(n / 2 + n % 2)),
                        true,
                    )
                } else {
                    (self, false)
                }
            }

            Element::Pair(a, b) => {
                let (new_a, has_split) = a.split();
                if has_split {
                    (Element::new_pair(new_a, *b), true)
                } else {
                    let (new_b, has_split) = b.split();
                    (Element::new_pair(new_a, new_b), has_split)
                }
            }
        }
    }

    fn reduce(mut self) -> Element {
        loop {
            let res = self.explode(0);
            self = res.el;
            if res.exploded {
                continue;
            }
            let (new_el, has_split) = self.split();
            self = new_el;
            if has_split {
                continue;
            }
            break;
        }
        self
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Value(n) => *n,
            Element::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }
}

struct ExplodeResult {
    el: Element,
    l_add: Option<usize>,
    r_add: Option<usize>,
    exploded: bool,
}

impl ExplodeResult {
    fn new(el: Element, l_add: Option<usize>, r_add: Option<usize>, exploded: bool) -> Self {
        ExplodeResult {
            el,
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
