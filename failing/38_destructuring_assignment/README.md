before simplification

```rust
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

fn assign(values: &mut [u32; 1], second: &Cell<u32>, after: &Cell<u32>, index: usize) {
    (values[0], values[index]) = (11, {
        second.set(second.get() + 1);
        22
    });
    after.set(after.get() + 1);
}

fn main() {
    let mut values = [0];
    let second = Cell::new(0);
    let after = Cell::new(0);
    assert!(
        catch_unwind(AssertUnwindSafe(|| assign(
            &mut values,
            &second,
            &after,
            std::hint::black_box(1)
        )))
        .is_err()
    );
    assert_eq!(values, [11]);
    assert_eq!(second.get(), 1);
    assert_eq!(after.get(), 0);
}
```
