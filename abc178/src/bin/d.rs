use proconio::{fastout, input};

const MOD: i64 = 1_000_000_000 + 7;

#[fastout]
fn main() {
    input! {
        s: i64,
    };

    let mut dp: Vec<i64> = vec![0; s as usize + 1];

    let mut x = 0;
    dp[0] = 1;
    for i in 3..s as usize + 1 {
        x += dp[i - 3];
        x %= MOD;
        dp[i] = x;
    }
    println!("{}", dp[s as usize]);
}
