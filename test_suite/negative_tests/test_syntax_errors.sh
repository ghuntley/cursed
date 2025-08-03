#!/bin/bash

# Test syntax error detection and reporting

set -e

test_syntax_error() {
    local test_name="$1"
    local test_file="$2"
    
    echo "Testing syntax error: $test_name"
    
    # Expect this to fail with syntax error
    if ./cursed-unified "$test_file" > /dev/null 2>&1; then
        echo "ERROR: Expected syntax error but compilation succeeded for $test_name"
        return 1
    else
        echo "PASS: Correctly detected syntax error for $test_name"
        return 0
    fi
}

# Missing semicolon/statement terminator
cat > missing_terminator.csd << 'EOF'
sus x drip = 42
sus y drip = 43  fr fr Missing proper termination
vibez.spill(x)
EOF

# Mismatched braces
cat > mismatched_braces.csd << 'EOF'
slay test() {
    vibez.spill("hello"
fr fr Missing closing brace
EOF

# Invalid variable declaration
cat > invalid_variable.csd << 'EOF'
sus 123invalid drip = 42  fr fr Variable name cannot start with number
EOF

# Invalid function syntax
cat > invalid_function.csd << 'EOF'
slay function_name  fr fr Missing parentheses
    vibez.spill("hello")
}
EOF

# Invalid struct syntax
cat > invalid_struct.csd << 'EOF'
squad Point
    spill x meal  fr fr Missing braces
    spill y meal
EOF

# Test all syntax errors
FAILED_TESTS=0

test_syntax_error "missing_terminator" "missing_terminator.csd" || ((FAILED_TESTS++))
test_syntax_error "mismatched_braces" "mismatched_braces.csd" || ((FAILED_TESTS++))
test_syntax_error "invalid_variable" "invalid_variable.csd" || ((FAILED_TESTS++))
test_syntax_error "invalid_function" "invalid_function.csd" || ((FAILED_TESTS++))
test_syntax_error "invalid_struct" "invalid_struct.csd" || ((FAILED_TESTS++))

# Cleanup
rm -f missing_terminator.csd mismatched_braces.csd invalid_variable.csd invalid_function.csd invalid_struct.csd

if [ $FAILED_TESTS -eq 0 ]; then
    echo "All syntax error tests passed"
    exit 0
else
    echo "$FAILED_TESTS syntax error tests failed"
    exit 1
fi
