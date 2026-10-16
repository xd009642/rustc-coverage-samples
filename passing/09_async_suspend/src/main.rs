use std::{
    future::{Future, poll_fn},
    pin::pin,
    task::{Context, Poll, Waker},
};

async fn suspended() -> u32 {
    let mut first = true;
    let value = poll_fn(|_| {
        if first {
            first = false;
            Poll::Pending
        } else {
            Poll::Ready(4)
        }
    })
    .await;
    value + 1
}

fn main() {
    let mut future = pin!(suspended());
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(5));
}
