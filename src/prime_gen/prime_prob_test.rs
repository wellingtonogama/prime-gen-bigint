use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};

const PRIMES: [i32; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

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
        b = b % n;
    }
    false
}

#[allow(dead_code)]
pub fn prime_test_m_rand(tests: u32, n: &BigInt) -> bool {
    let l = BigInt::from(2);
    let h = n - 2;
    let mut rng = rand::thread_rng();
    for _ in 0..tests {
        let a = rng.gen_bigint_range(&l, &h);
        if !prime_test(&a, n) {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn prime_test_m(n: &BigInt) -> bool {
    for i in 0..PRIMES.len() {
        if !prime_test(&BigInt::from(PRIMES[i]), n) {
            return false;
        }
    }
    true
}