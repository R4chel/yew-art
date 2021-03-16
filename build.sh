#!/usr/bin/env bash
cd $(dirname $0)
set -ex
outdir=web-gen
project=yew_art
mkdir -p $outdir
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/$project.wasm --out-dir $outdir
cp static/* $outdir/
