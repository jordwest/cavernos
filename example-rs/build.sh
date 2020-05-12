#!/bin/bash

#### Build the webassembly module

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY_IN=target/$TARGET/release/cavernos_example_rs.wasm
BINARY_OUT=target/$TARGET/release/cavernos_example_rs_optimised.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY_IN
mkdir -p www
wasm-opt -o $BINARY_OUT -Oz $BINARY_IN

