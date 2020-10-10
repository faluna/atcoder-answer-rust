use std::collections::BinaryHeap;
use text_io::read;

fn main() {
    let n: usize = read!();
    let mut a_vec: Vec<i64> = Vec::new();
    (0..n).for_each(|_| a_vec.push(read!()));
    a_vec.sort();
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut ans: i64 = 0;
    for a in a_vec.iter().rev() {
        let temp = heap.pop().unwrap_or(0);
        ans += temp;
        if ans == 0 {
            heap.push(*a);
        } else {
            (0..2).for_each(|_| heap.push(*a));
        }
    }
    println!("{}", ans);
}
