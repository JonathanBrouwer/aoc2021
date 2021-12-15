fn part1(inp: &str) -> usize {
    let mut input = parse_input(inp);
    input.sort();
    let target = input[input.len() / 2];
    input.iter().map(|x| x.abs_diff(target)).sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let s: usize = input.iter().sum();
    let cost = |target: usize| input.iter().map(|x| {
        let diff = x.abs_diff(target);
        diff * (diff + 1) / 2
    }).sum();
    return usize::min(cost(s / input.len()), cost((s + input.len() - 1) / input.len()));
}

fn parse_input(inp: &str) -> Vec<usize> {
    inp.split(",").map(|n| n.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(37, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(359648, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(168, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(100727924, result);
    }
}



