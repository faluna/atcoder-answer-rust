use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x_y_vec: [(i64, i64); n],
    };
    let mut a_vec: Vec<i64> = Vec::with_capacity(n);
    let mut b_vec: Vec<i64> = Vec::with_capacity(n);
    x_y_vec.iter().for_each(|(x, y)| {
        a_vec.push(x + y);
        b_vec.push(x - y);
    });
    a_vec.sort();
    b_vec.sort();
    println!(
        "{}",
        (a_vec.last().unwrap() - a_vec[0]).max(b_vec.last().unwrap() - b_vec[0])
    );
}
