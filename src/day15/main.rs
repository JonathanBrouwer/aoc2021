use itertools::{iproduct, Itertools};
use petgraph::algo::{astar, dijkstra};
use petgraph::Directed;
use petgraph::prelude::{GraphMap, NodeIndex};

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let size = input.len();
    let nbs = |x: usize, y: usize| {
        let mut vec = vec![];
        if x != 0 { vec.push((x - 1, y)) }
        if y != 0 { vec.push((x, y - 1)) }
        if x != size - 1 { vec.push((x + 1, y)) }
        if y != size - 1 { vec.push((x, y + 1)) }
        vec
    };

    let mut graph: GraphMap<(usize, usize), usize, Directed> = GraphMap::new();
    for (x, y) in iproduct!(0..size, 0..size) {
        for (nx, ny) in nbs(x, y) {
            graph.add_edge((nx, ny), (x, y), input[y][x]);
        }
    }

    astar(&graph, (0, 0), |g| g == (size - 1, size - 1),
          |(from, to, edge)| *edge,
          |node| (size - 1 - node.0) + (size - 1 - node.1)).unwrap().0
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let size_original = input.len();
    let size = input.len() * 5;
    let nbs = |x: usize, y: usize| {
        let mut vec = vec![];
        if x != 0 { vec.push((x - 1, y)) }
        if y != 0 { vec.push((x, y - 1)) }
        if x != size - 1 { vec.push((x + 1, y)) }
        if y != size - 1 { vec.push((x, y + 1)) }
        vec
    };

    let mut graph: GraphMap<(usize, usize), usize, Directed> = GraphMap::with_capacity(size * size, size * size);
    for (x, y) in iproduct!(0..size, 0..size) {
        let xp = x % size_original;
        let yp = y % size_original;
        let offset = x / size_original + y / size_original;
        let mut cost = input[yp][xp] + offset;
        if cost > 9 { cost -= 9 }

        for (nx, ny) in nbs(x, y) {
            graph.add_edge((nx, ny), (x, y), cost);
        }
    }

    astar(&graph, (0, 0), |g| g == (size - 1, size - 1),
          |(from, to, edge)| *edge,
          |node| (size - 1 - node.0) + (size - 1 - node.1)).unwrap().0
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



