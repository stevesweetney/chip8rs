use futures_task::noop_waker_ref;
use std::future::Future;
use std::marker::Send;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::{Future, Pin, Send};
    pub struct NoWakeFuture<T> {
        pub(super) inner: Pin<Box<dyn Future<Output = T>>>,
    }

    impl<T> NoWakeFuture<T> {
        pub fn new(inner: Pin<Box<dyn Future<Output = T>>>) -> Self {
            Self { inner }
        }
    }

    unsafe impl<T> Send for NoWakeFuture<T> {}
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::{Future, Pin, Send};
    pub struct NoWakeFuture<T> {
        pub(super) inner: Pin<Box<dyn Future<Output = T> + Send>>,
    }

    impl<T> NoWakeFuture<T> {
        pub fn new(inner: Pin<Box<dyn Future<Output = T> + Send>>) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::NoWakeFuture;

#[cfg(target_arch = "wasm32")]
pub use wasm::NoWakeFuture;

impl<T> Future for NoWakeFuture<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut futures_task::Context<'_>) -> Poll<Self::Output> {
        let mut cx = Context::from_waker(noop_waker_ref());

        self.inner.as_mut().poll(&mut cx)
    }
}
