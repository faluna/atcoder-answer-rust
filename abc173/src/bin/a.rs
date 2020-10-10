use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
    };
    let mut ans: i64 = 0;
    for i in 1..=10 {
        let pay = 1000 * i;
        if pay >= n {
            ans = pay;
            break;
        }
    }
    println!("{}", ans - n);
}
