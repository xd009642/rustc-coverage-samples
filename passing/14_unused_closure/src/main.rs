use std::cell::Cell;

fn build(runs: &Cell<u32>) -> impl Fn() -> u32 + '_ {
    move || {
        runs.set(runs.get() + 1);
        42
    }
}

fn main() {
    let runs = Cell::new(0);
    let closure = build(&runs);
    let _ = closure;
    assert_eq!(runs.get(), 0);
}
