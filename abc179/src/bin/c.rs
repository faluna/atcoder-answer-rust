use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0;
    for i in 1..=n - 1 {
        ans += (n - 1) / i;
    }
    println!("{}", ans);
}
