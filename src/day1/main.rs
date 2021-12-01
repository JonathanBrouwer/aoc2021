use itertools::Itertools;

pub(crate) fn part1(inp: &str) -> usize {
    let nums = parse_input(inp);
    nums.into_iter().tuple_windows::<(usize, usize)>().filter(|(a, b)| a < b).count()
}

fn part2(inp: &str) -> usize {
    let nums = parse_input(inp);
    nums.windows(3).map(|w| w.iter().sum())
        .tuple_windows::<(usize, usize)>().filter(|(a, b)| a < b).count()
}

fn parse_input(inp: &str) -> Vec<usize> {
    //Parse input into vec
    return inp.lines().map(|num| num.parse().unwrap()).collect();
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(7, result);
    }

    #[test]
    pub(crate) fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1709, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(5, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1761, result);
    }
}



