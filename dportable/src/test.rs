/// Configure wasm tests to run in browser.
pub use dportable_macros::dtest_configure;

/// Convert test into two tests for wasm (using `wasm_bindgen_test`
/// and non-wasm targets (using `tokio::test`).
pub use dportable_macros::dtest;
