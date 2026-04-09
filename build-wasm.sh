#!/usr/bin/env bash
set -euo pipefail

wasm-pack build crates/mock-wasm --target web

# wasm-pack's built-in wasm-opt invocation fails because Rust emits
# bulk-memory instructions (memory.copy) by default for wasm32-unknown-unknown,
# but wasm-opt defaults to the MVP feature set. Run wasm-opt manually with
# --enable-bulk-memory to work around this.
wasm-opt crates/mock-wasm/pkg/mock_wasm_bg.wasm \
  -O --enable-bulk-memory \
  -o crates/mock-wasm/pkg/mock_wasm_bg.wasm
