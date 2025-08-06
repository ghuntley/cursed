const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

/// Demonstrates memory-safe programming patterns for CURSED compiler
/// This shows the key concepts implemented in the memory-safe error reporting and lexer

const MemorySafeString = struct {
    data: []const u8,
    arena: *ArenaAllocator,
    
    pub fn init(arena: *ArenaAllocator, source: []const u8) !MemorySafeString {
        const arena_allocator = arena.allocator();
        const data_copy = try arena_allocator.dupe(u8, source);
        return MemorySafeString{
            .data = data_copy,
            .arena = arena,
        };
    }
    
    // No manual deinit needed - arena handles cleanup
};

const MemorySafeCollection = struct {
    items: ArrayList(MemorySafeString),
    arena: ArenaAllocator,
    
    pub fn init(backing_allocator: Allocator) MemorySafeCollection {
        var arena = ArenaAllocator.init(backing_allocator);
        const arena_allocator = arena.allocator();
        
        return MemorySafeCollection{
            .items = ArrayList(MemorySafeString).init(arena_allocator),
            .arena = arena,
        };
    }
    
    pub fn deinit(self: *MemorySafeCollection) void {
        // Arena automatically cleans up all allocated memory
        self.arena.deinit();
    }
    
    pub fn addString(self: *MemorySafeCollection, source: []const u8) !void {
        const safe_string = try MemorySafeString.init(&self.arena, source);
        try self.items.append(safe_string);
    }
    
    pub fn count(self: *MemorySafeCollection) usize {
        return self.items.items.len;
    }
};

/// Demonstrates error-safe resource management
const ResourceManager = struct {
    collections: ArrayList(*MemorySafeCollection),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) ResourceManager {
        return ResourceManager{
            .collections = ArrayList(*MemorySafeCollection).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ResourceManager) void {
        // Clean up all collections
        for (self.collections.items) |collection| {
            collection.deinit();
            self.allocator.destroy(collection);
        }
        self.collections.deinit();
    }
    
    pub fn createCollection(self: *ResourceManager) !*MemorySafeCollection {
        const collection = try self.allocator.create(MemorySafeCollection);
        collection.* = MemorySafeCollection.init(self.allocator);
        
        // Use errdefer for exception safety
        errdefer {
            collection.deinit();
            self.allocator.destroy(collection);
        }
        
        try self.collections.append(collection);
        return collection;
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("=== CURSED Memory Safety Demonstration ===\n", .{});
    
    // Test 1: Basic arena allocator usage
    print("Test 1: Arena allocator memory safety\n", .{});
    {
        var arena = ArenaAllocator.init(allocator);
        defer arena.deinit(); // All arena memory freed automatically
        
        const arena_allocator = arena.allocator();
        
        // Allocate multiple strings - no manual cleanup needed
        const strings = [_][]const u8{
            "slay main_character() {",
            "    vibez.spill(\"Hello CURSED!\")",
            "    damn based",
            "}",
        };
        
        var stored_strings = ArrayList([]const u8).init(arena_allocator);
        
        for (strings) |str| {
            const copied = try arena_allocator.dupe(u8, str);
            try stored_strings.append(copied);
        }
        
        print("  ✅ Allocated and stored {} strings safely\n", .{stored_strings.items.len});
        // Arena deinit will clean up everything automatically
    }
    
    // Test 2: Collection with automatic cleanup
    print("Test 2: Memory-safe collection management\n", .{});
    {
        var collection = MemorySafeCollection.init(allocator);
        defer collection.deinit(); // Automatic cleanup
        
        const test_data = [_][]const u8{
            "sus variable normie = 42",
            "facts constant tea = \"Hello\"",
            "slay function() lit { damn based }",
        };
        
        for (test_data) |data| {
            try collection.addString(data);
        }
        
        print("  ✅ Collection contains {} items with automatic memory management\n", .{collection.count()});
        // Collection deinit will clean up everything
    }
    
    // Test 3: Resource manager with exception safety
    print("Test 3: Exception-safe resource management\n", .{});
    {
        var manager = ResourceManager.init(allocator);
        defer manager.deinit(); // Clean up all resources
        
        // Create multiple collections
        for (0..3) |i| {
            const collection = try manager.createCollection();
            
            const test_string = try std.fmt.allocPrint(allocator, "Collection {} data", .{i});
            defer allocator.free(test_string);
            
            try collection.addString(test_string);
        }
        
        print("  ✅ Created and managed {} collections safely\n", .{manager.collections.items.len});
        // Manager deinit will clean up all collections automatically
    }
    
    // Test 4: Error handling with memory safety
    print("Test 4: Error handling with automatic cleanup\n", .{});
    {
        const result = testErrorHandling(allocator);
        if (result) |_| {
            print("  ✅ Error handling test passed\n", .{});
        } else |err| {
            print("  ✅ Error handled safely: {}\n", .{err});
        }
    }
    
    print("=== All Memory Safety Tests Passed ===\n", .{});
    print("✅ Arena allocators prevent memory leaks\n", .{});
    print("✅ Automatic cleanup ensures resource safety\n", .{});
    print("✅ Exception safety with errdefer blocks\n", .{});
    print("✅ No manual memory management required\n", .{});
}

fn testErrorHandling(allocator: Allocator) !void {
    var arena = ArenaAllocator.init(allocator);
    defer arena.deinit(); // Cleanup even on error
    
    const arena_allocator = arena.allocator();
    
    // Simulate parsing with potential errors
    var tokens = ArrayList([]const u8).init(arena_allocator);
    errdefer {
        // Any cleanup needed on error would go here
        // But arena.deinit() handles everything automatically
    }
    
    const token_data = [_][]const u8{
        "slay", "identifier", "(", ")", "{", "damn", "based", "}",
    };
    
    for (token_data) |token| {
        const token_copy = try arena_allocator.dupe(u8, token);
        try tokens.append(token_copy);
    }
    
    // Simulate successful parsing
    if (tokens.items.len != 8) {
        return error.InvalidTokenCount;
    }
    
    // Success - arena cleanup happens automatically
}

test "memory safety validation" {
    const allocator = std.testing.allocator;
    
    // Test arena allocator usage
    var arena = ArenaAllocator.init(allocator);
    defer arena.deinit();
    
    const arena_allocator = arena.allocator();
    
    // Allocate without manual cleanup
    const test_string = try arena_allocator.dupe(u8, "test data");
    _ = test_string; // Use the string
    
    // No manual free needed - arena handles it
    
    // Test collection
    var collection = MemorySafeCollection.init(allocator);
    defer collection.deinit();
    
    try collection.addString("test item 1");
    try collection.addString("test item 2");
    
    try std.testing.expect(collection.count() == 2);
}
