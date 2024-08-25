#!/bin/bash

mkdir -p testing/bin

gcc -o ./testing/bin/main ./testing/testing.c -lSDL2

if [[ -z $1 ]]; then
    ./testing/bin/main
fi
