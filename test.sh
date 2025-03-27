#!/bin/sh
set -e

# native tests
cargo test

# wasm tests
cd dportable
wasm-pack test --chrome --headless
cd ..
