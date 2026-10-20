use std::panic::{AssertUnwindSafe, catch_unwind};

fn divide(divisor: u32, before: &mut u32, after: &mut u32) -> u32 {
    *before += 1;
    let value = 10 / divisor;
    *after += 1;
    value + 1
}

fn main() {
    let mut before = 0;
    let mut after = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| { divide(0, &mut before, &mut after) })).is_err());
    assert_eq!((before, after), (1, 0));
}
