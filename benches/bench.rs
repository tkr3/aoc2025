use criterion::{criterion_group, criterion_main};

// To run individual benchmarks use:
// $ cargo bench --bench bench -- <name>
// where <name> can be like: day_07, 07, 07/1, 7/2

#[rustfmt::skip]
aoc2025::benches!(
    day_01
);

criterion_main!(benchmarks);
