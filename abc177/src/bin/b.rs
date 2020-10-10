use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    };
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let s_len = s_chars.len();
    let t_len = t_chars.len();
    // let mut dp = vec![vec![0; t_chars.len() + 1]; s_chars.len() + 1];
    let mut ans = -10000000;
    for i in 0..s_len - t_len + 1 {
        let mut temp = 0;
        for j in 0..t_len {
            if s_chars[i + j] == t_chars[j] {
                temp += 1;
            }
        }
        ans = ans.max(temp);
    }
    println!("{}", t_len as i64 - ans);

    // for i in 0..t_len {
    //     for j in i..s_len - t_len + 1 + i {
    //         dp[j + 1][i + 1] = dp[j][i] + if t_chars[i] == s_chars[j] { 1 } else { 0 };
    //     }
    // }
    // println!("{}", t_len as i64 - *dp[s_len].iter().max().unwrap());
}
