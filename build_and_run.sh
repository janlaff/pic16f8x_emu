#!/bin/bash

[ ! -f "static/pic16f8x_emu.js" ] && ln -s "target/wasm32-unknown-unknown/pic16f8x_emu.js" "static/"
[ ! -f "static/pic16f8x_emu.wasm" ] && ln -s "target/wasm32-unknown-unknown/pic16f8x_emu.wasm" "static/"

cargo +nightly web start
