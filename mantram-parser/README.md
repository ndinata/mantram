# mantram-parser 📿🦀

This is the parser library + wasm wrapper powering [mantram.blog](https://mantram.blog),
implemented with [`nom`](https://docs.rs/nom/latest/nom/).

The following crates are used to handle the Rust -> Typescript journey:

- `wasm-bindgen` + `wasm-pack` for rust -> wasm heavylifting
- `serde-wasm-bindgen` + `tsify` for easy serialisation + TS typegen

[vite-plugin-wasm](https://github.com/Menci/vite-plugin-wasm) helps with getting
the wasm output work nicely with Astro/Vite.
