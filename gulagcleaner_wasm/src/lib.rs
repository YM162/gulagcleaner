use wasm_bindgen::prelude::*;
use gulagcleaner_rs;

#[wasm_bindgen]
pub fn clean_pdf(data: Vec<u8>,force_naive: u8) -> Vec<u8> {
    gulagcleaner_rs::clean_pdf(data, force_naive)
}