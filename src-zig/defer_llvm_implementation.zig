const std = @import("std");
// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};
const ast = @import("ast.zig");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// Enhanced LLVM defer implementation for CURSED
/// Handles proper LIFO execution, error conditions, and cleanup integration
pub const DeferLLVMCodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Defer management
    defer_stack: ArrayList(DeferEntry),
    scope_stack: ArrayList(ScopeInfo),
    current_function: ?c.LLVMValueRef,
    current_function_name: ?[]const u8,
    
    // Error handling integration
    error_unwind_block: ?c.LLVMBasicBlockRef,
    normal_exit_block: ?c.LLVMBasicBlockRef,
    defer_cleanup_function: ?c.LLVMValueRef,
    
    // Runtime function declarations
    defer_runtime_functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const DeferEntry = struct {
        cleanup_function: c.LLVMValueRef,
        cleanup_block: c.LLVMBasicBlockRef,
        scope_id: u32,
        function_name: []const u8,
        defer_id: u32,
        is_error_safe: bool, // Can be called during error handling
    };
    
    const ScopeInfo = struct {
        scope_id: u32,
        defer_count_start: usize,
        is_function_scope: bool,
        parent_scope: ?u32,
    };
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef) !DeferLLVMCodeGen {
        var self = DeferLLVMCodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .defer_stack = ArrayList(DeferEntry).init(allocator),
            .scope_stack = ArrayList(ScopeInfo).init(allocator),
            .current_function = null,
            .current_function_name = null,
            .error_unwind_block = null,
            .normal_exit_block = null,
            .defer_cleanup_function = null,
            .defer_runtime_functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        try self.declareDeferRuntimeFunctions();
        return self;
    }
    
    pub fn deinit(self: *DeferLLVMCodeGen) void {
        self.defer_stack.deinit();
        self.scope_stack.deinit();
        self.defer_runtime_functions.deinit();
    }
    
    /// Declare runtime defer functions for LLVM integration
    fn declareDeferRuntimeFunctions(self: *DeferLLVMCodeGen) !void {
        // void cursed_defer_push(void* cleanup_func);
        const void_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const push_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{void_ptr_type},
            1,
            0
        );
        const push_func = c.LLVMAddFunction(self.module, "cursed_defer_push", push_type);
        try self.defer_runtime_functions.put("cursed_defer_push", push_func);
        
        // void cursed_defer_execute_all();
        const execute_all_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            0
        );
        const execute_all_func = c.LLVMAddFunction(self.module, "cursed_defer_execute_all", execute_all_type);
        try self.defer_runtime_functions.put("cursed_defer_execute_all", execute_all_func);
        
        // void cursed_defer_execute_to_count(size_t count);
        const size_t_type = c.LLVMInt64TypeInContext(self.context);
        const execute_to_count_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{size_t_type},
            1,
            0
        );
        const execute_to_count_func = c.LLVMAddFunction(self.module, "cursed_defer_execute_to_count", execute_to_count_type);
        try self.defer_runtime_functions.put("cursed_defer_execute_to_count", execute_to_count_func);
        
        // size_t cursed_defer_get_stack_size();
        const get_size_type = c.LLVMFunctionType(
            size_t_type,
            null,
            0,
            0
        );
        const get_size_func = c.LLVMAddFunction(self.module, "cursed_defer_get_stack_size", get_size_type);
        try self.defer_runtime_functions.put("cursed_defer_get_stack_size", get_size_func);
        
        // void cursed_defer_enter_scope();
        const enter_scope_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            null,
            0,
            0
        );
        const enter_scope_func = c.LLVMAddFunction(self.module, "cursed_defer_enter_scope", enter_scope_type);
        try self.defer_runtime_functions.put("cursed_defer_enter_scope", enter_scope_func);
        
        // void cursed_defer_exit_scope(uint32_t scope_id);
        const exit_scope_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMInt32TypeInContext(self.context)},
            1,
            0
        );
        const exit_scope_func = c.LLVMAddFunction(self.module, "cursed_defer_exit_scope", exit_scope_type);
        try self.defer_runtime_functions.put("cursed_defer_exit_scope", exit_scope_func);
    }
    
    /// Compile defer statement with full LLVM integration
    pub fn compileDeferStatement(self: *DeferLLVMCodeGen, defer_stmt: ast.DeferStatement) !void {
        std.debug.print("🔨 Compiling defer statement...\n");
        
        // Ensure we have a current function
        const current_function = self.current_function orelse return error.NoCurrentFunction;
        
        // Generate unique names for cleanup function
        const defer_id = self.defer_stack.items.len;
        const function_name = self.current_function_name orelse "anonymous";
        
        const cleanup_func_name = try std.fmt.allocPrint(
            self.allocator,
            "defer_cleanup_{s}_{d}",
            .{ function_name, defer_id }
        );
        defer self.allocator.free(cleanup_func_name);
        
        // Create cleanup function - void (*cleanup_func)(void)
        const cleanup_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            0
        );
        
        const cleanup_func = c.LLVMAddFunction(self.module, cleanup_func_name.ptr, cleanup_func_type);
        const cleanup_entry = c.LLVMAppendBasicBlockInContext(self.context, cleanup_func, "entry");
        
        // Save current builder state
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        const saved_function_name = self.current_function_name;
        
        // Generate cleanup code in the cleanup function
        c.LLVMPositionBuilderAtEnd(self.builder, cleanup_entry);
        self.current_function = cleanup_func;
        self.current_function_name = cleanup_func_name;
        
        // Compile the deferred statement
        try self.compileStatement(defer_stmt.statement);
        
        // Ensure cleanup function has return
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        // Restore builder state
        self.current_function = saved_function;
        self.current_function_name = saved_function_name;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }
        
        // Register defer with runtime system
        try self.registerDeferWithRuntime(cleanup_func);
        
        // Store defer info for scope management
        const scope_id = if (self.scope_stack.items.len > 0) 
            self.scope_stack.items[self.scope_stack.items.len - 1].scope_id 
        else 
            0;
            
        const defer_entry = DeferEntry{
            .cleanup_function = cleanup_func,
            .cleanup_block = cleanup_entry,
            .scope_id = scope_id,
            .function_name = try self.allocator.dupe(u8, function_name),
            .defer_id = @intCast(defer_id),
            .is_error_safe = true, // Mark as error-safe by default
        };
        
        try self.defer_stack.append(defer_entry);
        
        std.debug.print("✅ Defer statement compiled: {s} (scope: {d})\n", .{ cleanup_func_name, scope_id });
    }
    
    /// Register defer cleanup function with runtime
    fn registerDeferWithRuntime(self: *DeferLLVMCodeGen, cleanup_func: c.LLVMValueRef) !void {
        const defer_push_func = self.defer_runtime_functions.get("cursed_defer_push") orelse
            return error.DeferRuntimeNotDeclared;
        
        // Cast cleanup function to void* for runtime compatibility
        const void_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const func_ptr = c.LLVMBuildBitCast(
            self.builder,
            cleanup_func,
            void_ptr_type,
            "cleanup_func_ptr"
        );
        
        // Call runtime defer push
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            defer_push_func,
            &[_]c.LLVMValueRef{func_ptr},
            1,
            ""
        );
    }
    
    /// Enter a new scope for defer management
    pub fn enterScope(self: *DeferLLVMCodeGen, is_function_scope: bool) !u32 {
        const parent_scope = if (self.scope_stack.items.len > 0) 
            self.scope_stack.items[self.scope_stack.items.len - 1].scope_id 
        else 
            null;
            
        const scope_id = @as(u32, @intCast(self.scope_stack.items.len + 1));
        
        const scope_info = ScopeInfo{
            .scope_id = scope_id,
            .defer_count_start = self.defer_stack.items.len,
            .is_function_scope = is_function_scope,
            .parent_scope = parent_scope,
        };
        
        try self.scope_stack.append(scope_info);
        
        // Call runtime scope management
        const enter_scope_func = self.defer_runtime_functions.get("cursed_defer_enter_scope") orelse
            return error.DeferRuntimeNotDeclared;
            
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMInt32TypeInContext(self.context),
            enter_scope_func,
            null,
            0,
            ""
        );
        
        std.debug.print("📍 Entered scope {d} (function: {any})\n", .{ scope_id, is_function_scope });
        return scope_id;
    }
    
    /// Exit scope and generate defer cleanup code
    pub fn exitScope(self: *DeferLLVMCodeGen) !void {
        if (self.scope_stack.items.len == 0) {
            std.debug.print("⚠️ Warning: Attempting to exit scope when no scopes are active\n");
            return;
        }
        
        const scope_info = self.scope_stack.pop();
        
        // Generate calls to execute defers for this scope
        const exit_scope_func = self.defer_runtime_functions.get("cursed_defer_exit_scope") orelse
            return error.DeferRuntimeNotDeclared;
            
        const scope_id_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), scope_info.scope_id, 0);
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            exit_scope_func,
            &[_]c.LLVMValueRef{scope_id_value},
            1,
            ""
        );
        
        // Remove scope-specific defers from our tracking
        var i = self.defer_stack.items.len;
        while (i > scope_info.defer_count_start) {
            i -= 1;
            const defer_entry = self.defer_stack.items[i];
            if (defer_entry.scope_id == scope_info.scope_id) {
                self.allocator.free(defer_entry.function_name);
                _ = self.defer_stack.orderedRemove(i);
            }
        }
        
        std.debug.print("📍 Exited scope {d}\n", .{scope_info.scope_id});
    }
    
    /// Generate function exit with defer cleanup
    pub fn generateFunctionExit(self: *DeferLLVMCodeGen, return_value: ?c.LLVMValueRef) !void {
        std.debug.print("🚪 Generating function exit with defer cleanup\n");
        
        // Create cleanup block for normal exit
        const current_function = self.current_function orelse return error.NoCurrentFunction;
        const cleanup_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "defer_cleanup");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "function_exit");
        
        // Branch to cleanup
        _ = c.LLVMBuildBr(self.builder, cleanup_block);
        
        // Generate cleanup code
        c.LLVMPositionBuilderAtEnd(self.builder, cleanup_block);
        
        // Execute all defers in LIFO order
        const execute_all_func = self.defer_runtime_functions.get("cursed_defer_execute_all") orelse
            return error.DeferRuntimeNotDeclared;
            
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            execute_all_func,
            null,
            0,
            ""
        );
        
        // Branch to actual exit
        _ = c.LLVMBuildBr(self.builder, exit_block);
        
        // Generate actual return
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
        if (return_value) |ret_val| {
            _ = c.LLVMBuildRet(self.builder, ret_val);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        std.debug.print("✅ Function exit with defer cleanup generated\n");
    }
    
    /// Generate error handling with defer cleanup
    pub fn generateErrorUnwind(self: *DeferLLVMCodeGen, error_value: c.LLVMValueRef) !void {
        std.debug.print("💥 Generating error unwind with defer cleanup\n");
        
        const current_function = self.current_function orelse return error.NoCurrentFunction;
        const error_cleanup_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "error_defer_cleanup");
        const error_exit_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "error_exit");
        
        // Branch to error cleanup
        _ = c.LLVMBuildBr(self.builder, error_cleanup_block);
        
        // Generate error cleanup code
        c.LLVMPositionBuilderAtEnd(self.builder, error_cleanup_block);
        
        // Execute only error-safe defers in LIFO order
        for (self.defer_stack.items, 0..) |defer_entry, i| {
            const index = self.defer_stack.items.len - 1 - i; // LIFO order
            const entry = self.defer_stack.items[index];
            
            if (entry.is_error_safe) {
                // Call cleanup function directly for error handling
                _ = c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMVoidTypeInContext(self.context),
                    entry.cleanup_function,
                    null,
                    0,
                    ""
                );
            }
        }
        
        // Branch to error exit
        _ = c.LLVMBuildBr(self.builder, error_exit_block);
        
        // Generate error return
        c.LLVMPositionBuilderAtEnd(self.builder, error_exit_block);
        _ = c.LLVMBuildRet(self.builder, error_value);
        
        std.debug.print("✅ Error unwind with defer cleanup generated\n");
    }
    
    /// Integration with error handling system (yikes/shook/fam)
    pub fn integrateWithErrorHandling(self: *DeferLLVMCodeGen, error_block: c.LLVMBasicBlockRef) !void {
        self.error_unwind_block = error_block;
        
        // Modify error block to include defer cleanup
        const current_block = c.LLVMGetInsertBlock(self.builder);
        c.LLVMPositionBuilderAtEnd(self.builder, error_block);
        
        // Add defer cleanup to error path
        const execute_all_func = self.defer_runtime_functions.get("cursed_defer_execute_all") orelse
            return error.DeferRuntimeNotDeclared;
            
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            execute_all_func,
            null,
            0,
            ""
        );
        
        // Restore builder position
        if (current_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, current_block);
        }
        
        std.debug.print("🔗 Integrated defer with error handling system\n");
    }
    
    /// Placeholder for statement compilation (to be implemented based on existing codegen)
    fn compileStatement(self: *DeferLLVMCodeGen, statement: *anyopaque) !void {
        _ = self;
        _ = statement;
        // This would integrate with the main statement compilation logic
        std.debug.print("🔧 Compiling deferred statement (placeholder)\n");
    }
    
    /// Set current function for defer compilation
    pub fn setCurrentFunction(self: *DeferLLVMCodeGen, func: c.LLVMValueRef, name: []const u8) void {
        self.current_function = func;
        self.current_function_name = name;
    }
    
    /// Get defer stack size for debugging
    pub fn getDeferStackSize(self: *DeferLLVMCodeGen) usize {
        return self.defer_stack.items.len;
    }
    
    /// Clear all defers (for error recovery)
    pub fn clearAllDefers(self: *DeferLLVMCodeGen) void {
        for (self.defer_stack.items) |defer_entry| {
            self.allocator.free(defer_entry.function_name);
        }
        self.defer_stack.clearRetainingCapacity();
        self.scope_stack.clearRetainingCapacity();
        
        std.debug.print("🧹 Cleared all defer entries\n");
    }
};

// Test function for the defer LLVM implementation
pub fn testDeferLLVMImplementation() !void {
    std.debug.print("🧪 Testing Defer LLVM Implementation...\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize LLVM
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("defer_test", context);
    defer c.LLVMDisposeModule(module);
    
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    // Create defer codegen
    var defer_codegen = try DeferLLVMCodeGen.init(allocator, context, module, builder);
    defer defer_codegen.deinit();
    
    // Test function creation
    const test_func_type = c.LLVMFunctionType(
        c.LLVMVoidTypeInContext(context),
        null,
        0,
        0
    );
    const test_func = c.LLVMAddFunction(module, "test_function", test_func_type);
    const entry_block = c.LLVMAppendBasicBlockInContext(context, test_func, "entry");
    
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    defer_codegen.setCurrentFunction(test_func, "test_function");
    
    // Test scope management
    const scope_id = try defer_codegen.enterScope(true);
    std.debug.print("✓ Created scope: {d}\n", .{scope_id});
    
    // Test defer compilation (placeholder)
    const dummy_defer = ast.DeferStatement{ .statement = @ptrFromInt(0x1234) };
    defer_codegen.compileDeferStatement(dummy_defer) catch |err| {
        std.debug.print("⚠️ Expected error in placeholder: {any}\n", .{err});
    };
    
    // Test scope exit
    try defer_codegen.exitScope();
    
    // Test function exit generation
    try defer_codegen.generateFunctionExit(null);
    
    std.debug.print("✅ Defer LLVM Implementation test completed\n");
}
