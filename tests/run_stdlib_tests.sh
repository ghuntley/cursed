#!/bin/bash

# This script runs all the standard library tests

echo "Running standard library tests..."

# Build the project first
cargo build || { echo "Build failed!"; exit 1; }

# Create array of test files
test_files=(
    "tests/stdlib_basic_test.csd"
    "tests/stringz_test.csd"
    "tests/mathz_test.csd"
    "tests/timez_test.csd"
    "tests/vibe_life_test.csd"
    "tests/dropz_test.csd"
    "tests/concurrenz_test.csd"
    "tests/web_vibez_test.csd"
    "tests/json_tea_test.csd"
    "tests/regex_vibez_test.csd"
    "tests/cryptz_test.csd"
    "tests/reflectz_test.csd"
)

# Run each test file
for test_file in "${test_files[@]}"; do
    echo "\nRunning test: $test_file"
    ./target/debug/cursed "$test_file"
    
    if [ $? -eq 0 ]; then
        echo "✅ Test passed: $test_file"
    else
        echo "❌ Test failed: $test_file"
        exit 1
    fi
done

echo "\n✅ All standard library tests passed!"