fn part1(xrange: (isize, isize), yrange: (isize, isize)) -> isize {
    let mut max = 0;
    for dx in 0..=xrange.1 {
        for dy in 0..1000 {
            let mut state = State{ pos: (0, 0), vel: (dx, dy) };
            let v = state.hits(xrange,yrange).unwrap_or(isize::MIN);
            max = max.max(v);
        }
    }
    return max;
}

fn part2(xrange: (isize, isize), yrange: (isize, isize)) -> usize {
    let mut count = 0;
    for dx in 0..=xrange.1 {
        for dy in -1000..1000 {
            let mut state = State{ pos: (0, 0), vel: (dx, dy) };
            if state.hits(xrange,yrange).is_some() {
                count += 1;
            }
        }
    }
    return count;
}

struct State {
    pos: (isize, isize),
    vel: (isize, isize)
}

impl State {
    pub fn update(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.vel.0 -= self.vel.0.signum();
        self.vel.1 -= 1;
    }

    pub fn hits(&mut self, xrange: (isize, isize), yrange: (isize, isize)) -> Option<isize> {
        let mut maxy = isize::MIN;
        loop {
            if self.pos.0 > xrange.1 || self.pos.1 < yrange.0 { return None }
            maxy = maxy.max(self.pos.1);
            if xrange.0 <= self.pos.0 && self.pos.0 <= xrange.1 && yrange.0 <= self.pos.1 && self.pos.1 <= yrange.1 { return Some(maxy) }
            self.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1((20, 30), (-10, -5));
        assert_eq!(45, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1((265,287), (-103, -58));
        println!("Part 1: {}", result);
        assert_eq!(5253, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2((20, 30), (-10, -5));
        assert_eq!(112, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2((265,287), (-103, -58));
        println!("Part 2: {}", result);
        assert_eq!(1770, result);
    }
}



