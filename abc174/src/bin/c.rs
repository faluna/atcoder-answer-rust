use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: i64,
    };
    let kk = if k % 7 == 0 { k * 9 / 7 } else { k * 9 };
    let mut ans = -1;
    let mut temp = 1;
    for i in 1..=9000_000 {
        temp = temp * 10 % kk;
        if temp == 1 {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
