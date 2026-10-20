use std::{
    fmt::{self, Display, Write},
    panic::{AssertUnwindSafe, catch_unwind},
    sync::atomic::{AtomicU32, Ordering},
};

struct Value<'a>(&'a AtomicU32);

impl Display for Value<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fetch_add(1, Ordering::Relaxed);
        if std::env::args().len() == 1 {
            panic!("formatting fails");
        }
        formatter.write_str("value")
    }
}

fn render(calls: &AtomicU32, after: &AtomicU32) {
    let mut output = String::new();
    write!(output, "{}", Value(calls)).expect("writing to String cannot return fmt::Error");
    after.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    let calls = AtomicU32::new(0);
    let after = AtomicU32::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| render(&calls, &after))).is_err());
    assert_eq!(
        (calls.load(Ordering::Relaxed), after.load(Ordering::Relaxed)),
        (1, 0)
    );
}
