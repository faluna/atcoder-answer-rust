use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: i64,
        mut k: i64,
        d: i64,
    };
    if x < 0 {
        x *= -1;
    };
    let alpha = x / d;
    // let beta = x % d;
    let beta = x - alpha * d;
    println!(
        "{}",
        if k <= alpha {
            x - k * d
        } else {
            if (k - alpha) % 2 == 0 {
                beta
            } else {
                d - beta
            }
        }
    )
}
