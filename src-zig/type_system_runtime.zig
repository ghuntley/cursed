const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const interpreter = @import("interpreter.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;
const safeDupeString = error_handling.safeDupeString;

// Enhanced runtime type information
pub const RuntimeTypeInfo = struct {
    type_id: u32,
    type_name: []const u8,
    size: usize,
    alignment: usize,
    kind: TypeKind,
    fields: ?[]FieldInfo,
    methods: ?[]MethodInfo,
    allocator: Allocator,

    pub const TypeKind = enum {
        Basic,
        Struct,
        Interface,
        Array,
        Pointer,
        Function,
    };

    pub const FieldInfo = struct {
        name: []const u8,
        field_type: u32, // type_id
        offset: usize,
        is_public: bool,
    };

    pub const MethodInfo = struct {
        name: []const u8,
        return_type: u32, // type_id
        parameter_types: []u32, // type_ids
        is_virtual: bool,
        vtable_index: ?u32,
    };

    pub fn init(allocator: Allocator, type_id: u32, name: []const u8, kind: TypeKind) CursedError!RuntimeTypeInfo {
        const name_copy = try safeDupeString(allocator, name);
        
        return RuntimeTypeInfo{
            .type_id = type_id,
            .type_name = name_copy,
            .size = 0,
            .alignment = 1,
            .kind = kind,
            .fields = null,
            .methods = null,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *RuntimeTypeInfo) void {
        self.allocator.free(self.type_name);
        if (self.fields) |fields| {
            for (fields) |field| {
                self.allocator.free(field.name);
            }
            self.allocator.free(fields);
        }
        if (self.methods) |methods| {
            for (methods) |method| {
                self.allocator.free(method.name);
                self.allocator.free(method.parameter_types);
            }
            self.allocator.free(methods);
        }
    }

    pub fn setFields(self: *RuntimeTypeInfo, fields: []FieldInfo) void {
        self.fields = fields;
        self.calculateLayout();
    }

    pub fn setMethods(self: *RuntimeTypeInfo, methods: []MethodInfo) void {
        self.methods = methods;
    }

    pub fn getField(self: *RuntimeTypeInfo, name: []const u8) ?FieldInfo {
        if (self.fields) |fields| {
            for (fields) |field| {
                if (std.mem.eql(u8, field.name, name)) {
                    return field;
                }
            }
        }
        return null;
    }

    pub fn getMethod(self: *RuntimeTypeInfo, name: []const u8) ?MethodInfo {
        if (self.methods) |methods| {
            for (methods) |method| {
                if (std.mem.eql(u8, method.name, name)) {
                    return method;
                }
            }
        }
        return null;
    }

    fn calculateLayout(self: *RuntimeTypeInfo) void {
        if (self.fields) |fields| {
            var current_offset: usize = 0;
            var max_alignment: usize = 1;

            for (fields, 0..) |*field, i| {
                // Simple layout - each field gets 8 bytes for now
                field.offset = current_offset;
                current_offset += 8;
                max_alignment = @max(max_alignment, 8);
                _ = i;
            }

            self.size = current_offset;
            self.alignment = max_alignment;
        }
    }
};

// Enhanced garbage collector integration
pub const GCTypeRegistry = struct {
    types: HashMap(u32, RuntimeTypeInfo, std.hash_map.DefaultHashContext(u32), std.hash_map.default_max_load_percentage),
    type_id_counter: u32,
    allocator: Allocator,

    pub fn init(allocator: Allocator) GCTypeRegistry {
        return GCTypeRegistry{
            .types = HashMap(u32, RuntimeTypeInfo, std.hash_map.DefaultHashContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .type_id_counter = 1,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *GCTypeRegistry) void {
        var iter = self.types.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.types.deinit();
    }

    pub fn registerType(self: *GCTypeRegistry, name: []const u8, kind: RuntimeTypeInfo.TypeKind) !u32 {
        const type_id = self.type_id_counter;
        self.type_id_counter += 1;

        const type_info = RuntimeTypeInfo.init(self.allocator, type_id, name, kind);
        try self.types.put(type_id, type_info);
        return type_id;
    }

    pub fn getType(self: *GCTypeRegistry, type_id: u32) ?*RuntimeTypeInfo {
        return self.types.getPtr(type_id);
    }

    pub fn findTypeByName(self: *GCTypeRegistry, name: []const u8) ?*RuntimeTypeInfo {
        var iter = self.types.iterator();
        while (iter.next()) |entry| {
            if (std.mem.eql(u8, entry.value_ptr.type_name, name)) {
                return entry.value_ptr;
            }
        }
        return null;
    }
};

// Enhanced memory allocation with type safety
pub const TypedAllocator = struct {
    allocator: Allocator,
    gc_registry: *GCTypeRegistry,
    allocated_objects: ArrayList(*TypedObject),

    pub const TypedObject = struct {
        data: []u8,
        type_id: u32,
        ref_count: u32,
        mark: bool, // for GC marking

        pub fn init(allocator: Allocator, type_id: u32, size: usize) !*TypedObject {
            const object = try allocator.create(TypedObject);
            object.* = TypedObject{
                .data = try allocator.alloc(u8, size),
                .type_id = type_id,
                .ref_count = 1,
                .mark = false,
            };
            return object;
        }

        pub fn deinit(self: *TypedObject, allocator: Allocator) void {
            allocator.free(self.data);
            allocator.destroy(self);
        }

        pub fn retain(self: *TypedObject) void {
            self.ref_count += 1;
        }

        pub fn release(self: *TypedObject, allocator: Allocator) void {
            self.ref_count -= 1;
            if (self.ref_count == 0) {
                self.deinit(allocator);
            }
        }
    };

    pub fn init(allocator: Allocator, gc_registry: *GCTypeRegistry) TypedAllocator {
        return TypedAllocator{
            .allocator = allocator,
            .gc_registry = gc_registry,
            .allocated_objects = ArrayList(*TypedObject).init(allocator),
        };
    }

    pub fn deinit(self: *TypedAllocator) void {
        for (self.allocated_objects.items) |object| {
            object.deinit(self.allocator);
        }
        self.allocated_objects.deinit();
    }

    pub fn allocateStruct(self: *TypedAllocator, type_id: u32) !*TypedObject {
        const type_info = self.gc_registry.getType(type_id) orelse return error.UnknownType;
        const object = try TypedObject.init(self.allocator, type_id, type_info.size);
        try self.allocated_objects.append(object);
        return object;
    }

    pub fn getFieldPtr(self: *TypedAllocator, object: *TypedObject, field_name: []const u8) ![]u8 {
        const type_info = self.gc_registry.getType(object.type_id) orelse return error.UnknownType;
        const field_info = type_info.getField(field_name) orelse return error.UnknownField;
        
        if (field_info.offset + 8 > object.data.len) return error.InvalidOffset;
        return object.data[field_info.offset..field_info.offset + 8];
    }

    pub fn collectGarbage(self: *TypedAllocator) !void {
        // Mark phase
        for (self.allocated_objects.items) |object| {
            object.mark = false;
        }

        // Simple sweep phase - remove unreferenced objects
        var i: usize = 0;
        while (i < self.allocated_objects.items.len) {
            const object = self.allocated_objects.items[i];
            if (object.ref_count == 0) {
                object.deinit(self.allocator);
                _ = self.allocated_objects.swapRemove(i);
            } else {
                i += 1;
            }
        }
    }
};

// Interface implementation tracking
pub const InterfaceRegistry = struct {
    implementations: HashMap(InterfaceImplKey, VTablePtr, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub const InterfaceImplKey = struct {
        struct_type_id: u32,
        interface_type_id: u32,
    };

    pub const InterfaceImplKeyContext = struct {
        pub fn hash(self: @This(), key: InterfaceImplKey) u64 {
            _ = self;
            return @as(u64, key.struct_type_id) << 32 | @as(u64, key.interface_type_id);
        }

        pub fn eql(self: @This(), a: InterfaceImplKey, b: InterfaceImplKey) bool {
            _ = self;
            return a.struct_type_id == b.struct_type_id and a.interface_type_id == b.interface_type_id;
        }
    };

    pub const VTablePtr = *const VTable;

    pub const VTable = struct {
        interface_type_id: u32,
        method_pointers: []MethodPtr,

        pub const MethodPtr = *const fn (object: *TypedAllocator.TypedObject, args: []interpreter.Value) anyerror!interpreter.Value;
    };

    pub fn init(allocator: Allocator) InterfaceRegistry {
        return InterfaceRegistry{
            .implementations = HashMap(InterfaceImplKey, VTablePtr, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *InterfaceRegistry) void {
        self.implementations.deinit();
    }

    pub fn registerImplementation(self: *InterfaceRegistry, struct_type_id: u32, interface_type_id: u32, vtable: VTablePtr) !void {
        const key = InterfaceImplKey{
            .struct_type_id = struct_type_id,
            .interface_type_id = interface_type_id,
        };
        try self.implementations.put(key, vtable);
    }

    pub fn getVTable(self: *InterfaceRegistry, struct_type_id: u32, interface_type_id: u32) ?VTablePtr {
        const key = InterfaceImplKey{
            .struct_type_id = struct_type_id,
            .interface_type_id = interface_type_id,
        };
        return self.implementations.get(key);
    }
};

// Type checking utilities
pub const TypeChecker = struct {
    gc_registry: *GCTypeRegistry,
    interface_registry: *InterfaceRegistry,

    pub fn init(gc_registry: *GCTypeRegistry, interface_registry: *InterfaceRegistry) TypeChecker {
        return TypeChecker{
            .gc_registry = gc_registry,
            .interface_registry = interface_registry,
        };
    }

    pub fn checkStructFieldAccess(self: *TypeChecker, struct_type_id: u32, field_name: []const u8) bool {
        const type_info = self.gc_registry.getType(struct_type_id) orelse return false;
        return type_info.getField(field_name) != null;
    }

    pub fn checkInterfaceImplementation(self: *TypeChecker, struct_type_id: u32, interface_type_id: u32) bool {
        return self.interface_registry.getVTable(struct_type_id, interface_type_id) != null;
    }

    pub fn checkMethodCall(self: *TypeChecker, object_type_id: u32, method_name: []const u8) bool {
        const type_info = self.gc_registry.getType(object_type_id) orelse return false;
        return type_info.getMethod(method_name) != null;
    }

    pub fn areTypesCompatible(self: *TypeChecker, source_type_id: u32, target_type_id: u32) bool {
        if (source_type_id == target_type_id) return true;

        const source_type = self.gc_registry.getType(source_type_id) orelse return false;
        const target_type = self.gc_registry.getType(target_type_id) orelse return false;

        // Check if source struct implements target interface
        if (source_type.kind == .Struct and target_type.kind == .Interface) {
            return self.checkInterfaceImplementation(source_type_id, target_type_id);
        }

        return false;
    }
};
