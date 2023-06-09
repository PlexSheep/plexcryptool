/// # binary module
///
/// This module contains some functions that manipulate binary values.
///
/// ___
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

pub mod pbox6;

use pyo3::prelude::*;

/// rotate 32 bit left
#[pyfunction]
pub fn rotl32 (value: u32, count: u32) -> u32 {
    value.rotate_left(count as u32)
}

/// rotate 32 bit left
#[pyfunction]
pub fn rotr32 (value: u32, count: u32) -> u32 {
    value.rotate_right(count as u32)
}

/// simple xor
#[pyfunction]
pub fn xor(a: u128, b: u128) -> u128 {
    a ^ b
}
