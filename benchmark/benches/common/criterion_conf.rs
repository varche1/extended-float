use criterion::Criterion;

pub fn configure_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(1000))
        .measurement_time(std::time::Duration::from_millis(2000))
}
