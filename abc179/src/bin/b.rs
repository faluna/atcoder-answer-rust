use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d1_2_vec: [(i64, i64); n],
    };
    let mut count = 0;
    for d1_2 in d1_2_vec {
        if d1_2.0 == d1_2.1 {
            count += 1;
            if count == 3 {
                println!("Yes");
                return;
            }
        } else {
            count = 0;
        }
    }
    println!("No");
}
