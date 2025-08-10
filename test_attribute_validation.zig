const std = @import("std");
const testing = std.testing;

const attribute_system = @import("src-zig/attribute_system.zig");

test "parseAttributeType rejects unknown attributes" {
    // Test valid attributes
    const valid_attrs = [_][]const u8{
        "performance", "inline", "optimize", "unroll", "vectorize",
        "memory_layout", "align", "pack", "cache", "debug", "no_debug",
        "profile_guided", "export", "import", "extern", "link_section",
        "unsafe", "bounds", "overflow", "atomic", "thread_safe", "lock",
        "test", "benchmark", "fuzz", "doc", "deprecated", "since"
    };
    
    for (valid_attrs) |attr_name| {
        const attr_type = attribute_system.parseAttributeType(attr_name);
        try testing.expect(attr_type != null);
        std.debug.print("✓ Valid attribute: @{s}\n", .{attr_name});
    }
    
    // Test invalid attributes that should return null
    const invalid_attrs = [_][]const u8{
        "unknown_attr", "invalid", "typo", "custom", "foo", "bar"
    };
    
    for (invalid_attrs) |attr_name| {
        const attr_type = attribute_system.parseAttributeType(attr_name);
        try testing.expect(attr_type == null);
        std.debug.print("✓ Invalid attribute rejected: @{s}\n", .{attr_name});
    }
}
