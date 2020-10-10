use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        u_v_vec: [(i64, i64); n - 1],
    };
    let mut v_count: i64 = 0;
    for i in 1..=n {
        v_count += i as i64 * (n - i + 1) as i64;
    }
    let mut e_count: i64 = 0;
    u_v_vec.iter().for_each(|(u, v)| {
        let (a, b): (i64, i64);
        if u > v {
            a = *v;
            b = *u;
        } else {
            a = *u;
            b = *v;
        }
        e_count += a * (n as i64 - b + 1);
    });
    let ans = v_count - e_count;
    println!("{}", ans);
}
