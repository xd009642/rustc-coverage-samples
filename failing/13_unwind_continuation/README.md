before simplification

```rust
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

fn may_panic(fail: bool) {
    if fail {
        panic!("intentional unwind");
    }
}

fn work(after: &Cell<u32>) {
    may_panic(std::hint::black_box(true));
    after.set(after.get() + 1);
}

fn main() {
    let after = Cell::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| work(&after))).is_err());
    assert_eq!(after.get(), 0);
}
```
