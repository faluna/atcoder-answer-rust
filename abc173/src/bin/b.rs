use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s_vec: [String; n],
    }
    let mut count: Vec<usize> = vec![0; 4];
    s_vec.iter().for_each(|s| match &s[..] {
        "AC" => *count.get_mut(0).expect("") += 1,
        "WA" => *count.get_mut(1).expect("") += 1,
        "TLE" => *count.get_mut(2).expect("") += 1,
        "RE" => *count.get_mut(3).expect("") += 1,
        _ => (),
    });
    println!(
        "AC x {}\nWA x {}\nTLE x {}\nRE x {}",
        count[0], count[1], count[2], count[3]
    );
}
