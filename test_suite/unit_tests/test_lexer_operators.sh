#!/bin/bash

# Test lexer operator recognition

set -e

# Test all operators
cat > operator_test.csd << 'EOF'
fr fr Test arithmetic operators
sus a drip = 5 + 3
sus b drip = 5 - 3
sus c drip = 5 * 3
sus d drip = 5 / 3

fr fr Test comparison operators
sus e lit = 5 > 3
sus f lit = 5 < 3
sus g lit = 5 >= 3
sus h lit = 5 <= 3
sus i lit = 5 == 3
sus j lit = 5 != 3

fr fr Test logical operators
sus k lit = based && cringe
sus l lit = based || cringe
sus m lit = !based

fr fr Test assignment operators
sus n drip = 42
n = n + 1
EOF

# Test that operators are properly recognized
./cursed-unified operator_test.csd

# Cleanup
rm -f operator_test.csd

exit 0
