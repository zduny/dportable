# dportable

Portable interface for various utilities.

Intended targets are:
 - native platforms with [tokio](https://docs.rs/tokio/latest/tokio/) async runtime,
 - WebAssembly targeted to browsers, including WebWorkers,
   under standard single-threaded model.

Following features are provided:
 - `Mutex` and `RwLock` (using [parking_lot](https://docs.rs/parking_lot/latest/parking_lot/) on native platforms)
    and `std::cell::RefCell` in WASM.  
 - asynchronous `spawn` (not requiring `Send` in WASM) and `sleep`,
 - `Timeout` future,
 - `dtest` attribute macro to create tests for both
    native and WASM targets, also `dtest_configure`
    macro to configure tests to run in browser.
 - `create_non_sync_send_variant_for_wasm` utility macro for creating
    non-`Send` and non-`Sync` variants of traits for use in WASM.
 - `random` function.
