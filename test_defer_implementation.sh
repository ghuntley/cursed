#!/bin/bash

echo "🧪 Testing CURSED Defer Implementation"
echo "======================================="

# Test 1: Basic defer functionality
echo "Test 1: Basic defer functionality"
echo "later vibez.spill(\"Defer test 1\")" > test_defer_1.csd
echo "vibez.spill(\"Normal execution\")" >> test_defer_1.csd

# Test 2: Multiple defer statements (LIFO order)
echo "Test 2: Multiple defer statements"
cat > test_defer_2.csd << 'EOF'
slay test_multiple_defer() lit {
    vibez.spill("Start function")
    later vibez.spill("Defer 1 - should execute last")
    later vibez.spill("Defer 2 - should execute second")
    later vibez.spill("Defer 3 - should execute first")
    vibez.spill("End function")
    damn based
}
test_multiple_defer()
EOF

# Test 3: Defer with early return
echo "Test 3: Defer with early return"
cat > test_defer_3.csd << 'EOF'
slay test_defer_return() lit {
    vibez.spill("Function start")
    later vibez.spill("Defer cleanup - should execute before return")
    
    lowkey based {
        vibez.spill("Taking early return")
        damn based
    }
    
    vibez.spill("This should not execute")
    damn cap
}
test_defer_return()
EOF

# Test 4: Defer with resource management
echo "Test 4: Defer with resource management"
cat > test_defer_4.csd << 'EOF'
slay test_resource_management() lit {
    vibez.spill("Opening resource")
    
    later vibez.spill("Closing resource")
    later vibez.spill("Cleaning up temporary files")
    
    vibez.spill("Using resource")
    vibez.spill("Resource operations complete")
    
    damn based
}
test_resource_management()
EOF

# Test 5: Nested defer scopes
echo "Test 5: Nested defer scopes"
cat > test_defer_5.csd << 'EOF'
slay test_nested_defer() lit {
    vibez.spill("Outer function start")
    later vibez.spill("Outer defer cleanup")
    
    lowkey based {
        vibez.spill("Inner block start")
        later vibez.spill("Inner defer cleanup")
        vibez.spill("Inner block end")
    }
    
    vibez.spill("Outer function end")
    damn based
}
test_nested_defer()
EOF

echo ""
echo "🔍 Running defer tests..."
echo ""

# Function to run a test
run_test() {
    local test_num=$1
    local test_file=$2
    local description=$3
    
    echo "Running Test $test_num: $description"
    echo "File: $test_file"
    echo "----------------------------------------"
    
    # Check if file exists
    if [ ! -f "$test_file" ]; then
        echo "❌ Test file not found: $test_file"
        return 1
    fi
    
    # Show file content
    echo "📄 Test code:"
    cat "$test_file"
    echo ""
    
    # Try interpretation mode
    echo "🔄 Interpretation mode:"
    if cargo run --bin cursed "$test_file" 2>&1; then
        echo "✅ Interpretation mode: PASSED"
    else
        echo "❌ Interpretation mode: FAILED"
    fi
    
    echo ""
    
    # Try compilation mode
    echo "🔄 Compilation mode:"
    if cargo run --bin cursed -- compile "$test_file" 2>&1; then
        executable_name=$(basename "$test_file" .csd)
        if [ -f "$executable_name" ]; then
            echo "Running compiled executable..."
            ./"$executable_name"
            echo "✅ Compilation mode: PASSED"
        else
            echo "❌ Compilation mode: Executable not found"
        fi
    else
        echo "❌ Compilation mode: FAILED"
    fi
    
    echo ""
    echo "=========================================="
    echo ""
}

# Run all tests
run_test 1 "test_defer_1.csd" "Basic defer functionality"
run_test 2 "test_defer_2.csd" "Multiple defer statements (LIFO)"
run_test 3 "test_defer_3.csd" "Defer with early return"
run_test 4 "test_defer_4.csd" "Defer with resource management"
run_test 5 "test_defer_5.csd" "Nested defer scopes"

echo ""
echo "🏁 All defer tests completed!"
echo "📊 Check the output above for detailed results"

# Cleanup
echo "🧹 Cleaning up test files..."
rm -f test_defer_*.csd
rm -f test_defer_*  # Remove any compiled executables

echo "✅ Cleanup complete!"
