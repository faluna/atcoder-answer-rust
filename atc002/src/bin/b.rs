use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        m: i64,
        p: i64,
    };

    let mut n_temp: i64 = n % m;
    let mut i: usize = 0;
    let mut ans: i64 = 1;
    while p >> i > 0 {
        if p >> i & 1 == 1 {
            ans = (ans * n_temp) % m;
        }
        n_temp = (n_temp * n_temp) % m;
        i += 1;
    }
    println!("{}", ans);
}
