use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let t: f64 = h + m / 60.;
    let omega_a = (2. * PI * t) / 12.;
    let omega_b = 2. * PI * t;
    let theta_a = omega_a * t;
    let theta_b = omega_b * t;
    let theta_ab = theta_b - theta_a;
    // if theta_a < theta_b {
    // theta_b - theta_a
    // } else {
    // theta_a - theta_b
    // };

    // let (a, b) = if theta_a < theta_b { (a, b) } else { (b, a) };
    let temp: f64 = a.powf(2.) - 2. * a * b * theta_ab.cos() + b.powf(2.);
    println!("{}", temp.sqrt());
}
