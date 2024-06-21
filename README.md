# mantram 📿

[mantram.blog](https://mantram.blog/) is a mobile-friendly compilation of
various Buddhist mantrams/suttas (真言).

I built this site so that my family can recite mantrams on their phones more
easily. No more having to zoom and pan across blurry photos of book pages!

All mantrams are available as Markdown files in [src/content/posts](./src/content/posts/).

For suggestions/corrections feel free to open an issue or email me :)

## tech stack

The site is built with the lovely [Astro](https://astro.build) framework and
[shadcn-ui](https://ui.shadcn.com) React components, and is deployed on GitHub
Pages. The [parser](./mantram-parser/) is implemented in Rust (exposed via wasm).

To run the project locally, you need to have the following installed:

- [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [rust toolchain](https://www.rust-lang.org/tools/install) + [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

```sh
$ git clone https://github.com/ndinata/mantram.git
$ cd mantram
$ wasm-pack build mantram-parser
$ npm install
$ npm run dev # starts a localhost dev server
```
