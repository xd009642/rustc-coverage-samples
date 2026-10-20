use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::OnceLock,
};

fn read(cache: &OnceLock<u32>, attempts: &mut u32, after: &mut u32) -> u32 {
    let value = cache.get_or_init(|| {
        *attempts += 1;
        if *attempts == 1 {
            panic!("first initialization fails");
        }
        42
    });
    *after += 1;
    *value
}

fn main() {
    let cache = OnceLock::new();
    let mut attempts = 0;
    let mut after = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| read(&cache, &mut attempts, &mut after))).is_err());
    assert_eq!(cache.get(), None);
    assert_eq!(read(&cache, &mut attempts, &mut after), 42);
    assert_eq!((attempts, after), (2, 1));
}
