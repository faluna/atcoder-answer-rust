use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        alpha: char,
    };
    if alpha.is_ascii_lowercase() {
        println!("a");
    } else {
        println!("A");
    }
}
