#!/bin/bash

# Comprehensive Post-Quantum Cryptography Test Runner
# 
# This script runs all PQC tests with proper environment setup
# and comprehensive reporting.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VERBOSE=false
REPORT=false
COVERAGE=false
QUICK=false
PERFORMANCE=false
INTEGRATION=false
STRESS=false
OUTPUT_FILE=""

# Test categories
BASIC_TESTS="crypto_pqc_test"
PERFORMANCE_TESTS="crypto_pqc_performance_test"
INTEGRATION_TESTS="crypto_pqc_integration_test"

usage() {
    echo "Post-Quantum Cryptography Test Runner"
    echo "====================================="
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --quick                 Run quick validation tests only"
    echo "  --performance          Run performance tests (ignored by default)"
    echo "  --integration          Run integration tests only"
    echo "  --stress               Run stress tests (subset of performance)"
    echo "  --coverage             Generate coverage report"
    echo "  --verbose              Verbose output"
    echo "  --report FILE          Generate detailed report to file"
    echo "  --help                 Show this help message"
    echo ""
    echo "Test Categories:"
    echo "  Basic Tests:           Functional validation of all PQC algorithms"
    echo "  Performance Tests:     Benchmarking and performance analysis"
    echo "  Integration Tests:     End-to-end workflows and real-world scenarios"
    echo ""
    echo "Examples:"
    echo "  $0                           # Run basic tests"
    echo "  $0 --quick                   # Quick validation"
    echo "  $0 --performance --verbose   # Performance tests with verbose output"
    echo "  $0 --coverage --report pqc_report.md  # Full testing with report"
}

log() {
    if [ "$VERBOSE" = true ]; then
        echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $1"
    fi
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

run_test_suite() {
    local test_name="$1"
    local test_flags="$2"
    local description="$3"
    
    echo -e "\n${BLUE}Running $description...${NC}"
    log "Test command: ./fix_linking.sh cargo test --test $test_name $test_flags"
    
    if ./fix_linking.sh cargo test --test "$test_name" $test_flags; then
        success "$description completed successfully"
        return 0
    else
        error "$description failed"
        return 1
    fi
}

generate_report() {
    if [ "$REPORT" = true ] && [ -n "$OUTPUT_FILE" ]; then
        echo "Generating comprehensive PQC test report..."
        
        cat > "$OUTPUT_FILE" << 'EOF'
# Post-Quantum Cryptography Test Report

## Executive Summary

This report provides comprehensive test results for the CURSED programming language's
post-quantum cryptography implementation.

## Test Categories

### Basic Functional Tests
- Kyber KEM (Key Encapsulation Mechanism)
- Dilithium Digital Signatures
- SPHINCS+ Hash-based Signatures  
- Falcon Compact Signatures
- NTRU Encryption
- Quantum Resistance Assessment
- Error Handling and Edge Cases

### Performance Tests
- Algorithm benchmarking across security levels
- Memory usage analysis
- Throughput and latency measurement
- Concurrent operation scaling
- Performance regression detection

### Integration Tests
- Hybrid cryptographic schemes
- Multi-party communication protocols
- Secure channel establishment
- Real-world workflow simulation
- Migration and compatibility testing

## Test Environment

EOF
        
        echo "- Date: $(date)" >> "$OUTPUT_FILE"
        echo "- System: $(uname -a)" >> "$OUTPUT_FILE"
        echo "- Rust version: $(rustc --version)" >> "$OUTPUT_FILE"
        echo "- Cargo version: $(cargo --version)" >> "$OUTPUT_FILE"
        
        echo -e "\n## Test Results\n" >> "$OUTPUT_FILE"
        
        success "Report generated: $OUTPUT_FILE"
    fi
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK=true
            shift
            ;;
        --performance)
            PERFORMANCE=true
            shift
            ;;
        --integration)
            INTEGRATION=true
            shift
            ;;
        --stress)
            STRESS=true
            PERFORMANCE=true
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --report)
            REPORT=true
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Main execution
echo -e "${GREEN}Post-Quantum Cryptography Test Suite${NC}"
echo "======================================"

# Check if fix_linking.sh exists
if [ ! -f "./fix_linking.sh" ]; then
    error "fix_linking.sh not found. Please ensure you're in the correct directory."
    exit 1
fi

# Make fix_linking.sh executable
chmod +x ./fix_linking.sh

# Initialize test results
FAILED_TESTS=0
TOTAL_TESTS=0

# Run tests based on options
if [ "$QUICK" = true ]; then
    echo -e "\n${YELLOW}Running Quick PQC Validation Tests${NC}"
    
    if run_test_suite "crypto_pqc_test" "test_kyber_encaps_decaps_round_trip" "Kyber Round-trip Test"; then
        ((TOTAL_TESTS++))
    else
        ((FAILED_TESTS++))
        ((TOTAL_TESTS++))
    fi
    
    if run_test_suite "crypto_pqc_test" "test_dilithium_sign_verify_round_trip" "Dilithium Round-trip Test"; then
        ((TOTAL_TESTS++))
    else
        ((FAILED_TESTS++))
        ((TOTAL_TESTS++))
    fi
    
elif [ "$PERFORMANCE" = true ]; then
    echo -e "\n${YELLOW}Running PQC Performance Tests${NC}"
    
    if run_test_suite "$PERFORMANCE_TESTS" "-- --ignored" "PQC Performance Tests"; then
        ((TOTAL_TESTS++))
    else
        ((FAILED_TESTS++))
        ((TOTAL_TESTS++))
    fi
    
elif [ "$INTEGRATION" = true ]; then
    echo -e "\n${YELLOW}Running PQC Integration Tests${NC}"
    
    if run_test_suite "$INTEGRATION_TESTS" "" "PQC Integration Tests"; then
        ((TOTAL_TESTS++))
    else
        ((FAILED_TESTS++))
        ((TOTAL_TESTS++))
    fi
    
else
    # Run all basic tests by default
    echo -e "\n${YELLOW}Running Comprehensive PQC Test Suite${NC}"
    
    # Basic functional tests
    if run_test_suite "$BASIC_TESTS" "" "Basic PQC Functional Tests"; then
        ((TOTAL_TESTS++))
    else
        ((FAILED_TESTS++))
        ((TOTAL_TESTS++))
    fi
    
    # Integration tests  
    if run_test_suite "$INTEGRATION_TESTS" "" "PQC Integration Tests"; then
        ((TOTAL_TESTS++))
    else
        ((FAILED_TESTS++))
        ((TOTAL_TESTS++))
    fi
fi

# Coverage analysis
if [ "$COVERAGE" = true ]; then
    echo -e "\n${YELLOW}Generating Coverage Report${NC}"
    
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        log "Running coverage analysis..."
        ./fix_linking.sh cargo tarpaulin --tests crypto_pqc_test crypto_pqc_performance_test crypto_pqc_integration_test --out html --output-dir target/coverage/pqc
        success "Coverage report generated in target/coverage/pqc/"
    else
        warning "cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
    fi
fi

# Generate report
generate_report

# Summary
echo -e "\n${GREEN}Test Suite Summary${NC}"
echo "=================="
echo "Total test suites: $TOTAL_TESTS"
echo "Failed test suites: $FAILED_TESTS"
echo "Success rate: $(( (TOTAL_TESTS - FAILED_TESTS) * 100 / TOTAL_TESTS ))%"

if [ $FAILED_TESTS -eq 0 ]; then
    success "All PQC tests passed!"
    exit 0
else
    error "$FAILED_TESTS test suite(s) failed"
    exit 1
fi
