use std::cell::Cell;

macro_rules! twice {
    ($action:expr) => {{
        $action;
        $action;
    }};
}

fn run(hits: &Cell<u32>) {
    twice!(hits.set(hits.get() + 1));
}

fn main() {
    let hits = Cell::new(0);
    run(&hits);
    assert_eq!(hits.get(), 2);
}
