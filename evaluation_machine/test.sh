#!/bin/bash
rm -rf ./test_area/.mygit
rustc main.rs
./main
rm -rf ./main