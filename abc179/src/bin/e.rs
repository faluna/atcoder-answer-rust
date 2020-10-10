use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize,
        mut x: i64,
        m: i64,
    };
    let mut checked: Vec<i64> = vec![-1; m as usize];
    let mut a_vec: Vec<i64> = Vec::new();
    let mut ans = 0;
    let mut len = 0;
    let mut tot = 0;
    while checked[x as usize] == -1 {
        a_vec.push(x);
        checked[x as usize] = len;
        len += 1;
        tot += x;
        x = (x * x) % m;
    }

    let c = len - checked[x as usize];
    let mut s = 0;
    for i in checked[x as usize]..len {
        s += a_vec[i as usize];
    }
    if n <= len as usize {
        for i in 0..n {
            ans += a_vec[i];
        }
    } else {
        ans += tot;
        n -= len as usize;
        ans += s * (n as i64 / c);
        n %= c as usize;
        for i in 0..n {
            ans += a_vec[checked[x as usize] as usize + i];
        }
    }
    println!("{}", ans);
}
