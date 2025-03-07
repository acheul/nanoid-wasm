#[cfg(feature="wasm")]
mod wasm {
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  extern "C" {
    // Crypto.getRandomValues()
    #[wasm_bindgen(js_namespace = ["globalThis", "crypto"], js_name = getRandomValues, catch)]
    fn get_random_values(buf: &js_sys::Uint8Array) -> Result<(), JsValue>;
  }

  pub fn get_nanoid(size: u32, chars: &[char], chars_len: usize) -> String {
    
    let buf = js_sys::Uint8Array::new_with_length(size);
    get_random_values(&buf).unwrap_throw();
    buf.to_vec().into_iter().map(|i| chars[(i as usize)%chars_len]).collect::<String>()
  }
}

#[cfg(feature="not-wasm")]
mod not_wasm {

  pub fn get_nanoid(size: u32, chars: &[char], chars_len: usize) -> String {
    (0..size).map(|_| chars[rand::random_range(..chars_len)]).collect::<String>()
  }
}


#[cfg(any(feature="wasm", feature="not-wasm"))]
pub fn get_nanoid(size: u32, chars: &[char], chars_len: usize) -> String {

  #[cfg(feature="wasm")]
  return wasm::get_nanoid(size, chars, chars_len);

  #[cfg(feature="not-wasm")]
  return not_wasm::get_nanoid(size, chars, chars_len);
}


/// # Example
/// ```
/// use nanoid_wasm::nanoid;
/// 
/// let id = nanoid_wasm::nanoid(21);
/// println!("{}", id); // some random id with 21 characters;
/// ```
#[cfg(any(feature="wasm", feature="not-wasm"))]
pub fn nanoid(size: u32) -> String {

  let chars: [char; 64] = [
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];

  get_nanoid(size, &chars, chars.len())
}