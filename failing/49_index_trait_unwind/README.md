before simplification

```rust
use std::{
    cell::Cell,
    ops::Index,
    panic::{AssertUnwindSafe, catch_unwind},
};

struct Indexed<'a> {
    calls: &'a Cell<u32>,
    value: u32,
}

impl Index<usize> for Indexed<'_> {
    type Output = u32;

    fn index(&self, index: usize) -> &u32 {
        self.calls.set(self.calls.get() + 1);
        if index > 0 {
            panic!("no such element");
        }
        &self.value
    }
}

fn read(items: &Indexed<'_>, index: usize, after: &Cell<u32>) -> u32 {
    let value = items[index];
    after.set(after.get() + 1);
    value + 1
}

fn main() {
    let calls = Cell::new(0);
    let after = Cell::new(0);
    let items = Indexed {
        calls: &calls,
        value: 7,
    };
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            read(&items, std::hint::black_box(1), &after)
        }))
        .is_err()
    );
    assert_eq!((calls.get(), after.get()), (1, 0));
}
```
