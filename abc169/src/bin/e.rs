use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_b: [(i64, i64); n],
    }
    let (mut a_vec, mut b_vec): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new());
    for (a, b) in a_b {
        a_vec.push(a);
        b_vec.push(b);
    }
    a_vec.sort();
    b_vec.sort();

    let ans: i64;
    if n % 2 == 1 {
        let l = a_vec.get(n / 2).expect("");
        let r = b_vec.get(n / 2).expect("");
        ans = r - l + 1;
    } else {
        let l = a_vec.get(n / 2 - 1).expect("") + a_vec.get(n / 2).expect("");
        let r = b_vec.get(n / 2 - 1).expect("") + b_vec.get(n / 2).expect("");
        ans = r - l + 1;
    }
    println!("{}", ans)
}
