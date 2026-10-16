fn extract(input: Option<u32>) -> u32 {
    let Some(value) = input else {
        return 10;
    };
    value + 1
}

fn main() {
    assert_eq!(extract(None), 10);
    assert_eq!(extract(Some(4)), 5);
}
