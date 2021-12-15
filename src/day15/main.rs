use petgraph::algo::{astar};
use crate::day15::grid::Grid;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let size = input.len();
    let graph = Grid::new(size);
    astar(&graph, (0, 0), |g| g == (size - 1, size - 1), |(_from, to, _)| input[to.1][to.0], |cur| (size - 1 - cur.0) + (size - 1 - cur.1)).unwrap().0
}

pub fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let size_original = input.len();
    let size = input.len() * 5;

    let graph = Grid::new(size);
    astar(&graph, (0, 0), |g| g == (size - 1, size - 1), |(_from, (x, y), _)| {
        let cost = input[y % size_original][x % size_original];
        let offset = x / size_original + y / size_original;
        let mut cost = cost + offset;
        if cost > 9 { cost -= 9 }
        cost
    }, |cur| (size - 1 - cur.0) + (size - 1 - cur.1)).unwrap().0
}

fn parse_input(inp: &str) -> Vec<Vec<usize>> {
    inp.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(40, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(741, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(315, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2976, result);
    }
}



