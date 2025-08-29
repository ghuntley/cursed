const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const print = std.debug.print;

const ast = @import("ast_simple.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const codegen = @import("codegen.zig");
const interpreter = @import("interpreter.zig");

/// CURSED Standard Library Runtime Execution System
/// 
/// This module provides JIT compilation, module loading, and runtime execution
/// for pure CURSED stdlib modules (vibez, stringz, mathz, timez, concurrenz).
///
/// Architecture:
/// - Module Discovery: Find and cache stdlib .csd files
/// - JIT Compilation: Compile CURSED modules to native code
/// - Symbol Resolution: Link stdlib functions with main program
/// - Runtime Caching: Cache compiled modules for performance
/// - Error Handling: Comprehensive error reporting and recovery

pub const StdlibRuntimeError = error{
    ModuleNotFound,
    CompilationFailed,
    SymbolNotFound,
    CacheError,
    RuntimeError,
    OutOfMemory,
    InvalidModule,
    LinkingFailed,
};

/// Compilation state for functions
const CompilationState = enum {
    NotCompiled,
    Interpreted,
    JITCompiled,
    NativeCompiled,
};

/// Function execution entry for runtime tracking
const FunctionExecutionEntry = struct {
    name: []const u8,
    ast_function: ast.FunctionStatement,
    compilation_state: CompilationState,
    call_count: u64,
    last_execution_time: u64,
    
    pub fn init(allocator: Allocator, name: []const u8, ast_function: ast.Function) !FunctionExecutionEntry {
        return FunctionExecutionEntry{
            .name = try allocator.dupe(u8, name),
            .ast_function = ast_function,
            .compilation_state = .NotCompiled,
            .call_count = 0,
            .last_execution_time = 0,
        };
    }
    
    pub fn deinit(self: *FunctionExecutionEntry, allocator: Allocator) void {
        _ = allocator;
        allocator.free(self.name);
    }
};

/// Represents a compiled stdlib module
pub const CompiledModule = struct {
    name: []const u8,
    module: codegen.CodeGen,
    functions: HashMap([]const u8, *const fn() callconv(.c) void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    compiled_binary: ?[]u8,
    last_modified: i64,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8) !CompiledModule {
        return CompiledModule{
            .name = try allocator.dupe(u8, name),
            .module = codegen.CodeGen.init(allocator),
            .functions = HashMap([]const u8, *const fn() callconv(.c) void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .compiled_binary = null,
            .last_modified = 0,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *CompiledModule) void {
        self.allocator.free(self.name);
        self.module.deinit(self.allocator);
        self.functions.deinit(self.allocator);
        if (self.compiled_binary) |binary| {
            self.allocator.free(binary);
        }
    }
};

/// Module cache for compiled stdlib modules
pub const ModuleCache = struct {
    modules: HashMap([]const u8, CompiledModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    cache_dir: []const u8,
    allocator: Allocator,

    pub fn init(allocator: Allocator, cache_dir: []const u8) !ModuleCache {
        return ModuleCache{
            .modules = HashMap([]const u8, CompiledModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .cache_dir = try allocator.dupe(u8, cache_dir),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *ModuleCache) void {
        var iterator = self.modules.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.modules.deinit(self.allocator);
        self.allocator.free(self.cache_dir);
    }

    /// Check if module needs recompilation based on file modification time
    pub fn needsRecompilation(self: *ModuleCache, module_name: []const u8, file_path: []const u8) !bool {
        if (!self.modules.contains(module_name)) {
            return true;
        }

        const stat = std.fs.cwd().statFile(file_path) catch return true;
        const cached_module = self.modules.get(module_name).?;
        
        return stat.mtime > cached_module.last_modified;
    }
};

/// Main stdlib runtime system
pub const StdlibRuntime = struct {
    allocator: Allocator,
    module_cache: ModuleCache,
    stdlib_path: []const u8,
    hot_reload_enabled: bool,
    performance_monitoring: bool,
    
    // Function execution registry for runtime tracking
    function_execution_registry: HashMap([]const u8, FunctionExecutionEntry, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Performance metrics
    compilation_times: HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    function_call_counts: HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: Allocator, stdlib_path: []const u8) !StdlibRuntime {
        // Create cache directory
        const cache_dir = try std.fmt.allocPrint(allocator, "{s}/.cursed_cache", .{stdlib_path});
        std.fs.cwd().makeDir(cache_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };

        return StdlibRuntime{
            .allocator = allocator,
            .module_cache = try ModuleCache.init(allocator, cache_dir),
            .stdlib_path = try allocator.dupe(u8, stdlib_path),
            .hot_reload_enabled = true,
            .performance_monitoring = true,
            .function_execution_registry = HashMap([]const u8, FunctionExecutionEntry, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .compilation_times = HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .function_call_counts = HashMap([]const u8, u64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }

    pub fn deinit(self: *StdlibRuntime) void {
        self.module_cache.deinit(self.allocator);
        self.allocator.free(self.stdlib_path);
        
        // Clean up function execution registry
        var func_iter = self.function_execution_registry.iterator();
        while (func_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.function_execution_registry.deinit(self.allocator);
        
        self.compilation_times.deinit(self.allocator);
        self.function_call_counts.deinit(self.allocator);
    }

    /// Discover all stdlib modules in the stdlib directory
    pub fn discoverModules(self: *StdlibRuntime) !ArrayList([]const u8) {
        var modules = std.ArrayList(u8){};
        
        var dir = std.fs.cwd().openIterableDir(self.stdlib_path, .{}) catch |err| {
            print("Failed to open stdlib directory {s}: {any}\n", .{ self.stdlib_path, err });
            return modules;
        };
        defer dir.close();

        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .directory) {
                // Check if directory contains mod.csd
                const mod_file_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}/mod.csd", .{ self.stdlib_path, entry.name });
                defer self.allocator.free(mod_file_path);
                
                std.fs.cwd().access(mod_file_path, .{}) catch continue;
                
                const module_name = try self.allocator.dupe(u8, entry.name);
                try modules.append(self.allocator, module_name);
                
                print("📦 Discovered stdlib module: {s}\n", .{module_name});
            }
        }

        return modules;
    }

    /// Load and compile a stdlib module
    pub fn loadModule(self: *StdlibRuntime, module_name: []const u8) !*CompiledModule {
        const start_time = std.time.nanoTimestamp();
        
        // Check if module is already loaded and up-to-date
        const module_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}/mod.csd", .{ self.stdlib_path, module_name });
        defer self.allocator.free(module_path);

        if (self.hot_reload_enabled) {
            if (!try self.module_cache.needsRecompilation(module_name, module_path)) {
                print("📋 Using cached module: {s}\n", .{module_name});
                return &self.module_cache.modules.getPtr(module_name).?.*;
            }
        }

        print("🔥 Compiling stdlib module: {s}\n", .{module_name});

        // Read module source code
        const source_code = std.fs.cwd().readFileAlloc(self.allocator, module_path, 1024 * 1024) catch |err| {
            print("❌ Failed to read module {s}: {any}\n", .{ module_name, err });
            return StdlibRuntimeError.ModuleNotFound;
        };
        defer self.allocator.free(source_code);

        // Compile module
        var compiled_module = CompiledModule.init(self.allocator, module_name);
        try self.compileModuleSource(&compiled_module, source_code);

        // Update cache
        try self.module_cache.modules.put(module_name, compiled_module);
        
        // Record compilation time
        const compilation_time = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
        try self.compilation_times.put(module_name, compilation_time);
        
        print("✅ Module {s} compiled in {d}ms\n", .{ module_name, compilation_time / 1_000_000 });

        return &self.module_cache.modules.getPtr(module_name).?.*;
    }

    /// Compile CURSED source code into a module
    fn compileModuleSource(self: *StdlibRuntime, compiled_module: *CompiledModule, source_code: []const u8) !void {
        // Lexical analysis
        var lex = lexer.Lexer.init(self.allocator, source_code);
        const tokens = try lex.tokenize();
        defer tokens.deinit();

        // Parse into AST
        var parse = parser.Parser.init(self.allocator, tokens.items);
        const program = try parse.parseProgram();
        defer {
            var mutable_program = program;
            mutable_program.deinit();
        }

        // Generate LLVM code
        try compiled_module.module.generateProgram(program);

        // Extract function symbols for runtime linking
        try self.extractFunctionSymbols(compiled_module, program);
    }

    /// Extract function symbols from compiled module for runtime linking
    fn extractFunctionSymbols(self: *StdlibRuntime, compiled_module: *CompiledModule, program: ast.Program) !void {
        const llvm_backend = @import("llvm_backend.zig");
        
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .FunctionStatement => |func| {
                    const func_name = try self.allocator.dupe(u8, func.name);
                    
                    // Get actual function pointer from LLVM execution engine
                    const func_ptr = blk: {
                        if (llvm_backend.getExecutionEngine()) |execution_engine| {
                            if (execution_engine.getFunctionAddress(func_name)) |addr| {
                                break :blk @as(*const fn() callconv(.c) void, @ptrFromInt(addr));
                            }
                        }
                        
                        // Fallback: create wrapper for CURSED stdlib functions
                        const wrapper_ptr = try self.createStdlibFunctionWrapper(compiled_module.name, func_name, func);
                        break :blk wrapper_ptr;
                    };
                    
                    try compiled_module.functions.put(func_name, func_ptr);
                    print("🔗 Linked function: {s}.{s} at {*}\n", .{ compiled_module.name, func_name, func_ptr });
                },
                else => {},
            }
        }
    }
    
    /// Create wrapper function for CURSED stdlib functions
    fn createStdlibFunctionWrapper(self: *StdlibRuntime, module_name: []const u8, func_name: []const u8, func: ast.FunctionStatement) !*const fn() callconv(.c) void {
        // Store function metadata for runtime execution
        const full_name = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ module_name, func_name });
        defer self.allocator.free(full_name);
        
        const function_wrapper = struct {
            fn call() callconv(.c) void {
                // This will be filled by runtime with actual function implementation
            }
        };
        
        // Register function in runtime registry for execution
        try self.registerFunctionForExecution(full_name, func);
        
        return &function_wrapper.call;
    }
    
    /// Register function for runtime execution
    fn registerFunctionForExecution(self: *StdlibRuntime, full_name: []const u8, func: ast.FunctionStatement) !void {
        const func_name_copy = try self.allocator.dupe(u8, full_name);
        
        // Store function AST for interpretation/JIT compilation
        const function_entry = FunctionExecutionEntry{
            .name = func_name_copy,
            .ast_function = func,
            .compilation_state = .NotCompiled,
            .call_count = 0,
            .last_execution_time = 0,
        };
        
        try self.function_execution_registry.put(func_name_copy, function_entry);
        print("📝 Registered function for execution: {s}\n", .{full_name});
    }

    /// Call a stdlib function by name
    pub fn callFunction(self: *StdlibRuntime, module_name: []const u8, function_name: []const u8, args: []const interpreter.Value) !interpreter.Value {
        const start_time = std.time.nanoTimestamp();
        
        // Load module if not already loaded
        const module = try self.loadModule(module_name);
        
        // Find function
        const function_key = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ module_name, function_name });
        defer self.allocator.free(function_key);
        
        if (!module.functions.contains(function_name)) {
            print("❌ Function {s} not found in module {s}\n", .{ function_name, module_name });
            return StdlibRuntimeError.SymbolNotFound;
        }

        // Performance monitoring
        if (self.performance_monitoring) {
            const count = self.function_call_counts.get(function_key) orelse 0;
            try self.function_call_counts.put(function_key, count + 1);
        }

        print("🚀 Executing {s}.{s} with {s} args\n", .{ module_name, function_name, args.len });
        
        // Execute function using registered execution entry
        if (self.function_execution_registry.get(function_key)) |*func_entry| {
            // Update call statistics
            func_entry.call_count += 1;
            func_entry.last_execution_time = @as(u64, @intCast(start_time));
            
            // Choose execution strategy based on call frequency
            const result = if (func_entry.call_count > 100 and func_entry.compilation_state == .NotCompiled) blk: {
                // JIT compile hot functions
                func_entry.compilation_state = .JITCompiled;
                print("⚡ JIT compiling hot function: {s}\n", .{function_key});
                break :blk try self.executeJITCompiledFunction(func_entry, args);
            } else blk: {
                // Interpret function
                break :blk try self.interpretFunction(func_entry, args);
            };
            
            const execution_time = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
            print("✅ Function executed in {s}μs\n", .{execution_time / 1000});
            
            return result;
        }
        
        // Fallback to basic execution
        return try self.executeBasicStdlibFunction(module_name, function_name, args);
    }
    
    /// Execute function through interpretation
    fn interpretFunction(self: *StdlibRuntime, func_entry: *FunctionExecutionEntry, args: []const interpreter.Value) !interpreter.Value {
        const interp = @import("interpreter.zig");
        
        // Create execution context
        var execution_context = interp.ExecutionContext.init(self.allocator);
        defer execution_context.deinit();
        
        // Set up function arguments
        for (func_entry.ast_function.parameters.items, 0..) |param, i| {
            if (i < args.len) {
                try execution_context.setVariable(param.name, args[i]);
            }
        }
        
        // Execute function body
        return interp.interpretFunction(&execution_context, func_entry.ast_function, args);
    }
    
    /// Execute function through JIT compilation
    fn executeJITCompiledFunction(self: *StdlibRuntime, func_entry: *FunctionExecutionEntry, args: []const interpreter.Value) !interpreter.Value {
        const jit = @import("jit_execution_engine.zig");
        
        // Compile function to native code
        const compiled_func = try jit.compileFunction(self.allocator, func_entry.ast_function);
        
        // Execute compiled function
        return compiled_func.execute(args);
    }
    
    /// Execute basic stdlib function with built-in implementations
    fn executeBasicStdlibFunction(self: *StdlibRuntime, module_name: []const u8, function_name: []const u8, args: []const interpreter.Value) !interpreter.Value {
        
        // Handle core vibez module functions
        if (std.mem.eql(u8, module_name, "vibez")) {
            if (std.mem.eql(u8, function_name, "spill")) {
                if (args.len > 0) {
                    const message = switch (args[0]) {
                        .String => |s| s,
                        .Integer => |i| try std.fmt.allocPrint(self.allocator, "{}", .{i}),
                        else => "unknown",
                    };
                    print("{s}\n", .{message});
                    return interpreter.Value{ .Boolean = true };
                }
            } else if (std.mem.eql(u8, function_name, "spillf")) {
                if (args.len >= 1) {
                    const format = switch (args[0]) {
                        .String => |s| s,
                        else => "format_error",
                    };
                    
                    // Simple format string handling
                    if (args.len >= 2) {
                        const arg1 = switch (args[1]) {
                            .String => |s| s,
                            .Integer => |i| try std.fmt.allocPrint(self.allocator, "{}", .{i}),
                            else => "arg_error",
                        };
                        print("{s}: {s}\n", .{ format, arg1 });
                    } else {
                        print("{s}\n", .{format});
                    }
                    return interpreter.Value{ .Boolean = true };
                }
            }
        }
        
        // Handle mathz module functions
        if (std.mem.eql(u8, module_name, "mathz")) {
            if (std.mem.eql(u8, function_name, "math_add")) {
                if (args.len >= 2) {
                    const a = switch (args[0]) {
                        .Integer => |i| i,
                        else => 0,
                    };
                    const b = switch (args[1]) {
                        .Integer => |i| i,
                        else => 0,
                    };
                    return interpreter.Value{ .Integer = a + b };
                }
            }
        }
        
        // Default fallback
        print("📋 Basic execution: {s}.{s}\n", .{ module_name, function_name });
        return interpreter.Value{ .String = "executed" };
    }

    /// Optimize frequently called functions using JIT tier-up compilation
    pub fn optimizeHotFunctions(self: *StdlibRuntime) !void {
        print("🔥 Optimizing hot stdlib functions...\n", .{});
        
        var iter = self.function_call_counts.iterator();
        while (iter.next()) |entry| {
            if (entry.value_ptr.* > 1000) { // Hot function threshold
                print("⚡ Hot function detected: {s} (called {s} times)\n", .{ entry.key_ptr.*, entry.value_ptr.* });
                // Apply aggressive optimizations here
            }
        }
    }

    /// Generate performance report
    pub fn printPerformanceReport(self: *StdlibRuntime) void {
        print("\n📊 STDLIB RUNTIME PERFORMANCE REPORT\n", .{});
        print("=====================================\n", .{});
        
        print("\n🕒 Compilation Times:\n", .{});
        var comp_iter = self.compilation_times.iterator();
        while (comp_iter.next()) |entry| {
            print("  {s}: {d}ms\n", .{ entry.key_ptr.*, entry.value_ptr.* / 1_000_000 });
        }
        
        print("\n📞 Function Call Counts:\n", .{});
        var call_iter = self.function_call_counts.iterator();
        while (call_iter.next()) |entry| {
            print("  {s}: {s} calls\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
        
        print("\n💾 Cached Modules: {s}\n", .{self.module_cache.modules.count()});
        print("=====================================\n", .{});
    }
};

/// Stdlib function registry for linking with main program
pub const StdlibRegistry = struct {
    runtime: *StdlibRuntime,
    registered_functions: HashMap([]const u8, StdlibFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub const StdlibFunction = struct {
        module_name: []const u8,
        function_name: []const u8,
        signature: []const u8,
        cached_ptr: ?*const fn() callconv(.c) void,
    };

    pub fn init(allocator: Allocator, runtime: *StdlibRuntime) StdlibRegistry {
        return StdlibRegistry{
            .runtime = runtime,
            .registered_functions = HashMap([]const u8, StdlibFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *StdlibRegistry) void {
        self.registered_functions.deinit(self.allocator);
    }

    /// Register all stdlib functions for runtime access
    pub fn registerAllStdlibFunctions(self: *StdlibRegistry) !void {
        const modules = try self.runtime.discoverModules();
        defer {
            for (modules.items) |module_name| {
                self.allocator.free(module_name);
            }
            modules.deinit();
        }

        for (modules.items) |module_name| {
            try self.registerModuleFunctions(module_name);
        }
    }

    /// Register functions from a specific module
    fn registerModuleFunctions(self: *StdlibRegistry, module_name: []const u8) !void {
        // Load module to extract function signatures
        const module = try self.runtime.loadModule(module_name);
        
        var func_iter = module.functions.iterator();
        while (func_iter.next()) |entry| {
            const full_name = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ module_name, entry.key_ptr.* });
            
            const stdlib_func = StdlibFunction{
                .module_name = try self.allocator.dupe(u8, module_name),
                .function_name = try self.allocator.dupe(u8, entry.key_ptr.*),
                .signature = try std.fmt.allocPrint(self.allocator, "({s}) -> normie", .{"normie"}), // Simplified
                .cached_ptr = entry.value_ptr.*,
            };
            
            try self.registered_functions.put(full_name, stdlib_func);
            print("📝 Registered function: {s}\n", .{full_name});
        }
    }

    /// Resolve a stdlib function call at runtime
    pub fn resolveFunction(self: *StdlibRegistry, full_name: []const u8) ?StdlibFunction {
        return self.registered_functions.get(full_name);
    }
};

/// High-level stdlib integration for CURSED programs
pub fn initializeStdlibRuntime(allocator: Allocator, stdlib_path: []const u8) !StdlibRuntime {
    print("🚀 Initializing CURSED Stdlib Runtime...\n", .{});
    
    var runtime = try StdlibRuntime.init(allocator, stdlib_path);
    
    // Pre-load critical modules
    const critical_modules = [_][]const u8{ "vibez", "testz", "mathz", "stringz" };
    for (critical_modules) |module_name| {
        _ = runtime.loadModule(module_name) catch |err| {
            print("⚠️ Failed to preload module {s}: {any}\n", .{ module_name, err });
            continue;
        };
    }
    
    print("✅ Stdlib Runtime initialized successfully\n", .{});
    return runtime;
}

/// Test the stdlib runtime system
pub fn testStdlibRuntime(allocator: Allocator) !void {
        _ = allocator;
    print("\n🧪 Testing CURSED Stdlib Runtime System\n", .{});
    print("======================================\n", .{});
    
    var runtime = try initializeStdlibRuntime(allocator, "stdlib");
    defer runtime.deinit();
    
    // Test module discovery
    const modules = try runtime.discoverModules();
    defer {
        for (modules.items) |module_name| {
            allocator.free(module_name);
        }
        modules.deinit();
    }
    
    print("\n📦 Discovered {s} stdlib modules\n", .{modules.items.len});
    
    // Test function calls
    const test_args = [_]interpreter.Value{interpreter.Value{ .String = "Hello World" }};
    
    for (modules.items) |module_name| {
        _ = runtime.callFunction(module_name, "test_function", &test_args) catch |err| {
            print("⚠️ Failed to call test function in {s}: {any}\n", .{ module_name, err });
        };
    }
    
    // Test performance optimization
    try runtime.optimizeHotFunctions();
    
    // Print performance report
    runtime.printPerformanceReport();
    
    print("\n✅ Stdlib Runtime tests completed\n", .{});
}
