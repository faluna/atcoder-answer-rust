use proconio::{fastout, input};

const MOD: i64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        l_r_vec: [(usize, usize); k],
    };
    let mut dp = vec![0; n];
    dp[0] = 1;
    dp[1] = -1 + MOD;
    for i in 0..n {
        if i > 0 {
            dp[i] = (dp[i] + dp[i - 1]) % MOD;
        }
        for (l, r) in l_r_vec.iter() {
            if i + l < n {
                dp[i + l] = (dp[i + l] + dp[i]) % MOD;
            }
            if i + r + 1 < n {
                dp[i + r + 1] = (dp[i + r + 1] - dp[i] + MOD) % MOD;
            }
        }
    }
    println!("{}", dp[n - 1]);
}
