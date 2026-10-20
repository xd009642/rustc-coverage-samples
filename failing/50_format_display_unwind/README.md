before simplification

```rust
use std::{
    cell::Cell,
    fmt::{self, Display, Write},
    panic::{AssertUnwindSafe, catch_unwind},
};

struct Value<'a>(&'a Cell<u32>);

impl Display for Value<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.set(self.0.get() + 1);
        if std::hint::black_box(true) {
            panic!("formatting fails");
        }
        formatter.write_str("value")
    }
}

fn render(calls: &Cell<u32>, after: &Cell<u32>) {
    let mut output = String::new();
    write!(output, "{}", Value(calls)).expect("writing to String cannot return fmt::Error");
    after.set(after.get() + 1);
}

fn main() {
    let calls = Cell::new(0);
    let after = Cell::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| render(&calls, &after))).is_err());
    assert_eq!((calls.get(), after.get()), (1, 0));
}
```
