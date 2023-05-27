#![allow(dead_code)]
/// factorize a large integer
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use pyo3::prelude::*;

#[pyfunction]
/// find the prime factors of n
pub fn prime_factors(mut n: u128, verbose: bool) -> Vec<u128> {
    let mut i: u128 = 2;
    let mut factors: Vec<u128> = Vec::new();
    while i.pow(2) <= n {
        if n % i > 0 {
            i += 1;
        }
        else {
            n = n.checked_div(i).expect("n / i is not an integer");
            factors.push(i);
        }
        if verbose {
            println!("i={i}\t{:?}", factors);
        }
    }
    if n > 1 {
        factors.push(n);
    }
    return factors;
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(360, true), vec![2, 2, 2, 3, 3, 5]);
    // see https://math.tools/numbers/prime-factors/3603234
    assert_eq!(prime_factors(3603234, true), vec![2, 3, 223, 2693]);
}
