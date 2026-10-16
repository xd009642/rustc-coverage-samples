use std::{
    cell::Cell,
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

#[async_trait::async_trait(?Send)]
trait Work {
    async fn run(&self, ok: bool) -> Result<u32, ()>;
}

#[async_trait::async_trait(?Send)]
impl Work for Cell<u32> {
    async fn run(&self, ok: bool) -> Result<u32, ()> {
        self.set(self.get() + 1);
        let value = std::future::ready(if ok { Ok(4) } else { Err(()) }).await?;
        Ok(value + 1)
    }
}

fn main() {
    let runs = Cell::new(0);
    for (input, expected) in [(false, Err(())), (true, Ok(5))] {
        let mut future = pin!(runs.run(input));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(expected)
        );
    }
    assert_eq!(runs.get(), 2);
}
