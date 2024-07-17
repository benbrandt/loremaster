#!/bin/bash
set -x

cargo component build --target wasm32-wasip1 --release --workspace
wac encode -d loremaster:cultures=target/wasm32-wasip1/release/cultures.wasm -d loremaster:characters=target/wasm32-wasip1/release/characters.wasm -d loremaster:handler=target/wasm32-wasip1/release/loremaster.wasm ./wit/composition.wac -o target/wasm32-wasip1/release/handler.wasm
