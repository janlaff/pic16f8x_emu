#!/bin/bash

rustup run nightly wasm-pack build --target web
cp pkg/pic16f8x_emu.js live/
cp pkg/pic16f8x_emu_bg.wasm live/
rollup live/main.js --file live/bundle.js --format iife
