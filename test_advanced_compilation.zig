const std = @import("std");
const simple_fixed = @import("src-zig/llvm_simple_fixed.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const advanced_source = 
        \\// Advanced CURSED features test
        \\ready (x) { mood 1: vibez.spill("Pattern 1") }
        \\sus channel drip = dm_create()
        \\dm_send(channel, 42)
        \\later { vibez.spill("Cleanup") }
        \\sus result drip = risky_operation()?
        \\stan worker_function()
        \\vibez.spill("All features!")
    ;
    
    std.debug.print("🚀 Testing advanced CURSED features compilation...\n", .{});
    
    try simple_fixed.compileAdvancedFeatures(allocator, advanced_source, "advanced_features_optimized.ll");
    
    std.debug.print("✅ Advanced features compilation test complete!\n", .{});
}
