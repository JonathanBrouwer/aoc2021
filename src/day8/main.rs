use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let lens = [2, 3, 4, 7];
    input.iter().map(|i| i.1.iter().filter(|s| lens.contains(&s.len())).count()).sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let overlap = |a: &str, b: &str| {
        a.chars().filter(|c| b.chars().contains(c)).count()
    };

    input.iter().map(|input| {
        let one = input.0.iter().find(|x| x.len() == 2).unwrap();
        let four = input.0.iter().find(|x| x.len() == 4).unwrap();
        let seven = input.0.iter().find(|x| x.len() == 7).unwrap();

        input.1.iter().map(|test| {
            match test.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    match (overlap(test, one), overlap(test, four)) {
                        (1, 2) => 2,
                        (2, 3) => 3,
                        (1, 3) => 5,
                        _ => unreachable!()
                    }
                }
                6 => {
                    match (overlap(test, one), overlap(test, four)) {
                        (2, 3) => 0,
                        (1, 3) => 6,
                        (2, 4) => 9,
                        _ => unreachable!()
                    }
                }
                7 => 8,
                _ => unreachable!()
            }
        }).fold(0, |a, d| a * 10 + d)
    }).sum()
}

fn parse_input(inp: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    inp.lines().map(|line| line.split(" | ").map(|part| part.split(" ").collect()).collect_tuple().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example2"));
        assert_eq!(26, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(421, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(5353, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(61229, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(986163, result);
    }
}



