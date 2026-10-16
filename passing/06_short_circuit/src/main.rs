fn choose(left: bool, middle: bool, right: bool) -> bool {
    left && (middle || right)
}

fn main() {
    assert!(!choose(false, false, false));
    assert!(choose(true, true, false));
    assert!(!choose(true, false, false));
}
