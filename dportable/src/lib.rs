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
