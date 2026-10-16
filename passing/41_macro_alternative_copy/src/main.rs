use std::cell::Cell;

macro_rules! either {
    ($flag:expr, $body:expr) => {
        if $flag { $body } else { $body }
    };
}

fn run(flag: bool, calls: &Cell<u32>) -> u32 {
    either!(flag, {
        calls.set(calls.get() + 1);
        17
    })
}

fn main() {
    let calls = Cell::new(0);
    assert_eq!(run(std::hint::black_box(false), &calls), 17);
    assert_eq!(calls.get(), 1);
}
