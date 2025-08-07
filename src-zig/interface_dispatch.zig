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

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

/// Interface dispatch system with vtable generation and dynamic method calls
pub const InterfaceDispatcher = struct {
    allocator: Allocator,
    interface_registry: *InterfaceRegistry,
    vtables: HashMap(InterfaceImplKey, *VTable, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage),
    
    // Interface type registry
    interface_types: HashMap([]const u8, InterfaceType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Implementation tracking
    implementations: HashMap(ImplKey, ImplementationInfo, ImplKeyContext, std.hash_map.default_max_load_percentage),

    const Self = @This();

    pub fn init(allocator: Allocator, interface_registry: *InterfaceRegistry) Self {
        return Self{
            .allocator = allocator,
            .interface_registry = interface_registry,
            .vtables = HashMap(InterfaceImplKey, *VTable, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = HashMap([]const u8, InterfaceType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .implementations = HashMap(ImplKey, ImplementationInfo, ImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *Self) void {
        // Clean up vtables
        var vtable_iterator = self.vtables.iterator();
        while (vtable_iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
            self.allocator.destroy(entry.value_ptr.*);
        }
        self.vtables.deinit();
        
        // Clean up interface types
        var interface_iterator = self.interface_types.iterator();
        while (interface_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.interface_types.deinit();
        
        // Clean up implementations
        self.implementations.deinit();
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
                vtable.deinit();
                self.allocator.destroy(vtable);
                return InterfaceDispatchError.MethodNotImplemented;
            }
        }

        return vtable;
    }

    /// Validate that all interface methods are implemented
    fn validateImplementation(self: *Self, interface_type: InterfaceType, methods: []const MethodImpl) !bool {
        _ = self;
        
        for (interface_type.methods.items) |interface_method| {
            var found = false;
            for (methods) |method_impl| {
                if (std.mem.eql(u8, interface_method.name, method_impl.name)) {
                    // TODO: Add signature validation
                    found = true;
                    break;
                }
            }
            if (!found) return false;
        }
        return true;
    }

    /// Dispatch interface method call
    pub fn dispatchMethodCall(self: *Self, object: *InterfaceInstance, method_name: []const u8, args: []Value) !Value {
        const vtable = object.vtable;
        
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

    /// Generate LLVM IR for vtable with complete implementation
    pub fn generateVTableLLVM(self: *Self, module: c.LLVMModuleRef, context: c.LLVMContextRef, struct_name: []const u8, interface_name: []const u8) !c.LLVMValueRef {
        const vtable = self.getVTable(struct_name, interface_name) orelse {
            return InterfaceDispatchError.ImplementationNotFound;
        };

        // Create vtable type (array of function pointers)
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
        
        // Create global vtable variable
        const vtable_name = try std.fmt.allocPrint(self.allocator, "vtable_{s}_{s}", .{ struct_name, interface_name });
        defer self.allocator.free(vtable_name);
        
        const vtable_global = c.LLVMAddGlobal(module, vtable_type, vtable_name.ptr);
        c.LLVMSetLinkage(vtable_global, c.LLVMInternalLinkage);
        
        // Initialize vtable with function pointers
        var method_values = try self.allocator.alloc(c.LLVMValueRef, vtable.method_count);
        defer self.allocator.free(method_values);
        
        for (vtable.methods, 0..) |method_func, i| {
            // Create function name for method implementation
            const method_name = try std.fmt.allocPrint(self.allocator, "{s}_{s}_impl", .{ struct_name, method_func.*.name });
            defer self.allocator.free(method_name);
            
            // Get or create function
            const func = c.LLVMGetNamedFunction(module, method_name.ptr) orelse {
                // Create function placeholder if not exists
                const method_func_type = c.LLVMFunctionType(
                    c.LLVMVoidTypeInContext(context),
                    null,
                    0,
                    0
                );
                c.LLVMAddFunction(module, method_name.ptr, method_func_type);
            };
            
            method_values[i] = func;
        }
        
        // Create constant array with method implementations
        const vtable_init = c.LLVMConstArray(func_ptr_type, method_values.ptr, @as(u32, @intCast(vtable.method_count)));
        c.LLVMSetInitializer(vtable_global, vtable_init);
        
        return vtable_global;
    }

    /// Generate LLVM IR for interface method dispatch
    pub fn generateMethodDispatchLLVM(self: *Self, module: c.LLVMModuleRef, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, interface_instance: c.LLVMValueRef, method_name: []const u8, args: []c.LLVMValueRef) !c.LLVMValueRef {
        _ = self; // Mark unused parameter
        _ = module; // Mark unused parameter
        _ = method_name; // Mark unused parameter
        // Extract vtable pointer from interface instance
        const vtable_ptr = c.LLVMBuildStructGEP2(
            builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            interface_instance,
            0,
            "vtable_ptr"
        );
        
        const vtable = c.LLVMBuildLoad2(
            builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            vtable_ptr,
            "vtable"
        );
        
        // Find method index (simplified - in practice would use metadata)
        const method_index = 0; // TODO: Look up actual method index
        
        // Get method function pointer from vtable
        const method_ptr_ptr = c.LLVMBuildGEP2(
            builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            vtable,
            &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(context), method_index, 0)},
            1,
            "method_ptr_ptr"
        );
        
        const method_ptr = c.LLVMBuildLoad2(
            builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            method_ptr_ptr,
            "method_ptr"
        );
        
        // Call the method through function pointer
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

    /// Create interface instance value in LLVM
    pub fn createInterfaceInstanceLLVM(self: *Self, context: c.LLVMContextRef, builder: c.LLVMBuilderRef, vtable: c.LLVMValueRef, data_ptr: c.LLVMValueRef) !c.LLVMValueRef {
        _ = self; // Mark unused parameter
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
        
        // Set vtable pointer
        const vtable_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 0, "vtable_field");
        _ = c.LLVMBuildStore(builder, vtable, vtable_field);
        
        // Set data pointer
        const data_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 1, "data_field");
        _ = c.LLVMBuildStore(builder, data_ptr, data_field);
        
        // Set type info (null for now)
        const type_info_field = c.LLVMBuildStructGEP2(builder, interface_type, interface_instance, 2, "type_info_field");
        const null_ptr = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0));
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
        _ = method_name;
        // Find method in vtable and call it
        // This is a simplified version - real implementation would need method index lookup
        return self.vtable.methods[0].call(args); // TODO: Proper method resolution
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
            .methods = ArrayList(MethodSignature).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *InterfaceType) void {
        self.methods.deinit();
    }

    pub fn addMethod(self: *InterfaceType, method: MethodSignature) !void {
        try self.methods.append(method);
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
    defer interface_registry.deinit();
    
    var dispatcher = InterfaceDispatcher.init(allocator, &interface_registry);
    defer dispatcher.deinit();
    
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
    defer interface_registry.deinit();
    
    var dispatcher = InterfaceDispatcher.init(allocator, &interface_registry);
    defer dispatcher.deinit();
    
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
