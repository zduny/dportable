//! Async value that can be set only once.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{future::FusedFuture, ready, FutureExt};

use super::{mutable, AlreadySet};

/// Async value that can be set only once.
#[derive(Debug, Clone)]
pub struct AsyncValue<T> {
    inner: mutable::AsyncValue<T>,
    terminated: bool,
}

impl<T> AsyncValue<T>
where
    T: Clone,
{
    /// Create new async value.
    pub fn new() -> Self {
        AsyncValue {
            inner: mutable::AsyncValue::new(),
            terminated: false,
        }
    }

    /// Set value.
    pub fn set(&self, value: T) -> Result<(), AlreadySet> {
        self.inner.set(value)
    }

    /// Return value or [None] if not yet set.
    pub fn try_get(&self) -> Option<T> {
        self.inner.try_get()
    }
}

impl<T> Default for AsyncValue<T>
where
    T: Clone,
{
    fn default() -> Self {
        AsyncValue::new()
    }
}

impl<T> Future for AsyncValue<T>
where
    T: Clone,
{
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = ready!(self.inner.poll_unpin(cx));
        self.terminated = true;
        Poll::Ready(result)
    }
}

impl<T> FusedFuture for AsyncValue<T>
where
    T: Clone,
{
    fn is_terminated(&self) -> bool {
        self.terminated
    }
}

/// Asynchronous notifier.
#[derive(Debug, Clone, Default)]
pub struct Notifier(AsyncValue<()>);

impl Notifier {
    /// Create new async notifier.
    pub fn new() -> Self {
        Notifier(AsyncValue::new())
    }

    /// Returns [true] if already notified.
    pub fn already_notified(&self) -> bool {
        self.0.try_get().is_some()
    }

    /// Notify others (and myself).
    pub fn notify(&self) {
        let _ = self.0.set(());
    }
}

impl Future for Notifier {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.poll_unpin(cx)
    }
}

impl FusedFuture for Notifier {
    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::test::{dtest, dtest_configure};
    use crate::value::AlreadySet;
    use crate::{spawn, time::sleep};

    use super::{AsyncValue, Notifier};

    dtest_configure!();

    #[dtest]
    async fn test_async_value() {
        let value = AsyncValue::new();
        let value_clone = value.clone();
        let join_handle = spawn(async move { value_clone.await });
        assert_eq!(value.try_get(), None);
        value.set(5).unwrap();
        assert_eq!(value.set(1), Err(AlreadySet {}));
        assert_eq!(value.await, 5);
        assert_eq!(join_handle.await.unwrap(), 5);
    }

    #[dtest]
    async fn test_notifier() {
        let notifier = Notifier::new();
        let notifier_clone = notifier.clone();
        let join_handle = spawn(async move { notifier_clone.await });
        let notifier_clone = notifier.clone();
        spawn(async move {
            sleep(Duration::from_millis(10)).await;
            notifier_clone.notify();
        });
        notifier.await;
        join_handle.await.unwrap();
    }
}
