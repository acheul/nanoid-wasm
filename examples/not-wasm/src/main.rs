fn main() {

  for _ in 0..10 {

    let id = nanoid_wasm::nanoid(21);
    println!("{}", id);
  }
}
