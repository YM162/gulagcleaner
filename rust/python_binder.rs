use pyo3::prelude::*;

#[path = "gulagcleaner/gulagcleaner.rs"] mod gulagcleaner;

/// I only need to replace the sum_as_string function with the clean_pdf function, and the arguments to be u8 vectors. Maybe I need to change some types here to pure rust.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(gulagcleaner::sum_as_string(a, b))
}