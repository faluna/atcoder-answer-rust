use proconio::{fastout, input};

const K: usize = 43;

#[fastout]
fn main() {
    input! {
        n: usize,
        a_vec: [i64; n],
    };
    let mut a_iter = a_vec.iter();
    let a: i64 = *a_iter.next().expect("");
    let b: i64 = *a_iter.next().expect("");
    let s: i64 = a + b;
    let x: i64 = a_iter.fold(0, |acc, a| acc ^ a);
    let d: i64 = (s - x) / 2;
    if d < 0 || d & x != 0 {
        println!("-1");
        return;
    }
    if d > a {
        println!("-1");
        return;
    }
    let mut y: i64 = 0;
    let mut x_stack: Vec<i64> = Vec::new();
    let mut xx: i64 = x;
    x_stack.push(xx % 2);
    while xx != 0 {
        xx >>= 1;
        x_stack.push(xx % 2);
    }
    'calc: for (idx, bit) in x_stack.iter().enumerate().rev() {
        if *bit == 0 {
            continue;
        } else {
            let temp = y | (bit << (x_stack.len() - idx));
            if (d ^ temp) > a {
                break 'calc;
            } else {
                y = temp;
            }
        }
    }
    let aa = d ^ y;
    println!("{}", a - aa);
}

// fn main() {
//     input! {
//         n: usize,
//         a_vec: [i64; n],
//     }
//     let mut a = a_vec[0];
//     let b = a_vec[1];
//     let mut x = 0;
//     (2..n).for_each(|i| x ^= a_vec[i]);
//     let mut s = a + b;
//     let mut dp = vec![vec![vec![-1; 2]; 2]; K];
//     dp[0][0][0] = 0;
//     let mut v: i64 = 1;
//     for i in 0..(K - 1) {
//         let cx = x & 1;
//         let cs = s & 1;
//         let ca = a & 1;
//         for j in 0..2 {
//             for k in 0..2 {
//                 if dp[i][j][k] == -1 {
//                     continue;
//                 }
//                 for na in 0..2 {
//                     for nb in 0..2 {
//                         let ni = i + 1;
//                         let mut nj = 0;
//                         let mut _nk = k;
//                         if na ^ nb != cx {
//                             continue;
//                         }
//                         let ns = na + nb + j as i64;
//                         if ns % 2 != cs {
//                             continue;
//                         }
//                         if ns >= 2 {
//                             nj = 1;
//                         }
//                         if ca < na {
//                             _nk = 1;
//                         } else if ca == na {
//                             _nk = k;
//                         } else {
//                             _nk = 0;
//                         }
//                         dp[ni][nj][_nk] = std::cmp::max(dp[ni][nj][_nk], dp[i][j][k] | (v * na));
//                     }
//                 }
//             }
//         }
//         x >>= 1;
//         s >>= 1;
//         a >>= 1;
//         v <<= 1;
//     }

//     a = dp[K - 1][0][0];
//     if a == -1 || a == 0 {
//         println!("-1");
//         return;
//     }
//     let ans = a_vec[0] - a;
//     println!("{}", ans);
// }
