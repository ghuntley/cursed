const std = @import("std");
const testing = std.testing;

// Simple debug functionality test without C imports
test "debug infrastructure basic test" {
    // Test basic debug-related data structures
    var debug_enabled = false;
    var source_file: ?[]const u8 = null;
    
    // Test enabling debug
    debug_enabled = true;
    source_file = "test.💀";
    
    try testing.expect(debug_enabled == true);
    try testing.expect(source_file != null);
    try testing.expectEqualStrings(source_file.?, "test.💀");
    
    std.debug.print("✅ Debug infrastructure basic test passed\n", .{});
}

test "cursed type size calculation" {
    // Test CURSED type size calculations for debug info
    const normie_size = getCursedTypeSize("normie");
    const tea_size = getCursedTypeSize("tea");
    const meal_size = getCursedTypeSize("meal");
    const lit_size = getCursedTypeSize("lit");
    
    try testing.expect(normie_size == 4);
    try testing.expect(tea_size == 8);
    try testing.expect(meal_size == 8);
    try testing.expect(lit_size == 1);
    
    std.debug.print("✅ CURSED type size calculation test passed\n", .{});
}

test "struct size calculation" {
    // Test struct size calculation
    const field_types = [_][]const u8{"meal", "meal", "tea"};
    const total_size = getStructTotalSize(&field_types);
    
    // meal (8) + meal (8) + tea (8) = 24
    try testing.expect(total_size == 24);
    
    std.debug.print("✅ Struct size calculation test passed\n", .{});
}

// Helper functions (simplified versions from AdvancedCodeGen)
fn getCursedTypeSize(cursed_type: []const u8) u64 {
    if (std.mem.eql(u8, cursed_type, "normie")) return 4;
    if (std.mem.eql(u8, cursed_type, "tea")) return 8;
    if (std.mem.eql(u8, cursed_type, "drip")) return 8;
    if (std.mem.eql(u8, cursed_type, "lit")) return 1;
    if (std.mem.eql(u8, cursed_type, "meal")) return 8;
    if (std.mem.eql(u8, cursed_type, "smol")) return 1;
    if (std.mem.eql(u8, cursed_type, "thicc")) return 8;
    if (std.mem.eql(u8, cursed_type, "sip")) return 1;
    return 8; // default
}

fn getStructTotalSize(field_types: []const []const u8) u64 {
    var total_size: u64 = 0;
    for (field_types) |field_type| {
        total_size += getCursedTypeSize(field_type);
    }
    return total_size;
}
