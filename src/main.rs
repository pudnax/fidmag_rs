// #![feature(slice_patterns)]
use ndarray::prelude::*;

use itertools::Itertools;

use std::f64::consts::PI;
const N: [usize; 3] = [100, 25, 1];
static DEMAG_DIM: [usize; 4] = [2 * N[0] - 1, 2 * N[1] - 1, 2 * N[2] - 1, 6];
const MU0: f64 = 4e-7 * PI;
const GAMMA: f64 = 2.211e5;
const MS: f64 = 8e5;
const A: f64 = 1.3e-11;
const ALPHA: f64 = 0.02;

const EPS: f64 = std::f64::EPSILON;
// const EPS: f64 = 1e-18;

use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n_demag = Array::<f64, _>::zeros(DEMAG_DIM);
    let mut n_demag = n_demag.view_mut();

    for (i, t) in [
        (f as fn(&[_]) -> _, [0usize, 1, 2]),
        (g, [0, 1, 2]),
        (g, [0, 2, 1]),
        (f, [1, 2, 0]),
        (g, [1, 2, 0]),
        (f, [2, 0, 1]),
    ]
    .iter()
    .enumerate()
    {
        set_n_demag(&mut n_demag, i, t.0, t.1);
        // println!("{}: {:?}", i, t.1);
    }

    Ok(())
}

fn set_n_demag<N, T>(
    demag: &mut ArrayViewMut<N, T>,
    c: usize,
    func: impl Fn(&[f64]) -> f64,
    permute: [usize; 3],
) {
}

fn f(p: &[f64]) -> f64 {
    let [x, y, z] = [p[0].abs(), p[1].abs(), p[2].abs()];
    // let (x, y, z) = match p {
    //     [x, y, z, ..] => (x.abs(), y.abs(), z.abs()),
    //     _ => panic!(),
    // };

    y / 2.0 * (z * z - x * x) * (y / ((x * x + z * z).sqrt() + EPS)).asinh()
        + z / 2.0 * (y * y - x * x) * (z / ((x * x + y * y).sqrt() + EPS)).asinh()
        - x * y * z * (y * z / (x * (x * x + y * y + z * z).sqrt() + EPS)).atanh()
        + 1.0 / 6.0 * (2. * x * x - y * y - z * z) * (x * x + y * y + z * z).sqrt()
}

fn g(p: &[f64]) -> f64 {
    let [x, y, z] = [p[0], p[1], p[2].abs()];
    // let (x, y, z) = match p {
    //     [x, y, z, ..] => (x, y, z.abs()),
    //     _ => panic!(),
    // };

    x * y * z * (z / ((x * x + y * y).sqrt() + EPS)).asinh()
        + y / 6.0 * (3.0 * z * z - y * y) * (x / ((y * y + z * z).sqrt() + EPS)).asinh()
        + x / 6.0 * (3.0 * z * z - x * x) * (y / ((x * x + z * z).sqrt() + EPS)).asinh()
        - z * z * z / 6.0 * (x * y / (z * (x * x + y * y + z * z).sqrt() + EPS)).atan()
        - z * y * y / 2.0 * (x * z / (y * (x * x + y * y + z * z).sqrt() + EPS)).atan()
        - z * x * x / 2.0 * (y * z / (x * (x * x + y * y + z * z).sqrt() + EPS)).atan()
        - x * y * (x * x + y * y + z * z).sqrt() / 3.0
}

#[cfg(test)]
mod test {
    use super::{f, g};

    fn assert_float(a: f64, b: f64) {
        assert!((a - b).abs() < 2. * std::f64::EPSILON);
    }

    #[test]
    fn test_f() {
        let test_cases = &[
            ([0., 0., 0.], 0.),
            ([-0.1, -0.1, -0.1], -0.000_658_478_948_462_408_4),
            ([0.1, 0.1, 0.1], -0.000_658_478_948_462_408_4),
            ([-1., -1., -1.], -0.658_478_948_462_408_5),
            ([1., 1., 1.], -0.658_478_948_462_408_5),
        ];

        for (input, expected) in test_cases {
            assert_float(f(input), *expected);
        }
    }

    #[test]
    fn test_g() {
        let test_cases = &[
            ([0., 0., 0.], 0.),
            ([-0.1, -0.1, -0.1], -0.000_090_750_593_283_627_15),
            ([0.1, 0.1, 0.1], -0.000_090_750_593_283_627_15),
            ([-1., -1., -1.], -0.090_750_593_283_627_22),
            ([1., 1., 1.], -0.090_750_593_283_627_22),
        ];

        for (input, expected) in test_cases {
            assert_float(g(input), *expected);
        }
    }
}

struct BinarySeq {
    num: usize,
    counter: usize,
}

impl BinarySeq {
    fn new(num: usize) -> BinarySeq {
        BinarySeq {
            num: num - 1,
            counter: 0,
        }
    }
}

impl Iterator for BinarySeq {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.counter < (1 << (self.num + 1)) {
            Some(
                (0..self.num)
                    .enumerate()
                    .map(|(i, _)| self.counter / (1 << self.num - i) % 2)
                    .collect(),
            )
        } else {
            self.counter = 0;
            None
        };
        self.counter += 1;
        res
    }
}
