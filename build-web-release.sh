#!/bin/bash
export EMMAKEN_CFLAGS="-s USE_SDL=2"
export EMMAKEN_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0" 
cargo build --release --target wasm32-unknown-emscripten
cp target/wasm32-unknown-emscripten/release/hyperbolic-raycaster.js -t html
cp target/wasm32-unknown-emscripten/release/hyperbolic_raycaster.wasm -t html