use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l_vec: [i64; n],
    };
    l_vec.sort();
    if n < 3 {
        println!("0");
        return;
    }
    let mut ans = 0;
    for l1i in 0..(n - 2) {
        let l1 = l_vec[l1i];
        for l2i in (l1i + 1)..(n - 1) {
            let l2 = l_vec[l2i];
            if l2 == l1 {
                continue;
            }
            for l3i in (l2i + 1)..n {
                let l3 = l_vec[l3i];
                if l1 == l3 || l2 == l3 {
                    continue;
                }
                if l3 - l2 < l1 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
