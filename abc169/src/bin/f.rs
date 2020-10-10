use proconio::{fastout, input};

const MOD_NUM: i64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: i64,
        a_vec: [i64; n],
    }
    let mut dp: Vec<i64> = vec![0; s as usize + 1];
    dp[0] = 1;

    for i in 0..n {
        let mut p: Vec<i64> = vec![0; s as usize + 1];
        std::mem::swap(&mut dp, &mut p);
        for j in 0..(s as usize + 1) {
            dp[j] += p[j] * 2;
            dp[j] %= MOD_NUM;
            if j as i64 + a_vec[i] <= s {
                dp[j + a_vec[i] as usize] += p[j];
                dp[j + a_vec[i] as usize] %= MOD_NUM;
            }
        }
    }
    println!("{}", dp[s as usize])
}
