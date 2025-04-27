use criterion::{Criterion, criterion_group, criterion_main};
use extended_float::types::ExtendedFloat;

// TODO: refactor it to new approach
fn bench_ops(c: &mut Criterion) {
    for i in [1, 5, 10, 15, 20] {
        // for i in [5] {
        c.bench_function(format!("bench to_string {}_decimals", i).as_str(), |b| {
            // TODO: iter_with_setup is deprecated now
            b.iter_with_setup(
                || ExtendedFloat::from(4.0 + 3.0 / 10.0_f64.powf(i as f64)),
                |param| param.format(),
            )
        });
    }
}

criterion_group!(benches, bench_ops);
criterion_main!(benches);
