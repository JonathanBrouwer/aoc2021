fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    sim(input, 80)
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    sim(input, 256)
}

fn sim(input: Vec<usize>, days: usize) -> usize {
    let mut state = [0usize; 9];
    input.into_iter().for_each(|n| state[n]+=1);

    for _ in 0..days {
        let reset = state[0];
        state.rotate_left(1);
        state[6] += reset;
        state[8] = reset;
    }

    state.into_iter().sum()
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
        assert_eq!(5934, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(380243, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(26984457539, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1708791884591, result);
    }
}



