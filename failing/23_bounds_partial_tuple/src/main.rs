use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

fn build(before: &Cell<u32>, after: &Cell<u32>, index: usize) -> (u32, u32, u32) {
    (
        {
            before.set(before.get() + 1);
            10
        },
        [20][index],
        {
            after.set(after.get() + 1);
            30
        },
    )
}

fn main() {
    let before = Cell::new(0);
    let after = Cell::new(0);
    let result = catch_unwind(AssertUnwindSafe(|| {
        build(&before, &after, std::hint::black_box(1))
    }));
    assert!(result.is_err());
    assert_eq!(before.get(), 1);
    assert_eq!(after.get(), 0);
}
