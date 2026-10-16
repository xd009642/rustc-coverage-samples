use std::{
    cell::Cell,
    future::{Future, pending},
    task::{Context, Poll, Waker},
};

async fn cancelled(before: &Cell<u32>, after: &Cell<u32>) {
    before.set(before.get() + 1);
    pending::<()>().await;
    after.set(after.get() + 1);
}

fn main() {
    let before = Cell::new(0);
    let after = Cell::new(0);
    let mut future = Box::pin(cancelled(&before, &after));
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Pending
    );
    drop(future);
    assert_eq!(before.get(), 1);
    assert_eq!(after.get(), 0);
}
