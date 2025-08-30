const std = @import("std");
const LLVMIRPipeline = @import("src-zig/llvm_ir_pipeline.zig").LLVMIRPipeline;
const print = std.debug.print;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Test source code - simplest working case
    const source =
        \\slay main_character() {
        \\}
    ;

    print("🚀 Testing LLVM IR Pipeline with source:\n{s}\n", .{source});
    print("================================================\n", .{});

    // Create pipeline
    const pipeline = LLVMIRPipeline.init(allocator, "test_module") catch |err| {
        print("❌ Failed to create pipeline: {}\n", .{err});
        return;
    };
    defer pipeline.deinit();

    // Test 1: Compile source to executable
    print("\n🧪 Test 1: Full compilation pipeline\n", .{});
    pipeline.compileSource(source, "test_binary", true) catch |err| {
        print("❌ Full compilation failed: {}\n", .{err});
        return;
    };

    // Test 2: Write IR only
    print("\n🧪 Test 2: IR generation only\n", .{});
    const pipeline2 = LLVMIRPipeline.init(allocator, "test_ir_module") catch |err| {
        print("❌ Failed to create second pipeline: {}\n", .{err});
        return;
    };
    defer pipeline2.deinit();
    
    // Parse and generate IR
    var lex = @import("src-zig/lexer.zig").Lexer.init(allocator, source);
    const token_list = lex.tokenize() catch |err| {
        print("❌ Tokenization failed: {}\n", .{err});
        return;
    };
    const tokens = token_list.items;
    var parse = @import("src-zig/parser.zig").Parser.init(allocator, tokens);
    defer parse.deinit();
    const program = parse.parseProgram() catch |err| {
        print("❌ Parsing failed: {}\n", .{err});
        return;
    };
    
    pipeline2.generateIR(program) catch |err| {
        print("❌ IR generation failed: {}\n", .{err});
        return;
    };
    
    pipeline2.writeIRToFile("test_output.ll") catch |err| {
        print("❌ IR file write failed: {}\n", .{err});
        return;
    };

    print("\n✅ All tests completed successfully!\n", .{});
    
    // Test execution
    print("\n🏃 Testing execution of compiled binary:\n", .{});
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{"./test_binary"},
    }) catch |err| {
        print("❌ Failed to execute binary: {}\n", .{err});
        return;
    };
    
    print("Exit code: {}\n", .{result.term});
    if (result.stdout.len > 0) {
        print("Stdout:\n{s}\n", .{result.stdout});
    }
    if (result.stderr.len > 0) {
        print("Stderr:\n{s}\n", .{result.stderr});
    }
}
