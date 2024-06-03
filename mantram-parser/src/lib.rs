#![allow(non_snake_case, clippy::empty_docs)]

mod parser;

use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use parser::Character;

#[wasm_bindgen(js_name = "parseMantramString")]
pub fn parse_mantram_string(input: &str) -> Result<CharacterList, JsValue> {
    let (_, chars) =
        parser::mantram_string(input).map_err(|_| "parsing failed.")?;

    Ok(CharacterList(chars))
}

// A newtype around `Vec<Character>` is used as the return type because `Vec<T>`
// doesn't seem to work with `wasm_bindgen` when `T` is "tsified".
// Reference: https://github.com/rustwasm/wasm-bindgen/discussions/3697
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct CharacterList(Vec<Character>);
