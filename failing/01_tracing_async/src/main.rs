use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

async fn db_get(id: u64) -> Result<String, &'static str> {
    if id == 0 {
        Err("missing")
    } else {
        Ok(String::from("row"))
    }
}

#[tracing::instrument]
async fn fetch(id: u64) -> Result<String, &'static str> {
    let row = db_get(id).await?;
    Ok(row)
}

fn main() {
    for (id, expected) in [(0, Err("missing")), (1, Ok(String::from("row")))] {
        let mut future = pin!(fetch(id));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(expected));
    }
}
