const std = @import("std");
const LLVMIRPipeline = @import("src-zig/llvm_ir_pipeline.zig").LLVMIRPipeline;
const print = std.debug.print;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Read the test file
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: {s} <test_file.💀.💀>\n", .{args[0]});
        return;
    }
    
    const filename = args[1];
    const source = std.fs.cwd().readFileAlloc(allocator, filename, std.math.maxInt(usize)) catch |err| {
        print("❌ Could not read file {s}: {}\n", .{ filename, err });
        return;
    };

    print("🚀 Testing LLVM IR Pipeline with {s}:\n", .{filename});
    print("Source code:\n{s}\n", .{source});
    print("================================================\n", .{});

    // Create pipeline
    const pipeline = LLVMIRPipeline.init(allocator, "test_module") catch |err| {
        print("❌ Failed to create pipeline: {}\n", .{err});
        return;
    };
    defer pipeline.deinit();

    // Test compilation with verbose output
    print("\n🧪 Testing full compilation pipeline\n", .{});
    pipeline.compileSource(source, "test_debug_binary", true) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        // Let's try to at least generate IR to see what goes wrong
        print("\n🔍 Attempting to generate IR only...\n", .{});
        
        var lex = @import("src-zig/lexer.zig").Lexer.init(allocator, source);
        const token_list = lex.tokenize() catch |lex_err| {
            print("❌ Tokenization failed: {}\n", .{lex_err});
            return;
        };
        const tokens = token_list.items;
        var parse = @import("src-zig/parser.zig").Parser.init(allocator, tokens);
        defer parse.deinit();
        const program = parse.parseProgram() catch |parse_err| {
            print("❌ Parsing failed: {}\n", .{parse_err});
            return;
        };
        
        pipeline.generateIR(program) catch |ir_err| {
            print("❌ IR generation failed: {}\n", .{ir_err});
            return;
        };
        
        pipeline.writeIRToFile("test_debug.ll") catch |write_err| {
            print("❌ IR writing failed: {}\n", .{write_err});
            return;
        };
        
        print("📄 Generated IR saved to test_debug.ll\n", .{});
        pipeline.dumpIR();
        return;
    };

    print("✅ Compilation successful!\n", .{});
}
