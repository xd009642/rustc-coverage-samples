use std::panic::{AssertUnwindSafe, catch_unwind};

fn condition(checks: &mut u32) -> bool {
    *checks += 1;
    if std::env::args().len() == 1 {
        panic!("condition panic");
    }
    true
}

fn choose(checks: &mut u32, yes: &mut u32, no: &mut u32) -> u32 {
    if std::env::args().len() == 1 && condition(checks) {
        *yes += 1;
        11
    } else {
        *no += 1;
        22
    }
}

fn main() {
    let mut checks = 0;
    let mut yes = 0;
    let mut no = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| choose(&mut checks, &mut yes, &mut no))).is_err());
    assert_eq!((checks, yes, no), (1, 0, 0));
}
