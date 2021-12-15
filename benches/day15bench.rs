use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use aoc2021::day15::main::part2;

pub fn criterion_benchmark(c: &mut Criterion) {
    let inp = include_str!("../src/day15/input");
    c.bench_function("day15 part2", |b| b.iter(|| {
        part2(black_box(inp))
    }));

}

criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = criterion_benchmark
}
criterion_main!(benches);