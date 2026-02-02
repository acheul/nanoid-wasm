#[cfg(target_family = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::*;

    pub fn get_nanoid_with_chars(size: usize, chars: &[char]) -> String {
        let chars_len = chars.len();
        let mut buf: Vec<u8> = vec![0u8; size];
        let crypto = web_sys::window().unwrap_throw().crypto().unwrap_throw();
        crypto
            .get_random_values_with_u8_array(&mut buf)
            .unwrap_throw();
        buf.into_iter()
            .map(|i| chars[(i as usize) % chars_len])
            .collect::<String>()
    }
}

#[cfg(not(target_family = "wasm"))]
mod not_wasm {

    pub fn get_nanoid_with_chars(size: usize, chars: &[char]) -> String {
        let chars_len = chars.len();
        (0..size)
            .map(|_| chars[rand::random_range(..chars_len)])
            .collect::<String>()
    }
}

/// # Example
/// ```rust, no_run
/// use nanoid_wasm::get_nanoid_with_chars;
///
/// let id = nanoid_wasm::get_nanoid_with_chars(21, &['a', 'b', 'c', 'd']);
/// println!("{}", id); // some random id with 21 characters among ['a', 'b', 'c', 'd']
/// ```
pub fn get_nanoid_with_chars(size: usize, chars: &[char]) -> String {
    #[cfg(target_family = "wasm")]
    return wasm::get_nanoid_with_chars(size, chars);

    #[cfg(not(target_family = "wasm"))]
    return not_wasm::get_nanoid_with_chars(size, chars);
}

/// # Example
/// ```rust, no_run
/// use nanoid_wasm::get_nanoid;
///
/// let id = nanoid_wasm::get_nanoid(21);
/// println!("{}", id); // some random id with 21 characters;
/// ```
pub fn get_nanoid(size: usize) -> String {
    let chars: [char; 64] = [
        '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    get_nanoid_with_chars(size, &chars)
}

/// # Example
///
/// ```rust
/// use nanoid_wasm::nanoid;
///
/// let size = 21;
/// let id: String = nanoid!(); // 21 characters
/// assert_eq!(id.len(), 21);
/// println!("{}", id); // some random id with 21 characters;
///
/// let id: String = nanoid!(8); // 8 characters
/// assert_eq!(id.len(), 8);
/// println!("{}", id);
///
/// let chars = ['a', 'b', 'c', 'd'];
/// let id = nanoid!(5, &chars); // 5 characters among ['a', 'b', 'c', 'd']
/// assert_eq!(id.len(), 5);
/// assert!(id.chars().all(|c| chars.contains(&c)));
/// println!("{}", id);
/// ```
#[macro_export]
macro_rules! nanoid {
    () => {
        nanoid_wasm::get_nanoid(21)
    };
    ($size:expr) => {
        nanoid_wasm::get_nanoid($size)
    };
    ($size:expr, $chars:expr) => {
        nanoid_wasm::get_nanoid_with_chars($size, $chars)
    };
}
