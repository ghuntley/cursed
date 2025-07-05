#!/bin/bash

# Test CURSED language evaluation in REPL
echo "Testing CURSED language evaluation..."

# Test basic expressions
echo -e "42\n\"Hello, World!\"\ntrue\n:quit" | timeout 10 ./target/x86_64-unknown-linux-gnu/debug/cursed repl

echo ""
echo "CURSED language evaluation test completed!"
