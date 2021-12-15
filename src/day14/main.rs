use std::collections::{HashMap};
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    solve(inp, 10)
}

fn part2(inp: &str) -> usize {
    solve(inp, 40)
}

fn solve(inp: &str, iterations: usize) -> usize {
    let (start, rules) = parse_input(inp);

    let mut rulesmap = HashMap::new();
    rules.into_iter().for_each(|(inp, outp)| {
        rulesmap.insert((inp.chars().next().unwrap(), inp.chars().nth(1).unwrap()), outp.chars().next().unwrap());
    });

    let mut map: HashMap<(char, char), usize> = HashMap::new();
    let buf: String = "_".to_owned() + start + "_";
    buf.chars().tuple_windows().for_each(|(c1, c2)| {
        *map.entry((c1, c2)).or_insert(0) += 1
    });

    //Do step
    for _ in 0..iterations {
        let mut newmap = HashMap::new();
        map.into_iter().for_each(|(k, v)| {
            match rulesmap.get(&k) {
                None => {
                    *newmap.entry(k).or_insert(0) += v;
                }
                Some(c) => {
                    *newmap.entry((k.0, *c)).or_insert(0) += v;
                    *newmap.entry((*c, k.1)).or_insert(0) += v;
                }
            }
        });
        map = newmap;
    }

    let mut ccount = HashMap::new();
    for (k, v) in map {
        *ccount.entry(k.0).or_insert(0) += v;
    }
    ccount.remove(&'_');

    ccount.values().max().unwrap() - ccount.values().min().unwrap()
}

fn parse_input(inp: &str) -> (&str, Vec<(&str, &str)>) {
    let start = inp.lines().next().unwrap();
    let rules = inp.lines().skip(2).map(|line| line.split(" -> ").collect_tuple().unwrap()).collect();
    (start, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(1588, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2509, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(2188189693529, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2827627697643, result);
    }
}



