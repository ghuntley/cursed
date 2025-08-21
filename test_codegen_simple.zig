const std = @import("std");
const ast = @import("src-zig/ast.zig");
const codegen = @import("src-zig/codegen.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("🧪 Testing CURSED Code Generation\n", .{});

    // Initialize codegen
    var cg = codegen.CodeGen.init(allocator);
    defer cg.deinit();

    std.debug.print("✅ CodeGen initialized successfully\n", .{});

    // Create a simple program
    var program = ast.Program.init(allocator);
    defer program.deinit(allocator);

    std.debug.print("✅ Program AST created successfully\n", .{});

    // Try to compile (this will create basic structure)
    cg.compile(program) catch |err| {
        std.debug.print("⚠️  Compilation failed: {}\n", .{err});
        return;
    };

    std.debug.print("✅ Basic compilation successful\n", .{});

    // Generate and print IR
    const ir = cg.generateIR() catch |err| {
        std.debug.print("⚠️  IR generation failed: {}\n", .{err});
        return;
    };
    defer allocator.free(ir);

    std.debug.print("📄 Generated LLVM IR:\n{s}\n", .{ir});
    std.debug.print("🎉 Code generation test completed successfully!\n", .{});
}
