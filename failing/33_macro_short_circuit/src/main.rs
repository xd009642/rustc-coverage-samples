macro_rules! any_twice {
    ($test:expr) => {
        $test || $test
    };
}

fn run(calls: &mut u32) -> bool {
    any_twice!({
        *calls += 1;
        *calls == 2
    })
}

fn main() {
    let mut calls = 0;
    assert!(run(&mut calls));
    assert_eq!(calls, 2);
}
