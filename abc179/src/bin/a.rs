use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    };
    let mut c_vec: Vec<char> = s.chars().collect();
    if *c_vec.last().unwrap() != 's' {
        c_vec.push('s');
    } else {
        c_vec.append(&mut vec!['e', 's']);
    }
    println!("{}", c_vec.iter().collect::<String>());
}
