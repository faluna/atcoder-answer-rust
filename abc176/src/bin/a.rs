use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        x: i64,
        t: i64,
    };
    println!("{}", (n / x + if n % x == 0 { 0 } else { 1 }) * t);
}
