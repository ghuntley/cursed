#!/bin/bash
# Comprehensive End-to-End Compilation Pipeline Tests
# Tests entire compiler pipeline: lexing -> parsing -> codegen -> execution

set -e

echo "🧪 Starting Comprehensive E2E Compiler Pipeline Tests"
echo "========================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test execution with validation
run_test() {
    local test_name="$1"
    local test_file="$2"
    local expected_output="$3"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -e "${BLUE}🔍 Testing: $test_name${NC}"
    
    # Test interpretation mode
    echo "  📝 Testing interpretation mode..."
    if timeout 30 cargo run --bin cursed "$test_file" > /tmp/interp_output.txt 2>&1; then
        if [[ -n "$expected_output" ]] && ! grep -q "$expected_output" /tmp/interp_output.txt; then
            echo -e "  ${RED}❌ FAIL: Interpretation output doesn't match expected${NC}"
            echo "    Expected: $expected_output"
            echo "    Got: $(cat /tmp/interp_output.txt | head -n 5)"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return 1
        fi
        echo -e "  ${GREEN}✅ Interpretation passed${NC}"
    else
        echo -e "  ${RED}❌ FAIL: Interpretation failed${NC}"
        cat /tmp/interp_output.txt | head -n 10
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
    
    # Test compilation mode
    echo "  🔧 Testing compilation mode..."
    local executable_name=$(basename "$test_file" .csd)
    
    if timeout 60 cargo run --bin cursed -- compile "$test_file" > /tmp/compile_output.txt 2>&1; then
        echo -e "  ${GREEN}✅ Compilation passed${NC}"
        
        # Test executable execution
        if [[ -f "./$executable_name" ]]; then
            echo "  🚀 Testing executable execution..."
            if timeout 30 "./$executable_name" > /tmp/exec_output.txt 2>&1; then
                if [[ -n "$expected_output" ]] && ! grep -q "$expected_output" /tmp/exec_output.txt; then
                    echo -e "  ${RED}❌ FAIL: Executable output doesn't match expected${NC}"
                    echo "    Expected: $expected_output"
                    echo "    Got: $(cat /tmp/exec_output.txt | head -n 5)"
                    FAILED_TESTS=$((FAILED_TESTS + 1))
                    return 1
                fi
                echo -e "  ${GREEN}✅ Executable execution passed${NC}"
                rm -f "./$executable_name"  # Cleanup
            else
                echo -e "  ${RED}❌ FAIL: Executable execution failed${NC}"
                cat /tmp/exec_output.txt | head -n 10
                FAILED_TESTS=$((FAILED_TESTS + 1))
                return 1
            fi
        else
            echo -e "  ${RED}❌ FAIL: Executable not generated${NC}"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return 1
        fi
    else
        echo -e "  ${RED}❌ FAIL: Compilation failed${NC}"
        cat /tmp/compile_output.txt | head -n 10
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
    
    PASSED_TESTS=$((PASSED_TESTS + 1))
    echo -e "${GREEN}✅ $test_name: PASSED${NC}"
    echo
}

# Build the compiler first
echo "🔨 Building CURSED compiler..."
if ! cargo build --release > /tmp/build_output.txt 2>&1; then
    echo -e "${RED}❌ Build failed!${NC}"
    cat /tmp/build_output.txt
    exit 1
fi
echo -e "${GREEN}✅ Build successful${NC}"
echo

# Create test directory
mkdir -p /tmp/cursed_e2e_tests
cd /tmp/cursed_e2e_tests

# Copy cursed binary for easier access
cp /home/ghuntley/code/cursed/target/release/cursed ./cursed

echo "📋 Creating comprehensive test programs..."
echo "========================================"

# Test 1: Basic Syntax and Variables
cat > test_basic_syntax.csd << 'EOF'
fr fr Basic syntax test
sus name tea = "CURSED"
sus count drip = 42
sus flag lit = based

vibez.spill("Basic test passed!")
vibez.spill("Name:", name)
vibez.spill("Count:", count)
vibez.spill("Flag:", flag)
EOF

# Test 2: Functions and Control Flow
cat > test_functions_control.csd << 'EOF'
fr fr Functions and control flow test
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

slay test_conditional(x drip) tea {
    lowkey x > 0 {
        damn "positive"
    } highkey x < 0 {
        damn "negative"
    } highkey {
        damn "zero"
    }
}

sus result drip = add_numbers(5, 3)
sus condition tea = test_conditional(result)
vibez.spill("Function test passed!")
vibez.spill("Result:", result)
vibez.spill("Condition:", condition)
EOF

# Test 3: Structs and Methods
cat > test_structs_methods.csd << 'EOF'
fr fr Structs and methods test
squad Point {
    spill x drip
    spill y drip
}

slay Point.distance_from_origin() meal {
    damn math.sqrt(x * x + y * y)
}

sus point Point = Point{x: 3, y: 4}
sus distance meal = point.distance_from_origin()
vibez.spill("Struct test passed!")
vibez.spill("Distance:", distance)
EOF

# Test 4: Arrays and Collections
cat > test_arrays_collections.csd << 'EOF'
fr fr Arrays and collections test
sus numbers []drip = [1, 2, 3, 4, 5]
sus sum drip = 0

bestie i := 0; i < numbers.len(); i = i + 1 {
    sum = sum + numbers[i]
}

vibez.spill("Collection test passed!")
vibez.spill("Sum:", sum)
EOF

# Test 5: Error Handling
cat > test_error_handling.csd << 'EOF'
fr fr Error handling test
slay divide_safe(a drip, b drip) drip {
    lowkey b == 0 {
        yikes "Division by zero"
    }
    damn a / b
}

sus result drip = 0
fam {
    result = divide_safe(10, 2)
    vibez.spill("Error handling test passed!")
    vibez.spill("Result:", result)
} shook err {
    vibez.spill("Caught error:", err)
}
EOF

# Test 6: Interfaces
cat > test_interfaces.csd << 'EOF'
fr fr Interface test
collab Drawable {
    slay draw() tea
}

squad Circle {
    spill radius drip
}

flex Circle => Drawable {
    slay draw() tea {
        damn "Drawing circle with radius " + radius.to_string()
    }
}

sus shape Drawable = Circle{radius: 5}
sus drawing tea = shape.draw()
vibez.spill("Interface test passed!")
vibez.spill("Drawing:", drawing)
EOF

# Test 7: Generics
cat > test_generics.csd << 'EOF'
fr fr Generic types test
slay identity<T>(value T) T {
    damn value
}

slay swap<T>(a T, b T) (T, T) {
    damn (b, a)
}

sus int_result drip = identity<drip>(42)
sus str_result tea = identity<tea>("hello")
sus (x, y) = swap<drip>(1, 2)

vibez.spill("Generic test passed!")
vibez.spill("Int:", int_result)
vibez.spill("String:", str_result)
vibez.spill("Swapped:", x, y)
EOF

# Test 8: Pattern Matching
cat > test_pattern_matching.csd << 'EOF'
fr fr Pattern matching test
slay classify_number(x drip) tea {
    damn match x {
        0 => "zero",
        1 => "one",
        x if x > 0 => "positive",
        _ => "negative"
    }
}

sus result tea = classify_number(42)
vibez.spill("Pattern matching test passed!")
vibez.spill("Classification:", result)
EOF

# Test 9: Concurrency (Basic)
cat > test_concurrency_basic.csd << 'EOF'
fr fr Basic concurrency test
sus ch = make_channel<drip>()

stan {
    dm_send(ch, 42)
    dm_close(ch)
}

sus value drip = dm_recv(ch)
vibez.spill("Concurrency test passed!")
vibez.spill("Received:", value)
EOF

# Test 10: Standard Library Integration
cat > test_stdlib_integration.csd << 'EOF'
yeet "testz"

test_start("stdlib integration")

sus name tea = "CURSED"
sus upper tea = name.to_upper()
assert_eq_string(upper, "CURSED")

sus numbers []drip = [1, 2, 3]
sus length drip = numbers.len()
assert_eq_int(length, 3)

print_test_summary()
vibez.spill("Stdlib integration test passed!")
EOF

echo "🧪 Running End-to-End Pipeline Tests..."
echo "======================================="

# Set working directory back to CURSED project
cd /home/ghuntley/code/cursed

# Run all tests
run_test "Basic Syntax and Variables" "/tmp/cursed_e2e_tests/test_basic_syntax.csd" "Basic test passed!"
run_test "Functions and Control Flow" "/tmp/cursed_e2e_tests/test_functions_control.csd" "Function test passed!"
run_test "Structs and Methods" "/tmp/cursed_e2e_tests/test_structs_methods.csd" "Struct test passed!"
run_test "Arrays and Collections" "/tmp/cursed_e2e_tests/test_arrays_collections.csd" "Collection test passed!"
run_test "Error Handling" "/tmp/cursed_e2e_tests/test_error_handling.csd" "Error handling test passed!"
run_test "Interfaces" "/tmp/cursed_e2e_tests/test_interfaces.csd" "Interface test passed!"
run_test "Generics" "/tmp/cursed_e2e_tests/test_generics.csd" "Generic test passed!"
run_test "Pattern Matching" "/tmp/cursed_e2e_tests/test_pattern_matching.csd" "Pattern matching test passed!"
run_test "Basic Concurrency" "/tmp/cursed_e2e_tests/test_concurrency_basic.csd" "Concurrency test passed!"
run_test "Standard Library Integration" "/tmp/cursed_e2e_tests/test_stdlib_integration.csd" "Stdlib integration test passed!"

echo "📊 Final Test Results"
echo "===================="
echo -e "Total tests: ${BLUE}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [[ $FAILED_TESTS -eq 0 ]]; then
    echo -e "${GREEN}🎉 All end-to-end pipeline tests passed!${NC}"
    echo -e "${GREEN}✅ CURSED compiler pipeline is fully functional${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Compiler pipeline needs fixes.${NC}"
    exit 1
fi

# Cleanup
rm -rf /tmp/cursed_e2e_tests
rm -f /tmp/interp_output.txt /tmp/compile_output.txt /tmp/exec_output.txt /tmp/build_output.txt
