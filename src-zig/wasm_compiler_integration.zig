// Integration of WASM backend with main CURSED compiler
// Adds --backend wasm and --target wasm32-* support to the compiler CLI
const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const WasmBackend = @import("wasm_backend.zig");
const Parser = @import("parser.zig");
const Lexer = @import("lexer.zig");

// WASM compilation configuration
pub const WasmCompilerConfig = struct {
    input_file: []const u8,
    output_file: ?[]const u8 = null,
    target: WasmBackend.WasmTarget = .browser,
    optimize: bool = true,
    debug: bool = false,
    enable_simd: bool = false,
    enable_threads: bool = false,
    memory_pages: u32 = 256,
    export_all: bool = false,
    generate_js_bindings: bool = true,
    generate_html: bool = false,
    verbose: bool = false,
};

// WASM compiler implementation
pub const WasmCompiler = struct {
    allocator: Allocator,
    config: WasmCompilerConfig,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, config: WasmCompilerConfig) Self {
        return Self{
            .allocator = allocator,
            .config = config,
        };
    }
    
    pub fn compile(self: *Self) !void {
        if (self.config.verbose) {
            print("🚀 CURSED WASM Compiler\n");
            print("Target: {s}\n", .{@tagName(self.config.target)});
            print("Input: {s}\n", .{self.config.input_file});
        }
        
        // Read source file
        const source = try self.readSourceFile();
        defer self.allocator.free(source);
        
        // Configure WASM backend options
        const wasm_options = WasmBackend.WasmOptions{
            .target = self.config.target,
            .optimize_size = self.config.optimize,
            .enable_simd = self.config.enable_simd,
            .enable_threads = self.config.enable_threads,
            .enable_gc = true,
            .enable_js_interop = (self.config.target == .browser),
            .memory_pages = self.config.memory_pages,
            .export_all = self.config.export_all,
            .debug_info = self.config.debug,
        };
        
        // Compile to WASM
        const wasm_binary = switch (self.config.target) {
            .browser => try WasmBackend.compileToBrowser(self.allocator, source),
            .wasi => try WasmBackend.compileToWasi(self.allocator, source),
            .freestanding => try WasmBackend.compileToFreestanding(self.allocator, source),
        };
        defer self.allocator.free(wasm_binary);
        
        // Determine output file
        const output_file = self.config.output_file orelse try self.getDefaultOutputFile();
        defer if (self.config.output_file == null) self.allocator.free(output_file);
        
        // Write WASM binary
        try self.writeWasmFile(output_file, wasm_binary);
        
        // Generate additional files based on target
        switch (self.config.target) {
            .browser => {
                if (self.config.generate_js_bindings) {
                    try self.generateJavaScriptBindings(output_file);
                }
                if (self.config.generate_html) {
                    try self.generateHTMLWrapper(output_file);
                }
            },
            .wasi => {
                if (self.config.verbose) {
                    print("✅ Generated WASI-compatible WASM module\n");
                    print("Run with: wasmtime {s}\n", .{output_file});
                }
            },
            .freestanding => {
                if (self.config.verbose) {
                    print("✅ Generated freestanding WASM module\n");
                }
            },
        }
        
        if (self.config.verbose) {
            print("✅ Compilation successful: {s}\n", .{output_file});
            print("📊 Binary size: {} bytes\n", .{wasm_binary.len});
        }
    }
    
    fn readSourceFile(self: *Self) ![]u8 {
        const file = std.fs.cwd().openFile(self.config.input_file, .{}) catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ Error: File '{s}' not found\n", .{self.config.input_file});
                return err;
            },
            else => return err,
        };
        defer file.close();
        
        const file_size = try file.getEndPos();
        const contents = try self.allocator.alloc(u8, file_size);
        _ = try file.readAll(contents);
        
        return contents;
    }
    
    fn getDefaultOutputFile(self: *Self) ![]u8 {
        const input_basename = std.fs.path.basename(self.config.input_file);
        const name_without_ext = if (std.mem.lastIndexOf(u8, input_basename, ".")) |dot_index|
            input_basename[0..dot_index]
        else
            input_basename;
            
        const extension = switch (self.config.target) {
            .browser, .wasi, .freestanding => ".wasm",
        };
        
        return std.fmt.allocPrint(self.allocator, "{s}{s}", .{ name_without_ext, extension });
    }
    
    fn writeWasmFile(self: *Self, filename: []const u8, wasm_binary: []const u8) !void {
        const file = try std.fs.cwd().createFile(filename, .{});
        defer file.close();
        
        try file.writeAll(wasm_binary);
        
        if (self.config.verbose) {
            print("📝 Wrote WASM binary: {s}\n", .{filename});
        }
    }
    
    fn generateJavaScriptBindings(self: *Self, wasm_file: []const u8) !void {
        const js_file = try std.fmt.allocPrint(self.allocator, "{s}.js", .{wasm_file[0..wasm_file.len-5]});
        defer self.allocator.free(js_file);
        
        const js_code = 
            \\// CURSED WebAssembly JavaScript Bindings
            \\// Auto-generated by CURSED WASM compiler
            \\
            \\class CursedModule {
            \\    constructor() {
            \\        this.instance = null;
            \\        this.memory = null;
            \\        this.textDecoder = new TextDecoder();
            \\        this.textEncoder = new TextEncoder();
            \\    }
            \\    
            \\    async load(wasmPath) {
            \\        try {
            \\            const wasmModule = await WebAssembly.instantiateStreaming(
            \\                fetch(wasmPath), 
            \\                this.getImports()
            \\            );
            \\            
            \\            this.instance = wasmModule.instance;
            \\            this.memory = this.instance.exports.memory;
            \\            
            \\            console.log('✅ CURSED WASM module loaded successfully');
            \\            return this;
            \\        } catch (error) {
            \\            console.error('❌ Failed to load CURSED WASM module:', error);
            \\            throw error;
            \\        }
            \\    }
            \\    
            \\    getImports() {
            \\        return {
            \\            js: {
            \\                console_log: (ptr, len) => {
            \\                    const str = this.readString(ptr, len);
            \\                    console.log(str);
            \\                },
            \\                alert: (ptr, len) => {
            \\                    const str = this.readString(ptr, len);
            \\                    alert(str);
            \\                }
            \\            },
            \\            dom: {
            \\                createElement: (tagPtr, tagLen) => {
            \\                    const tagName = this.readString(tagPtr, tagLen);
            \\                    const element = document.createElement(tagName);
            \\                    return this.storeObject(element);
            \\                },
            \\                appendChild: (parentId, childId) => {
            \\                    const parent = this.getObject(parentId);
            \\                    const child = this.getObject(childId);
            \\                    parent.appendChild(child);
            \\                },
            \\                getElementById: (idPtr, idLen) => {
            \\                    const id = this.readString(idPtr, idLen);
            \\                    const element = document.getElementById(id);
            \\                    return element ? this.storeObject(element) : 0;
            \\                }
            \\            }
            \\        };
            \\    }
            \\    
            \\    readString(ptr, len) {
            \\        const bytes = new Uint8Array(this.memory.buffer, ptr, len);
            \\        return this.textDecoder.decode(bytes);
            \\    }
            \\    
            \\    writeString(str) {
            \\        const bytes = this.textEncoder.encode(str);
            \\        const ptr = this.instance.exports.cursed_wasm_alloc(bytes.length);
            \\        const memory = new Uint8Array(this.memory.buffer, ptr, bytes.length);
            \\        memory.set(bytes);
            \\        return { ptr, len: bytes.length };
            \\    }
            \\    
            \\    // Object storage for DOM elements
            \\    storeObject(obj) {
            \\        if (!this.objectStore) this.objectStore = new Map();
            \\        const id = this.nextObjectId || 1;
            \\        this.nextObjectId = id + 1;
            \\        this.objectStore.set(id, obj);
            \\        return id;
            \\    }
            \\    
            \\    getObject(id) {
            \\        return this.objectStore?.get(id) || null;
            \\    }
            \\    
            \\    // High-level API for CURSED functions
            \\    main() {
            \\        if (this.instance?.exports.main) {
            \\            return this.instance.exports.main();
            \\        }
            \\        throw new Error('main function not found in WASM module');
            \\    }
            \\    
            \\    // Call any exported CURSED function
            \\    call(functionName, ...args) {
            \\        const func = this.instance?.exports[functionName];
            \\        if (!func) {
            \\            throw new Error(`Function '${functionName}' not found in WASM module`);
            \\        }
            \\        return func(...args);
            \\    }
            \\    
            \\    // Get all available exports
            \\    getExports() {
            \\        return Object.keys(this.instance?.exports || {});
            \\    }
            \\}
            \\
            \\// Global convenience functions
            \\window.CursedModule = CursedModule;
            \\
            \\// Auto-load if script is included directly
            \\if (typeof window !== 'undefined') {
            \\    window.loadCursed = async function(wasmPath) {
            \\        const module = new CursedModule();
            \\        await module.load(wasmPath);
            \\        return module;
            \\    };
            \\}
            \\
            \\// Node.js support
            \\if (typeof module !== 'undefined') {
            \\    module.exports = CursedModule;
            \\}
            \\
        ;
        
        const file = try std.fs.cwd().createFile(js_file, .{});
        defer file.close();
        
        try file.writeAll(js_code);
        
        if (self.config.verbose) {
            print("📝 Generated JavaScript bindings: {s}\n", .{js_file});
        }
    }
    
    fn generateHTMLWrapper(self: *Self, wasm_file: []const u8) !void {
        const html_file = try std.fmt.allocPrint(self.allocator, "{s}.html", .{wasm_file[0..wasm_file.len-5]});
        defer self.allocator.free(html_file);
        
        const basename = std.fs.path.basename(wasm_file);
        const js_basename = try std.fmt.allocPrint(self.allocator, "{s}.js", .{basename[0..basename.len-5]});
        defer self.allocator.free(js_basename);
        
        const html_code = try std.fmt.allocPrint(self.allocator,
            \\<!DOCTYPE html>
            \\<html lang="en">
            \\<head>
            \\    <meta charset="UTF-8">
            \\    <meta name="viewport" content="width=device-width, initial-scale=1.0">
            \\    <title>CURSED WebAssembly Application</title>
            \\    <style>
            \\        body {{
            \\            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            \\            max-width: 800px;
            \\            margin: 0 auto;
            \\            padding: 20px;
            \\            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            \\            color: white;
            \\            min-height: 100vh;
            \\        }}
            \\        .container {{
            \\            background: rgba(255, 255, 255, 0.1);
            \\            padding: 30px;
            \\            border-radius: 15px;
            \\            backdrop-filter: blur(10px);
            \\            box-shadow: 0 8px 32px rgba(31, 38, 135, 0.37);
            \\        }}
            \\        h1 {{
            \\            text-align: center;
            \\            margin-bottom: 30px;
            \\            font-size: 2.5em;
            \\            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
            \\        }}
            \\        .status {{
            \\            padding: 15px;
            \\            margin: 20px 0;
            \\            border-radius: 8px;
            \\            border-left: 4px solid #00ff88;
            \\            background: rgba(0, 255, 136, 0.1);
            \\        }}
            \\        .error {{
            \\            border-left-color: #ff4757;
            \\            background: rgba(255, 71, 87, 0.1);
            \\        }}
            \\        .controls {{
            \\            text-align: center;
            \\            margin: 30px 0;
            \\        }}
            \\        button {{
            \\            background: linear-gradient(45deg, #ff6b6b, #ee5a24);
            \\            color: white;
            \\            border: none;
            \\            padding: 12px 30px;
            \\            font-size: 16px;
            \\            border-radius: 25px;
            \\            cursor: pointer;
            \\            margin: 10px;
            \\            transition: transform 0.2s, box-shadow 0.2s;
            \\            box-shadow: 0 4px 15px rgba(0,0,0,0.2);
            \\        }}
            \\        button:hover {{
            \\            transform: translateY(-2px);
            \\            box-shadow: 0 6px 20px rgba(0,0,0,0.3);
            \\        }}
            \\        button:active {{
            \\            transform: translateY(0);
            \\        }}
            \\        .output {{
            \\            background: rgba(0, 0, 0, 0.3);
            \\            padding: 20px;
            \\            border-radius: 8px;
            \\            margin-top: 20px;
            \\            font-family: 'Courier New', monospace;
            \\            white-space: pre-wrap;
            \\            max-height: 400px;
            \\            overflow-y: auto;
            \\            border: 1px solid rgba(255, 255, 255, 0.2);
            \\        }}
            \\        .footer {{
            \\            text-align: center;
            \\            margin-top: 40px;
            \\            opacity: 0.8;
            \\            font-size: 0.9em;
            \\        }}
            \\    </style>
            \\</head>
            \\<body>
            \\    <div class="container">
            \\        <h1>🚀 CURSED WebAssembly</h1>
            \\        
            \\        <div id="status" class="status">
            \\            <p>Loading CURSED WASM application...</p>
            \\        </div>
            \\        
            \\        <div class="controls" id="controls" style="display: none;">
            \\            <button onclick="runMain()">Run Main Function</button>
            \\            <button onclick="showExports()">Show Exports</button>
            \\            <button onclick="clearOutput()">Clear Output</button>
            \\        </div>
            \\        
            \\        <div id="output" class="output" style="display: none;"></div>
            \\        
            \\        <div class="footer">
            \\            <p>Powered by CURSED Programming Language</p>
            \\            <p><em>"Write code so cursed, it becomes blessed"</em></p>
            \\        </div>
            \\    </div>
            \\    
            \\    <script src="{s}"></script>
            \\    <script>
            \\        let cursedModule;
            \\        let output;
            \\        
            \\        // Override console.log to capture output
            \\        const originalConsoleLog = console.log;
            \\        console.log = function(...args) {{
            \\            originalConsoleLog(...args);
            \\            if (output) {{
            \\                output.textContent += args.join(' ') + '\n';
            \\                output.scrollTop = output.scrollHeight;
            \\            }}
            \\        }};
            \\        
            \\        async function loadWasm() {{
            \\            try {{
            \\                cursedModule = await loadCursed('{s}');
            \\                
            \\                document.getElementById('status').innerHTML = 
            \\                    '<p style="color: #00ff88;">✅ CURSED WASM module loaded successfully!</p>';
            \\                document.getElementById('controls').style.display = 'block';
            \\                document.getElementById('output').style.display = 'block';
            \\                
            \\                output = document.getElementById('output');
            \\                console.log('CURSED WASM module ready!');
            \\                console.log('Available exports:', cursedModule.getExports().join(', '));
            \\                
            \\            }} catch (error) {{
            \\                document.getElementById('status').innerHTML = 
            \\                    `<p style="color: #ff4757;">❌ Failed to load WASM: ${{error.message}}</p>`;
            \\                document.getElementById('status').classList.add('error');
            \\                console.error(error);
            \\            }}
            \\        }}
            \\        
            \\        function runMain() {{
            \\            try {{
            \\                console.log('=== Running main() ===');
            \\                const result = cursedModule.main();
            \\                console.log('Main function returned:', result);
            \\            }} catch (error) {{
            \\                console.error('Error running main:', error.message);
            \\            }}
            \\        }}
            \\        
            \\        function showExports() {{
            \\            console.log('=== Available Exports ===');
            \\            const exports = cursedModule.getExports();
            \\            exports.forEach(exportName => {{
            \\                console.log(`- ${{exportName}}`);
            \\            }});
            \\        }}
            \\        
            \\        function clearOutput() {{
            \\            output.textContent = '';
            \\        }}
            \\        
            \\        // Load WASM module on page load
            \\        window.addEventListener('load', loadWasm);
            \\    </script>
            \\</body>
            \\</html>
            \\
        , .{ js_basename, basename });
        defer self.allocator.free(html_code);
        
        const file = try std.fs.cwd().createFile(html_file, .{});
        defer file.close();
        
        try file.writeAll(html_code);
        
        if (self.config.verbose) {
            print("📝 Generated HTML wrapper: {s}\n", .{html_file});
            print("🌐 Open in browser: file://{s}/{s}\n", .{try std.fs.realpathAlloc(self.allocator, std.fs.cwd(), "."), html_file});
        }
    }
};

// CLI argument parsing for WASM compilation
pub fn parseWasmArgs(allocator: Allocator, args: []const []const u8) !?WasmCompilerConfig {
    if (args.len < 2) return null;
    
    var config = WasmCompilerConfig{
        .input_file = "",
    };
    
    var i: usize = 1;
    var found_wasm = false;
    
    while (i < args.len) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--backend") and i + 1 < args.len) {
            if (std.mem.eql(u8, args[i + 1], "wasm")) {
                found_wasm = true;
            }
            i += 2;
        } else if (std.mem.startsWith(u8, arg, "--target=wasm") or 
                   std.mem.eql(u8, arg, "--target") and i + 1 < args.len and 
                   std.mem.startsWith(u8, args[i + 1], "wasm")) {
            found_wasm = true;
            
            // Parse specific WASM target
            const target_str = if (std.mem.startsWith(u8, arg, "--target=")) 
                arg[9..] 
            else if (i + 1 < args.len) 
                args[i + 1]
            else 
                "wasm32-browser";
                
            if (std.mem.eql(u8, target_str, "wasm32-wasi") or std.mem.eql(u8, target_str, "wasi")) {
                config.target = .wasi;
            } else if (std.mem.eql(u8, target_str, "wasm32-freestanding") or std.mem.eql(u8, target_str, "freestanding")) {
                config.target = .freestanding;
            } else {
                config.target = .browser;
            }
            
            i += if (std.mem.startsWith(u8, arg, "--target=")) 1 else 2;
        } else if (std.mem.eql(u8, arg, "-o") or std.mem.eql(u8, arg, "--output")) {
            if (i + 1 < args.len) {
                config.output_file = args[i + 1];
                i += 2;
            } else {
                print("Error: {s} requires an argument\n", .{arg});
                return error.InvalidArgument;
            }
        } else if (std.mem.eql(u8, arg, "--optimize") or std.mem.eql(u8, arg, "-O")) {
            config.optimize = true;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--debug") or std.mem.eql(u8, arg, "-g")) {
            config.debug = true;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--enable-simd")) {
            config.enable_simd = true;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--enable-threads")) {
            config.enable_threads = true;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--export-all")) {
            config.export_all = true;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--no-js-bindings")) {
            config.generate_js_bindings = false;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--generate-html")) {
            config.generate_html = true;
            i += 1;
        } else if (std.mem.eql(u8, arg, "--verbose") or std.mem.eql(u8, arg, "-v")) {
            config.verbose = true;
            i += 1;
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            // Input file
            if (config.input_file.len == 0) {
                config.input_file = arg;
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    
    if (!found_wasm) return null;
    
    if (config.input_file.len == 0) {
        print("Error: No input file specified\n");
        return error.MissingInputFile;
    }
    
    return config;
}

// Main WASM compilation function
pub fn compileWasm(allocator: Allocator, config: WasmCompilerConfig) !void {
    var compiler = WasmCompiler.init(allocator, config);
    try compiler.compile();
}

// Help text for WASM compilation
pub fn printWasmHelp() void {
    print(
        \\CURSED WebAssembly Compilation:
        \\
        \\Usage:
        \\  cursed-zig --backend wasm [options] <input.csd>
        \\  cursed-zig --target wasm32-<target> [options] <input.csd>
        \\
        \\WASM Targets:
        \\  wasm32-browser      Browser-compatible WASM with JS interop (default)
        \\  wasm32-wasi         WASI-compatible for serverless/CLI applications
        \\  wasm32-freestanding Minimal WASM with no host dependencies
        \\
        \\Options:
        \\  -o, --output FILE       Output WASM file
        \\  -O, --optimize          Enable optimizations (default: true)
        \\  -g, --debug             Include debug information
        \\  --enable-simd           Enable SIMD instructions
        \\  --enable-threads        Enable threading support
        \\  --export-all            Export all functions
        \\  --no-js-bindings        Don't generate JavaScript bindings
        \\  --generate-html         Generate HTML wrapper for browser target
        \\  -v, --verbose           Verbose output
        \\
        \\Examples:
        \\  # Compile for browser
        \\  cursed-zig --backend wasm --generate-html hello.csd
        \\
        \\  # Compile for WASI
        \\  cursed-zig --target wasm32-wasi server.csd
        \\
        \\  # Optimized browser build
        \\  cursed-zig --target wasm32-browser -O --enable-simd app.csd
        \\
    );
}
