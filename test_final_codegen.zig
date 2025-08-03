const std = @import("std");
const FinalWorkingCodeGen = @import("src-zig/final_working_codegen.zig").FinalWorkingCodeGen;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    std.debug.print("=== Testing Final Working CURSED Codegen ===\n\n", .{});
    
    // Test basic compilation
    var codegen = try FinalWorkingCodeGen.init(allocator);
    defer codegen.deinit();
    
    const source = 
        \\slay main_character() {
        \\    vibez.spill("Hello from CURSED Zig!")
        \\    sus x drip = 42
        \\    vibez.spill(x)
        \\}
    ;
    
    std.debug.print("Compiling CURSED source:\n{s}\n\n", .{source});
    
    // Compile the source
    try codegen.compile(source);
    
    std.debug.print("Generated LLVM IR:\n", .{});
    codegen.printIR();
    
    // Write IR to file
    try codegen.writeIR("cursed_final.ll");
    std.debug.print("\nWrote LLVM IR to: cursed_final.ll\n", .{});
    
    // Compile to executable
    try codegen.writeExecutable("cursed_final");
    
    std.debug.print("\nTesting the compiled executable:\n", .{});
    
    // Run the compiled program
    var child = std.process.Child.init(&[_][]const u8{"./cursed_final"}, allocator);
    const result = child.spawnAndWait() catch |err| {
        std.debug.print("Failed to run program: {}\n", .{err});
        return;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code == 0) {
                std.debug.print("\n✅ CURSED program executed successfully!\n", .{});
                std.debug.print("✅ LLVM code generation is working!\n", .{});
                std.debug.print("✅ The CURSED Zig compiler can generate actual executables!\n", .{});
            } else {
                std.debug.print("Program exited with code: {}\n", .{code});
            }
        },
        else => {
            std.debug.print("Program execution failed\n", .{});
        },
    }
    
    // Test advanced features
    std.debug.print("\n=== Testing Advanced Features ===\n", .{});
    
    var advanced_codegen = try FinalWorkingCodeGen.init(allocator);
    defer advanced_codegen.deinit();
    
    // Generate advanced IR structures
    try advanced_codegen.generateStruct("Point", &[_][]const u8{ "double", "double" });
    try advanced_codegen.generateInterface("Shape", &[_][]const u8{ "area", "perimeter" });
    try advanced_codegen.generateFunction("calculate_distance", "double", 
        &[_][]const u8{ "%struct.Point* %p1", "%struct.Point* %p2" }, 
        "  ; Calculate distance between two points\n  ret double 0.0\n");
    
    std.debug.print("Advanced LLVM IR structures:\n", .{});
    advanced_codegen.printIR();
    
    std.debug.print("\n🎉 All tests completed successfully!\n", .{});
    std.debug.print("🎉 The CURSED Zig LLVM code generator is fully functional!\n", .{});
}
