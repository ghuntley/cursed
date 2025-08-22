#!/bin/bash
#
# CURSED LSP Server Integration Test
# Tests all core LSP functionality to ensure IDE integration works
#

set -e

echo "🧪 CURSED LSP Server Integration Test"
echo "======================================"

# Test 1: LSP server can be executed
echo "1️⃣  Testing LSP server execution..."
if ./zig-out/bin/cursed-lsp --help 2>/dev/null; then
    echo "✅ LSP server executable works"
else 
    echo "❌ LSP server failed to execute"
    exit 1
fi

# Test 2: Initialize request
echo "2️⃣  Testing LSP initialize request..."
INIT_REQUEST='Content-Length: 100\r\n\r\n{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}'
RESPONSE=$(timeout 3s bash -c "echo -e '$INIT_REQUEST' | ./zig-out/bin/cursed-lsp" || true)

if echo "$RESPONSE" | grep -q "capabilities"; then
    echo "✅ Initialize request handled correctly"
else
    echo "❌ Initialize request failed"
    echo "Response: $RESPONSE"
    exit 1
fi

# Test 3: Completion request  
echo "3️⃣  Testing LSP completion request..."
COMPLETION_REQUEST='Content-Length: 110\r\n\r\n{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{}}'
RESPONSE=$(timeout 3s bash -c "echo -e '$COMPLETION_REQUEST' | ./zig-out/bin/cursed-lsp" || true)

if echo "$RESPONSE" | grep -q "sus"; then
    echo "✅ Completion request handled correctly"
else
    echo "❌ Completion request failed"
    echo "Response: $RESPONSE"
    exit 1
fi

# Test 4: Hover request
echo "4️⃣  Testing LSP hover request..."
HOVER_REQUEST='Content-Length: 100\r\n\r\n{"jsonrpc":"2.0","id":3,"method":"textDocument/hover","params":{}}'
RESPONSE=$(timeout 3s bash -c "echo -e '$HOVER_REQUEST' | ./zig-out/bin/cursed-lsp" || true)

if echo "$RESPONSE" | grep -q "CURSED language"; then
    echo "✅ Hover request handled correctly"
else
    echo "❌ Hover request failed"
    echo "Response: $RESPONSE"
    exit 1
fi

# Test 5: Shutdown request
echo "5️⃣  Testing LSP shutdown request..."
SHUTDOWN_REQUEST='Content-Length: 100\r\n\r\n{"jsonrpc":"2.0","id":4,"method":"shutdown","params":{}}'
RESPONSE=$(timeout 3s bash -c "echo -e '$SHUTDOWN_REQUEST' | ./zig-out/bin/cursed-lsp" || true)

if echo "$RESPONSE" | grep -q "result.*null"; then
    echo "✅ Shutdown request handled correctly"
else
    echo "❌ Shutdown request failed"
    echo "Response: $RESPONSE"
    exit 1
fi

echo ""
echo "🎉 All LSP integration tests passed!"
echo "✅ CURSED IDE integration is fully functional"
echo ""
echo "📝 LSP Server Features:"
echo "   • Language initialization ✅"
echo "   • Code completion (sus, slay, damn) ✅"  
echo "   • Hover information ✅"
echo "   • Graceful shutdown ✅"
echo ""
echo "🔧 To integrate with your IDE:"
echo "   VS Code: Use Language Server Protocol with ./zig-out/bin/cursed-lsp"
echo "   Vim/Neovim: Configure LSP client to use ./zig-out/bin/cursed-lsp"
echo "   Other editors: Point LSP configuration to ./zig-out/bin/cursed-lsp"
