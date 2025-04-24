use criterion::{Criterion, black_box, criterion_group, criterion_main};
use extended_float::types::ExtendedFloat;
mod common;
use common::random::RandomF64StringIterator;

const SEED: u64 = 123456789;
const PRECISIONS: [usize; 5] = [0, 1, 3, 5, 10];

fn bench_conversions(c: &mut Criterion) {
    for precision in PRECISIONS {
        let mut iter = RandomF64StringIterator::new(SEED, precision, -100_000.0..=100_000.0);
        c.bench_function(
            format!("bench parse from string {}_precision", precision).as_str(),
            |b| {
                b.iter_with_setup(
                    || iter.next().unwrap(),
                    |param| {
                        let result = param.parse::<ExtendedFloat<f64>>().unwrap();
                        black_box(result)
                    },
                )
            },
        );
    }
}

fn bench_conversions_f64(c: &mut Criterion) {
    for precision in PRECISIONS {
        let mut iter = RandomF64StringIterator::new(SEED, precision, -100_000.0..=100_000.0);
        c.bench_function(
            format!("bench parse from string {}_precision [f64]", precision).as_str(),
            |b| {
                b.iter_with_setup(
                    || iter.next().unwrap(),
                    |param| {
                        let result = param.parse::<f64>().unwrap();
                        black_box(result)
                    },
                )
            },
        );
    }
}

criterion_group!(benches, bench_conversions, bench_conversions_f64);
criterion_main!(benches);
