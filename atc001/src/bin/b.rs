use proconio::{fastout, input};

struct UnionFind {
    parent: Vec<i64>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let parent: Vec<i64> = (0..size as i64).collect();
        Self { parent }
    }

    fn root(&mut self, x: i64) -> i64 {
        if *self
            .parent
            .get(x as usize)
            .expect("Get the parent of x, but error")
            == x
        {
            x
        } else {
            let rrx = self.root(*self.parent.get(x as usize).expect(""));
            let x_par = self
                .parent
                .get_mut(x as usize)
                .expect("Get the mutable parent of x, but error.");
            *x_par = rrx;
            *x_par
        }
    }

    fn unite(&mut self, x: i64, y: i64) {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return;
        } else {
            *self.parent.get_mut(rx as usize).expect("") = ry;
        }
    }

    fn same(&mut self, x: i64, y: i64) -> bool {
        self.root(x) == self.root(y)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        p_a_b_vec: [(usize, i64, i64); q],
    };
    let mut union_find: UnionFind = UnionFind::new(n);
    p_a_b_vec.iter().for_each(|(p, a, b)| {
        if *p == 0 {
            union_find.unite(*a, *b);
        } else {
            if union_find.same(*a, *b) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    })
}
