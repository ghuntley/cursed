const std = @import("std");
const AdvancedCodeGen = @import("src-zig/advanced_codegen.zig").AdvancedCodeGen;
const WorkingCodeGen = @import("src-zig/working_codegen.zig").WorkingCodeGen;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    // Test the working codegen directly
    std.debug.print("Testing WorkingCodeGen...\n", .{});
    var working_codegen = try WorkingCodeGen.init(allocator);
    defer working_codegen.deinit();
    
    const source = 
        \\slay main_character() {
        \\    vibez.spill("Hello from CURSED Zig!")
        \\    sus x drip = 42
        \\    vibez.spill(x)
        \\}
    ;
    
    // Compile the source
    working_codegen.compile(source) catch |err| {
        std.debug.print("Compilation failed: {}\n", .{err});
        return;
    };
    
    std.debug.print("Generated LLVM IR:\n", .{});
    working_codegen.printIR();
    
    // Write executable
    std.debug.print("\nCompiling to executable...\n", .{});
    working_codegen.writeExecutable("test_cursed_program") catch |err| {
        std.debug.print("Failed to write executable: {}\n", .{err});
        return;
    };
    
    std.debug.print("Successfully compiled CURSED program!\n", .{});
    
    // Test the advanced codegen
    std.debug.print("\nTesting AdvancedCodeGen...\n", .{});
    var advanced_codegen = try AdvancedCodeGen.init(allocator);
    defer advanced_codegen.deinit();
    
    advanced_codegen.compileSource(source) catch |err| {
        std.debug.print("Advanced compilation failed: {}\n", .{err});
        return;
    };
    
    std.debug.print("Advanced codegen test completed!\n", .{});
}
