//! Mutex and RwLock.

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(test)]
mod tests {
    use super::{Mutex, RwLock};
    use crate::test::{dtest, dtest_configure};

    dtest_configure!();

    #[dtest]
    async fn test_mutex() {
        let a = Mutex::new(2);
        println!("{}", a.lock());
        assert_eq!(2, *a.lock());
        *a.lock() = 4;
        assert_eq!(4, *a.lock());
    }

    #[dtest]
    async fn test_rw_lock() {
        let a = RwLock::new(2);
        println!("{}", a.read());
        assert_eq!(2, *a.read());
        *a.write() = 4;
        assert_eq!(4, *a.read());
    }
}
