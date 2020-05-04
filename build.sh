#!/bin/bash

#### Build the webassembly module

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/wasm_minimal.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p www
wasm-opt -o www/wasm_minimal.wasm -Oz $BINARY
ls -lh www/wasm_minimal.wasm


