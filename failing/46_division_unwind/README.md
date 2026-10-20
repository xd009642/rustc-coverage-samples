before simplification

```rust
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

fn divide(divisor: u32, before: &Cell<u32>, after: &Cell<u32>) -> u32 {
    before.set(before.get() + 1);
    let value = 10 / divisor;
    after.set(after.get() + 1);
    value + 1
}

fn main() {
    let before = Cell::new(0);
    let after = Cell::new(0);
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            divide(std::hint::black_box(0), &before, &after)
        }))
        .is_err()
    );
    assert_eq!((before.get(), after.get()), (1, 0));
}
```
