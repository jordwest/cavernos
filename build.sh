#!/bin/bash

#### Build the webassembly module

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/wasm_minimal.wasm
#BINARY=target/$TARGET/debug/wasm_minimal.wasm

cargo build --target $TARGET --release
#cargo build --target $TARGET
wasm-strip $BINARY
mkdir -p www
wasm-opt -o www/wasm_minimal.wasm -Oz $BINARY
ls -lh www/wasm_minimal.wasm


