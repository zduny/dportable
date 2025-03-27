//! Portable interface for various utilities.
//! 
//! Intended targets are:
//!  - native platforms with [tokio] async runtime,
//!  - WebAssembly targeted to browsers, including WebWorkers, 
//!    under standard single-threaded model.
//! 
//! Following features are provided:
//!  - [Mutex] and [RwLock] (using [parking_lot] on native platforms) 
//!    and [std::cell::RefCell] in WASM.
//!  - asynchronous [spawn] (not requiring [Send] in WASM) and [sleep](time::sleep),
//!  - [Timeout](time::Timeout) future,
//!  - [dtest](test::dtest) attribute macro to create tests for both 
//!    native and WASM targets, also [dtest_configure](test::dtest_configure) 
//!    macro to configure tests to run in browser.

pub mod test;

pub mod time;

mod lock;
pub use lock::*;

pub mod value;

#[cfg(not(target_arch = "wasm32"))]
pub use tokio::{
    spawn,
    task::{JoinError, JoinHandle},
};

#[cfg(target_arch = "wasm32")]
pub use js_utils::spawn::*;

#[cfg(test)]
mod tests {
    use crate::test::dtest;

    use super::spawn;

    #[dtest]
    async fn test_spawn() {
        let result = spawn(async move { 4 });
        assert_eq!(4, result.await.unwrap());
    }
}
