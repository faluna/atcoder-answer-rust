use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c_vec: [i64; n],
        q_vec: [(usize, usize); q],
    };
    let mut pi: Vec<i64> = vec![-1; n + 1];
    let mut ps: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        let l = pi[c_vec[i] as usize];
        if l != -1 {
            ps[l as usize].push(i);
        }
        pi[c_vec[i] as usize] = i as i64;
    }
    let mut qs: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for qi in 0..q {
        let (l, r) = (q_vec[qi].0 - 1, q_vec[qi].1 - 1);
        qs[l].push((r, qi));
    }

    let mut ans: Vec<i64> = vec![-1; q];
    let mut d = BITree::new(n);
    for x in (0..n).rev() {
        for y in ps[x].iter() {
            d.add(*y, 1);
        }
        for query in qs[x].iter() {
            let r = query.0;
            let i = query.1;
            ans[i] = (r - x + 1) as i64 - d.sum(r);
        }
    }

    for i in ans.iter() {
        println!("{}", i);
    }
}

struct BITree {
    n: usize,
    bit: Vec<i64>,
}

impl BITree {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    fn add(&mut self, i: usize, x: i64) {
        let mut idx = i;
        while idx <= self.n {
            self.bit[idx] += x;
            idx += (idx as i64 & -(idx as i64)) as usize;
        }
    }

    fn sum(&self, i: usize) -> i64 {
        let mut ans: i64 = 0;
        let mut idx = i;
        while idx > 0 {
            ans += self.bit[idx];
            idx -= (idx as i64 & -(idx as i64)) as usize;
        }
        ans
    }
}
