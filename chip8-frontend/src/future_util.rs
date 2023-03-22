use futures_task::noop_waker_ref;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct NoWakeFuture<T> {
    inner: Pin<Box<dyn Future<Output = T> + Send>>,
}

impl<T> NoWakeFuture<T> {
    pub fn new(inner: Pin<Box<dyn Future<Output = T> + Send>>) -> Self {
        Self { inner }
    }
}

impl<T> Future for NoWakeFuture<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut futures_task::Context<'_>) -> Poll<Self::Output> {
        let mut cx = Context::from_waker(noop_waker_ref());

        self.inner.as_mut().poll(&mut cx)
    }
}
