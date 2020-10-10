use proconio::{fastout, input};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Pair(i64, usize);

const DI: [isize; 4] = [-1, 0, 1, 0];
const DJ: [isize; 4] = [0, -1, 0, 1];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        mut start_point: (usize, usize),
        mut goal_point: (usize, usize),
        grid: String,
    };
    let grid: Vec<Vec<char>> = grid
        .split('\n')
        .map(|part_str| part_str.chars().collect::<Vec<char>>())
        .collect();
    start_point.0 -= 1;
    start_point.1 -= 1;
    goal_point.0 -= 1;
    goal_point.1 -= 1;
    let to_id = |i: usize, j: usize, v: usize| (i * w + j) * 4 + v;
    let dist: Rc<RefCell<Vec<i64>>> = Rc::new(RefCell::new(vec![i64::MAX; h * w * 4]));
    let q: Rc<RefCell<BinaryHeap<Reverse<Pair>>>> = Rc::new(RefCell::new(BinaryHeap::new()));
    let push = |i: usize, j: usize, v: usize, x: i64| {
        let id = to_id(i, j, v);
        if *dist.borrow().get(id).expect("error at push clojure") <= x {
            return;
        }
        {
            *dist
                .borrow_mut()
                .get_mut(id)
                .expect("error at push clojure 2") = x;
            q.borrow_mut().push(Reverse(Pair(x, id)));
        }
    };
    let mod_ceil = |x: i64| -> i64 { (x + k as i64 - 1) / k as i64 * k as i64 };
    (0..4).for_each(|v| push(start_point.0, start_point.1, v, 0));

    while let Some(Reverse(point)) = q.borrow_mut().pop() {
        if *dist.borrow().get(point.1).expect("error in while loop 1") != point.0 {
            continue;
        };
        let i = point.1 / w / 4;
        let j = point.1 / 4 % w;
        let v = point.1 % 4;
        (0..4).for_each(|nv| push(i, j, nv, mod_ceil(point.0)));
        let (ni, nj) = (
            i as isize + DI.get(v).expect("error at while loop 2"),
            j as isize + DJ.get(v).expect("error at while loop 3"),
        );
        if let Some(row_grid) = grid.get(ni as usize) {
            if let Some(coord) = row_grid.get(nj as usize) {
                if *coord == '.' {
                    push(ni as usize, nj as usize, v, point.0 + 1);
                }
            }
        }
    }

    let mut ans = i64::MAX;
    (0..4).for_each(|v| {
        ans = ans.min(
            *dist
                .borrow()
                .get(to_id(goal_point.0, goal_point.1, v))
                .expect("error at fainal block"),
        )
    });
    if ans == i64::MAX {
        println!("-1");
    } else {
        ans = mod_ceil(ans) / k as i64;
        println!("{}", ans);
    }
}
