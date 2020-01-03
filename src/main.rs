fn main() {
    println!("Hello, world!");
}

use std::f64::consts::PI;
const n: (f64, f64, f64) = (100., 25., 1.);
const mu0: f64 = 4e-7 * PI;
const gamma: f64 = 2.211e5;
const ms: f64 = 8e5;
const A: f64 = 1.3e-11;
const alpha: f64 = 0.02;

const eps: f64 = 1e-18;

fn f(p: [f64; 3]) -> f64 {
    let mut p = p;
    p.iter_mut().for_each(|x| {
        *x = x.abs();
    });
    let [x, y, z] = p;

    y / 2.0 * (z * z - x * x) * (y / ((x * x + z * z).sqrt() + eps)).asinh()
        + z / 2.0 * (y * y - x * x) * (z / ((x * x + y * y).sqrt() + eps)).asinh()
        - x * y * z * (y * z / (x * (x * x + y * y + z * z).sqrt() + eps)).atanh()
        + 1.0 / 6.0 * (2. * x * x - y * y - z * z) * (x * x + y * y + z * z).sqrt()
}

fn g(p: [f64; 3]) -> f64 {
    let [x, y, z] = p;
    let z = z.abs();

    x * y * z * (z / ((x * z + y * y).sqrt() + eps)).asinh()
        + y / 6.0 * (3.0 * z * z - y * y) * (x / ((y * y + z * z).sqrt() + eps)).asinh()
        + x / 6.0 * (3.0 * z * z - x * z) * (y / ((x * z + z * z).sqrt() + eps)).asinh()
        - z * z / 6.0 * (x * y / (z * (x * z + y * y + z * z).sqrt() + eps)).atan()
        - z * y * y / 2.0 * (x * z / (y * (x * x + y * y + z * z).sqrt() + eps)).atan()
        - z * x * x / 2.0 * (y * z / (x * (x * x + y * y + z * z).sqrt() + eps)).atan()
        - x * y * (x * x + y * y + z * z).sqrt() / 3.0
}
