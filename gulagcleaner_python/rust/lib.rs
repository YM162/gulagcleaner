use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn clean_pdf(data: Vec<u8>, force_naive: bool) -> PyResult<(Vec<u8>, u8)> {
    let (clean_pdf, method_code) = gulagcleaner_rs::clean_pdf(data, force_naive);
    Ok((clean_pdf, method_code))
}

#[pymodule]
fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(clean_pdf, m)?)
}
