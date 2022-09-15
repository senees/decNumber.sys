#!/usr/bin/env bash

WORKING_DIRECTORY=$(pwd)
DMNTK_BINARY_PATH="$WORKING_DIRECTORY"/target/debug

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

# run all tests
cargo test

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage
# generate coverage info
grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*chrono-tz*" --ignore "*tests*" -o ./target/lcov/lcov.info
# generate coverage report
genhtml -t "dec-number-sys" -q -o ./target/coverage ./target/lcov/lcov.info
# display final message
echo ""
echo "open coverage report: file://$WORKING_DIRECTORY/target/coverage/index.html"
echo ""