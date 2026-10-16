use std::sync::{
    LazyLock,
    atomic::{AtomicU32, Ordering},
};

static INITIALIZATIONS: AtomicU32 = AtomicU32::new(0);
static VALUE: LazyLock<u32> = LazyLock::new(|| {
    INITIALIZATIONS.fetch_add(1, Ordering::Relaxed);
    23
});

fn read() -> u32 {
    *VALUE
}

fn main() {
    assert_eq!(read(), 23);
    assert_eq!(read(), 23);
    assert_eq!(INITIALIZATIONS.load(Ordering::Relaxed), 1);
}
