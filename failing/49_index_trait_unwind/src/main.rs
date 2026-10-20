use std::{
    ops::Index,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::atomic::{AtomicU32, Ordering},
};

struct Indexed<'a> {
    calls: &'a AtomicU32,
    value: u32,
}

impl Index<usize> for Indexed<'_> {
    type Output = u32;

    fn index(&self, index: usize) -> &u32 {
        self.calls.fetch_add(1, Ordering::Relaxed);
        if index > 0 {
            panic!("no such element");
        }
        &self.value
    }
}

fn read(items: &Indexed<'_>, index: usize, after: &AtomicU32) -> u32 {
    let value = items[index];
    after.fetch_add(1, Ordering::Relaxed);
    value + 1
}

fn main() {
    let calls = AtomicU32::new(0);
    let after = AtomicU32::new(0);
    let items = Indexed {
        calls: &calls,
        value: 7,
    };
    assert!(catch_unwind(AssertUnwindSafe(|| { read(&items, 1, &after) })).is_err());
    assert_eq!(
        (calls.load(Ordering::Relaxed), after.load(Ordering::Relaxed)),
        (1, 0)
    );
}
