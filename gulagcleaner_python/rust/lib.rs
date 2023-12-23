use pyo3::prelude::*;

#[pyfunction]
pub fn clean_pdf(data: Vec<u8>,force_naive: u8) -> PyResult<Vec<u8>> {
    Ok(gulagcleaner_rs::clean_pdf(data, force_naive))
}

#[pymodule]
fn _lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(clean_pdf, m)?)?;
    Ok(())
}

