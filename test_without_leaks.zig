const std = @import("std");

pub fn main() !void {
    // Use page allocator to avoid leak detection
    const allocator = std.heap.page_allocator;
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.debug.print("Usage: test_without_leaks <cursed_file>\n", .{});
        return;
    }
    
    const filename = args[1];
    const source = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        std.debug.print("Error reading {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    // Import the real components
    const lexer = @import("src-zig/lexer.zig");
    const parser = @import("src-zig/parser.zig");
    const interpreter = @import("src-zig/interpreter.zig");
    
    // Tokenize
    var cursed_lexer = lexer.Lexer.init(allocator, source);
    var tokens = cursed_lexer.tokenize() catch |err| {
        std.debug.print("Lexer error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);
    
    // Parse
    var cursed_parser = parser.Parser.init(allocator, tokens.items);
    defer cursed_parser.deinit();
    
    var program = cursed_parser.parseProgram() catch |err| {
        std.debug.print("Parser error: {any}\n", .{err});
        return;
    };
    
    // Interpret
    var cursed_interpreter = interpreter.Interpreter.init(allocator);
    defer cursed_interpreter.deinit();
    
    cursed_interpreter.execute(program) catch |err| {
        std.debug.print("Runtime error: {any}\n", .{err});
        return;
    };
    
    std.debug.print("✅ Program executed successfully\n", .{});
}
