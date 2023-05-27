#![allow(dead_code)]
/// euclidian algorithm, find greatest common divider
///
/// This does not implement the euclidian algorithm by itself.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use num::Integer;

use pyo3::prelude::*;

#[pyfunction]
/// extended euclidian algorithm
pub fn egcd(mut a: u128, mut b: u128) -> Vec<i128> {
    if a > b {
        let tmp = a;
        a = b;
        b = tmp;
    }
        let egcd = (a as i128).extended_gcd(&(b as i128));
        return vec![egcd.gcd, egcd.x, egcd.y]
}

#[pyfunction]
/// own implementation of egcd
pub fn alt_egcd(mut a: i128, mut b: i128, recursion: bool) -> Vec<i128> {
    if recursion && a > b {
        let tmp = a;
        a = b;
        b = tmp;
    }
    if a == 0 {
        return vec![b, 0, 1]
    }
    let v = alt_egcd(b % a, a, true);
    let mut result =  vec![
        v[0], 
        v[2] - (b.checked_div(a).unwrap()) * v[1], 
        v[1],
    ];
    return result;
}

#[test]
fn test_alt_gcd() {
    assert_eq!(egcd(12193, 123213), alt_egcd(12193, 123213, false));
    assert_eq!(egcd(52193, 123212), alt_egcd(52193, 123212, false));
}

#[pyfunction]
/// euclidian algorithm
pub fn gcd(a: u128, b: u128) -> u128 {
    a.gcd(&b)
}
