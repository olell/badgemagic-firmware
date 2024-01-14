#!/bin/bash

set -e

if [ -d "build/" ]; then
    echo "Found build/, start building."
    cd build
    cmake -GNinja ..
    ninja
else
    echo "Not Found build/, we will create it."
    mkdir build
    cd build
    cmake -GNinja ..
    ninja
fi


# Check if "--flash" flag is present
if [[ "$*" == *"--flash"* ]]; then
    wchisp flash ./debug/ch58x-ninja.bin
fi
