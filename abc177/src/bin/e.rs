use proconio::{fastout, input};

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    if x == 0 {
        return y;
    } else if y == 0 {
        return x;
    }
    if x < y {
        let temp = x;
        x = y;
        y = temp;
    }
    let m = x % y;
    if m == 0 {
        y
    } else {
        gcd(y, m)
    }
}

const A: i64 = 1_000_005;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
    };
    let mut c_vec: Vec<i64> = vec![0; A as usize];
    a_vec
        .iter()
        .for_each(|a| *c_vec.get_mut(*a as usize).unwrap() += 1);
    let mut pairwise = true;
    for i in 2..A as usize {
        let mut cnt = 0;
        for j in (i..A as usize).step_by(i) {
            cnt += c_vec[j];
        }
        if cnt > 1 {
            pairwise = false;
        }
    }
    if pairwise {
        println!("pairwise coprime");
        return;
    }

    let mut g = 0;
    for i in 0..n {
        g = gcd(g, a_vec[i]);
    }
    if g == 1 {
        println!("setwise coprime");
        return;
    }
    println!("not coprime");
}
