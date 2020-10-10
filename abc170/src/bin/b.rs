use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64,
    }

    let y_x2 = y - 2 * x;
    let x4_y = 4 * x - y;
    if y_x2 % 2 == 0 && x4_y % 2 == 0 && y_x2 >= 0 && x4_y >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
