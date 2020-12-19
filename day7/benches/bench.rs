use criterion::{criterion_group, criterion_main, Criterion};
use day7::{get_input, part1, part2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = get_input().unwrap();

    c.bench_function("part1", |b| b.iter(|| part1(&input)));
    c.bench_function("part2", |b| b.iter(|| part2(&input)));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
