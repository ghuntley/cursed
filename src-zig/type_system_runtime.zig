const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const atomic = std.atomic;

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

// Import collision handling system
const type_collision = @import("type_id_collision_handling.zig");
const CollisionResistantTypeRegistry = type_collision.CollisionResistantTypeRegistry;
const TypeId = type_collision.TypeId;

// Enhanced garbage collector integration with collision-resistant type registry
pub const GCTypeRegistry = struct {
    /// Legacy interface for compatibility
    types: std.HashMap(u32, RuntimeTypeInfo, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    
    /// New collision-resistant registry
    collision_resistant_registry: CollisionResistantTypeRegistry,
    
    /// Legacy type ID counter
    type_id_counter: u32,
    
    /// Type ID mapping for legacy compatibility
    legacy_id_mapping: std.HashMap(u32, TypeId, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    
    allocator: Allocator,

    pub fn init() GCTypeRegistry {
        return GCTypeRegistry{
            .types = std.HashMap(u32, RuntimeTypeInfo, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .collision_resistant_registry = CollisionResistantTypeRegistry.init(allocator),
            .type_id_counter = 1,
            .legacy_id_mapping = std.HashMap(u32, TypeId, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *GCTypeRegistry) void {
        var iter = self.types.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.types.deinit();
        
        self.collision_resistant_registry.deinit();
        
        var mapping_iter = self.legacy_id_mapping.iterator();
        while (mapping_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.legacy_id_mapping.deinit();
    }

    /// Enhanced type registration with collision detection
    pub fn registerType(self: *GCTypeRegistry, name: []const u8, kind: RuntimeTypeInfo.TypeKind) !u32 {
        // Check for existing type first
        if (self.findTypeByName(name)) |existing| {
            std.log.info("Type '{s}' already exists with ID {d}, returning existing ID", .{name, existing.type_id});
            return existing.type_id;
        }
        
        const legacy_type_id = self.type_id_counter;
        self.type_id_counter += 1;

        const type_info = try RuntimeTypeInfo.init(self.allocator, legacy_type_id, name, kind);
        
        // Register in both systems for compatibility
        try self.types.put(legacy_type_id, type_info);
        
        // Register in collision-resistant system
        const new_type_id = try self.collision_resistant_registry.registerType(name, type_info);
        try self.legacy_id_mapping.put(legacy_type_id, new_type_id);
        
        // Validate registry integrity periodically
        if (self.type_id_counter % 100 == 0) {
            if (!try self.collision_resistant_registry.validateIntegrity()) {
                std.log.err("Type registry integrity validation failed at type ID {d}", .{legacy_type_id});
            }
        }
        
        // Check if rehashing is needed
        try self.collision_resistant_registry.rehashIfNeeded();
        
        return legacy_type_id;
    }

    /// Enhanced type lookup with collision-aware search
    pub fn getType(self: *GCTypeRegistry, type_id: u32) ?*RuntimeTypeInfo {
        // Try legacy lookup first for compatibility
        if (self.types.getPtr(type_id)) |legacy_type| {
            return legacy_type;
        }
        
        // Try collision-resistant lookup via mapping
        if (self.legacy_id_mapping.get(type_id)) |new_type_id| {
            return self.collision_resistant_registry.getTypeById(new_type_id);
        }
        
        return null;
    }

    /// Enhanced name-based lookup with collision detection
    pub fn findTypeByName(self: *GCTypeRegistry, name: []const u8) ?*RuntimeTypeInfo {
        // First try collision-resistant registry
        if (self.collision_resistant_registry.getTypeByName(name)) |found| {
            return found;
        }
        
        // Fallback to legacy search
        var iter = self.types.iterator();
        while (iter.next()) |entry| {
            if (std.mem.eql(u8, entry.value_ptr.type_name, name)) {
                return entry.value_ptr;
            }
        }
        return null;
    }
    
    /// Get collision statistics for monitoring
    pub fn getCollisionStats(self: *GCTypeRegistry) type_collision.CollisionResistantTypeRegistry.CollisionStats {
        return self.collision_resistant_registry.getCollisionStats();
    }
    
    /// Validate the integrity of both registries
    pub fn validateIntegrity(self: *GCTypeRegistry) !bool {
        // Validate collision-resistant registry
        if (!try self.collision_resistant_registry.validateIntegrity()) {
            return false;
        }
        
        // Cross-validate legacy mappings
        var mapping_iter = self.legacy_id_mapping.iterator();
        var validation_errors: u32 = 0;
        
        while (mapping_iter.next()) |entry| {
            const legacy_id = entry.key_ptr.*;
            const new_type_id = entry.value_ptr.*;
            
            const legacy_type = self.types.get(legacy_id);
            const new_type = self.collision_resistant_registry.getTypeById(new_type_id);
            
            if (legacy_type == null or new_type == null) {
                std.log.err("Mapping inconsistency: legacy_id={d} exists in legacy={} new={}", 
                          .{legacy_id, legacy_type != null, new_type != null});
                validation_errors += 1;
                continue;
            }
            
            if (!std.mem.eql(u8, legacy_type.?.type_name, new_type.?.type_name)) {
                std.log.err("Name mismatch in mapping: legacy='{s}' new='{s}'", 
                          .{legacy_type.?.type_name, new_type.?.type_name});
                validation_errors += 1;
            }
        }
        
        if (validation_errors > 0) {
            std.log.err("GCTypeRegistry validation failed with {d} mapping errors", .{validation_errors});
            return false;
        }
        
        return true;
    }
    
    /// Print detailed registry statistics
    pub fn debugPrintStats(self: *GCTypeRegistry) void {
        std.log.info("=== GCTypeRegistry Statistics ===", .{});
        std.log.info("Legacy types registered: {d}", .{self.types.count()});
        std.log.info("Type ID counter: {d}", .{self.type_id_counter});
        std.log.info("Legacy mappings: {d}", .{self.legacy_id_mapping.count()});
        
        const stats = self.getCollisionStats();
        stats.debugPrint();
        
        std.log.info("=================================", .{});
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
        ref_count: atomic.Value(u32),
        mark: atomic.Value(bool), // for GC marking

        pub fn init(allocator: Allocator, type_id: u32, size: usize) !*TypedObject {
            const object = try allocator.create(TypedObject);
            object.* = TypedObject{
                .data = try allocator.alloc(u8, size),
                .type_id = type_id,
                .ref_count = atomic.Value(u32).init(1),
                .mark = atomic.Value(bool).init(false),
            };
            return object;
        }

        pub fn deinit(self: *TypedObject, allocator: Allocator) void {
            allocator.free(self.data);
            allocator.destroy(self);
        }

        pub fn retain(self: *TypedObject) CursedError!void {
            // Atomic increment with acquire-release ordering for thread safety
            const old_count = self.ref_count.fetchAdd(1, .acq_rel);
            
            // Validate reference count consistency
            if (old_count == 0) {
                std.log.err("Attempted to retain object with zero reference count", .{});
                return CursedError.MemoryCorruption;
            }
            if (old_count >= std.math.maxInt(u32) - 1) {
                std.log.err("Reference count overflow: {}", .{old_count});
                return CursedError.MemoryCorruption;
            }
        }

        pub fn release(self: *TypedObject, allocator: Allocator) CursedError!void {
            // Atomic decrement with acquire-release ordering
            const old_count = self.ref_count.fetchSub(1, .acq_rel);
            
            // Validate reference count consistency  
            if (old_count == 0) {
                std.log.err("Attempted to release object with zero reference count (double-free)", .{});
                return CursedError.MemoryCorruption;
            }
            
            // Only deallocate if this was the last reference
            if (old_count == 1) {
                self.deinit();
            }
        }

        // Helper method to get current reference count (for debugging)
        pub fn getRefCount(self: *TypedObject) u32 {
            return self.ref_count.load(.acquire);
        }
    };

    pub fn init(allocator: Allocator, gc_registry: *GCTypeRegistry) TypedAllocator {
        return TypedAllocator{
            .allocator = allocator,
            .gc_registry = gc_registry,
            .allocated_objects = .empty,
        };
    }

    pub fn deinit(self: *TypedAllocator) void {
        for (self.allocated_objects.items) |object| {
            object.deinit();
        }
        self.allocated_objects.deinit();
    }

    pub fn allocateStruct(self: *TypedAllocator, type_id: u32) !*TypedObject {
        const type_info = self.gc_registry.getType(type_id) orelse return error.UnknownType;
        const object = try TypedObject.init(self.allocator, type_id, type_info.size);
        try self.allocated_objects.append(self.allocator, object);
        return object;
    }

    pub fn getFieldPtr(self: *TypedAllocator, object: *TypedObject, field_name: []const u8) ![]u8 {
        const type_info = self.gc_registry.getType(object.type_id) orelse return error.UnknownType;
        const field_info = type_info.getField(field_name) orelse return error.UnknownField;
        
        if (field_info.offset + 8 > object.data.len) return error.InvalidOffset;
        return object.data[field_info.offset..field_info.offset + 8];
    }

    pub fn collectGarbage(self: *TypedAllocator) !void {
        // Mark phase - use atomic operations for thread safety
        for (self.allocated_objects.items) |object| {
            object.mark.store(false, .release);
        }

        // Simple sweep phase - remove unreferenced objects  
        var i: usize = 0;
        while (i < self.allocated_objects.items.len) {
            const object = self.allocated_objects.items[i];
            // Atomically check reference count
            if (object.ref_count.load(.acquire) == 0) {
                object.deinit();
                _ = self.allocated_objects.swapRemove(i);
            } else {
                i += 1;
            }
        }
    }
};

// Enhanced interface implementation tracking with collision resistance
pub const InterfaceRegistry = struct {
    /// Legacy implementation tracking
    implementations: HashMap(InterfaceImplKey, VTablePtr, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage),
    
    /// Collision-resistant interface registry
    collision_resistant_impls: type_collision.InterfaceImplRegistry,
    
    /// VTable storage with collision detection
    vtable_storage: HashMap(u64, VTableEntry, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),
    
    allocator: Allocator,

    pub const InterfaceImplKey = struct {
        struct_type_id: u32,
        interface_type_id: u32,
        
        /// Enhanced hash with collision resistance
        pub fn getEnhancedHash(self: InterfaceImplKey) u64 {
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&self.struct_type_id));
            hasher.update(std.mem.asBytes(&self.interface_type_id));
            return hasher.final();
        }
    };

    pub const InterfaceImplKeyContext = struct {
        pub fn hash(self: @This(), key: InterfaceImplKey) u64 {
            _ = self;
            // Use enhanced hash instead of simple bit shifting
            return key.getEnhancedHash();
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
    
    pub const VTableEntry = struct {
        vtable: VTablePtr,
        access_count: u32,
        registration_time: i64,
        
        pub fn init(vtable: VTablePtr) VTableEntry {
            return VTableEntry{
                .vtable = vtable,
                .access_count = 0,
                .registration_time = std.time.milliTimestamp(),
            };
        }
    };

    pub fn init() InterfaceRegistry {
        return InterfaceRegistry{
            .implementations = HashMap(InterfaceImplKey, VTablePtr, InterfaceImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .collision_resistant_impls = type_collision.InterfaceImplRegistry.init(allocator),
            .vtable_storage = HashMap(u64, VTableEntry, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *InterfaceRegistry) void {
        self.implementations.deinit();
        self.collision_resistant_impls.deinit();
        self.vtable_storage.deinit();
    }

    /// Enhanced registration with collision detection
    pub fn registerImplementation(self: *InterfaceRegistry, struct_type_id: u32, interface_type_id: u32, vtable: VTablePtr) !void {
        const key = InterfaceImplKey{
            .struct_type_id = struct_type_id,
            .interface_type_id = interface_type_id,
        };
        
        // Register in legacy system
        try self.implementations.put(key, vtable);
        
        // Register in collision-resistant system (requires type names)
        // For now, generate placeholder names based on IDs
        const struct_name = try std.fmt.allocPrint(self.allocator, "struct_{d}", .{struct_type_id});
        defer self.allocator.free(struct_name);
        
        const interface_name = try std.fmt.allocPrint(self.allocator, "interface_{d}", .{interface_type_id});
        defer self.allocator.free(interface_name);
        
        try self.collision_resistant_impls.registerImplementation(struct_name, interface_name);
        
        // Store VTable with collision-resistant key
        const vtable_hash = key.getEnhancedHash();
        const vtable_entry = VTableEntry.init(vtable);
        try self.vtable_storage.put(vtable_hash, vtable_entry);
    }
    
    /// Register implementation by name (preferred method)
    pub fn registerImplementationByName(self: *InterfaceRegistry, struct_name: []const u8, interface_name: []const u8, vtable: VTablePtr) !void {
        // Register in collision-resistant system
        try self.collision_resistant_impls.registerImplementation(struct_name, interface_name);
        
        // Generate hash for VTable storage
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(struct_name);
        hasher.update(interface_name);
        const vtable_hash = hasher.final();
        
        const vtable_entry = VTableEntry.init(vtable);
        try self.vtable_storage.put(vtable_hash, vtable_entry);
    }

    /// Enhanced VTable lookup with collision resistance
    pub fn getVTable(self: *InterfaceRegistry, struct_type_id: u32, interface_type_id: u32) ?VTablePtr {
        const key = InterfaceImplKey{
            .struct_type_id = struct_type_id,
            .interface_type_id = interface_type_id,
        };
        
        // Try legacy lookup first
        if (self.implementations.get(key)) |vtable| {
            return vtable;
        }
        
        // Try collision-resistant lookup
        const vtable_hash = key.getEnhancedHash();
        if (self.vtable_storage.getPtr(vtable_hash)) |entry| {
            entry.access_count += 1;
            return entry.vtable;
        }
        
        return null;
    }
    
    /// Get VTable by type names
    pub fn getVTableByName(self: *InterfaceRegistry, struct_name: []const u8, interface_name: []const u8) ?VTablePtr {
        // Check if implementation exists
        if (self.collision_resistant_impls.isImplemented(struct_name, interface_name) catch false) {
            // Generate hash for VTable lookup
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(struct_name);
            hasher.update(interface_name);
            const vtable_hash = hasher.final();
            
            if (self.vtable_storage.getPtr(vtable_hash)) |entry| {
                entry.access_count += 1;
                return entry.vtable;
            }
        }
        
        return null;
    }
    
    /// Check if interface is implemented with collision-resistant lookup
    pub fn isImplemented(self: *InterfaceRegistry, struct_name: []const u8, interface_name: []const u8) bool {
        return self.collision_resistant_impls.isImplemented(struct_name, interface_name) catch false;
    }
    
    /// Validate interface registry integrity
    pub fn validateIntegrity(self: *InterfaceRegistry) !bool {
        var validation_errors: u32 = 0;
        
        // Cross-validate legacy and collision-resistant systems
        var legacy_iter = self.implementations.iterator();
        while (legacy_iter.next()) |entry| {
            const key = entry.key_ptr.*;
            const vtable_hash = key.getEnhancedHash();
            
            if (self.vtable_storage.get(vtable_hash) == null) {
                std.log.err("VTable missing for struct_type_id={d}, interface_type_id={d}", 
                          .{key.struct_type_id, key.interface_type_id});
                validation_errors += 1;
            }
        }
        
        if (validation_errors > 0) {
            std.log.err("Interface registry validation failed with {d} errors", .{validation_errors});
            return false;
        }
        
        return true;
    }
    
    /// Debug print interface registry statistics
    pub fn debugPrintStats(self: *InterfaceRegistry) void {
        std.log.info("=== Interface Registry Statistics ===", .{});
        std.log.info("Legacy implementations: {d}", .{self.implementations.count()});
        std.log.info("VTable entries: {d}", .{self.vtable_storage.count()});
        
        // Calculate average access count
        var total_accesses: u64 = 0;
        var vtable_iter = self.vtable_storage.iterator();
        while (vtable_iter.next()) |entry| {
            total_accesses += entry.value_ptr.access_count;
        }
        
        const avg_accesses = if (self.vtable_storage.count() > 0) 
            @as(f64, @floatFromInt(total_accesses)) / @as(f64, @floatFromInt(self.vtable_storage.count()))
        else 
            0.0;
            
        std.log.info("Average VTable accesses: {d:.2}", .{avg_accesses});
        std.log.info("======================================", .{});
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

        // CURSED type compatibility rules (numeric type coercion)
        if (source_type.kind == .Basic and target_type.kind == .Basic) {
            return self.checkNumericCompatibility(source_type.type_name, target_type.type_name);
        }

        return false;
    }

    pub fn checkNumericCompatibility(self: *TypeChecker, source_name: []const u8, target_name: []const u8) bool {
        _ = self;
        
        // CURSED numeric type hierarchy:
        // drip (int) <-> normie (int) <-> thicc (big int)
        // smol (i8) <-> mid (i16) <-> normie (i32)
        // snack (f32) <-> meal (f64)
        
        if (std.mem.eql(u8, source_name, target_name)) return true;
        
        const numeric_groups = [_][]const []const u8{
            &[_][]const u8{ "drip", "normie", "thicc" },
            &[_][]const u8{ "smol", "mid", "normie" },
            &[_][]const u8{ "snack", "meal" },
        };
        
        for (numeric_groups) |group| {
            var source_in_group = false;
            var target_in_group = false;
            
            for (group) |type_name| {
                if (std.mem.eql(u8, source_name, type_name)) source_in_group = true;
                if (std.mem.eql(u8, target_name, type_name)) target_in_group = true;
            }
            
            if (source_in_group and target_in_group) return true;
        }
        
        return false;
    }

    pub fn validateFunctionCall(self: *TypeChecker, function_type_id: u32, arg_types: []const u32) !bool {
        const func_type = self.gc_registry.getType(function_type_id) orelse return error.UnknownType;
        
        if (func_type.kind != .Function) return error.NotAFunction;
        
        // Check parameter count (simplified - would need method info in real implementation)
        return arg_types.len > 0; // Basic validation
    }

    pub fn validateArrayAccess(self: *TypeChecker, array_type_id: u32, index_type_id: u32) !u32 {
        const array_type = self.gc_registry.getType(array_type_id) orelse return error.UnknownType;
        const index_type = self.gc_registry.getType(index_type_id) orelse return error.UnknownType;
        
        if (array_type.kind != .Array) return error.NotAnArray;
        
        // Check index is numeric
        const numeric_types = [_][]const u8{ "drip", "normie", "thicc", "smol", "mid" };
        var is_numeric = false;
        for (numeric_types) |numeric_type| {
            if (std.mem.eql(u8, index_type.type_name, numeric_type)) {
                is_numeric = true;
                break;
            }
        }
        
        if (!is_numeric) return error.NonNumericIndex;
        
        // Return element type (simplified - would need actual element type tracking)
        return 1; // Placeholder element type ID
    }

    pub fn validateStructFieldAccess(self: *TypeChecker, struct_type_id: u32, field_name: []const u8) !u32 {
        const struct_type = self.gc_registry.getType(struct_type_id) orelse return error.UnknownType;
        
        if (struct_type.kind != .Struct) return error.NotAStruct;
        
        const field = struct_type.getField(field_name) orelse return error.FieldNotFound;
        
        return field.field_type;
    }
};

// Runtime type checking integration
pub const RuntimeChecker = struct {
    type_checker: TypeChecker,
    allocator: Allocator,

    pub fn init(allocator: Allocator, gc_registry: *GCTypeRegistry, interface_registry: *InterfaceRegistry) RuntimeChecker {
        return RuntimeChecker{
            .type_checker = TypeChecker.init(gc_registry, interface_registry),
            .allocator = allocator,
        };
    }

    pub fn checkExpressionType(self: *RuntimeChecker, expression_kind: ExpressionKind, operand_types: []const u32) !u32 {
        switch (expression_kind) {
            .BinaryArithmetic => {
                if (operand_types.len != 2) return error.InvalidOperandCount;
                
                const left_type = operand_types[0];
                const right_type = operand_types[1];
                
                if (self.type_checker.areTypesCompatible(left_type, right_type)) {
                    return left_type; // Return left type for binary arithmetic
                } else {
                    return error.IncompatibleTypes;
                }
            },
            .BinaryComparison => {
                if (operand_types.len != 2) return error.InvalidOperandCount;
                
                const left_type = operand_types[0];
                const right_type = operand_types[1];
                
                if (self.type_checker.areTypesCompatible(left_type, right_type)) {
                    // Return boolean type ID (would need registry lookup)
                    return 999; // Placeholder for boolean type ID
                } else {
                    return error.IncompatibleTypes;
                }
            },
            .FunctionCall => {
                if (operand_types.len == 0) return error.InvalidOperandCount;
                
                const function_type = operand_types[0];
                const arg_types = operand_types[1..];
                
                if (try self.type_checker.validateFunctionCall(function_type, arg_types)) {
                    // Return function return type (simplified)
                    return 1; // Placeholder return type
                } else {
                    return error.InvalidFunctionCall;
                }
            },
            .ArrayAccess => {
                if (operand_types.len != 2) return error.InvalidOperandCount;
                
                const array_type = operand_types[0];
                const index_type = operand_types[1];
                
                return self.type_checker.validateArrayAccess(array_type, index_type);
            },
            .FieldAccess => {
                // Field access requires additional context (field name)
                // This would be handled with more context in a real implementation
                return error.NotImplemented;
            },
        }
    }

    pub const ExpressionKind = enum {
        BinaryArithmetic,
        BinaryComparison,
        FunctionCall,
        ArrayAccess,
        FieldAccess,
    };
};

// Type registration helpers for built-in CURSED types
pub fn registerBuiltinTypes(gc_registry: *GCTypeRegistry) !void {
    // Register primitive types
    _ = try gc_registry.registerType("lit", .Basic);      // boolean
    _ = try gc_registry.registerType("drip", .Basic);     // integer
    _ = try gc_registry.registerType("normie", .Basic);   // integer
    _ = try gc_registry.registerType("thicc", .Basic);    // big integer
    _ = try gc_registry.registerType("smol", .Basic);     // small integer
    _ = try gc_registry.registerType("mid", .Basic);      // medium integer
    _ = try gc_registry.registerType("tea", .Basic);      // string
    _ = try gc_registry.registerType("sip", .Basic);      // character
    _ = try gc_registry.registerType("snack", .Basic);    // float
    _ = try gc_registry.registerType("meal", .Basic);     // double
    _ = try gc_registry.registerType("cap", .Basic);      // void/unit
}

// Test helper functions
test "runtime type checking with collision handling" {
    const allocator = std.testing.allocator;
    var gc_registry = GCTypeRegistry.init(allocator);
    defer gc_registry.deinit();
    
    var interface_registry = InterfaceRegistry.init(allocator);
    defer interface_registry.deinit();
    
    // Register built-in types
    try registerBuiltinTypes(&gc_registry);
    
    var checker = TypeChecker.init(&gc_registry, &interface_registry);
    
    // Test numeric compatibility
    const drip_type = gc_registry.findTypeByName("drip").?;
    const normie_type = gc_registry.findTypeByName("normie").?;
    
    try std.testing.expect(checker.areTypesCompatible(drip_type.type_id, normie_type.type_id));
    
    // Test collision handling by registering many types
    var i: u32 = 0;
    while (i < 50) : (i += 1) {
        const type_name = try std.fmt.allocPrint(allocator, "TestType{d}", .{i});
        defer allocator.free(type_name);
        
        _ = try gc_registry.registerType(type_name, .Basic);
    }
    
    // Validate integrity after bulk registration
    try std.testing.expect(try gc_registry.validateIntegrity());
    
    // Print collision statistics
    gc_registry.debugPrintStats();
    
    std.log.info("Type collision handling test completed successfully", .{});
}

test "runtime expression checking" {
    const allocator = std.testing.allocator;
    var gc_registry = GCTypeRegistry.init(allocator);
    defer gc_registry.deinit();
    
    var interface_registry = InterfaceRegistry.init(allocator);
    defer interface_registry.deinit();
    
    try registerBuiltinTypes(&gc_registry);
    
    var runtime_checker = RuntimeChecker.init(allocator, &gc_registry, &interface_registry);
    
    const drip_type_id = gc_registry.findTypeByName("drip").?.type_id;
    const operand_types = [_]u32{ drip_type_id, drip_type_id };
    
    const result_type = try runtime_checker.checkExpressionType(.BinaryArithmetic, &operand_types);
    try std.testing.expect(result_type == drip_type_id);
}
