use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n_str: String,
    };
    let acc: i64 = n_str
        .chars()
        .fold(0, |acc, c| c.to_digit(10).unwrap() as i64 + acc);
    println!("{}", if acc % 9 == 0 { "Yes" } else { "No" });
}
