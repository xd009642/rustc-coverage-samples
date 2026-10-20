macro_rules! twice {
    ($action:expr) => {{
        $action;
        $action;
    }};
}

fn run(hits: &mut u32) {
    twice!(*hits += 1);
}

fn main() {
    let mut hits = 0;
    run(&mut hits);
    assert_eq!(hits, 2);
}
