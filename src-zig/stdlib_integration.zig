const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const print = std.debug.print;

const ast = @import("ast.zig");
const interpreter = @import("interpreter.zig");
const stdlib_runtime = @import("stdlib_runtime.zig");
const jit_execution_engine = @import("jit_execution_engine.zig");

/// CURSED Stdlib Integration Layer
/// 
/// This module provides seamless integration between the CURSED compiler
/// and the pure CURSED stdlib modules. It handles:
/// 
/// - Automatic stdlib function resolution during compilation
/// - Runtime linking of stdlib modules to main programs
/// - Performance optimization through adaptive JIT compilation
/// - Error handling and debugging support
/// - Memory management between main program and stdlib
/// - Thread-safe concurrent stdlib execution

pub const StdlibIntegrationError = error{
    ModuleResolutionFailed,
    FunctionNotFound,
    TypeMismatch,
    LinkingFailed,
    RuntimeError,
    OutOfMemory,
    CompilationFailed,
    InvalidArguments,
};

/// Stdlib function call information
pub const StdlibCall = struct {
    module_name: []const u8,
    function_name: []const u8,
    arguments: []const interpreter.Value,
    return_type: []const u8,
    call_site: ?*ast.Expression,
    
    pub fn fullName(self: StdlibCall, allocator: Allocator) ![]u8 {
        return std.fmt.allocPrint(allocator, "{s}.{s}", .{ self.module_name, self.function_name });
    }
};

/// Cached stdlib function metadata
pub const StdlibFunctionMetadata = struct {
    module_name: []const u8,
    function_name: []const u8,
    signature: []const u8,
    return_type: []const u8,
    parameter_types: []const []const u8,
    is_pure: bool,
    is_thread_safe: bool,
    complexity: enum { Constant, Linear, Quadratic, Exponential },
    last_compilation_time: u64,
    optimization_level: u8,
    
    pub fn init(allocator: Allocator, module_name: []const u8, function_name: []const u8) !StdlibFunctionMetadata {
        return StdlibFunctionMetadata{
            .module_name = try allocator.dupe(u8, module_name),
            .function_name = try allocator.dupe(u8, function_name),
            .signature = try allocator.dupe(u8, "() -> normie"),
            .return_type = try allocator.dupe(u8, "normie"),
            .parameter_types = &[_][]const u8{},
            .is_pure = false,
            .is_thread_safe = true,
            .complexity = .Linear,
            .last_compilation_time = 0,
            .optimization_level = 0,
        };
    }
    
    pub fn deinit(self: *StdlibFunctionMetadata, allocator: Allocator) void {
        allocator.free(self.module_name);
        allocator.free(self.function_name);
        allocator.free(self.signature);
        allocator.free(self.return_type);
        for (self.parameter_types) |param_type| {
            allocator.free(param_type);
        }
        allocator.free(self.parameter_types);
    }
};

/// Main integration layer
pub const StdlibIntegration = struct {
    allocator: Allocator,
    runtime: stdlib_runtime.StdlibRuntime,
    jit_engine: jit_execution_engine.JITExecutionEngine,
    function_registry: stdlib_runtime.StdlibRegistry,
    
    // Function metadata cache
    function_metadata: HashMap([]const u8, StdlibFunctionMetadata, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Module dependency graph
    module_dependencies: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Performance tracking
    stdlib_call_count: u64,
    total_stdlib_time: u64,
    cache_hits: u64,
    cache_misses: u64,
    
    // Configuration
    auto_optimization_enabled: bool,
    max_inline_depth: u32,
    debug_mode: bool,

    pub fn init(allocator: Allocator, stdlib_path: []const u8) !StdlibIntegration {
        var runtime = try stdlib_runtime.initializeStdlibRuntime(allocator, stdlib_path);
        const jit_engine = try jit_execution_engine.JITExecutionEngine.init(allocator);
        var function_registry = stdlib_runtime.StdlibRegistry.init(allocator, &runtime);
        
        // Register all stdlib functions
        try function_registry.registerAllStdlibFunctions();
        
        return StdlibIntegration{
            .allocator = allocator,
            .runtime = runtime,
            .jit_engine = jit_engine,
            .function_registry = function_registry,
            .function_metadata = HashMap([]const u8, StdlibFunctionMetadata, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .module_dependencies = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .stdlib_call_count = 0,
            .total_stdlib_time = 0,
            .cache_hits = 0,
            .cache_misses = 0,
            .auto_optimization_enabled = true,
            .max_inline_depth = 3,
            .debug_mode = false,
        };
    }

    pub fn deinit(self: *StdlibIntegration) void {
        self.runtime.deinit();
        self.jit_engine.deinit();
        self.function_registry.deinit();
        
        // Clean up function metadata
        var metadata_iter = self.function_metadata.iterator();
        while (metadata_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.function_metadata.deinit();
        
        // Clean up module dependencies
        var deps_iter = self.module_dependencies.iterator();
        while (deps_iter.next()) |entry| {
            for (entry.value_ptr.items) |dep| {
                self.allocator.free(dep);
            }
            entry.value_ptr.deinit();
        }
        self.module_dependencies.deinit();
    }

    /// Resolve a stdlib function call at compile time
    pub fn resolveStdlibCall(self: *StdlibIntegration, call: StdlibCall) !interpreter.Value {
        const start_time = std.time.nanoTimestamp();
        
        const full_name = try call.fullName(self.allocator);
        defer self.allocator.free(full_name);
        
        // Update call statistics
        self.stdlib_call_count += 1;
        
        if (self.debug_mode) {
            print("🔍 Resolving stdlib call: {s}\n", .{full_name});
        }
        
        // Validate function signature and types
        if (try self.validateFunctionCall(call)) |validation_error| {
            print("❌ Type validation failed for {s}: {s}\n", .{ full_name, validation_error });
            return StdlibIntegrationError.TypeMismatch;
        }
        
        // Check function registry
        if (self.function_registry.resolveFunction(full_name)) |_| {
            // Function found in registry
            self.cache_hits += 1;
            
            // Execute through runtime system
            const result = try self.runtime.callFunction(call.module_name, call.function_name, call.arguments);
            
            // Update performance metrics
            const execution_time = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
            self.total_stdlib_time += execution_time;
            
            if (self.debug_mode) {
                print("✅ Stdlib call completed in {}μs\n", .{execution_time / 1000});
            }
            
            return result;
        } else {
            // Function not found in registry - try dynamic loading
            self.cache_misses += 1;
            return self.dynamicResolveFunction(call);
        }
    }
    
    /// Validate function call arguments and return types
    fn validateFunctionCall(self: *StdlibIntegration, call: StdlibCall) !?[]const u8 {
        const full_name = try call.fullName(self.allocator);
        defer self.allocator.free(full_name);
        
        // Get function metadata for type checking
        if (self.function_metadata.get(full_name)) |metadata| {
            // Check argument count
            if (call.arguments.len != metadata.parameter_types.len) {
                return try std.fmt.allocPrint(self.allocator, "Expected {} arguments, got {}", .{ metadata.parameter_types.len, call.arguments.len });
            }
            
            // Check argument types
            for (call.arguments, 0..) |arg, i| {
                const expected_type = metadata.parameter_types[i];
                const actual_type = switch (arg) {
                    .String => "tea",
                    .Integer => "normie",
                    .Float => "meal",
                    .Boolean => "lit",
                    else => "unknown",
                };
                
                if (!std.mem.eql(u8, actual_type, expected_type)) {
                    return try std.fmt.allocPrint(self.allocator, "Argument {} expected type {s}, got {s}", .{ i, expected_type, actual_type });
                }
            }
            
            return null; // Validation passed
        }
        
        // Create default metadata for unknown functions
        const metadata = StdlibFunctionMetadata.init(self.allocator, call.module_name, call.function_name);
        try self.function_metadata.put(try self.allocator.dupe(u8, full_name), metadata);
        
        return null; // Allow unknown functions to pass
    }

    /// Dynamically load and resolve a stdlib function
    fn dynamicResolveFunction(self: *StdlibIntegration, call: StdlibCall) !interpreter.Value {
        print("🔄 Dynamic resolution for: {s}.{s}\n", .{ call.module_name, call.function_name });
        
        // Load module if not already loaded
        _ = try self.runtime.loadModule(call.module_name);
        
        // Re-register functions from the newly loaded module
        try self.function_registry.registerModuleFunctions(call.module_name);
        
        // Try resolution again
        const full_name = try call.fullName(self.allocator);
        defer self.allocator.free(full_name);
        
        if (self.function_registry.resolveFunction(full_name)) |_| {
            return self.jit_engine.executeFunction(full_name, call.arguments);
        }
        
        print("❌ Function {s} not found in module {s}\n", .{ call.function_name, call.module_name });
        return StdlibIntegrationError.FunctionNotFound;
    }

    /// Preload and optimize critical stdlib modules
    pub fn preloadCriticalModules(self: *StdlibIntegration) !void {
        print("🚀 Preloading critical stdlib modules...\n", .{});
        
        const critical_modules = [_][]const u8{
            "vibez",    // I/O operations - used in almost every program
            "testz",    // Testing framework - used in development
            "mathz",    // Mathematical operations - commonly used
            "stringz",  // String processing - frequently needed
        };
        
        for (critical_modules) |module_name| {
            _ = self.runtime.loadModule(module_name) catch |err| {
                print("⚠️ Failed to preload {s}: {any}\n", .{ module_name, err });
                continue;
            };
            
            // Precompile hot functions for this module
            try self.precompileHotFunctions(module_name);
        }
        
        print("✅ Critical modules preloaded successfully\n", .{});
    }

    /// Precompile hot functions for better startup performance
    fn precompileHotFunctions(self: *StdlibIntegration, module_name: []const u8) !void {
        const hot_functions = switch (std.mem.eql(u8, module_name, "vibez")) {
            true => [_][]const u8{ "spill", "spillf", "spillln", "scan" },
            false => switch (std.mem.eql(u8, module_name, "mathz")) {
                true => [_][]const u8{ "math_add", "math_multiply", "sqrt_meal", "sin_meal" },
                false => switch (std.mem.eql(u8, module_name, "stringz")) {
                    true => [_][]const u8{ "length", "concat", "substring", "contains" },
                    false => &[_][]const u8{},
                },
            },
        };
        
        for (hot_functions) |func_name| {
            const full_name = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ module_name, func_name });
            defer self.allocator.free(full_name);
            
            // Simulate calls to trigger JIT compilation
            const dummy_args = [_]interpreter.Value{interpreter.Value{ .Integer = 0 }};
            _ = self.jit_engine.executeFunction(full_name, &dummy_args) catch {
                continue; // Skip functions that can't be precompiled
            };
        }
    }

    /// Optimize all stdlib modules based on usage patterns
    pub fn optimizeAllModules(self: *StdlibIntegration) !void {
        print("⚡ Optimizing all stdlib modules...\n", .{});
        
        // Optimize JIT hot functions
        try self.jit_engine.optimizeHotFunctions();
        
        // Optimize runtime modules
        try self.runtime.optimizeHotFunctions();
        
        print("✅ Stdlib optimization completed\n", .{});
    }

    /// Analyze stdlib usage patterns and dependencies
    pub fn analyzeUsagePatterns(self: *StdlibIntegration) !void {
        print("📊 Analyzing stdlib usage patterns...\n", .{});
        
        // Build module dependency graph
        const modules = try self.runtime.discoverModules();
        defer {
            for (modules.items) |module_name| {
                self.allocator.free(module_name);
            }
            modules.deinit();
        }
        
        for (modules.items) |module_name| {
            try self.analyzeModuleDependencies(module_name);
        }
        
        // Identify optimization opportunities
        try self.identifyOptimizationOpportunities();
        
        print("✅ Usage pattern analysis completed\n", .{});
    }

    /// Analyze dependencies for a specific module
    fn analyzeModuleDependencies(self: *StdlibIntegration, module_name: []const u8) !void {
        const module_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}/mod.csd", .{ self.runtime.stdlib_path, module_name });
        defer self.allocator.free(module_path);
        
        const source_code = std.fs.cwd().readFileAlloc(self.allocator, module_path, 1024 * 1024) catch {
            return; // Skip if can't read module
        };
        defer self.allocator.free(source_code);
        
        var dependencies = .empty;
        
        // Simple dependency analysis - look for 'yeet' statements
        var lines = std.mem.split(u8, source_code, "\n");
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (std.mem.startsWith(u8, trimmed, "yeet \"")) {
                const start = trimmed.len > 6 and trimmed[5] == '"';
                if (start) {
                    const end_quote = std.mem.indexOf(u8, trimmed[6..], "\"");
                    if (end_quote) |end| {
                        const dep_name = trimmed[6..6+end];
                        const dep_copy = try self.allocator.dupe(u8, dep_name);
                        try dependencies.append(self.allocator, dep_copy);
                        
                        if (self.debug_mode) {
                            print("  📦 {s} depends on {s}\n", .{ module_name, dep_name });
                        }
                    }
                }
            }
        }
        
        try self.module_dependencies.put(try self.allocator.dupe(u8, module_name), dependencies);
    }

    /// Identify optimization opportunities based on usage patterns
    fn identifyOptimizationOpportunities(self: *StdlibIntegration) !void {
        print("🔍 Identifying optimization opportunities...\n", .{});
        
        // Find modules with many dependencies (good candidates for optimization)
        var deps_iter = self.module_dependencies.iterator();
        while (deps_iter.next()) |entry| {
            const module_name = entry.key_ptr.*;
            const deps = entry.value_ptr.*;
            
            if (deps.items.len > 2) {
                print("  🎯 High-dependency module: {s} ({} deps)\n", .{ module_name, deps.items.len });
            }
        }
        
        // Analyze function call patterns
        const hot_threshold = self.stdlib_call_count / 10; // Top 10% of calls
        var func_iter = self.jit_engine.functions.iterator();
        while (func_iter.next()) |entry| {
            const jit_func = entry.value_ptr;
            if (jit_func.call_count > hot_threshold) {
                print("  🔥 Hot function: {s}.{s} ({} calls)\n", .{ 
                    jit_func.module_name, 
                    jit_func.name, 
                    jit_func.call_count 
                });
            }
        }
    }

    /// Generate comprehensive integration report
    pub fn generateIntegrationReport(self: *StdlibIntegration) void {
        print("\n📋 STDLIB INTEGRATION PERFORMANCE REPORT\n", .{});
        print("========================================\n", .{});
        
        print("📊 Overall Statistics:\n", .{});
        print("  Total stdlib calls: {}\n", .{self.stdlib_call_count});
        print("  Total execution time: {}ms\n", .{self.total_stdlib_time / 1_000_000});
        print("  Cache hit rate: {d:.2}%\n", .{
            if (self.stdlib_call_count > 0) 
                @as(f64, @floatFromInt(self.cache_hits)) / @as(f64, @floatFromInt(self.stdlib_call_count)) * 100.0 
            else 0.0
        });
        
        if (self.stdlib_call_count > 0) {
            print("  Average call time: {d:.2}μs\n", .{
                @as(f64, @floatFromInt(self.total_stdlib_time)) / @as(f64, @floatFromInt(self.stdlib_call_count)) / 1000.0
            });
        }
        
        print("\n🏗️ Module Dependencies:\n", .{});
        var deps_iter = self.module_dependencies.iterator();
        while (deps_iter.next()) |entry| {
            print("  {s}: {} dependencies\n", .{ entry.key_ptr.*, entry.value_ptr.items.len });
        }
        
        print("\n⚡ Performance Optimization Status:\n", .{});
        print("  Auto-optimization: {}\n", .{if (self.auto_optimization_enabled) "enabled" else "disabled"});
        print("  Max inline depth: {}\n", .{self.max_inline_depth});
        print("  Debug mode: {}\n", .{if (self.debug_mode) "enabled" else "disabled"});
        
        // Delegate to sub-systems for detailed reports
        print("\n" ++ "=" ** 40 ++ "\n", .{});
        self.runtime.printPerformanceReport();
        self.jit_engine.generatePerformanceReport();
        
        print("========================================\n", .{});
    }

    /// Enable/disable debug mode
    pub fn setDebugMode(self: *StdlibIntegration, enabled: bool) void {
        self.debug_mode = enabled;
        print("🐛 Debug mode: {}\n", .{if (enabled) "enabled" else "disabled"});
    }

    /// Configure auto-optimization settings
    pub fn configureOptimization(self: *StdlibIntegration, enabled: bool, max_inline_depth: u32) void {
        self.auto_optimization_enabled = enabled;
        self.max_inline_depth = max_inline_depth;
        print("⚙️ Auto-optimization: {}, inline depth: {}\n", .{ enabled, max_inline_depth });
    }
};

/// High-level stdlib integration for CURSED programs
pub fn createStdlibIntegration(allocator: Allocator, stdlib_path: []const u8) !StdlibIntegration {
    print("🚀 Creating CURSED Stdlib Integration...\n", .{});
    
    var integration = try StdlibIntegration.init(allocator, stdlib_path);
    
    // Preload critical modules
    try integration.preloadCriticalModules();
    
    // Analyze usage patterns
    try integration.analyzeUsagePatterns();
    
    print("✅ Stdlib Integration created successfully\n", .{});
    return integration;
}

/// Test the stdlib integration system
pub fn testStdlibIntegration(allocator: Allocator) !void {
    print("\n🧪 Testing CURSED Stdlib Integration\n", .{});
    print("===================================\n", .{});
    
    var integration = try createStdlibIntegration(allocator, "stdlib");
    defer integration.deinit();
    
    // Test function resolution
    const test_calls = [_]StdlibCall{
        .{
            .module_name = "vibez",
            .function_name = "spill",
            .arguments = &[_]interpreter.Value{interpreter.Value{ .String = "Hello World" }},
            .return_type = "normie",
            .call_site = null,
        },
        .{
            .module_name = "mathz",
            .function_name = "math_add",
            .arguments = &[_]interpreter.Value{
                interpreter.Value{ .Integer = 42 },
                interpreter.Value{ .Integer = 24 },
            },
            .return_type = "drip",
            .call_site = null,
        },
        .{
            .module_name = "stringz",
            .function_name = "length",
            .arguments = &[_]interpreter.Value{interpreter.Value{ .String = "test string" }},
            .return_type = "drip",
            .call_site = null,
        },
    };
    
    // Test multiple function calls
    for (test_calls) |call| {
        _ = integration.resolveStdlibCall(call) catch |err| {
            print("⚠️ Failed to resolve {s}.{s}: {any}\n", .{ call.module_name, call.function_name, err });
        };
    }
    
    // Test optimization
    try integration.optimizeAllModules();
    
    // Generate performance report
    integration.generateIntegrationReport();
    
    print("\n✅ Stdlib Integration tests completed\n", .{});
}
