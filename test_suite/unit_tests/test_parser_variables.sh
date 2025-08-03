#!/bin/bash

# Test parser variable declaration handling

set -e

# Test variable declarations
cat > variable_test.csd << 'EOF'
fr fr Test variable declarations
sus int_var drip = 42
sus float_var meal = 3.14
sus string_var tea = "hello"
sus bool_var lit = based

fr fr Test short variable declaration
x := 42
name := "world"
active := based

fr fr Test tuple assignment
(a, b, c) := (1, 2, 3)
EOF

# Test that variables are properly parsed
./cursed-unified variable_test.csd

# Cleanup
rm -f variable_test.csd

exit 0
