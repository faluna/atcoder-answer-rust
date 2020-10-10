use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: i64,
    };
    n -= 1;
    let mut ans: Vec<u8> = Vec::new();
    loop {
        let remainder = n % 26;
        ans.push(b'a' + remainder as u8);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    println!(
        "{}",
        ans.into_iter().rev().map(|c| c as char).collect::<String>()
    );
}
