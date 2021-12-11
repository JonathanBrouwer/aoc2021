use std::collections::VecDeque;
use itertools::{iproduct, Itertools};

fn part1(inp: &str) -> usize {
    let mut state = parse_input(inp);
    let size = state.len();
    let nbs = |x:usize,y:usize| {
        let minx = if x == 0 {x} else {x - 1};
        let maxx = if x == size - 1 {x} else {x+1};
        let miny = if y == 0 {y} else {y-1};
        let maxy = if y == size - 1 {y} else {y+1};
        iproduct!(minx..=maxx, miny..=maxy).collect_vec()
    };

    let mut total = 0;
    let mut queue = VecDeque::new();
    for _ in 0..100 {
        for (x, y) in iproduct!(0..size, 0..size) {
            state[x][y] += 1;
            if state[x][y] == 10 { queue.push_back((x, y)) }
        }
        while let Some((x, y)) = queue.pop_back() {
            total += 1;
            for (nx, ny) in nbs(x, y) {
                state[nx][ny] += 1;
                if state[nx][ny] == 10 {
                    queue.push_back((nx, ny));
                }
            }
        }
        for (x, y) in iproduct!(0..size, 0..size) {
            if state[x][y] >= 10 {
                state[x][y] = 0;
            }
        }
    }

    total
}

fn part2(inp: &str) -> usize {
    let mut state = parse_input(inp);
    let size = state.len();
    let nbs = |x:usize,y:usize| {
        let minx = if x == 0 {x} else {x - 1};
        let maxx = if x == size - 1 {x} else {x+1};
        let miny = if y == 0 {y} else {y-1};
        let maxy = if y == size - 1 {y} else {y+1};
        iproduct!(minx..=maxx, miny..=maxy).collect_vec()
    };

    let mut queue = VecDeque::new();
    for i in 1.. {
        for (x, y) in iproduct!(0..size, 0..size) {
            state[x][y] += 1;
            if state[x][y] == 10 { queue.push_back((x, y)) }
        }
        while let Some((x, y)) = queue.pop_back() {
            for (nx, ny) in nbs(x, y) {
                state[nx][ny] += 1;
                if state[nx][ny] == 10 {
                    queue.push_back((nx, ny));
                }
            }
        }
        if state.iter().flatten().all(|v| *v >= 10) {
            return i
        }
        for (x, y) in iproduct!(0..size, 0..size) {
            if state[x][y] >= 10 {
                state[x][y] = 0;
            }
        }
    }
    unreachable!()
}

fn parse_input(inp: &str) -> Vec<Vec<usize>> {
    inp.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(259, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(1656, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1615, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(6, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(195, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(249, result);
    }
}



