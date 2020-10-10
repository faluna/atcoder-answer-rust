use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        l_r_vec: [(usize, usize); h],
    };
    let mut right_point_vec: Vec<i64> = (0..w as i64).collect();
}
