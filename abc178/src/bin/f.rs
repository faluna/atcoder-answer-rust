use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
        b_vec: [i64; n],
    };
    let mut count_vec = vec![0; n + 1];
    let mut a_b_vec = a_vec.clone();
    a_b_vec.extend(b_vec.clone());
    for c in a_b_vec {
        count_vec[c as usize] += 1;
    }
    for c in count_vec {
        if c > n {
            println!("No");
            return;
        }
    }
    println!("Yes");
    let mut dec_x = 0;
    'outer: for x in 0..=n {
        for i in 0..n {
            if a_vec[i] == b_vec[(i + x) % n] {
                continue 'outer;
            }
        }
        dec_x = x;
    }
    for i in 0..n {
        print!("{} ", b_vec[((i + dec_x) % n) as usize]);
    }
}
