#!/bin/bash

# CURSED Tree-sitter Build Script

set -e

echo "🚀 Building CURSED tree-sitter grammar..."

# Generate parser from grammar
echo "📝 Generating parser..."
npx tree-sitter generate

# Run tests
echo "🧪 Running tests..."
npx tree-sitter test

# Build Node.js bindings if package.json exists
if [ -f "package.json" ]; then
    echo "📦 Building Node.js bindings..."
    npm run build 2>/dev/null || echo "⚠️ npm build failed or not configured"
fi

# Generate highlighting test if we have queries
if [ -f "queries/highlights.scm" ]; then
    echo "🎨 Testing syntax highlighting..."
    npx tree-sitter highlight test/corpus/basic.txt 2>/dev/null || echo "⚠️ Highlight test failed"
fi

echo "✅ CURSED tree-sitter build complete!"
echo ""
echo "Usage:"
echo "  npx tree-sitter parse examples/hello.💀    # Parse a file"
echo "  npx tree-sitter test                        # Run tests"
echo "  npx tree-sitter highlight file.💀          # Test highlighting"
echo ""
echo "Integration:"
echo "  See integration.md for IDE and compiler integration"
