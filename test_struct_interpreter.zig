const std = @import("std");
const print = std.debug.print;

const lexer = @import("src-zig/lexer.zig");
const SimpleInterpreter = @import("src-zig/simple_interpreter.zig").SimpleInterpreter;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test CURSED program with struct
    const source = 
        \\squad Point {
        \\    spill x normie
        \\    spill y normie
        \\}
        \\
        \\sus point Point = Point{x: 10, y: 20}
        \\vibez.spill("Point created successfully")
        \\vibez.spill("X:", point.x)
        \\vibez.spill("Y:", point.y)
    ;

    print("Testing CURSED struct system with simple interpreter\n", .{});
    print("Source code:\n{s}\n\n", .{source});

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("Lexer error: {}\n", .{err});
        return;
    };

    print("Tokens generated: {}\n", .{tokens.items.len});
    for (tokens.items, 0..) |token, i| {
        print("  {}: {any} = '{s}'\n", .{ i, token.kind, token.lexeme });
    }
    print("\n", .{});

    // Execute with simple interpreter
    var interpreter = SimpleInterpreter.init(allocator);
    defer interpreter.deinit();

    interpreter.execute(tokens.items) catch |err| {
        print("Interpreter error: {}\n", .{err});
        return;
    };

    print("Struct system test completed!\n", .{});
}
