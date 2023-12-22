use wasm_bindgen::prelude::*;
use gulagcleaner;

#[wasm_bindgen]
pub fn sum_as_string(a: usize, b: usize) -> String {
    gulagcleaner::sum_as_string(a, b)
}