const std = @import("std");
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");
const Interpreter = @import("src-zig/interpreter.zig").Interpreter;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = 
        \\yeet "fs"
        \\yeet "io"
        \\
        \\slay test_fs() {
        \\    sus content tea = fs.read_file("test.txt")
        \\    io.println(content)
        \\}
        \\
        \\slay main_character() {
        \\    test_fs()
        \\}
    ;

    std.debug.print("=== Testing new fs and io modules ===\n", .{});
    
    // Tokenize
    var cursed_lexer = lexer.Lexer.init(allocator, source);
    var tokens = cursed_lexer.tokenize() catch |err| {
        std.debug.print("Lexer error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);
    
    std.debug.print("✅ Tokenized {d} tokens\n", .{tokens.items.len});
    
    // Parse
    var cursed_parser = parser.Parser.init(allocator, tokens.items);
    var program = cursed_parser.parseProgram() catch |err| {
        std.debug.print("Parser error: {any}\n", .{err});
        return;
    };
    defer program.deinit(allocator);
    
    std.debug.print("✅ Parsed {d} statements\n", .{program.statements.items.len});
    
    // Interpret
    var interpreter = Interpreter.init(allocator);
    defer interpreter.deinit();
    
    interpreter.execute(program) catch |err| {
        std.debug.print("Interpreter error: {any}\n", .{err});
        return;
    };
    
    std.debug.print("✅ Execution completed successfully\n", .{});
}
