#[macro_use]
extern crate criterion;
extern crate revc;

use criterion::Criterion;
use revc::reverse_complement;

fn bench_reverse_complement(c: &mut Criterion) {
    let input = "ACGT".repeat(250);
    c.bench_function("1000 char input", move |b| {
        b.iter(|| reverse_complement(&input))
    });
}

criterion_group!(benches, bench_reverse_complement);
criterion_main!(benches);
