before simplification

```rust
use std::{
    cell::Cell,
    ops::Add,
    panic::{AssertUnwindSafe, catch_unwind},
};

struct Number<'a>(&'a Cell<u32>);

impl Add for Number<'_> {
    type Output = u32;
    fn add(self, _: Self) -> u32 {
        self.0.set(self.0.get() + 1);
        if std::hint::black_box(true) {
            panic!("operator panic");
        }
        7
    }
}

fn sum(calls: &Cell<u32>, after: &Cell<u32>) -> u32 {
    let total = Number(calls) + Number(calls);
    after.set(after.get() + 1);
    total + 1
}

fn main() {
    let calls = Cell::new(0);
    let after = Cell::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| sum(&calls, &after))).is_err());
    assert_eq!((calls.get(), after.get()), (1, 0));
}
```
