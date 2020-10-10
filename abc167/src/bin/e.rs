use proconio::{fastout, input};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const MOD: i64 = 998244353;

#[derive(Debug, Clone, Default, Copy, PartialEq)]
struct ModInt {
    x: i64,
}

impl ModInt {
    fn new(x: i64) -> Self {
        Self { x: x % MOD }
    }

    fn inv(&self) -> Self {
        let res = self.pow(MOD - 2);
        res
    }

    fn pow(&self, t: i64) -> Self {
        let mut res: Self = Self::new(1);
        if t != 0 {
            let mut a: Self = self.pow(t >> 1);
            a.pow(2);
            if t & 1 == 1 {
                a *= self;
            }
            res = a;
        }
        res
    }
}

impl AddAssign<&Self> for ModInt {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        if self.x >= MOD {
            self.x -= MOD;
        }
    }
}

impl Add for ModInt {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += &other;
        self
    }
}

impl SubAssign<&Self> for ModInt {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        if self.x >= MOD {
            self.x -= MOD;
        }
    }
}

impl Sub for ModInt {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= &other;
        self
    }
}

impl MulAssign<&Self> for ModInt {
    fn mul_assign(&mut self, rhs: &Self) {
        self.x *= rhs.x;
        self.x %= MOD;
    }
}

impl Mul for ModInt {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self *= &other;
        self
    }
}

impl DivAssign<&Self> for ModInt {
    fn div_assign(&mut self, rhs: &Self) {
        *self *= &rhs.inv();
    }
}

impl Div for ModInt {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self /= &other;
        self
    }
}

impl From<usize> for ModInt {
    fn from(t: usize) -> Self {
        Self::new(t as i64)
    }
}

impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

struct Combination {
    fact: Vec<ModInt>,
    ifact: Vec<ModInt>,
}

impl Combination {
    fn new(n: usize) -> Self {
        let mut fact = Vec::with_capacity(n);
        let mut ifact = Vec::with_capacity(n);
        fact.push(ModInt::new(0));
        for i in 1..=n {
            fact[i] = fact[i - 1] * ModInt::from(i);
        }
        ifact[n] = fact[n].inv();
        for i in (1..=n).rev() {
            ifact[i - 1] = ifact[i] * ModInt::from(i);
        }
        Self { fact, ifact }
    }

    fn calc(&self, n: usize, k: usize) -> ModInt {
        if k > n {
            ModInt::new(0)
        } else {
            self.fact[n] * self.ifact[k] * self.ifact[n - k]
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let c: Combination = Combination::new(200005);
    let mut ans: ModInt = ModInt::new(0);
    let mut col = ModInt::new(m as i64);
    for x in (0..=(n - 1)).rev() {
        let mut now = col;
        now *= &c.calc(n - 1, x);
        if x <= k {
            ans += &now;
        }
        col *= &ModInt::from(m - 1);
    }
    println!("{}", ans)
}
