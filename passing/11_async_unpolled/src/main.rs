use std::cell::Cell;

async fn never_polled(runs: &Cell<u32>) -> u32 {
    runs.set(runs.get() + 1);
    42
}

fn main() {
    let runs = Cell::new(0);
    let future = never_polled(&runs);
    drop(future);
    assert_eq!(runs.get(), 0);
}
