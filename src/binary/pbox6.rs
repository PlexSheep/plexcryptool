#![allow(dead_code)]
/// # pbox 6
///
/// This module contains a simple 8 bit pbox.
///
/// ___
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

use pyo3::prelude::*;

#[test]
fn test_pbox6() {
    assert_eq!(pbox6(0b11110000), 0b10110100);
}

#[pyfunction]
/// Basic 8 bit pbox for an assignment
pub fn pbox6(n: u8) -> u8 {
    return (n & 0b10101010) | ((n & 0b01010100) >> 2 ) | ((n & 1) << 7);
}
