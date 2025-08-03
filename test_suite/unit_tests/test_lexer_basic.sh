#!/bin/bash

# Test basic lexer functionality

set -e

# Create test files
cat > test_basic_tokens.csd << 'EOF'
sus x drip = 42
sus name tea = "hello"
sus flag lit = based
EOF

cat > test_keywords.csd << 'EOF'
slay function_name() {
    damn "result"
}
EOF

cat > test_operators.csd << 'EOF'
sus result drip = 5 + 3 * 2
sus comparison lit = x > y
EOF

# Test lexer can handle basic tokens
echo 'vibez.spill("Lexer test")' > lexer_simple.csd
./cursed-unified lexer_simple.csd

# Cleanup
rm -f test_basic_tokens.csd test_keywords.csd test_operators.csd lexer_simple.csd

exit 0
