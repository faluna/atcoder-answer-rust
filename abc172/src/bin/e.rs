use proconio::{fastout, input};

const MOD: i64 = 1_000_000_000 + 7;

fn mypow(num: i64, powidx: i64) -> i64 {
    if powidx == 0 {
        return 1;
    } else {
        let mut a = mypow(num, powidx >> 1);
        a = a * a % MOD;
        if powidx & 1 == 1 {
            a *= num;
        }
        a % MOD
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut fact: Vec<i64> = Vec::new();
    let upper_limit: i64 = 5 * 1_00_000 + 1;
    fact.push(1);
    for i in 1..upper_limit {
        fact.push((fact[i as usize - 1] * i) % MOD);
    }
    let mut ifact: Vec<i64> = Vec::new();
    ifact.push(1);
    for i in 1..upper_limit {
        let pow_fact = mypow(fact[i as usize], MOD - 2);
        ifact.push(pow_fact);
    }
    let pattern_a = (fact[m] * ifact[n] % MOD * ifact[m - n] % MOD * fact[n]) % MOD;
    let mut pattern_b: i64 = 0;
    for i in 0..=n {
        let tmp = (fact[n] * ifact[i] % MOD * ifact[n - i] % MOD)
            * (fact[m - i] * ifact[n - i] % MOD * ifact[m - n] % MOD)
            % MOD
            * fact[n - i]
            % MOD;
        if i % 2 == 0 {
            pattern_b += tmp;
            pattern_b %= MOD;
        } else {
            pattern_b -= tmp;
            pattern_b %= MOD;
        }
    }
    println!("{}", pattern_a * pattern_b % MOD);
}
