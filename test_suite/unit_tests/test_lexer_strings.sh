#!/bin/bash

# Test lexer string literal handling

set -e

# Test string literals
cat > string_test.csd << 'EOF'
fr fr Test string literals
sus simple tea = "hello world"
sus empty tea = ""
sus escaped tea = "hello \"world\""
sus multiline tea = "line one\nline two"
sus unicode tea = "Hello 🌍"
vibez.spill(simple)
vibez.spill(escaped)
EOF

# Test that strings are properly lexed
./cursed-unified string_test.csd

# Cleanup
rm -f string_test.csd

exit 0
