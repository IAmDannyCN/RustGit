#!/bin/bash
clear
rm -rf ./test_area/{*,.mygit} 2>/dev/null

# Ask user for test name
read -p "Enter test name to run : " test_name

# Compile (suppressing warnings but keeping errors) and run with the specified test
rustc -A warnings main.rs && ./main "$test_name"

# Clean up
rm -rf ./test_area/{*,.mygit} 2>/dev/null