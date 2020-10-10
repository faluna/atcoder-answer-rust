use proconio::{fastout, input};
use std::collections::VecDeque;

const STEPS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        sy_sx: (usize, usize),
        gy_gx: (usize, usize),
        c_strs: [String; r],
    };
    let c_maze: Vec<Vec<char>> = c_strs
        .into_iter()
        .map(|c_str| c_str.chars().collect())
        .collect();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut checked: Vec<Vec<i64>> = vec![vec![i64::MAX; c]; r];
    checked[sy_sx.0 - 1][sy_sx.0 - 1] = 0;
    queue.push_back(sy_sx);
    while let Some(now) = queue.pop_front() {
        for step in STEPS.iter() {
            let next_y: usize = (now.0 as isize + step.0) as usize;
            let next_x: usize = (now.1 as isize + step.1) as usize;
            match c_maze[next_y - 1][next_x - 1] {
                '.' => {
                    let temp = checked[now.0 - 1][now.1 - 1] + 1;
                    if temp < checked[next_y - 1][next_x - 1] {
                        checked[next_y - 1][next_x - 1] = temp;
                        queue.push_back((next_y, next_x));
                    }
                }
                _ => (),
            }
        }
    }
    // println!("{:?}", checked);
    println!("{}", checked[gy_gx.0 - 1][gy_gx.1 - 1]);
}
