use std::hint::black_box;

mod common;
use common::files::read_file_as_strings;
use extended_float::types::ExtendedFloat;
use iai_callgrind::{LibraryBenchmarkConfig, library_benchmark, library_benchmark_group, main};

#[library_benchmark]
#[bench::zero("0")]
#[bench::zero_2("0.0")]
#[bench::size_1("1.2")]
#[bench::size_3("123.456")]
#[bench::size_5("12345.67890")]
#[bench::size_10("12345.1234567890")]
fn from_str_simple(input: &str) -> ExtendedFloat<f64> {
    black_box(input.parse::<ExtendedFloat<f64>>().unwrap())
}

fn setup_from_file_100_k_sum(path: &str) -> Vec<String> {
    read_file_as_strings(path).expect("failed to read data file")
}

#[library_benchmark]
#[bench::from_file_100_k_sum(
    args = ("test_data/prices.log"),
    setup = setup_from_file_100_k_sum
)]
fn bench_parse_file_sum(lines: Vec<String>) -> ExtendedFloat<f64> {
    let mut sum = ExtendedFloat::<f64>::default();
    for line in black_box(&lines) {
        let x: ExtendedFloat<f64> = black_box(line.parse().unwrap());
        sum = black_box(sum + x);
    }
    black_box(sum)
}

fn setup_from_file_100_k_allocate(path: &str) -> (Vec<String>, Vec<ExtendedFloat<f64>>) {
    let lines = read_file_as_strings(path).expect("failed to read data file");
    let values = vec![ExtendedFloat::<f64>::default(); lines.len()];
    (lines, values)
}

#[library_benchmark]
#[bench::from_file_100_k_allocate(
    args = ("test_data/prices.log"),
    setup = setup_from_file_100_k_allocate
)]
fn bench_parse_file_allocate(
    data: (Vec<String>, Vec<ExtendedFloat<f64>>),
) -> Vec<ExtendedFloat<f64>> {
    let (lines, mut values) = data;
    for (i, line) in black_box(&lines).iter().enumerate() {
        values[i] = black_box(line.parse::<ExtendedFloat<f64>>().unwrap());
    }
    black_box(values)
}

library_benchmark_group!(
    name = from_str_group;
    benchmarks = from_str_simple, bench_parse_file_sum, bench_parse_file_allocate
);

main!(
    config = LibraryBenchmarkConfig::default().callgrind_args(["collect-jumps=yes"]);
    library_benchmark_groups = from_str_group
);
