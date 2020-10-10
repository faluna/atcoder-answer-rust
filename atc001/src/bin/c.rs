use num::complex::Complex;
use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_b_vec: [(i64, i64); n],
    };
    // println!("{:?}", a_b_vec);
    println!("0");
    let mut a_vec: Vec<i64> = vec![0];
    a_vec.append(&mut a_b_vec.iter().map(|(a, _)| *a).collect::<Vec<i64>>());
    let mut b_vec: Vec<i64> = vec![0];
    b_vec.append(&mut a_b_vec.iter().map(|(_, b)| *b).collect::<Vec<i64>>());

    let ans = convolve(&mut a_vec, &mut b_vec);
    let mut ans_iter = ans.iter();
    while let Some(val) = ans_iter.next() {
        println!("{}", val);
    }
}

/// Reverse a number displayed binary.
/// e.g. 0001 -> 1000, 1101 -> 1011
///
/// # Arguments
///
/// * `i` - A number made reversed
/// * `h` - Number of binary digits
///
/// # Example
/// ```
/// let x: usize = 6;
/// assert_eq!(reverse(x, 3), 3);
/// ```
///
fn reverse(i: usize, h: usize) -> usize {
    let mut j: usize = 0;
    for k in 0..h {
        j |= (i >> k & 1) << (h - 1 - k);
    }
    j
}

fn _fft(a: &mut [Complex<f64>]) -> &[Complex<f64>] {
    let n = a.len();
    let mut h = 0;
    while 1 << h < n {
        h += 1;
    }
    for i in 0..n {
        let j = reverse(i, h);
        if i < j {
            a.swap(i, j);
        }
    }
    let mut b = 1;
    while b < n {
        for j in 0..b {
            let w: Complex<f64> =
                Complex::from_polar(&1.0f64, &(-(2. * PI) / (2. * b as f64) * j as f64));
            let mut k = 0;
            while k < n {
                let s: Complex<f64> = *a.get(j + k).expect("Error of getting a[j + k]");
                let t: Complex<f64> = a.get(j + k + b).expect("Error of getting a[j + k +b]") * w;
                *a.get_mut(j + k).expect("error of getting mutable a[j + k]") = s + t;
                *a.get_mut(j + k + b)
                    .expect("error of getting mutable a[j + k + b]") = s - t;
                k += b * 2;
            }
        }
        b *= 2;
    }
    a
}

fn _ifft(a: &mut [Complex<f64>]) -> &[Complex<f64>] {
    let n = a.len();
    let mut h = 0;
    while 1 << h < n {
        h += 1;
    }
    for i in 0..n {
        let j = reverse(i, h);
        if i < j {
            a.swap(i, j);
        }
    }
    let mut b = 1;
    while b < n {
        for j in 0..b {
            let w: Complex<f64> =
                Complex::from_polar(&1.0f64, &((2. * PI) / (2. * b as f64) * j as f64));
            let mut k = 0;
            while k < n {
                let s: Complex<f64> = *a.get(j + k).expect("Error of getting a[j + k]");
                let t: Complex<f64> = a.get(j + k + b).expect("Error of getting a[j + k +b]") * w;
                *a.get_mut(j + k).expect("error of getting mutable a[j + k]") = s + t;
                *a.get_mut(j + k + b)
                    .expect("error of getting mutable a[j + k + b]") = s - t;
                k += b * 2;
            }
        }
        b *= 2;
    }
    a.iter_mut()
        .for_each(|val| *val /= Complex::new(n as f64, 0.));
    a
}

fn fft(a: Vec<i64>) -> Vec<Complex<f64>> {
    let mut a_complex: Vec<Complex<f64>> = a
        .into_iter()
        .map(|val| Complex::new(val as f64, 0.))
        .collect();
    let a_complex = _fft(a_complex.as_mut_slice());
    a_complex.to_vec()
}

fn ifft(a: Vec<f64>) -> Vec<Complex<f64>> {
    let mut a_complex: Vec<Complex<f64>> = a.into_iter().map(|val| Complex::new(val, 0.)).collect();
    let a_complex = _ifft(a_complex.as_mut_slice());
    a_complex.to_vec()
}

fn convolve(a: &mut Vec<i64>, b: &mut Vec<i64>) -> Vec<i64> {
    let s = a.len() + b.len() - 1;
    let mut t = 1;
    while t < s {
        t *= 2;
    }
    a.resize(t, 0);
    b.resize(t, 0);
    let a_fft = fft(a.clone());
    let b_fft = fft(b.clone());
    let mut c_fft: Vec<Complex<f64>> = a_fft.iter().zip(b_fft.iter()).map(|(a, b)| a * b).collect();
    let c = _ifft(&mut c_fft);
    c.iter().map(|c| (c.re + 0.5) as i64).collect()
}
