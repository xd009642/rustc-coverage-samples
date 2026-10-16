use std::{
    cell::Cell,
    sync::atomic::{AtomicU32, Ordering},
};

static INITIALIZATIONS: AtomicU32 = AtomicU32::new(0);

thread_local! {
    static VALUE: Cell<u32> = {
        INITIALIZATIONS.fetch_add(1, Ordering::Relaxed);
        Cell::new(41)
    };
}

fn read() -> u32 {
    VALUE.with(Cell::get)
}

fn main() {
    assert_eq!(read(), 41);
    assert_eq!(read(), 41);
    assert_eq!(INITIALIZATIONS.load(Ordering::Relaxed), 1);
}
