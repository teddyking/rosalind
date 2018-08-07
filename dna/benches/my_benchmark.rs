#[macro_use]
extern crate criterion;
extern crate dna;

use criterion::Criterion;
use dna::acgt_counts;

fn bench_acgt_counts(c: &mut Criterion) {
    let input = "ACGT".repeat(250);
    c.bench_function("1000 char input", move |b| b.iter(|| acgt_counts(&input)));
}

criterion_group!(benches, bench_acgt_counts);
criterion_main!(benches);
