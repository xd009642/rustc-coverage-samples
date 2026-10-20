before simplification

```rust
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::OnceLock,
};

fn read(cache: &OnceLock<u32>, attempts: &Cell<u32>, after: &Cell<u32>) -> u32 {
    let value = cache.get_or_init(|| {
        attempts.set(attempts.get() + 1);
        if attempts.get() == 1 {
            panic!("first initialization fails");
        }
        42
    });
    after.set(after.get() + 1);
    *value
}

fn main() {
    let cache = OnceLock::new();
    let attempts = Cell::new(0);
    let after = Cell::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| read(&cache, &attempts, &after))).is_err());
    assert_eq!(cache.get(), None);
    assert_eq!(read(&cache, &attempts, &after), 42);
    assert_eq!((attempts.get(), after.get()), (2, 1));
}
```
