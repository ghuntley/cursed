const std = @import("std");
const llvm_real = @import("llvm_real.zig");
const ast = @import("ast.zig");

// Simple test for LLVM function compilation
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("Testing LLVM Real Code Generation...\n", .{});

    // Create a basic LLVM code generator
    var codegen = llvm_real.RealLLVMCodeGen.init(allocator) catch |err| {
        std.debug.print("LLVM Initialization failed: {}\n", .{err});
        std.debug.print("This indicates the C wrapper or LLVM libraries are not working.\n", .{});
        return;
    };
    defer codegen.deinit();

    std.debug.print("LLVM initialization successful!\n", .{});

    // Create a minimal program with a simple function
    var program = ast.Program.init(allocator);
    defer program.deinit(allocator);

    // Test generateProgram
    codegen.generateProgram(program) catch |err| {
        std.debug.print("Program generation failed: {}\n", .{err});
        std.debug.print("This indicates issues with basic LLVM IR generation.\n", .{});
        return;
    };

    std.debug.print("Program generation successful!\n", .{});

    // Print the generated IR
    codegen.printModule();

    std.debug.print("LLVM test completed successfully!\n", .{});
}
