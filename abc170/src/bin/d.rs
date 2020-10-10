use proconio::{fastout, input};

const M: usize = 1000005;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
    }

    let mut cnt: Vec<usize> = vec![0; M];
    for a in a_vec.iter() {
        if cnt[*a as usize] != 0 {
            cnt[*a as usize] = 2;
            continue;
        }
        for i in ((*a as usize)..M).step_by(*a as usize) {
            cnt[i] += 1;
        }
    }
    let mut ans = 0;
    for a in a_vec {
        if cnt[a as usize] == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
