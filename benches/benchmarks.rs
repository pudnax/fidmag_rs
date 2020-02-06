use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use fidmag_rs;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

fn target_01(c: &mut Criterion) {
    c.bench_function("f_func", |b| {
        b.iter(|| {
            let size = 500;
            let mut input: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
                .into_iter()
                .map(|x| Complex::new(x, 0.))
                .collect();
            let mut output = vec![Zero::zero(); size];
            fidmag_rs::fft::fft(&mut input, &mut output);
        })
    });
}

criterion_group! {
name = group;
config = Criterion::default().warm_up_time(Duration::from_secs(3));
targets = target_01
}

criterion_main!(group);
