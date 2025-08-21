const std = @import("std");
const gc_module = @import("gc.zig");
const GC = gc_module.GC;
const GCConfig = gc_module.GCConfig;

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

/// GC Integration with LLVM Code Generation
/// This module provides the bridge between the CURSED garbage collector
/// and LLVM-generated code, including:
/// - GC-aware allocation functions
/// - Stack map generation for precise GC
/// - Write barrier insertion
/// - Root set management for LLVM-generated code

/// Oracle's Week 2: Stack Map Registry for runtime GC integration
pub const StackMapRegistry = struct {
    maps: std.HashMap(u64, StackMapInfo, std.hash_map.DefaultContext(u64), std.hash_map.default_max_load_percentage),
    allocator: std.mem.Allocator,
    
    pub const StackMapInfo = struct {
        function_id: u64,
        root_count: u32,
        function_name: []const u8,
        generated_at: i64,
    };
    
    pub fn init(allocator: std.mem.Allocator) StackMapRegistry {
        return StackMapRegistry{
            .maps = std.HashMap(u64, StackMapInfo, std.hash_map.DefaultContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *StackMapRegistry) void {
        var iterator = self.maps.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.function_name);
        }
        self.maps.deinit(self.allocator);
    }
    
    pub fn registerStackMap(self: *StackMapRegistry, function_id: u64, root_count: u32, function_name: []const u8) !void {
        const owned_name = try self.allocator.dupe(u8, function_name);
        const info = StackMapInfo{
            .function_id = function_id,
            .root_count = root_count,
            .function_name = owned_name,
            .generated_at = std.time.timestamp(),
        };
        try self.maps.put(function_id, info);
    }
    
    pub fn getStackMapInfo(self: *StackMapRegistry, function_id: u64) ?StackMapInfo {
        return self.maps.get(function_id);
    }
    
    pub fn getTotalRootCount(self: *StackMapRegistry) u32 {
        var total: u32 = 0;
        var iterator = self.maps.iterator();
        while (iterator.next()) |entry| {
            total += entry.value_ptr.root_count;
        }
        return total;
    }
};

pub const GCIntegration = struct {
    gc: *GC,
    llvm_context: c.LLVMContextRef,
    llvm_module: c.LLVMModuleRef,
    llvm_builder: c.LLVMBuilderRef,
    stackmap_registry: ?*StackMapRegistry,
    
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
    
    /// Generate LLVM stackmaps for precise garbage collection - Oracle's Week 2 Implementation
    pub fn generateStackMap(self: *GCIntegration, function: c.LLVMValueRef, live_pointers: []c.LLVMValueRef) !void {
        const context = self.context;
        const module = self.module;
        const builder = self.builder;
        
        // Generate precise LLVM stack map for GC root scanning
        const i64_type = c.LLVMInt64TypeInContext(context);
        const i32_type = c.LLVMInt32TypeInContext(context);
        const void_type = c.LLVMVoidTypeInContext(context);
        
        const stack_map_func = c.LLVMGetNamedFunction(module, "llvm.experimental.stackmap") orelse {
            // Create stackmap intrinsic with proper signature
            const stackmap_type = c.LLVMFunctionType(void_type, 
                &[_]c.LLVMTypeRef{i64_type, i32_type}, 2, 1); // Variadic for live roots
            return c.LLVMAddFunction(module, "llvm.experimental.stackmap", stackmap_type);
        };
        
        // Create unique stack map ID for this function using Oracle's hashing strategy
        const function_name = c.LLVMGetValueName(function);
        const name_slice = if (function_name != null) std.mem.sliceTo(function_name, 0) else "anonymous";
        const stack_map_id = @as(u64, @truncate(std.hash_map.hashString(name_slice)));
        
        // Prepare stackmap arguments: ID, shadow bytes, followed by live roots
        var total_args = std.ArrayList(c.LLVMValueRef).init(self.allocator);
        defer total_args.deinit();
        
        // Oracle's Week 2: Precise stackmap metadata
        try total_args.append(c.LLVMConstInt(i64_type, stack_map_id, 0)); // Unique function ID
        try total_args.append(c.LLVMConstInt(i32_type, 0, 0)); // Shadow bytes (0 = scan all)
        
        // Add all live pointer locations as GC roots - with type validation
        var valid_pointers: u32 = 0;
        for (live_pointers) |ptr| {
            if (ptr == null) continue;
            
            const ptr_type = c.LLVMTypeOf(ptr);
            const type_kind = c.LLVMGetTypeKind(ptr_type);
            
            // Oracle's Week 2: Only track actual heap pointers for precise scanning
            if (type_kind == c.LLVMPointerTypeKind) {
                try total_args.append(ptr);
                valid_pointers += 1;
            }
        }
        
        // Generate the stackmap intrinsic call with proper builder setup
        const current_block = c.LLVMGetInsertBlock(builder);
        if (current_block == null) {
            std.debug.print("⚠️  Warning: No insert block for stackmap generation\n", .{});
            return;
        }
        
        const stackmap_call = c.LLVMBuildCall2(builder, 
            c.LLVMGlobalGetValueType(stack_map_func), 
            stack_map_func, 
            total_args.items.ptr, 
            @intCast(total_args.items.len), 
            "oracle_gc_stackmap");
            
        // Oracle's Week 2: Add comprehensive metadata for GC safepoints
        if (stackmap_call != null) {
            // Mark as GC safepoint with precise scanning metadata
            const safepoint_kind = c.LLVMGetMDKindIDInContext(context, "gc.safepoint", 12);
            const safepoint_metadata = c.LLVMMDStringInContext(context, "oracle_precise", 14);
            const safepoint_node = c.LLVMMDNodeInContext(context, &[_]c.LLVMValueRef{safepoint_metadata}, 1);
            c.LLVMSetMetadata(stackmap_call, safepoint_kind, safepoint_node);
            
            // Add function-level GC strategy metadata  
            const gc_strategy_kind = c.LLVMGetMDKindIDInContext(context, "gc.strategy", 11);
            const gc_strategy_metadata = c.LLVMMDStringInContext(context, "cursed-precise", 14);
            const gc_strategy_node = c.LLVMMDNodeInContext(context, &[_]c.LLVMValueRef{gc_strategy_metadata}, 1);
            c.LLVMSetMetadata(function, gc_strategy_kind, gc_strategy_node);
            
            // Oracle's Week 2: Root count metadata for validation
            const root_count_kind = c.LLVMGetMDKindIDInContext(context, "gc.root_count", 12);
            const root_count_metadata = c.LLVMConstInt(i32_type, valid_pointers, 0);
            const root_count_node = c.LLVMMDNodeInContext(context, &[_]c.LLVMValueRef{root_count_metadata}, 1);
            c.LLVMSetMetadata(stackmap_call, root_count_kind, root_count_node);
        }
        
        // Record stackmap for runtime integration
        if (self.stackmap_registry) |registry| {
            try registry.registerStackMap(stack_map_id, valid_pointers, name_slice);
        }
        
        std.debug.print("✅ Oracle's Week 2: Generated precise stackmap for '{}' with {} validated roots (ID: 0x{x})\n", 
            .{name_slice, valid_pointers, stack_map_id});
    }
    
    /// Oracle's Week 2: Object lifetime management with precise tracking
    pub fn trackObjectLifetime(self: *GCIntegration, obj_ptr: c.LLVMValueRef, size: c.LLVMValueRef) !void {
        const context = self.llvm_context;
        const builder = self.llvm_builder;
        const module = self.llvm_module;
        
        // Create or get object tracking function
        const track_func = c.LLVMGetNamedFunction(module, "cursed_gc_track_object") orelse {
            const void_type = c.LLVMVoidTypeInContext(context);
            const ptr_type = c.LLVMPointerTypeInContext(context, 0);
            const size_type = c.LLVMInt64TypeInContext(context);
            
            const track_type = c.LLVMFunctionType(void_type, 
                &[_]c.LLVMTypeRef{ptr_type, size_type}, 2, 0);
            return c.LLVMAddFunction(module, "cursed_gc_track_object", track_type);
        };
        
        // Generate tracking call
        _ = c.LLVMBuildCall2(builder,
            c.LLVMGlobalGetValueType(track_func),
            track_func,
            &[_]c.LLVMValueRef{obj_ptr, size},
            2,
            "gc_track");
    }
    
    /// Oracle's Week 2: GC root scanning with stack map integration
    pub fn generateRootScanning(self: *GCIntegration, function: c.LLVMValueRef, live_objects: []c.LLVMValueRef) !void {
        const context = self.llvm_context;
        const builder = self.llvm_builder; 
        const module = self.llvm_module;
        
        // Create GC root scanning function
        const scan_func = c.LLVMGetNamedFunction(module, "cursed_gc_scan_roots") orelse {
            const void_type = c.LLVMVoidTypeInContext(context);
            const ptr_type = c.LLVMPointerTypeInContext(context, 0);
            const count_type = c.LLVMInt32TypeInContext(context);
            
            const scan_type = c.LLVMFunctionType(void_type,
                &[_]c.LLVMTypeRef{ptr_type, count_type}, 2, 0);
            return c.LLVMAddFunction(module, "cursed_gc_scan_roots", scan_type);
        };
        
        // Create root array for scanning
        const ptr_type = c.LLVMPointerTypeInContext(context, 0);
        const array_type = c.LLVMArrayType(ptr_type, @intCast(live_objects.len));
        
        // Allocate stack space for root array
        const root_array = c.LLVMBuildAlloca(builder, array_type, "gc_root_array");
        
        // Populate root array with live objects
        for (live_objects, 0..) |obj, i| {
            if (obj == null) continue;
            
            const index_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), i, 0);
            const elem_ptr = c.LLVMBuildInBoundsGEP2(builder, array_type, root_array, 
                &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0), index_val}, 2, "root_elem");
            
            // Cast object to pointer if needed
            const obj_as_ptr = if (c.LLVMGetTypeKind(c.LLVMTypeOf(obj)) == c.LLVMPointerTypeKind)
                obj
            else
                c.LLVMBuildIntToPtr(builder, obj, ptr_type, "obj_as_ptr");
                
            _ = c.LLVMBuildStore(builder, obj_as_ptr, elem_ptr);
        }
        
        // Generate root scanning call
        const root_array_ptr = c.LLVMBuildBitCast(builder, root_array, ptr_type, "root_array_ptr");
        const count_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @intCast(live_objects.len), 0);
        
        _ = c.LLVMBuildCall2(builder,
            c.LLVMGlobalGetValueType(scan_func),
            scan_func,
            &[_]c.LLVMValueRef{root_array_ptr, count_val},
            2,
            "gc_scan_call");
            
        std.debug.print("✅ Oracle's Week 2: Generated root scanning for {} live objects\n", .{live_objects.len});
    }
    
    /// Generate GC statepoints for function prologue/epilogue
    pub fn generateGCStatepoints(self: *GCIntegration, function: c.LLVMValueRef) !void {
        const context = self.context;
        const module = self.module;
        const builder = self.builder;
        
        // Get or create LLVM statepoint intrinsic
        const statepoint_func = c.LLVMGetNamedFunction(module, "llvm.experimental.gc.statepoint.p0i8");
        const statepoint_call = if (statepoint_func != null) 
            statepoint_func
        else blk: {
            // Create statepoint intrinsic signature
            const void_type = c.LLVMVoidTypeInContext(context);
            const i64_type = c.LLVMInt64TypeInContext(context);
            const i32_type = c.LLVMInt32TypeInContext(context);
            const ptr_type = c.LLVMPointerTypeInContext(context, 0);
            
            const statepoint_type = c.LLVMFunctionType(void_type, 
                &[_]c.LLVMTypeRef{i64_type, i32_type, ptr_type, i32_type}, 4, 1); // Variadic
            break :blk c.LLVMAddFunction(module, "llvm.experimental.gc.statepoint.p0i8", statepoint_type);
        };
        
        // Generate GC safepoint at function entry
        const entry_block = c.LLVMGetEntryBasicBlock(function);
        const first_inst = c.LLVMGetFirstInstruction(entry_block);
        
        if (first_inst != null) {
            c.LLVMPositionBuilderBefore(builder, first_inst);
        } else {
            c.LLVMPositionBuilderAtEnd(builder, entry_block);
        }
        
        // Generate statepoint call: statepoint(id, flags, callee, num_args, ...)
        const statepoint_id = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0x12345678, 0);
        const statepoint_flags = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0); // No flags
        const dummy_callee = c.LLVMConstNull(c.LLVMPointerTypeInContext(context, 0));
        const num_args = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0);
        
        _ = c.LLVMBuildCall2(builder, 
            c.LLVMGlobalGetValueType(statepoint_call.?),
            statepoint_call.?, 
            &[_]c.LLVMValueRef{statepoint_id, statepoint_flags, dummy_callee, num_args}, 4, 
            "gc_statepoint_entry");
            
        // Generate GC safepoint at function exits (before return instructions)
        var current_block = c.LLVMGetFirstBasicBlock(function);
        while (current_block != null) {
            var inst = c.LLVMGetLastInstruction(current_block);
            if (inst != null and c.LLVMGetInstructionOpcode(inst) == c.LLVMRet) {
                // Insert statepoint before return
                c.LLVMPositionBuilderBefore(builder, inst);
                
                _ = c.LLVMBuildCall2(builder, 
                    c.LLVMGlobalGetValueType(statepoint_call.?),
                    statepoint_call.?, 
                    &[_]c.LLVMValueRef{statepoint_id, statepoint_flags, dummy_callee, num_args}, 4, 
                    "gc_statepoint_exit");
            }
            current_block = c.LLVMGetNextBasicBlock(current_block);
        }
        
        std.debug.print("✅ Generated GC statepoints for function '{}'\n", .{c.LLVMGetValueName(function)});
    }
    
    /// Wire up complete GC integration for heap stress testing
    pub fn wireGCIntegration(self: *GCIntegration, module_functions: []c.LLVMValueRef) !void {
        for (module_functions) |function| {
            // Generate stackmaps for all functions
            var live_pointers = std.ArrayList(c.LLVMValueRef).init(self.allocator);
            defer live_pointers.deinit();
            
            // Collect all pointer values in function as potential GC roots
            var current_block = c.LLVMGetFirstBasicBlock(function);
            while (current_block != null) {
                var inst = c.LLVMGetFirstInstruction(current_block);
                while (inst != null) {
                    const inst_type = c.LLVMTypeOf(inst);
                    if (c.LLVMGetTypeKind(inst_type) == c.LLVMPointerTypeKind) {
                        try live_pointers.append(inst);
                    }
                    inst = c.LLVMGetNextInstruction(inst);
                }
                current_block = c.LLVMGetNextBasicBlock(current_block);
            }
            
            // Generate stackmap and statepoints for this function
            try self.generateStackMap(function, live_pointers.items);
            try self.generateGCStatepoints(function);
        }
        
        std.debug.print("✅ Wired GC integration for {} functions\n", .{module_functions.len});
        
        // Record stackmap for runtime GC integration
        std.debug.print("✅ Generated precise stackmap for function '{}' with {} live roots\n", 
            .{ std.mem.sliceTo(function_name, 0), live_pointers.len });
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
