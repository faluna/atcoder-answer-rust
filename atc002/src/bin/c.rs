use proconio::{fastout, input};
use std::collections::BinaryHeap;
use std::ops::Add;

#[derive(Clone, Debug)]
struct Node {
    item: i64,
    left: SkewHeap,
    right: SkewHeap,
    length: usize,
}

impl Node {
    fn new(item: i64) -> Self {
        Self {
            item,
            left: SkewHeap::new(),
            right: SkewHeap::new(),
            length: 1,
        }
    }

    fn swap(&mut self) {
        let &mut Node {
            ref mut left,
            ref mut right,
            ..
        } = self;
        std::mem::swap(left, right);
    }

    fn divide(self) -> (i64, SkewHeap, SkewHeap) {
        let Node {
            item, left, right, ..
        } = self;
        (item, left, right)
    }

    fn len(&mut self) -> usize {
        let ans: usize = 1 + self.right.len();
        self.length = ans;
        self.length
    }
}

#[derive(Clone, Debug)]
enum Vertex {
    Tree(SkewHeap),
    Leaf(i64),
}

impl Add for Vertex {
    type Output = i64;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Vertex::Tree(sh), Vertex::Tree(other)) => sh.root().unwrap() + other.root().unwrap(),
            (Vertex::Tree(sh), Vertex::Leaf(other)) => sh.root().unwrap() + other,
            (Vertex::Leaf(s_root), Vertex::Tree(other)) => s_root + other.root().unwrap(),
            (Vertex::Leaf(s_root), Vertex::Leaf(other)) => s_root + other,
        }
    }
}

#[derive(Clone, Debug)]
struct SkewHeap(Option<Box<Node>>);

impl SkewHeap {
    fn new() -> Self {
        SkewHeap(None)
    }

    fn from(w: i64) -> Self {
        SkewHeap(Some(Box::new(Node::new(w))))
    }

    fn root(&self) -> Option<i64> {
        match self {
            SkewHeap(Some(heap)) => Some(heap.as_ref().item),
            SkewHeap(None) => None,
        }
    }

    fn meld(&mut self, mut other: Self) {
        if self.0.is_none() && other.0.is_none() {
            return;
        } else if self.0.is_some() && other.0.is_none() {
            return;
        } else if self.0.is_none() && other.0.is_some() {
            std::mem::swap(self, &mut other);
            return;
        }
        if self.root().unwrap_or(i64::MAX) < other.root().unwrap_or(i64::MIN) {
            std::mem::swap(self, &mut other);
        }
        if let Some(root_heap) = self.0.as_mut() {
            root_heap.right.meld(other);
        }
    }

    fn push(&mut self, w: i64) {
        let added = Some(Box::new(Node::new(w)));
        if self.0.is_some() {
            let added_heap = SkewHeap(added);
            self.meld(added_heap);
        } else {
            self.0 = added;
        }
    }

    fn pop(&mut self) -> Option<i64> {
        let mut ans: Option<i64> = None;

        if let Some(node) = self.0.take() {
            let (root, mut left_heap, right_heap, ..) = node.divide();
            ans = Some(root);
            if left_heap.0.is_some() && right_heap.0.is_none() {
                *self = left_heap;
            } else if left_heap.0.is_none() && right_heap.0.is_some() {
                *self = right_heap;
            } else if left_heap.0.is_some() && right_heap.0.is_none() {
                left_heap.meld(right_heap);
            }
        }
        ans
    }

    fn len(&mut self) -> usize {
        if let Some(node) = self.0.as_mut() {
            node.len()
        } else {
            0
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        w_vec: [i64; n],
    };
    let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    for i in 0..n - 1 {
        pq.push((w_vec[i] + w_vec[i + 1], i));
    }
    let mut v_pidx: Vec<Option<(Vertex, usize)>> = w_vec
        .iter()
        .enumerate()
        .map(|(i, w)| Some((Vertex::Leaf(*w), i + 1)))
        .collect();

    let mut ans = 0;
    while let Some((min_w, min_i)) = pq.pop() {
        if let Some((v_main, i_pair)) = v_pidx.get(min_i).unwrap().clone() {
            if let Some((v_pair, _)) = v_pidx.get(i_pair).unwrap().clone() {
                if min_w == v_main.clone() + v_pair.clone() {
                    ans += min_w;
                    let mut sh: SkewHeap = SkewHeap::from(min_w);
                    match v_main {
                        Vertex::Tree(added_sh) => sh.meld(added_sh),
                        Vertex::Leaf(added_w) => sh.push(added_w),
                    }
                    match v_pair {
                        Vertex::Tree(added_sh) => sh.meld(added_sh),
                        Vertex::Leaf(added_w) => sh.push(added_w),
                    }
                    *v_pidx.get_mut(i_pair).unwrap() = None;
                    let mut next_i = i_pair;
                    while v_pidx.get(next_i).is_none() {
                        next_i += 1;
                    }
                    *v_pidx.get_mut(min_i).unwrap() = Some((Vertex::Tree(sh.clone()), next_i));
                    let pushed_w = sh.root().unwrap()
                        + match v_pidx.get(next_i) {
                            Some(Some((Vertex::Tree(other), _))) => other.root().unwrap(),
                            Some(Some((Vertex::Leaf(other), _))) => *other,
                            _ => 0,
                        };
                    pq.push((pushed_w, min_i));
                }
            }
        }
    }
    println!("{}", ans);
}
