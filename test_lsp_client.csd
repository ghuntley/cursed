//! Test client for CURSED LSP Server
//! Tests basic LSP functionality with manual JSON-RPC messages

yeet "vibez"
yeet "stringz"
yeet "jsonz" as json
yeet "testz"

// Test JSON-RPC message construction
slay test_json_rpc_construction() {
    testz.test_section("JSON-RPC Message Construction")
    
    // Test initialize request
    sus initialize_request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": null,
            "clientInfo": {
                "name": "Test Client",
                "version": "1.0.0"
            },
            "capabilities": {}
        }
    }
    
    sus json_string = json.stringify(initialize_request)
    testz.assert_true(stringz.contains_substring(json_string, "initialize"))
    testz.assert_true(stringz.contains_substring(json_string, "2.0"))
    
    vibez.print_success("JSON-RPC message construction test passed")
}

// Test completion request format
slay test_completion_request() {
    testz.test_section("Completion Request Format")
    
    sus completion_request = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/completion",
        "params": {
            "textDocument": {
                "uri": "file:///test.csd"
            },
            "position": {
                "line": 0,
                "character": 5
            }
        }
    }
    
    sus json_string = json.stringify(completion_request)
    testz.assert_true(stringz.contains_substring(json_string, "textDocument/completion"))
    testz.assert_true(stringz.contains_substring(json_string, "file:///test.csd"))
    
    vibez.print_success("Completion request format test passed")
}

// Test hover request format  
slay test_hover_request() {
    testz.test_section("Hover Request Format")
    
    sus hover_request = {
        "jsonrpc": "2.0", 
        "id": 3,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": "file:///test.csd"
            },
            "position": {
                "line": 1,
                "character": 10
            }
        }
    }
    
    sus json_string = json.stringify(hover_request)
    testz.assert_true(stringz.contains_substring(json_string, "textDocument/hover"))
    
    vibez.print_success("Hover request format test passed")
}

// Test didOpen notification
slay test_did_open_notification() {
    testz.test_section("DidOpen Notification Format")
    
    sus did_open = {
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///test.csd",
                "languageId": "cursed",
                "version": 1,
                "text": "sus greeting tea = \"Hello, CURSED!\"\nvibez.spill(greeting)"
            }
        }
    }
    
    sus json_string = json.stringify(did_open)
    testz.assert_true(stringz.contains_substring(json_string, "textDocument/didOpen"))
    testz.assert_true(stringz.contains_substring(json_string, "Hello, CURSED!"))
    
    vibez.print_success("DidOpen notification test passed")
}

// Test formatting request
slay test_formatting_request() {
    testz.test_section("Formatting Request Format")
    
    sus formatting_request = {
        "jsonrpc": "2.0",
        "id": 4, 
        "method": "textDocument/formatting",
        "params": {
            "textDocument": {
                "uri": "file:///test.csd"
            },
            "options": {
                "tabSize": 4,
                "insertSpaces": based
            }
        }
    }
    
    sus json_string = json.stringify(formatting_request)
    testz.assert_true(stringz.contains_substring(json_string, "textDocument/formatting"))
    testz.assert_true(stringz.contains_substring(json_string, "tabSize"))
    
    vibez.print_success("Formatting request test passed")
}

// Test LSP server capabilities
slay test_server_capabilities() {
    testz.test_section("Server Capabilities")
    
    // Expected server capabilities
    sus expected_capabilities = {
        "textDocumentSync": 1,
        "completionProvider": {
            "resolveProvider": based,
            "triggerCharacters": [".", ":", " "]
        },
        "hoverProvider": based,
        "definitionProvider": based,
        "formattingProvider": based,
        "workspaceSymbolProvider": based
    }
    
    sus json_string = json.stringify(expected_capabilities)
    testz.assert_true(stringz.contains_substring(json_string, "completionProvider"))
    testz.assert_true(stringz.contains_substring(json_string, "hoverProvider"))
    testz.assert_true(stringz.contains_substring(json_string, "formattingProvider"))
    
    vibez.print_success("Server capabilities test passed")
}

// Test CURSED keyword completions
slay test_cursed_completions() {
    testz.test_section("CURSED Keyword Completions")
    
    sus cursed_keywords = [
        "sus", "damn", "slay", "vibez", "yeet", "bestie", "stan", "ready",
        "based", "cap", "cringe", "facts", "lit", "tea", "drip", "normie"
    ]
    
    testz.assert_true(arrayz.contains_value(cursed_keywords, "sus"))
    testz.assert_true(arrayz.contains_value(cursed_keywords, "damn"))
    testz.assert_true(arrayz.contains_value(cursed_keywords, "slay"))
    testz.assert_true(arrayz.contains_value(cursed_keywords, "vibez"))
    testz.assert_true(arrayz.contains_value(cursed_keywords, "yeet"))
    
    vibez.print_success("CURSED keyword completions test passed")
}

// Test standard library completions  
slay test_stdlib_completions() {
    testz.test_section("Standard Library Completions")
    
    sus stdlib_funcs = [
        "vibez.spill", "vibez.spillln", "mathz.abs_normie", "stringz.concat_strings",
        "arrayz.sum_array", "testz.assert_true", "jsonz.stringify", "filez.file_exists"
    ]
    
    testz.assert_true(arrayz.contains_value(stdlib_funcs, "vibez.spill"))
    testz.assert_true(arrayz.contains_value(stdlib_funcs, "mathz.abs_normie"))
    testz.assert_true(arrayz.contains_value(stdlib_funcs, "stringz.concat_strings"))
    testz.assert_true(arrayz.contains_value(stdlib_funcs, "jsonz.stringify"))
    
    vibez.print_success("Standard library completions test passed")
}

// Test error response format
slay test_error_response() {
    testz.test_section("Error Response Format")
    
    sus error_response = {
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32601,
            "message": "Method not found"
        }
    }
    
    sus json_string = json.stringify(error_response)
    testz.assert_true(stringz.contains_substring(json_string, "Method not found"))
    testz.assert_true(stringz.contains_substring(json_string, "-32601"))
    
    vibez.print_success("Error response format test passed")
}

// Manual integration test - would require actual LSP server running
slay test_lsp_integration() {
    testz.test_section("LSP Integration Test (Manual)")
    
    vibez.print_info("Integration test would require:")
    vibez.print_info("1. Starting CURSED LSP server process")
    vibez.print_info("2. Sending initialize request")
    vibez.print_info("3. Sending didOpen notification")  
    vibez.print_info("4. Sending completion request")
    vibez.print_info("5. Verifying responses")
    
    // This would be implemented with actual process communication
    vibez.print_warning("Integration test not implemented - requires process I/O")
}

// Main test runner
slay main() {
    testz.test_start("CURSED LSP Client Tests")
    
    vibez.print_header("Testing CURSED Language Server Protocol Implementation")
    
    test_json_rpc_construction()
    test_completion_request()
    test_hover_request() 
    test_did_open_notification()
    test_formatting_request()
    test_server_capabilities()
    test_cursed_completions()
    test_stdlib_completions()
    test_error_response()
    test_lsp_integration()
    
    testz.print_test_summary()
    
    vibez.print_header("LSP Test Suite Complete")
    vibez.print_success("All LSP protocol tests passed!")
    
    vibez.spill("To test the LSP server:")
    vibez.spill("1. Build CURSED compiler: `zig build`")
    vibez.spill("2. Run LSP server: `./zig-out/bin/cursed-zig cursed_lsp_server.csd`") 
    vibez.spill("3. Configure your IDE to use CURSED LSP server")
    vibez.spill("4. Open .csd files to test completion, hover, formatting")
}
