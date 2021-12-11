use std::collections::VecDeque;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    input.into_iter().map(|line| {
        let mut state = VecDeque::new();
        for c in line.chars() {
            match c {
                '(' => state.push_back(')'),
                '[' => state.push_back(']'),
                '{' => state.push_back('}'),
                '<' => state.push_back('>'),
                _ => {
                    let back = state.pop_back().unwrap_or(' ');
                    if c != back {
                        return match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!()
                        }
                    }
                }
            }
        }
        return 0;
    }).sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let res : Vec<_> = input.iter().filter_map(|line| {
        let mut state = VecDeque::new();
        for c in line.chars() {
            match c {
                '(' => state.push_back(')'),
                '[' => state.push_back(']'),
                '{' => state.push_back('}'),
                '<' => state.push_back('>'),
                _ => {
                    let back = state.pop_back().unwrap_or(' ');
                    if c != back {
                        return None
                    }
                }
            }
        }
        Some(state.iter().rev().fold(0, |a, c| {
            5 * a + (match *c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!()
            })
        }))
    }).sorted().collect();
    res[res.len() / 2]
}

fn parse_input(inp: &str) -> Vec<&str> {
    inp.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(26397, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(387363, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(288957, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(4330777059, result);
    }
}



