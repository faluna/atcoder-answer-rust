use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        grid: [String;h],
    }
    let grid: Vec<Vec<char>> = grid
        .into_iter()
        .map(|row_str| row_str.chars().collect::<Vec<char>>())
        .collect();
    let mut count_row: Vec<i64> = vec![0; h];
    let mut count_col: Vec<i64> = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                count_row[i] += 1;
                count_col[j] += 1;
            }
        }
    }

    let mut ans: i64 = 0;
    for mut row_comb in 0..(1 << h) {
        let mut count: i64 = grid
            .iter()
            .flatten()
            .fold(0, |acc, g| if *g == '#' { acc + 1 } else { acc });
        let mut check_row: Vec<bool> = vec![false; h];
        for row in 0..h {
            if row_comb & 1 == 1 {
                check_row[row] = true;
                count -= count_row[row];
            }
            row_comb >>= 1;
        }
        for mut col_comb in 0..(1 << w) {
            let mut count2 = count;
            for col in 0..w {
                if col_comb & 1 == 1 {
                    count2 -= count_col[col];
                    for (idx, check) in check_row.iter().enumerate() {
                        if *check {
                            if grid[idx][col] == '#' {
                                count2 += 1;
                            }
                        }
                    }
                }
                col_comb >>= 1;
            }
            if count2 == k as i64 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
