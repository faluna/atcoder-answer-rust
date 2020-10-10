use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(i64, i64); q],
    };
    let mut w_row = n as i64;
    let mut w_col = n as i64;
    let mut lost_black = 0;

    for query in queries {
        lost_black += match query {
            (1, x) => {
                w_col = w_col.min(x);
                w_row - 2
            }
            (2, x) => {
                w_row = w_row.min(x);
                w_col - 2
            }
            _ => 0,
        };
        println!("{}", lost_black);
    }
    let ans = (n as i64 - 2) * (n as i64 - 2) - lost_black;
    println!("{}", ans);
}
