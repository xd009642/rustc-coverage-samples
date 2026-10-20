use std::panic::{AssertUnwindSafe, catch_unwind};

fn may_panic(fail: bool) {
    if fail {
        panic!("intentional unwind");
    }
}

fn work(after: &mut u32) {
    may_panic(std::env::args().len() == 1);
    *after += 1;
}

fn main() {
    let mut after = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| work(&mut after))).is_err());
    assert_eq!(after, 0);
}
