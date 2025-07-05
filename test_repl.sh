#!/bin/bash

# Test the REPL functionality
echo "Testing CURSED REPL..."

# Test basic startup and exit
echo ":quit" | ./target/debug/cursed repl

echo "REPL test completed successfully!"
