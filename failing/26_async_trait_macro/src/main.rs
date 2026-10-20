use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

#[async_trait::async_trait(?Send)]
trait Work {
    async fn run(&mut self, ok: bool) -> Result<u32, ()>;
}

#[async_trait::async_trait(?Send)]
impl Work for u32 {
    async fn run(&mut self, ok: bool) -> Result<u32, ()> {
        *self += 1;
        let value = std::future::ready(if ok { Ok(4) } else { Err(()) }).await?;
        Ok(value + 1)
    }
}

fn main() {
    let mut runs = 0_u32;
    for (input, expected) in [(false, Err(())), (true, Ok(5))] {
        let mut future = pin!(runs.run(input));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(expected)
        );
    }
    assert_eq!(runs, 2);
}
