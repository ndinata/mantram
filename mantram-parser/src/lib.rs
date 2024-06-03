mod parser;

// use wasm_bindgen::prelude::*;

pub fn parse_mantram_string(input: &str) {
    let res = parser::mantram_string(input);
}
