#[macro_use]
extern crate criterion;
extern crate rna;

use criterion::Criterion;
use rna::dna_to_rna;

fn bench_dna_to_rna(c: &mut Criterion) {
    let input = "ACGT".repeat(250);
    c.bench_function("1000 char input", move |b| b.iter(|| dna_to_rna(&input)));
}

criterion_group!(benches, bench_dna_to_rna);
criterion_main!(benches);
