#!/bin/bash

echo "Building tree-sitter grammar for CURSED..."

# Check if tree-sitter CLI is installed
if ! command -v tree-sitter &> /dev/null; then
    echo "tree-sitter CLI not found. Installing..."
    npm install -g tree-sitter-cli
fi

# Generate parser
echo "Generating parser..."
tree-sitter generate

# Test grammar
echo "Testing grammar..."
tree-sitter test

# Parse a test file
echo "Parsing test file..."
if [ -f "../test_basic_types_working.csd" ]; then
    tree-sitter parse "../test_basic_types_working.csd"
else
    echo "Test file not found, creating a simple test..."
    cat > test_simple.csd << 'EOF'
vibe main

sus x normie = 42
sus y tea = "hello"
sus z lit = based

slay main() {
    vibez.spill("Hello, CURSED!")
    vibez.spill(x)
    vibez.spill(y)
    vibez.spill(z)
}
EOF
    tree-sitter parse test_simple.csd
fi

# Generate language bindings
echo "Generating language bindings..."
if [ -d "src" ]; then
    echo "Parser generated successfully!"
    ls -la src/
else
    echo "Parser generation failed!"
    exit 1
fi

# Test with real CURSED files
echo "Testing with real CURSED files..."
for file in ../*.csd; do
    if [ -f "$file" ]; then
        echo "Testing $file..."
        tree-sitter parse "$file" > /dev/null 2>&1
        if [ $? -eq 0 ]; then
            echo "✅ $file parsed successfully"
        else
            echo "❌ $file failed to parse"
        fi
    fi
done

echo "Tree-sitter grammar build complete!"
