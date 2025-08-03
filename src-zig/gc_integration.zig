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
    
    allocator: std.mem.Allocator,
    
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
            .gc_alloc_func = undefined,
            .gc_add_root_func = undefined,
            .gc_remove_root_func = undefined,
            .gc_write_barrier_func = undefined,
            .gc_collect_func = undefined,
            .gc_add_finalizer_func = undefined,
            .gc_ptr_type = undefined,
            .gc_header_type = undefined,
            .allocator = allocator,
        };
        
        try integration.declareFunctions();
        try integration.setupTypes();
        
        return integration;
    }
    
    /// Clean up GC integration
    pub fn deinit(self: *GCIntegration) void {
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
        
        // TODO: Implement LLVM stack map generation
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
        // TODO: Implement proper safepoint generation
    }
    
    /// Generate function prologue for GC
    pub fn generateFunctionPrologue(self: *GCIntegration, function: c.LLVMValueRef) void {
        // Save current builder position
        const current_block = c.LLVMGetInsertBlock(self.llvm_builder);
        
        // Create entry block if it doesn't exist
        const entry_block = c.LLVMGetEntryBasicBlock(function);
        
        // Position at beginning of entry block
        c.LLVMPositionBuilderAtEnd(self.llvm_builder, entry_block);
        
        // TODO: Add GC prologue code
        // - Register stack frame
        // - Initialize local roots
        
        // Restore builder position
        c.LLVMPositionBuilderAtEnd(self.llvm_builder, current_block);
    }
    
    /// Generate function epilogue for GC
    pub fn generateFunctionEpilogue(self: *GCIntegration, function: c.LLVMValueRef) void {
        _ = function;
        
        // TODO: Add GC epilogue code
        // - Unregister stack frame
        // - Clean up local roots
        
        _ = self;
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
        // This is a simplified finalizer wrapper
        // Real implementation would properly handle function pointers
        _ = object;
        _ = finalizer;
        _ = gc;
        // TODO: Implement proper finalizer registration from LLVM
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
