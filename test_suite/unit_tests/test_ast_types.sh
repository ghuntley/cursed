#!/bin/bash

# Test AST type checking

set -e

# Test type checking
cat > type_test.csd << 'EOF'
fr fr Test type checking
sus int_val drip = 42
sus float_val meal = 3.14
sus string_val tea = "hello"
sus bool_val lit = based

fr fr Test type conversions
sus converted_float meal = int_val.(meal)
sus converted_int drip = float_val.(drip)

fr fr Test array types
sus int_array []drip = [1, 2, 3, 4]
sus string_array []tea = ["a", "b", "c"]

fr fr Test map types
sus int_map map[tea]drip = {"a": 1, "b": 2}

fr fr Test function types as values
slay add(a drip, b drip) drip {
    damn a + b
}

sus operation = add
sus result drip = operation(5, 3)
EOF

# Test that types are properly checked
./cursed-unified type_test.csd

# Cleanup
rm -f type_test.csd

exit 0
