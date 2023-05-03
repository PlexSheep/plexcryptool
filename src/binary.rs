#![allow(dead_code)]
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
