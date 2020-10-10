use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    let zipper: Vec<_> = s.chars().zip(t.chars()).collect();
    let ans = zipper.iter().fold(
        0,
        |acc, (s_char, t_char)| if *s_char != *t_char { acc + 1 } else { acc },
    );
    println!("{}", ans);
}
