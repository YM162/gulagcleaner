use gulagcleaner_rs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn clean_pdf(data: Vec<u8>, force_naive: bool) -> Vec<u8> {
    let (mut clean_pdf, method_code) = gulagcleaner_rs::clean_pdf(data, force_naive);
    // Append method code as the last byte so JS can read it
    // JS usage: const method = result[result.length - 1]; const pdf = result.slice(0, -1);
    clean_pdf.push(method_code);
    clean_pdf
}
