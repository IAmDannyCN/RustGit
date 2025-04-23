#!/bin/bash

clear
echo -e "compiling test files...\n"
g++ code/pre_test.cpp -o pre_test
for ((i=1; i<=4; i++)); do
    g++ code/test$i.cpp -o test$i
    echo -e "compiled test $i\n"
done
echo -e "starting tests, enter to continue...\n"
read -r
clear

for ((i=1; i<=4; i++)); do
    echo -e "\n\n\nRunning pre_test..."
    ./pre_test
    echo -e "Running test $i...\n"
    ./test$i
    echo -e "enter to continue...\n"
    read -r
    rm -rf test$i
    clear
done

echo -e "\n\n\nRunning pre_test..."
./pre_test
rm -rf pre_test
echo -e "\n\nAll tests completed."
echo -e "enter to continue...\n"
read -r
clear