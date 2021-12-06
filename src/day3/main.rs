
fn part1(inp: &str, len: usize) -> usize {
    let input = parse_input(inp);
    let half = input.len() / 2;
    let state = input.into_iter().fold(vec![0usize; len], |mut state, next| {
        for s in 0..len {
            if (next & (1 << s)) != 0 { state[s] += 1 }
        }
        state
    });

    let mut gamma = 0;
    for s in 0..len {
        if state[s] > half { gamma |= 1 << s }
    }
    let epsilon_mask = (1 << len) - 1;
    let epsilon = gamma ^ epsilon_mask;
    return gamma * epsilon;
}

fn part2(inp: &str, len: usize) -> usize {
    let input = parse_input(inp);

    let ox = part2_calc(input.clone(), true, len);
    let co2 = part2_calc(input, false, len);
    ox * co2
}

fn part2_calc(left: Vec<usize>, take_most_common: bool, mut bit: usize) -> usize {
    assert!(left.len() > 0);
    if left.len() == 1 {
        return left[0]
    }
    bit -= 1;
    let one_count = left.iter().filter(|w| *w & (1 << bit) != 0).count();
    let most_common_one = one_count * 2 >= left.len();
    let take_one = most_common_one ^ (!take_most_common);
    let target = if take_one { 1 << bit } else {0};
    part2_calc(left.into_iter().filter(|w| w & (1 << bit) == target).collect(), take_most_common, bit)
}

fn parse_input(inp: &str) -> Vec<usize> {
    inp.lines().map(|s| usize::from_str_radix(&s, 2).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"), 5);
        assert_eq!(198, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"), 12);
        println!("Part 1: {}", result);
        assert_eq!(693486, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"), 5);
        assert_eq!(230, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 12);
        println!("Part 2: {}", result);
        assert_eq!(3379326, result);
    }
}



