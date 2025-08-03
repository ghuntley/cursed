#!/bin/bash

# Test end-to-end hello world functionality

set -e

# Create hello world test
cat > hello_world_test.csd << 'EOF'
vibez.spill("Hello, CURSED!")
EOF

# Test interpretation pipeline
INTERP_OUTPUT=$(./cursed-unified hello_world_test.csd 2>&1)
if [[ "$INTERP_OUTPUT" == *"Hello, CURSED!"* ]]; then
    echo "Interpretation mode: PASS"
else
    echo "Interpretation mode: FAIL"
    exit 1
fi

# Test compilation pipeline
./cursed-unified --compile hello_world_test.csd
if [ -f ./hello_world_test ]; then
    COMP_OUTPUT=$(./hello_world_test 2>&1)
    if [[ "$COMP_OUTPUT" == *"Hello, CURSED!"* ]]; then
        echo "Compilation mode: PASS"
    else
        echo "Compilation mode: FAIL"
        exit 1
    fi
    rm -f hello_world_test
else
    echo "Compilation failed to produce executable"
    exit 1
fi

# Cleanup
rm -f hello_world_test.csd

exit 0
