/// Integration layer between runtime generic system and LLVM codegen
/// Provides seamless integration of runtime generics with compilation pipeline

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const generics = @import("generics.zig");
const runtime_generics = @import("runtime_generic_system.zig");
const type_system = @import("type_system_runtime.zig");
const advanced_codegen = @import("advanced_codegen.zig");
const error_handling = @import("error_handling.zig");

// LLVM C imports
const c = @cImport({
    @cInclude("llvm_c_bindings.h");
});

/// Integrated generic compiler that bridges runtime and compile-time generics
pub const IntegratedGenericCompiler = struct {
    allocator: Allocator,
    
    // Core components
    runtime_type_env: *runtime_generics.RuntimeTypeEnvironment,
    monomorphizer: *generics.Monomorphizer,
    runtime_engine: *runtime_generics.RuntimeGenericEngine,
    
    // LLVM integration
    llvm_context: c.LLVMContextRef,
    llvm_module: c.LLVMModuleRef,
    
    // Compilation cache and optimization
    compiled_instances: HashMap([]const u8, CompiledInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    optimization_level: OptimizationLevel,
    
    // Performance tracking
    compile_stats: CompilationStats,
    
    pub const OptimizationLevel = enum {
        None,        // -O0
        Size,        // -Os  
        Speed,       // -O2
        Aggressive,  // -O3
    };
    
    pub const CompiledInstance = struct {
        mangled_name: []const u8,
        llvm_function: ?c.LLVMValueRef,
        llvm_type: ?c.LLVMTypeRef,
        runtime_type: runtime_generics.RuntimeType,
        compile_time_ms: u64,
        code_size_bytes: usize,
        
        pub fn init(allocator: Allocator, mangled_name: []const u8, runtime_type: runtime_generics.RuntimeType) CompiledInstance {
            return CompiledInstance{
                .mangled_name = allocator.dupe(u8, mangled_name) catch unreachable,
                .llvm_function = null,
                .llvm_type = null,
                .runtime_type = runtime_type,
                .compile_time_ms = 0,
                .code_size_bytes = 0,
            };
        }
        
        pub fn deinit(self: *CompiledInstance, allocator: Allocator) void {
        _ = allocator;
            allocator.free(self.mangled_name);
            self.runtime_type.deinit(self.allocator);
        }
    };
    
    pub const CompilationStats = struct {
        total_instantiations: u64,
        cache_hits: u64,
        cache_misses: u64,
        total_compile_time_ms: u64,
        average_compile_time_ms: f64,
        total_code_size_bytes: usize,
        
        pub fn init() CompilationStats {
            return CompilationStats{
                .total_instantiations = 0,
                .cache_hits = 0,
                .cache_misses = 0,
                .total_compile_time_ms = 0,
                .average_compile_time_ms = 0.0,
                .total_code_size_bytes = 0,
            };
        }
        
        pub fn recordInstantiation(self: *CompilationStats, compile_time_ms: u64, code_size: usize, cache_hit: bool) void {
            self.total_instantiations += 1;
            self.total_compile_time_ms += compile_time_ms;
            self.total_code_size_bytes += code_size;
            
            if (cache_hit) {
                self.cache_hits += 1;
            } else {
                self.cache_misses += 1;
            }
            
            self.average_compile_time_ms = @as(f64, @floatFromInt(self.total_compile_time_ms)) / @as(f64, @floatFromInt(self.total_instantiations));
        }
    };
    
    pub fn init(
        allocator: Allocator,
        runtime_type_env: *runtime_generics.RuntimeTypeEnvironment,
        llvm_context: c.LLVMContextRef,
        llvm_module: c.LLVMModuleRef
    ) !IntegratedGenericCompiler {
        var monomorphizer = try allocator.create(generics.Monomorphizer);
        monomorphizer.* = generics.Monomorphizer.init(allocator, llvm_context, llvm_module);
        
        var runtime_engine = try allocator.create(runtime_generics.RuntimeGenericEngine);
        runtime_engine.* = runtime_generics.RuntimeGenericEngine.init(allocator, runtime_type_env, monomorphizer);
        
        return IntegratedGenericCompiler{
            .allocator = allocator,
            .runtime_type_env = runtime_type_env,
            .monomorphizer = monomorphizer,
            .runtime_engine = runtime_engine,
            .llvm_context = llvm_context,
            .llvm_module = llvm_module,
            .compiled_instances = HashMap([]const u8, CompiledInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .optimization_level = .Speed,
            .compile_stats = CompilationStats.init(),
        };
    }
    
    pub fn deinit(self: *IntegratedGenericCompiler) void {
        var instance_iter = self.compiled_instances.iterator();
        while (instance_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.compiled_instances.deinit(self.allocator);
        
        self.runtime_engine.deinit(self.allocator);
        self.allocator.destroy(self.runtime_engine);
        
        self.monomorphizer.deinit(self.allocator);
        self.allocator.destroy(self.monomorphizer);
    }
    
    /// Compile generic function with runtime type arguments
    pub fn compileGenericFunction(
        self: *IntegratedGenericCompiler,
        generic_name: []const u8,
        type_args: []runtime_generics.RuntimeType,
        function_body: ast.Statement
    ) !c.LLVMValueRef {
        const start_time = std.time.milliTimestamp();
        
        // Generate mangled name for cache lookup
        const mangled_name = try self.generateMangledName(generic_name, type_args);
        defer self.allocator.free(mangled_name);
        
        // Check compilation cache
        if (self.compiled_instances.get(mangled_name)) |instance| {
            self.compile_stats.recordInstantiation(0, 0, true); // Cache hit
            return instance.llvm_function.?;
        }
        
        // Perform runtime instantiation
        const instantiated_type = try self.runtime_type_env.instantiateGeneric(generic_name, type_args);
        
        // Convert runtime types to AST types for monomorphizer
        var ast_type_args = std.ArrayList(u8){};
        defer ast_type_args.deinit();
        
        for (type_args) |runtime_type| {
            const ast_type = try self.runtimeTypeToAstType(runtime_type);
            try ast_type_args.append(allocator, ast_type);
        }
        
        // Request monomorphization
        const specialized_name = try self.monomorphizer.requestInstantiation(
            generic_name,
            ast_type_args.items,
            "integrated_compiler"
        );
        
        // Process all pending instantiations
        try self.monomorphizer.processInstantiations();
        
        // Get generated LLVM function
        const llvm_function = self.monomorphizer.getInstantiatedFunction(specialized_name) orelse {
            return error.CompilationFailed;
        };
        
        // Apply optimizations
        try self.applyOptimizations(llvm_function);
        
        // Create compilation record
        const compile_time = @as(u64, @intCast(std.time.milliTimestamp() - start_time));
        const code_size = self.estimateCodeSize(llvm_function);
        
        var instance = CompiledInstance.init(self.allocator, mangled_name, instantiated_type);
        instance.llvm_function = llvm_function;
        instance.compile_time_ms = compile_time;
        instance.code_size_bytes = code_size;
        
        const owned_mangled_name = try self.allocator.dupe(u8, mangled_name);
        try self.compiled_instances.put(owned_mangled_name, instance);
        
        self.compile_stats.recordInstantiation(compile_time, code_size, false); // Cache miss
        
        std.log.info("Compiled generic function {s} in {}ms, size: {} bytes", 
            .{ generic_name, compile_time, code_size });
        
        return llvm_function;
    }
    
    /// Compile generic struct with runtime type arguments  
    pub fn compileGenericStruct(
        self: *IntegratedGenericCompiler,
        generic_name: []const u8,
        type_args: []runtime_generics.RuntimeType,
        struct_body: ast.StructStatement
    ) !c.LLVMTypeRef {
        _ = struct_body;
        
        const start_time = std.time.milliTimestamp();
        
        // Generate mangled name
        const mangled_name = try self.generateMangledName(generic_name, type_args);
        defer self.allocator.free(mangled_name);
        
        // Check cache
        if (self.compiled_instances.get(mangled_name)) |instance| {
            self.compile_stats.recordInstantiation(0, 0, true);
            return instance.llvm_type.?;
        }
        
        // Perform instantiation
        const instantiated_type = try self.runtime_type_env.instantiateGeneric(generic_name, type_args);
        
        // Convert to AST types for monomorphizer
        var ast_type_args = std.ArrayList(u8){};
        defer ast_type_args.deinit();
        
        for (type_args) |runtime_type| {
            const ast_type = try self.runtimeTypeToAstType(runtime_type);
            try ast_type_args.append(allocator, ast_type);
        }
        
        // Request monomorphization
        const specialized_name = try self.monomorphizer.requestInstantiation(
            generic_name,
            ast_type_args.items,
            "struct_compiler"
        );
        
        try self.monomorphizer.processInstantiations();
        
        // Get generated LLVM type
        const llvm_type = self.monomorphizer.getInstantiatedType(specialized_name) orelse {
            return error.CompilationFailed;
        };
        
        // Create compilation record
        const compile_time = @as(u64, @intCast(std.time.milliTimestamp() - start_time));
        const code_size = self.estimateTypeSize(llvm_type);
        
        var instance = CompiledInstance.init(self.allocator, mangled_name, instantiated_type);
        instance.llvm_type = llvm_type;
        instance.compile_time_ms = compile_time;
        instance.code_size_bytes = code_size;
        
        const owned_mangled_name = try self.allocator.dupe(u8, mangled_name);
        try self.compiled_instances.put(owned_mangled_name, instance);
        
        self.compile_stats.recordInstantiation(compile_time, code_size, false);
        
        return llvm_type;
    }
    
    /// Batch compilation for improved performance
    pub fn batchCompileGenerics(
        self: *IntegratedGenericCompiler,
        requests: []CompilationRequest
    ) ![]CompilationResult {
        var results = std.ArrayList(u8){};
        
        // Sort requests by priority and dependencies
        const sorted_requests = try self.sortCompilationRequests(requests);
        defer self.allocator.free(sorted_requests);
        
        // Compile in dependency order
        for (sorted_requests) |request| {
            const result = switch (request.kind) {
                .Function => CompilationResult{
                    .name = request.name,
                    .llvm_function = try self.compileGenericFunction(
                        request.name,
                        request.type_args,
                        request.body.Function
                    ),
                    .llvm_type = null,
                },
                .Struct => CompilationResult{
                    .name = request.name,
                    .llvm_function = null,
                    .llvm_type = try self.compileGenericStruct(
                        request.name,
                        request.type_args,
                        request.body.Struct
                    ),
                },
            };
            
            try results.append(allocator, result);
        }
        
        return results.toOwnedSlice();
    }
    
    pub const CompilationRequest = struct {
        name: []const u8,
        type_args: []runtime_generics.RuntimeType,
        kind: RequestKind,
        body: RequestBody,
        priority: Priority,
        dependencies: [][]const u8,
        
        pub const RequestKind = enum { Function, Struct };
        pub const Priority = enum { Low, Normal, High, Critical };
        
        pub const RequestBody = union(RequestKind) {
            Function: ast.Statement,
            Struct: ast.StructStatement,
        };
    };
    
    pub const CompilationResult = struct {
        name: []const u8,
        llvm_function: ?c.LLVMValueRef,
        llvm_type: ?c.LLVMTypeRef,
    };
    
    /// Type inference for generic function calls
    pub fn inferAndCompileGenericCall(
        self: *IntegratedGenericCompiler,
        generic_name: []const u8,
        arg_types: []runtime_generics.RuntimeType,
        expected_return: ?runtime_generics.RuntimeType
    ) !c.LLVMValueRef {
        // Infer type arguments from call site
        const inferred_args = try self.runtime_type_env.inferTypeArguments(
            generic_name,
            arg_types,
            expected_return
        );
        defer self.allocator.free(inferred_args);
        
        // Get function body (would come from AST in real implementation)
        const dummy_body = ast.Statement{ .Return = ast.ReturnStatement{ .value = null } };
        
        return self.compileGenericFunction(generic_name, inferred_args, dummy_body);
    }
    
    /// Set optimization level
    pub fn setOptimizationLevel(self: *IntegratedGenericCompiler, level: OptimizationLevel) void {
        self.optimization_level = level;
    }
    
    /// Get compilation statistics
    pub fn getCompilationStats(self: *IntegratedGenericCompiler) CompilationStats {
        return self.compile_stats;
    }
    
    /// Clear compilation cache
    pub fn clearCache(self: *IntegratedGenericCompiler) void {
        var instance_iter = self.compiled_instances.iterator();
        while (instance_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.compiled_instances.clearRetainingCapacity();
        
        std.log.info("Cleared generic compilation cache", .{});
    }
    
    // Helper methods
    
    fn generateMangledName(self: *IntegratedGenericCompiler, generic_name: []const u8, type_args: []runtime_generics.RuntimeType) ![]const u8 {
        var name_parts = std.ArrayList(u8){};
        defer name_parts.deinit();
        
        try name_parts.appendSlice(generic_name);
        try name_parts.appendSlice("_");
        
        for (type_args, 0..) |arg, i| {
            if (i > 0) try name_parts.appendSlice("_");
            const arg_name = try arg.getMangledName(self.allocator);
            defer self.allocator.free(arg_name);
            try name_parts.appendSlice(arg_name);
        }
        
        return name_parts.toOwnedSlice(self.allocator);
    }
    
    fn runtimeTypeToAstType(self: *IntegratedGenericCompiler, runtime_type: runtime_generics.RuntimeType) !ast.Type {
        return switch (runtime_type.kind) {
            .Primitive => {
                if (std.mem.eql(u8, runtime_type.name, "lit")) return ast.Type{ .Primitive = .Lit };
                if (std.mem.eql(u8, runtime_type.name, "drip")) return ast.Type{ .Primitive = .Drip };
                if (std.mem.eql(u8, runtime_type.name, "normie")) return ast.Type{ .Primitive = .Normie };
                if (std.mem.eql(u8, runtime_type.name, "thicc")) return ast.Type{ .Primitive = .Thicc };
                if (std.mem.eql(u8, runtime_type.name, "smol")) return ast.Type{ .Primitive = .Smol };
                if (std.mem.eql(u8, runtime_type.name, "meal")) return ast.Type{ .Primitive = .Meal };
                if (std.mem.eql(u8, runtime_type.name, "snack")) return ast.Type{ .Primitive = .Snack };
                if (std.mem.eql(u8, runtime_type.name, "tea")) return ast.Type{ .Primitive = .Tea };
                if (std.mem.eql(u8, runtime_type.name, "vibes")) return ast.Type{ .Primitive = .Vibes };
                return error.UnknownPrimitiveType;
            },
            .Struct, .Interface, .Instantiated => ast.Type{ .Identifier = runtime_type.name },
            .Array => {
                if (runtime_type.type_args) |args| {
                    if (args.items.len > 0) {
                        const element_type = try self.allocator.create(ast.Type);
                        element_type.* = try self.runtimeTypeToAstType(args.items[0]);
                        return ast.Type{ .Array = ast.ArrayType{
                            .element_type = element_type,
                            .size = 1, // Default size
                        }};
                    }
                }
                return error.InvalidArrayType;
            },
            .Slice => {
                if (runtime_type.type_args) |args| {
                    if (args.items.len > 0) {
                        const element_type = try self.allocator.create(ast.Type);
                        element_type.* = try self.runtimeTypeToAstType(args.items[0]);
                        return ast.Type{ .Slice = ast.SliceType{
                            .element_type = element_type,
                        }};
                    }
                }
                return error.InvalidSliceType;
            },
            else => ast.Type{ .Identifier = runtime_type.name },
        };
    }
    
    fn applyOptimizations(self: *IntegratedGenericCompiler, llvm_function: c.LLVMValueRef) !void {
        switch (self.optimization_level) {
            .None => {}, // No optimizations
            .Size => {
                // Size optimizations
                if (c.LLVMGetFunctionCallConv) |_| {
                    c.LLVMSetFunctionCallConv(llvm_function, c.LLVMCCallConv);
                }
            },
            .Speed => {
                // Speed optimizations
                if (c.LLVMAddAttributeAtIndex) |add_attr| {
                    const inline_attr = c.LLVMCreateEnumAttribute(
                        self.llvm_context,
                        c.LLVMGetEnumAttributeKindForName("alwaysinline", 12),
                        0
                    );
                    add_attr(llvm_function, c.LLVMAttributeFunctionIndex, inline_attr);
                }
            },
            .Aggressive => {
                // Aggressive optimizations
                if (c.LLVMAddAttributeAtIndex) |add_attr| {
                    const inline_attr = c.LLVMCreateEnumAttribute(
                        self.llvm_context,
                        c.LLVMGetEnumAttributeKindForName("alwaysinline", 12),
                        0
                    );
                    add_attr(llvm_function, c.LLVMAttributeFunctionIndex, inline_attr);
                    
                    const fast_attr = c.LLVMCreateEnumAttribute(
                        self.llvm_context,
                        c.LLVMGetEnumAttributeKindForName("fast", 4),
                        0
                    );
                    add_attr(llvm_function, c.LLVMAttributeFunctionIndex, fast_attr);
                }
            },
        }
    }
    
    fn estimateCodeSize(self: *IntegratedGenericCompiler, llvm_function: c.LLVMValueRef) usize {
        _ = self;
        
        var size: usize = 0;
        var bb = c.LLVMGetFirstBasicBlock(llvm_function);
        
        while (bb != null) {
            var inst = c.LLVMGetFirstInstruction(bb);
            while (inst != null) {
                size += 4; // Rough estimate: 4 bytes per instruction
                inst = c.LLVMGetNextInstruction(inst);
            }
            bb = c.LLVMGetNextBasicBlock(bb);
        }
        
        return size;
    }
    
    fn estimateTypeSize(self: *IntegratedGenericCompiler, llvm_type: c.LLVMTypeRef) usize {
        _ = self;
        
        // Simple size estimation based on type kind
        const type_kind = c.LLVMGetTypeKind(llvm_type);
        return switch (type_kind) {
            c.LLVMVoidTypeKind => 0,
            c.LLVMIntegerTypeKind => @as(usize, c.LLVMGetIntTypeWidth(llvm_type)) / 8,
            c.LLVMFloatTypeKind => 4,
            c.LLVMDoubleTypeKind => 8,
            c.LLVMPointerTypeKind => 8, // 64-bit pointer
            c.LLVMStructTypeKind => {
                var total_size: usize = 0;
                const field_count = c.LLVMCountStructElementTypes(llvm_type);
                var i: u32 = 0;
                while (i < field_count) : (i += 1) {
                    const field_type = c.LLVMStructGetTypeAtIndex(llvm_type, i);
                    total_size += self.estimateTypeSize(field_type);
                }
                return total_size;
            },
            c.LLVMArrayTypeKind => {
                const element_type = c.LLVMGetElementType(llvm_type);
                const array_length = c.LLVMGetArrayLength(llvm_type);
                return self.estimateTypeSize(element_type) * array_length;
            },
            else => 8, // Default size
        };
    }
    
    fn sortCompilationRequests(self: *IntegratedGenericCompiler, requests: []CompilationRequest) ![]CompilationRequest {
        var sorted = std.ArrayList(u8){};
        
        // Simple topological sort based on dependencies and priority
        for (requests) |request| {
            try sorted.append(allocator, request);
        }
        
        // Sort by priority (higher priority first)
        std.sort.insertion(CompilationRequest, sorted.items, {}, struct {
            fn lessThan(_: void, a: CompilationRequest, b: CompilationRequest) bool {
                return @intFromEnum(a.priority) > @intFromEnum(b.priority);
            }
        }.lessThan);
        
        return sorted.toOwnedSlice();
    }
};

/// High-level generic compilation API
pub const GenericCompilationAPI = struct {
    compiler: *IntegratedGenericCompiler,
    
    pub fn init(compiler: *IntegratedGenericCompiler) GenericCompilationAPI {
        return GenericCompilationAPI{ .compiler = compiler };
    }
    
    /// Compile generic function with type inference
    pub fn compileGenericWithInference(
        self: *GenericCompilationAPI,
        generic_name: []const u8,
        call_site_args: []runtime_generics.RuntimeType
    ) !c.LLVMValueRef {
        return self.compiler.inferAndCompileGenericCall(generic_name, call_site_args, null);
    }
    
    /// Compile generic with explicit type arguments
    pub fn compileGenericExplicit(
        self: *GenericCompilationAPI,
        generic_name: []const u8,
        type_args: []runtime_generics.RuntimeType
    ) !c.LLVMValueRef {
        const dummy_body = ast.Statement{ .Return = ast.ReturnStatement{ .value = null } };
        return self.compiler.compileGenericFunction(generic_name, type_args, dummy_body);
    }
    
    /// Precompile commonly used generic instantiations
    pub fn precompileCommonInstantiations(self: *GenericCompilationAPI) !void {
        const common_generics = [_]struct { name: []const u8, types: []const []const u8 }{
            .{ .name = "Array", .types = &[_][]const u8{ "drip", "normie", "tea" } },
            .{ .name = "Option", .types = &[_][]const u8{ "drip", "tea" } },
            .{ .name = "Result", .types = &[_][]const u8{ "drip", "tea" } },
        };
        
        for (common_generics) |generic| {
            for (generic.types) |type_name| {
                // Create runtime type from name
                const runtime_type = runtime_generics.RuntimeType.init(
                    self.compiler.allocator,
                    .Primitive,
                    type_name
                );
                
                const type_args = [_]runtime_generics.RuntimeType{runtime_type};
                
                _ = self.compileGenericExplicit(generic.name, &type_args) catch |err| {
                    std.log.warn("Failed to precompile {s}<{s}>: {}", .{ generic.name, type_name, err });
                };
            }
        }
        
        std.log.info("Precompiled common generic instantiations", .{});
    }
    
    /// Get performance metrics
    pub fn getMetrics(self: *GenericCompilationAPI) IntegratedGenericCompiler.CompilationStats {
        return self.compiler.getCompilationStats();
    }
};

/// Test the integrated generic system
test "integrated generic compilation" {
    const allocator = std.testing.allocator;
    
    // Initialize LLVM
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    // Initialize type environment
    var type_env = runtime_generics.RuntimeTypeEnvironment.init(allocator);
    defer type_env.deinit();
    
    try runtime_generics.initializeBuiltinTypes(&type_env);
    
    // Initialize integrated compiler
    var compiler = try IntegratedGenericCompiler.init(allocator, &type_env, context, module);
    defer compiler.deinit();
    
    // Create API
    var api = GenericCompilationAPI.init(&compiler);
    
    // Test precompilation
    try api.precompileCommonInstantiations();
    
    // Check stats
    const stats = api.getMetrics();
    try std.testing.expect(stats.total_instantiations > 0);
    
    std.log.info("Integrated generic compilation test completed successfully", .{});
}
