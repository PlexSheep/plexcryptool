#![allow(dead_code)]
/// binary functions
///
/// This module contains some functions that manipulate binary values.
///
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

/**
 * Pythons bit operations are trash, so I made a rust lib for that.
 */
use pyo3::prelude::*;

#[pyfunction]
pub fn rotl32 (value: u32, count: u32) -> u32 {
    value.rotate_left(count as u32)
}

#[pyfunction]
pub fn rotr32 (value: u32, count: u32) -> u32 {
    value.rotate_right(count as u32)
}
