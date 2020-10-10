use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        row: usize,
        column: usize,
        n: usize,
        r_c_v_vec: [(usize, usize, i64); n],
    };
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 4]; column]; row];
    let mut v_map: Vec<Vec<i64>> = vec![vec![0; column]; row];
    for (r, c, v) in r_c_v_vec {
        v_map[r - 1][c - 1] = v;
    }
    for i in 0..row {
        for j in 0..column {
            for k in 0..4 {
                if i > 0 {
                    dp[i][j][0] = dp[i][j][0].max(dp[i - 1][j][k]);
                }
                if j > 0 {
                    if k < 3 {
                        if v_map[i][j] > 0 {
                            dp[i][j][k + 1] = dp[i][j][k + 1].max(dp[i][j - 1][k] + v_map[i][j]);
                        }
                    } else {
                        dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k]);
                    }
                }
            }
        }
    }
    println!("{}", dp[row - 1][column - 1].iter().max().unwrap());
}
