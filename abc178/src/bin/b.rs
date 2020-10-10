use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };

    let a_c = a * c;
    let a_d = a * d;
    let b_c = b * c;
    let b_d = b * d;

    let mut ans = a_c;
    ans = ans.max(a_d);
    ans = ans.max(b_c);
    ans = ans.max(b_d);

    println!("{}", ans);
}
