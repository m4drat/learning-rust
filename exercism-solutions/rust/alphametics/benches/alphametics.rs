use alphametics::{solve_default, solve_improved};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_alphametics(c: &mut Criterion) {
    let mut group = c.benchmark_group("Alphametics");
    for i in [
        "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
        "SEND + MORE == MONEY",
        "HE + SEES + THE == LIGHT",
        "NO + NO + TOO == LATE",
        "A + A + A + A + A + A + A + A + A + A + A + B == BCC",
        "AS + A == MOM",
        "ABC + DEF == GH",
        "ACA + DD == BD",
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("Default", i), i, |b, i| {
            b.iter(|| solve_default(*i))
        });
        group.bench_with_input(BenchmarkId::new("Improved", i), i, |b, i| {
            b.iter(|| solve_improved(*i))
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_alphametics
}
criterion_main!(benches);
