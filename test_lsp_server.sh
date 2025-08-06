#!/bin/bash

# Test CURSED LSP Server functionality
echo "Testing CURSED Language Server Protocol..."

# Create a simple test file for LSP testing
cat > test_lsp_input.json << 'EOF'
Content-Length: 107

{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{},"rootUri":"file:///test"}}
Content-Length: 246

{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///test.csd","version":1,"languageId":"cursed","text":"slay test_function() {\n    vibez.spill(\"Hello CURSED!\")\n    sus x normie = 42\n}"}}}
Content-Length: 135

{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":1,"character":4}}}
Content-Length: 128

{"jsonrpc":"2.0","id":3,"method":"textDocument/hover","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":2,"character":8}}}
Content-Length: 132

{"jsonrpc":"2.0","id":4,"method":"textDocument/definition","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":2,"character":8}}}
Content-Length: 104

{"jsonrpc":"2.0","id":5,"method":"textDocument/documentSymbol","params":{"textDocument":{"uri":"file:///test.csd"}}}
Content-Length: 53

{"jsonrpc":"2.0","id":6,"method":"shutdown","params":{}}
EOF

echo "Starting LSP server test..."

# Test LSP server with prepared input
timeout 10s ./zig-out/bin/cursed-lsp < test_lsp_input.json > lsp_test_output.log 2>&1

echo "LSP test completed. Output:"
echo "=========================="
cat lsp_test_output.log
echo "=========================="

# Clean up
rm -f test_lsp_input.json lsp_test_output.log

echo "LSP server test finished."
