use proconio::fastout;
use text_io::read;

#[fastout]
fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let x: i64 = read!();
    let mut c_vec: Vec<i64> = Vec::with_capacity(n);
    let mut a_vec: Vec<Vec<i64>> = Vec::with_capacity(n);
    for _ in 0..n {
        let temp_vec: Vec<i64> = Vec::with_capacity(m);
        a_vec.push(temp_vec);
    }
    for i in 0..n {
        let temp_c = read!();
        c_vec.push(temp_c);
        for _ in 0..m {
            a_vec[i].push(read!());
        }
    }
    let mut ans: i64 = i64::MAX;
    let all: usize = 1 << n;

    for mask in 0..all {
        let mut level_vec: Vec<i64> = vec![0; m];
        let mut cost: i64 = 0;
        for i in 0..n {
            if mask >> i & 1 == 1 {
                cost += c_vec[i];
                for j in 0..m {
                    level_vec[j] += a_vec[i][j];
                }
            }
        }
        let mut ok: bool = true;
        for j in 0..m {
            if level_vec[j] < x {
                ok = false;
            }
        }
        if ok {
            ans = ans.min(cost);
        }
    }
    if ans == i64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
