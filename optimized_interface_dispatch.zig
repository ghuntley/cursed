const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const interface_dispatch = @import("src-zig/interface_dispatch.zig");
const InterfaceDispatcher = interface_dispatch.InterfaceDispatcher;
const VTable = interface_dispatch.VTable;
const InterfaceInstance = interface_dispatch.InterfaceInstance;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

/// Optimized Interface Dispatch System with enhanced vtable generation
/// Key optimizations:
/// 1. Method index caching for O(1) lookup
/// 2. Vtable deduplication for memory efficiency 
/// 3. Inline caching for hot method calls
/// 4. Optimized LLVM IR generation with better register allocation
/// 5. Static vtable generation at compile time
pub const OptimizedInterfaceDispatcher = struct {
    base_dispatcher: InterfaceDispatcher,
    
    // Performance optimizations
    method_index_cache: HashMap(MethodCallKey, u32, MethodCallKeyContext, std.hash_map.default_max_load_percentage),
    vtable_dedup_cache: HashMap(VTableSignature, *VTable, VTableSignatureContext, std.hash_map.default_max_load_percentage),
    inline_cache: HashMap(InlineCacheKey, InlineCacheEntry, InlineCacheKeyContext, std.hash_map.default_max_load_percentage),
    
    // Compile-time optimization data
    static_vtables: HashMap([]const u8, StaticVTableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    hot_methods: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), // call count tracking
    
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, base_dispatcher: InterfaceDispatcher) Self {
        return Self{
            .base_dispatcher = base_dispatcher,
            .method_index_cache = HashMap(MethodCallKey, u32, MethodCallKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .vtable_dedup_cache = HashMap(VTableSignature, *VTable, VTableSignatureContext, std.hash_map.default_max_load_percentage).init(allocator),
            .inline_cache = HashMap(InlineCacheKey, InlineCacheEntry, InlineCacheKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .static_vtables = HashMap([]const u8, StaticVTableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .hot_methods = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.method_index_cache.deinit();
        self.vtable_dedup_cache.deinit();
        self.inline_cache.deinit();
        self.static_vtables.deinit();
        self.hot_methods.deinit();
    }
    
    /// Fast method dispatch with inline caching
    pub fn fastDispatchMethodCall(self: *Self, interface_instance: *InterfaceInstance, method_name: []const u8, args: []interface_dispatch.Value) !interface_dispatch.Value {
        // Track method call frequency for optimization
        self.trackMethodCall(method_name);
        
        // Try inline cache first for hot methods
        const cache_key = InlineCacheKey{
            .vtable_ptr = @intFromPtr(interface_instance.vtable),
            .method_name = method_name,
        };
        
        if (self.inline_cache.get(cache_key)) |cache_entry| {
            if (cache_entry.vtable == interface_instance.vtable) {
                // Inline cache hit - direct method call
                const method_func = interface_instance.vtable.methods[cache_entry.method_index];
                return try method_func.call(args);
            }
        }
        
        // Cache miss - use fast method index lookup
        const method_index = try self.getMethodIndexFast(interface_instance.vtable, method_name);
        
        // Update inline cache
        const cache_entry = InlineCacheEntry{
            .vtable = interface_instance.vtable,
            .method_index = method_index,
            .call_count = 1,
        };
        try self.inline_cache.put(cache_key, cache_entry);
        
        // Execute method
        const method_func = interface_instance.vtable.methods[method_index];
        return try method_func.call(args);
    }
    
    /// Fast method index lookup with caching
    fn getMethodIndexFast(self: *Self, vtable: *VTable, method_name: []const u8) !u32 {
        const lookup_key = MethodCallKey{
            .vtable_ptr = @intFromPtr(vtable),
            .method_name = method_name,
        };
        
        if (self.method_index_cache.get(lookup_key)) |index| {
            return index;
        }
        
        // Fallback to linear search and cache result
        const interface_type = self.base_dispatcher.interface_types.get(vtable.interface_name) orelse {
            return interface_dispatch.InterfaceDispatchError.InterfaceNotFound;
        };
        
        for (interface_type.methods.items, 0..) |method, i| {
            if (std.mem.eql(u8, method.name, method_name)) {
                const index = @as(u32, @intCast(i));
                try self.method_index_cache.put(lookup_key, index);
                return index;
            }
        }
        
        return interface_dispatch.InterfaceDispatchError.MethodNotFound;
    }
    
    /// Track method call frequency for optimization decisions
    fn trackMethodCall(self: *Self, method_name: []const u8) void {
        const current_count = self.hot_methods.get(method_name) orelse 0;
        self.hot_methods.put(method_name, current_count + 1) catch {};
    }
    
    /// Generate optimized vtable with deduplication
    pub fn generateOptimizedVTable(self: *Self, struct_name: []const u8, interface_name: []const u8, methods: []const interface_dispatch.MethodImpl) !*VTable {
        // Create vtable signature for deduplication
        const signature = try self.createVTableSignature(interface_name, methods);
        
        // Check if we already have this vtable
        if (self.vtable_dedup_cache.get(signature)) |existing_vtable| {
            return existing_vtable;
        }
        
        // Create new vtable using base implementation
        const vtable = try self.base_dispatcher.createVTable(struct_name, interface_name, methods);
        
        // Cache for future deduplication
        try self.vtable_dedup_cache.put(signature, vtable);
        
        return vtable;
    }
    
    /// Create vtable signature for deduplication
    fn createVTableSignature(self: *Self, interface_name: []const u8, methods: []const interface_dispatch.MethodImpl) !VTableSignature {
        var signature_data = ArrayList(u8).init(self.allocator);
        defer signature_data.deinit();
        
        try signature_data.appendSlice(interface_name);
        try signature_data.append('|');
        
        for (methods) |method| {
            try signature_data.appendSlice(method.name);
            try signature_data.append(',');
        }
        
        return VTableSignature{
            .hash = std.hash_map.hashString(signature_data.items),
            .data = try self.allocator.dupe(u8, signature_data.items),
        };
    }
    
    /// Generate highly optimized LLVM IR for vtable
    pub fn generateOptimizedVTableLLVM(self: *Self, module: c.LLVMModuleRef, context: c.LLVMContextRef, struct_name: []const u8, interface_name: []const u8) !c.LLVMValueRef {
        // Check if we have a static vtable already computed
        const static_key = try std.fmt.allocPrint(self.allocator, "{s}_{s}", .{ struct_name, interface_name });
        defer self.allocator.free(static_key);
        
        if (self.static_vtables.get(static_key)) |static_info| {
            return static_info.llvm_value;
        }
        
        const vtable = self.base_dispatcher.getVTable(struct_name, interface_name) orelse {
            return interface_dispatch.InterfaceDispatchError.ImplementationNotFound;
        };
        
        // Create optimized vtable type with proper alignment
        const func_ptr_type = c.LLVMPointerType(
            c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                null,
                0,
                0
            ),
            0
        );
        
        const vtable_type = c.LLVMArrayType(func_ptr_type, @as(u32, @intCast(vtable.method_count)));
        
        // Create vtable with better naming and linkage optimization
        const vtable_name = try std.fmt.allocPrint(self.allocator, "optimized_vtable_{s}_{s}", .{ struct_name, interface_name });
        defer self.allocator.free(vtable_name);
        
        const vtable_global = c.LLVMAddGlobal(module, vtable_type, vtable_name.ptr);
        
        // Optimize linkage based on usage patterns
        if (self.isHotVTable(struct_name, interface_name)) {
            c.LLVMSetLinkage(vtable_global, c.LLVMExternalLinkage); // Make visible for inlining
        } else {
            c.LLVMSetLinkage(vtable_global, c.LLVMInternalLinkage); // Keep private
        }
        
        // Set optimal alignment for cache performance
        c.LLVMSetAlignment(vtable_global, 8);
        
        // Mark as constant for optimization
        c.LLVMSetGlobalConstant(vtable_global, 1);
        
        // Initialize vtable with optimized method pointers
        var method_values = try self.allocator.alloc(c.LLVMValueRef, vtable.method_count);
        defer self.allocator.free(method_values);
        
        for (vtable.methods, 0..) |method_func, i| {
            const method_name = try std.fmt.allocPrint(self.allocator, "opt_{s}_{s}_impl", .{ struct_name, method_func.*.name });
            defer self.allocator.free(method_name);
            
            const func = c.LLVMGetNamedFunction(module, method_name.ptr) orelse {
                // Create optimized function with proper attributes
                const method_func_type = c.LLVMFunctionType(
                    c.LLVMVoidTypeInContext(context),
                    null,
                    0,
                    0
                );
                const new_func = c.LLVMAddFunction(module, method_name.ptr, method_func_type);
                
                // Add optimization attributes for hot methods
                if (self.isHotMethod(method_func.*.name)) {
                    c.LLVMAddFunctionAttr(new_func, c.LLVMHotAttribute);
                    c.LLVMAddFunctionAttr(new_func, c.LLVMInlineHintAttribute);
                }
                
                new_func
            };
            
            method_values[i] = func;
        }
        
        const vtable_init = c.LLVMConstArray(func_ptr_type, method_values.ptr, @as(u32, @intCast(vtable.method_count)));
        c.LLVMSetInitializer(vtable_global, vtable_init);
        
        // Cache static vtable info
        const static_info = StaticVTableInfo{
            .llvm_value = vtable_global,
            .method_count = vtable.method_count,
            .is_hot = self.isHotVTable(struct_name, interface_name),
        };
        try self.static_vtables.put(static_key, static_info);
        
        return vtable_global;
    }
    
    /// Generate highly optimized method dispatch with minimal overhead
    pub fn generateOptimizedMethodDispatchLLVM(self: *Self, module: c.LLVMModuleRef, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, interface_instance: c.LLVMValueRef, method_name: []const u8, method_index: u32, args: []c.LLVMValueRef) !c.LLVMValueRef {
        _ = module;
        
        // For hot methods, generate optimized dispatch
        if (self.isHotMethod(method_name)) {
            return try self.generateHotMethodDispatch(context, builder, interface_instance, method_index, args);
        }
        
        // Standard optimized dispatch for regular methods
        return try self.generateStandardMethodDispatch(context, builder, interface_instance, method_index, args);
    }
    
    /// Generate highly optimized dispatch for hot methods
    fn generateHotMethodDispatch(self: *Self, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, interface_instance: c.LLVMValueRef, method_index: u32, args: []c.LLVMValueRef) !c.LLVMValueRef {
        _ = self;
        
        // Create efficient struct layout access
        const vtable_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // Extract vtable with minimal indirection
        const vtable_ptr = c.LLVMBuildStructGEP2(
            builder,
            vtable_ptr_type,
            interface_instance,
            0,
            "hot_vtable_ptr"
        );
        
        const vtable = c.LLVMBuildLoad2(
            builder,
            vtable_ptr_type,
            vtable_ptr,
            "hot_vtable"
        );
        
        // Direct method access with constant index
        const method_index_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), method_index, 0);
        const method_ptr_ptr = c.LLVMBuildGEP2(
            builder,
            vtable_ptr_type,
            vtable,
            &[_]c.LLVMValueRef{method_index_val},
            1,
            "hot_method_ptr_ptr"
        );
        
        const method_ptr = c.LLVMBuildLoad2(
            builder,
            vtable_ptr_type,
            method_ptr_ptr,
            "hot_method_ptr"
        );
        
        // Optimized call with tail call optimization for eligible methods
        const result = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            method_ptr,
            args.ptr,
            @as(u32, @intCast(args.len)),
            "hot_method_result"
        );
        
        // Enable tail call optimization for hot methods
        c.LLVMSetTailCall(result, 1);
        
        return result;
    }
    
    /// Generate standard optimized dispatch
    fn generateStandardMethodDispatch(self: *Self, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, interface_instance: c.LLVMValueRef, method_index: u32, args: []c.LLVMValueRef) !c.LLVMValueRef {
        _ = self;
        
        const vtable_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        const vtable_ptr = c.LLVMBuildStructGEP2(
            builder,
            vtable_ptr_type,
            interface_instance,
            0,
            "vtable_ptr"
        );
        
        const vtable = c.LLVMBuildLoad2(
            builder,
            vtable_ptr_type,
            vtable_ptr,
            "vtable"
        );
        
        const method_index_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), method_index, 0);
        const method_ptr_ptr = c.LLVMBuildGEP2(
            builder,
            vtable_ptr_type,
            vtable,
            &[_]c.LLVMValueRef{method_index_val},
            1,
            "method_ptr_ptr"
        );
        
        const method_ptr = c.LLVMBuildLoad2(
            builder,
            vtable_ptr_type,
            method_ptr_ptr,
            "method_ptr"
        );
        
        const result = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            method_ptr,
            args.ptr,
            @as(u32, @intCast(args.len)),
            "method_result"
        );
        
        return result;
    }
    
    /// Check if vtable is frequently used (hot)
    fn isHotVTable(self: *Self, struct_name: []const u8, interface_name: []const u8) bool {
        _ = struct_name;
        _ = interface_name;
        // Simple heuristic - in practice would track vtable usage
        return self.vtable_dedup_cache.count() < 10; // Keep first 10 vtables hot
    }
    
    /// Check if method is frequently called (hot)
    fn isHotMethod(self: *Self, method_name: []const u8) bool {
        const call_count = self.hot_methods.get(method_name) orelse 0;
        return call_count > 10; // Methods called more than 10 times are hot
    }
    
    /// Performance statistics
    pub fn getPerformanceStats(self: *Self) PerformanceStats {
        return PerformanceStats{
            .method_cache_size = self.method_index_cache.count(),
            .vtable_dedup_count = self.vtable_dedup_cache.count(),
            .inline_cache_hits = self.inline_cache.count(),
            .hot_method_count = self.hot_methods.count(),
        };
    }
};

/// Performance optimization data structures
const MethodCallKey = struct {
    vtable_ptr: usize,
    method_name: []const u8,
};

const MethodCallKeyContext = struct {
    pub fn hash(self: @This(), key: MethodCallKey) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(std.mem.asBytes(&key.vtable_ptr));
        hasher.update(key.method_name);
        return hasher.final();
    }
    
    pub fn eql(self: @This(), a: MethodCallKey, b: MethodCallKey) bool {
        _ = self;
        return a.vtable_ptr == b.vtable_ptr and std.mem.eql(u8, a.method_name, b.method_name);
    }
};

const VTableSignature = struct {
    hash: u64,
    data: []const u8,
};

const VTableSignatureContext = struct {
    pub fn hash(self: @This(), key: VTableSignature) u64 {
        _ = self;
        return key.hash;
    }
    
    pub fn eql(self: @This(), a: VTableSignature, b: VTableSignature) bool {
        _ = self;
        return a.hash == b.hash and std.mem.eql(u8, a.data, b.data);
    }
};

const InlineCacheKey = struct {
    vtable_ptr: usize,
    method_name: []const u8,
};

const InlineCacheKeyContext = struct {
    pub fn hash(self: @This(), key: InlineCacheKey) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(std.mem.asBytes(&key.vtable_ptr));
        hasher.update(key.method_name);
        return hasher.final();
    }
    
    pub fn eql(self: @This(), a: InlineCacheKey, b: InlineCacheKey) bool {
        _ = self;
        return a.vtable_ptr == b.vtable_ptr and std.mem.eql(u8, a.method_name, b.method_name);
    }
};

const InlineCacheEntry = struct {
    vtable: *VTable,
    method_index: u32,
    call_count: u32,
};

const StaticVTableInfo = struct {
    llvm_value: c.LLVMValueRef,
    method_count: usize,
    is_hot: bool,
};

const PerformanceStats = struct {
    method_cache_size: u32,
    vtable_dedup_count: u32,
    inline_cache_hits: u32,
    hot_method_count: u32,
};
