#!/usr/bin/env node
/**
 * LSP Performance Test Client for CURSED Language Server
 * Tests completion latency (<50ms) and diagnostics latency (<200ms) requirements
 */

const { spawn } = require('child_process');
const fs = require('fs');

class LSPTestClient {
    constructor(serverPath) {
        this.serverPath = serverPath;
        this.msgId = 0;
    }

    async testLSPPerformance() {
        console.log('🚀 CURSED LSP Performance Test');
        console.log('===============================');
        
        // Start LSP server
        const server = spawn(this.serverPath, ['--stdio'], {
            stdio: ['pipe', 'pipe', 'pipe']
        });
        
        let responseBuffer = '';
        const responses = new Map();
        
        server.stdout.on('data', (data) => {
            responseBuffer += data.toString();
            this.processMessages(responseBuffer, responses);
        });
        
        server.stderr.on('data', (data) => {
            console.error('LSP Error:', data.toString());
        });
        
        // Test initialization
        console.log('Testing LSP initialization...');
        const initStart = Date.now();
        const initId = await this.sendMessage(server, 'initialize', {
            processId: process.pid,
            rootUri: `file://${process.cwd()}`,
            capabilities: {
                textDocument: {
                    completion: { completionItem: { snippetSupport: true } },
                    hover: { contentFormat: ['markdown', 'plaintext'] },
                    definition: { linkSupport: true }
                }
            }
        });
        
        // Wait for response
        await this.waitForResponse(responses, initId, 5000);
        const initLatency = Date.now() - initStart;
        console.log(`✓ Initialization: ${initLatency}ms`);
        
        // Send initialized notification
        await this.sendNotification(server, 'initialized', {});
        
        // Test document open and diagnostics
        console.log('Testing document open and diagnostics...');
        const diagStart = Date.now();
        await this.sendNotification(server, 'textDocument/didOpen', {
            textDocument: {
                uri: 'file:///test.csd',
                languageId: 'cursed',
                version: 1,
                text: fs.readFileSync('./lsp_performance_test.csd', 'utf8')
            }
        });
        const diagLatency = Date.now() - diagStart;
        console.log(`✓ Diagnostics: ${diagLatency}ms`);
        
        // Test completion
        console.log('Testing code completion...');
        const completionStart = Date.now();
        const completionId = await this.sendMessage(server, 'textDocument/completion', {
            textDocument: { uri: 'file:///test.csd' },
            position: { line: 5, character: 10 }
        });
        
        await this.waitForResponse(responses, completionId, 1000);
        const completionLatency = Date.now() - completionStart;
        console.log(`✓ Completion: ${completionLatency}ms`);
        
        // Test results
        const results = {
            initialization: { latency: initLatency, pass: initLatency < 2000 },
            diagnostics: { latency: diagLatency, pass: diagLatency < 200 },
            completion: { latency: completionLatency, pass: completionLatency < 50 }
        };
        
        console.log('\nPerformance Test Results:');
        console.log(`Initialization (<2000ms): ${results.initialization.pass ? 'PASS' : 'FAIL'} (${initLatency}ms)`);
        console.log(`Diagnostics (<200ms): ${results.diagnostics.pass ? 'PASS' : 'FAIL'} (${diagLatency}ms)`);  
        console.log(`Completion (<50ms): ${results.completion.pass ? 'PASS' : 'FAIL'} (${completionLatency}ms)`);
        
        const allPass = results.initialization.pass && results.diagnostics.pass && results.completion.pass;
        console.log(`\nOverall: ${allPass ? 'PASS' : 'FAIL'} ✨`);
        
        // Cleanup
        await this.sendMessage(server, 'shutdown', {});
        await this.sendNotification(server, 'exit', {});
        server.kill();
        
        return allPass;
    }
    
    async sendMessage(server, method, params) {
        this.msgId++;
        const message = {
            jsonrpc: '2.0',
            id: this.msgId,
            method,
            params: params || {}
        };
        
        const content = JSON.stringify(message);
        const header = `Content-Length: ${content.length}\r\n\r\n`;
        const fullMessage = header + content;
        
        server.stdin.write(fullMessage);
        return this.msgId;
    }
    
    async sendNotification(server, method, params) {
        const message = {
            jsonrpc: '2.0',
            method,
            params: params || {}
        };
        
        const content = JSON.stringify(message);
        const header = `Content-Length: ${content.length}\r\n\r\n`;
        const fullMessage = header + content;
        
        server.stdin.write(fullMessage);
    }
    
    processMessages(buffer, responses) {
        let remaining = buffer;
        
        while (remaining.includes('\r\n\r\n')) {
            const headerEnd = remaining.indexOf('\r\n\r\n');
            const headers = remaining.substring(0, headerEnd);
            const contentLengthMatch = headers.match(/Content-Length: (\d+)/);
            
            if (!contentLengthMatch) break;
            
            const contentLength = parseInt(contentLengthMatch[1]);
            const messageStart = headerEnd + 4;
            
            if (remaining.length < messageStart + contentLength) break;
            
            const content = remaining.substring(messageStart, messageStart + contentLength);
            remaining = remaining.substring(messageStart + contentLength);
            
            try {
                const message = JSON.parse(content);
                if (message.id) {
                    responses.set(message.id, message);
                }
            } catch (e) {
                console.error('Failed to parse LSP message:', e);
            }
        }
    }
    
    async waitForResponse(responses, id, timeout = 5000) {
        const start = Date.now();
        
        while (Date.now() - start < timeout) {
            if (responses.has(id)) {
                return responses.get(id);
            }
            await new Promise(resolve => setTimeout(resolve, 10));
        }
        
        throw new Error(`Timeout waiting for response ${id}`);
    }
}

// Run the test
async function main() {
    const serverPath = './zig-out/bin/cursed-lsp';
    const client = new LSPTestClient(serverPath);
    
    try {
        const success = await client.testLSPPerformance();
        process.exit(success ? 0 : 1);
    } catch (error) {
        console.error('Test failed:', error.message);
        process.exit(1);
    }
}

if (require.main === module) {
    main();
}
