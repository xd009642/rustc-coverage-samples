use std::panic::{AssertUnwindSafe, catch_unwind};

fn assign(values: &mut [u32; 1], second: &mut u32, after: &mut u32, index: usize) {
    (values[0], values[index]) = (11, {
        *second += 1;
        22
    });
    *after += 1;
}

fn main() {
    let mut values = [0];
    let mut second = 0;
    let mut after = 0;
    assert!(
        catch_unwind(AssertUnwindSafe(|| assign(
            &mut values,
            &mut second,
            &mut after,
            1
        )))
        .is_err()
    );
    assert_eq!(values, [11]);
    assert_eq!(second, 1);
    assert_eq!(after, 0);
}
