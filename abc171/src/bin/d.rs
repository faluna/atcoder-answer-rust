use proconio::{fastout, input};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
        q: usize,
        b_c_vec: [(i64, i64); q],
    }
    let mut map: BTreeMap<i64, usize> = BTreeMap::new();
    a_vec.iter().for_each(|a| *map.entry(*a).or_insert(0) += 1);
    let mut ans: i64 = map.iter().fold(0, |acc, (a, &num)| a * num as i64 + acc);

    b_c_vec.iter().for_each(|(b, c)| {
        if let Some(num) = map.remove(b) {
            ans -= b * num as i64;
            *map.entry(*c).or_insert(0) += num;
            ans += c * num as i64;
        }
        println!("{}", ans);
    })
}
