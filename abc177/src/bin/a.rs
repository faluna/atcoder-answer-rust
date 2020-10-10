use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: i64,
        t: i64,
        s: i64,
    };

    println!("{}", if d <= t * s { "Yes" } else { "No" });
}
