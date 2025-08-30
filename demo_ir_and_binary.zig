const std = @import("std");
const LLVMIRPipeline = @import("src-zig/llvm_ir_pipeline.zig").LLVMIRPipeline;
const print = std.debug.print;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const source =
        \\slay main_character() {
        \\}
    ;

    print("🚀 CURSED LLVM IR Pipeline Demo\n", .{});
    print("================================\n", .{});

    // Demo 1: Generate IR file only
    print("\n📝 Demo 1: Generating LLVM IR file\n", .{});
    print("-----------------------------------\n", .{});
    
    const pipeline1 = LLVMIRPipeline.init(allocator, "cursed_ir_demo") catch |err| {
        print("❌ Failed to initialize pipeline: {}\n", .{err});
        return;
    };
    defer pipeline1.deinit();
    
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
    
    pipeline1.generateIR(program) catch |err| {
        print("❌ IR generation failed: {}\n", .{err});
        return;
    };
    
    pipeline1.writeIRToFile("demo.ll") catch |err| {
        print("❌ IR write failed: {}\n", .{err});
        return;
    };

    // Demo 2: Generate binary
    print("\n🔥 Demo 2: Compiling to executable\n", .{});
    print("----------------------------------\n", .{});
    
    const pipeline2 = LLVMIRPipeline.init(allocator, "cursed_binary_demo") catch |err| {
        print("❌ Failed to initialize pipeline: {}\n", .{err});
        return;
    };
    defer pipeline2.deinit();
    
    pipeline2.compileSource(source, "demo_binary", false) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        return;
    };

    // Show results
    print("\n✨ Results:\n", .{});
    print("----------\n", .{});
    print("📄 LLVM IR file: demo.ll\n", .{});
    print("⚙️  Executable:   demo_binary\n", .{});

    // Show IR contents
    print("\n🔍 Generated LLVM IR:\n", .{});
    print("---------------------\n", .{});
    const ir_content = std.fs.cwd().readFileAlloc(allocator, "demo.ll", 1024*1024) catch |err| {
        print("❌ Failed to read IR file: {}\n", .{err});
        return;
    };
    defer allocator.free(ir_content);
    print("{s}\n", .{ir_content});

    // Test execution
    print("🏃 Testing binary execution:\n", .{});
    print("----------------------------\n", .{});
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{"./demo_binary"},
    }) catch |err| {
        print("❌ Failed to execute: {}\n", .{err});
        return;
    };
    
    print("Exit code: {}\n", .{result.term});
    print("\n🎉 CURSED LLVM pipeline demo completed successfully!\n", .{});
}
