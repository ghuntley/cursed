#!/bin/bash

# Test basic code generation

set -e

# Test basic operations
cat > codegen_basic_test.csd << 'EOF'
fr fr Test basic code generation
sus a drip = 5
sus b drip = 3
sus sum drip = a + b
sus diff drip = a - b
sus prod drip = a * b
sus quot drip = a / b

vibez.spill(sum)
vibez.spill(diff)
vibez.spill(prod)
vibez.spill(quot)
EOF

# Test interpretation mode
./cursed-unified codegen_basic_test.csd

# Test compilation mode
./cursed-unified --compile codegen_basic_test.csd
if [ -f ./codegen_basic_test ]; then
    ./codegen_basic_test
    rm -f codegen_basic_test
fi

# Cleanup
rm -f codegen_basic_test.csd

exit 0
