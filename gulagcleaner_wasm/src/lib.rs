use wasm_bindgen::prelude::*;
use gulagcleaner;

#[wasm_bindgen]
pub fn clean_pdf(a: usize, b: usize) -> String {
    gulagcleaner::clean_pdf(a, b)
}