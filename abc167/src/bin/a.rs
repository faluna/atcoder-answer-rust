use proconio::{fastout, input};
use text_io::read;

#[fastout]
fn main() {
    let s: String = read!();
    let t: String = read!();
    let mut part_t: Vec<char> = t.chars().collect();
    part_t.pop();
    let part_t: String = part_t.iter().collect();
    if s == part_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
