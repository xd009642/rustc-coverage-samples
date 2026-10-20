before simplification

```rust
use std::cell::Cell;

fn accepts(input: Result<u32, u32>, checks: &Cell<u32>) -> bool {
    matches!(input, Ok(value) | Err(value) if {
        checks.set(checks.get() + 1);
        value > 4
    })
}

fn main() {
    let checks = Cell::new(0);
    assert!(accepts(Err(7), &checks));
    assert_eq!(checks.get(), 1);
}
```
