use pyo3::prelude::*;

/// I only need to replace the sum_as_string function with the clean_pdf function, and the arguments to be u8 vectors. Maybe I need to change some types here to pure rust.
#[pyfunction]
pub fn clean_pdf(data: Vec<u8>,force_naive: u8) -> PyResult<Vec<u8>> {
    Ok(gulagcleaner_rs::clean_pdf(data, force_naive))
}

/// I only need to replace the sum_as_string function with the clean_pdf function
#[pymodule]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(clean_pdf, m)?)?;
    Ok(())
}

