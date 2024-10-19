#!/bin/bash
set -x

cargo component build --target wasm32-wasip2 --release --workspace
wac compose -d loremaster:cultures=target/wasm32-wasip2/release/cultures.wasm -d loremaster:characters=target/wasm32-wasip2/release/characters.wasm -d loremaster:handler=target/wasm32-wasip2/release/loremaster.wasm ./wit/composition.wac -o target/wasm32-wasip2/release/handler.wasm
