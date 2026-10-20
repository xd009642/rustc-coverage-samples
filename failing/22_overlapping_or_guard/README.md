before simplification

```rust
use std::cell::Cell;

fn choose(value: u32, checks: &Cell<u32>) -> u32 {
    match value {
        1 | _
            if {
                checks.set(checks.get() + 1);
                false
            } =>
        {
            10
        }
        _ => 20,
    }
}

fn main() {
    let checks = Cell::new(0);
    assert_eq!(choose(1, &checks), 20);
    assert_eq!(checks.get(), 2);
}
```
