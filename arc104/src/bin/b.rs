use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    };
    let s: Vec<char> = s.chars().collect();

    let mut dp: Vec<i64> = vec![0; n + 1];

    dp[1] = match (s[0], s[1]) {
        ('A', 'T') | ('T', 'A') | ('C', 'G') | ('G', 'C') => 1,
        _ => 0,
    };
    let mut pre: char = s[1];
    for i in 2..n {
        dp[i] = match (pre, s[i]) {
            ('A', 'T') | ('T', 'A') | ('C', 'G') | ('G', 'C') => 1 + dp[i - 2],
            _ => dp[i - 1],
        };
        pre = s[i];
    }

    println!("{}", dp[n - 1]);
}
