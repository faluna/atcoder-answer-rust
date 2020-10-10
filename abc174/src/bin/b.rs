use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        x_y: [(i64, i64); n],
    };
    let d2 = d.pow(2);
    let mut ans = 0;
    for (x, y) in x_y.iter() {
        if x.pow(2) + y.pow(2) - d2 <= 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
