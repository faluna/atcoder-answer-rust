use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: f64,
    }
    let int_b = (b * 100. + 0.5) as u64;
    let ans: u64 = a * int_b;
    let ans: u64 = ans / 100;

    println!("{}", ans)
}
