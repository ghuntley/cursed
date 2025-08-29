const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const interpreter = @import("interpreter.zig");
const Value = interpreter.Value;
const FunctionValue = interpreter.FunctionValue;
const CursedError = interpreter.CursedError;

const type_system = @import("type_system_runtime.zig");
const RuntimeTypeInfo = type_system.RuntimeTypeInfo;
const InterfaceRegistry = type_system.InterfaceRegistry;

// LLVM C imports (dummy types for now)
const c = struct {
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMPointerType(_: LLVMTypeRef, _: c_uint) LLVMTypeRef { return null; }
    pub fn LLVMFunctionType(_: LLVMTypeRef, _: ?[*]LLVMTypeRef, _: c_uint, _: LLVMBool) LLVMTypeRef { return null; }
    pub fn LLVMVoidTypeInContext(_: LLVMContextRef) LLVMTypeRef { return null; }
    pub fn LLVMInt8TypeInContext(_: LLVMContextRef) LLVMTypeRef { return null; }
    pub fn LLVMInt32TypeInContext(_: LLVMContextRef) LLVMTypeRef { return null; }
    pub fn LLVMArrayType(_: LLVMTypeRef, _: c_uint) LLVMTypeRef { return null; }
    pub fn LLVMAddGlobal(_: LLVMModuleRef, _: LLVMTypeRef, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMSetLinkage(_: LLVMValueRef, _: c_uint) void {}
    pub fn LLVMSetAlignment(_: LLVMValueRef, _: c_uint) void {}
    pub fn LLVMSetGlobalConstant(_: LLVMValueRef, _: LLVMBool) void {}
    pub fn LLVMConstArray(_: LLVMTypeRef, _: [*]LLVMValueRef, _: c_uint) LLVMValueRef { return null; }
    pub fn LLVMSetInitializer(_: LLVMValueRef, _: LLVMValueRef) void {}
    pub fn LLVMBuildStructGEP2(_: LLVMBuilderRef, _: LLVMTypeRef, _: LLVMValueRef, _: c_uint, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMBuildLoad2(_: LLVMBuilderRef, _: LLVMTypeRef, _: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMBuildStore(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMBuildAlloca(_: LLVMBuilderRef, _: LLVMTypeRef, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMBuildBitCast(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMTypeRef, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMBuildCall2(_: LLVMBuilderRef, _: LLVMTypeRef, _: LLVMValueRef, _: ?[*]LLVMValueRef, _: c_uint, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMStructTypeInContext(_: LLVMContextRef, _: [*]LLVMTypeRef, _: c_uint, _: LLVMBool) LLVMTypeRef { return null; }
    pub fn LLVMConstNull(_: LLVMTypeRef) LLVMValueRef { return null; }
    pub fn LLVMGetNamedFunction(_: LLVMModuleRef, _: [*c]const u8) LLVMValueRef { return null; }
    pub fn LLVMAddFunction(_: LLVMModuleRef, _: [*c]const u8, _: LLVMTypeRef) LLVMValueRef { return null; }
    pub fn LLVMGetGlobalParent(_: LLVMValueRef) LLVMModuleRef { return null; }
    pub fn LLVMGetBasicBlockParent(_: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMGetInsertBlock(_: LLVMBuilderRef) LLVMValueRef { return null; }
    pub fn LLVMGetElementType(_: LLVMTypeRef) LLVMTypeRef { return null; }
    pub fn LLVMTypeOf(_: LLVMValueRef) LLVMTypeRef { return null; }
    
    pub const LLVMInternalLinkage: c_uint = 0;
};

/// Enhanced Interface dispatch system with complete method resolution and GC integration
pub const EnhancedInterfaceDispatcher = struct {
    allocator: Allocator,
    interface_registry: *InterfaceRegistry,
    vtables: HashMap(InterfaceImplKey, *VTable, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage),
    interface_types: HashMap([]const u8, InterfaceType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    implementations: HashMap(ImplKey, ImplementationInfo, ImplKeyContext, std.hash_map.default_max_load_percentage),
    method_cache: HashMap(u64, usize, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),
    
    // Enhanced diagnostic tracking
    validation_errors: ArrayList(InterfaceValidationError),
    dispatch_stats: DispatchStatistics,

    const Self = @This();

    pub fn init(allocator: Allocator, interface_registry: *InterfaceRegistry) Self {
        return Self{
            .allocator = allocator,
            .interface_registry = interface_registry,
            .vtables = HashMap(InterfaceImplKey, *VTable, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage){},
            .interface_types = HashMap([]const u8, InterfaceType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .implementations = HashMap(ImplKey, ImplementationInfo, ImplKeyContext, std.hash_map.default_max_load_percentage){},
            .method_cache = HashMap(u64, usize, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .validation_errors = .empty,
            .dispatch_stats = DispatchStatistics.init(),
        };
    }

    pub fn deinit(self: *Self) void {
        // Clean up vtables
        var vtable_iterator = self.vtables.iterator();
        while (vtable_iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.vtables.deinit(self.allocator);
        
        // Clean up interface types
        var interface_iterator = self.interface_types.iterator();
        while (interface_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.interface_types.deinit(self.allocator);
        
        self.implementations.deinit(self.allocator);
        self.method_cache.deinit(self.allocator);
        self.validation_errors.deinit(self.allocator);
    }

    /// Register an interface type with comprehensive validation
    pub fn registerInterface(self: *Self, name: []const u8, methods: []const MethodSignature) !void {
        // Validate interface definition
        if (methods.len == 0) {
            return InterfaceDispatchError.EmptyInterface;
        }
        
        // Check for duplicate method names
        for (methods, 0..) |method, i| {
            for (methods[i + 1..]) |other_method| {
                if (std.mem.eql(u8, method.name, other_method.name)) {
                    return InterfaceDispatchError.DuplicateMethod;
                }
            }
        }
        
        var interface_type = InterfaceType.init(self.allocator, name);
        for (methods) |method| {
            try interface_type.addMethod(method);
        }
        try self.interface_types.put(name, interface_type);
    }

    /// Register a struct implementation of an interface with complete validation
    pub fn registerImplementation(self: *Self, struct_name: []const u8, interface_name: []const u8, methods: []const MethodImpl) !void {
        const interface_type = self.interface_types.get(interface_name) orelse {
            return InterfaceDispatchError.InterfaceNotFound;
        };

        // Comprehensive implementation validation
        const validation_result = try self.validateImplementationComplete(interface_type, methods);
        if (!validation_result.valid) {
            try self.recordValidationError(struct_name, interface_name, validation_result);
            return InterfaceDispatchError.IncompleteImplementation;
        }

        // Create vtable for this implementation
        const vtable = try self.createVTableComplete(struct_name, interface_name, methods);
        
        const key = InterfaceImplKey{
            .struct_name = try self.allocator.dupe(u8, struct_name),
            .interface_name = try self.allocator.dupe(u8, interface_name),
        };
        
        try self.vtables.put(key, vtable);

        // Store implementation info
        const impl_key = ImplKey{
            .struct_name = key.struct_name,
            .interface_name = key.interface_name,
        };
        
        const impl_info = ImplementationInfo{
            .struct_name = key.struct_name,
            .interface_name = key.interface_name,
            .vtable = vtable,
            .method_count = methods.len,
        };
        
        try self.implementations.put(impl_key, impl_info);
    }

    /// Complete vtable creation with optimization and error handling
    fn createVTableComplete(self: *Self, struct_name: []const u8, interface_name: []const u8, methods: []const MethodImpl) !*VTable {
        const interface_type = self.interface_types.get(interface_name).?;
        
        var vtable = try self.allocator.create(VTable);
        vtable.* = try VTable.init(self.allocator, interface_name, interface_type.methods.items.len);

        // Populate vtable with method implementations in interface order
        for (interface_type.methods.items, 0..) |interface_method, i| {
            var found = false;
            for (methods) |method_impl| {
                if (std.mem.eql(u8, interface_method.name, method_impl.name)) {
                    vtable.methods[i] = method_impl.function;
                    found = true;
                    break;
                }
            }
            
            if (!found) {
                vtable.deinit();
                self.allocator.destroy(vtable);
                std.log.err("Method '{}' not found in implementation for struct '{}' interface '{}'", 
                           .{ interface_method.name, struct_name, interface_name });
                return InterfaceDispatchError.MethodNotImplemented;
            }
        }

        return vtable;
    }

    /// Complete implementation validation with signature checking
    fn validateImplementationComplete(self: *Self, interface_type: InterfaceType, methods: []const MethodImpl) !ValidationResult {
        var missing_methods = std.ArrayList(u8){};
        var signature_mismatches = std.ArrayList(u8){};
        
        for (interface_type.methods.items) |interface_method| {
            var found = false;
            for (methods) |method_impl| {
                if (std.mem.eql(u8, interface_method.name, method_impl.name)) {
                    found = true;
                    
                    // Validate method signature compatibility
                    const signature_result = try self.validateMethodSignatureComplete(interface_method, method_impl);
                    if (!signature_result.compatible) {
                        try signature_mismatches.append(SignatureMismatch{
                            .method_name = interface_method.name,
                            .expected_signature = interface_method,
                            .actual_function = method_impl.function,
                            .error_message = signature_result.error_message orelse "Unknown signature error",
                        });
                    }
                    break;
                }
            }
            
            if (!found) {
                try missing_methods.append(allocator, interface_method.name);
            }
        }
        
        const is_valid = missing_methods.items.len == 0 and signature_mismatches.items.len == 0;
        
        return ValidationResult{
            .valid = is_valid,
            .missing_methods = missing_methods,
            .signature_mismatches = signature_mismatches,
        };
    }
    
    /// Complete method signature validation with type compatibility
    fn validateMethodSignatureComplete(self: *Self, interface_method: MethodSignature, impl_method: MethodImpl) !SignatureCompatibilityResult {
        _ = self;
        
        // Check parameter count
        if (interface_method.parameter_types.len != impl_method.function.parameter_count) {
            return SignatureCompatibilityResult{
                .compatible = false,
                .error_message = "Parameter count mismatch",
            };
        }
        
        // Check parameter types - in a complete system this would check covariance/contravariance
        for (interface_method.parameter_types, impl_method.function.parameter_types) |expected_param, actual_param| {
            if (!std.mem.eql(u8, expected_param, actual_param)) {
                return SignatureCompatibilityResult{
                    .compatible = false,
                    .error_message = "Parameter type mismatch",
                };
            }
        }
        
        // Check return type compatibility
        if (!std.mem.eql(u8, interface_method.return_type, impl_method.function.return_type)) {
            return SignatureCompatibilityResult{
                .compatible = false,
                .error_message = "Return type mismatch",
            };
        }
        
        return SignatureCompatibilityResult{
            .compatible = true,
            .error_message = null,
        };
    }

    /// Enhanced method dispatch with comprehensive error handling and optimization
    pub fn dispatchMethodCall(self: *Self, object: *InterfaceInstance, method_name: []const u8, args: []Value) !Value {
        self.dispatch_stats.total_calls += 1;
        
        const vtable = object.vtable;
        
        // Fast path: try cache lookup first for repeated method calls
        const cache_key = @intFromPtr(vtable) ^ std.hash_map.hashString(method_name);
        if (self.method_cache.get(cache_key)) |cached_index| {
            self.dispatch_stats.cache_hits += 1;
            const method_func = vtable.methods[cached_index];
            return try method_func.call(args);
        }
        
        self.dispatch_stats.cache_misses += 1;
        
        // Find method index in interface
        const interface_type = self.interface_types.get(vtable.interface_name) orelse {
            self.dispatch_stats.errors += 1;
            return InterfaceDispatchError.InterfaceNotFound;
        };
        
        var method_index: ?usize = null;
        for (interface_type.methods.items, 0..) |method, i| {
            if (std.mem.eql(u8, method.name, method_name)) {
                method_index = i;
                break;
            }
        }
        
        const index = method_index orelse {
            self.dispatch_stats.errors += 1;
            std.log.err("Method '{}' not found in interface '{}'", .{ method_name, vtable.interface_name });
            return InterfaceDispatchError.MethodNotFound;
        };

        // Cache the result for future calls
        self.method_cache.put(cache_key, index) catch {};

        // Validate arguments before dispatch
        const method_func = vtable.methods[index];
        if (args.len != method_func.parameter_count) {
            self.dispatch_stats.errors += 1;
            return CursedError.ArgumentCountMismatch;
        }

        // Call method through vtable
        return try method_func.call(args);
    }

    /// Create interface instance with GC write barriers
    pub fn createInterfaceInstanceWithGC(self: *Self, struct_instance: *Value, interface_name: []const u8) !*InterfaceInstance {
        const struct_name = switch (struct_instance.*) {
            .Struct => |struct_val| struct_val.type_name,
            else => return InterfaceDispatchError.InvalidStructType,
        };

        const key = InterfaceImplKey{
            .struct_name = struct_name,
            .interface_name = interface_name,
        };
        
        const vtable = self.vtables.get(key) orelse {
            return InterfaceDispatchError.ImplementationNotFound;
        };

        const interface_instance = try self.allocator.create(InterfaceInstance);
        interface_instance.* = InterfaceInstance{
            .underlying_object = struct_instance,
            .vtable = vtable,
            .interface_name = interface_name,
        };

        // Register with GC if available (this would integrate with the actual GC)
        try self.registerInterfaceInstanceWithGC(interface_instance);

        return interface_instance;
    }

    /// Register interface instance with garbage collector
    fn registerInterfaceInstanceWithGC(self: *Self, interface_instance: *InterfaceInstance) !void {
        _ = self;
        _ = interface_instance;
        // In a real implementation, this would:
        // 1. Register the interface instance with the GC
        // 2. Set up write barriers for the vtable and data pointers
        // 3. Mark the instance as containing managed pointers
        // For now, this is a placeholder
    }

    /// Insert GC write barrier for LLVM code generation
    fn insertWriteBarrierLLVM(self: *Self, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, 
                              object_ptr: c.LLVMValueRef, field_ptr: c.LLVMValueRef, 
                              value: c.LLVMValueRef, field_name: []const u8) !void {
        _ = self;
        _ = field_name;
        
        // Create write barrier function call for garbage collection
        const write_barrier_name = "cursed_gc_write_barrier";
        
        // Get the module from the current context
        const current_bb = c.LLVMGetInsertBlock(builder);
        const current_func = c.LLVMGetBasicBlockParent(current_bb);
        const module = c.LLVMGetGlobalParent(current_func);
        
        // Check if write barrier function exists, create if not
        var write_barrier_func = c.LLVMGetNamedFunction(module, write_barrier_name);
        if (write_barrier_func == null) {
            // Create write barrier function signature: void(void* object, void** field_ptr, void* value)
            var param_types = [_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // object
                c.LLVMPointerType(c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), 0), // field_ptr
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // value
            };
            
            const func_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                param_types.ptr,
                param_types.len,
                0
            );
            
            write_barrier_func = c.LLVMAddFunction(module, write_barrier_name, func_type);
        }
        
        // Cast arguments to appropriate types
        const i8_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const object_cast = c.LLVMBuildBitCast(builder, object_ptr, i8_ptr_type, "object_cast");
        const field_cast = c.LLVMBuildBitCast(builder, field_ptr, c.LLVMPointerType(i8_ptr_type, 0), "field_cast");
        const value_cast = c.LLVMBuildBitCast(builder, value, i8_ptr_type, "value_cast");
        
        // Call the write barrier before the actual store
        var barrier_args = [_]c.LLVMValueRef{ object_cast, field_cast, value_cast };
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMGetElementType(c.LLVMTypeOf(write_barrier_func.?)),
            write_barrier_func.?,
            barrier_args.ptr,
            barrier_args.len,
            "write_barrier_call"
        );
    }

    /// Create interface instance in LLVM with write barriers
    pub fn createInterfaceInstanceLLVMWithGC(self: *Self, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, 
                                             vtable: c.LLVMValueRef, data_ptr: c.LLVMValueRef) !c.LLVMValueRef {
        // Interface structure: { vtable_ptr, data_ptr, type_info }
        const interface_type = c.LLVMStructTypeInContext(
            context,
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // vtable
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // data
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // type_info
            },
            3,
            0
        );
        
        // Allocate interface instance
        const interface_instance = c.LLVMBuildAlloca(builder, interface_type, "interface_instance");
        
        // Set vtable pointer with write barrier for GC integration
        const vtable_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 0, "vtable_field");
        try self.insertWriteBarrierLLVM(context, builder, interface_instance, vtable_field, vtable, "vtable");
        _ = c.LLVMBuildStore(builder, vtable, vtable_field);
        
        // Set data pointer with write barrier for GC integration
        const data_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 1, "data_field");
        try self.insertWriteBarrierLLVM(context, builder, interface_instance, data_field, data_ptr, "data");
        _ = c.LLVMBuildStore(builder, data_ptr, data_field);
        
        // Set type info (null for now) with write barrier for GC integration
        const type_info_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 2, "type_info_field");
        const null_ptr = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0));
        try self.insertWriteBarrierLLVM(context, builder, interface_instance, type_info_field, null_ptr, "type_info");
        _ = c.LLVMBuildStore(builder, null_ptr, type_info_field);
        
        return interface_instance;
    }

    /// Record validation error for diagnostics
    fn recordValidationError(self: *Self, struct_name: []const u8, interface_name: []const u8, validation_result: ValidationResult) !void {
        const error_info = InterfaceValidationError{
            .struct_name = try self.allocator.dupe(u8, struct_name),
            .interface_name = try self.allocator.dupe(u8, interface_name),
            .validation_result = validation_result,
        };
        try self.validation_errors.append(self.allocator, error_info);
    }

    /// Get comprehensive validation diagnostics
    pub fn getValidationDiagnostics(self: *Self) []const InterfaceValidationError {
        return self.validation_errors.items;
    }

    /// Get dispatch performance statistics
    pub fn getDispatchStatistics(self: *Self) DispatchStatistics {
        return self.dispatch_stats;
    }
};

/// Enhanced VTable structure with metadata
pub const VTable = struct {
    interface_name: []const u8,
    methods: []*FunctionValue,
    method_count: usize,
    allocator: Allocator,
    
    // Enhanced metadata
    creation_time: i64,
    access_count: u64,

    pub fn init(allocator: Allocator, interface_name: []const u8, method_count: usize) !VTable {
        const methods = try allocator.alloc(*FunctionValue, method_count);
        return VTable{
            .interface_name = try allocator.dupe(u8, interface_name),
            .methods = methods,
            .method_count = method_count,
            .allocator = allocator,
            .creation_time = std.time.timestamp(),
            .access_count = 0,
        };
    }

    pub fn deinit(self: *VTable) void {
        self.allocator.free(self.interface_name);
        self.allocator.free(self.methods);
    }

    pub fn getMethod(self: *VTable, index: usize) ?*FunctionValue {
        if (index >= self.method_count) return null;
        self.access_count += 1;
        return self.methods[index];
    }

    pub fn setMethod(self: *VTable, index: usize, function: *FunctionValue) !void {
        if (index >= self.method_count) return InterfaceDispatchError.InvalidMethodIndex;
        self.methods[index] = function;
    }
};

/// Enhanced Interface instance with GC integration
pub const InterfaceInstance = struct {
    underlying_object: *Value,
    vtable: *VTable,
    interface_name: []const u8,
    
    // GC integration fields
    gc_managed: bool,
    reference_count: u32,

    pub fn init(underlying_object: *Value, vtable: *VTable, interface_name: []const u8) InterfaceInstance {
        return InterfaceInstance{
            .underlying_object = underlying_object,
            .vtable = vtable,
            .interface_name = interface_name,
            .gc_managed = false,
            .reference_count = 1,
        };
    }

    pub fn callMethod(self: *InterfaceInstance, method_name: []const u8, args: []Value) !Value {
        // Find method in vtable and call it with proper method resolution
        for (self.vtable.methods, 0..) |method, index| {
            _ = index;
            if (std.mem.eql(u8, method.name, method_name)) {
                // Validate argument count
                if (args.len != method.parameter_count) {
                    return CursedError.ArgumentCountMismatch;
                }
                
                // Update access statistics
                self.vtable.access_count += 1;
                
                // Call the method with validated arguments
                return method.call(args);
            }
        }
        
        // Method not found in interface
        std.log.err("Method '{}' not found in interface '{}'", .{ method_name, self.interface_name });
        return CursedError.MethodNotFound;
    }
};

/// Enhanced Interface type definition
pub const InterfaceType = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    allocator: Allocator,
    
    // Enhanced metadata
    inheritance_chain: ArrayList([]const u8),
    attributes: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: Allocator, name: []const u8) InterfaceType {
        return InterfaceType{
            .name = name,
            .methods = .empty,
            .allocator = allocator,
            .inheritance_chain = .empty,
            .attributes = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }

    pub fn deinit(self: *InterfaceType) void {
        self.methods.deinit(self.allocator);
        self.inheritance_chain.deinit(self.allocator);
        self.attributes.deinit(self.allocator);
    }

    pub fn addMethod(self: *InterfaceType, method: MethodSignature) !void {
        try self.methods.append(self.allocator, method);
    }

    pub fn addParentInterface(self: *InterfaceType, parent_name: []const u8) !void {
        try self.inheritance_chain.append(self.allocator, try self.allocator.dupe(u8, parent_name));
    }
};

/// Method signature for interface definition with enhanced type information
pub const MethodSignature = struct {
    name: []const u8,
    parameter_types: [][]const u8,
    return_type: []const u8,
    
    // Enhanced signature information
    is_generic: bool,
    generic_constraints: ?[][]const u8,
    attributes: ?[][]const u8,
};

/// Method implementation with enhanced information
pub const MethodImpl = struct {
    name: []const u8,
    function: *FunctionValue,
    
    // Enhanced implementation information
    visibility: Visibility,
    is_override: bool,
    
    pub const Visibility = enum {
        Public,
        Protected,
        Private,
        Package,
    };
};

/// Enhanced diagnostic and error types
pub const ValidationResult = struct {
    valid: bool,
    missing_methods: ArrayList([]const u8),
    signature_mismatches: ArrayList(SignatureMismatch),
};

pub const SignatureMismatch = struct {
    method_name: []const u8,
    expected_signature: MethodSignature,
    actual_function: *FunctionValue,
    error_message: []const u8,
};

pub const SignatureCompatibilityResult = struct {
    compatible: bool,
    error_message: ?[]const u8,
};

pub const InterfaceValidationError = struct {
    struct_name: []const u8,
    interface_name: []const u8,
    validation_result: ValidationResult,
};

pub const DispatchStatistics = struct {
    total_calls: u64,
    cache_hits: u64,
    cache_misses: u64,
    errors: u64,
    
    pub fn init() DispatchStatistics {
        return DispatchStatistics{
            .total_calls = 0,
            .cache_hits = 0,
            .cache_misses = 0,
            .errors = 0,
        };
    }
    
    pub fn cacheHitRate(self: DispatchStatistics) f64 {
        if (self.total_calls == 0) return 0.0;
        return @as(f64, @floatFromInt(self.cache_hits)) / @as(f64, @floatFromInt(self.total_calls));
    }
};

/// Key types for lookups
pub const InterfaceImplKey = struct {
    struct_name: []const u8,
    interface_name: []const u8,
};

pub const InterfaceImplKeyContext = struct {
    pub fn hash(self: @This(), key: InterfaceImplKey) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(key.struct_name);
        hasher.update(key.interface_name);
        return hasher.final();
    }

    pub fn eql(self: @This(), a: InterfaceImplKey, b: InterfaceImplKey) bool {
        _ = self;
        return std.mem.eql(u8, a.struct_name, b.struct_name) and 
               std.mem.eql(u8, a.interface_name, b.interface_name);
    }
};

pub const ImplKey = struct {
    struct_name: []const u8,
    interface_name: []const u8,
};

pub const ImplKeyContext = struct {
    pub fn hash(self: @This(), key: ImplKey) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(key.struct_name);
        hasher.update(key.interface_name);
        return hasher.final();
    }

    pub fn eql(self: @This(), a: ImplKey, b: ImplKey) bool {
        _ = self;
        return std.mem.eql(u8, a.struct_name, b.struct_name) and 
               std.mem.eql(u8, a.interface_name, b.interface_name);
    }
};

pub const ImplementationInfo = struct {
    struct_name: []const u8,
    interface_name: []const u8,
    vtable: *VTable,
    method_count: usize,
};

/// Enhanced interface dispatch errors
pub const InterfaceDispatchError = error{
    InterfaceNotFound,
    ImplementationNotFound,
    MethodNotFound,
    MethodNotImplemented,
    IncompleteImplementation,
    InvalidStructType,
    InvalidMethodIndex,
    TypeMismatch,
    EmptyInterface,
    DuplicateMethod,
    SignatureIncompatible,
    AccessibilityViolation,
};

// Comprehensive tests
test "enhanced interface dispatch system" {
    const testing = std.testing;
    const allocator = testing.allocator;
    
    var interface_registry = InterfaceRegistry.init(allocator);
    defer interface_registry.deinit();
    
    var dispatcher = EnhancedInterfaceDispatcher.init(allocator, &interface_registry);
    defer dispatcher.deinit();
    
    // Test comprehensive interface registration and validation
    const drawable_methods = [_]MethodSignature{
        MethodSignature{
            .name = "draw",
            .parameter_types = &[_][]const u8{},
            .return_type = "void",
            .is_generic = false,
            .generic_constraints = null,
            .attributes = null,
        },
        MethodSignature{
            .name = "get_area",
            .parameter_types = &[_][]const u8{},
            .return_type = "normie",
            .is_generic = false,
            .generic_constraints = null,
            .attributes = null,
        },
    };
    
    try dispatcher.registerInterface("Drawable", &drawable_methods);
    
    // Verify interface registration
    try testing.expect(dispatcher.interface_types.contains("Drawable"));
    
    const drawable_type = dispatcher.interface_types.get("Drawable").?;
    try testing.expect(drawable_type.methods.items.len == 2);
    try testing.expect(std.mem.eql(u8, drawable_type.methods.items[0].name, "draw"));
    try testing.expect(std.mem.eql(u8, drawable_type.methods.items[1].name, "get_area"));
    
    // Test statistics
    const initial_stats = dispatcher.getDispatchStatistics();
    try testing.expect(initial_stats.total_calls == 0);
    try testing.expect(initial_stats.cache_hit_rate() == 0.0);
}

test "interface validation and error handling" {
    const testing = std.testing;
    const allocator = testing.allocator;
    
    var interface_registry = InterfaceRegistry.init(allocator);
    defer interface_registry.deinit();
    
    var dispatcher = EnhancedInterfaceDispatcher.init(allocator, &interface_registry);
    defer dispatcher.deinit();
    
    // Test empty interface error
    const empty_methods = [_]MethodSignature{};
    try testing.expectError(InterfaceDispatchError.EmptyInterface, 
                           dispatcher.registerInterface("Empty", &empty_methods));
    
    // Test duplicate method error
    const duplicate_methods = [_]MethodSignature{
        MethodSignature{
            .name = "test",
            .parameter_types = &[_][]const u8{},
            .return_type = "void",
            .is_generic = false,
            .generic_constraints = null,
            .attributes = null,
        },
        MethodSignature{
            .name = "test",
            .parameter_types = &[_][]const u8{},
            .return_type = "drip",
            .is_generic = false,
            .generic_constraints = null,
            .attributes = null,
        },
    };
    
    try testing.expectError(InterfaceDispatchError.DuplicateMethod, 
                           dispatcher.registerInterface("Duplicate", &duplicate_methods));
}
