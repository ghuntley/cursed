#!/bin/bash

# Debug script for Cursed benchmarks
# Runs each benchmark in benchmarks/cursed/ through the Cursed interpreter
# Reports issues with each benchmark

echo "=== Running Cursed Benchmarks Debug Script ==="
echo

# Get the Cursed executable path
CURSED_PATH="./target/debug/cursed"

# Check if Cursed executable exists
if [ ! -f "$CURSED_PATH" ]; then
    echo "Error: Cursed executable not found at $CURSED_PATH"
    echo "Please build the project first using 'cargo build'"
    exit 1
fi

# Function to run a benchmark
run_benchmark() {
    local benchmark=$1
    local benchmark_name=$(basename "$benchmark" .csd)
    
    echo "Running benchmark: $benchmark_name"
    echo "-----------------------------------"
    
    # Run the benchmark through Cursed and capture output
    OUTPUT=$($CURSED_PATH "$benchmark" 2>&1)
    EXIT_CODE=$?
    
    # Check the exit code
    if [ $EXIT_CODE -eq 0 ]; then
        echo "✅ Benchmark $benchmark_name completed successfully"
    else
        echo "❌ Benchmark $benchmark_name failed with exit code $EXIT_CODE"
        
        # Extract and show parsing errors
        if echo "$OUTPUT" | grep -q "Parser found errors"; then
            echo "Parsing errors detected:"
            echo "$OUTPUT" | grep -A 20 "Parser found errors" | head -n 20
        fi
        
        # Extract and show compilation errors
        if echo "$OUTPUT" | grep -q "Compilation failed"; then
            echo "Compilation errors detected:"
            echo "$OUTPUT" | grep -A 5 "Compilation failed" | head -n 5
        fi
        
        # Prompt to continue
        echo "Continue with next benchmark? (y/n)"
        read -r response
        if [[ "$response" != "y" ]]; then
            exit 1
        fi
    fi
    
    echo
}

# Find all .csd files in benchmarks/cursed directory
BENCHMARKS=$(find benchmarks/cursed -name "*.csd" -type f | sort)

# Check if any benchmarks were found
if [ -z "$BENCHMARKS" ]; then
    echo "No benchmarks found in benchmarks/cursed/"
    exit 1
fi

echo "Found $(echo "$BENCHMARKS" | wc -l) benchmarks to run"
echo

# Run each benchmark
FAILED_BENCHMARKS=()
for benchmark in $BENCHMARKS; do
    run_benchmark "$benchmark"
    if [ $? -ne 0 ]; then
        FAILED_BENCHMARKS+=("$benchmark")
    fi
done

# Report results
echo "=== Benchmark Testing Complete ==="
if [ ${#FAILED_BENCHMARKS[@]} -eq 0 ]; then
    echo "✅ All benchmarks completed successfully!"
else
    echo "❌ ${#FAILED_BENCHMARKS[@]} benchmarks failed:"
    for benchmark in "${FAILED_BENCHMARKS[@]}"; do
        echo "  - $(basename "$benchmark")"
    done
    
fi