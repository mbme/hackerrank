#!/usr/bin/env bash

set -e # fail fast

function compare {
    diff <(sed -e '$a\' "$1") <(sed -e '$a\' "$2") \
        && echo "$2: OK" || echo "$2: NOT OK"
}

# 1) compile solution
rustc src/main.rs

# 2) run with input
rm -f result.txt # ignore error if file is missing
./main < tmp/input/input00.txt > result.txt

# 3) compare outputs
compare result.txt tmp/output/output00.txt
