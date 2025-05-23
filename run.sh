#!/bin/bash

if [ -z "$RUST_HOME" ]; then
    echo "Error: RUST_HOME is not set."
    exit 1
fi
cargo build --release || exit 1
if [ ! -d "$RUST_HOME/test" ]; then
    mkdir -p "$RUST_HOME/test"
fi
rm -rf "$RUST_HOME/test/.mygit" 2>/dev/null
#rm -rf "$RUST_HOME/test/.git" 2>/dev/null
# cp "$RUST_HOME/target/debug/rust-git" "$RUST_HOME/test/"
cp "$RUST_HOME/target/release/rust-git" "$RUST_HOME/test/"