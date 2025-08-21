#!/usr/bin/env python3
"""
Simple test harness for CURSED LSP Server
Sends manual JSON-RPC messages to test LSP functionality
"""

import json
import subprocess
import sys
import time

def create_lsp_message(content):
    """Create LSP message with Content-Length header"""
    content_bytes = content.encode('utf-8')
    content_length = len(content_bytes)
    
    header = f"Content-Length: {content_length}\r\n\r\n"
    return header.encode('utf-8') + content_bytes

def create_initialize_request():
    """Create LSP initialize request"""
    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": None,
            "clientInfo": {
                "name": "CURSED LSP Test Client",
                "version": "1.0.0"
            },
            "rootUri": "file:///tmp/cursed_test",
            "capabilities": {
                "textDocument": {
                    "completion": {
                        "completionItem": {
                            "snippetSupport": True,
                            "documentationFormat": ["markdown", "plaintext"]
                        }
                    },
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    "formatting": {}
                }
            }
        }
    }
    
    return json.dumps(request, separators=(',', ':'))

def create_initialized_notification():
    """Create initialized notification"""
    notification = {
        "jsonrpc": "2.0", 
        "method": "initialized",
        "params": {}
    }
    
    return json.dumps(notification, separators=(',', ':'))

def create_did_open_notification():
    """Create textDocument/didOpen notification"""
    notification = {
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///tmp/test.csd",
                "languageId": "cursed",
                "version": 1,
                "text": """sus greeting tea = "Hello, CURSED!"
slay main() {
    vibez.spill(greeting)
    sus number drip = mathz.abs_normie(-42)
    stringz.
}"""
            }
        }
    }
    
    return json.dumps(notification, separators=(',', ':'))

def create_completion_request():
    """Create textDocument/completion request"""
    request = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/completion", 
        "params": {
            "textDocument": {
                "uri": "file:///tmp/test.csd"
            },
            "position": {
                "line": 4,
                "character": 12  # After "stringz."
            }
        }
    }
    
    return json.dumps(request, separators=(',', ':'))

def create_hover_request():
    """Create textDocument/hover request"""
    request = {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "textDocument/hover",
        "params": {
            "textDocument": {
                "uri": "file:///tmp/test.csd"
            },
            "position": {
                "line": 0,
                "character": 4  # Over "greeting"
            }
        }
    }
    
    return json.dumps(request, separators=(',', ':'))

def create_formatting_request():
    """Create textDocument/formatting request"""
    request = {
        "jsonrpc": "2.0",
        "id": 4,
        "method": "textDocument/formatting",
        "params": {
            "textDocument": {
                "uri": "file:///tmp/test.csd"
            },
            "options": {
                "tabSize": 4,
                "insertSpaces": True
            }
        }
    }
    
    return json.dumps(request, separators=(',', ':'))

def create_shutdown_request():
    """Create shutdown request"""
    request = {
        "jsonrpc": "2.0",
        "id": 5,
        "method": "shutdown",
        "params": None
    }
    
    return json.dumps(request, separators=(',', ':'))

def test_lsp_server():
    """Test CURSED LSP server with manual messages"""
    
    print("=== CURSED LSP Server Test ===")
    print()
    
    # Start LSP server process (would need actual implementation)
    print("1. Initialize Request:")
    init_msg = create_initialize_request()
    print("Content-Length:", len(init_msg.encode('utf-8')))
    print("Message:", init_msg)
    print()
    
    print("2. Initialized Notification:")
    initialized_msg = create_initialized_notification()
    print("Content-Length:", len(initialized_msg.encode('utf-8')))
    print("Message:", initialized_msg)
    print()
    
    print("3. Did Open Notification:")
    did_open_msg = create_did_open_notification()
    print("Content-Length:", len(did_open_msg.encode('utf-8')))
    print("Message:", did_open_msg)
    print()
    
    print("4. Completion Request:")
    completion_msg = create_completion_request()
    print("Content-Length:", len(completion_msg.encode('utf-8')))
    print("Message:", completion_msg)
    print()
    
    print("5. Hover Request:")
    hover_msg = create_hover_request()
    print("Content-Length:", len(hover_msg.encode('utf-8')))
    print("Message:", hover_msg)
    print()
    
    print("6. Formatting Request:")
    formatting_msg = create_formatting_request()
    print("Content-Length:", len(formatting_msg.encode('utf-8')))
    print("Message:", formatting_msg)
    print()
    
    print("7. Shutdown Request:")
    shutdown_msg = create_shutdown_request()
    print("Content-Length:", len(shutdown_msg.encode('utf-8')))
    print("Message:", shutdown_msg)
    print()

def generate_test_files():
    """Generate test files for LSP validation"""
    
    # Test CURSED file
    test_cursed_content = '''//! Test CURSED file for LSP validation
yeet "vibez"
yeet "mathz"
yeet "stringz"

sus global_greeting tea = "Hello from CURSED LSP!"

slay calculate_something(x drip, y drip) drip {
    sus result drip = mathz.add_two(x, y)
    damn result
}

slay main() {
    vibez.spill(global_greeting)
    
    sus number1 drip = 42
    sus number2 drip = mathz.abs_normie(-10)
    
    sus total drip = calculate_something(number1, number2)
    vibez.spill("Total:", total)
    
    sus message tea = stringz.concat_strings("Result: ", stringz.int_to_string(total))
    vibez.spillln(message)
}
'''
    
    with open('/tmp/test_lsp.csd', 'w') as f:
        f.write(test_cursed_content)
    
    print("Generated test file: /tmp/test_lsp.csd")
    
    # Generate LSP configuration for VS Code
    vscode_settings = {
        "cursed.lsp.enable": True,
        "cursed.lsp.server.command": "./zig-out/bin/cursed-zig",
        "cursed.lsp.server.args": ["cursed_lsp_server.csd"],
        "cursed.lsp.trace.server": "verbose"
    }
    
    with open('/tmp/cursed_lsp_settings.json', 'w') as f:
        json.dump(vscode_settings, f, indent=2)
    
    print("Generated LSP settings: /tmp/cursed_lsp_settings.json")

def validate_json_rpc_messages():
    """Validate that all JSON-RPC messages are properly formatted"""
    
    print("=== JSON-RPC Message Validation ===")
    
    messages = {
        "initialize": create_initialize_request(),
        "initialized": create_initialized_notification(),  
        "didOpen": create_did_open_notification(),
        "completion": create_completion_request(),
        "hover": create_hover_request(),
        "formatting": create_formatting_request(),
        "shutdown": create_shutdown_request()
    }
    
    for name, message in messages.items():
        try:
            parsed = json.loads(message)
            
            # Validate JSON-RPC 2.0 structure
            assert parsed.get("jsonrpc") == "2.0", f"{name}: Missing or invalid jsonrpc field"
            assert "method" in parsed, f"{name}: Missing method field"
            
            if "id" in parsed:
                print(f"✓ {name}: Valid JSON-RPC request")
            else:
                print(f"✓ {name}: Valid JSON-RPC notification")
                
        except Exception as e:
            print(f"✗ {name}: Invalid JSON-RPC - {e}")
    
    print()

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--validate":
        validate_json_rpc_messages()
    elif len(sys.argv) > 1 and sys.argv[1] == "--generate":
        generate_test_files()
    else:
        test_lsp_server()
        print()
        print("To validate JSON-RPC messages: python3 test_lsp_messages.py --validate")
        print("To generate test files: python3 test_lsp_messages.py --generate")
