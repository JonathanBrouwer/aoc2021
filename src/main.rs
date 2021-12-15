use criterion::black_box;
use aoc2021::day15::main::part2;

pub fn main() {
    let inp = include_str!("../src/day15/input");
    part2(black_box(inp));

}