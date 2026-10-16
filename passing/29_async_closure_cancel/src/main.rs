use std::{
    cell::Cell,
    future::{Future, pending},
    task::{Context, Poll, Waker},
};

fn run(before: &Cell<u32>, after: &Cell<u32>) {
    let action = async || {
        before.set(before.get() + 1);
        pending::<()>().await;
        after.set(after.get() + 1);
    };
    let mut future = Box::pin(action());
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Pending
    );
    drop(future);
}

fn main() {
    let before = Cell::new(0);
    let after = Cell::new(0);
    run(&before, &after);
    assert_eq!(before.get(), 1);
    assert_eq!(after.get(), 0);
}
