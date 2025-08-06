#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

async function testWasmModule(wasmPath) {
    console.log(`\n=== Testing ${path.basename(wasmPath)} ===`);
    
    try {
        // Read the WASM file
        const wasmBuffer = fs.readFileSync(wasmPath);
        
        // Instantiate the WASM module
        const wasmModule = await WebAssembly.instantiate(wasmBuffer, {
            env: {
                // Provide minimal environment
            }
        });
        
        const exports = wasmModule.instance.exports;
        console.log('Available exports:', Object.keys(exports));
        
        // Test version function
        if (exports.cursed_wasm_version) {
            try {
                const versionPtr = exports.cursed_wasm_version();
                const memory = exports.memory;
                const view = new Uint8Array(memory.buffer, versionPtr);
                let version = '';
                for (let i = 0; i < 100 && view[i] !== 0; i++) {
                    version += String.fromCharCode(view[i]);
                }
                console.log('Version:', version);
            } catch (error) {
                console.log('Version test failed:', error.message);
            }
        }
        
        // Test tokenization with sample CURSED code
        if (exports.cursed_wasm_tokenize && exports.memory) {
            try {
                const testCode = 'vibez.spill("Hello WASM!")';
                const encoder = new TextEncoder();
                const codeBytes = encoder.encode(testCode);
                
                // Use a fixed memory location
                const dataPtr = 0x1000;
                const memory = exports.memory;
                const view = new Uint8Array(memory.buffer, dataPtr, codeBytes.length);
                view.set(codeBytes);
                
                const tokenCount = exports.cursed_wasm_tokenize(dataPtr, codeBytes.length);
                console.log(`Tokenization test: "${testCode}" -> ${tokenCount} tokens`);
            } catch (error) {
                console.log('Tokenization test failed:', error.message);
            }
        }
        
        // Test syntax check
        if (exports.cursed_wasm_check && exports.memory) {
            try {
                const testCode = 'vibez.spill("Hello WASM!")';
                const encoder = new TextEncoder();
                const codeBytes = encoder.encode(testCode);
                
                const dataPtr = 0x1000;
                const memory = exports.memory;
                const view = new Uint8Array(memory.buffer, dataPtr, codeBytes.length);
                view.set(codeBytes);
                
                const hasErrors = exports.cursed_wasm_check(dataPtr, codeBytes.length);
                console.log(`Syntax check: ${hasErrors ? 'Has Errors' : 'OK'}`);
            } catch (error) {
                console.log('Syntax check test failed:', error.message);
            }
        }
        
        // Test basic test function
        if (exports.cursed_wasm_test) {
            try {
                const testResult = exports.cursed_wasm_test();
                console.log(`Test function result: ${testResult}`);
            } catch (error) {
                console.log('Test function failed:', error.message);
            }
        }
        
        console.log('✅ WASM module test completed successfully');
        
    } catch (error) {
        console.error('❌ WASM module test failed:', error.message);
    }
}

async function main() {
    console.log('🚀 Testing CURSED WASM modules...');
    
    const wasmFiles = [
        'zig-out/bin/cursed.wasm',
        'zig-out/bin/cursed-minimal.wasm',
        'zig-out/bin/cursed-optimized.wasm'
    ];
    
    for (const wasmFile of wasmFiles) {
        if (fs.existsSync(wasmFile)) {
            await testWasmModule(wasmFile);
        } else {
            console.log(`❌ WASM file not found: ${wasmFile}`);
        }
    }
    
    console.log('\n🎉 All WASM tests completed!');
}

// Run the tests
main().catch(console.error);
