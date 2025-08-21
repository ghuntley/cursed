const std = @import("std");
const testing = std.testing;

test "linker script manager integration" {
    const linker_manager = @import("linker_script_manager.zig");
    var manager = linker_manager.LinkerScriptManager.init(testing.allocator, "/test");
    defer manager.deinit(allocator);
    
    // Test basic functionality
    const config = try manager.getLinkerConfig("x86_64-unknown-linux-gnu");
    try testing.expect(config.linker_args.len > 0);
}

test "target triple normalization integration" {
    const target_norm = @import("target_triple_normalization.zig");
    var normalizer = target_norm.TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit(allocator);
    
    // Test basic normalization
    const triple = try normalizer.normalizeTriple("linux-x64");
    try testing.expectEqualStrings(triple.arch, "x86_64");
    try testing.expectEqualStrings(triple.os, "linux");
}
