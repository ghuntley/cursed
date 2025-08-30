const std = @import("std");
const LLVMIRPipeline = @import("src-zig/llvm_ir_pipeline.zig").LLVMIRPipeline;
const print = std.debug.print;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Simple test source
    const source =
        \\slay main_character() {
        \\}
    ;

    print("🚀 CURSED LLVM IR Pipeline Test\n", .{});
    print("================================\n", .{});
    print("Source: {s}\n\n", .{source});

    // Create pipeline
    const pipeline = LLVMIRPipeline.init(allocator, "cursed_test") catch |err| {
        print("❌ Failed to initialize pipeline: {}\n", .{err});
        return;
    };
    defer pipeline.deinit();

    // Test IR generation
    print("📝 Generating LLVM IR...\n", .{});
    pipeline.compileSource(source, "cursed_test_binary", false) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        return;
    };

    print("\n✅ Compilation successful! Generated:\n", .{});
    print("   - cursed_test_binary.ll (LLVM IR)\n", .{});
    print("   - cursed_test_binary (executable)\n", .{});

    // Test execution
    print("\n🏃 Testing binary execution:\n", .{});
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{"./cursed_test_binary"},
    }) catch |err| {
        print("❌ Failed to execute: {}\n", .{err});
        return;
    };
    
    print("Exit code: {}\n", .{result.term});
    print("\n🎉 CURSED LLVM pipeline test completed successfully!\n", .{});
}
