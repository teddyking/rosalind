#[macro_use]
extern crate criterion;
extern crate counting_dna_nucleotides;

use counting_dna_nucleotides::acgt_counts;
use criterion::Criterion;

fn bench_acgt_counts(c: &mut Criterion) {
    let input = "ACGT".repeat(250);
    c.bench_function("1000 char input", move |b| b.iter(|| acgt_counts(&input)));
}

criterion_group!(benches, bench_acgt_counts);
criterion_main!(benches);
