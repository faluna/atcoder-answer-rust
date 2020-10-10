use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p_vec: [usize; n],
        c_vec: [i64; n],
    };
    for i in 0..n {
        p_vec[i] -= 1;
    }
    let mut ans = i64::MIN;
    for i in 0..n {
        let mut x = i;
        let mut s_vec: Vec<i64> = Vec::new();
        let mut tot: i64 = 0;
        loop {
            x = p_vec[x];
            s_vec.push(c_vec[x]);
            tot += c_vec[x];
            if x == i {
                break;
            }
        }
        // println!("test");
        let l = s_vec.len();
        let mut t = 0;
        for ii in 0..l {
            t += s_vec[ii];
            if (ii + 1) > k {
                break;
            }
            let mut now = t;
            if tot > 0 {
                let e = (k - (ii + 1)) / l;
                now += tot * e as i64;
                // println!("{}", now);
            }
            ans = ans.max(now);
        }
    }
    println!("{}", ans);
}
