#!/usr/bin/env bash


cargo run
wasm-pack build
cd lox-rs-app
# open browser
npm run start

