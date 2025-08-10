const std = @import("std");
const SimpleLLVMIRGenerator = @import("src-zig/simple_llvm_ir_generator.zig").SimpleLLVMIRGenerator;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = 
        \\slay main_character() {
        \\    vibez.spill("Hello from CURSED LLVM!");
        \\    sus x drip = 42;
        \\    vibez.spill(x);
        \\}
    ;
    
    var generator = SimpleLLVMIRGenerator.init(allocator);
    defer generator.deinit();
    
    generator.setVerbose(true);
    
    std.debug.print("🚀 Testing Simple LLVM IR Generator...\n", .{});
    
    try generator.generateFromSource(source);
    
    std.debug.print("🔍 Generated IR:\n{s}\n", .{generator.getIR()});
    
    try generator.writeToFile("test_output.ll");
    
    std.debug.print("✅ LLVM IR written to test_output.ll\n", .{});
    
    // Try to compile it
    try generator.compileToExecutable("test_output.ll", "test_program");
    
    std.debug.print("✅ Compilation test completed!\n", .{});
}
