use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let rem = n % 10;
    println!(
        "{}",
        match rem {
            2 | 4 | 5 | 7 | 9 => "hon",
            0 | 1 | 6 | 8 => "pon",
            3 => "bon",
            _ => "",
        }
    )
}
