fn select(first: Option<u32>, second: Option<u32>) -> u32 {
    if let Some(a) = first
        && a > 2
        && let Some(b) = second
        && b > a
    {
        a + b
    } else {
        99
    }
}

fn main() {
    assert_eq!(select(None, Some(8)), 99);
    assert_eq!(select(Some(1), Some(8)), 99);
    assert_eq!(select(Some(3), None), 99);
    assert_eq!(select(Some(3), Some(2)), 99);
    assert_eq!(select(Some(3), Some(8)), 11);
}
