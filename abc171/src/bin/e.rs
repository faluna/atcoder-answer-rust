use proconio::{fastout, input};

fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
    };

    let a_fold = a_vec.iter().fold(0, |acc, a| acc ^ a);
    a_vec.iter().for_each(|a| {
        print!("{} ", a_fold ^ a);
    })
}
