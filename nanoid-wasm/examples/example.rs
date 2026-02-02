fn main() {
    use nanoid_wasm::nanoid;

    for _ in 0..5 {
        let id = nanoid!(); // 21 characters
        assert_eq!(id.len(), 21);
        println!("{}", id);
    }

    for _ in 0..5 {
        let id = nanoid!(8); // 8 characters
        assert_eq!(id.len(), 8);
        println!("{}", id);
    }

    for _ in 0..5 {
        let chars = ['a', 'b', 'c', 'd'];
        let id = nanoid!(5, &chars); // 5 characters among ['a', 'b', 'c', 'd']
        assert_eq!(id.len(), 5);
        assert!(id.chars().all(|c| chars.contains(&c)));
        println!("{}", id);
    }
}
