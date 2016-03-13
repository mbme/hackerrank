#!/usr/bin/env bash

set -e # fail fast

if [ "$1" == "unit" ]; then
    echo "running unit tests..."
    rustc --test src/main.rs && ./main
    exit 0
fi


function compare {
    diff <(sed -e '$a\' "$1") <(sed -e '$a\' "$2") \
        && echo "$2: OK" || echo "$2: NOT OK"
}

# 1) compile solution
rustc src/main.rs

for INPUT_FILE in tmp/input/input*.txt; do
    echo ""
    echo "Testing with $INPUT_FILE"
    rm -f result.txt # ignore error if file is missing

    # 2) run with input from the file
    ./main < "$INPUT_FILE" > result.txt

    name=${INPUT_FILE##*/} # input00.txt
    id=${name#input} # 00.txt

    # 3) compare outputs
    compare result.txt "tmp/output/output$id"
done
