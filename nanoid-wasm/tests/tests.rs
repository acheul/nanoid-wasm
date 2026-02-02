#[test]
fn test_nanoid() {
    use nanoid_wasm::nanoid;
    let a = nanoid!();
    let b = nanoid!();
    assert_eq!(a.len(), 21);
    assert_eq!(b.len(), 21);
    assert_ne!(a, b);

    assert_eq!(nanoid!(8).len(), 8);

    let chars = ['a', 'b', 'c', 'd'];
    let id = nanoid!(5, &chars);
    assert_eq!(id.len(), 5);
    assert!(id.chars().all(|c| chars.contains(&c)));
}

use wasm_bindgen_test::wasm_bindgen_test;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn test_nanoid_wasm() {
    use nanoid_wasm::nanoid;
    let a = nanoid!();
    let b = nanoid!();
    assert_eq!(a.len(), 21);
    assert_eq!(b.len(), 21);
    assert_ne!(a, b);

    assert_eq!(nanoid!(8).len(), 8);

    let chars = ['a', 'b', 'c', 'd'];
    let id = nanoid!(5, &chars);
    assert_eq!(id.len(), 5);
    assert!(id.chars().all(|c| chars.contains(&c)));
}
