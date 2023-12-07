// use wasm_bindgen::prelude::wasm_bindgen;             SAU cu *: use wasm_bindgen::prelude::*
use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log; //log_1 will log 2 values, log_2 will log 2 values, etc.

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encode_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encode_img);
    data_url
}
