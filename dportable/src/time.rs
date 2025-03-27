//! Utilities for tracking time.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::{future::FusedFuture, ready, FutureExt};
#[cfg(not(target_arch = "wasm32"))]
pub use tokio::time::{sleep, sleep_until, Instant, Sleep};

#[cfg(target_arch = "wasm32")]
pub use js_utils::sleep::*;

/// Timeout future.
#[derive(Debug)]
pub enum Timeout {
    /// Timeout with duration.
    Duration {
        /// Timeout duration.
        duration: Duration,

        /// Sleep future.
        sleep: Pin<Box<Sleep>>,
    },

    /// Timeout that never occurs.
    Never,
}

impl Timeout {
    /// Create new timeout with specified duration.
    pub fn new(duration: Duration) -> Self {
        Timeout::Duration {
            duration,
            sleep: Box::pin(sleep(duration)),
        }
    }

    /// Create new timeout that never occurs.
    pub fn never() -> Self {
        Timeout::Never
    }

    /// Reset timeout (with duration it was created with).
    pub fn reset(&mut self) {
        if let Timeout::Duration { duration, sleep } = self {
            let deadline = Instant::now() + *duration;
            sleep.as_mut().reset(deadline)
        }
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut *self {
            Timeout::Duration { sleep, .. } => {
                ready!(sleep.poll_unpin(cx));
                Poll::Ready(())
            }
            Timeout::Never => Poll::Pending,
        }
    }
}

impl FusedFuture for Timeout {
    fn is_terminated(&self) -> bool {
        match self {
            Timeout::Duration { .. } => false,
            Timeout::Never => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{pin::pin, time::Duration};

    use futures::future::{select, Either};

    use crate::{spawn, test::dtest};

    use super::{sleep, sleep_until, Instant, Timeout};

    #[dtest]
    async fn test_sleep() {
        sleep(Duration::from_millis(10)).await;
    }

    #[dtest]
    async fn test_sleep_until() {
        sleep_until(Instant::now() + Duration::from_millis(10)).await;
    }

    #[dtest]
    async fn test_timeout() {
        let mut timeout = Timeout::new(Duration::from_millis(10));
        let task = spawn(async move { 2 });
        let result = select(&mut timeout, task).await;
        match result {
            Either::Right((Ok(result), _)) => assert_eq!(result, 2),
            _ => panic!("wrong result"),
        }
        timeout.reset();

        let sleep = sleep(Duration::from_millis(50));
        let sleep = pin!(sleep);
        let result = select(timeout, sleep).await;
        match result {
            Either::Left(_) => (),
            _ => panic!("wrong result"),
        }
    }
}
