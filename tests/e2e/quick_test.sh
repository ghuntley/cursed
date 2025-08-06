#!/bin/bash

# Quick test runner for development - runs a subset of critical tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CURSED_BIN="${CURSED_BIN:-../../zig-out/bin/cursed}"
CURSED_UNIFIED="${CURSED_UNIFIED:-../../zig-out/bin/cursed}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    local level="$1"
    shift
    case "$level" in
        "INFO") echo -e "${BLUE}[INFO]${NC} $*" ;;
        "PASS") echo -e "${GREEN}[PASS]${NC} $*" ;;
        "FAIL") echo -e "${RED}[FAIL]${NC} $*" ;;
    esac
}

# Critical tests that must pass for basic functionality
CRITICAL_TESTS=(
    "basic/01_variables.csd"
    "basic/02_functions.csd"
    "basic/03_basic_io.csd"
    "control_flow/01_if_else.csd"
    "stdlib/01_testz_framework.csd"
)

echo -e "${BLUE}CURSED Quick Test Suite${NC}"
echo "======================="
echo ""

total=0
passed=0

for test_path in "${CRITICAL_TESTS[@]}"; do
    full_path="$SCRIPT_DIR/$test_path"
    
    if [[ ! -f "$full_path" ]]; then
        log "FAIL" "$test_path (file not found)"
        ((total++))
        continue
    fi
    
    log "INFO" "Testing $test_path"
    
    # Try interpretation mode first
    if [[ -x "$CURSED_BIN" ]]; then
        if timeout 15 "$CURSED_BIN" "$full_path" >/dev/null 2>&1; then
            log "PASS" "$test_path (interpret)"
            ((passed++))
        elif [[ -x "$CURSED_UNIFIED" ]] && timeout 15 "$CURSED_UNIFIED" "$full_path" >/dev/null 2>&1; then
            log "PASS" "$test_path (unified)"
            ((passed++))
        else
            log "FAIL" "$test_path"
        fi
    elif [[ -x "$CURSED_UNIFIED" ]]; then
        if timeout 15 "$CURSED_UNIFIED" "$full_path" >/dev/null 2>&1; then
            log "PASS" "$test_path (unified)"
            ((passed++))
        else
            log "FAIL" "$test_path"
        fi
    else
        log "FAIL" "$test_path (no binary found)"
    fi
    
    ((total++))
done

echo ""
echo "Quick Test Summary:"
echo "==================="
echo "Tests run: $total"
echo -e "Passed:    ${GREEN}$passed${NC}"
echo -e "Failed:    ${RED}$((total - passed))${NC}"

if [[ $passed -eq $total ]]; then
    echo -e "${GREEN}All critical tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some critical tests failed.${NC}"
    echo "Run './run_e2e_tests.sh --verbose' for detailed output"
    exit 1
fi
