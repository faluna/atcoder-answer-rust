use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_vec: [i64; 3*n],
    };
    let a_vec: Vec<i64> = a_vec.iter().map(|a| a - 1).collect();
    let mut dp: Vec<Vec<i64>> = vec![vec![i64::MIN; n + 1]; n + 1];
    let upd = |dp: &mut Vec<Vec<i64>>, i: usize, j: usize, x: i64| {
        dp[i][j] = dp[i][j].max(x);
        dp[j][i] = dp[j][i].max(x);
        dp[i][n] = dp[i][n].max(x);
        dp[n][i] = dp[n][i].max(x);
        dp[j][n] = dp[j][n].max(x);
        dp[n][j] = dp[n][j].max(x);
        dp[n][n] = dp[n][n].max(x);
    };
    upd(&mut dp, a_vec[0] as usize, a_vec[1] as usize, 0);
    let mut base = 0;
    for i in 0..n - 1 {
        let i1 = 2 + 3 * i;
        let mut x = a_vec[i1];
        let mut y = a_vec[i1 + 1];
        let mut z = a_vec[i1 + 2];
        if x == y && y == z {
            base += 1;
            continue;
        }
        let mut q: VecDeque<(usize, usize, i64)> = VecDeque::new();
        for _ in 0..3 {
            for j in 0..n + 1 {
                let mut now = dp[j][n];
                if y == z {
                    now = now.max(dp[j][y as usize] + 1);
                }
                q.push_back((j, x as usize, now));
            }
            let now = std::cmp::max(dp[n][n], dp[z as usize][z as usize] + 1);
            q.push_back((x as usize, y as usize, now));
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut y, &mut z);
        }
        while let Some(t) = q.pop_front() {
            let (i, j, x) = t;
            upd(&mut dp, i, j, x);
        }
    }
    let mut ans = dp[n][n];
    let l = a_vec[n * 3 - 1] as usize;
    ans = ans.max(dp[l][l] + 1);
    ans += base;
    println!("{}", ans);
}
