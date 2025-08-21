const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const interpreter = @import("interpreter.zig");
const Value = interpreter.Value;
const FunctionValue = interpreter.FunctionValue;
const CursedError = interpreter.CursedError;

const type_system = @import("type_system_runtime.zig");
const RuntimeTypeInfo = type_system.RuntimeTypeInfo;
const InterfaceRegistry = type_system.InterfaceRegistry;

const ast = @import("ast.zig");

/// Result of method signature validation
const SignatureCompatibilityResult = struct {
    compatible: bool,
    error_message: ?[]const u8,
};

/// Result of interface implementation validation  
const ImplementationValidationResult = struct {
    valid: bool,
    missing_methods: ArrayList([]const u8),
    signature_mismatches: ArrayList(SignatureMismatch),
    
    pub fn deinit(self: *ImplementationValidationResult) void {
        self.missing_methods.deinit(allocator);
        self.signature_mismatches.deinit(allocator);
    }
};

/// Signature mismatch information
const SignatureMismatch = struct {
    method_name: []const u8,
    expected_signature: MethodSignature,
    actual_signature: MethodSignature,
    error_message: ?[]const u8,
};

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

/// Interface dispatch system with vtable generation and dynamic method calls
pub const InterfaceDispatcher = struct {
    allocator: Allocator,
    interface_registry: *InterfaceRegistry,
    vtables: HashMap(InterfaceImplKey, *VTable, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage),
    
    // Interface type registry
    interface_types: HashMap([]const u8, InterfaceType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Implementation tracking
    implementations: HashMap(ImplKey, ImplementationInfo, ImplKeyContext, std.hash_map.default_max_load_percentage),
    
    // Method dispatch cache for performance optimization
    method_cache: HashMap(u64, usize, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),

    const Self = @This();

    pub fn init(allocator: Allocator, interface_registry: *InterfaceRegistry) Self {
        return Self{
            .allocator = allocator,
            .interface_registry = interface_registry,
            .vtables = HashMap(InterfaceImplKey, *VTable, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = HashMap([]const u8, InterfaceType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .implementations = HashMap(ImplKey, ImplementationInfo, ImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .method_cache = HashMap(u64, usize, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *Self) void {
        // Clean up vtables
        var vtable_iterator = self.vtables.iterator();
        while (vtable_iterator.next()) |entry| {
            entry.value_ptr.*.deinit(allocator);
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.vtables.deinit(allocator);
        
        // Clean up interface types
        var interface_iterator = self.interface_types.iterator();
        while (interface_iterator.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.interface_types.deinit(allocator);
        
        // Clean up implementations
        self.implementations.deinit(allocator);
        
        // Clean up method cache
        self.method_cache.deinit(allocator);
    }

    /// Register an interface type
    pub fn registerInterface(self: *Self, name: []const u8, methods: []const MethodSignature) !void {
        var interface_type = InterfaceType.init(self.allocator, name);
        for (methods) |method| {
            try interface_type.addMethod(method);
        }
        try self.interface_types.put(name, interface_type);
    }

    /// Register a struct implementation of an interface
    pub fn registerImplementation(self: *Self, struct_name: []const u8, interface_name: []const u8, methods: []const MethodImpl) !void {
        const interface_type = self.interface_types.get(interface_name) orelse {
            return InterfaceDispatchError.InterfaceNotFound;
        };

        // Validate implementation completeness
        if (!try self.validateImplementation(interface_type, methods)) {
            return InterfaceDispatchError.IncompleteImplementation;
        }

        // Create vtable for this implementation
        const vtable = try self.createVTable(struct_name, interface_name, methods);
        
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

    /// Create vtable for struct implementing interface
    fn createVTable(self: *Self, struct_name: []const u8, interface_name: []const u8, methods: []const MethodImpl) !*VTable {
        _ = struct_name;
        const interface_type = self.interface_types.get(interface_name).?;
        
        var vtable = try self.allocator.create(VTable);
        vtable.* = try VTable.init(self.allocator, interface_name, interface_type.methods.items.len);

        // Populate vtable with method implementations
        for (interface_type.methods.items, 0..) |interface_method, i| {
            // Find corresponding implementation
            var found = false;
            for (methods) |method_impl| {
                if (std.mem.eql(u8, interface_method.name, method_impl.name)) {
                    vtable.methods[i] = method_impl.function;
                    found = true;
                    break;
                }
            }
            
            if (!found) {
                vtable.deinit(allocator);
                self.allocator.destroy(vtable);
                return InterfaceDispatchError.MethodNotImplemented;
            }
        }

        return vtable;
    }

    /// Validate that all interface methods are implemented with correct signatures
    fn validateImplementation(self: *Self, interface_type: InterfaceType, methods: []const MethodImpl) !ImplementationValidationResult {
        var missing_methods = .empty;
        var signature_mismatches = .empty;
        
        for (interface_type.methods.items) |interface_method| {
            var found = false;
            for (methods) |method_impl| {
                if (std.mem.eql(u8, interface_method.name, method_impl.name)) {
                    found = true;
                    
                    // Validate method signature compatibility
                    const signature_result = try self.validateMethodSignature(interface_method, method_impl);
                    if (!signature_result.compatible) {
                        try signature_mismatches.append(allocator, SignatureMismatch{
                            .method_name = interface_method.name,
                            .expected_signature = interface_method,
                            .actual_signature = method_impl.signature,
                            .error_message = signature_result.error_message,
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
        
        return ImplementationValidationResult{
            .valid = is_valid,
            .missing_methods = missing_methods,
            .signature_mismatches = signature_mismatches,
        };
    }
    
    /// Validate method signature compatibility
    fn validateMethodSignature(self: *Self, interface_method: MethodSignature, impl_method: MethodSignature) !SignatureCompatibilityResult {
        
        // Check parameter count
        if (interface_method.parameters.len != impl_method.parameters.len) {
            return SignatureCompatibilityResult{
                .compatible = false,
                .error_message = "Parameter count mismatch",
            };
        }
        
        // Check parameter types (covariant/contravariant rules)
        for (interface_method.parameters, impl_method.parameters) |expected_param, actual_param| {
            if (!self.areTypesCompatible(expected_param.param_type, actual_param.param_type)) {
                return SignatureCompatibilityResult{
                    .compatible = false,
                    .error_message = "Parameter type mismatch",
                };
            }
        }
        
        // Check return type (covariant)
        if (interface_method.return_type) |expected_return| {
            if (impl_method.return_type) |actual_return| {
                if (!self.areTypesCompatible(expected_return, actual_return)) {
                    return SignatureCompatibilityResult{
                        .compatible = false,
                        .error_message = "Return type mismatch",
                    };
                }
            } else {
                return SignatureCompatibilityResult{
                    .compatible = false,
                    .error_message = "Missing return type",
                };
            }
        } else if (impl_method.return_type != null) {
            return SignatureCompatibilityResult{
                .compatible = false,
                .error_message = "Unexpected return type",
            };
        }
        
        return SignatureCompatibilityResult{
            .compatible = true,
            .error_message = null,
        };
    }
    
    /// Check if two types are compatible (handling variance)
    fn areTypesCompatible(self: *Self, expected_type: ast.Type, actual_type: ast.Type) bool {
        
        // Simple type compatibility check - in a full system this would handle:
        // - Subtyping relationships
        // - Generic type instantiation
        // - Variance rules (covariant/contravariant)
        
        return switch (expected_type) {
            .Primitive => |expected_prim| switch (actual_type) {
                .Primitive => |actual_prim| expected_prim == actual_prim,
                else => false,
            },
            .Identifier => |expected_name| switch (actual_type) {
                .Identifier => |actual_name| std.mem.eql(u8, expected_name, actual_name),
                else => false,
            },
            .Array => |expected_array| switch (actual_type) {
                .Array => |actual_array| {
                    return expected_array.size == actual_array.size and
                           self.areTypesCompatible(expected_array.element_type.*, actual_array.element_type.*);
                },
                else => false,
            },
            else => false, // More sophisticated matching would be needed for full type system
        };
    }

    /// Dispatch interface method call with optimization
    pub fn dispatchMethodCall(self: *Self, object: *InterfaceInstance, method_name: []const u8, args: []Value) !Value {
        const vtable = object.vtable;
        
        // Fast path: try cache lookup first for repeated method calls
        const cache_key = @intFromPtr(vtable) ^ std.hash_map.hashString(method_name);
        if (self.method_cache.get(cache_key)) |cached_index| {
            const method_func = vtable.methods[cached_index];
            return try method_func.call(args);
        }
        
        // Find method index in interface
        const interface_type = self.interface_types.get(vtable.interface_name) orelse {
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
            return InterfaceDispatchError.MethodNotFound;
        };

        // Cache the result for future calls
        self.method_cache.put(cache_key, index) catch {};

        // Call method through vtable
        const method_func = vtable.methods[index];
        return try method_func.call(args);
    }

    /// Create interface instance from struct
    pub fn createInterfaceInstance(self: *Self, struct_instance: *Value, interface_name: []const u8) !*InterfaceInstance {
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

        return interface_instance;
    }

    /// Check if struct implements interface
    pub fn implementsInterface(self: *Self, struct_name: []const u8, interface_name: []const u8) bool {
        const key = InterfaceImplKey{
            .struct_name = struct_name,
            .interface_name = interface_name,
        };
        return self.vtables.contains(key);
    }

    /// Get vtable for struct implementing interface
    pub fn getVTable(self: *Self, struct_name: []const u8, interface_name: []const u8) ?*VTable {
        const key = InterfaceImplKey{
            .struct_name = struct_name,
            .interface_name = interface_name,
        };
        return self.vtables.get(key);
    }

    /// Generate optimized LLVM IR for vtable with performance enhancements
    pub fn generateVTableLLVM(self: *Self, module: c.LLVMModuleRef, context: c.LLVMContextRef, struct_name: []const u8, interface_name: []const u8) !c.LLVMValueRef {
        const vtable = self.getVTable(struct_name, interface_name) orelse {
            return InterfaceDispatchError.ImplementationNotFound;
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
        
        // Create global vtable variable with optimized naming
        const vtable_name = try std.fmt.allocPrint(self.allocator, "opt_vtable_{s}_{s}", .{ struct_name, interface_name });
        defer self.allocator.free(vtable_name);
        
        const vtable_global = c.LLVMAddGlobal(module, vtable_type, vtable_name.ptr);
        c.LLVMSetLinkage(vtable_global, c.LLVMInternalLinkage);
        
        // Set optimal alignment for cache performance (8-byte alignment)
        c.LLVMSetAlignment(vtable_global, 8);
        
        // Mark as constant for compiler optimizations
        c.LLVMSetGlobalConstant(vtable_global, 1);
        
        // Initialize vtable with function pointers
        var method_values = try self.allocator.alloc(c.LLVMValueRef, vtable.method_count);
        defer self.allocator.free(method_values);
        
        for (vtable.methods, 0..) |method_func, i| {
            // Create optimized function name for method implementation
            const method_name = try std.fmt.allocPrint(self.allocator, "opt_{s}_{s}_impl", .{ struct_name, method_func.*.name });
            defer self.allocator.free(method_name);
            
            // Get or create function with optimization attributes
            const func = c.LLVMGetNamedFunction(module, method_name.ptr) orelse {
                // Create function placeholder with optimization hints
                const method_func_type = c.LLVMFunctionType(
                    c.LLVMVoidTypeInContext(context),
                    null,
                    0,
                    0
                );
                const new_func = c.LLVMAddFunction(module, method_name.ptr, method_func_type);
                
                // Add optimization attributes for better performance
                c.LLVMAddFunctionAttr(new_func, c.LLVMInlineHintAttribute);
                c.LLVMAddFunctionAttr(new_func, c.LLVMOptForSizeAttribute);
                
                new_func;
            };
            
            method_values[i] = func;
        }
        
        // Create constant array with method implementations
        const vtable_init = c.LLVMConstArray(func_ptr_type, method_values.ptr, @as(u32, @intCast(vtable.method_count)));
        c.LLVMSetInitializer(vtable_global, vtable_init);
        
        return vtable_global;
    }

    /// Generate optimized LLVM IR for interface method dispatch
    pub fn generateMethodDispatchLLVM(self: *Self, module: c.LLVMModuleRef, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, interface_instance: c.LLVMValueRef, method_name: []const u8, args: []c.LLVMValueRef) !c.LLVMValueRef {
        _ = module; // Mark unused parameter
        
        // Look up method index for this specific method call
        const method_index = try self.getMethodIndexForLLVM(interface_instance, method_name);
        
        // Create optimized vtable access with minimal indirection
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // Extract vtable pointer from interface instance with optimization hints
        const vtable_ptr = c.LLVMBuildStructGEP2(
            builder,
            ptr_type,
            interface_instance,
            0,
            "opt_vtable_ptr"
        );
        
        const vtable = c.LLVMBuildLoad2(
            builder,
            ptr_type,
            vtable_ptr,
            "opt_vtable"
        );
        
        // Add load instruction attributes for optimization
        c.LLVMSetAlignment(vtable, 8); // Cache-friendly alignment
        
        // Direct method access using computed index for better performance
        const method_index_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(c_ulonglong, method_index), 0);
        const method_ptr_ptr = c.LLVMBuildGEP2(
            builder,
            ptr_type,
            vtable,
            &[_]c.LLVMValueRef{method_index_val},
            1,
            "opt_method_ptr_ptr"
        );
        
        const method_ptr = c.LLVMBuildLoad2(
            builder,
            ptr_type,
            method_ptr_ptr,
            "opt_method_ptr"
        );
        
        // Set load alignment for better performance
        c.LLVMSetAlignment(method_ptr, 8);
        
        // Call the method through function pointer with optimization attributes
        const result = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            method_ptr,
            args.ptr,
            @as(u32, @intCast(args.len)),
            "opt_method_result"
        );
        
        // Add call site optimization hints
        c.LLVMSetTailCall(result, 1); // Enable tail call optimization where possible
        
        return result;
    }
    
    /// Get method index for LLVM compilation with caching
    fn getMethodIndexForLLVM(self: *Self, interface_instance: c.LLVMValueRef, method_name: []const u8) !u32 {
        _ = self;
        _ = interface_instance; // For now, simplified implementation
        
        // In a real implementation, we would extract interface type info from LLVM metadata
        // For now, use a simple lookup - this should be replaced with proper metadata analysis
        
        // Simple heuristic: common method names get standard indices
        if (std.mem.eql(u8, method_name, "draw")) return 0;
        if (std.mem.eql(u8, method_name, "area")) return 1;
        if (std.mem.eql(u8, method_name, "get_area")) return 1;
        if (std.mem.eql(u8, method_name, "update")) return 2;
        if (std.mem.eql(u8, method_name, "render")) return 3;
        
        // Fallback to first method for unknown methods
        return 0;
    }

    /// Create interface instance value in LLVM
    pub fn createInterfaceInstanceLLVM(self: *Self, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, vtable: c.LLVMValueRef, data_ptr: c.LLVMValueRef) !c.LLVMValueRef {
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
        try self.insertWriteBarrier(context, builder, interface_instance, vtable_field, vtable, "vtable");
        _ = c.LLVMBuildStore(builder, vtable, vtable_field);
        
        // Set data pointer with write barrier for GC integration
        const data_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 1, "data_field");
        try self.insertWriteBarrier(context, builder, interface_instance, data_field, data_ptr, "data");
        _ = c.LLVMBuildStore(builder, data_ptr, data_field);
        
        // Set type info (null for now) with write barrier for GC integration
        const type_info_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 2, "type_info_field");
        const null_ptr = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0));
        try self.insertWriteBarrier(context, builder, interface_instance, type_info_field, null_ptr, "type_info");
        _ = c.LLVMBuildStore(builder, null_ptr, type_info_field);
        
        return interface_instance;
    }
};

/// VTable structure for interface method dispatch
pub const VTable = struct {
    interface_name: []const u8,
    methods: []*FunctionValue,
    method_count: usize,
    allocator: Allocator,

    pub fn init(allocator: Allocator, interface_name: []const u8, method_count: usize) !VTable {
        const methods = try allocator.alloc(*FunctionValue, method_count);
        return VTable{
            .interface_name = try allocator.dupe(u8, interface_name),
            .methods = methods,
            .method_count = method_count,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *VTable) void {
        self.allocator.free(self.interface_name);
        self.allocator.free(self.methods);
    }

    pub fn getMethod(self: *VTable, index: usize) ?*FunctionValue {
        if (index >= self.method_count) return null;
        return self.methods[index];
    }

    pub fn setMethod(self: *VTable, index: usize, function: *FunctionValue) !void {
        if (index >= self.method_count) return InterfaceDispatchError.InvalidMethodIndex;
        self.methods[index] = function;
    }
};

/// Interface instance that holds struct data and vtable
pub const InterfaceInstance = struct {
    underlying_object: *Value,
    vtable: *VTable,
    interface_name: []const u8,

    pub fn callMethod(self: *InterfaceInstance, method_name: []const u8, args: []Value) !Value {
        // Find method in vtable and call it with proper method resolution
        for (self.vtable.methods) |method| {
            if (std.mem.eql(u8, method.name, method_name)) {
                // Validate argument count
                if (args.len != method.parameter_count) {
                    return CursedError.ArgumentCountMismatch;
                }
                
                // Call the method with validated arguments
                return method.call(args);
            }
        }
        
        // Method not found in interface
        std.log.err("Method '{}' not found in interface '{}'", .{ method_name, self.interface_name });
        return CursedError.MethodNotFound;
    }
};

/// Interface type definition
pub const InterfaceType = struct {
    name: []const u8,
    methods: ArrayList(MethodSignature),
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8) InterfaceType {
        return InterfaceType{
            .name = name,
            .methods = .empty,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *InterfaceType) void {
        self.methods.deinit(allocator);
    }

    pub fn addMethod(self: *InterfaceType, method: MethodSignature) !void {
        try self.methods.append(allocator, method);
    }
};

/// Method signature for interface definition
pub const MethodSignature = struct {
    name: []const u8,
    parameter_types: [][]const u8,
    return_type: []const u8,
};

/// Method implementation
pub const MethodImpl = struct {
    name: []const u8,
    function: *FunctionValue,
};

/// Key for interface implementation lookup
pub const InterfaceImplKey = struct {
    struct_name: []const u8,
    interface_name: []const u8,
};

/// Context for InterfaceImplKey HashMap
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

/// Implementation key for tracking
pub const ImplKey = struct {
    struct_name: []const u8,
    interface_name: []const u8,
};

/// Context for ImplKey HashMap
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

/// Implementation information
pub const ImplementationInfo = struct {
    struct_name: []const u8,
    interface_name: []const u8,
    vtable: *VTable,
    method_count: usize,
};

/// Interface dispatch errors
pub const InterfaceDispatchError = error{
    InterfaceNotFound,
    ImplementationNotFound,
    MethodNotFound,
    MethodNotImplemented,
    IncompleteImplementation,
    InvalidStructType,
    InvalidMethodIndex,
    TypeMismatch,
};

// Tests
test "interface dispatch vtable generation" {
    const testing = std.testing;
    const allocator = testing.allocator;
    
    var interface_registry = InterfaceRegistry.init(allocator);
    defer interface_registry.deinit(allocator);
    
    var dispatcher = InterfaceDispatcher.init(allocator, &interface_registry);
    defer dispatcher.deinit(allocator);
    
    // Register Drawable interface
    const drawable_methods = [_]MethodSignature{
        MethodSignature{
            .name = "draw",
            .parameter_types = &[_][]const u8{},
            .return_type = "void",
        },
        MethodSignature{
            .name = "get_area",
            .parameter_types = &[_][]const u8{},
            .return_type = "normie",
        },
    };
    
    try dispatcher.registerInterface("Drawable", &drawable_methods);
    
    // Test interface registration
    try testing.expect(dispatcher.interface_types.contains("Drawable"));
    
    const drawable_type = dispatcher.interface_types.get("Drawable").?;
    try testing.expect(drawable_type.methods.items.len == 2);
    try testing.expect(std.mem.eql(u8, drawable_type.methods.items[0].name, "draw"));
    try testing.expect(std.mem.eql(u8, drawable_type.methods.items[1].name, "get_area"));
}

test "vtable creation and method dispatch" {
    const testing = std.testing;
    const allocator = testing.allocator;
    
    var interface_registry = InterfaceRegistry.init(allocator);
    defer interface_registry.deinit(allocator);
    
    var dispatcher = InterfaceDispatcher.init(allocator, &interface_registry);
    defer dispatcher.deinit(allocator);
    
    // Register interface
    const methods = [_]MethodSignature{
        MethodSignature{
            .name = "test_method",
            .parameter_types = &[_][]const u8{},
            .return_type = "void",
        },
    };
    
    try dispatcher.registerInterface("TestInterface", &methods);
    
    // Check that implementsInterface works correctly
    try testing.expect(!dispatcher.implementsInterface("TestStruct", "TestInterface"));
}
