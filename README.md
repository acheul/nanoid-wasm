# NanoId for wasm


## Usage

```toml
# Cargo.toml

[dependencies]
nanoid-wasm = { path="../../nanoid-wasm", features=["not-wasm"] }
```

```rust
// in_your_rust.rs

let size = 21;
let id: String = nanoid_wasm::nanoid(size);
println!("{}", id); // some random id with 21 characters;
```

## Features
There are only two features: `wasm` and `not-wasm`.
Default feature imports nothing. Must designate either of `wasm` or `not-wasm`.

## Demo Page (with leptos)
[https://acheul.github.io/nanoid-wasm/](https://acheul.github.io/nanoid-wasm/)

## Also Look
* [crate nanoid](https://crates.io/crates/nanoid)
* [crate uuid](https://crates.io/crates/uuid)
  * They supports `js` feature for wasm environment.