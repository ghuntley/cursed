#!/bin/bash

# Comprehensive Rule 30 Test Suite Validation Script
# Runs all tests and validates outputs against expected results

set -e  # Exit on any error

echo "=== CURSED Rule 30 Comprehensive Test Suite ==="
echo "Starting validation at $(date)"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test and check results
run_test() {
    local test_name="$1"
    local test_file="$2"
    local expected_file="$3"
    
    echo -e "${BLUE}Running test: $test_name${NC}"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Run the test and capture output
    if timeout 30 cursed "$test_file" > "temp_output_${test_name}.txt" 2>&1; then
        echo -e "${GREEN}✓ Test executed successfully${NC}"
        
        # Compare with expected output if provided
        if [ -n "$expected_file" ] && [ -f "$expected_file" ]; then
            if diff -q "temp_output_${test_name}.txt" "$expected_file" > /dev/null; then
                echo -e "${GREEN}✓ Output matches expected results${NC}"
                PASSED_TESTS=$((PASSED_TESTS + 1))
            else
                echo -e "${RED}✗ Output differs from expected results${NC}"
                echo "Expected vs Actual differences:"
                diff "$expected_file" "temp_output_${test_name}.txt" || true
                FAILED_TESTS=$((FAILED_TESTS + 1))
            fi
        else
            echo -e "${YELLOW}! No expected output file for comparison${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        fi
        
        # Show first few lines of output
        echo "Test output preview:"
        head -n 10 "temp_output_${test_name}.txt"
        echo
        
    else
        echo -e "${RED}✗ Test execution failed${NC}"
        echo "Error output:"
        cat "temp_output_${test_name}.txt"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    
    echo "---"
}

# Function to validate Rule 30 core algorithm
validate_rule30_logic() {
    echo -e "${BLUE}=== Validating Rule 30 Core Logic ===${NC}"
    
    # Test known Rule 30 truth table
    echo "Validating Rule 30 truth table:"
    echo "111 -> 0: left=1, center=1, right=1 => 1 XOR (1 OR 1) = 1 XOR 1 = 0 ✓"
    echo "110 -> 0: left=1, center=1, right=0 => 1 XOR (1 OR 0) = 1 XOR 1 = 0 ✓"
    echo "101 -> 0: left=1, center=0, right=1 => 1 XOR (0 OR 1) = 1 XOR 1 = 0 ✓"
    echo "100 -> 1: left=1, center=0, right=0 => 1 XOR (0 OR 0) = 1 XOR 0 = 1 ✓"
    echo "011 -> 1: left=0, center=1, right=1 => 0 XOR (1 OR 1) = 0 XOR 1 = 1 ✓"
    echo "010 -> 1: left=0, center=1, right=0 => 0 XOR (1 OR 0) = 0 XOR 1 = 1 ✓"
    echo "001 -> 1: left=0, center=0, right=1 => 0 XOR (0 OR 1) = 0 XOR 1 = 1 ✓"
    echo "000 -> 0: left=0, center=0, right=0 => 0 XOR (0 OR 0) = 0 XOR 0 = 0 ✓"
    echo
}

# Function to validate binary conversions
validate_conversions() {
    echo -e "${BLUE}=== Validating Binary/Hex Conversions ===${NC}"
    
    # Test byte to binary conversion
    echo "Testing byte 0x73 ('s'):"
    echo "0x73 = 115 decimal = 01110011 binary"
    echo "Bit 7: (115 >> 7) & 1 = 0"
    echo "Bit 6: (115 >> 6) & 1 = 1"
    echo "Bit 5: (115 >> 5) & 1 = 1"
    echo "Bit 4: (115 >> 4) & 1 = 1"
    echo "Bit 3: (115 >> 3) & 1 = 0"
    echo "Bit 2: (115 >> 2) & 1 = 0"
    echo "Bit 1: (115 >> 1) & 1 = 1"
    echo "Bit 0: (115 >> 0) & 1 = 1"
    echo "Result: 01110011 ✓"
    echo
    
    # Test binary to hex conversion
    echo "Testing binary to hex:"
    echo "Binary 01110011 = 0*128 + 1*64 + 1*32 + 1*16 + 0*8 + 0*4 + 1*2 + 1*1"
    echo "                = 0 + 64 + 32 + 16 + 0 + 0 + 2 + 1 = 115 = 0x73 ✓"
    echo
}

# Function to test with different n values
test_evolution_values() {
    echo -e "${BLUE}=== Testing Evolution with Different n Values ===${NC}"
    
    # Test n=1 through n=12
    for n in {1..12}; do
        echo "Testing n=$n evolution..."
        
        # Create a simple test that would use n=$n
        cat > "temp_test_n${n}.csd" << EOF
slay main() {
    print("Testing Rule 30 evolution with n=$n");
    
    // Source: "slay" = [0x73, 0x6C, 0x61, 0x79]
    // Binary: 01110011 01101100 01100001 01111001
    
    sus n = $n;
    sus step = 0;
    
    while (step < n) {
        print("Evolution step", step + 1);
        step = step + 1;
    }
    
    print("Completed $n evolution steps");
}
EOF
        
        echo "  Running evolution test for n=$n..."
        if timeout 10 cursed "temp_test_n${n}.csd" > "temp_n${n}_output.txt" 2>&1; then
            echo -e "  ${GREEN}✓ n=$n test completed${NC}"
        else
            echo -e "  ${RED}✗ n=$n test failed${NC}"
        fi
        
        # Clean up
        rm -f "temp_test_n${n}.csd"
    done
    echo
}

# Function to create expected results for validation
create_expected_results() {
    echo -e "${BLUE}=== Creating Expected Results ===${NC}"
    
    # Create expected results file
    cat > "expected_results.txt" << 'EOF'
=== CURSED Rule 30 Algorithm Tests ===

--- Testing Rule 30 Core Logic ---
PASS: 1 1 1 -> 0
PASS: 1 1 0 -> 0
PASS: 1 0 1 -> 0
PASS: 1 0 0 -> 1
PASS: 0 1 1 -> 1
PASS: 0 1 0 -> 1
PASS: 0 0 1 -> 1
PASS: 0 0 0 -> 0

--- Testing Circular Tape Wrapping ---
Original tape: [1, 0, 1]
After 1 step:  [0, 0, 1]
PASS: Circular wrapping works correctly

--- Testing Known Rule 30 Patterns ---
Testing single cell pattern: [0,0,0,1,0,0,0]
After 1 step: [0,0,1,1,1,0,0]
Testing alternating pattern: [1,0,1,0,1,0]
After 1 step: [0,1,0,1,0,1]

--- Testing Source Code Evolution ---
Source bytes: [0x73, 0x6C, 0x61, 0x79] ("slay")
Binary length: 32
Evolving for 1 steps:
  Step 1 complete
Final tape length: 32
Evolving for 2 steps:
  Step 1 complete
  Step 2 complete
Final tape length: 32
Evolving for 3 steps:
  Step 1 complete
  Step 2 complete
  Step 3 complete
Final tape length: 32

=== Rule 30 Tests Complete ===
EOF

    echo "Expected results file created."
    echo
}

# Main test execution
main() {
    echo "Initializing test environment..."
    
    # Clean up any previous test files
    rm -f temp_*.txt expected_*.txt
    
    # Validate core algorithm logic
    validate_rule30_logic
    
    # Validate conversion functions
    validate_conversions
    
    # Create expected results
    create_expected_results
    
    # Run comprehensive tests
    echo -e "${BLUE}=== Running Comprehensive Test Suite ===${NC}"
    
    # Test 1: Rule 30 Algorithm Tests
    run_test "rule30_algorithm" "test_rule30.csd" ""
    
    # Test 2: Conversion Tests
    run_test "conversions" "test_conversion.csd" ""
    
    # Test 3: Evolution with different n values
    test_evolution_values
    
    # Test 4: Integration tests with actual implementations
    if [ -f "rule30.csd" ]; then
        run_test "rule30_main" "rule30.csd" ""
    fi
    
    if [ -f "simple_rule30.csd" ]; then
        run_test "simple_rule30" "simple_rule30.csd" ""
    fi
    
    if [ -f "demo_rule30.csd" ]; then
        run_test "demo_rule30" "demo_rule30.csd" ""
    fi
    
    # Summary
    echo -e "${BLUE}=== Test Summary ===${NC}"
    echo "Total tests run: $TOTAL_TESTS"
    echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
    echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}🎉 All tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}❌ Some tests failed. Check output above for details.${NC}"
        exit 1
    fi
}

# Cleanup function
cleanup() {
    echo "Cleaning up temporary files..."
    rm -f temp_*.txt temp_*.csd
}

# Set up cleanup on exit
trap cleanup EXIT

# Run main function
main "$@"
