use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        mut a_list: [i128; n],
    }

    for a in a_list.iter() {
        if a == &0 {
            println!("0");
            return;
        }
    }
    for a in a_list.iter() {
        if a >= &(1e18 as i128) {
            println!("0");
            return;
        }
    }
    let mut ans: i128 = 1;
    for a in a_list {
        if a <= 1000000000000000000 / ans {
            ans *= a;
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
