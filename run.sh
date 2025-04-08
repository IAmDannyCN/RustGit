#!/bin/bash

if [ -z "$RUST_HOME" ]; then
    echo "Error: RUST_HOME is not set."
    exit 1
fi
cargo build || exit 1
if [ ! -d "$RUST_HOME/test" ]; then
    mkdir -p "$RUST_HOME/test"
fi
rm -rf "$RUST_HOME/test/"{.,}* 2>/dev/null
cp "$RUST_HOME/target/debug/git" "$RUST_HOME/test/"