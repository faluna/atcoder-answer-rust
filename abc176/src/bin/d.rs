use proconio::{fastout, input};
use std::collections::VecDeque;

const MOVE_A: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const MOVE_B: [(isize, isize); 20] = [
    (-2, 2),
    (-2, -1),
    (-2, 0),
    (-2, -1),
    (-2, -2),
    (-1, 2),
    (-1, 1),
    (-1, -1),
    (-1, -2),
    (0, 2),
    (0, -2),
    (1, 2),
    (1, 1),
    (1, -1),
    (1, -2),
    (2, 2),
    (2, 1),
    (2, 0),
    (2, -1),
    (2, -2),
];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c_pos: (usize, usize),
        d_pos: (usize, usize),
        s_str: [String; h],
    };
    let c_pos = (c_pos.0 - 1, c_pos.1 - 1);
    let d_pos = (d_pos.0 - 1, d_pos.1 - 1);
    let maze: Vec<Vec<char>> = s_str.iter().map(|s| s.chars().collect()).collect();
    let mut counted: Vec<Vec<i64>> = vec![vec![i64::MAX; w]; h];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_front(c_pos);
    counted[c_pos.0][c_pos.1] = 0;
    while let Some(cur_pos) = que.pop_front() {
        let cur_cost = counted[cur_pos.0][cur_pos.1];
        for mv in MOVE_A.iter() {
            let next = (cur_pos.0 as isize + mv.0, cur_pos.1 as isize + mv.1);
            if next.0 < 0 || h as isize <= next.0 || next.1 < 0 || w as isize <= next.1 {
                continue;
            }
            let next = (next.0 as usize, next.1 as usize);
            if maze[next.0][next.1] == '#' {
                continue;
            }
            if counted[next.0][next.1] <= cur_cost {
                continue;
            }
            counted[next.0][next.1] = cur_cost;
            que.push_front(next);
        }
        for mv in MOVE_B.iter() {
            let next = (cur_pos.0 as isize + mv.0, cur_pos.1 as isize + mv.1);
            if next.0 < 0 || h as isize <= next.0 || next.1 < 0 || w as isize <= next.1 {
                continue;
            }
            let next = (next.0 as usize, next.1 as usize);
            if maze[next.0][next.1] == '#' {
                continue;
            }
            if counted[next.0][next.1] != i64::MAX {
                continue;
            }
            counted[next.0][next.1] = cur_cost + 1;
            que.push_back(next);
        }
    }
    if counted[d_pos.0][d_pos.1] != i64::MAX {
        println!("{}", counted[d_pos.0][d_pos.1]);
    } else {
        println!("-1");
    }
}
