#[macro_use]
extern crate criterion;
extern crate transcribing_dna_to_rna;

use transcribing_dna_to_rna::dna_to_rna;
use criterion::Criterion;

fn bench_dna_to_rna(c: &mut Criterion) {
    let input = "ACGT".repeat(250);
    c.bench_function("1000 char input", move |b| b.iter(|| dna_to_rna(&input)));
}

criterion_group!(benches, bench_dna_to_rna);
criterion_main!(benches);
