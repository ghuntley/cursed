const std = @import("std");
const testing = std.testing;
const AdvancedCodeGen = @import("advanced_codegen.zig").AdvancedCodeGen;
const DebugInfoGenerator = @import("debug_info.zig").DebugInfoGenerator;

// Test debug information integration with the CURSED compiler
test "debug info integration" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    // Enable debug info for test file
    try codegen.enableDebugInfo("debug_info_comprehensive_test.csd");
    
    // Verify debug generator was initialized
    try testing.expect(codegen.debug_enabled);
    try testing.expect(codegen.debug_generator != null);
    try testing.expect(codegen.source_file != null);
    
    std.debug.print("✅ Debug info integration test passed\n", .{});
}

/// Test CURSED type debug info generation
test "cursed type debug info" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    try codegen.enableDebugInfo("test.csd");
    
    // Test creating debug info for CURSED types
    const normie_type = try codegen.getCursedDebugType("normie");
    const tea_type = try codegen.getCursedDebugType("tea");
    const meal_type = try codegen.getCursedDebugType("meal");
    const lit_type = try codegen.getCursedDebugType("lit");
    
    try testing.expect(normie_type != null);
    try testing.expect(tea_type != null);
    try testing.expect(meal_type != null);
    try testing.expect(lit_type != null);
    
    std.debug.print("✅ CURSED type debug info test passed\n", .{});
}

/// Test struct debug info generation
test "struct debug info" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    try codegen.enableDebugInfo("test.csd");
    
    // Create debug info for Point struct
    const field_names = [_][]const u8{"x", "y", "label"};
    const field_types = [_][]const u8{"meal", "meal", "tea"};
    
    const struct_debug = try codegen.generateStructDebugInfo("Point", &field_names, &field_types);
    try testing.expect(struct_debug != null);
    
    std.debug.print("✅ Struct debug info test passed\n", .{});
}

/// Test interface debug info generation
test "interface debug info" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    try codegen.enableDebugInfo("test.csd");
    
    // Create debug info for Drawable interface
    const method_names = [_][]const u8{"draw", "area"};
    
    const interface_debug = try codegen.generateInterfaceDebugInfo("Drawable", &method_names);
    try testing.expect(interface_debug != null);
    
    std.debug.print("✅ Interface debug info test passed\n", .{});
}

/// Test source location tracking
test "source location tracking" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    try codegen.enableDebugInfo("test.csd");
    
    // Verify source locations map was initialized
    try testing.expect(codegen.source_locations.count() == 0);
    
    std.debug.print("✅ Source location tracking test passed\n", .{});
}

/// Test debug scope management
test "debug scope management" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    try codegen.enableDebugInfo("test.csd");
    
    // Test scope push/pop
    const scope1 = try codegen.pushDebugScope(10, 0);
    try testing.expect(scope1 != null);
    
    const scope2 = try codegen.pushDebugScope(20, 4);
    try testing.expect(scope2 != null);
    
    codegen.popDebugScope();
    codegen.popDebugScope();
    
    std.debug.print("✅ Debug scope management test passed\n", .{});
}

/// Test complete debug compilation workflow
test "complete debug compilation" {
    const allocator = testing.allocator;
    
    var codegen = try AdvancedCodeGen.init(allocator);
    defer codegen.deinit(allocator);
    
    // Enable debug info
    try codegen.enableDebugInfo("debug_info_comprehensive_test.csd");
    
    // Simulate compilation with debug info
    const source = 
        \\slay main() {
        \\    sus x normie = 42
        \\    vibez.spill("Hello Debug World!")
        \\    damn x
        \\}
    ;
    
    try codegen.compileSource(source);
    
    std.debug.print("✅ Complete debug compilation test passed\n", .{});
}
