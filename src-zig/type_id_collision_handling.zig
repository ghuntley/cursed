//! Enhanced Type ID Collision Handling System
//! Implements robust hash table collision resolution for runtime type safety
//! Addresses critical type identity management and duplicate detection issues

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const atomic = std.atomic;

const type_system = @import("type_system_runtime.zig");
const RuntimeTypeInfo = type_system.RuntimeTypeInfo;
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

/// Enhanced type ID with collision detection
pub const TypeId = struct {
    primary_hash: u64,      // Primary hash for quick lookup
    secondary_hash: u64,    // Secondary hash for collision detection
    canonical_name: []const u8,  // Full type name for verification
    type_fingerprint: TypeFingerprint,  // Deep structural identity
    generation: u32,        // Generation counter for uniqueness
    
    pub const TypeFingerprint = struct {
        structure_hash: u64,   // Hash of type structure
        field_count: u32,      // Number of fields/parameters
        method_count: u32,     // Number of methods
        size_bytes: usize,     // Type size in bytes
        alignment: usize,      // Type alignment
        flags: TypeFlags,      // Type characteristics
        
        pub const TypeFlags = packed struct {
            is_generic: bool = false,
            is_interface: bool = false,
            is_concrete: bool = true,
            is_send: bool = false,
            is_sync: bool = false,
            is_copy: bool = false,
            has_drop: bool = false,
            is_zero_sized: bool = false,
            _padding: u8 = 0,
        };
        
        pub fn init() TypeFingerprint {
            return TypeFingerprint{
                .structure_hash = 0,
                .field_count = 0,
                .method_count = 0,
                .size_bytes = 0,
                .alignment = 1,
                .flags = TypeFlags{},
            };
        }
        
        /// Deep comparison of type fingerprints
        pub fn equals(self: TypeFingerprint, other: TypeFingerprint) bool {
            return self.structure_hash == other.structure_hash and
                   self.field_count == other.field_count and
                   self.method_count == other.method_count and
                   self.size_bytes == other.size_bytes and
                   self.alignment == other.alignment and
                   std.meta.eql(self.flags, other.flags);
        }
    };
    
    pub fn init(allocator: Allocator, type_name: []const u8, fingerprint: TypeFingerprint) !TypeId {
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(type_name);
        hasher.update(std.mem.asBytes(&fingerprint.structure_hash));
        const primary = hasher.final();
        
        // Secondary hash using different algorithm
        var hasher2 = std.hash.Fnv1a_64.init();
        hasher2.update(type_name);
        hasher2.update(std.mem.asBytes(&fingerprint.field_count));
        hasher2.update(std.mem.asBytes(&fingerprint.method_count));
        const secondary = hasher2.final();
        
        return TypeId{
            .primary_hash = primary,
            .secondary_hash = secondary,
            .canonical_name = try allocator.dupe(u8, type_name),
            .type_fingerprint = fingerprint,
            .generation = 0,
        };
    }
    
    pub fn deinit(self: *TypeId, allocator: Allocator) void {
        allocator.free(self.canonical_name);
    }
    
    /// Check if two TypeIds represent the same type
    pub fn equals(self: TypeId, other: TypeId) bool {
        // Fast path: check hashes first
        if (self.primary_hash != other.primary_hash) return false;
        if (self.secondary_hash != other.secondary_hash) return false;
        
        // Verify names match
        if (!std.mem.eql(u8, self.canonical_name, other.canonical_name)) return false;
        
        // Deep comparison of fingerprints
        return self.type_fingerprint.equals(other.type_fingerprint);
    }
    
    /// Generate a unique composite hash
    pub fn getCompositeHash(self: TypeId) u64 {
        var hasher = std.hash.Wyhash.init(self.generation);
        hasher.update(std.mem.asBytes(&self.primary_hash));
        hasher.update(std.mem.asBytes(&self.secondary_hash));
        hasher.update(self.canonical_name);
        return hasher.final();
    }
};

/// Enhanced collision-resistant type registry
pub const CollisionResistantTypeRegistry = struct {
    /// Main hash table with collision detection
    primary_table: HashMap(u64, TypeEntry, HashContext, std.hash_map.default_max_load_percentage),
    
    /// Secondary table for collision resolution
    overflow_table: HashMap(u64, ArrayList(TypeEntry), HashContext, std.hash_map.default_max_load_percentage),
    
    /// Name-based lookup for verification
    name_index: HashMap([]const u8, TypeId, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    /// Fingerprint-based lookup for structural identity
    fingerprint_index: HashMap(u64, ArrayList(TypeId), HashContext, std.hash_map.default_max_load_percentage),
    
    /// Statistics and monitoring
    collision_stats: CollisionStats,
    allocator: Allocator,
    generation_counter: atomic.Value(u32),
    
    pub const TypeEntry = struct {
        type_id: TypeId,
        type_info: RuntimeTypeInfo,
        registration_time: i64,  // Timestamp for debugging
        access_count: u32,       // Usage statistics
        
        pub fn init(type_id: TypeId, type_info: RuntimeTypeInfo) TypeEntry {
            return TypeEntry{
                .type_id = type_id,
                .type_info = type_info,
                .registration_time = std.time.milliTimestamp(),
                .access_count = 0,
            };
        }
    };
    
    pub const CollisionStats = struct {
        total_insertions: u32 = 0,
        primary_collisions: u32 = 0,
        secondary_collisions: u32 = 0,
        overflow_entries: u32 = 0,
        duplicate_detections: u32 = 0,
        false_positives: u32 = 0,
        
        pub fn reportCollisionRate(self: CollisionStats) f64 {
            if (self.total_insertions == 0) return 0.0;
            return @as(f64, @floatFromInt(self.primary_collisions)) / 
                   @as(f64, @floatFromInt(self.total_insertions));
        }
        
        pub fn debugPrint(self: CollisionStats) void {
            std.log.info("Type Registry Collision Statistics:", .{});
            std.log.info("  Total insertions: {d}", .{self.total_insertions});
            std.log.info("  Primary collisions: {d} ({d:.2}%)", .{
                self.primary_collisions, 
                self.reportCollisionRate() * 100.0
            });
            std.log.info("  Secondary collisions: {d}", .{self.secondary_collisions});
            std.log.info("  Overflow entries: {d}", .{self.overflow_entries});
            std.log.info("  Duplicate detections: {d}", .{self.duplicate_detections});
            std.log.info("  False positives: {d}", .{self.false_positives});
        }
    };
    
    pub const HashContext = struct {
        pub fn hash(self: @This(), key: u64) u64 {
            _ = self;
            return key;
        }
        
        pub fn eql(self: @This(), a: u64, b: u64) bool {
            _ = self;
            return a == b;
        }
    };
    
    pub fn init() CollisionResistantTypeRegistry {
        return CollisionResistantTypeRegistry{
            .primary_table = HashMap(u64, TypeEntry, HashContext, std.hash_map.default_max_load_percentage).init(allocator),
            .overflow_table = HashMap(u64, ArrayList(TypeEntry), HashContext, std.hash_map.default_max_load_percentage).init(allocator),
            .name_index = HashMap([]const u8, TypeId, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .fingerprint_index = HashMap(u64, ArrayList(TypeId), HashContext, std.hash_map.default_max_load_percentage).init(allocator),
            .collision_stats = CollisionStats{},
            .allocator = allocator,
            .generation_counter = atomic.Value(u32).init(1),
        };
    }
    
    pub fn deinit(self: *CollisionResistantTypeRegistry) void {
        // Clean up primary table
        var primary_iter = self.primary_table.iterator();
        while (primary_iter.next()) |entry| {
            entry.value_ptr.type_id.deinit();
            entry.value_ptr.type_info.deinit();
        }
        self.primary_table.deinit();
        
        // Clean up overflow table
        var overflow_iter = self.overflow_table.iterator();
        while (overflow_iter.next()) |entry| {
            for (entry.value_ptr.items) |*type_entry| {
                type_entry.type_id.deinit();
                type_entry.type_info.deinit();
            }
            entry.value_ptr.deinit();
        }
        self.overflow_table.deinit();
        
        // Clean up name index
        var name_iter = self.name_index.iterator();
        while (name_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.name_index.deinit();
        
        // Clean up fingerprint index
        var fp_iter = self.fingerprint_index.iterator();
        while (fp_iter.next()) |entry| {
            for (entry.value_ptr.items) |*type_id| {
                type_id.deinit();
            }
            entry.value_ptr.deinit();
        }
        self.fingerprint_index.deinit();
    }
    
    /// Register a new type with collision detection and prevention
    pub fn registerType(self: *CollisionResistantTypeRegistry, type_name: []const u8, type_info: RuntimeTypeInfo) !TypeId {
        self.collision_stats.total_insertions += 1;
        
        // First check if type already exists by name
        if (self.name_index.get(type_name)) |existing_type_id| {
            self.collision_stats.duplicate_detections += 1;
            std.log.warn("Type '{s}' already registered, returning existing TypeId", .{type_name});
            return existing_type_id;
        }
        
        // Generate fingerprint for this type
        const fingerprint = try self.generateTypeFingerprint(type_info);
        
        // Check for structural duplicates
        if (try self.findStructuralDuplicate(fingerprint, type_name)) |duplicate_id| {
            self.collision_stats.duplicate_detections += 1;
            std.log.warn("Structural duplicate detected for type '{s}'", .{type_name});
            return duplicate_id;
        }
        
        // Create new TypeId with unique generation
        var type_id = try TypeId.init(self.allocator, type_name, fingerprint);
        type_id.generation = self.generation_counter.fetchAdd(1, .acq_rel);
        
        const composite_hash = type_id.getCompositeHash();
        const type_entry = TypeEntry.init(type_id, type_info);
        
        // Try to insert into primary table
        const insert_result = try self.insertWithCollisionHandling(composite_hash, type_entry);
        
        // Update indices for fast lookup
        try self.updateIndices(type_id, fingerprint);
        
        return insert_result;
    }
    
    /// Insert with comprehensive collision handling
    fn insertWithCollisionHandling(self: *CollisionResistantTypeRegistry, hash: u64, entry: TypeEntry) !TypeId {
        // Try primary table first
        if (self.primary_table.getPtr(hash)) |existing_entry| {
            self.collision_stats.primary_collisions += 1;
            
            // Check if it's actually the same type (hash collision vs duplicate)
            if (existing_entry.type_id.equals(entry.type_id)) {
                return existing_entry.type_id;
            }
            
            // Handle collision by moving to overflow table
            return self.handleCollision(hash, entry);
        }
        
        // Primary slot is free
        try self.primary_table.put(hash, entry);
        return entry.type_id;
    }
    
    /// Handle hash collisions using overflow table
    fn handleCollision(self: *CollisionResistantTypeRegistry, hash: u64, entry: TypeEntry) !TypeId {
        // Get or create overflow list for this hash
        if (self.overflow_table.getPtr(hash)) |overflow_list| {
            // Check for duplicates in overflow list
            for (overflow_list.items) |existing_entry| {
                if (existing_entry.type_id.equals(entry.type_id)) {
                    return existing_entry.type_id;
                }
            }
            
            // Add to existing overflow list
            try overflow_list.append(entry);
        } else {
            // Create new overflow list
            var new_list = .empty;
            try new_list.append(entry);
            try self.overflow_table.put(hash, new_list);
        }
        
        self.collision_stats.overflow_entries += 1;
        return entry.type_id;
    }
    
    /// Generate comprehensive type fingerprint
    fn generateTypeFingerprint(self: *CollisionResistantTypeRegistry, type_info: RuntimeTypeInfo) !TypeId.TypeFingerprint {
        _ = self;
        
        var fingerprint = TypeId.TypeFingerprint.init();
        fingerprint.size_bytes = type_info.size;
        fingerprint.alignment = type_info.alignment;
        
        // Set flags based on type kind and properties
        fingerprint.flags.is_interface = (type_info.kind == .Interface);
        fingerprint.flags.is_generic = (type_info.kind == .Function and type_info.methods != null);
        fingerprint.flags.is_zero_sized = (type_info.size == 0);
        
        // Hash type structure
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(std.mem.asBytes(&type_info.kind));
        hasher.update(std.mem.asBytes(&type_info.size));
        hasher.update(std.mem.asBytes(&type_info.alignment));
        
        // Include field information in hash
        if (type_info.fields) |fields| {
            fingerprint.field_count = @intCast(fields.len);
            for (fields) |field| {
                hasher.update(field.name);
                hasher.update(std.mem.asBytes(&field.field_type));
                hasher.update(std.mem.asBytes(&field.offset));
            }
        }
        
        // Include method information in hash
        if (type_info.methods) |methods| {
            fingerprint.method_count = @intCast(methods.len);
            for (methods) |method| {
                hasher.update(method.name);
                hasher.update(std.mem.asBytes(&method.return_type));
                for (method.parameter_types) |param_type| {
                    hasher.update(std.mem.asBytes(&param_type));
                }
            }
        }
        
        fingerprint.structure_hash = hasher.final();
        return fingerprint;
    }
    
    /// Find structurally identical types
    fn findStructuralDuplicate(self: *CollisionResistantTypeRegistry, fingerprint: TypeId.TypeFingerprint, type_name: []const u8) !?TypeId {
        const fp_hash = fingerprint.structure_hash;
        
        if (self.fingerprint_index.get(fp_hash)) |type_list| {
            for (type_list.items) |type_id| {
                // Same fingerprint but different name might be aliasing
                if (type_id.type_fingerprint.equals(fingerprint)) {
                    if (!std.mem.eql(u8, type_id.canonical_name, type_name)) {
                        self.collision_stats.false_positives += 1;
                        std.log.info("Structural match but different name: '{s}' vs '{s}'", 
                                   .{type_name, type_id.canonical_name});
                    }
                    return type_id;
                }
            }
        }
        
        return null;
    }
    
    /// Update all lookup indices
    fn updateIndices(self: *CollisionResistantTypeRegistry, type_id: TypeId, fingerprint: TypeId.TypeFingerprint) !void {
        // Update name index
        const owned_name = try self.allocator.dupe(u8, type_id.canonical_name);
        try self.name_index.put(owned_name, type_id);
        
        // Update fingerprint index
        const fp_hash = fingerprint.structure_hash;
        if (self.fingerprint_index.getPtr(fp_hash)) |type_list| {
            try type_list.append(self.allocator, type_id);
        } else {
            var new_list = .empty;
            try new_list.append(self.allocator, type_id);
            try self.fingerprint_index.put(fp_hash, new_list);
        }
    }
    
    /// Lookup type by name with collision-aware search
    pub fn getTypeByName(self: *CollisionResistantTypeRegistry, type_name: []const u8) ?*RuntimeTypeInfo {
        // First try name index
        const type_id = self.name_index.get(type_name) orelse return null;
        
        // Find in primary or overflow tables
        return self.findTypeInfo(type_id);
    }
    
    /// Lookup type by TypeId with collision handling
    pub fn getTypeById(self: *CollisionResistantTypeRegistry, target_id: TypeId) ?*RuntimeTypeInfo {
        return self.findTypeInfo(target_id);
    }
    
    /// Find type info in collision-resistant tables
    fn findTypeInfo(self: *CollisionResistantTypeRegistry, target_id: TypeId) ?*RuntimeTypeInfo {
        const hash = target_id.getCompositeHash();
        
        // Check primary table
        if (self.primary_table.getPtr(hash)) |entry| {
            entry.access_count += 1;
            if (entry.type_id.equals(target_id)) {
                return &entry.type_info;
            }
        }
        
        // Check overflow table
        if (self.overflow_table.get(hash)) |overflow_list| {
            for (overflow_list.items) |*entry| {
                entry.access_count += 1;
                if (entry.type_id.equals(target_id)) {
                    return &entry.type_info;
                }
            }
        }
        
        return null;
    }
    
    /// Validate type registry integrity
    pub fn validateIntegrity(self: *CollisionResistantTypeRegistry) !bool {
        var validation_errors: u32 = 0;
        
        // Check primary table consistency
        var primary_iter = self.primary_table.iterator();
        while (primary_iter.next()) |entry| {
            const expected_hash = entry.value_ptr.type_id.getCompositeHash();
            if (entry.key_ptr.* != expected_hash) {
                std.log.err("Primary table hash mismatch for type '{s}'", .{entry.value_ptr.type_id.canonical_name});
                validation_errors += 1;
            }
        }
        
        // Check overflow table consistency
        var overflow_iter = self.overflow_table.iterator();
        while (overflow_iter.next()) |entry| {
            for (entry.value_ptr.items) |overflow_entry| {
                const expected_hash = overflow_entry.type_id.getCompositeHash();
                if (entry.key_ptr.* != expected_hash) {
                    std.log.err("Overflow table hash mismatch for type '{s}'", .{overflow_entry.type_id.canonical_name});
                    validation_errors += 1;
                }
            }
        }
        
        // Verify name index consistency
        var name_iter = self.name_index.iterator();
        while (name_iter.next()) |entry| {
            if (!std.mem.eql(u8, entry.key_ptr.*, entry.value_ptr.canonical_name)) {
                std.log.err("Name index inconsistency: key='{s}', value='{s}'", 
                          .{entry.key_ptr.*, entry.value_ptr.canonical_name});
                validation_errors += 1;
            }
        }
        
        if (validation_errors > 0) {
            std.log.err("Type registry validation failed with {d} errors", .{validation_errors});
            return false;
        }
        
        std.log.info("Type registry validation passed", .{});
        return true;
    }
    
    /// Export collision statistics
    pub fn getCollisionStats(self: *CollisionResistantTypeRegistry) CollisionStats {
        return self.collision_stats;
    }
    
    /// Rehash table when collision rate becomes too high
    pub fn rehashIfNeeded(self: *CollisionResistantTypeRegistry) !void {
        const collision_rate = self.collision_stats.reportCollisionRate();
        
        if (collision_rate > 0.75) { // Rehash if collision rate > 75%
            std.log.info("High collision rate ({d:.2}%), triggering rehash", .{collision_rate * 100.0});
            try self.rehashTables();
        }
    }
    
    /// Rehash all tables with new hash functions
    fn rehashTables(self: *CollisionResistantTypeRegistry) !void {
        _ = self; // Mark parameter as used
        // This would implement table rehashing with updated hash functions
        // For now, just log the intention
        std.log.info("Table rehashing not yet implemented", .{});
    }
};

/// Enhanced interface implementation tracking with collision resistance
pub const InterfaceImplRegistry = struct {
    impl_table: HashMap(ImplKey, bool, ImplKeyContext, std.hash_map.default_max_load_percentage),
    collision_table: HashMap(u64, ArrayList(ImplEntry), std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub const ImplKey = struct {
        type_hash: u64,
        interface_hash: u64,
        type_name: []const u8,
        interface_name: []const u8,
        
        pub fn init(allocator: Allocator, type_name: []const u8, interface_name: []const u8) !ImplKey {
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(type_name);
            const type_hash = hasher.final();
            
            hasher = std.hash.Wyhash.init(1);
            hasher.update(interface_name);
            const interface_hash = hasher.final();
            
            return ImplKey{
                .type_hash = type_hash,
                .interface_hash = interface_hash,
                .type_name = try allocator.dupe(u8, type_name),
                .interface_name = try allocator.dupe(u8, interface_name),
            };
        }
        
        pub fn deinit(self: *ImplKey, allocator: Allocator) void {
            allocator.free(self.type_name);
            allocator.free(self.interface_name);
        }
        
        pub fn getCompositeHash(self: ImplKey) u64 {
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&self.type_hash));
            hasher.update(std.mem.asBytes(&self.interface_hash));
            return hasher.final();
        }
        
        pub fn equals(self: ImplKey, other: ImplKey) bool {
            return self.type_hash == other.type_hash and
                   self.interface_hash == other.interface_hash and
                   std.mem.eql(u8, self.type_name, other.type_name) and
                   std.mem.eql(u8, self.interface_name, other.interface_name);
        }
    };
    
    pub const ImplEntry = struct {
        key: ImplKey,
        is_implemented: bool,
        verification_time: i64,
    };
    
    pub const ImplKeyContext = struct {
        pub fn hash(self: @This(), key: ImplKey) u64 {
            _ = self;
            return key.getCompositeHash();
        }
        
        pub fn eql(self: @This(), a: ImplKey, b: ImplKey) bool {
            _ = self;
            return a.equals(b);
        }
    };
    
    pub fn init() InterfaceImplRegistry {
        return InterfaceImplRegistry{
            .impl_table = HashMap(ImplKey, bool, ImplKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .collision_table = HashMap(u64, ArrayList(ImplEntry), std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *InterfaceImplRegistry) void {
        var iter = self.impl_table.iterator();
        while (iter.next()) |entry| {
            entry.key_ptr.deinit();
        }
        self.impl_table.deinit();
        
        var collision_iter = self.collision_table.iterator();
        while (collision_iter.next()) |entry| {
            for (entry.value_ptr.items) |*impl_entry| {
                impl_entry.key.deinit();
            }
            entry.value_ptr.deinit();
        }
        self.collision_table.deinit();
    }
    
    /// Register interface implementation with collision detection
    pub fn registerImplementation(self: *InterfaceImplRegistry, type_name: []const u8, interface_name: []const u8) !void {
        const key = try ImplKey.init(self.allocator, type_name, interface_name);
        
        // Try to insert, handle collisions
        const result = try self.impl_table.getOrPut(key);
        if (result.found_existing) {
            // Handle collision - move to collision table if different
            if (!result.key_ptr.equals(key)) {
                try self.handleImplCollision(key, true);
            } else {
                key.deinit();
            }
        } else {
            result.value_ptr.* = true;
        }
    }
    
    /// Handle interface implementation collisions
    fn handleImplCollision(self: *InterfaceImplRegistry, key: ImplKey, is_implemented: bool) !void {
        const hash = key.getCompositeHash();
        
        const entry = ImplEntry{
            .key = key,
            .is_implemented = is_implemented,
            .verification_time = std.time.milliTimestamp(),
        };
        
        if (self.collision_table.getPtr(hash)) |collision_list| {
            try collision_list.append(entry);
        } else {
            var new_list = .empty;
            try new_list.append(self.allocator, entry);
            try self.collision_table.put(hash, new_list);
        }
    }
    
    /// Check if type implements interface with collision-aware lookup
    pub fn isImplemented(self: *InterfaceImplRegistry, type_name: []const u8, interface_name: []const u8) !bool {
        const temp_key = try ImplKey.init(self.allocator, type_name, interface_name);
        defer temp_key.deinit();
        
        // Check main table
        if (self.impl_table.get(temp_key)) |result| {
            return result;
        }
        
        // Check collision table
        const hash = temp_key.getCompositeHash();
        if (self.collision_table.get(hash)) |collision_list| {
            for (collision_list.items) |entry| {
                if (entry.key.equals(temp_key)) {
                    return entry.is_implemented;
                }
            }
        }
        
        return false;
    }
};

test "type ID collision handling" {
    const allocator = std.testing.allocator;
    
    var registry = CollisionResistantTypeRegistry.init(allocator);
    defer registry.deinit();
    
    // Create test types with potential collision
    const type_info1 = try RuntimeTypeInfo.init(allocator, 1, "TestType1", .Basic);
    const type_info2 = try RuntimeTypeInfo.init(allocator, 2, "TestType2", .Basic);
    
    const type_id1 = try registry.registerType("TestType1", type_info1);
    const type_id2 = try registry.registerType("TestType2", type_info2);
    
    // Verify types are distinct
    try std.testing.expect(!type_id1.equals(type_id2));
    
    // Test lookups
    const found1 = registry.getTypeByName("TestType1");
    const found2 = registry.getTypeByName("TestType2");
    
    try std.testing.expect(found1 != null);
    try std.testing.expect(found2 != null);
    try std.testing.expect(found1.?.type_id == 1);
    try std.testing.expect(found2.?.type_id == 2);
    
    // Validate registry integrity
    try std.testing.expect(try registry.validateIntegrity());
    
    std.log.info("Type ID collision handling test passed", .{});
}

test "interface implementation collision handling" {
    const allocator = std.testing.allocator;
    
    var registry = InterfaceImplRegistry.init(allocator);
    defer registry.deinit();
    
    // Register some implementations
    try registry.registerImplementation("MyStruct", "Displayable");
    try registry.registerImplementation("MyStruct", "Comparable");
    try registry.registerImplementation("OtherStruct", "Displayable");
    
    // Test lookups
    try std.testing.expect(try registry.isImplemented("MyStruct", "Displayable"));
    try std.testing.expect(try registry.isImplemented("MyStruct", "Comparable"));
    try std.testing.expect(try registry.isImplemented("OtherStruct", "Displayable"));
    try std.testing.expect(!try registry.isImplemented("OtherStruct", "Comparable"));
    
    std.log.info("Interface implementation collision handling test passed", .{});
}
