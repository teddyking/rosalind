#[macro_use]
extern crate criterion;
extern crate fib;

use criterion::Criterion;
use fib::rabbits;

fn bench_rabbits(c: &mut Criterion) {
    let n = 20;
    let k = 3;
    c.bench_function("n = 20, k = 3", move |b| b.iter(|| rabbits(n, k)));
}

criterion_group!(benches, bench_rabbits);
criterion_main!(benches);
