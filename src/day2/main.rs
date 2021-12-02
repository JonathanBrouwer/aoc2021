use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let (x, y) = input.into_iter().fold1(|(x1, y1), (x2, y2) | (x1 + x2, y1 + y2)).unwrap();
    (x * y) as usize
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let (x, y, _) = input.into_iter().fold((0, 0, 0), |(x1, y1, aim), (dx, daim)| (x1 + dx, y1 + aim * dx, aim + daim));
    (x * y) as usize
}

fn parse_input(inp: &str) -> Vec<(isize, isize)> {
    inp.lines().map(|line| (line.split(" ").next().unwrap(), line.split(" ").skip(1).next().unwrap().parse::<isize>().unwrap()))
        .map(|(dir, mag)|
            match dir {
                "forward" => (mag, 0),
                "down" => (0, mag),
                "up" => (0, -mag),
                _ => unreachable!()
            }
        ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(150, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2187380, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(900, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2086357770, result);

        //Too low: 1248660374
    }
}



