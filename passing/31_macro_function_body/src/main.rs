use std::sync::atomic::{AtomicU32, Ordering};

static HITS: AtomicU32 = AtomicU32::new(0);

macro_rules! define {
    ($body:block) => { fn generated() -> u32 $body };
}

define!({
    HITS.fetch_add(1, Ordering::Relaxed);
    42
});

fn main() {
    assert_eq!(generated(), 42);
    assert_eq!(HITS.load(Ordering::Relaxed), 1);
}
