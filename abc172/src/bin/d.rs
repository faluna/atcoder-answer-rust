use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    let root_n: i64 = (n as f64).sqrt().floor() as i64;
    let mut ans = 0;
    for i in 1..=root_n {
        let sums = ((n / i) * (n / i + 1)) / 2 - (i * (i - 1)) / 2;
        ans += 2 * sums * i - i * i;
    }
    println!("{}", ans);
}

// fn main() {
//     input! {
//         n: i64,
//     }
//     let mut ans = 0;
//     for i in 1..=n {
//         let k = n / i;
//         ans += i * (k * (k + 1) / 2);
//     }
//     println!("{}", ans);
// }

// fn main() {
//     input! {
//         n: usize,
//     }
//     let mut table: Vec<i64> = vec![0; n + 1];
//     for i in 1..=n {
//         for j in (i..=n).step_by(i) {
//             table[j] += 1;
//         }
//     }
//     let mut ans: i64 = 0;
//     for i in 1..=n {
//         ans += i as i64 * table[i as usize];
//     }
//     println!("{}", ans);
// }

// fn main() {
//     input! {
//         n: usize,
//     }
//     let mut table: Vec<i64> = vec![1; n + 1];
//     let mut check: Vec<bool> = vec![false; n + 1];
//     for i in 2..=n {
//         if *check.get(i).expect("") == true {
//             continue;
//         }
//         check[i] = true;
//         let max = n / i + 1;
//         for j in 1..max {
//             check[i * j] = true;
//             let mut tmp = j;
//             let mut cnt = 1;
//             while tmp % i == 0 {
//                 tmp /= i;
//                 cnt += 1;
//             }
//             table[i * j] *= i.pow(cnt) as i64 * (cnt as i64 + 1);
//         }
//     }
//     println!("{}", table.iter().sum::<i64>() - 1);
// }

// fn main() {
//     input! {
//         n: usize,
//     }
//     let mut table: Vec<i64> = (0..=n).map(|i| i as i64).collect();
//     'loop1: for i in 2..=n {
//         if *table.get(i).expect("") < i as i64 {
//             continue 'loop1;
//         }
//         for j in (i..=n).step_by(i) {
//             table[j] = i as i64;
//         }
//     }
//     let mut ans = 0;

//     for mut i in 1_i64..=n as i64 {
//         let mut cnt: HashMap<i64, usize> = HashMap::new();
//         let mut ret = i as i64;
//         while i != 1 {
//             let prime = table[i as usize];
//             *cnt.entry(prime).or_insert(0) += 1;
//             i /= prime;
//         }
//         cnt.values().for_each(|val| {
//             ret *= *val as i64 + 1;
//         });
//         ans += ret;
//     }
//     println!("{}", ans);
// }
