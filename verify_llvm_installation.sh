#!/bin/bash

echo "=== LLVM Tools Installation Verification ==="

# Add LLVM tools to PATH
export PATH="/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin:/nix/store/32jfd5s845ys74nkzlgsh7cnq83y4lri-clang-wrapper-19.1.7/bin:$PATH"

echo "1. Checking for llc..."
if which llc >/dev/null 2>&1; then
    echo "✅ llc found at: $(which llc)"
    llc --version | head -3
else
    echo "❌ llc not found"
fi

echo -e "\n2. Checking for clang..."
if which clang >/dev/null 2>&1; then
    echo "✅ clang found at: $(which clang)"
    clang --version | head -1
else
    echo "❌ clang not found"
fi

echo -e "\n3. Testing CURSED native compilation..."
echo 'vibez.spill("Native compilation test successful!")' > simple_test.csd

echo "   3a. Testing interpretation mode..."
cargo run --bin cursed simple_test.csd

echo -e "\n   3b. Testing native compilation..."
cargo run --bin cursed -- compile simple_test.csd

echo -e "\n   3c. Running compiled executable..."
if [ -f "simple_test" ]; then
    ./simple_test
    echo "✅ Native compilation successful!"
else
    echo "❌ Native compilation failed - executable not created"
fi

echo -e "\n=== Installation Complete ==="
