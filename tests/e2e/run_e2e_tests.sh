#!/bin/bash

# Comprehensive End-to-End Test Runner for CURSED
# Tests the entire compilation pipeline from source to execution

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
CURSED_BIN="${CURSED_BIN:-../../zig-out/bin/cursed-zig}"
CURSED_UNIFIED="${CURSED_UNIFIED:-../../cursed-unified}"
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMP_DIR="/tmp/cursed-e2e-tests-$$"
TIMEOUT=30

# Test categories
CATEGORIES=(
    "basic"
    "control_flow" 
    "data_structures"
    "error_handling"
    "concurrency"
    "stdlib"
    "integration"
)

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Parse command line arguments
INTERPRET_ONLY=false
COMPILE_ONLY=false
CATEGORY_FILTER=""
VERBOSE=false

usage() {
    echo "Usage: $0 [OPTIONS] [CATEGORY]"
    echo ""
    echo "Options:"
    echo "  --interpret-only    Run tests in interpretation mode only"
    echo "  --compile-only      Run tests in compilation mode only"
    echo "  --verbose          Enable verbose output"
    echo "  --help             Show this help message"
    echo ""
    echo "Categories: ${CATEGORIES[*]}"
    echo ""
    echo "Examples:"
    echo "  $0                     # Run all tests"
    echo "  $0 basic              # Run basic tests only"
    echo "  $0 --interpret-only   # Run all tests in interpret mode"
    echo "  $0 --compile-only basic # Run basic tests in compile mode"
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --interpret-only)
            INTERPRET_ONLY=true
            shift
            ;;
        --compile-only)
            COMPILE_ONLY=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help)
            usage
            exit 0
            ;;
        -*)
            echo "Unknown option $1"
            usage
            exit 1
            ;;
        *)
            CATEGORY_FILTER="$1"
            shift
            ;;
    esac
done

# Validate category filter
if [[ -n "$CATEGORY_FILTER" ]]; then
    if [[ ! " ${CATEGORIES[*]} " =~ " ${CATEGORY_FILTER} " ]]; then
        echo -e "${RED}Error: Invalid category '$CATEGORY_FILTER'${NC}"
        echo "Valid categories: ${CATEGORIES[*]}"
        exit 1
    fi
fi

# Create temp directory
mkdir -p "$TEMP_DIR"
trap "rm -rf '$TEMP_DIR'" EXIT

log() {
    local level="$1"
    shift
    local message="$*"
    
    case "$level" in
        "INFO")
            echo -e "${BLUE}[INFO]${NC} $message"
            ;;
        "PASS")
            echo -e "${GREEN}[PASS]${NC} $message"
            ;;
        "FAIL")
            echo -e "${RED}[FAIL]${NC} $message"
            ;;
        "WARN")
            echo -e "${YELLOW}[WARN]${NC} $message"
            ;;
        "SKIP")
            echo -e "${YELLOW}[SKIP]${NC} $message"
            ;;
    esac
}

debug() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${BLUE}[DEBUG]${NC} $*"
    fi
}

check_binary() {
    local binary="$1"
    local name="$2"
    
    if [[ ! -f "$binary" ]]; then
        log "WARN" "$name not found at $binary"
        return 1
    fi
    
    if [[ ! -x "$binary" ]]; then
        log "WARN" "$name is not executable at $binary"
        return 1
    fi
    
    return 0
}

run_test_file() {
    local test_file="$1"
    local mode="$2" # "interpret" or "compile"
    local output_file="$3"
    local error_file="$4"
    
    local binary=""
    local args=""
    
    case "$mode" in
        "interpret")
            if check_binary "$CURSED_BIN" "cursed-zig"; then
                binary="$CURSED_BIN"
                args="$test_file"
            elif check_binary "$CURSED_UNIFIED" "cursed-unified"; then
                binary="$CURSED_UNIFIED"
                args="$test_file"
            else
                return 2 # Skip
            fi
            ;;
        "compile")
            if check_binary "$CURSED_BIN" "cursed-zig"; then
                binary="$CURSED_BIN"
                args="--compile $test_file"
            else
                return 2 # Skip
            fi
            ;;
    esac
    
    debug "Running: timeout $TIMEOUT $binary $args"
    
    if timeout "$TIMEOUT" "$binary" $args >"$output_file" 2>"$error_file"; then
        return 0 # Success
    else
        local exit_code=$?
        if [[ $exit_code -eq 124 ]]; then
            echo "Test timed out after ${TIMEOUT}s" >> "$error_file"
        fi
        return 1 # Failure
    fi
}

test_single_file() {
    local test_file="$1"
    local relative_path="$2"
    
    debug "Testing file: $relative_path"
    
    local base_name=$(basename "$test_file" .csd)
    local category=$(dirname "$relative_path")
    
    # Skip error cases for now (they need special handling)
    if [[ "$test_file" == *"/error_cases/"* ]]; then
        log "SKIP" "$relative_path (error case handling not yet implemented)"
        ((SKIPPED_TESTS++))
        return 0
    fi
    
    local interpret_result=0
    local compile_result=0
    local test_passed=true
    
    # Test interpretation mode
    if [[ "$COMPILE_ONLY" != "true" ]]; then
        local interpret_output="$TEMP_DIR/${base_name}_interpret.out"
        local interpret_error="$TEMP_DIR/${base_name}_interpret.err"
        
        debug "Testing interpretation mode"
        run_test_file "$test_file" "interpret" "$interpret_output" "$interpret_error"
        interpret_result=$?
        
        if [[ $interpret_result -eq 0 ]]; then
            debug "Interpretation mode: SUCCESS"
            if [[ "$VERBOSE" == "true" ]]; then
                echo "--- Interpretation Output ---"
                cat "$interpret_output"
                echo "--- End Interpretation Output ---"
            fi
        elif [[ $interpret_result -eq 2 ]]; then
            log "SKIP" "$relative_path (interpretation) - no suitable binary"
            ((SKIPPED_TESTS++))
            return 0
        else
            debug "Interpretation mode: FAILED"
            test_passed=false
            if [[ "$VERBOSE" == "true" ]]; then
                echo "--- Interpretation Error ---"
                cat "$interpret_error"
                echo "--- End Interpretation Error ---"
            fi
        fi
    fi
    
    # Test compilation mode
    if [[ "$INTERPRET_ONLY" != "true" ]]; then
        local compile_output="$TEMP_DIR/${base_name}_compile.out"
        local compile_error="$TEMP_DIR/${base_name}_compile.err"
        
        debug "Testing compilation mode"
        run_test_file "$test_file" "compile" "$compile_output" "$compile_error"
        compile_result=$?
        
        if [[ $compile_result -eq 0 ]]; then
            debug "Compilation mode: SUCCESS"
            if [[ "$VERBOSE" == "true" ]]; then
                echo "--- Compilation Output ---"
                cat "$compile_output"
                echo "--- End Compilation Output ---"
            fi
        elif [[ $compile_result -eq 2 ]]; then
            log "SKIP" "$relative_path (compilation) - no suitable binary"
            ((SKIPPED_TESTS++))
            return 0
        else
            debug "Compilation mode: FAILED"
            test_passed=false
            if [[ "$VERBOSE" == "true" ]]; then
                echo "--- Compilation Error ---"
                cat "$compile_error"
                echo "--- End Compilation Error ---"
            fi
        fi
    fi
    
    # Report results
    if [[ "$test_passed" == "true" ]]; then
        local modes=""
        if [[ "$COMPILE_ONLY" != "true" && $interpret_result -eq 0 ]]; then
            modes="interpret"
        fi
        if [[ "$INTERPRET_ONLY" != "true" && $compile_result -eq 0 ]]; then
            if [[ -n "$modes" ]]; then
                modes="$modes+compile"
            else
                modes="compile"
            fi
        fi
        log "PASS" "$relative_path ($modes)"
        ((PASSED_TESTS++))
    else
        log "FAIL" "$relative_path"
        ((FAILED_TESTS++))
        
        # Show error details
        if [[ "$COMPILE_ONLY" != "true" && $interpret_result -ne 0 && $interpret_result -ne 2 ]]; then
            echo "  Interpretation errors:"
            sed 's/^/    /' "$TEMP_DIR/${base_name}_interpret.err" || true
        fi
        if [[ "$INTERPRET_ONLY" != "true" && $compile_result -ne 0 && $compile_result -ne 2 ]]; then
            echo "  Compilation errors:"
            sed 's/^/    /' "$TEMP_DIR/${base_name}_compile.err" || true
        fi
    fi
    
    ((TOTAL_TESTS++))
}

run_category_tests() {
    local category="$1"
    local category_dir="$TEST_DIR/$category"
    
    if [[ ! -d "$category_dir" ]]; then
        log "WARN" "Category directory not found: $category_dir"
        return
    fi
    
    log "INFO" "Running tests for category: $category"
    
    # Find all .csd files in the category
    local test_files=()
    while IFS= read -r -d '' file; do
        test_files+=("$file")
    done < <(find "$category_dir" -name "*.csd" -type f -print0 | sort -z)
    
    if [[ ${#test_files[@]} -eq 0 ]]; then
        log "WARN" "No test files found in category: $category"
        return
    fi
    
    for test_file in "${test_files[@]}"; do
        local relative_path="${test_file#$TEST_DIR/}"
        test_single_file "$test_file" "$relative_path"
    done
}

main() {
    echo -e "${BLUE}CURSED End-to-End Test Suite${NC}"
    echo "=================================="
    echo ""
    
    # Show configuration
    log "INFO" "Test directory: $TEST_DIR"
    log "INFO" "Primary binary: $CURSED_BIN"
    log "INFO" "Unified binary: $CURSED_UNIFIED"
    log "INFO" "Temp directory: $TEMP_DIR"
    
    if [[ "$INTERPRET_ONLY" == "true" ]]; then
        log "INFO" "Mode: Interpretation only"
    elif [[ "$COMPILE_ONLY" == "true" ]]; then
        log "INFO" "Mode: Compilation only"
    else
        log "INFO" "Mode: Both interpretation and compilation"
    fi
    
    echo ""
    
    # Run tests
    if [[ -n "$CATEGORY_FILTER" ]]; then
        run_category_tests "$CATEGORY_FILTER"
    else
        for category in "${CATEGORIES[@]}"; do
            run_category_tests "$category"
            echo ""
        done
    fi
    
    # Show summary
    echo "=================================="
    echo -e "${BLUE}Test Summary${NC}"
    echo "=================================="
    echo "Total tests:  $TOTAL_TESTS"
    echo -e "Passed:       ${GREEN}$PASSED_TESTS${NC}"
    echo -e "Failed:       ${RED}$FAILED_TESTS${NC}"
    echo -e "Skipped:      ${YELLOW}$SKIPPED_TESTS${NC}"
    echo ""
    
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "${GREEN}All tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}Some tests failed.${NC}"
        exit 1
    fi
}

main "$@"
