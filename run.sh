#!/bin/bash

RUST_HOME=~/rust
cargo build || exit 1
rm -rf "$RUST_HOME/test/"{.,}* 2>/dev/null
cp "$RUST_HOME/target/debug/git" "$RUST_HOME/test/"