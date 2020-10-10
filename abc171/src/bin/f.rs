use proconio::{fastout, input};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const MOD: i64 = 1_000_000_007;

#[derive(Debug, PartialEq, Clone)]
struct Mint(i64);

impl Mint {
    fn new(x: i64) -> Self {
        Self((x % MOD + MOD) % MOD)
    }

    fn pow(mut self, t: i64) -> Self {
        let ans = if t == 0 {
            self.0 = 1;
            self
        } else {
            let mut a = self.clone().pow(t >> 1);
            a *= a.clone();
            if t == 1 {
                a *= self;
            }
            a
        };
        ans
    }

    fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
}

impl Neg for Mint {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.0)
    }
}

impl AddAssign for Mint {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = match self.0 + rhs.0 {
            x if x >= MOD => x - MOD,
            x => x,
        };
    }
}

impl SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = match self.0 + MOD - rhs.0 {
            x if x >= MOD => x - MOD,
            x => x,
        };
    }
}

impl MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 * rhs.0) % MOD
    }
}

impl Add for Mint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut ans = self;
        ans += rhs;
        ans
    }
}

impl Sub for Mint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut ans = self;
        ans -= rhs;
        ans
    }
}

impl Mul for Mint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ans = self;
        ans *= rhs;
        ans
    }
}

impl DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl Div for Mint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut ans = self;
        ans /= rhs;
        ans
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Combination {
    fact: Vec<Mint>,
    ifact: Vec<Mint>,
}

impl Combination {
    fn new(n: usize) -> Self {
        assert!(n < MOD as usize);
        let mut fact_temp = Mint::new(1);

        let fact: Vec<Mint> = (0..=n)
            .map(|i| {
                if i == 0 {
                    fact_temp.clone()
                } else {
                    fact_temp = fact_temp.clone() * Mint::new(i as i64);
                    fact_temp.clone()
                }
            })
            .collect();
        let mut ifact_temp = fact
            .get(n)
            .expect("error at generating ifact[n]")
            .clone()
            .inv();
        let ifact = (0..=n)
            .rev()
            .map(|i| {
                if i == n {
                    ifact_temp.clone()
                } else {
                    ifact_temp = ifact_temp.clone() * Mint::new(i as i64 + 1);
                    ifact_temp.clone()
                }
            })
            .collect();

        Self { fact, ifact }
    }

    fn cal_c(&self, n: usize, k: usize) -> Mint {
        if k > n {
            Mint::new(0)
        } else {
            self.fact.get(n).expect("error at fact[n]").clone()
                * self.ifact.get(k).expect("error at ifact[k]").clone()
                * self
                    .ifact
                    .get(n - k)
                    .expect("error at ifact[n - k]")
                    .clone()
        }
    }
}

#[fastout]
fn main() {
    input! {
        k: usize,
        s: String,
    };
    let n: usize = s.len();
    let comb: Combination = Combination::new(n + k);
    let mut ans: Mint = Mint::new(0);

    (0..=k).for_each(|i| {
        let mut now = Mint::new(26).pow(k as i64 - i as i64);
        now *= Mint::new(25).pow(i as i64);
        now *= comb.cal_c(i + n - 1, n - 1);
        ans += now;
    });

    println!("{}", ans.0);
}
