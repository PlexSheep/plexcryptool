#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//!
//! This is a mixed rust/python library that also offers an executable.
//! The intended usage is the solving of tasks for cryptology and maybe math, in the context of a
//! # various tools for use in cryptology contexts
//! university degree. I wrote this for cryptology at DHBW Mannheim.
//!
//! ## main function
//! This project contains an executable, see [main.rs](main.rs)
//!
//! ## lib module
//! This project contains is a library, see [lib.rs](lib.rs).
//! Note that this library offers Python bindings using [PyO3](pyo3.rs)
//! ___
//! Author:     Christoph J. Scherr <software@cscherr.de>
//!
//! License:    MIT
//!
//! Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
use pyo3::prelude::*;

mod binary;
mod math;
mod algo;

#[pymodule]
fn register_binary_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let binary_module = PyModule::new(py, "binary")?;
    binary_module.add_function(wrap_pyfunction!(binary::rotl32, binary_module)?)?;
    binary_module.add_function(wrap_pyfunction!(binary::rotr32, binary_module)?)?;
    binary_module.add_function(wrap_pyfunction!(binary::xor, binary_module)?)?;
    parent_module.add_submodule(binary_module)?;
    Ok(())
}

#[pymodule]
fn register_math_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let math_module = PyModule::new(py, "math")?;
    math_module.add_function(wrap_pyfunction!(math::modexp::py_modular_exponentiation, math_module)?)?;
    math_module.add_function(wrap_pyfunction!(math::pm1::py_p_minus_one, math_module)?)?;
    parent_module.add_submodule(math_module)?;
    Ok(())
}

#[pymodule]
fn register_algo_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let algo_module = PyModule::new(py, "algo")?;
    algo_module.add_function(wrap_pyfunction!(algo::feistel0::encrypt, algo_module)?)?;
    algo_module.add_function(wrap_pyfunction!(algo::feistel0::decrypt, algo_module)?)?;
    algo_module.add_function(wrap_pyfunction!(algo::feistel0::sbox, algo_module)?)?;
    algo_module.add_function(wrap_pyfunction!(algo::feistel0::key_scheduler, algo_module)?)?;
    parent_module.add_submodule(algo_module)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn plexcryptool(py: Python, m: &PyModule) -> PyResult<()> {
    register_binary_module(py, m)?;
    register_math_module(py, m)?;
    Ok(())
}
