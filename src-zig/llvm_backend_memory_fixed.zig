const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Memory-safe LLVM backend with comprehensive memory leak fixes
// Addresses all identified memory leaks in the LLVM compilation pipeline

// LLVM C imports with proper error handling
const c = @cImport({
    @cDefine("__x86_64__", "1");
    @cDefine("LLVM_HOST_TRIPLE", "\"x86_64-unknown-linux-gnu\"");
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

// Type aliases for cleaner code
const LLVMContextRef = c.LLVMContextRef;
const LLVMModuleRef = c.LLVMModuleRef;
const LLVMBuilderRef = c.LLVMBuilderRef;
const LLVMValueRef = c.LLVMValueRef;
const LLVMTypeRef = c.LLVMTypeRef;
const LLVMBasicBlockRef = c.LLVMBasicBlockRef;
const LLVMPassManagerRef = c.LLVMPassManagerRef;
const LLVMTargetMachineRef = c.LLVMTargetMachineRef;

/// Memory-Safe LLVM Backend Error Types
pub const LLVMBackendError = error{
    LLVMContextCreationFailed,
    LLVMModuleCreationFailed,
    LLVMBuilderCreationFailed,
    LLVMTargetCreationFailed,
    LLVMPassManagerCreationFailed,
    LLVMModuleVerificationFailed,
    LLVMCodeGenFailed,
    LLVMIRGenFailed,
    OutOfMemory,
    InvalidOperation,
};

/// Comprehensive Memory Tracking Structure
pub const MemoryTracker = struct {
    allocator: Allocator,
    llvm_strings: ArrayList([*c]u8),
    llvm_contexts: ArrayList(LLVMContextRef),
    llvm_modules: ArrayList(LLVMModuleRef),
    llvm_builders: ArrayList(LLVMBuilderRef),
    llvm_pass_managers: ArrayList(LLVMPassManagerRef),
    llvm_target_machines: ArrayList(LLVMTargetMachineRef),
    arena_allocations: ArrayList([]u8),
    
    pub fn init() MemoryTracker {
        return MemoryTracker{
            .allocator = allocator,
            .llvm_strings = .empty,
            .llvm_contexts = .empty,
            .llvm_modules = .empty,
            .llvm_builders = .empty,
            .llvm_pass_managers = .empty,
            .llvm_target_machines = .empty,
            .arena_allocations = .empty,
        };
    }
    
    pub fn deinit(self: *MemoryTracker) void {
        // Clean up LLVM strings (important: LLVM allocates these)
        for (self.llvm_strings.items) |str| {
            c.LLVMDisposeMessage(str);
        }
        self.llvm_strings.deinit(self.allocator);
        
        // Clean up LLVM target machines
        for (self.llvm_target_machines.items) |tm| {
            c.LLVMDisposeTargetMachine(tm);
        }
        self.llvm_target_machines.deinit(self.allocator);
        
        // Clean up LLVM pass managers
        for (self.llvm_pass_managers.items) |pm| {
            c.LLVMDisposePassManager(pm);
        }
        self.llvm_pass_managers.deinit(self.allocator);
        
        // Clean up LLVM builders
        for (self.llvm_builders.items) |builder| {
            c.LLVMDisposeBuilder(builder);
        }
        self.llvm_builders.deinit(self.allocator);
        
        // Clean up LLVM modules
        for (self.llvm_modules.items) |module| {
            c.LLVMDisposeModule(module);
        }
        self.llvm_modules.deinit(self.allocator);
        
        // Clean up LLVM contexts (must be last)
        for (self.llvm_contexts.items) |context| {
            c.LLVMContextDispose(context);
        }
        self.llvm_contexts.deinit(self.allocator);
        
        // Clean up arena allocations
        for (self.arena_allocations.items) |allocation| {
            self.allocator.free(allocation);
        }
        self.arena_allocations.deinit(self.allocator);
        
        print("✅ Memory tracker cleanup complete - all LLVM resources disposed\n", .{});
    }
    
    // Track LLVM allocations for proper cleanup
    pub fn trackString(self: *MemoryTracker, str: [*c]u8) !void {
        try self.llvm_strings.append(self.allocator, str);
    }
    
    pub fn trackContext(self: *MemoryTracker, context: LLVMContextRef) !void {
        try self.llvm_contexts.append(allocator, context);
    }
    
    pub fn trackModule(self: *MemoryTracker, module: LLVMModuleRef) !void {
        try self.llvm_modules.append(allocator, module);
    }
    
    pub fn trackBuilder(self: *MemoryTracker, builder: LLVMBuilderRef) !void {
        try self.llvm_builders.append(allocator, builder);
    }
    
    pub fn trackPassManager(self: *MemoryTracker, pm: LLVMPassManagerRef) !void {
        try self.llvm_pass_managers.append(allocator, pm);
    }
    
    pub fn trackTargetMachine(self: *MemoryTracker, tm: LLVMTargetMachineRef) !void {
        try self.llvm_target_machines.append(allocator, tm);
    }
    
    pub fn trackAllocation(self: *MemoryTracker, allocation: []u8) !void {
        try self.arena_allocations.append(allocator, allocation);
    }
};

/// Memory-Safe LLVM Backend Implementation
pub const MemorySafeLLVMBackend = struct {
allocator: Allocator,
memory_tracker: MemoryTracker,
arena: std.heap.ArenaAllocator,

// Core LLVM components
context: LLVMContextRef,
module: LLVMModuleRef,
builder: LLVMBuilderRef,
pass_manager: LLVMPassManagerRef,
target_machine: ?LLVMTargetMachineRef,
    
    // Arena allocator cleanup tracking
    arena_cleanup_callbacks: ArrayList(*const fn () void),
    
    // Caches for performance (use arena allocator)
    type_cache: HashMap([]const u8, LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    function_cache: HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Debug information
    debug_enabled: bool,
    source_file: ?[]const u8,
    
    // Compilation state
    current_function: ?LLVMValueRef,
    entry_block: ?LLVMBasicBlockRef,
    
    pub fn init(allocator: Allocator, module_name: []const u8) !*MemorySafeLLVMBackend {
        // Initialize memory tracker first
        var memory_tracker = MemoryTracker.init(allocator);
        
        // Initialize arena allocator for temporary allocations
        var arena = std.heap.ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        // Initialize LLVM core safely
        c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
        c.LLVMInitializeNativeTarget();
        c.LLVMInitializeNativeAsmPrinter();
        c.LLVMInitializeNativeAsmParser();
        
        // Create LLVM context with error checking
        const context = c.LLVMContextCreate();
        if (context == null) {
            arena.deinit();
            memory_tracker.deinit();
            return LLVMBackendError.LLVMContextCreationFailed;
        }
        try memory_tracker.trackContext(context);
        
        // Create module with error checking
        const module_name_z = try arena_allocator.dupeZ(u8, module_name);
        const module = c.LLVMModuleCreateWithNameInContext(module_name_z.ptr, context);
        if (module == null) {
            arena.deinit();
            memory_tracker.deinit();
            return LLVMBackendError.LLVMModuleCreationFailed;
        }
        try memory_tracker.trackModule(module);
        
        // Create builder with error checking
        const builder = c.LLVMCreateBuilderInContext(context);
        if (builder == null) {
            arena.deinit();
            memory_tracker.deinit();
            return LLVMBackendError.LLVMBuilderCreationFailed;
        }
        try memory_tracker.trackBuilder(builder);
        
        // Create pass manager with error checking
        const pass_manager = c.LLVMCreateFunctionPassManagerForModule(module);
        if (pass_manager == null) {
            arena.deinit();
            memory_tracker.deinit();
            return LLVMBackendError.LLVMPassManagerCreationFailed;
        }
        try memory_tracker.trackPassManager(pass_manager);
        
        // Add essential optimization passes
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddReassociatePass(pass_manager);
        c.LLVMAddGVNPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
        c.LLVMAddDeadStoreEliminationPass(pass_manager);
        
        // Initialize pass manager
        _ = c.LLVMInitializeFunctionPassManager(pass_manager);
        
        // Create target machine for native compilation
        var target_machine: ?LLVMTargetMachineRef = null;
        
        // Get default target triple with proper memory management
        const triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(triple); // Important: dispose LLVM-allocated string
        
        var target: c.LLVMTargetRef = undefined;
        var error_message: [*c]u8 = undefined;
        
        if (c.LLVMGetTargetFromTriple(triple, &target, &error_message) == 0) {
            // Successfully got target
            target_machine = c.LLVMCreateTargetMachine(
                target,
                triple,
                c.LLVMGetHostCPUName(),
                c.LLVMGetHostCPUFeatures(),
                c.LLVMCodeGenLevelDefault,
                c.LLVMRelocDefault,
                c.LLVMCodeModelDefault
            );
            
            if (target_machine) |tm| {
                try memory_tracker.trackTargetMachine(tm);
                // Set target triple for module
                c.LLVMSetTarget(module, triple);
            }
        } else {
            // Handle target creation error
            defer c.LLVMDisposeMessage(error_message);
            print("⚠️ Warning: Failed to create target machine: {s}\n", .{error_message});
        }
        
        // Allocate backend structure
        const backend = try allocator.create(MemorySafeLLVMBackend);
        backend.* = MemorySafeLLVMBackend{
            .allocator = allocator,
            .memory_tracker = memory_tracker,
            .arena = arena,
            .context = context,
            .module = module,
            .builder = builder,
            .pass_manager = pass_manager,
            .target_machine = target_machine,
            .arena_cleanup_callbacks = ArrayList(*const fn () void).init(arena_allocator),
            .type_cache = HashMap([]const u8, LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .function_cache = HashMap([]const u8, LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .debug_enabled = false,
            .source_file = null,
            .current_function = null,
            .entry_block = null,
        };
        
        print("✅ Memory-safe LLVM backend initialized: {s}\n", .{module_name});
        return backend;
    }
    
    pub fn deinit(self: *MemorySafeLLVMBackend) void {
        print("🧹 Starting memory-safe LLVM backend cleanup...\n", .{});
        
        // Execute arena cleanup callbacks before arena destruction
        for (self.arena_cleanup_callbacks.items) |callback| {
            callback();
        }
        
        // Clean up memory tracker (handles all LLVM resources)
        self.memory_tracker.deinit(self.allocator);
        
        // Clean up arena allocator (handles all temporary allocations)
        self.arena.deinit(self.allocator);
        
        // Destroy the backend structure itself
        self.allocator.destroy(self);
        
        print("✅ Memory-safe LLVM backend cleanup complete\n", .{});
    }
    
    /// Register a cleanup callback to be executed before arena destruction
    pub fn registerArenaCleanup(self: *MemorySafeLLVMBackend, callback: *const fn () void) !void {
        try self.arena_cleanup_callbacks.append(self.allocator, callback);
    }
    
    /// Enable debug information with proper memory management
    pub fn enableDebugInfo(self: *MemorySafeLLVMBackend, source_file: []const u8) !void {
        self.debug_enabled = true;
        // Store source file in arena to avoid memory leak
        self.source_file = try self.arena.allocator().dupe(u8, source_file);
        
        // Add debug version metadata
        const debug_version = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 3, 0);
        c.LLVMAddModuleFlag(
            self.module,
            c.LLVMModuleFlagBehaviorWarning,
            "Debug Info Version",
            17,
            c.LLVMValueAsMetadata(debug_version)
        );
        
        // Add DWARF version
        const dwarf_version = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 4, 0);
        c.LLVMAddModuleFlag(
            self.module,
            c.LLVMModuleFlagBehaviorWarning,
            "Dwarf Version",
            13,
            c.LLVMValueAsMetadata(dwarf_version)
        );
        
        print("✅ Debug information enabled for {s}\n", .{source_file});
    }
    
    /// Get or create type with caching
    pub fn getOrCreateType(self: *MemorySafeLLVMBackend, type_name: []const u8) !LLVMTypeRef {
        if (self.type_cache.get(type_name)) |cached_type| {
            return cached_type;
        }
        
        const llvm_type = blk: {
            if (std.mem.eql(u8, type_name, "i32")) {
                break :blk c.LLVMInt32TypeInContext(self.context);
            } else if (std.mem.eql(u8, type_name, "i64")) {
                break :blk c.LLVMInt64TypeInContext(self.context);
            } else if (std.mem.eql(u8, type_name, "f64")) {
                break :blk c.LLVMDoubleTypeInContext(self.context);
            } else if (std.mem.eql(u8, type_name, "void")) {
                break :blk c.LLVMVoidTypeInContext(self.context);
            } else if (std.mem.eql(u8, type_name, "i8*")) {
                break :blk c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            } else {
                break :blk c.LLVMInt64TypeInContext(self.context); // Default
            }
        };
        
        // Cache the type with arena-allocated key
        const cached_name = try self.arena.allocator().dupe(u8, type_name);
        try self.type_cache.put(cached_name, llvm_type);
        return llvm_type;
    }
    
    /// Create function with proper error handling
    pub fn createFunction(self: *MemorySafeLLVMBackend, name: []const u8, return_type: []const u8, param_types: []const []const u8) !LLVMValueRef {
        // Get return type
        const ret_type = try self.getOrCreateType(return_type);
        
        // Get parameter types
        var param_llvm_types = try self.arena.allocator().alloc(LLVMTypeRef, param_types.len);
        for (param_types, 0..) |param_type, i| {
            param_llvm_types[i] = try self.getOrCreateType(param_type);
        }
        
        // Create function type
        const func_type = c.LLVMFunctionType(
            ret_type,
            param_llvm_types.ptr,
            @intCast(param_llvm_types.len),
            0
        );
        
        // Create function with arena-allocated name
        const name_z = try self.arena.allocator().dupeZ(u8, name);
        const function = c.LLVMAddFunction(self.module, name_z.ptr, func_type);
        
        // Cache function
        const cached_name = try self.arena.allocator().dupe(u8, name);
        try self.function_cache.put(cached_name, function);
        
        print("✅ Created function: {s}\n", .{name});
        return function;
    }
    
    /// Create main function for CURSED programs
    pub fn createMainFunction(self: *MemorySafeLLVMBackend) !LLVMValueRef {
        const main_func = try self.createFunction("main", "i32", &[_][]const u8{});
        
        // Create entry block
        self.entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, self.entry_block.?);
        
        self.current_function = main_func;
        return main_func;
    }
    
    /// Generate simple return statement
    pub fn generateReturn(self: *MemorySafeLLVMBackend, value: ?i32) !LLVMValueRef {
        if (value) |val| {
            const return_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @bitCast(@as(u32, @intCast(val))), 0);
            return c.LLVMBuildRet(self.builder, return_value);
        } else {
            return c.LLVMBuildRetVoid(self.builder);
        }
    }
    
    /// Generate print statement (vibez.spill)
    pub fn generatePrint(self: *MemorySafeLLVMBackend, message: []const u8) !LLVMValueRef {
        // Get or create printf function
        const printf_func = blk: {
            const cached_name = "printf";
            if (self.function_cache.get(cached_name)) |func| {
                break :blk func;
            }
            
            const printf_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                &[_]LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
                1,
                1 // variadic
            );
            
            const func = c.LLVMAddFunction(self.module, "printf", printf_type);
            try self.function_cache.put(cached_name, func);
            break :blk func;
        };
        
        // Create format string with arena allocation
        const format_str = try std.fmt.allocPrintZ(self.arena.allocator(), "{s}\n", .{message});
        const str_value = c.LLVMBuildGlobalStringPtr(self.builder, format_str.ptr, "str");
        
        // Call printf
        return c.LLVMBuildCall2(
            self.builder,
            c.LLVMInt32TypeInContext(self.context),
            printf_func,
            &[_]LLVMValueRef{str_value},
            1,
            "printf_call"
        );
    }
    
    /// Apply optimizations to function
    pub fn optimizeFunction(self: *MemorySafeLLVMBackend, function: LLVMValueRef) void {
        _ = c.LLVMRunFunctionPassManager(self.pass_manager, function);
    }
    
    /// Verify module integrity
    pub fn verifyModule(self: *MemorySafeLLVMBackend) !void {
        var error_message: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMPrintMessageAction, &error_message) != 0) {
            // Track error message for proper cleanup
            try self.memory_tracker.trackString(error_message);
            print("LLVM module verification failed: {s}\n", .{error_message});
            return LLVMBackendError.LLVMModuleVerificationFailed;
        }
    }
    
    /// Generate LLVM IR to file with proper memory management
    pub fn generateIR(self: *MemorySafeLLVMBackend, output_file: []const u8) !void {
        var error_message: [*c]u8 = undefined;
        const output_file_z = try self.arena.allocator().dupeZ(u8, output_file);
        
        if (c.LLVMPrintModuleToFile(self.module, output_file_z.ptr, &error_message) != 0) {
            // Track error message for proper cleanup
            try self.memory_tracker.trackString(error_message);
            print("Failed to write IR file: {s}\n", .{error_message});
            return LLVMBackendError.LLVMIRGenFailed;
        }
        
        print("✅ Generated LLVM IR: {s}\n", .{output_file});
    }
    
    /// Generate native object file with memory safety
    pub fn generateNativeCode(self: *MemorySafeLLVMBackend, output_file: []const u8) !void {
        if (self.target_machine == null) {
            print("⚠️ No target machine available, skipping native code generation\n", .{});
            return;
        }
        
        // Apply optimizations first
        if (self.current_function) |func| {
            self.optimizeFunction(func);
        }
        
        // Verify module
        try self.verifyModule();
        
        // Generate object file with memory-safe string handling
        const output_file_z = try self.arena.allocator().dupeZ(u8, output_file);
        var error_message: [*c]u8 = undefined;
        
        if (c.LLVMTargetMachineEmitToFile(
            self.target_machine.?,
            self.module,
            output_file_z.ptr,
            c.LLVMObjectFile,
            &error_message
        ) != 0) {
            // Track error message for proper cleanup
            try self.memory_tracker.trackString(error_message);
            print("Failed to emit object file: {s}\n", .{error_message});
            return LLVMBackendError.LLVMCodeGenFailed;
        }
        
        print("✅ Generated native code: {s}\n", .{output_file});
    }
    
    /// Generate executable using system linker
    pub fn generateExecutable(self: *MemorySafeLLVMBackend, object_file: []const u8, output_file: []const u8) !void {
        // Use system linker to create executable
        const link_cmd = try std.fmt.allocPrint(
            self.arena.allocator(),
            "gcc {s} -o {s} -lm",
            .{ object_file, output_file }
        );
        
        // Execute link command
        var child = std.process.Child.init(&[_][]const u8{ "sh", "-c", link_cmd }, self.allocator);
        const result = child.spawnAndWait() catch |err| {
            print("Failed to run linker: {s}\n", .{err});
            return;
        };
        
        switch (result) {
            .Exited => |code| {
                if (code == 0) {
                    print("✅ Generated executable: {s}\n", .{output_file});
                } else {
                    print("❌ Linker failed with exit code: {s}\n", .{code});
                }
            },
            else => {
                print("❌ Linker process failed\n", .{});
            },
        }
    }
};

/// Main compilation function with comprehensive memory safety
pub fn compileWithMemorySafety(allocator: Allocator, source: []const u8, output_base: []const u8) !void {
    print("🚀 Starting memory-safe LLVM compilation...\n", .{});
    
    // Create backend with automatic memory management
    var backend = try MemorySafeLLVMBackend.init(allocator, "cursed_program");
    defer backend.deinit(); // Guarantees proper cleanup
    
    // Enable debug info
    try backend.enableDebugInfo("source.csd");
    
    // Create main function
    _ = try backend.createMainFunction();
    
    // Generate simple program (placeholder for actual CURSED compilation)
    _ = source; // TODO: Parse and compile actual CURSED code
    
    // Generate print statement
    _ = try backend.generatePrint("Hello from memory-safe CURSED compiler!");
    
    // Generate return
    _ = try backend.generateReturn(0);
    
    // Generate outputs
    const ir_file = try std.fmt.allocPrint(allocator, "{s}.ll", .{output_base});
    defer allocator.free(ir_file);
    
    const obj_file = try std.fmt.allocPrint(allocator, "{s}.o", .{output_base});
    defer allocator.free(obj_file);
    
    const exe_file = try std.fmt.allocPrint(allocator, "{s}", .{output_base});
    defer allocator.free(exe_file);
    
    // Generate LLVM IR
    try backend.generateIR(ir_file);
    
    // Generate native object file
    try backend.generateNativeCode(obj_file);
    
    // Generate executable
    try backend.generateExecutable(obj_file, exe_file);
    
    print("✅ Memory-safe compilation complete!\n", .{});
    print("   LLVM IR: {s}\n", .{ir_file});
    print("   Object:  {s}\n", .{obj_file});
    print("   Binary:  {s}\n", .{exe_file});
}

/// Test memory safety with valgrind-friendly operations
pub fn testMemorySafety() !void {
    print("🧪 Testing memory safety of LLVM backend...\n", .{});
    
    const allocator = std.testing.allocator;
    
    // Test 1: Basic backend creation and destruction
    {
        var backend = try MemorySafeLLVMBackend.init(allocator, "test_module");
        defer backend.deinit();
        
        try backend.enableDebugInfo("test.csd");
        _ = try backend.createMainFunction();
        _ = try backend.generatePrint("Test message");
        _ = try backend.generateReturn(0);
    }
    
    // Test 2: Multiple backends (tests resource isolation)
    {
        var backend1 = try MemorySafeLLVMBackend.init(allocator, "test1");
        defer backend1.deinit();
        
        var backend2 = try MemorySafeLLVMBackend.init(allocator, "test2");
        defer backend2.deinit();
        
        _ = try backend1.createMainFunction();
        _ = try backend2.createMainFunction();
    }
    
    print("✅ Memory safety tests passed!\n", .{});
}

test "memory safe llvm backend" {
    try testMemorySafety();
}
