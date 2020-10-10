use proconio::{fastout, input};

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        h_w_vec: [(usize, usize); m],
    };
    let h_w_vec: Vec<(usize, usize)> = h_w_vec
        .iter()
        .map(|(el_h, el_w)| (el_h - 1, el_w - 1))
        .collect();
    let mut row_vec: Vec<i64> = vec![0; w];
    let mut col_vec: Vec<i64> = vec![0; h];
    for h_w in h_w_vec.iter() {
        col_vec[h_w.0] += 1;
        row_vec[h_w.1] += 1;
    }
    let c_max = row_vec.iter().max().unwrap();
    let c_max_vec: Vec<(usize, &i64)> = row_vec
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == *c_max)
        .collect();
    let r_max = col_vec.iter().max().unwrap();
    let r_max_vec: Vec<(usize, &i64)> = col_vec
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == *r_max)
        .collect();
    let ans = c_max + r_max;
    for c_i_v in c_max_vec.iter() {
        'r_loop: for r_i_v in r_max_vec.iter() {
            for h_w in h_w_vec.iter() {
                if c_i_v.0 == h_w.1 && r_i_v.0 == h_w.0 {
                    continue 'r_loop;
                }
            }
            println!("{}", ans);
            return;
        }
    }
    println!("{}", ans - 1);
    // 'loop1: loop {
    //     let temp_rv = row_vec.clone();
    //     let c_max = temp_rv.iter().enumerate().max_by_key(|(_, &v)| v).unwrap();
    //     let temp_cv = col_vec.clone();
    //     let r_max = temp_cv.iter().enumerate().max_by_key(|(_, &v)| v).unwrap();
    //     let mut judge = true;
    //     let mut _switch = true;
    //     for h_w in h_w_vec.iter() {
    //         if h_w.0 != c_max.0 || h_w.1 != r_max.0 {
    //             if ans < r_max.1 + c_max.1 {
    //                 ans = r_max.1 + c_max.1;
    //             }
    //         } else {
    //             let temp = ans;
    //             ans = r_max.1 + c_max.1 - 1;
    //             if ans < temp {
    //                 break;
    //             } else {
    //                 if _switch {
    //                     row_vec.push(i64::MIN);
    //                     row_vec.swap_remove(r_max.0);
    //                 } else {
    //                     col_vec.push(i64::MIN);
    //                     col_vec.swap_remove(c_max.0);
    //                 }
    //                 judge = false;
    //                 _switch = !_switch;
    //                 break;
    //             }
    //         }
    //     }
    //     if judge {
    //         break 'loop1;
    //     }
    // }
    // println!("{}", ans);
}
