use proconio::{fastout, input};
use std::collections::{BTreeMap, VecDeque};

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, -1, 0, 1];

#[derive(Debug, Clone, Default)]
struct L {
    a: i32,
    b: i32,
    c: i32,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_c: [(i32, i32, i32); n],
        d_e_f: [(i32, i32, i32); m],
    };
    let mut map_x: BTreeMap<i32, i32> = BTreeMap::new();
    let mut map_y: BTreeMap<i32, i32> = BTreeMap::new();
    map_x.entry(0).or_insert(0);
    map_y.entry(0).or_insert(0);
    let mut lh: Vec<L> = Vec::new();
    let mut lv: Vec<L> = Vec::new();

    for (a, b, c) in a_b_c.iter() {
        map_x.entry(*a).or_insert(0);
        map_x.entry(*b).or_insert(0);
        map_y.entry(*c).or_insert(0);
        lh.push(L {
            a: *a,
            b: *b,
            c: *c,
        });
    }
    for (c, a, b) in d_e_f.iter() {
        map_y.entry(*a).or_insert(0);
        map_y.entry(*b).or_insert(0);
        map_x.entry(*c).or_insert(0);
        lv.push(L {
            a: *a,
            b: *b,
            c: *c,
        });
    }

    let (mut xs, mut ys): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
    for (key, value) in map_x.iter_mut() {
        *value = xs.len() as i32;
        xs.push(*key);
    }
    for (key, value) in map_y.iter_mut() {
        *value = ys.len() as i32;
        ys.push(*key);
    }

    let h = xs.len() * 2;
    let w = ys.len() * 2;

    let mut d: Vec<Vec<i32>> = vec![vec![0; w]; h];
    for i in 0..n {
        let a = map_x[&lh[i as usize].a] * 2;
        let b = map_x[&lh[i as usize].b] * 2;
        let c = map_x[&lh[i as usize].c] * 2;
        for x in a..=b {
            d[x as usize][c as usize] = -1;
        }
    }
    for i in 0..m {
        let a = map_x[&lv[i as usize].a] * 2;
        let b = map_x[&lv[i as usize].b] * 2;
        let c = map_x[&lv[i as usize].c] * 2;
        for y in a..=b {
            d[c as usize][y as usize] = -1;
        }
    }

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let (sx, sy) = (map_x[&0] * 2, map_y[&0] * 2);
    q.push_back((sx, sy));
    while let Some((x, y)) = q.pop_front() {
        for v in 0..4 {
            let nx = x + DX[v];
            let ny = y + DY[v];
            if nx < 0 || nx >= h as i32 {
                continue;
            }
            if ny < 0 || ny >= h as i32 {
                continue;
            }
            if d[nx as usize][ny as usize] != 0 {
                continue;
            }
            d[nx as usize][ny as usize] = 1;
            q.push_back((nx, ny));
        }
    }

    let mut ans: f64 = 0.;
    for x in 0..h {
        for y in 0..w {
            if d[x][y] != 1 {
                continue;
            }
            if x == 0 || x == h - 1 || y == 0 || y == w - 1 {
                println!("INF");
                return ();
            }
            if x % 2 == 0 || y % 2 == 0 {
                continue;
            }
            let ex = xs[x / 2 + 1] - xs[x / 2];
            let ey = ys[y / 2 + 1] - ys[y / 2];
            ans += (ex * ey) as f64;
        }
    }
    println!("{}", ans);
}
