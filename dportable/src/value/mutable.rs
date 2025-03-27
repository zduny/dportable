//! Async value that can be reset to empty state.

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures::{
    channel::oneshot::{channel, Receiver, Sender},
    future::Shared,
    ready, FutureExt,
};

use crate::{Mutex, RwLock};

use super::AlreadySet;

/// Async value that can be reset to empty state.
#[derive(Debug)]
pub struct AsyncValue<T> {
    value: Arc<RwLock<Option<T>>>,
    sender: Arc<Mutex<Option<Sender<T>>>>,
    receiver: Arc<Mutex<Shared<Receiver<T>>>>,
}

impl<T> Clone for AsyncValue<T> {
    fn clone(&self) -> Self {
        let receiver = Arc::new(Mutex::new(self.receiver.lock().clone()));
        Self {
            value: self.value.clone(),
            sender: self.sender.clone(),
            receiver,
        }
    }
}

impl<T> AsyncValue<T>
where
    T: Clone,
{
    /// Create new mutable async value.
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let value = Arc::new(RwLock::new(None));
        let sender = Arc::new(Mutex::new(Some(sender)));
        let receiver = Arc::new(Mutex::new(receiver.shared()));
        AsyncValue {
            value,
            sender,
            receiver,
        }
    }

    /// Set value.
    pub fn set(&self, new_value: T) -> Result<(), AlreadySet> {
        let mut value = self.value.write();
        let mut sender = self.sender.lock();
        if value.is_some() {
            Err(AlreadySet {})
        } else {
            *value = Some(new_value.clone());
            let _ = sender.take().unwrap().send(new_value);
            Ok(())
        }
    }

    /// Take value out.
    ///
    /// It will reset this async value to empty state.
    pub fn take(&self) -> Option<T> {
        let mut value = self.value.write();
        let mut sender = self.sender.lock();
        let mut receiver = self.receiver.lock();
        let result = value.take();
        if result.is_some() {
            let (new_sender, new_receiver) = channel();
            let new_receiver = new_receiver.shared();
            *sender = Some(new_sender);
            *receiver = new_receiver;
        }
        result
    }

    /// Return value or [None] if not set.
    pub fn try_get(&self) -> Option<T> {
        self.value.read().clone()
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

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &*self.value.read() {
            Some(value) => Poll::Ready(value.clone()),
            None => match ready!(self.receiver.lock().poll_unpin(cx)) {
                Ok(value) => Poll::Ready(value),
                Err(_) => Poll::Pending,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spawn;
    use crate::test::{dtest, dtest_configure};
    use crate::value::AlreadySet;

    use super::AsyncValue;

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
    async fn test_take() {
        let value = AsyncValue::new();
        assert_eq!(value.take(), None);
        value.set(3).unwrap();
        assert_eq!(value.take().unwrap(), 3);
        assert_eq!(value.take(), None);
        value.set(5).unwrap();
        assert_eq!(value.await, 5);
    }
}
