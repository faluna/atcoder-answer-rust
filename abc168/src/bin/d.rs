use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    let mut graph: Vec<Vec<usize>> = Vec::new();
    input! {
        n: usize,
        m: usize,
        a_b: [(usize, usize); m],
    };
    (0..n).for_each(|_| {
        graph.push(Vec::new());
    });
    a_b.iter().for_each(|(a, b)| {
        graph.get_mut(*a).expect("a").push(*b);
        graph.get_mut(*b).expect("b").push(*a);
    });
    let mut dist: Vec<i32> = vec![-1; n];
    let mut que: VecDeque<usize> = VecDeque::new();

    que.push_back(0);
    while let Some(v) = que.pop_front() {
        graph.get(v).expect("c").iter().for_each(|nv| {
            if dist.get(*nv) == Some(&-1) {
                let temp = dist.get(v).expect("d") + 1;
                if let Some(elem) = dist.get_mut(*nv) {
                    *elem = temp
                };
                que.push_back(*nv);
            }
        })
    }
    println!("Yes");
    for vtx in 1..n {
        let dpth = dist.get(vtx).expect("e");
        let pre_vtxes = graph.get(vtx).expect("f");
        for pre_vtx in pre_vtxes {
            if *dist.get(*pre_vtx).expect("g") == dpth - 1 {
                println!("{}", pre_vtx + 1);
            }
        }
    }
}
