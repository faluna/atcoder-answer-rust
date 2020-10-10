use proconio::{fastout, input};

#[derive(Debug, Clone)]
struct Sieve {
    n: i128,
    f: Vec<i128>,
    primes: Vec<i128>,
}

impl Sieve {
    fn new(n: i128) -> Self {
        let mut f = vec![-1; n as usize + 1];
        let mut primes: Vec<i128> = Vec::new();
        (0..2).for_each(|_| f.push(-1));
        for i in 2..=n as usize {
            if let Some(_) = f.get(i) {
                continue;
            }
            primes.push(i as i128);
            f[i] = i as i128;
            for j in (i * i..=n as usize).step_by(i) {
                match f.get(j as usize) {
                    Some(-1) => {
                        let value = f.get_mut(j as usize).expect("");
                        *value = i as i128;
                    }
                    Some(_) => (),
                    None => (),
                }
            }
        }
        Self { n, f, primes }
    }

    fn is_prime(&self, x: i128) -> bool {
        self.f[x as usize] == x
    }

    fn factor_list(&self, x: i128) -> Vec<i128> {
        let mut res: Vec<i128> = Vec::new();
        let mut _x_copy = x.clone();
        for p in self.primes.iter() {
            while _x_copy % *p as i128 == 0 {
                res.push(self.f[x as usize]);
                _x_copy /= self.f[x as usize];
            }
        }
        if _x_copy != 1 {
            res.push(x);
        }
        res
    }

    fn factor(&self, x: i128) -> Vec<(i128, i128)> {
        let fl: Vec<i128> = self.factor_list(x);
        if fl.len() == 0 {
            return Vec::new();
        }
        let mut res: Vec<(i128, i128)> = vec![(fl[0], 0)];
        for p in fl.iter() {
            if res.last().expect("").0 == *p {
                let mut _elem = res.last().expect("").1;
                _elem += 1;
            } else {
                res.push((*p, 1));
            }
        }
        res
    }
}

impl Default for Sieve {
    fn default() -> Self {
        Self {
            n: 1,
            f: Vec::new(),
            primes: Vec::new(),
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: i128,
    }
    let sieve = Sieve::new(n);
    let fs = sieve.factor(n as i128);
    let mut ans = 0;

    for p in fs {
        let mut x = p.1;
        let mut b = 1;
        while b <= x {
            x -= b;
            b += 1;
            ans += 1;
        }
    }
    println!("{}", ans)
}
