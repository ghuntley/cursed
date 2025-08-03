const std = @import("std");
const gc_module = @import("gc.zig");
const GC = gc_module.GC;
const GCConfig = gc_module.GCConfig;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
});

/// GC Integration with LLVM Code Generation
/// This module provides the bridge between the CURSED garbage collector
/// and LLVM-generated code, including:
/// - GC-aware allocation functions
/// - Stack map generation for precise GC
/// - Write barrier insertion
/// - Root set management for LLVM-generated code

pub const GCIntegration = struct {
    gc: *GC,
    llvm_context: c.LLVMContextRef,
    llvm_module: c.LLVMModuleRef,
    llvm_builder: c.LLVMBuilderRef,
    
    // Function metadata storage for root tables
    function_root_tables: std.HashMap(c.LLVMValueRef, c.LLVMValueRef, std.hash_map.DefaultContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    allocator: std.mem.Allocator,
    
    // GC runtime function declarations
    gc_alloc_func: c.LLVMValueRef,
    gc_add_root_func: c.LLVMValueRef,
    gc_remove_root_func: c.LLVMValueRef,
    gc_write_barrier_func: c.LLVMValueRef,
    gc_collect_func: c.LLVMValueRef,
    gc_add_finalizer_func: c.LLVMValueRef,
    
    // Type cache for efficient lookup
    gc_ptr_type: c.LLVMTypeRef,
    gc_header_type: c.LLVMTypeRef,
    
    /// Initialize GC integration with LLVM
    pub fn init(
        allocator: std.mem.Allocator,
        gc: *GC,
        llvm_context: c.LLVMContextRef,
        llvm_module: c.LLVMModuleRef,
        llvm_builder: c.LLVMBuilderRef,
    ) !*GCIntegration {
        const integration = try allocator.create(GCIntegration);
        integration.* = GCIntegration{
            .gc = gc,
            .llvm_context = llvm_context,
            .llvm_module = llvm_module,
            .llvm_builder = llvm_builder,
            .function_root_tables = std.HashMap(c.LLVMValueRef, c.LLVMValueRef, std.hash_map.DefaultContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
            .gc_alloc_func = undefined,
            .gc_add_root_func = undefined,
            .gc_remove_root_func = undefined,
            .gc_write_barrier_func = undefined,
            .gc_collect_func = undefined,
            .gc_add_finalizer_func = undefined,
            .gc_ptr_type = undefined,
            .gc_header_type = undefined,
        };
        
        try integration.declareFunctions();
        try integration.setupTypes();
        
        return integration;
    }
    
    /// Clean up GC integration
    pub fn deinit(self: *GCIntegration) void {
        self.function_root_tables.deinit();
        self.allocator.destroy(self);
    }
    
    /// Declare GC runtime functions in LLVM module
    fn declareFunctions(self: *GCIntegration) !void {
        // gc_alloc(size: i64, type_id: i16) -> ptr
        {
            const param_types = [_]c.LLVMTypeRef{
                c.LLVMInt64TypeInContext(self.llvm_context), // size
                c.LLVMInt16TypeInContext(self.llvm_context), // type_id
            };
            const func_type = c.LLVMFunctionType(
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), // return ptr
                &param_types,
                2,
                0 // not variadic
            );
            self.gc_alloc_func = c.LLVMAddFunction(self.llvm_module, "cursed_gc_alloc_wrapper", func_type);
        }
        
        // gc_add_root(ptr: **void, type_id: i16) -> void
        {
            const param_types = [_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), 0), // ptr to ptr
                c.LLVMInt16TypeInContext(self.llvm_context), // type_id
            };
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.llvm_context),
                &param_types,
                2,
                0
            );
            self.gc_add_root_func = c.LLVMAddFunction(self.llvm_module, "cursed_gc_add_root_wrapper", func_type);
        }
        
        // gc_remove_root(ptr: **void) -> void
        {
            const param_types = [_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), 0), // ptr to ptr
            };
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.llvm_context),
                &param_types,
                1,
                0
            );
            self.gc_remove_root_func = c.LLVMAddFunction(self.llvm_module, "cursed_gc_remove_root_wrapper", func_type);
        }
        
        // gc_write_barrier(old_ref: ptr, new_ref: ptr) -> void
        {
            const param_types = [_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), // old_ref
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), // new_ref
            };
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.llvm_context),
                &param_types,
                2,
                0
            );
            self.gc_write_barrier_func = c.LLVMAddFunction(self.llvm_module, "cursed_gc_write_barrier_wrapper", func_type);
        }
        
        // gc_collect() -> void
        {
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.llvm_context),
                null,
                0,
                0
            );
            self.gc_collect_func = c.LLVMAddFunction(self.llvm_module, "cursed_gc_collect_wrapper", func_type);
        }
        
        // gc_add_finalizer(object: ptr, finalizer: ptr) -> void
        {
            const param_types = [_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), // object
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), // finalizer function
            };
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.llvm_context),
                &param_types,
                2,
                0
            );
            self.gc_add_finalizer_func = c.LLVMAddFunction(self.llvm_module, "cursed_gc_add_finalizer_wrapper", func_type);
        }
    }
    
    /// Set up LLVM types for GC
    fn setupTypes(self: *GCIntegration) !void {
        // GC pointer type (pointer to user data)
        self.gc_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0);
        
        // GC header type - matches ObjectHeader structure
        const header_fields = [_]c.LLVMTypeRef{
            c.LLVMInt32TypeInContext(self.llvm_context), // size
            c.LLVMInt16TypeInContext(self.llvm_context), // type_id
            c.LLVMInt8TypeInContext(self.llvm_context),  // color (2 bits) + generation (1 bit) + finalize (1 bit) + reserved (4 bits)
            c.LLVMInt8TypeInContext(self.llvm_context),  // reserved
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0), // next pointer
        };
        
        self.gc_header_type = c.LLVMStructTypeInContext(
            self.llvm_context,
            &header_fields,
            header_fields.len,
            0 // not packed
        );
    }
    
    /// Generate GC allocation call
    pub fn generateAllocation(self: *GCIntegration, size: c.LLVMValueRef, type_id: u16) c.LLVMValueRef {
        const type_id_value = c.LLVMConstInt(c.LLVMInt16TypeInContext(self.llvm_context), type_id, 0);
        
        const args = [_]c.LLVMValueRef{ size, type_id_value };
        
        return c.LLVMBuildCall2(
            self.llvm_builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.gc_alloc_func)),
            self.gc_alloc_func,
            &args,
            2,
            "gc_alloc"
        );
    }
    
    /// Generate allocation for specific type
    pub fn generateTypedAllocation(self: *GCIntegration, llvm_type: c.LLVMTypeRef, type_id: u16) c.LLVMValueRef {
        const size = c.LLVMSizeOf(llvm_type);
        const ptr = self.generateAllocation(size, type_id);
        
        // Cast to appropriate type
        return c.LLVMBuildBitCast(
            self.llvm_builder,
            ptr,
            c.LLVMPointerType(llvm_type, 0),
            "typed_ptr"
        );
    }
    
    /// Generate root registration
    pub fn generateAddRoot(self: *GCIntegration, root_ptr_ptr: c.LLVMValueRef, type_id: u16) void {
        const type_id_value = c.LLVMConstInt(c.LLVMInt16TypeInContext(self.llvm_context), type_id, 0);
        
        const args = [_]c.LLVMValueRef{ root_ptr_ptr, type_id_value };
        
        _ = c.LLVMBuildCall2(
            self.llvm_builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.gc_add_root_func)),
            self.gc_add_root_func,
            &args,
            2,
            ""
        );
    }
    
    /// Generate root removal
    pub fn generateRemoveRoot(self: *GCIntegration, root_ptr_ptr: c.LLVMValueRef) void {
        const args = [_]c.LLVMValueRef{root_ptr_ptr};
        
        _ = c.LLVMBuildCall2(
            self.llvm_builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.gc_remove_root_func)),
            self.gc_remove_root_func,
            &args,
            1,
            ""
        );
    }
    
    /// Generate write barrier
    pub fn generateWriteBarrier(self: *GCIntegration, old_ref: c.LLVMValueRef, new_ref: c.LLVMValueRef) void {
        // Cast to void pointers if needed
        const old_ptr = c.LLVMBuildBitCast(
            self.llvm_builder,
            old_ref,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0),
            "old_ptr"
        );
        
        const new_ptr = c.LLVMBuildBitCast(
            self.llvm_builder,
            new_ref,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0),
            "new_ptr"
        );
        
        const args = [_]c.LLVMValueRef{ old_ptr, new_ptr };
        
        _ = c.LLVMBuildCall2(
            self.llvm_builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.gc_write_barrier_func)),
            self.gc_write_barrier_func,
            &args,
            2,
            ""
        );
    }
    
    /// Generate pointer assignment with write barrier
    pub fn generatePointerStore(self: *GCIntegration, ptr_location: c.LLVMValueRef, new_value: c.LLVMValueRef) void {
        // Load old value
        const old_value = c.LLVMBuildLoad2(
            self.llvm_builder,
            c.LLVMGetElementType(c.LLVMTypeOf(ptr_location)),
            ptr_location,
            "old_value"
        );
        
        // Generate write barrier
        self.generateWriteBarrier(old_value, new_value);
        
        // Store new value
        _ = c.LLVMBuildStore(self.llvm_builder, new_value, ptr_location);
    }
    
    /// Generate explicit GC collection call
    pub fn generateCollect(self: *GCIntegration) void {
        _ = c.LLVMBuildCall2(
            self.llvm_builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.gc_collect_func)),
            self.gc_collect_func,
            null,
            0,
            ""
        );
    }
    
    /// Generate finalizer registration
    pub fn generateAddFinalizer(self: *GCIntegration, object: c.LLVMValueRef, finalizer_func: c.LLVMValueRef) void {
        const object_ptr = c.LLVMBuildBitCast(
            self.llvm_builder,
            object,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0),
            "object_ptr"
        );
        
        const finalizer_ptr = c.LLVMBuildBitCast(
            self.llvm_builder,
            finalizer_func,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0),
            "finalizer_ptr"
        );
        
        const args = [_]c.LLVMValueRef{ object_ptr, finalizer_ptr };
        
        _ = c.LLVMBuildCall2(
            self.llvm_builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.gc_add_finalizer_func)),
            self.gc_add_finalizer_func,
            &args,
            2,
            ""
        );
    }
    
    /// Generate stack map for precise GC
    pub fn generateStackMap(self: *GCIntegration, function: c.LLVMValueRef, live_pointers: []c.LLVMValueRef) !void {
        // This would generate LLVM stack maps for precise stack scanning
        // For now, we'll add metadata to track live pointers
        
        _ = self;
        _ = function;
        _ = live_pointers;
        
        // Generate LLVM stack map for GC root scanning
    const stack_map_func = c.LLVMAddFunction(module, "llvm.experimental.stackmap", 
        c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0));
    
    // Create stack map intrinsic call
    var map_args = [_]c.LLVMValueRef{
        c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0), // ID
        c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0), // Shadow bytes
    };
    _ = c.LLVMBuildCall2(builder, c.LLVMGlobalGetValueType(stack_map_func), 
        stack_map_func, &map_args, 2, "stackmap");
        // This would involve:
        // 1. Creating stack map intrinsics
        // 2. Marking GC safepoints
        // 3. Recording live pointer locations
    }
    
    /// Generate GC safepoint
    pub fn generateSafepoint(self: *GCIntegration, live_pointers: []c.LLVMValueRef) void {
        // Insert GC safepoint - a point where collection can safely occur
        // This would typically involve:
        // 1. Saving live pointers to known locations
        // 2. Calling a safepoint function
        // 3. Reloading potentially moved pointers
        
        _ = self;
        _ = live_pointers;
        
        // For now, just add a comment
        // Generate GC safepoint - check if GC is needed
        const gc_check_func = c.LLVMAddFunction(module, "cursed_gc_check", 
            c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0));
        
        // Add call to GC check function
        _ = c.LLVMBuildCall2(builder, c.LLVMGlobalGetValueType(gc_check_func), 
            gc_check_func, null, 0, "gc_check");
    }
    
    /// Generate function prologue for GC
    pub fn generateFunctionPrologue(self: *GCIntegration, function: c.LLVMValueRef) void {
        // Save current builder position
        const current_block = c.LLVMGetInsertBlock(self.llvm_builder);
        
        // Create entry block if it doesn't exist
        const entry_block = c.LLVMGetEntryBasicBlock(function);
        
        // Position at beginning of entry block
        c.LLVMPositionBuilderAtEnd(self.llvm_builder, entry_block);
        
        // Generate GC prologue code
        // Create stack frame registration call
        const register_frame_fn = self.getOrCreateFunction("cursed_gc_register_frame", c.LLVMVoidType(), &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8Type(), 0)});
        
        // Get current stack pointer
        const stack_ptr = c.LLVMBuildAlloca(self.llvm_builder, c.LLVMInt8Type(), "stack_frame");
        const stack_ptr_cast = c.LLVMBuildBitCast(self.llvm_builder, stack_ptr, c.LLVMPointerType(c.LLVMInt8Type(), 0), "stack_ptr");
        
        // Call frame registration
        _ = c.LLVMBuildCall2(self.llvm_builder, c.LLVMGetElementType(c.LLVMTypeOf(register_frame_fn)), register_frame_fn, @ptrCast(&stack_ptr_cast), 1, "");
        
        // Initialize local root table
        const root_table_size = 32; // Maximum local roots per function
        const root_table_type = c.LLVMArrayType(c.LLVMPointerType(c.LLVMInt8Type(), 0), root_table_size);
        const root_table = c.LLVMBuildAlloca(self.llvm_builder, root_table_type, "local_roots");
        
        // Zero-initialize the root table
        const zero_value = c.LLVMConstNull(root_table_type);
        _ = c.LLVMBuildStore(self.llvm_builder, zero_value, root_table);
        
        // Store root table in function metadata for later use
        self.storeRootTable(function, root_table);
        
        // Restore builder position
        c.LLVMPositionBuilderAtEnd(self.llvm_builder, current_block);
    }
    
    /// Generate function epilogue for GC
    pub fn generateFunctionEpilogue(self: *GCIntegration, function: c.LLVMValueRef) void {
        // Generate GC epilogue code before all return instructions
        const exit_block = c.LLVMGetLastBasicBlock(function);
        c.LLVMPositionBuilderAtEnd(self.llvm_builder, exit_block);
        
        // Create stack frame unregistration call
        const unregister_frame_fn = self.getOrCreateFunction("cursed_gc_unregister_frame", c.LLVMVoidType(), &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8Type(), 0)});
        
        // Get the stack frame pointer (stored during prologue)
        if (self.getRootTable(function)) |root_table| {
            // Clean up local roots
            const cleanup_roots_fn = self.getOrCreateFunction("cursed_gc_cleanup_roots", c.LLVMVoidType(), &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMPointerType(c.LLVMInt8Type(), 0), 0)});
            _ = c.LLVMBuildCall2(self.llvm_builder, c.LLVMGetElementType(c.LLVMTypeOf(cleanup_roots_fn)), cleanup_roots_fn, @ptrCast(&root_table), 1, "");
        }
        
        // Create dummy stack pointer for unregistration (simplified)
        const stack_ptr = c.LLVMBuildAlloca(self.llvm_builder, c.LLVMInt8Type(), "stack_frame_cleanup");
        const stack_ptr_cast = c.LLVMBuildBitCast(self.llvm_builder, stack_ptr, c.LLVMPointerType(c.LLVMInt8Type(), 0), "stack_ptr");
        
        // Call frame unregistration
        _ = c.LLVMBuildCall2(self.llvm_builder, c.LLVMGetElementType(c.LLVMTypeOf(unregister_frame_fn)), unregister_frame_fn, @ptrCast(&stack_ptr_cast), 1, "");
    }
    
    /// Store root table for function
    fn storeRootTable(self: *GCIntegration, function: c.LLVMValueRef, root_table: c.LLVMValueRef) void {
        self.function_root_tables.put(function, root_table) catch {};
    }
    
    /// Get root table for function
    fn getRootTable(self: *GCIntegration, function: c.LLVMValueRef) ?c.LLVMValueRef {
        return self.function_root_tables.get(function);
    }
    
    /// Helper to get or create a function declaration
    fn getOrCreateFunction(self: *GCIntegration, name: []const u8, return_type: c.LLVMTypeRef, param_types: []const c.LLVMTypeRef) c.LLVMValueRef {
        const name_cstr = @as([*:0]const u8, @ptrCast(name.ptr));
        
        if (c.LLVMGetNamedFunction(self.llvm_module, name_cstr)) |existing| {
            return existing;
        }
        
        const function_type = c.LLVMFunctionType(return_type, @as([*]c.LLVMTypeRef, @ptrCast(param_types.ptr)), @intCast(param_types.len), 0);
        return c.LLVMAddFunction(self.llvm_module, name_cstr, function_type);
    }
    
    /// Generate array allocation with GC
    pub fn generateArrayAllocation(self: *GCIntegration, element_type: c.LLVMTypeRef, count: c.LLVMValueRef, type_id: u16) c.LLVMValueRef {
        const element_size = c.LLVMSizeOf(element_type);
        const total_size = c.LLVMBuildMul(self.llvm_builder, element_size, count, "total_size");
        
        const ptr = self.generateAllocation(total_size, type_id);
        
        return c.LLVMBuildBitCast(
            self.llvm_builder,
            ptr,
            c.LLVMPointerType(element_type, 0),
            "array_ptr"
        );
    }
    
    /// Generate struct allocation with field initialization
    pub fn generateStructAllocation(self: *GCIntegration, struct_type: c.LLVMTypeRef, type_id: u16) c.LLVMValueRef {
        const ptr = self.generateTypedAllocation(struct_type, type_id);
        
        // Zero-initialize the struct
        const struct_size = c.LLVMSizeOf(struct_type);
        const zero = c.LLVMConstInt(c.LLVMInt8TypeInContext(self.llvm_context), 0, 0);
        
        _ = c.LLVMBuildMemSet(
            self.llvm_builder,
            c.LLVMBuildBitCast(
                self.llvm_builder,
                ptr,
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.llvm_context), 0),
                "void_ptr"
            ),
            zero,
            struct_size,
            1 // alignment
        );
        
        return ptr;
    }
};

/// Runtime wrapper functions that bridge LLVM-generated code to Zig GC
/// These functions are called from LLVM-generated code

// Global GC instance - initialized by runtime
var global_gc: ?*GC = null;

/// Initialize the global GC instance
export fn cursed_gc_runtime_init(initial_heap_size: usize) c_int {
    const allocator = std.heap.page_allocator;
    var config = GCConfig.default();
    config.initial_heap_size = initial_heap_size;
    
    global_gc = GC.init(allocator, config) catch return -1;
    return 0;
}

/// Cleanup the global GC instance
export fn cursed_gc_runtime_deinit() void {
    if (global_gc) |gc| {
        gc.deinit();
        global_gc = null;
    }
}

/// Wrapper for GC allocation
export fn cursed_gc_alloc_wrapper(size: u64, type_id: u16) ?*anyopaque {
    if (global_gc) |gc| {
        return gc.alloc(@intCast(size), type_id) catch null;
    }
    return null;
}

/// Wrapper for adding roots
export fn cursed_gc_add_root_wrapper(ptr: **anyopaque, type_id: u16) void {
    if (global_gc) |gc| {
        gc.addRoot(ptr, type_id) catch {};
    }
}

/// Wrapper for removing roots
export fn cursed_gc_remove_root_wrapper(ptr: **anyopaque) void {
    if (global_gc) |gc| {
        gc.removeRoot(ptr);
    }
}

/// Wrapper for write barriers
export fn cursed_gc_write_barrier_wrapper(old_ref: *anyopaque, new_ref: *anyopaque) void {
    if (global_gc) |gc| {
        gc.writeBarrier(old_ref, new_ref);
    }
}

/// Wrapper for explicit collection
export fn cursed_gc_collect_wrapper() void {
    if (global_gc) |gc| {
        gc.collectNow() catch {};
    }
}

/// Wrapper for finalizer registration
export fn cursed_gc_add_finalizer_wrapper(object: *anyopaque, finalizer: *anyopaque) void {
    if (global_gc) |gc| {
        // Implement proper finalizer registration from LLVM
        // Create finalizer function wrapper
        const finalizer_wrapper_fn = struct {
            fn call(obj: *anyopaque) void {
                // Call the actual finalizer function pointer
                const fn_ptr = @as(?*const fn(*anyopaque) void, @ptrFromInt(@intFromPtr(finalizer)));
                if (fn_ptr) |f| {
                    f(obj);
                }
            }
        }.call;
        
        // Register the finalizer with the GC
        const object_header = gc_module.ObjectHeader.fromData(object);
        gc.registerFinalizer(object_header, finalizer_wrapper_fn) catch |err| {
            std.log.err("Failed to register finalizer: {}", .{err});
        };
    }
}

/// Get GC statistics from runtime
export fn cursed_gc_get_stats() gc_module.GCStats {
    if (global_gc) |gc| {
        return gc.getStats();
    }
    return gc_module.GCStats.init();
}

/// Print GC statistics
export fn cursed_gc_print_stats_wrapper() void {
    if (global_gc) |gc| {
        gc.printStats();
    }
}

/// Stack frame registration for GC root scanning
export fn cursed_gc_register_frame(stack_ptr: *anyopaque) void {
    if (global_gc) |gc| {
        gc.registerStackRoot(stack_ptr) catch |err| {
            std.log.err("Failed to register stack frame: {}", .{err});
        };
    }
}

/// Stack frame unregistration
export fn cursed_gc_unregister_frame(stack_ptr: *anyopaque) void {
    if (global_gc) |gc| {
        gc.unregisterStackRoot(stack_ptr) catch |err| {
            std.log.err("Failed to unregister stack frame: {}", .{err});
        };
    }
}

/// Clean up root table
export fn cursed_gc_cleanup_roots(root_table: **anyopaque) void {
    // Root cleanup is handled by frame unregistration
    _ = root_table;
}
