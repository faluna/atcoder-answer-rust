use proconio::{fastout, input};

fn main() {
    input! {
        n: usize,
        c_str: String,
    };
    let mut left = 0;
    let mut right = n - 1;

    let mut ans = 0;
    let mut c_chars: Vec<char> = c_str.chars().collect();
    'outer: while left < right {
        while c_chars[left] == 'R' {
            left += 1;
            if left >= right {
                break 'outer;
            }
        }
        while c_chars[right] == 'W' {
            right -= 1;
            if left >= right {
                break 'outer;
            }
        }
        c_chars.swap(left, right);
        ans += 1;
    }
    println!("{}", ans);
}
