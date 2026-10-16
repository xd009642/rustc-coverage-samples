use std::{
    cell::Cell,
    future::{Future, poll_fn},
    panic::{AssertUnwindSafe, catch_unwind},
    pin::pin,
    task::{Context, Poll, Waker},
};

async fn operation(polls: &Cell<u32>, after: &Cell<u32>) -> u32 {
    let value = poll_fn(|_| {
        polls.set(polls.get() + 1);
        if std::hint::black_box(true) {
            panic!("child poll fails");
        }
        Poll::Ready(7)
    })
    .await;
    after.set(after.get() + 1);
    value + 1
}

fn main() {
    let polls = Cell::new(0);
    let after = Cell::new(0);
    let mut future = pin!(operation(&polls, &after));
    let mut context = Context::from_waker(Waker::noop());
    assert!(catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context))).is_err());
    assert_eq!((polls.get(), after.get()), (1, 0));
}
