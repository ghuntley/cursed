const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");
const interpreter = @import("src-zig/interpreter.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: test_defer_ast <file.csd>\n", .{});
        return;
    }
    
    const filename = args[1];
    print("🚀 Testing defer with AST interpreter: {s}\n", .{filename});
    
    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });
    
    // Tokenize
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error: {}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    print("📝 Lexed {} tokens\n", .{tokens.items.len});
    
    // Parse
    var p = parser.Parser.init(allocator, tokens.items);
    
    const program = p.parseProgram() catch |err| {
        print("❌ Parser error: {}\n", .{err});
        return;
    };
    
    print("🌳 Parsed AST successfully\n", .{});
    
    // Execute with AST interpreter
    var interp = interpreter.Interpreter.init(allocator);
    defer interp.deinit();
    
    print("🚀 Executing with AST interpreter...\n", .{});
    try interp.execute(program);
    print("✅ Execution completed\n", .{});
}
