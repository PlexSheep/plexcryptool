use pyo3::prelude::*;

mod binary;
mod iterated_squaring;

#[pymodule]
fn register_binary_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let binary_module = PyModule::new(py, "binary")?;
    binary_module.add_function(wrap_pyfunction!(binary::rotl32, binary_module)?)?;
    binary_module.add_function(wrap_pyfunction!(binary::rotr32, binary_module)?)?;
    parent_module.add_submodule(binary_module)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn plexcryptool(py: Python, m: &PyModule) -> PyResult<()> {
    register_binary_module(py, m)?;
    Ok(())
}
