use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc, FixedOffset, TimeZone};
use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use pyo3::prelude::*;

fn duration_to_py(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: i64 = 50_000;
        b.iter(|| (0..LEN).map(|i| Duration::milliseconds(i).into_py(py)));
    });
}

fn naive_date_to_py(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: i32 = 50_000;
        b.iter(|| (0..LEN).map(|i| NaiveDate::from_num_days_from_ce(i).into_py(py)));
    });
}

fn naive_time_to_py(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: u32 = 50_000;
        b.iter(|| (0..LEN).map(|i| NaiveTime::from_num_seconds_from_midnight(i, 0).into_py(py)));
    });
}

fn datetime_to_py(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: i64 = 50_000;
        b.iter(|| {
            (0..LEN).map(|i| {
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(i, 0), Utc).into_py(py)
            })
        });
    });
}

fn datetime_fixed_offset_to_py(b: &mut Bencher<'_>) {
    Python::with_gil(|py| {
        const LEN: i32 = 50_000;
        b.iter(|| {
            (0..LEN).map(|i| {
                FixedOffset::east(i)
                    .ymd(2000, 1, 1)
                    .and_hms(0, 0, 0)
                    .into_py(py)
            })
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("duration_to_py", duration_to_py);
    c.bench_function("naive_date_to_py", naive_date_to_py);
    c.bench_function("naive_time_to_py", naive_time_to_py);
    c.bench_function("datetime_to_py", datetime_to_py);
    c.bench_function("datetime_fixed_offset_to_py", datetime_fixed_offset_to_py);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
