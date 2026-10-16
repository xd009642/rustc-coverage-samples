fn choose<const FLAG: bool>(hits: &std::cell::Cell<u32>) -> u32 {
    hits.set(hits.get() + 1);
    if FLAG { 11 } else { 22 }
}

fn main() {
    let hits = std::cell::Cell::new(0);
    assert_eq!(choose::<true>(&hits), 11);
    assert_eq!(choose::<false>(&hits), 22);
    assert_eq!(hits.get(), 2);
}
