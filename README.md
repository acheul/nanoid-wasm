# NanoId for wasm

[![Crates.io](https://img.shields.io/crates/v/nanoid-wasm)](https://crates.io/crates/nanoid-wasm)
[![docs.rs](https://img.shields.io/docsrs/nanoid-wasm?label=docs.rs)](https://docs.rs/nanoid-wasm)

## Usage

```toml
# Cargo.toml

[dependencies]
nanoid-wasm = "0.2" # no features required since v0.2. It distinguishes target-family internally.
```

```rust
// in_your_rust.rs

use nanoid_wasm::nanoid;

let size = 21;
let id: String = nanoid!(); // 21 characters
println!("{}", id); // some random id with 21 characters;

let id: String = nanoid!(8); // 8 characters
println!("{}", id);

let id = nanoid!(5, &['a', 'b', 'c', 'd']); // 5 characters among ['a', 'b', 'c', 'd']
println!("{}", id);
```

## Features

No features required since v0.2. The crate distinguishes target-family internally.

You can safely use this crate on both wasm targets and non-wasm targets.

~~There are only two features: `wasm` and `not-wasm`.
Default feature imports nothing. Must designate either of `wasm` or `not-wasm`.~~

## Demo Page (with leptos)

[https://acheul.github.io/nanoid-wasm/](https://acheul.github.io/nanoid-wasm/)

## Also Look

- [crate nanoid](https://crates.io/crates/nanoid)
- [crate uuid](https://crates.io/crates/uuid)
  - They supports `js` feature for wasm environment.

## For Version Changes, look `VERSION.md`

- [VERSION.md](https://github.com/acheul/nanoid-wasm/blob/main/VERSION.md)
