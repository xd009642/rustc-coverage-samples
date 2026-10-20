before simplification

```rust
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

fn condition(checks: &Cell<u32>) -> bool {
    checks.set(checks.get() + 1);
    if std::hint::black_box(true) {
        panic!("condition panic");
    }
    true
}

fn choose(checks: &Cell<u32>, yes: &Cell<u32>, no: &Cell<u32>) -> u32 {
    if std::hint::black_box(true) && condition(checks) {
        yes.set(yes.get() + 1);
        11
    } else {
        no.set(no.get() + 1);
        22
    }
}

fn main() {
    let checks = Cell::new(0);
    let yes = Cell::new(0);
    let no = Cell::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| choose(&checks, &yes, &no))).is_err());
    assert_eq!((checks.get(), yes.get(), no.get()), (1, 0, 0));
}
```
