use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x_vec: [i64; 5],
    }

    for (i, x) in x_vec.iter().enumerate() {
        if *x == 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
