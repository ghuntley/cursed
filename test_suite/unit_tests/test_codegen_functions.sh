#!/bin/bash

# Test function code generation

set -e

# Test function codegen
cat > codegen_functions_test.csd << 'EOF'
fr fr Test function code generation
slay add(a drip, b drip) drip {
    damn a + b
}

slay multiply(x drip, y drip) drip {
    damn x * y
}

slay greet(name tea) {
    vibez.spill("Hello, " + name)
}

sus result1 drip = add(5, 3)
sus result2 drip = multiply(4, 6)
greet("World")

vibez.spill(result1)
vibez.spill(result2)
EOF

# Test interpretation mode
./cursed-unified codegen_functions_test.csd

# Test compilation mode
./cursed-unified --compile codegen_functions_test.csd
if [ -f ./codegen_functions_test ]; then
    ./codegen_functions_test
    rm -f codegen_functions_test
fi

# Cleanup
rm -f codegen_functions_test.csd

exit 0
