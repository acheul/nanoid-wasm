fn main() {

  use nanoid_wasm::nanoid;

  for _ in 0..5 {
    let id = nanoid!(); // 21 characters
    println!("{}", id);
  }

  for _ in 0..5 {
    let id = nanoid!(8); // 8 characters
    println!("{}", id);
  }

  for _ in 0..5 {
    let id = nanoid!(5, &['a', 'b', 'c', 'd']); // 5 characters among ['a', 'b', 'c', 'd']
    println!("{}", id);
  }
}