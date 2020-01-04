use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use fidmag_rs;

fn target_01(c: &mut Criterion) {
    c.bench_function("f_func", |b| b.iter(|| {}));
}

criterion_group! {
name = group;
config = Criterion::default().warm_up_time(Duration::from_secs(5));
targets = target_01
}

criterion_main!(group);
