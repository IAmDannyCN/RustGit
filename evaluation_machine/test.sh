#!/bin/bash

g++ code/pre_test.cpp -o pre_test
g++ code/test1.cpp -o test1
g++ code/test2.cpp -o test2
g++ code/test3.cpp -o test3
g++ code/test4.cpp -o test4

echo -e "\n\n\nRunning pre_test..."
./pre_test
echo -e "Running test1...\n"
./test1
echo -e "\n\n\nRunning pre_test..."
./pre_test
echo -e "Running test2...\n"
./test2
echo -e "\n\n\nRunning pre_test..."
./pre_test
echo -e "Running test3...\n"
./test3
echo -e "\n\n\nRunning pre_test..."
./pre_test
echo -e "Running test4...\n"
./test4

echo -e "\n\n\nRunning pre_test..."
./pre_test
rm -rf pre_test test1 test2 test3 test4
echo -e "\n\nAll tests completed."