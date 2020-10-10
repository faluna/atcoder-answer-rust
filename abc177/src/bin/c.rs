use proconio::{fastout, input};

const MOD_NUM: i64 = 1_000_000_000 + 7;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
    };
    let mut a_acc = a_vec.iter().fold(0, |acc, a| (acc + a) % MOD_NUM);
    let mut ans = 0;
    for i in 0..a_vec.len() - 1 {
        a_acc = a_acc - a_vec[i];
        if a_acc < 0 {
            a_acc += MOD_NUM;
        }
        ans = (ans + (a_vec[i] * a_acc)) % MOD_NUM;
    }
    println!("{}", ans);
}
