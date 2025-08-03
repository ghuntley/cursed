#!/bin/bash

# Test memory leak detection

set -e

# Create memory-intensive test
cat > memory_test.csd << 'EOF'
fr fr Memory leak detection test

slay allocate_memory() {
    sus large_array []drip = []
    
    bestie i := 0; i < 1000; i = i + 1 {
        large_array.push(i)
    }
    
    sus large_map map[drip]tea = {}
    
    bestie i := 0; i < 500; i = i + 1 {
        large_map[i] = "value_" + i.to_string()
    }
    
    vibez.spill("Allocated memory for iteration")
}

fr fr Run multiple iterations to test memory cleanup
bestie i := 0; i < 10; i = i + 1 {
    allocate_memory()
    vibez.spill("Iteration:", i)
}

vibez.spill("Memory test completed")
EOF

# Test with interpretation mode (basic test - no valgrind for now)
echo "Running memory test in interpretation mode..."
./cursed-unified memory_test.csd

# Test with compilation mode
echo "Running memory test in compilation mode..."
./cursed-unified --compile memory_test.csd
if [ -f ./memory_test ]; then
    ./memory_test
    rm -f memory_test
fi

# Cleanup
rm -f memory_test.csd

exit 0
