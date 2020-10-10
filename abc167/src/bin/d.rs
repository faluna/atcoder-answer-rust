use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        to_0: [i64; n],
    };
    let to_0 = to_0.iter().map(|x| *x - 1).collect();
    let mut k = k;
    let mut to: Vec<Vec<i64>> = Vec::new();
    to.push(to_0);
    for _ in 1..60 {
        to.push(vec![-1; n]);
    }
    for i in 0..(60 - 1) {
        for j in 0..n {
            to[i + 1][j] = to[i][to[i][j] as usize];
        }
    }

    let mut v: i64 = 0;
    for i in (0..60).rev() {
        let l = 1 << i;
        if l <= k {
            v = to[i][v as usize] as i64;
            k -= l;
        }
    }
    println!("{}", v + 1)
}
