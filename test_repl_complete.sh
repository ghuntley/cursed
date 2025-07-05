#!/bin/bash

# Test the complete REPL functionality
echo "Testing CURSED REPL functionality..."

# Test basic commands
echo -e ":help\n:version\n:vars\nlet x = 42\nlet message = \"Hello, World!\"\n:vars\n:history\n:quit" | timeout 15 ./target/x86_64-unknown-linux-gnu/debug/cursed repl

echo ""
echo "REPL functionality test completed!"
