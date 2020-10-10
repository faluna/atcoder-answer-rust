use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut p_vec: [i64; n],
    };
    p_vec.sort();
    let mut ans: i64 = 0;
    for i in 0..k {
        ans += p_vec.get(i).expect("error at for loop");
    }
    println!("{}", ans);
}
