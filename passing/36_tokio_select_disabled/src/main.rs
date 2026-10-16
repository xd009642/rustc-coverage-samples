use std::{
    cell::Cell,
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

async fn choose(created: &Cell<u32>, polled: &Cell<u32>, selected: &Cell<u32>) -> u32 {
    tokio::select! {
        biased;
        value = {
            created.set(created.get() + 1);
            async { polled.set(polled.get() + 1); 99_u32 }
        }, if false => value,
        value = std::future::ready(7_u32) => {
            selected.set(selected.get() + 1);
            value + 1
        }
    }
}

fn main() {
    let created = Cell::new(0);
    let polled = Cell::new(0);
    let selected = Cell::new(0);
    let mut future = pin!(choose(&created, &polled, &selected));
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(8)
    );
    assert_eq!((created.get(), polled.get(), selected.get()), (1, 0, 1));
}
