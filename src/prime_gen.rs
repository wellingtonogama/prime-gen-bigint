use num_bigint::BigInt;
use num_traits::Zero;

pub mod prime_prob_test;

#[allow(dead_code)]
//número de testes para a função prime_m_rand()
const N: u32 = 8;

//quantidade de números primos gerados
const L: usize = 10;

fn next_prime(n: &BigInt) -> BigInt {
    let mut p = n.clone();
    if &p % 2 == BigInt::zero() {
        p = p + 1;
    }
    while !prime_prob_test::prime_test_m_rand(N, &p) {
        p = p + 2;
    }
    p
}

pub fn prime_list(n: &BigInt) -> Vec<BigInt> {
    let mut vec: Vec<BigInt> = Vec::new();
    let mut num = n.clone();
    for _ in 0..L {
        let p = next_prime(&num);
        vec.push(p.clone());
        num = p + 2;
    }
    vec
}