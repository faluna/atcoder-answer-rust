use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a_vec: [i64; n],
    }
    let (mut left, mut right) = (0, 1_000_000_000);

    while right - left > 1 {
        let x = (left + right) / 2;
        let f = |query: i64| {
            let mut now = 0;
            for i in 0..n {
                now += (a_vec[i] - 1) / query;
            }
            now <= k
        };
        if f(x) {
            right = x;
        } else {
            left = x;
        }
    }
    println!("{}", right);
}
