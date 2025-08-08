const std = @import("std");

// Test minimal LLVM C imports
const c = @cImport({
    @cInclude("llvm_minimal.h");
});

pub fn main() !void {
    std.debug.print("Testing LLVM minimal imports...\n", .{});
    
    // Test basic LLVM context creation
    const context = c.LLVMGetGlobalContext();
    if (context != null) {
        std.debug.print("✅ LLVM context created successfully\n", .{});
    } else {
        std.debug.print("❌ Failed to create LLVM context\n", .{});
    }
    
    // Test initialization
    c.llvm_minimal_init();
    std.debug.print("✅ LLVM initialization completed\n", .{});
}
