#![allow(dead_code)]
/// P minus 1 method
///
/// Determine the prime factors of a number with the p minus 1 method.
/// Effecient for numbers with low ranged prime factors.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use pyo3::{prelude::*, exceptions::PyArithmeticError};

use num::integer::gcd;

use num_bigint::BigInt;

use primes::{Sieve, PrimeSet};

use crate::math::modexp;

/// excecute the p minus one calculation
pub fn p_minus_one(n: u128, max_prime: u128, verbose: bool) -> Result<Vec<u128>, String> {
    assert!(n > 2);
    let m1: u128 = n -1;
    let mut k_parts: Vec<(u128, u32)> = Vec::new();
    let mut prime_parts: Vec<u128> = Vec::new();
    //
    // get a list of the early primes
    let mut pset = Sieve::new();
    if verbose {
        println!("getting list of first {max_prime} primes");
}
    for (_i_prime, prime) in pset.iter().enumerate().take(max_prime as usize) {
        let num: u128 = prime as u128;
        if num > max_prime {
            break;
        }
        let mut exp: u32 = 1;
        if verbose {
            println!("current prime: {num}");
        }
        loop {
            if num.pow(exp + 1) < max_prime {
                exp += 1;
            }
            else {
                break;
            }
        }
        if verbose {
            println!("exponented prime: {}", num.pow(exp));
        }
        k_parts.push((num, exp));
    }
    let mut k = 1u128;
    for (num, exp) in k_parts.clone() {
        k = num.pow(exp) * k;
        if verbose {
            println!("k at step: {k}");
        }
    }
    if verbose {
        println!("k: {k}\nk parts: {:?}", k_parts);
    }

    let a = 2u128;
    let akn1: u128 = ((modexp::modular_exponentiation(
            BigInt::from(a), 
            BigInt::from(k), 
            BigInt::from(n), 
            false)
        ) - BigInt::from(1)).try_into().expect("Number too big");
    if verbose {
        println!("a: {a}\na**k-1 {akn1}");
    }
    let mut next_gcd = gcd(akn1, n);

    if next_gcd == 1 {
        return Err(format!("P minus one does not offer divisor for {n} with max_prime: {max_prime}"));
    }
    let mut q: u128;
    while next_gcd > 1 {
        prime_parts.push(next_gcd);
        q = n / next_gcd;
        next_gcd = gcd(q, n);
        if verbose {
            println!("nextgcd: {next_gcd}|q: {q}");
        }
        if prime_parts.contains(&next_gcd) {
            break;
        }
    }
    return Ok(prime_parts);
}

#[pyfunction]
#[pyo3(name = "p_minus_one")]
/// python wrapper for p_minus_one
pub fn py_p_minus_one(n: u128, max_prime: u128, verbose: bool)-> PyResult<Vec<u128>> {
    let res = p_minus_one(n, max_prime, verbose);
    match res {
        Ok(vec) => Ok(vec),
        Err(e) => Err(PyArithmeticError::new_err(e))
    }
}

/// alternative simple implementation for gcd
pub fn alt_gcd(mut a: u128, mut b: u128) -> u128 {
    let mut tmp: u128;
    while b > 0 {
        tmp = b;
        b = a % b;
        a = tmp;
    }
    return a;
}
