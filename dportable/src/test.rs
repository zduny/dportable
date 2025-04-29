//! Tests-related macros.

/// Configure WASM tests to run in browser.
pub use dportable_macros::dtest_configure;

/// Convert test into two tests for WASM (using
/// [wasm-bindgen-test](https://crates.io/crates/wasm-bindgen-test))
/// and non-WASM targets (using 
/// [tokio::test](https://docs.rs/tokio/latest/tokio/attr.test.html)).
///
/// ```
/// use dportable::test::dtest;
///
/// #[dtest]
/// async fn test_portable() {
///     assert_eq!(2 + 2, 4);
/// }
/// ```
pub use dportable_macros::dtest;
