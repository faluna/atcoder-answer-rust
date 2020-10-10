use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        n: usize,
        p_vec: [usize; n],
    }

    let mut d: [i64; 102] = [-1; 102];
    p_vec.iter().for_each(|p| {
        d[*p] = 1;
    });
    let mut ans: (i64, i64) = (9999999, -1);
    (0..=101).for_each(|i| {
        if d[i] != 1 {
            let dif = (x - i as i64).abs();
            ans = if ans.0 > dif {
                (dif, i as i64)
            } else if ans.0 == dif && ans.1 > i as i64 {
                (dif, i as i64)
            } else {
                ans
            };
        }
    });
    println!("{}", ans.1)
}
