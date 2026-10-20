use std::panic::{AssertUnwindSafe, catch_unwind};

fn build(before: &mut u32, after: &mut u32, index: usize) -> (u32, u32, u32) {
    (
        {
            *before += 1;
            10
        },
        [20][index],
        {
            *after += 1;
            30
        },
    )
}

fn main() {
    let mut before = 0;
    let mut after = 0;
    let result = catch_unwind(AssertUnwindSafe(|| build(&mut before, &mut after, 1)));
    assert!(result.is_err());
    assert_eq!(before, 1);
    assert_eq!(after, 0);
}
