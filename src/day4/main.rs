use itertools::{iproduct, Itertools};

fn part1(inp: &str) -> usize {
    solve_part(inp, false)
}

fn part2(inp: &str) -> usize {
    solve_part(inp, true)
}

fn solve_part(inp: &str, maximize: bool) -> usize {
    let (calls, boards) = parse_input(inp);
    let (min, max) = boards.iter().map(|board| {
        let mut state = [[false; 5]; 5];
        for (i, call) in calls.iter().enumerate() {
            for (x, y) in iproduct!(0..5, 0..5) {
                if board.data[x][y] == *call {state[x][y] = true}
            }
            if is_bingo(&state) {
                let sum: usize = state.iter().flatten().zip(board.data.iter().flatten()).filter(|(b, _)| !**b).map(|(_, v)| v).sum();
                return (i, *call, sum)
            }
        }
        unreachable!()
    }).minmax_by_key(|(i, _, _)| *i).into_option().unwrap();
    let (_, call, sum) = if maximize {max} else {min};
    call * sum
}

fn is_bingo(state: &[[bool; 5]; 5]) -> bool {
    //Horizontal
    'outer1: for x in 0..5 {
        for y in 0..5 {
            if !state[x][y] { continue 'outer1; }
        }
        return true;
    }
    'outer2: for y in 0..5 {
        for x in 0..5 {
            if !state[x][y] { continue 'outer2; }
        }
        return true;
    }
    return false;
}





fn parse_input(inp: &str) -> (Vec<usize>, Vec<Board>) {
    let lines: Vec<_> = inp.lines().collect();
    let calls: Vec<_> = lines[0].split(",").map(|n| n.parse().unwrap()).collect();
    let boards: Vec<Board> = lines[2..].iter().batching(|sublines| {
        if sublines.len() < 5 { return None }
        let board = Board{ data: sublines.take(5).map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect() };
        let _ = sublines.next();
        Some(board)
    }).collect();
    (calls, boards)
}

struct Board {
    data: Vec<Vec<usize>>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(4512, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(11536, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(1924, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1284, result);
    }
}



