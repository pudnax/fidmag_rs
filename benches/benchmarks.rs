use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use fidmag_rs;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

fn assert_eq_vecs(a: &[Complex<f64>], b: &[Complex<f64>]) {
    for (a, b) in a.iter().zip(b) {
        assert!((a - b).norm() < 0.1f64);
    }
}

fn target_01(c: &mut Criterion) {
    c.bench_function("fft", |b| {
        b.iter(|| {
            let mut input: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
                .into_iter()
                .map(|x| Complex::new(x, 0.))
                .collect();
            let mut output = vec![Zero::zero(); 9];
            fidmag_rs::fft::fft(&mut input, &mut output);
            let expected = [
                Complex::new(45.0, 0.),
                Complex::new(-4.5, 12.363_648_39),
                Complex::new(-4.5, 5.362_891_17),
                Complex::new(-4.5, 2.598_076_21),
                Complex::new(-4.5, 0.793_471_41),
                Complex::new(-4.5, -0.793_471_41),
                Complex::new(-4.5, -2.598_076_21),
                Complex::new(-4.5, -5.362_891_17),
                Complex::new(-4.5, -12.363_648_39),
            ];
            assert_eq_vecs(&expected, &output);
        })
    });
}

criterion_group! {
name = group;
config = Criterion::default().warm_up_time(Duration::from_secs(5));
targets = target_01
}

criterion_main!(group);
