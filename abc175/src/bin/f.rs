use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: i64 = 1_000_000_000_000_000_000;

#[derive(Clone, Debug)]
struct Edge {
    to: i64,
    co: i64,
}

impl Edge {
    fn new(to: i64, co: i64) -> Self {
        Self { to, co }
    }
}

#[fastout]
fn main() {
    input! {
        m: usize,
        mut s_c_vec: [(String, i64); m],
    };
    for s_c in s_c_vec.clone().iter() {
        s_c_vec.push((s_c.0.chars().rev().collect::<String>(), s_c.1));
    }
    let mut n = 1;
    let mut id: Vec<Vec<usize>> = vec![Vec::new(); m * 2];
    for i in 0..m * 2 {
        let l = s_c_vec[i].0.len();
        for _ in 0..l {
            id[i].push(n);
            n += 1;
        }
    }

    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); n as usize];
    for i in 0..m * 2 {
        let l = s_c_vec[i].0.len();
        for j in 0..l {
            let v = id[i][j];
            let mut temp_ni = 0;
            let mut u = 0;
            for ni in 0..m * 2 {
                temp_ni = ni;
                if i / m == ni / m {
                    continue;
                }
                let nl = s_c_vec[ni].0.len();
                let w = nl.min(l - j);
                if s_c_vec[i].0[j..j + w] != s_c_vec[ni].0[0..w] {
                    continue;
                }
                if nl > w {
                    u = id[ni][w];
                } else if l > j + w {
                    u = id[i][j + w];
                } else {
                    u = 0;
                }
                g[v].push(Edge::new(u as i64, s_c_vec[temp_ni].1));
            }
        }
    }
    let mut dist: Vec<i64> = vec![INF; n as usize];
    let mut que: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let push =
        |dist: &mut Vec<i64>, que: &mut BinaryHeap<Reverse<(i64, usize)>>, v: usize, x: i64| {
            if *dist.get(v).unwrap() <= x {
                return;
            }
            *dist.get_mut(v).unwrap() = x;
            que.push(Reverse((x, v)));
        };
    for i in 0..m {
        push(&mut dist, &mut que, id[i][0], s_c_vec[i].1);
    }
    while let Some(Reverse(elem)) = que.pop() {
        let x = elem.0;
        let v = elem.1;
        if *dist.get(v).unwrap() != x {
            continue;
        }
        for edge in g[v].clone() {
            push(&mut dist, &mut que, edge.to as usize, x + edge.co);
        }
    }

    let mut ans = dist[0];
    for i in 0..m * 2 {
        let l = s_c_vec[i].0.len();
        for j in 0..l {
            let t: String = String::from(&s_c_vec[i].0[j..]);
            if t == t.chars().rev().collect::<String>() {
                ans = ans.min(dist[id[i][j]]);
            }
        }
    }
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
