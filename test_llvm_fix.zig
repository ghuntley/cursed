const std = @import("std");
const llvm_backend_fixed = @import("src-zig/llvm_backend_fixed.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    try llvm_backend_fixed.testLLVMBackendFixed(allocator);
}
