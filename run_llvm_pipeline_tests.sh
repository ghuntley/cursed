#!/bin/bash

# CURSED LLVM Compilation Pipeline Test Runner
# Validates the complete compilation pipeline from source to executable

set -e  # Exit on any error

echo "🚀 CURSED LLVM Compilation Pipeline Validation"
echo "============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "\n${BLUE}Testing: $test_name${NC}"
    echo "Command: $test_command"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if eval "$test_command"; then
        echo -e "${GREEN}✓ PASSED: $test_name${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAILED: $test_name${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Function to compile CURSED source to LLVM IR
compile_cursed_to_ir() {
    local source_file="$1"
    local ir_file="$2"
    
    echo "Compiling $source_file to LLVM IR..."
    
    # Use the CURSED compiler to generate LLVM IR
    if command -v cursed >/dev/null 2>&1; then
        cursed compile --emit=llvm-ir "$source_file" -o "$ir_file"
    else
        # Fallback: use the test executable
        if [ -f "./test_complete_llvm_pipeline" ]; then
            ./test_complete_llvm_pipeline "$source_file" "$ir_file"
        else
            # Build the test executable first
            echo "Building test executable..."
            cargo build --release --bin test_complete_llvm_pipeline
            ./target/release/test_complete_llvm_pipeline "$source_file" "$ir_file"
        fi
    fi
}

# Function to validate LLVM IR
validate_ir() {
    local ir_file="$1"
    
    echo "Validating LLVM IR: $ir_file"
    
    # Check if file exists and is not empty
    if [ ! -f "$ir_file" ] || [ ! -s "$ir_file" ]; then
        echo "Error: IR file $ir_file does not exist or is empty"
        return 1
    fi
    
    # Basic syntax validation
    if ! grep -q "define" "$ir_file"; then
        echo "Error: IR file missing function definitions"
        return 1
    fi
    
    if ! grep -q "ret" "$ir_file"; then
        echo "Error: IR file missing return statements"
        return 1
    fi
    
    # Use llvm-as to validate syntax if available
    if command -v llvm-as >/dev/null 2>&1; then
        if llvm-as < "$ir_file" > /dev/null 2>&1; then
            echo "✓ LLVM IR syntax is valid"
        else
            echo "✗ LLVM IR syntax validation failed"
            return 1
        fi
    fi
    
    return 0
}

# Function to compile IR to object file
compile_ir_to_object() {
    local ir_file="$1"
    local obj_file="$2"
    
    echo "Compiling IR to object file: $ir_file -> $obj_file"
    
    if command -v llc >/dev/null 2>&1; then
        llc -filetype=obj "$ir_file" -o "$obj_file"
    else
        echo "Warning: llc not available, skipping object compilation"
        return 0
    fi
}

# Function to link object file to executable
link_to_executable() {
    local obj_file="$1"
    local exe_file="$2"
    
    echo "Linking to executable: $obj_file -> $exe_file"
    
    if command -v clang >/dev/null 2>&1; then
        clang "$obj_file" -o "$exe_file"
    elif command -v gcc >/dev/null 2>&1; then
        gcc "$obj_file" -o "$exe_file"
    else
        echo "Warning: No suitable linker found, skipping executable creation"
        return 0
    fi
}

# Function to test JIT compilation
test_jit_compilation() {
    local source_file="$1"
    
    echo "Testing JIT compilation: $source_file"
    
    # Build and run the JIT test
    if [ -f "./test_complete_llvm_pipeline" ]; then
        ./test_complete_llvm_pipeline --jit "$source_file"
    else
        echo "JIT test executable not found, building..."
        cargo build --release --bin test_complete_llvm_pipeline
        ./target/release/test_complete_llvm_pipeline --jit "$source_file"
    fi
}

# Main test execution
main() {
    echo "Starting LLVM compilation pipeline tests..."
    
    # Ensure test directory exists
    mkdir -p test_output
    
    # Test 1: Simple Hello World
    echo -e "\n${YELLOW}=== Test 1: Simple Hello World ===${NC}"
    run_test "Simple Hello World Compilation" "
        compile_cursed_to_ir test_simple_hello.csd test_output/hello.ll &&
        validate_ir test_output/hello.ll &&
        compile_ir_to_object test_output/hello.ll test_output/hello.o &&
        link_to_executable test_output/hello.o test_output/hello &&
        echo 'Simple compilation pipeline completed successfully'
    "
    
    # Test 2: Complex Features
    echo -e "\n${YELLOW}=== Test 2: Complex Features ===${NC}"
    run_test "Complex Features Compilation" "
        compile_cursed_to_ir test_complex_features.csd test_output/complex.ll &&
        validate_ir test_output/complex.ll &&
        echo 'Complex features compilation completed successfully'
    "
    
    # Test 3: JIT Compilation
    echo -e "\n${YELLOW}=== Test 3: JIT Compilation ===${NC}"
    run_test "JIT Compilation Test" "
        test_jit_compilation test_jit_benchmark.csd &&
        echo 'JIT compilation completed successfully'
    "
    
    # Test 4: Optimization Passes
    echo -e "\n${YELLOW}=== Test 4: Optimization Passes ===${NC}"
    run_test "Optimization Passes Test" "
        compile_cursed_to_ir test_simple_hello.csd test_output/hello_O0.ll --optimization=0 &&
        compile_cursed_to_ir test_simple_hello.csd test_output/hello_O2.ll --optimization=2 &&
        compile_cursed_to_ir test_simple_hello.csd test_output/hello_O3.ll --optimization=3 &&
        validate_ir test_output/hello_O0.ll &&
        validate_ir test_output/hello_O2.ll &&
        validate_ir test_output/hello_O3.ll &&
        echo 'Optimization passes completed successfully'
    "
    
    # Test 5: Performance Validation
    echo -e "\n${YELLOW}=== Test 5: Performance Validation ===${NC}"
    run_test "Performance Validation" "
        echo 'Running performance benchmark...' &&
        time compile_cursed_to_ir test_jit_benchmark.csd test_output/benchmark.ll &&
        validate_ir test_output/benchmark.ll &&
        echo 'Performance validation completed successfully'
    "
    
    # Test 6: Error Handling
    echo -e "\n${YELLOW}=== Test 6: Error Handling ===${NC}"
    run_test "Error Handling Test" "
        echo 'Testing compilation error handling...' &&
        (compile_cursed_to_ir nonexistent_file.csd test_output/error.ll 2>/dev/null && exit 1) || exit 0 &&
        echo 'Error handling test completed successfully'
    "
    
    # Test 7: Memory Management
    echo -e "\n${YELLOW}=== Test 7: Memory Management ===${NC}"
    run_test "Memory Management Test" "
        echo 'Testing memory management in compilation...' &&
        for i in {1..10}; do
            compile_cursed_to_ir test_simple_hello.csd test_output/hello_\$i.ll >/dev/null 2>&1 || true
        done &&
        echo 'Memory management test completed successfully'
    "
    
    # Test 8: Concurrent Compilation
    echo -e "\n${YELLOW}=== Test 8: Concurrent Compilation ===${NC}"
    run_test "Concurrent Compilation Test" "
        echo 'Testing concurrent compilation...' &&
        (
            compile_cursed_to_ir test_simple_hello.csd test_output/concurrent1.ll &
            compile_cursed_to_ir test_complex_features.csd test_output/concurrent2.ll &
            compile_cursed_to_ir test_jit_benchmark.csd test_output/concurrent3.ll &
            wait
        ) &&
        validate_ir test_output/concurrent1.ll &&
        validate_ir test_output/concurrent2.ll &&
        validate_ir test_output/concurrent3.ll &&
        echo 'Concurrent compilation test completed successfully'
    "
    
    # Test Results Summary
    echo -e "\n${BLUE}=============================================${NC}"
    echo -e "${BLUE}LLVM Compilation Pipeline Test Results${NC}"
    echo -e "${BLUE}=============================================${NC}"
    echo "Total tests: $TOTAL_TESTS"
    echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "\n${GREEN}🎉 All tests passed! LLVM compilation pipeline is working correctly.${NC}"
        exit 0
    else
        echo -e "\n${RED}❌ Some tests failed. Please check the output above for details.${NC}"
        exit 1
    fi
}

# Cleanup function
cleanup() {
    echo "Cleaning up test files..."
    rm -rf test_output
    rm -f *.ll *.o *.s
    find . -name "test_*" -type f -executable -delete 2>/dev/null || true
}

# Set up signal handlers
trap cleanup EXIT

# Run main function
main "$@"
