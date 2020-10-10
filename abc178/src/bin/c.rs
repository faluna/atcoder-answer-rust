use proconio::{fastout, input};

const MOD: i64 = 1_000_000_000 + 7;

#[fastout]
fn main() {
    input! {
        n: i64,
    };
    let ans = mpow(10, n as u64) - mpow(9, n as u64) - mpow(9, n as u64) + mpow(8, n as u64);
    let ans = ans % MOD;
    let ans = (ans + MOD) % MOD;
    println!("{}", ans);
}

fn mpow(n: i64, mut k: u64) -> i64 {
    let mut ans = 1;
    let mut temp = n;

    while k > 0 {
        if (k & 1) == 1 {
            ans = ans * temp % MOD;
        }
        temp = temp * temp % MOD;
        k >>= 1;
    }

    ans
}
