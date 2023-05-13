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
use num_traits::ToPrimitive;

use primes::{Sieve, PrimeSet, is_prime};

use crate::math::modexp;

const MAX_PRIMES: u128 = 80u128;

/// excecute the p minus one calculation
pub fn p_minus_one(n: u128, max_prime: u128, verbose: bool) -> Result<Vec<u128>, String> {
    if n < 3 {
        return Err(format!("n too small: {n}"));
    }
    if max_prime > MAX_PRIMES {
        return Err(format!("max_prime too large: {max_prime}"));
    }
    let mut k_parts: Vec<(u128, u32)> = Vec::new();
    let mut prime_parts: Vec<u128> = Vec::new();
    // get a list of the early primes
    let mut pset = Sieve::new();
    for (_i_prime, prime) in pset.iter().enumerate().take(max_prime as usize) {
        let num: u128 = prime as u128;
        if num > max_prime {
            break;
        }
        let mut exp: u32 = 1;
        loop {
            if num.pow(exp + 1) < max_prime {
                exp += 1;
            }
            else {
                break;
            }
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
    let mut a = 2u128;
    let mut akn1: u128;
    let mut g: u128;
    let mut q: u128;
    let mut n = n;
    println!("=======================================================================");
    loop {
        assert!(n > 1);
        dbg!(&n);
        if verbose {
            println!("modular exponentiation with: a={a} k={k} n={n}");
        }
        akn1 = modexp::modular_exponentiation(
            BigInt::from(a), 
            BigInt::from(k), 
            BigInt::from(n), 
            false).to_u128().expect("Number too large") - 1;
        if verbose {
            println!("a**k - 1 = {a}**{k} - 1 mod {n} = {akn1}");
        }
        g = gcd(akn1, n);
        if verbose {
            println!("g = gcd(akn1, n) = gcd({akn1}, {n}) = {g}");
        }
        if g == 1 {
            println!("=======================================================================");
            return Err(format!("P minus one does not work for this setup. Use another algorithm or choose a higher max prime."));
        }
        if g == n {
            dbg!(&n);
            if verbose {
                println!("g = {g} = {n} = n");
                println!("bad a, using a=a+1");
            }
            a += 1;
        }
        else {
            n = n / g;
            prime_parts.push(g);
            if is_prime(n as u64) {
                prime_parts.push(n);
                break;
            }
        }
        if verbose {
            println!("=======================================================================");
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
