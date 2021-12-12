use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use petgraph::prelude::*;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut queue: VecDeque<(Vec<&str>, &str)> = VecDeque::new();
    queue.push_back((vec![], "start"));

    let mut total = 0;
    while let Some((mut visited, next)) = queue.pop_back() {
        if next == "end" {
            total += 1;
            continue;
        }
        if next.chars().next().unwrap().is_lowercase() {
            visited.push(next);
        }
        for nb in input.neighbors(next) {
            if visited.contains(&nb) { continue }
            queue.push_back((visited.clone(), nb))
        }
    }

    return total
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut queue: VecDeque<(bool, Vec<&str>, &str)> = VecDeque::new();
    queue.push_back((false, vec![], "start"));

    let mut total = 0;
    while !queue.is_empty() {
        let (power, mut visited, next) = queue.pop_back().unwrap();
        if next == "end" {
            total += 1;
            continue;
        }
        if next.chars().next().unwrap().is_lowercase() {
            visited.push(next);
        }
        for nb in input.neighbors(next) {
            if visited.contains(&nb) {
                if !power && nb != "start" {
                    queue.push_back((true, visited.clone(), nb))
                }
            } else {
                queue.push_back((power, visited.clone(), nb))
            }
        }
    }

    return total
}

fn parse_input(inp: &str) -> GraphMap<&str, (), Undirected> {
    let mut graph = GraphMap::new();
    inp.lines().for_each(|l| {
        let (a, b) = l.split("-").collect_tuple().unwrap();
        graph.add_edge(a, b, ());
    });
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(10, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(19, result);
    }

    #[test]
    fn test_part1_ex3() {
        let result = part1(include_str!("example3"));
        assert_eq!(226, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(4970, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(36, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(103, result);
    }

    #[test]
    fn test_part2_ex3() {
        let result = part2(include_str!("example3"));
        assert_eq!(3509, result);
    }
    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(137948, result);
    }
}



