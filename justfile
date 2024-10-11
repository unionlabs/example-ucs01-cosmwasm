set shell := ["fish", "-c"]
set dotenv-load := true
set positional-arguments := true

alias b := build

default: build

setup:
    rustup target add wasm32-unknown-unknown

build:
    cargo build --target wasm32-unknown-unknown --release

[group('cleanup')]
rm-dots:
    sudo find . -type f -name "._*" -exec rm -r {} +
