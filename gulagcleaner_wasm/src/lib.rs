use wasm_bindgen::prelude::*;
use gulagcleaner_rs;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CleaningResult {
    result: Vec<u8>,
    method: u8,
}

#[wasm_bindgen]
pub fn clean_pdf(data: Vec<u8>,force_naive: u8) -> JsValue {
    let (clean_pdf,method_code) = gulagcleaner_rs::clean_pdf(data, force_naive);
    let example = CleaningResult {
        result: clean_pdf,
        method: method_code,
    };
    JsValue::from_serde(&example).unwrap()
}