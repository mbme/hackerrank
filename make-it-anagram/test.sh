#!/usr/bin/env bash

set -e # fail fast

export RUST_BACKTRACE=1 # show backtraces

if [ "$1" == "unit" ]; then
    echo "running unit tests..."
    rustc --test src/main.rs && ./main --nocapture
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

    if [ "$1" == "debug" ]; then
        ./main < "$INPUT_FILE"
    else
        # 2) run with input from the file
        ./main < "$INPUT_FILE" > result.txt

        name=${INPUT_FILE##*/} # input00.txt
        id=${name#input} # 00.txt

        # 3) compare outputs
        compare result.txt "tmp/output/output$id"
    fi

done
