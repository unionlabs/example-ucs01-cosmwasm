set shell := ["fish", "-c"]
set dotenv-load := true
set positional-arguments := true

alias b := build

default: build

build:
    echo "TODO..."

[group('cleanup')]
rm-dots:
    sudo find . -type f -name "._*" -exec rm -r {} +
