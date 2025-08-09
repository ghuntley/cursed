const std = @import("std");
const llvm_backend_fixed = @import("src-zig/llvm_backend_fixed.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const test_program = 
        \\fr fr Enhanced LLVM Backend Test
        \\vibez.spill("Testing enhanced LLVM backend...")
        \\
        \\sus x = 10
        \\sus y = 5
        \\sus result = x + y
        \\
        \\vibez.spill("Variables: x =", x, ", y =", y)
        \\vibez.spill("Addition result:", result)
        \\
        \\slay multiply(a, b) {
        \\    damn a * b
        \\}
        \\
        \\sus mult_result = multiply(6, 7)
        \\vibez.spill("multiply(6, 7) =", mult_result)
        \\
        \\vibez.spill("Enhanced LLVM test complete!")
    ;
    
    std.debug.print("🚀 Testing Enhanced LLVM Backend\n", .{});
    
    try llvm_backend_fixed.compileToLLVM(allocator, test_program, "enhanced_test.ll");
    
    std.debug.print("✅ LLVM IR generation completed\n", .{});
    
    // Try to compile to native
    try llvm_backend_fixed.compileIRToNative(allocator, "enhanced_test.ll", "enhanced_test");
    
    std.debug.print("✅ Native compilation completed\n", .{});
}
