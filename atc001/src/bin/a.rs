use proconio::{fastout, input};

const DXY: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c_str_vec: [String; h],
    }
    // println!("{:?}", c_str);
    let maze: Vec<Vec<char>> = c_str_vec
        .into_iter()
        .map(|c_str| c_str.chars().collect::<Vec<char>>())
        .collect();
    let mut start: (i64, i64) = (0, 0);
    'search_start: for (i, rows) in maze.iter().enumerate() {
        for (j, point) in rows.iter().enumerate() {
            if *point == 's' {
                start = (i as i64, j as i64);
                break 'search_start;
            }
        }
    }
    let mut stack: Vec<(i64, i64)> = Vec::new();
    stack.push(start);
    let mut check: Vec<Vec<bool>> = vec![vec![false; w]; h];
    while let Some(now) = stack.pop() {
        check[now.0 as usize][now.1 as usize] = true;
        for xy in DXY.iter() {
            let next = (now.0 + xy.0, now.1 + xy.1);
            if next.0 < 0 || h as i64 <= next.0 || next.1 < 0 || w as i64 <= next.1 {
                continue;
            }
            if maze[next.0 as usize][next.1 as usize] == '#' {
                continue;
            } else if maze[next.0 as usize][next.1 as usize] == 'g' {
                println!("Yes");
                return;
            }
            if !check[next.0 as usize][next.1 as usize] {
                stack.push(next);
            }
        }
    }
    println!("No");
}
