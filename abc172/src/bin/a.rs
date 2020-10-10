use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
    }
    let a_2: i64 = a * a;
    println!("{}", a + a_2 + a * a_2);
}
