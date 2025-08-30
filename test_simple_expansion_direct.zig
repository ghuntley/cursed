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
        \\slay test_fs_basic() {
        \\    sus content tea = fs.read_file("test.txt")
        \\    io.println(content)
        \\}
        \\
        \\slay test_fs_exists() {
        \\    sus exists lit = fs.file_exists("config.json")
        \\    io.println(exists)
        \\}
        \\
        \\slay test_fs_size() {
        \\    sus size thicc = fs.get_file_size("data.csv")
        \\    io.println(size)
        \\}
        \\
        \\slay main_character() {
        \\    test_fs_basic()
        \\    test_fs_exists()
        \\    test_fs_size()
        \\}
    ;

    std.debug.print("=== Testing expanded fs and io modules ===\n", .{});
    
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
