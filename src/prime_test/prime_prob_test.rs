use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};

fn prime_test(a: &BigInt, n: &BigInt) -> bool {
    let mut s: u32 = 0;
    let mut d = n - 1;
    while &d % 2 == BigInt::zero() {
        s += 1;
        d = d / 2;
    }
    let mut b = a.modpow(&d, &n);
    if &b % n == BigInt::one() {
        return true;
    }
    for _ in 0..s {
        if &b % n == BigInt::from(-1) || &b % n == BigInt::from(n - 1) {
            return true;
        }
        b = &b * &b;
    }
    false
}

pub fn prime_test_m(k: u32, n: &BigInt) -> bool {
    let l = BigInt::from(2);
    let h = n - 2;
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a = rng.gen_bigint_range(&l, &h);
        if !prime_test(&a, n) {
            return false
        }
    }
    true
}