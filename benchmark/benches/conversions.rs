use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use extended_float::types::ExtendedFloat;
mod common;

use common::files::read_lines_from_file;
use common::random::RandomF64StringIterator;

const SEED: u64 = 123456789;
// const PRECISIONS: [usize; 1] = [0,];
const PRECISIONS: [usize; 5] = [0, 1, 3, 5, 10];

// TODO: add benches with other crates
// TODO: try other bench created (CPU counters)

fn bench_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("[ExtendedFloat<f64>]");

    for precision in PRECISIONS {
        let mut iter = RandomF64StringIterator::new(SEED, precision, -100_000.0..=100_000.0);
        group.bench_function(
            format!("parse from string {}_precision", precision).as_str(),
            |b| {
                b.iter_batched(
                    || iter.next().unwrap(),
                    |data| {
                        let result = data.parse::<ExtendedFloat<f64>>().unwrap();
                        black_box(result)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    // TODO: preallocate vec
    group.bench_function("parse entire file", |b| {
        let file_data =
            read_lines_from_file("test_data/prices.log").expect("Failed to read data from file");
        let data: Vec<String> = file_data
            .iter()
            .map(|bytes: &Vec<u8>| String::from_utf8(bytes.clone()).expect("invalid UTF-8"))
            .collect();

        b.iter(|| {
            let results: Vec<ExtendedFloat<f64>> =
                data.iter().map(|line| line.parse().unwrap()).collect();
            black_box(results)
        });
    });

    group.finish();
}

#[cfg(feature = "comparison")]
fn bench_conversions_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("[f64]");

    for precision in PRECISIONS {
        let mut iter = RandomF64StringIterator::new(SEED, precision, -100_000.0..=100_000.0);
        group.bench_function(
            format!("parse from string {}_precision", precision).as_str(),
            |b| {
                b.iter_batched(
                    || iter.next().unwrap(),
                    |data| {
                        let result = data.parse::<f64>().unwrap();
                        black_box(result)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.bench_function("parse entire file", |b| {
        let file_data =
            read_lines_from_file("test_data/prices.log").expect("Failed to read data from file");
        let data: Vec<String> = file_data
            .iter()
            .map(|bytes: &Vec<u8>| String::from_utf8(bytes.clone()).expect("invalid UTF-8"))
            .collect();

        b.iter(|| {
            let results: Vec<f64> = data.iter().map(|line| line.parse().unwrap()).collect();
            black_box(results)
        });
    });

    group.finish();
}

#[cfg(feature = "comparison")]
fn bench_conversions_fastnum(c: &mut Criterion) {
    let mut group = c.benchmark_group("[fastnum]");

    for precision in PRECISIONS {
        let mut iter = RandomF64StringIterator::new(SEED, precision, -100_000.0..=100_000.0);
        group.bench_function(
            format!("parse from string {}_precision", precision).as_str(),
            |b| {
                b.iter_batched(
                    || iter.next().unwrap(),
                    |data| {
                        let result = fastnum::decimal::Decimal::<64>::from_str(
                            &data,
                            fastnum::decimal::Context::default(),
                        )
                        .unwrap();
                        black_box(result)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.bench_function("parse entire file", |b| {
        let file_data =
            read_lines_from_file("test_data/prices.log").expect("Failed to read data from file");
        let data: Vec<String> = file_data
            .iter()
            .map(|bytes: &Vec<u8>| String::from_utf8(bytes.clone()).expect("invalid UTF-8"))
            .collect();

        b.iter(|| {
            let results: Vec<fastnum::decimal::Decimal<64>> = data
                .iter()
                .map(|line| {
                    fastnum::decimal::Decimal::<64>::from_str(
                        line,
                        fastnum::decimal::Context::default(),
                    )
                    .unwrap()
                })
                .collect();
            black_box(results)
        });
    });

    group.finish();
}

#[cfg(not(feature = "comparison"))]
criterion_group!(benches, bench_conversions);

#[cfg(feature = "comparison")]
criterion_group!(
    benches,
    bench_conversions,
    bench_conversions_f64,
    bench_conversions_fastnum
);

criterion_main!(benches);
