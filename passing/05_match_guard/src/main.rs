fn classify(input: Option<u32>) -> u32 {
    match input {
        Some(value) if value > 3 => value + 10,
        Some(value) => value + 20,
        None => 30,
    }
}

fn main() {
    assert_eq!(classify(Some(4)), 14);
    assert_eq!(classify(Some(1)), 21);
}
