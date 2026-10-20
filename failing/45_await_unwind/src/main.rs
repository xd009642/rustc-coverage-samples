use std::{
    future::{Future, poll_fn},
    panic::{AssertUnwindSafe, catch_unwind},
    pin::pin,
    task::{Context, Poll, Waker},
};

async fn operation(polls: &mut u32, after: &mut u32) -> u32 {
    let value = poll_fn(|_| {
        *polls += 1;
        if std::env::args().len() == 1 {
            panic!("child poll fails");
        }
        Poll::Ready(7)
    })
    .await;
    *after += 1;
    value + 1
}

fn main() {
    let mut polls = 0;
    let mut after = 0;
    {
        let mut future = pin!(operation(&mut polls, &mut after));
        let mut context = Context::from_waker(Waker::noop());
        assert!(catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context))).is_err());
    }
    assert_eq!((polls, after), (1, 0));
}
