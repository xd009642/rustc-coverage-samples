use std::{
    cell::Cell,
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

async fn join(first: &Cell<u32>, second: &Cell<u32>) -> Result<(u32, u32), ()> {
    futures::try_join!(
        async {
            first.set(first.get() + 1);
            Err::<u32, ()>(())
        },
        async {
            second.set(second.get() + 1);
            Ok::<u32, ()>(7)
        },
    )
}

fn main() {
    let first = Cell::new(0);
    let second = Cell::new(0);
    let mut future = pin!(join(&first, &second));
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(Err(()))
    );
    assert_eq!((first.get(), second.get()), (1, 0));
}
