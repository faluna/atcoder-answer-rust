use proconio::{fastout, input};

struct UnionFind {
    set: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let set: Vec<i64> = vec![-1; n];
        Self { set }
    }

    fn root(&mut self, i: usize) -> usize {
        if self.set[i] < 0 {
            i
        } else {
            self.set[i] = self.root(self.set[i] as usize) as i64;
            self.set[i] as usize
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.root(x);
        let mut y_root = self.root(y);
        if x_root == y_root {
            false
        } else {
            if self.set[x_root] > self.set[y_root] {
                std::mem::swap(&mut x_root, &mut y_root);
            }
            self.set[x_root] += self.set[y_root];
            self.set[y_root] = x_root as i64;
            true
        }
    }

    fn size(&mut self, i: usize) -> usize {
        let par = self.root(i);
        (self.set[par] * -1) as usize
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_vec: [(usize, usize); m],
    };
    let a_b_vec: Vec<(usize, usize)> = a_b_vec.iter().map(|(a, b)| (a - 1, b - 1)).collect();
    let mut set: UnionFind = UnionFind::new(n);
    for a_b in a_b_vec.iter() {
        set.unite(a_b.0, a_b.1);
    }
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(set.size(i));
    }
    println!("{}", ans);
}
