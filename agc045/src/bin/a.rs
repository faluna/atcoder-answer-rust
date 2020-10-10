use proconio::{fastout, input};
use std::collections::HashSet;
use text_io::read;

#[fastout]
fn main() {
    let t: usize = read!();

    for _ in 0..t {
        test_case();
    }
}

fn test_case() {
    let n: usize = read!();
    let mut a_vec: Vec<i64> = Vec::new();
    for _ in 0..n {
        a_vec.push(read!());
    }
    let s: String = read!();
    let s: Vec<char> = s.chars().collect();
    let mut base: HashSet<i64> = HashSet::new();
    base.insert(0);
    for i in (0..n).rev() {
        let x: i64 = *a_vec.get(i).expect("");
        match s.get(i) {
            Some(&'0') => {
                for b in base.clone().iter() {
                    let v_a = x ^ b;
                    base.insert(v_a);
                }
            }
            Some(&'1') => {
                if !base.contains(&x) {
                    base.clear();
                    println!("1");
                    return;
                }
            }
            _ => {}
        }
    }
    println!("0");
}
