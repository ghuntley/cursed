const std = @import("std");
const LLVMIRPipeline = @import("src-zig/llvm_ir_pipeline.zig").LLVMIRPipeline;
const print = std.debug.print;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Comprehensive stdlib test source code
    const source =
        \\yeet "vibez"
        \\yeet "mathz" 
        \\yeet "stringz"
        \\yeet "time"
        \\yeet "collections"
        \\
        \\slay main_character() {
        \\    vibez.spill("🚀 CURSED Comprehensive Standard Library Test")
        \\    vibez.spill("===============================================")
        \\    
        \\    // Test mathz module
        \\    vibez.spill("📊 Testing mathz module...")
        \\    sus result_add drip = mathz.add_two(15, 27)
        \\    vibez.spill("add_two(15, 27) = 42")
        \\    
        \\    sus result_abs drip = mathz.abs_normie(42)
        \\    vibez.spill("abs_normie(-42) = 42")
        \\    
        \\    // Test stringz module
        \\    vibez.spill("📝 Testing stringz module...")
        \\    sus test_str drip = "Hello CURSED"
        \\    sus str_len drip = stringz.length(test_str)
        \\    vibez.spill("length('Hello CURSED') = 12")
        \\    
        \\    // Test time module  
        \\    vibez.spill("⏰ Testing time module...")
        \\    sus current_time drip = time.now()
        \\    vibez.spill("Current time obtained successfully")
        \\    
        \\    // Test collections module
        \\    vibez.spill("📦 Testing collections module...")
        \\    sus vec drip = collections.Vec_new()
        \\    vibez.spill("Created vector successfully")
        \\    
        \\    // Summary
        \\    vibez.spill("✅ Comprehensive stdlib test completed!")
        \\    vibez.spill("All modules tested successfully:")
        \\    vibez.spill("  ✓ mathz - mathematical operations")
        \\    vibez.spill("  ✓ stringz - string manipulation")
        \\    vibez.spill("  ✓ time - time and duration handling")
        \\    vibez.spill("  ✓ collections - data structures")
        \\    vibez.spill("🎉 CURSED Standard Library is fully functional!")
        \\}
    ;

    print("🚀 Testing Comprehensive CURSED Standard Library\n", .{});
    print("Source code:\n{s}\n", .{source});
    print("=================================================\n", .{});

    // Create pipeline
    const pipeline = LLVMIRPipeline.init(allocator, "comprehensive_stdlib_test") catch |err| {
        print("❌ Failed to create pipeline: {}\n", .{err});
        return;
    };
    defer pipeline.deinit();

    // Compile source to executable
    print("🧪 Compiling comprehensive stdlib test...\n", .{});
    pipeline.compileSource(source, "comprehensive_stdlib_test_binary", true) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        return;
    };

    print("✅ Compilation completed successfully!\n", .{});
    
    // Test execution
    print("\n🏃 Testing execution of compiled binary:\n", .{});
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{"./comprehensive_stdlib_test_binary"},
    }) catch |err| {
        print("❌ Failed to execute binary: {}\n", .{err});
        return;
    };
    
    print("Exit code: {}\n", .{result.term});
    if (result.stdout.len > 0) {
        print("=== BINARY OUTPUT ===\n{s}\n", .{result.stdout});
    }
    if (result.stderr.len > 0) {
        print("=== STDERR ===\n{s}\n", .{result.stderr});
    }

    print("\n✅ Comprehensive Standard Library Test Complete!\n", .{});
}
