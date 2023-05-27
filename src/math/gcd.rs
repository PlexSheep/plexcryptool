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
/// euclidian algorithm
pub fn gcd(a: u128, b: u128) -> u128 {
    a.gcd(&b)
}
