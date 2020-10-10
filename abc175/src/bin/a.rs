use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };
    println!(
        "{}",
        match s.as_str() {
            "RRR" => 3,
            "RRS" | "SRR" => 2,
            "SSS" => 0,
            _ => 1,
        }
    );
}
