#!/usr/bin/env python3
"""
LSP Performance Test for CURSED Language Server
Tests completion latency (<50ms) and diagnostics latency (<200ms) requirements
"""

import subprocess
import json
import time
import sys
import os

class LSPTester:
    def __init__(self, server_path):
        self.server_path = server_path
        self.process = None
        self.msg_id = 0
        
    def start_server(self):
        """Start the LSP server process"""
        self.process = subprocess.Popen(
            [self.server_path, '--stdio'],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=0
        )
        
    def send_message(self, method, params=None):
        """Send LSP message and measure response time"""
        self.msg_id += 1
        message = {
            "jsonrpc": "2.0",
            "id": self.msg_id,
            "method": method,
            "params": params or {}
        }
        
        content = json.dumps(message)
        header = f"Content-Length: {len(content)}\r\n\r\n"
        full_message = header + content
        
        start_time = time.time()
        self.process.stdin.write(full_message)
        self.process.stdin.flush()
        
        # Read response (simplified - in real implementation would parse properly)
        try:
            # Read header
            while True:
                line = self.process.stdout.readline()
                if line.strip() == "":
                    break
                    
            # Read content (approximate)
            response_data = self.process.stdout.read(1024)
            end_time = time.time()
            
            latency = (end_time - start_time) * 1000  # Convert to milliseconds
            return latency, response_data
            
        except Exception as e:
            end_time = time.time()
            latency = (end_time - start_time) * 1000
            return latency, f"Error: {e}"
    
    def test_initialization(self):
        """Test LSP initialization"""
        print("Testing LSP initialization...")
        latency, response = self.send_message("initialize", {
            "processId": os.getpid(),
            "rootUri": f"file://{os.getcwd()}",
            "capabilities": {
                "textDocument": {
                    "completion": {"completionItem": {"snippetSupport": True}},
                    "hover": {"contentFormat": ["markdown", "plaintext"]},
                    "definition": {"linkSupport": True}
                }
            }
        })
        print(f"Initialization latency: {latency:.2f}ms")
        return latency < 200
        
    def test_completion(self):
        """Test code completion performance"""
        print("Testing code completion...")
        latency, response = self.send_message("textDocument/completion", {
            "textDocument": {"uri": "file:///test.csd"},
            "position": {"line": 5, "character": 10}
        })
        print(f"Completion latency: {latency:.2f}ms")
        return latency < 50
        
    def test_diagnostics(self):
        """Test diagnostics performance"""  
        print("Testing diagnostics...")
        latency, response = self.send_message("textDocument/didOpen", {
            "textDocument": {
                "uri": "file:///test.csd",
                "languageId": "cursed",
                "version": 1,
                "text": open("lsp_performance_test.csd").read()
            }
        })
        print(f"Diagnostics latency: {latency:.2f}ms")
        return latency < 200
        
    def cleanup(self):
        """Cleanup the LSP server process"""
        if self.process:
            self.process.terminate()
            self.process.wait()

def main():
    server_path = "./zig-out/bin/cursed-lsp"
    
    if not os.path.exists(server_path):
        print(f"LSP server not found at {server_path}")
        return 1
        
    print("CURSED LSP Performance Test")
    print("=" * 40)
    
    tester = LSPTester(server_path)
    
    try:
        tester.start_server()
        time.sleep(1)  # Give server time to start
        
        # Run performance tests
        init_pass = tester.test_initialization()
        completion_pass = tester.test_completion() 
        diagnostics_pass = tester.test_diagnostics()
        
        print("\nPerformance Test Results:")
        print(f"Initialization: {'PASS' if init_pass else 'FAIL'}")
        print(f"Completion (<50ms): {'PASS' if completion_pass else 'FAIL'}")  
        print(f"Diagnostics (<200ms): {'PASS' if diagnostics_pass else 'FAIL'}")
        
        overall_pass = init_pass and completion_pass and diagnostics_pass
        print(f"\nOverall: {'PASS' if overall_pass else 'FAIL'}")
        
        return 0 if overall_pass else 1
        
    except Exception as e:
        print(f"Test failed with error: {e}")
        return 1
        
    finally:
        tester.cleanup()

if __name__ == "__main__":
    sys.exit(main())
