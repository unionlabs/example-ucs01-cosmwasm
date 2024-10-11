set shell := ["fish", "-c"]
set dotenv-load := true
set positional-arguments := true

alias b := build

default: build

setup:
    rustup target add wasm32-unknown-unknown

export RUSTFLAGS := '-C target-cpu=mvp -C opt-level=z'

build:
    RUSTFLAGS=$RUSTFLAGS cargo build \
      --target wasm32-unknown-unknown \
      --no-default-features \
      --lib \
      --release \
      -Z build-std=std,panic_abort \
      -Z build-std-features=panic_immediate_abort

fmt:
    cargo fmt
    just --fmt --unstable

[group('cleanup')]
rm-dots:
    sudo find . -type f -name "._*" -exec rm -r {} +
