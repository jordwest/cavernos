#!/bin/bash

# The dist folder will hold all the files needed to serve the demo
mkdir -p dist

# Build the JavaScript host
(cd runtimes/web && ./build.sh)
cp runtimes/web/dist/main.js ./dist/
cp runtimes/web/src/index.html ./dist/

# Copy the demo assets 
cp example-rs/assets/palette.png ./dist/
cp example-rs/assets/manifest.json ./dist/
cp example-rs/assets/text-font.png ./dist/
cp example-rs/assets/text-font-square.png ./dist/

# Build the demo WebAssembly module and copy it to dist
(cd example-rs && ./build.sh)
cp example-rs/target/wasm32-unknown-unknown/release/cavernos_example_rs_optimised.wasm ./dist/
