use std::collections::{HashSet, VecDeque};
use itertools::{iproduct, Itertools};

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let nbs = |x: usize, y: usize| {
        let mut points = vec![];
        if x > 0 { points.push((x-1,y)) }
        if y > 0 { points.push((x,y-1)) }
        if x < input.len() - 1 { points.push((x+1, y)) }
        if y < input[0].len() - 1 { points.push((x, y+1)) }
        points
    };

    iproduct!(0..input.len(), 0..input[0].len()).filter_map(|(x, y)| {
        //Find minpoints
        let cur = input[x][y];
        let nbmin = nbs(x, y).iter().map(|&(x, y)| input[x][y]).min().unwrap();

        if nbmin > cur {
            Some((x, y))
        }else {
            None
        }
    }).map(|(x, y)| input[x][y] + 1).sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let nbs = |x: usize, y: usize| {
        let mut points = vec![];
        if x > 0 { points.push((x-1,y)) }
        if y > 0 { points.push((x,y-1)) }
        if x < input.len() - 1 { points.push((x+1, y)) }
        if y < input[0].len() - 1 { points.push((x, y+1)) }
        points
    };

    iproduct!(0..input.len(), 0..input[0].len()).filter_map(|(x, y)| {
        //Find minpoints
        let cur = input[x][y];
        let nbmin = nbs(x, y).iter().map(|&(x, y)| input[x][y]).min().unwrap();

        if nbmin > cur {
            Some((x, y))
        }else {
            None
        }
    }).map(|(sx, sy)| {
        let mut points = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy));

        while let Some((x, y)) = queue.pop_front() {
            if points.contains(&(x,y)) { continue };
            points.insert((x, y));
            for (nx, ny) in nbs(x, y) {
                if input[nx][ny] == 9 { continue }
                if input[nx][ny] <= input[x][y] { continue }
                queue.push_back((nx, ny))
            }
        }

        points.len()
    }).sorted().rev().take(3).product()
}

fn parse_input(inp: &str) -> Vec<Vec<usize>> {
    inp.lines().map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(15, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(475, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(1134, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1092012, result);
    }
}



