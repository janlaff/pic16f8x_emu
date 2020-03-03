#!/bin/bash

rustup run nightly wasm-pack build --target web
rollup main.js --file bundle.js --format iife
