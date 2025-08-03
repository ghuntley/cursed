#!/bin/bash

# Test type error detection

set -e

test_type_error() {
    local test_name="$1"
    local test_file="$2"
    
    echo "Testing type error: $test_name"
    
    # Expect this to fail with type error
    if ./cursed-unified "$test_file" > /dev/null 2>&1; then
        echo "ERROR: Expected type error but compilation succeeded for $test_name"
        return 1
    else
        echo "PASS: Correctly detected type error for $test_name"
        return 0
    fi
}

# Type mismatch in assignment
cat > type_mismatch.csd << 'EOF'
sus x drip = 42
x = "string"  fr fr Cannot assign string to integer variable
EOF

# Function parameter type mismatch
cat > function_param_mismatch.csd << 'EOF'
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

sus result drip = add_numbers(42, "not a number")  fr fr Wrong parameter type
EOF

# Return type mismatch
cat > return_type_mismatch.csd << 'EOF'
slay get_number() drip {
    damn "not a number"  fr fr Returning string instead of integer
}
EOF

# Array type mismatch
cat > array_type_mismatch.csd << 'EOF'
sus numbers []drip = [1, 2, "three", 4]  fr fr Mixed types in integer array
EOF

# Struct field type mismatch
cat > struct_type_mismatch.csd << 'EOF'
squad Point {
    spill x meal
    spill y meal
}

sus point Point = Point{x: 1.0, y: "not a number"}  fr fr Wrong field type
EOF

# Test all type errors
FAILED_TESTS=0

test_type_error "type_mismatch" "type_mismatch.csd" || ((FAILED_TESTS++))
test_type_error "function_param_mismatch" "function_param_mismatch.csd" || ((FAILED_TESTS++))
test_type_error "return_type_mismatch" "return_type_mismatch.csd" || ((FAILED_TESTS++))
test_type_error "array_type_mismatch" "array_type_mismatch.csd" || ((FAILED_TESTS++))
test_type_error "struct_type_mismatch" "struct_type_mismatch.csd" || ((FAILED_TESTS++))

# Cleanup
rm -f type_mismatch.csd function_param_mismatch.csd return_type_mismatch.csd array_type_mismatch.csd struct_type_mismatch.csd

if [ $FAILED_TESTS -eq 0 ]; then
    echo "All type error tests passed"
    exit 0
else
    echo "$FAILED_TESTS type error tests failed"
    exit 1
fi
