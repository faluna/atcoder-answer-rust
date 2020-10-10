use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u32,
        s: String,
    }
    println!(
        "{}",
        if s.len() <= k as usize {
            s
        } else {
            s.as_str()[..k as usize].to_string() + "..."
        }
    )
}
