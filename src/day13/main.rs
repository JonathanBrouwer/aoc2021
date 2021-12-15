use std::collections::{HashSet};
use itertools::{Itertools};

fn part1(inp: &str) -> usize {
    let (points, folds) = parse_input(inp);

    let mut pointsmap = HashSet::new();
    points.iter().for_each(|&p| {pointsmap.insert(p);} );

    let fold = folds[0];
    pointsmap = pointsmap.iter().map(|&(x, y)| {
        if fold.0 && x >= fold.1 {
            (fold.1 - (x - fold.1), y)
        } else if !fold.0 && y >= fold.1 {
            (x, fold.1 - (y - fold.1))
        } else {
            (x, y)
        }
    }).collect();

    pointsmap.len()
}

fn part2(inp: &str) {
    let (points, folds) = parse_input(inp);

    let mut pointsmap = HashSet::new();
    points.iter().for_each(|&p| {pointsmap.insert(p);} );

    for fold in folds {
        pointsmap = pointsmap.iter().map(|&(x, y)| {
            if fold.0 && x >= fold.1 {
                (fold.1 - (x - fold.1), y)
            } else if !fold.0 && y >= fold.1 {
                (x, fold.1 - (y - fold.1))
            } else {
                (x, y)
            }
        }).collect();
    }

    let max = pointsmap.iter().fold((0usize, 0usize), |a, &v| (a.0.max(v.0), a.1.max(v.1)) );
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            print!("{}", if pointsmap.contains(&(x, y)) {"■"} else {" "})
        }
        println!();
    }
}

fn parse_input(inp: &str) -> (Vec<(usize, usize)>, Vec<(bool, usize)>) {
    let lines: Vec<&str> = inp.lines().collect();
    let ps: Vec<_> = lines.iter().take_while(|l| **l != "").map(|line| line.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap()).collect();
    let is = lines.iter().skip(ps.len() + 1).map(|l| {
        let (t0, t1) = l.split("=").collect_tuple().unwrap();
        (t0 == "fold along x", t1.parse().unwrap())
    }).collect();
    (ps, is)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(17, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(618, result);
    }

    #[test]
    fn test_part2_real() {
        part2(include_str!("input"));
        // println!("Part 2: {}", result);
        // assert_eq!(0, result);
    }
}



