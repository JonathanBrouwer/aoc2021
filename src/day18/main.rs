use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::day18::main::ExplodeResult::*;
use crate::day18::main::SnailfishNumber::*;

fn part1(inp: &str) -> usize {
    fold_input(parse_input(inp)).magnitude()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    input.into_iter().permutations(2).map(|n| fold_input(n).magnitude()).max().unwrap()
}

fn fold_input(input: Vec<SnailfishNumber>) -> SnailfishNumber {
    input.into_iter().fold1(|a, b| {
        let mut res = SnailfishNumber::Snailfish(Box::new(a), Box::new(b));
        res.reduce();
        res
    }).unwrap()
}

fn parse_input(inp: &str) -> Vec<SnailfishNumber> {
    inp.lines().map(|line| {
        parse_input_part(line).0
    }).collect()
}

fn parse_input_part(inp: &str) -> (SnailfishNumber, &str) {
    if inp.starts_with("[") {
        let (p1, inp) = parse_input_part(&inp[1..]);
        assert_eq!(inp.chars().next().unwrap(), ',');
        let (p2, inp) = parse_input_part(&inp[1..]);
        assert_eq!(inp.chars().next().unwrap(), ']');
        (SnailfishNumber::Snailfish(Box::new(p1), Box::new(p2)), &inp[1..])
    } else {
        let num = inp.chars().take_while(|c| c.is_ascii_digit()).join("").parse().unwrap();
        let rest = inp.trim_start_matches(|c: char| c.is_ascii_digit());
        (SnailfishNumber::Normal(num), rest)
    }
}

#[derive(Clone)]
enum SnailfishNumber {
    Normal(usize),
    Snailfish(Box<SnailfishNumber>, Box<SnailfishNumber>)
}

enum ExplodeResult {
    NoExplode,
    ExplodeDone(Option<usize>, Option<usize>)
}

impl SnailfishNumber {
    pub fn reduce(&mut self) {
        loop {
            // println!("{}", self);
            if let ExplodeDone(_, _) = self.explode_first(0) { continue }
            if self.split_first() { continue }
            break;
        }
        // println!("{}", self);
    }

    pub fn unwrap_normal(&self) -> usize {
        match self {
            Normal(n) => *n,
            Snailfish(_, _) => {unreachable!()}
        }
    }

    fn explode_first(&mut self, depth: usize) -> ExplodeResult {
        match self {
            SnailfishNumber::Normal(_) => {
                ExplodeResult::NoExplode
            }
            SnailfishNumber::Snailfish(l,r) => {
                if depth >= 4 {
                    let res = ExplodeDone(Some(l.unwrap_normal()), Some(r.unwrap_normal()));
                    *self = Normal(0);
                    res
                } else {
                    match l.explode_first(depth + 1) {
                        NoExplode => {
                            match r.explode_first(depth + 1) {
                                NoExplode => NoExplode,
                                ExplodeDone(el, er) => {
                                    if let Some(el) = el {
                                        l.add_right(el);
                                        ExplodeDone(None, er)
                                    } else {
                                        ExplodeDone(el, er)
                                    }
                                }
                            }
                        }
                        ExplodeDone(el, er) => {
                            if let Some(er) = er {
                                r.add_left(er);
                                ExplodeDone(el, None)
                            } else {
                                ExplodeDone(el, er)
                            }
                        }
                    }

                }
            }
        }
    }

    pub fn split_first(&mut self) -> bool {
        match self {
            Normal(n) => {
                return if *n >= 10 {
                    *self = Snailfish(Box::new(Normal(*n / 2)), Box::new(Normal((*n + 1) / 2)));
                    true
                } else {
                    false
                }
            }
            Snailfish(l, r) => {
                if l.split_first() { return true }
                if r.split_first() { return true }
                return false
            }
        }
    }

    fn add_left(&mut self, val: usize) {
        match self {
            Normal(n) => *n += val,
            Snailfish(l, _) => l.add_left(val)
        }
    }

    fn add_right(&mut self, val: usize) {
        match self {
            Normal(n) => *n += val,
            Snailfish(_, r) => r.add_right(val)
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            SnailfishNumber::Normal(n) => *n,
            SnailfishNumber::Snailfish(l, r) => {
                3 * l.magnitude() + 2 * r.magnitude()
            }
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Normal(n) => {
                write!(f, "{}", n)
            }
            Snailfish(l, r) => {
                write!(f, "[{},{}]", l, r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_explodes() {
        assert_eq!("[[[[0,9],2],3],4]", {
            let mut x = parse_input_part("[[[[[9,8],1],2],3],4]").0;
            x.explode_first(0);
            x.to_string()
        });
        assert_eq!("[7,[6,[5,[7,0]]]]", {
            let mut x = parse_input_part("[7,[6,[5,[4,[3,2]]]]]").0;
            x.explode_first(0);
            x.to_string()
        });
        assert_eq!("[[6,[5,[7,0]]],3]", {
            let mut x = parse_input_part("[[6,[5,[4,[3,2]]]],1]").0;
            x.explode_first(0);
            x.to_string()
        });
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", {
            let mut x = parse_input_part("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").0;
            x.explode_first(0);
            x.to_string()
        });
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", {
            let mut x = parse_input_part("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").0;
            x.explode_first(0);
            x.to_string()
        });
    }

    #[test]
    fn test_part1_full() {
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", {
            let xs = parse_input("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]");
            let x = fold_input(xs);
            x.to_string()
        });
        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", {
            let xs = parse_input("[1,1]\n[2,2]\n[3,3]\n[4,4]");
            let x = fold_input(xs);
            x.to_string()
        });
        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", {
            let xs = parse_input("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]");
            let x = fold_input(xs);
            x.to_string()
        });
        assert_eq!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", {
            let xs = parse_input(include_str!("example"));
            let x = fold_input(xs);
            x.to_string()
        });
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(4433, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(4559, result);
    }
}



