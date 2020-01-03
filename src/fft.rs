extern crate ndarray;
extern crate rustfft;

use ndarray::{ArrayViewMut, ArrayViewMut2, Dimension};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::{FFTnum, FFTplanner};

fn fast_fourier_transform_planner<T: FFTnum>(
    input: &mut [Complex<T>],
    output: &mut [Complex<T>],
    inverse: bool,
) {
    let mut planner = FFTplanner::new(inverse);
    let len = input.len();
    let fft = planner.plan_fft(len);
    fft.process(input, output);
}

pub fn fft<T: FFTnum>(input: &mut [Complex<T>], output: &mut [Complex<T>]) {
    fast_fourier_transform_planner(input, output, false);
}

pub fn ifft<T: FFTnum + From<u32>>(input: &mut [Complex<T>], output: &mut [Complex<T>]) {
    fast_fourier_transform_planner(input, output, true);
    output.iter_mut().for_each(|v| {
        *v = v.unscale(T::from(input.len() as u32));
    });
}

fn mutate_lane<T: Zero + Clone, D: Dimension>(
    input: &mut ArrayViewMut<T, D>,
    output: &mut ArrayViewMut<T, D>,
    f: impl Fn(&mut [T], &mut [T]),
    axis: usize,
) {
    if axis > 0 {
        input.swap_axes(0, axis);
        output.swap_axes(0, axis);

        let mut outrows = output.genrows_mut().into_iter();
        for row in input.genrows_mut() {
            let mut outrow = outrows.next().unwrap();
            let mut vec = row.to_vec();
            let mut out = vec![Zero::zero(); outrow.len()];
            f(&mut vec, &mut out);
            for i in 0..outrow.len() {
                outrow[i] = out.remove(0);
            }
        }

        input.swap_axes(0, axis);
        output.swap_axes(0, axis);
    } else {
        let mut outrows = output.genrows_mut().into_iter();
        for mut row in input.genrows_mut() {
            let mut outrow = outrows.next().unwrap();
            f(
                &mut row.as_slice_mut().unwrap(),
                &mut outrow.as_slice_mut().unwrap(),
            );
        }
    }
}

fn _fftn<D: Dimension>(
    input: &mut ArrayViewMut<Complex<f64>, D>,
    output: &mut ArrayViewMut<Complex<f64>, D>,
    axis: usize,
    inverse: bool,
) {
    if inverse {
        mutate_lane(input, output, ifft, axis)
    } else {
        mutate_lane(input, output, fft, axis)
    }
}

pub fn fftn<D: Dimension>(
    input: &mut ArrayViewMut<Complex<f64>, D>,
    output: &mut ArrayViewMut<Complex<f64>, D>,
    axis: usize,
) {
    _fftn(input, output, axis, false);
}

pub fn ifftn<D: Dimension>(
    input: &mut ArrayViewMut<Complex<f64>, D>,
    output: &mut ArrayViewMut<Complex<f64>, D>,
    axis: usize,
) {
    _fftn(input, output, axis, true);
}

fn _fftnd<D: Dimension>(
    input: &mut ArrayViewMut<Complex<f64>, D>,
    output: &mut ArrayViewMut<Complex<f64>, D>,
    axes: &[usize],
    inverse: bool,
) {
    let len = axes.len();
    for i in 0..len {
        let axis = axes[i];
        _fftn(input, output, axis, inverse);
        if i < len - 1 {
            let mut outrows = output.genrows_mut().into_iter();
            for mut row in input.genrows_mut() {
                let mut outrow = outrows.next().unwrap();
                row.as_slice_mut()
                    .unwrap()
                    .copy_from_slice(outrow.as_slice_mut().unwrap());
            }
        }
    }
}

pub fn fftnd<D: Dimension>(
    input: &mut ArrayViewMut<Complex<f64>, D>,
    output: &mut ArrayViewMut<Complex<f64>, D>,
    axes: &[usize],
) {
    _fftnd(input, output, axes, false);
}

pub fn ifftnd<D: Dimension>(
    input: &mut ArrayViewMut<Complex<f64>, D>,
    output: &mut ArrayViewMut<Complex<f64>, D>,
    axes: &[usize],
) {
    _fftnd(input, output, axes, true);
}

pub fn fft2(input: &mut ArrayViewMut2<Complex<f64>>, output: &mut ArrayViewMut2<Complex<f64>>) {
    fftnd(input, output, &[0, 1]);
}

pub fn ifft2(input: &mut ArrayViewMut2<Complex<f64>>, output: &mut ArrayViewMut2<Complex<f64>>) {
    ifftnd(input, output, &[1, 0]);
}

#[cfg(test)]
mod tests {
    use super::{fft, fft2, ifft, ifft2};
    use ndarray::ArrayViewMut;
    use rustfft::num_complex::Complex;
    use rustfft::num_traits::Zero;

    fn assert_eq_vecs(a: &[Complex<f64>], b: &[Complex<f64>]) {
        for (a, b) in a.iter().zip(b) {
            assert!((a - b).norm() < 0.1f64);
        }
    }

    #[test]
    fn test_fft() {
        let mut input: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
            .into_iter()
            .map(|x| Complex::new(x, 0.))
            .collect();
        let mut output = vec![Zero::zero(); 9];
        fft(&mut input, &mut output);
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
    }

    #[test]
    fn test_inverse_fft() {
        let mut input: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
            .into_iter()
            .map(|x| Complex::new(x, 0.))
            .collect();
        let expected = input.clone();
        let mut output = vec![Zero::zero(); 9];
        fft(&mut input, &mut output);
        let mut output2 = vec![Zero::zero(); 9];
        ifft(&mut output, &mut output2);
        assert_eq_vecs(&expected, &output2);
    }

    #[test]
    fn test_fft2() {
        let mut input: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
            .into_iter()
            .map(|x| Complex::new(x, 0.))
            .collect();
        let mut input_view = ArrayViewMut::from_shape((3, 3), &mut input).unwrap();

        let mut output = vec![Zero::zero(); 9];
        let mut output_view = ArrayViewMut::from_shape((3, 3), &mut output).unwrap();
        fft2(&mut input_view, &mut output_view);

        let expected = [
            Complex::new(45.0, 0.),
            Complex::new(-4.5, 2.598_076_21),
            Complex::new(-4.5, -2.598_076_21),
            Complex::new(-13.5, 7.794_228_63),
            Complex::new(0.0, 0.),
            Complex::new(0.0, 0.),
            Complex::new(-13.5, -7.794_228_63),
            Complex::new(0.0, 0.),
            Complex::new(0.0, 0.),
        ];
        assert_eq_vecs(&expected, &output);
    }

    #[test]
    fn test_inverse_fft2() {
        let mut input: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
            .into_iter()
            .map(|x| Complex::new(x, 0.))
            .collect();
        let mut input_view = ArrayViewMut::from_shape((3, 3), &mut input).unwrap();

        let mut output = vec![Zero::zero(); 9];
        let mut output_view = ArrayViewMut::from_shape((3, 3), &mut output).unwrap();

        fft2(&mut input_view, &mut output_view);

        let mut output2 = vec![Zero::zero(); 9];
        let mut output2_view = ArrayViewMut::from_shape((3, 3), &mut output2).unwrap();
        ifft2(&mut output_view, &mut output2_view);

        let expected: Vec<Complex<f64>> = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.]
            .into_iter()
            .map(|x| Complex::new(x, 0.))
            .collect();
        assert_eq_vecs(&expected, &output2);
    }
}
