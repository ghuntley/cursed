#!/bin/bash

echo "🔧 Starting targeted API compatibility fixes..."

# Fix 1: Replace std.time.sleep with std.Thread.sleep
echo "📝 Fixing std.time.sleep -> std.Thread.sleep..."
find src-zig/ -name "*.zig" -exec sed -i 's/std\.time\.sleep/std.Thread.sleep/g' {} \;

# Fix 2: Fix only specific ArrayList .empty patterns with proper allocator context
echo "📝 Fixing specific ArrayList .empty patterns..."

# Fix var X = .empty; patterns in specific contexts where we know the type
find src-zig/ -name "*.zig" -exec sed -i 's/var \([a-zA-Z_][a-zA-Z0-9_]*\): std\.ArrayList(\([^)]*\)) = \.empty;/var \1 = std.ArrayList(\2).init(self.allocator);/g' {} \;

echo "✅ Targeted API compatibility fixes applied!"
echo "🔧 Script completed! Please run 'zig build' to test."
