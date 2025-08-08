const std = @import("std");
const interpreter = @import("src-zig/interpreter.zig");
const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = "slay generic[T](val T) T { damn val }; vibez.spill(generic[drip](42))";
    
    std.debug.print("Testing generic function call with interpreter.zig\n", .{});
    std.debug.print("Source: {s}\n", .{source});
    
    // Tokenize
    var lex = lexer.Lexer.init(allocator, source);
    var tokens = lex.tokenize() catch |err| {
        std.debug.print("Tokenization error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    std.debug.print("Tokenized {d} tokens\n", .{tokens.items.len});
    
    // Parse
    var p = parser.Parser.init(allocator, tokens.items);
    var program = p.parseProgram() catch |err| {
        std.debug.print("Parsing error: {any}\n", .{err});
        return;
    };
    defer program.deinit(allocator);
    
    std.debug.print("Generated AST with {d} statements\n", .{program.statements.items.len});
    
    // Execute with interpreter
    var cursed_interpreter = interpreter.Interpreter.init(allocator);
    defer cursed_interpreter.deinit();
    
    cursed_interpreter.execute(program) catch |err| {
        std.debug.print("Execution error: {any}\n", .{err});
        return;
    };
    
    std.debug.print("✅ Test completed\n", .{});
}
