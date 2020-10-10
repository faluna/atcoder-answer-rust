use proconio::{fastout, input};

const MOD: i64 = 1_000_000_000 + 7;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a_vec: [i64; n],
    };
    let (mut s, mut t): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new());
    a_vec.iter().for_each(|a| {
        if *a >= 0 {
            s.push(*a);
        } else {
            t.push(*a);
        }
    });
    let s_len = s.len();
    let t_len = t.len();
    let mut _ok: bool = false;
    if s_len > 0 {
        if n == k {
            _ok = t_len % 2 == 0;
        } else {
            _ok = true;
        }
    } else {
        _ok = k % 2 == 0;
    }

    let mut ans = 1;
    if !_ok {
        a_vec.sort_by_key(|a| a.abs());
        for i in 0..k {
            ans *= a_vec[i];
        }
        ans = (ans % MOD) + MOD;
    } else {
        s.sort();
        t.sort_by(|a, b| b.cmp(a));
        if k % 2 == 1 {
            ans *= s.pop().expect("");
            ans %= MOD;
        }
        let mut p: Vec<i64> = Vec::new();
        while s.len() >= 2 {
            let x = s.pop().expect("") * s.pop().expect("") % MOD;
            p.push(x);
        }
        while t.len() >= 2 {
            let x = t.pop().expect("") * t.pop().expect("") % MOD;
            p.push(x);
        }
        p.sort_by(|a, b| b.cmp(a));
        for i in 0..(k / 2) {
            ans *= p[i];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
