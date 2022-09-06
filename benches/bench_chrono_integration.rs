use chrono::Duration;
use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use pyo3::prelude::*;

fn duration_to_py(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: i64 = 50_000;
        b.iter(|| (0..LEN).map(|i| Duration::from_millis(i as u64).into_py(py)));
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("duration_to_py", duration_to_py);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
