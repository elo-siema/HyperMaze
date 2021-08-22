#!/bin/bash
#export EMMAKEN_CFLAGS="-s USE_SDL=2"
export EMMAKEN_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0" 
set -ex

cargo build --target wasm32-unknown-emscripten --verbose
rm -rf static
mkdir static

cp target/wasm32-unknown-emscripten/debug/hypercasterv2.wasm -t static
cp target/wasm32-unknown-emscripten/debug/hypercasterv2.wasm.map -t static
cp target/wasm32-unknown-emscripten/debug/hypercasterv2.js -t static
cp html/index.html -t static
cp js/mq-js-bundle.js -t static
cp js/audio.js -t static