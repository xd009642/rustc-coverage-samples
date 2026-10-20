before simplification

```rust
use std::cell::Cell;

macro_rules! any_twice {
    ($test:expr) => {
        $test || $test
    };
}

fn run(calls: &Cell<u32>) -> bool {
    any_twice!({
        calls.set(calls.get() + 1);
        calls.get() == 2
    })
}

fn main() {
    let calls = Cell::new(0);
    assert!(run(&calls));
    assert_eq!(calls.get(), 2);
}
```
