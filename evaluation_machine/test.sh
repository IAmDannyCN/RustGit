#!/bin/bash
clear
rm -rf ./test_area/{*,.mygit} 2>/dev/null
rustc main.rs && ./main
# rm -rf ./main ./test_area/{*,.mygit} 2>/dev/null