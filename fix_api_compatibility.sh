#!/bin/bash

echo "🔧 Starting comprehensive API compatibility fixes..."

# Fix 1: Replace std.time.sleep with std.Thread.sleep
echo "📝 Fixing std.time.sleep -> std.Thread.sleep..."
find src-zig/ -name "*.zig" -exec sed -i 's/std\.time\.sleep/std.Thread.sleep/g' {} \;

# Fix 2: Fix ArrayList .len usage (replace .len with .items.len where needed)
echo "📝 Fixing ArrayList .len usage..."
find src-zig/ -name "*.zig" -exec sed -i 's/\.tokens\.len/\.tokens\.items\.len/g' {} \;
find src-zig/ -name "*.zig" -exec sed -i 's/\.statements\.len/\.statements\.items\.len/g' {} \;
find src-zig/ -name "*.zig" -exec sed -i 's/\.arguments\.len/\.arguments\.items\.len/g' {} \;
find src-zig/ -name "*.zig" -exec sed -i 's/\.parameters\.len/\.parameters\.items\.len/g' {} \;

# Fix 3: Fix ArrayList empty initialization (var name = .empty -> proper initialization)
echo "📝 Fixing ArrayList empty initialization..."
find src-zig/ -name "*.zig" -exec sed -i 's/var \([a-zA-Z_][a-zA-Z0-9_]*\): std::ArrayList(\([^)]*\)) = \.empty;/var \1 = std.ArrayList(\2).init(allocator);/g' {} \;
find src-zig/ -name "*.zig" -exec sed -i 's/var \([a-zA-Z_][a-zA-Z0-9_]*\) = \.empty;/var \1 = std.ArrayList(u8).init(allocator);/g' {} \;

echo "✅ Basic API compatibility fixes applied!"
echo "📋 Next steps: Manual review and specific fixes for:"
echo "   - HashMap.deinit() calls (remove allocator parameter)"
echo "   - StringInterpolationExpression.init (remove allocator parameter)"
echo "   - File writer APIs"
echo "   - Missing switch cases"
echo "   - Error set compatibility"

# Fix 4: Show files that need manual HashMap.deinit fixes
echo "🔍 Files needing HashMap.deinit fixes:"
grep -r "\.deinit(.*allocator" src-zig/ | grep -v "// OK" | head -20

echo "🔧 Script completed! Please run 'zig build' to test."
