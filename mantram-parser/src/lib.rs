mod parser;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_mantram_string(input: &str) -> Result<JsValue, JsValue> {
    let (_, chars) =
        parser::mantram_string(input).map_err(|_| "parsing failed.")?;

    Ok(to_value(&chars)?)
}
