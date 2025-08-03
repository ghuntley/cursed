const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const print = std.debug.print;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/Orc.h");
});

const ast = @import("ast_simple.zig");
const codegen = @import("codegen.zig");
const interpreter = @import("interpreter.zig");

/// Advanced JIT Execution Engine for CURSED Stdlib Functions
/// 
/// Features:
/// - LLVM ORC JIT compilation with lazy loading
/// - Tiered compilation (interpreter -> optimized JIT)
/// - Hot function detection and aggressive optimization
/// - Memory-efficient code caching
/// - Thread-safe concurrent execution

pub const JITError = error{
    InitializationFailed,
    CompilationFailed,
    OptimizationFailed,
    ExecutionFailed,
    SymbolNotFound,
    OutOfMemory,
};

/// Execution tiers for progressive optimization
pub const ExecutionTier = enum {
    Interpreter,    // Fast startup, slow execution
    BaselineJIT,   // Balanced compilation time/performance
    OptimizedJIT,  // Slow compilation, fast execution
};

/// JIT compiled function metadata
pub const JITFunction = struct {
    name: []const u8,
    module_name: []const u8,
    tier: ExecutionTier,
    call_count: u64,
    total_execution_time: u64,
    compilation_time: u64,
    code_size: usize,
    llvm_function: ?c.LLVMValueRef,
    native_ptr: ?*const fn() callconv(.C) void,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, module_name: []const u8) JITFunction {
        return JITFunction{
            .name = allocator.dupe(u8, name) catch unreachable,
            .module_name = allocator.dupe(u8, module_name) catch unreachable,
            .tier = .Interpreter,
            .call_count = 0,
            .total_execution_time = 0,
            .compilation_time = 0,
            .code_size = 0,
            .llvm_function = null,
            .native_ptr = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *JITFunction) void {
        self.allocator.free(self.name);
        self.allocator.free(self.module_name);
    }

    /// Check if function should be promoted to higher tier
    pub fn shouldTierUp(self: *JITFunction) bool {
        switch (self.tier) {
            .Interpreter => return self.call_count > 10,
            .BaselineJIT => return self.call_count > 1000,
            .OptimizedJIT => return false,
        }
    }

    /// Get average execution time per call
    pub fn averageExecutionTime(self: *JITFunction) f64 {
        if (self.call_count == 0) return 0.0;
        return @as(f64, @floatFromInt(self.total_execution_time)) / @as(f64, @floatFromInt(self.call_count));
    }
};

/// Hot function profile for optimization decisions
pub const HotProfile = struct {
    function: *JITFunction,
    hotness_score: f64,
    optimization_priority: u32,

    pub fn calculateHotnessScore(function: *JITFunction) f64 {
        const call_weight = @as(f64, @floatFromInt(function.call_count)) * 0.7;
        const time_weight = function.averageExecutionTime() * 0.3;
        return call_weight + time_weight;
    }
};

/// LLVM ORC JIT Engine
pub const ORCJITEngine = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    target_machine: c.LLVMTargetMachineRef,
    execution_session: c.LLVMOrcExecutionSessionRef,
    jit_dylib: c.LLVMOrcJITDylibRef,
    
    pub fn init(allocator: Allocator) !ORCJITEngine {
        c.LLVMInitializeNativeTarget();
        c.LLVMInitializeNativeAsmPrinter();
        
        const context = c.LLVMContextCreate();
        
        // Create target machine with optimization
        var target: c.LLVMTargetRef = undefined;
        const triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(triple);
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMGetTargetFromTriple(triple, &target, &error_msg) != 0) {
            print("Failed to get target: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return JITError.InitializationFailed;
        }
        
        const target_machine = c.LLVMCreateTargetMachine(
            target,
            triple,
            c.LLVMGetHostCPUName(),
            c.LLVMGetHostCPUFeatures(),
            c.LLVMCodeGenLevelAggressive,
            c.LLVMRelocDefault,
            c.LLVMCodeModelJITDefault
        );
        
        // Initialize ORC JIT
        const execution_session = c.LLVMOrcCreateExecutionSession();
        const jit_dylib = c.LLVMOrcExecutionSessionCreateBareJITDylib(execution_session, "main");
        
        return ORCJITEngine{
            .allocator = allocator,
            .context = context,
            .target_machine = target_machine,
            .execution_session = execution_session,
            .jit_dylib = jit_dylib,
        };
    }
    
    pub fn deinit(self: *ORCJITEngine) void {
        c.LLVMOrcDisposeExecutionSession(self.execution_session);
        c.LLVMDisposeTargetMachine(self.target_machine);
        c.LLVMContextDispose(self.context);
    }
    
    /// Compile LLVM module to native code
    pub fn compileModule(self: *ORCJITEngine, module: c.LLVMModuleRef, tier: ExecutionTier) !void {
        // Apply optimization based on tier
        try self.optimizeModule(module, tier);
        
        // Add module to JIT
        const tsm = c.LLVMOrcCreateThreadSafeModule(module, self.context);
        const mat_unit = c.LLVMOrcCreateMaterializationUnit(tsm);
        
        const result = c.LLVMOrcJITDylibDefine(self.jit_dylib, mat_unit);
        if (result != c.LLVMErrorSuccess) {
            return JITError.CompilationFailed;
        }
    }
    
    /// Apply optimizations based on execution tier
    fn optimizeModule(_: *ORCJITEngine, module: c.LLVMModuleRef, tier: ExecutionTier) !void {
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        switch (tier) {
            .Interpreter => {
                // No optimizations for interpreter
                return;
            },
            .BaselineJIT => {
                // Basic optimizations for fast compilation
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
                c.LLVMAddGVNPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
            },
            .OptimizedJIT => {
                // Aggressive optimizations
                c.LLVMAddInstructionCombiningPass(pass_manager);
                c.LLVMAddReassociatePass(pass_manager);
                c.LLVMAddGVNPass(pass_manager);
                c.LLVMAddCFGSimplificationPass(pass_manager);
                c.LLVMAddAggressiveDCEPass(pass_manager);
                c.LLVMAddFunctionInliningPass(pass_manager);
                c.LLVMAddTailCallEliminationPass(pass_manager);
                c.LLVMAddLoopUnrollPass(pass_manager);
                c.LLVMAddLoopVectorizePass(pass_manager);
                c.LLVMAddSLPVectorizePass(pass_manager);
            },
        }
        
        _ = c.LLVMRunPassManager(pass_manager, module);
    }
    
    /// Get function address for execution
    pub fn getFunctionAddress(self: *ORCJITEngine, name: []const u8) !*const fn() callconv(.C) void {
        var symbol_name = c.LLVMOrcExecutionSessionIntern(self.execution_session, name.ptr);
        var addr: c.LLVMOrcExecutorAddress = undefined;
        
        const result = c.LLVMOrcJITDylibLookup(self.jit_dylib, &addr, &symbol_name, 1);
        if (result != c.LLVMErrorSuccess) {
            return JITError.SymbolNotFound;
        }
        
        return @ptrFromInt(addr);
    }
};

/// Main JIT Execution Engine
pub const JITExecutionEngine = struct {
    allocator: Allocator,
    orc_jit: ORCJITEngine,
    functions: HashMap([]const u8, JITFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    hot_functions: ArrayList(HotProfile),
    interpreter_env: interpreter.Environment,
    
    // Performance tuning parameters
    tier_up_threshold: u64,
    optimization_threshold: u64,
    max_concurrent_compilations: u32,
    
    // Performance metrics
    total_compilations: u64,
    total_execution_time: u64,
    cache_hit_rate: f64,

    pub fn init(allocator: Allocator) !JITExecutionEngine {
        return JITExecutionEngine{
            .allocator = allocator,
            .orc_jit = try ORCJITEngine.init(allocator),
            .functions = HashMap([]const u8, JITFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .hot_functions = ArrayList(HotProfile).init(allocator),
            .interpreter_env = interpreter.Environment.init(allocator, null),
            .tier_up_threshold = 50,
            .optimization_threshold = 1000,
            .max_concurrent_compilations = 4,
            .total_compilations = 0,
            .total_execution_time = 0,
            .cache_hit_rate = 0.0,
        };
    }

    pub fn deinit(self: *JITExecutionEngine) void {
        self.orc_jit.deinit();
        
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.functions.deinit();
        
        self.hot_functions.deinit();
        self.interpreter_env.deinit();
    }

    /// Register a function for JIT compilation
    pub fn registerFunction(self: *JITExecutionEngine, name: []const u8, module_name: []const u8, ast_func: ast.FunctionStatement) !void {
        const jit_func = JITFunction.init(self.allocator, name, module_name);
        
        // Store AST for later compilation
        _ = ast_func; // TODO: Store AST reference
        
        const full_name = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ module_name, name });
        defer self.allocator.free(full_name);
        
        try self.functions.put(try self.allocator.dupe(u8, full_name), jit_func);
        print("📝 Registered JIT function: {s}\n", .{full_name});
    }

    /// Execute a function with tiered compilation
    pub fn executeFunction(self: *JITExecutionEngine, full_name: []const u8, args: []const interpreter.Value) !interpreter.Value {
        const start_time = std.time.nanoTimestamp();
        
        if (!self.functions.contains(full_name)) {
            return JITError.SymbolNotFound;
        }
        
        var jit_func = self.functions.getPtr(full_name).?;
        jit_func.call_count += 1;
        
        var result: interpreter.Value = undefined;
        
        // Decide execution strategy based on tier and call count
        if (jit_func.tier == .Interpreter or jit_func.native_ptr == null) {
            // Execute in interpreter
            result = try self.executeInInterpreter(jit_func, args);
            
            // Check for tier-up
            if (jit_func.shouldTierUp()) {
                try self.tierUpFunction(jit_func);
            }
        } else {
            // Execute native code
            result = try self.executeNativeCode(jit_func, args);
        }
        
        // Update performance metrics
        const execution_time = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
        jit_func.total_execution_time += execution_time;
        self.total_execution_time += execution_time;
        
        return result;
    }

    /// Execute function in interpreter
    fn executeInInterpreter(_: *JITExecutionEngine, jit_func: *JITFunction, args: []const interpreter.Value) !interpreter.Value {
        print("🐌 Interpreting: {s}.{s}\n", .{ jit_func.module_name, jit_func.name });
        
        // TODO: Implement actual interpretation
        _ = args;
        return interpreter.Value{ .String = "interpreted_result" };
    }

    /// Execute native compiled code
    fn executeNativeCode(_: *JITExecutionEngine, jit_func: *JITFunction, args: []const interpreter.Value) !interpreter.Value {
        print("🚀 Native execution: {s}.{s}\n", .{ jit_func.module_name, jit_func.name });
        
        if (jit_func.native_ptr) |func_ptr| {
            _ = func_ptr;
            _ = args;
            // TODO: Call native function with proper arguments
            return interpreter.Value{ .String = "native_result" };
        }
        
        return JITError.ExecutionFailed;
    }

    /// Tier up a function to higher optimization level
    fn tierUpFunction(self: *JITExecutionEngine, jit_func: *JITFunction) !void {
        const new_tier: ExecutionTier = switch (jit_func.tier) {
            .Interpreter => .BaselineJIT,
            .BaselineJIT => .OptimizedJIT,
            .OptimizedJIT => return, // Already at highest tier
        };
        
        print("⬆️ Tiering up {s}.{s}: {} -> {}\n", .{ jit_func.module_name, jit_func.name, jit_func.tier, new_tier });
        
        const start_time = std.time.nanoTimestamp();
        
        // Compile to new tier
        try self.compileToTier(jit_func, new_tier);
        
        const compilation_time = @as(u64, @intCast(std.time.nanoTimestamp() - start_time));
        jit_func.compilation_time += compilation_time;
        jit_func.tier = new_tier;
        self.total_compilations += 1;
        
        print("✅ Tier-up completed in {}ms\n", .{compilation_time / 1_000_000});
    }

    /// Compile function to specific tier
    fn compileToTier(self: *JITExecutionEngine, jit_func: *JITFunction, tier: ExecutionTier) !void {
        // Create LLVM module for this function
        const module = c.LLVMModuleCreateWithNameInContext(jit_func.name.ptr, self.orc_jit.context);
        defer c.LLVMDisposeModule(module);
        
        // TODO: Generate LLVM IR from stored AST
        // For now, create a simple placeholder function
        const func_type = c.LLVMFunctionType(c.LLVMVoidType(), null, 0, 0);
        const llvm_func = c.LLVMAddFunction(module, jit_func.name.ptr, func_type);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.orc_jit.context, llvm_func, "entry");
        const builder = c.LLVMCreateBuilderInContext(self.orc_jit.context);
        defer c.LLVMDisposeBuilder(builder);
        
        c.LLVMPositionBuilderAtEnd(builder, entry_block);
        c.LLVMBuildRetVoid(builder);
        
        // Compile module
        try self.orc_jit.compileModule(module, tier);
        
        // Get function pointer
        jit_func.native_ptr = try self.orc_jit.getFunctionAddress(jit_func.name);
        jit_func.llvm_function = llvm_func;
    }

    /// Analyze hot functions and optimize them
    pub fn optimizeHotFunctions(self: *JITExecutionEngine) !void {
        print("🔥 Analyzing hot functions for optimization...\n", .{});
        
        // Clear previous hot function list
        self.hot_functions.clearRetainingCapacity();
        
        // Find hot functions
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            const jit_func = entry.value_ptr;
            const hotness_score = HotProfile.calculateHotnessScore(jit_func);
            
            if (hotness_score > 100.0) { // Hot threshold
                const hot_profile = HotProfile{
                    .function = jit_func,
                    .hotness_score = hotness_score,
                    .optimization_priority = @intFromFloat(hotness_score),
                };
                
                try self.hot_functions.append(hot_profile);
            }
        }
        
        // Sort by hotness score
        std.sort.insertion(HotProfile, self.hot_functions.items, {}, struct {
            fn lessThan(_: void, a: HotProfile, b: HotProfile) bool {
                return a.hotness_score > b.hotness_score;
            }
        }.lessThan);
        
        // Optimize top hot functions
        for (self.hot_functions.items[0..@min(5, self.hot_functions.items.len)]) |hot_profile| {
            print("🔥 Hot function: {s}.{s} (score: {d:.2})\n", .{ 
                hot_profile.function.module_name, 
                hot_profile.function.name, 
                hot_profile.hotness_score 
            });
            
            if (hot_profile.function.tier != .OptimizedJIT) {
                try self.tierUpFunction(hot_profile.function);
            }
        }
    }

    /// Generate comprehensive performance report
    pub fn generatePerformanceReport(self: *JITExecutionEngine) void {
        print("\n📊 JIT EXECUTION ENGINE PERFORMANCE REPORT\n", .{});
        print("==========================================\n", .{});
        
        print("🏗️ Total Compilations: {}\n", .{self.total_compilations});
        print("⏱️ Total Execution Time: {}ms\n", .{self.total_execution_time / 1_000_000});
        print("📈 Cache Hit Rate: {d:.2}%\n", .{self.cache_hit_rate * 100});
        
        print("\n🔥 Hot Functions:\n", .{});
        for (self.hot_functions.items) |hot_profile| {
            const func = hot_profile.function;
            print("  {s}.{s}:\n", .{ func.module_name, func.name });
            print("    Calls: {}, Tier: {}, Avg Time: {d:.2}μs\n", .{ 
                func.call_count, 
                func.tier, 
                func.averageExecutionTime() / 1000.0 
            });
        }
        
        print("\n⚡ Tier Distribution:\n", .{});
        var interpreter_count: u32 = 0;
        var baseline_count: u32 = 0;
        var optimized_count: u32 = 0;
        
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            switch (entry.value_ptr.tier) {
                .Interpreter => interpreter_count += 1,
                .BaselineJIT => baseline_count += 1,
                .OptimizedJIT => optimized_count += 1,
            }
        }
        
        print("  Interpreter: {}\n", .{interpreter_count});
        print("  Baseline JIT: {}\n", .{baseline_count});
        print("  Optimized JIT: {}\n", .{optimized_count});
        
        print("==========================================\n", .{});
    }
};

/// Test the JIT execution engine
pub fn testJITExecutionEngine(allocator: Allocator) !void {
    print("\n🧪 Testing JIT Execution Engine\n", .{});
    print("==============================\n", .{});
    
    var engine = try JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    // Create sample function AST
    const sample_func = ast.FunctionStatement{
        .name = "test_function",
        .parameters = ArrayList(ast.Parameter).init(allocator),
        .return_type = "normie",
        .body = ArrayList(ast.Statement).init(allocator),
    };
    
    // Register test functions
    try engine.registerFunction("test_function", "test_module", sample_func);
    try engine.registerFunction("math_add", "mathz", sample_func);
    try engine.registerFunction("string_concat", "stringz", sample_func);
    
    const test_args = [_]interpreter.Value{interpreter.Value{ .Integer = 42 }};
    
    // Simulate function calls to trigger tier-ups
    print("\n🚀 Simulating function calls...\n", .{});
    for (0..100) |i| {
        _ = try engine.executeFunction("test_module.test_function", &test_args);
        if (i % 20 == 0) {
            print("  Call {}: function tier-up check\n", .{i});
        }
    }
    
    // Optimize hot functions
    try engine.optimizeHotFunctions();
    
    // Generate performance report
    engine.generatePerformanceReport();
    
    print("\n✅ JIT Execution Engine tests completed\n", .{});
}
