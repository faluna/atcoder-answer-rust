use proconio::{fastout, input};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a_vec: [i64; n],
        b_vec: [i64; m],
    }
    let mut a_acc_vec: Vec<i64> = Vec::new();
    a_acc_vec.push(0);
    a_vec.iter().enumerate().for_each(|(i, a)| {
        a_acc_vec.push(a + a_acc_vec.get(i).expect("error at calculating a_acc_vec"))
    });
    let mut b_acc_vec: Vec<i64> = Vec::new();
    b_acc_vec.push(0);
    b_vec.iter().enumerate().for_each(|(i, b)| {
        b_acc_vec.push(b + b_acc_vec.get(i).expect("error at calculating b_acc_vec"))
    });
    let (mut ans, mut j): (usize, usize) = (0, m);
    'out: for i in 0..=n {
        if let Some(a_acc) = a_acc_vec.get(i) {
            if *a_acc > k {
                break 'out;
            }
            'inner: while let Some(b_acc) = b_acc_vec.get(j) {
                if *b_acc > k - *a_acc {
                    j -= 1;
                } else {
                    break 'inner;
                }
            }
            ans = ans.max(i + j);
        }
    }
    println!("{}", ans);
}
