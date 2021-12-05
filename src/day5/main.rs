use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    solve(input.into_iter().filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2).collect())
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    solve(input)
}

fn solve(input: Vec<((isize, isize), (isize, isize))>) -> usize {
    let mut state = HashMap::<(isize, isize), usize>::new();
    for ((x1, y1), (x2, y2)) in input {
        let mut x = x1;
        let mut y = y1;
        loop {
            *state.entry((x, y)).or_default() += 1;
            if x == x2 && y == y2 { break }
            x += (x2 - x).signum();
            y += (y2 - y).signum();
        }
    }
    state.iter().filter(|(k, v)| **v > 1).count()
}

fn parse_input(inp: &str) -> Vec<((isize, isize), (isize, isize))> {
    inp.lines().map(|line| line.split(" -> ").map(|p| p.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap()).collect_tuple().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(5, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(5373, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(12, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(21514, result);
    }
}



