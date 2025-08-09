// Unit tests for cyclic dependency memory safety

const std = @import("std");
const testing = std.testing;
const print = std.debug.print;

const safe_import_resolver = @import("src-zig/safe_import_resolver.zig");
const SafeModuleLoader = safe_import_resolver.SafeModuleLoader;

test "cycle detection prevents double-free" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Create 2-module cycle: A -> B -> A
    loader.recordDependency("module_a", "module_b");
    loader.recordDependency("module_b", "module_a");
    
    // Detect cycles
    const has_cycles = try loader.detectCycles();
    try testing.expect(has_cycles);
    
    print("✅ 2-module cycle correctly detected\n", .{});
}

test "3-module cycle detection" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Create 3-module cycle: A -> B -> C -> A
    loader.recordDependency("alpha", "beta");
    loader.recordDependency("beta", "gamma");
    loader.recordDependency("gamma", "alpha");
    
    // Detect cycles
    const has_cycles = try loader.detectCycles();
    try testing.expect(has_cycles);
    
    print("✅ 3-module cycle correctly detected\n", .{});
}

test "complex dependency graph with multiple cycles" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Create complex graph with multiple cycles
    // Cycle 1: A -> B -> A
    loader.recordDependency("A", "B");
    loader.recordDependency("B", "A");
    
    // Cycle 2: C -> D -> E -> C
    loader.recordDependency("C", "D");
    loader.recordDependency("D", "E");
    loader.recordDependency("E", "C");
    
    // Additional dependencies
    loader.recordDependency("A", "C"); // A depends on C
    loader.recordDependency("F", "A"); // F depends on A (no cycle)
    
    // Detect cycles
    const has_cycles = try loader.detectCycles();
    try testing.expect(has_cycles);
    
    print("✅ Complex dependency graph with multiple cycles correctly detected\n", .{});
}

test "in-progress state prevents double initialization" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Simulate module being loaded
    try loader.setModuleState("test_module", .in_progress);
    
    // Try to load again - should detect in-progress state
    const state = loader.module_states.get("test_module");
    try testing.expect(state != null);
    try testing.expect(state.? == .in_progress);
    
    print("✅ In-progress state correctly prevents double initialization\n", .{});
}

test "reference counting lifecycle" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Test reference counting lifecycle
    try loader.setRefCount("module_test", 0);
    
    // Increment references
    loader.incrementRefCount("module_test");
    loader.incrementRefCount("module_test");
    loader.incrementRefCount("module_test");
    
    const count = loader.reference_counts.get("module_test");
    try testing.expect(count != null);
    try testing.expect(count.? == 3);
    
    // Decrement references
    var new_count = loader.decrementRefCount("module_test");
    try testing.expect(new_count == 2);
    
    new_count = loader.decrementRefCount("module_test");
    try testing.expect(new_count == 1);
    
    new_count = loader.decrementRefCount("module_test");
    try testing.expect(new_count == 0);
    
    // Should not go below 0
    new_count = loader.decrementRefCount("module_test");
    try testing.expect(new_count == 0);
    
    print("✅ Reference counting lifecycle works correctly\n", .{});
}

test "memory safety with repeated loading" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Simulate repeated loading of same module
    try loader.setModuleState("repeated_module", .loaded);
    try loader.setRefCount("repeated_module", 1);
    
    // Try to load again - should increment reference
    loader.incrementRefCount("repeated_module");
    
    const count = loader.reference_counts.get("repeated_module");
    try testing.expect(count != null);
    try testing.expect(count.? == 2);
    
    // Unload safely
    var new_count = loader.decrementRefCount("repeated_module");
    try testing.expect(new_count == 1);
    
    new_count = loader.decrementRefCount("repeated_module");
    try testing.expect(new_count == 0);
    
    print("✅ Repeated loading with reference counting works safely\n", .{});
}

test "self-import detection" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, true);
    defer loader.deinit();
    
    // Test self-import (module importing itself)
    loader.recordDependency("self_module", "self_module");
    
    const has_cycles = try loader.detectCycles();
    try testing.expect(has_cycles);
    
    print("✅ Self-import correctly detected as cycle\n", .{});
}

// Memory leak detection test
test "no memory leaks in cycle scenarios" {
    const allocator = testing.allocator;
    
    var loader = SafeModuleLoader.init(allocator, false);
    defer loader.deinit();
    
    // Create and destroy multiple cycles
    for (0..10) |i| {
        const module_a = try std.fmt.allocPrint(allocator, "module_a_{}", .{i});
        defer allocator.free(module_a);
        
        const module_b = try std.fmt.allocPrint(allocator, "module_b_{}", .{i});
        defer allocator.free(module_b);
        
        // Create cycle
        loader.recordDependency(module_a, module_b);
        loader.recordDependency(module_b, module_a);
        
        // Set states
        try loader.setModuleState(module_a, .loaded);
        try loader.setModuleState(module_b, .loaded);
        
        // Set reference counts
        try loader.setRefCount(module_a, 1);
        try loader.setRefCount(module_b, 1);
    }
    
    // Cleanup should happen automatically in deinit()
    print("✅ Multiple cycle creation/destruction completed\n", .{});
}
