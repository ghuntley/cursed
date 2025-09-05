const std = @import("std");
const llvm_pipeline = @import("src-zig/llvm_ir_pipeline.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Read the fizzbuzz source file
    const source = try std.fs.cwd().readFileAlloc(allocator, "fizzbuzz.💀", 1024 * 1024);
    defer allocator.free(source);
    
    std.debug.print("🧪 Testing LLVM IR generation for FizzBuzz...\n", .{});
    
    // Create LLVM pipeline
    var pipeline = try llvm_pipeline.LLVMIRPipeline.init(allocator, "fizzbuzz");
    defer pipeline.deinit();
    
    // Compile the source to an executable
    try pipeline.compileSource(source, "fizzbuzz_output", true);
    
    std.debug.print("✅ LLVM IR generation test completed successfully!\n", .{});
}
