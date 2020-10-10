use proconio::input;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a_b_vec: [(i64, usize); n],
        c_d_vec: [(i64, usize); q],
    };
    let mut yochien_vec: Vec<BTreeMap<i64, usize>> = Vec::new();
    for _ in 0..(2.0e5 as usize) {
        yochien_vec.push(BTreeMap::new());
    }
    a_b_vec.iter().for_each(|(a, b)| {
        if let Some(yochien) = yochien_vec.get_mut(*b - 1) {
            *yochien.entry(*a).or_insert(0) += 1;
        }
    });
    let yochiens: Rc<RefCell<Vec<BTreeMap<i64, usize>>>> = Rc::new(RefCell::new(yochien_vec));
    let mut saikyo_map: BTreeMap<i64, usize> = BTreeMap::new();
    (*yochiens).borrow().iter().for_each(|yochien| {
        if let Some(saikyo) = yochien.iter().rev().next() {
            *saikyo_map.entry(*saikyo.0).or_insert(0) += 1;
        };
    });
    let saikyos = Rc::new(RefCell::new(saikyo_map));

    let get_max = |i: usize| -> i64 {
        // let yochien = yochien_vec.get(i).expect("error at get_max 1");
        if yochiens.borrow().get(i).expect("error at get_max 1").len() == 0usize {
            -1
        } else {
            // let (key, _) = yochien_vec
            *yochiens
                .borrow()
                .get(i)
                .expect("error at get_max 2")
                .iter()
                .rev()
                .next()
                .expect("error at get_max 3")
                .0
        }
    };

    let add_yochien = |i: usize| {
        let x: i64 = get_max(i);
        if x == -1 {
            return;
        }
        *saikyos.borrow_mut().entry(x).or_insert(0) += 1;
    };

    let del_yochien = |i: usize| {
        let x = get_max(i);
        if x == -1 {
            return;
        }
        if let Some(saikyo_num) = saikyos.borrow_mut().remove(&x) {
            match saikyo_num {
                1 => (),
                n => {
                    let _ = saikyos.borrow_mut().entry(x).or_insert(n - 1);
                }
            }
        }
    };

    let add_enji = |i: usize, x: i64| {
        del_yochien(i);
        *yochiens
            .borrow_mut()
            .get_mut(i)
            .expect("error at add_enji 1")
            .entry(x)
            .or_insert(0) += 1;
    };

    let del_enji = |i: usize, x: i64| {
        del_yochien(i);
        if let Some(yochien_num) = yochiens
            .borrow_mut()
            .get_mut(i)
            .expect("error at del_enji 1")
            .remove(&x)
        {
            match yochien_num {
                1 => (),
                n => {
                    let _ = yochiens
                        .borrow_mut()
                        .get_mut(i)
                        .expect("error at del_enji 2")
                        .entry(x)
                        .or_insert(n - 1);
                }
            }
        }
        add_yochien(i);
    };

    c_d_vec.iter().for_each(|(c, d)| {
        del_enji(
            a_b_vec.get((c - 1) as usize).expect("error at main 1").1,
            a_b_vec.get((c - 1) as usize).expect("error at main 2").0,
        );
        a_b_vec
            .get_mut((c - 1) as usize)
            .expect("error at main 3")
            .1 = *d;
        add_enji(
            a_b_vec.get((c - 1) as usize).expect("error at main 4").1,
            a_b_vec.get((c - 1) as usize).expect("error at main 5").0,
        );
        println!(
            "{}",
            saikyos.borrow().iter().next().expect("error at main 6").0
        );
    })
}
