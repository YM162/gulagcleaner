use pyo3::prelude::*;

mod python_binder;

/// I only need to replace the sum_as_string function with the clean_pdf function
#[pymodule]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(python_binder::sum_as_string, m)?)?;
    Ok(())
}
