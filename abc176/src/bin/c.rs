use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a_vec: [i64; n],
    };

    let mut ans: i64 = 0;
    for i in 1..n {
        let dif = a_vec[i - 1] - a_vec[i];
        if dif > 0 {
            ans += dif;
            a_vec[i] += dif;
        }
    }
    println!("{}", ans);
}
