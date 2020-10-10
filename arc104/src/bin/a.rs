use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    };

    let x: i64 = (a + b) / 2;

    println!("{} {}", x, a - x);
}
