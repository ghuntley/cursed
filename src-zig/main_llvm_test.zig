const std = @import("std");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const llvm_real = @import("llvm_real.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.print("Usage: {s} <file.csd>\n", .{args[0]});
        return;
    }

    const filename = args[1];
    
    // Read the file
    const file_content = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        std.debug.print("Error reading file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer allocator.free(file_content);

    std.debug.print("✅ File read successfully: {d} bytes\n", .{file_content.len});
    std.debug.print("Content: {s}\n", .{file_content});

    // Tokenize the source code
    var cursed_lexer = lexer.Lexer.init(allocator, file_content);

    const tokens = cursed_lexer.tokenize() catch |err| {
        std.debug.print("❌ Tokenization error: {}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);

    std.debug.print("✅ Tokenization successful. Tokens: {d}\n", .{tokens.items.len});

    // Parse the program  
    var cursed_parser = parser.Parser.init(allocator, tokens.items);

    const program = cursed_parser.parseProgram() catch |err| {
        std.debug.print("❌ Parse error: {}\n", .{err});
        return;
    };

    std.debug.print("✅ Parsing successful. Statements: {d}\n", .{program.statements.items.len});

    // Initialize real LLVM backend
    var llvm_codegen = llvm_real.RealLLVMCodeGen.init(allocator) catch |err| {
        std.debug.print("❌ LLVM initialization failed: {}\n", .{err});
        return;
    };
    defer llvm_codegen.deinit(allocator);

    std.debug.print("✅ LLVM backend initialized\n", .{});

    // Generate LLVM IR
    llvm_codegen.generateProgram(program) catch |err| {
        std.debug.print("❌ LLVM code generation failed: {}\n", .{err});
        return;
    };

    std.debug.print("✅ LLVM IR generation completed\n", .{});

    // Print the generated LLVM IR
    llvm_codegen.printModule();

    // Write bitcode to file
    const output_file = "output.bc";
    llvm_codegen.writeToFile(output_file) catch |err| {
        std.debug.print("❌ Failed to write bitcode: {}\n", .{err});
        return;
    };

    std.debug.print("✅ Bitcode written to {s}\n", .{output_file});
}
