# mantram-parser 📿🦀

This is the parser library + wasm wrapper powering [mantram.blog](https://mantram.blog),
implemented with [`nom`](https://docs.rs/nom/latest/nom/).

The following crates are used to handle the Rust -> Typescript journey:

- `wasm-bindgen` + `wasm-pack` for the rust -> wasm heavylifting
- `serde-wasm-bindgen` + `tsify` for easy serialisation + TS typegen
