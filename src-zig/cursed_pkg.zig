// CURSED Package Manager - Zig Wrapper
// This wraps the CURSED-language package manager for native execution

const std = @import("std");
const interpreter = @import("interpreter.zig");
const parser = @import("parser.zig");
const lexer = @import("lexer.zig");

// Package manager entry point
pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Get command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Skip the program name (args[0])
    const pkg_args = if (args.len > 1) args[1..] else &[_][]const u8{};

    // Load the package manager CURSED source
    const pkg_manager_source = @embedFile("../tools/cursed-pkg/main.csd");

    // Create lexer and parse the source
    var lex = lexer.Lexer.init(allocator, pkg_manager_source);
    var tokens: std.ArrayList(lexer.Token) = .empty;
    defer tokens.deinit();

    // Tokenize the source
    while (true) {
        const token = lex.next_token();
        try tokens.append(token);
        if (token.type == .EOF) break;
    }

    // Parse the tokens into AST
    var parse = parser.Parser.init(allocator, tokens.items);
    const ast = parse.parse() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        std.process.exit(1);
    };

    // Create interpreter
    var interp = interpreter.Interpreter.init(allocator);
    defer interp.deinit();

    // Set up command line arguments in interpreter environment
    var arg_array: std.ArrayList(interpreter.Value) = .empty;
    defer arg_array.deinit();

    for (pkg_args) |arg| {
        const arg_value = interpreter.Value{ .String = try allocator.dupe(u8, arg) };
        try arg_array.append(arg_value);
    }

    const args_value = interpreter.Value{ .Array = arg_array.items };
    try interp.set_variable("args", args_value);

    // Execute the package manager
    const result = interp.interpret(ast) catch |err| {
        switch (err) {
            error.RuntimeError => {
                std.debug.print("Runtime error in package manager\n", .{});
                std.process.exit(1);
            },
            else => {
                std.debug.print("Error executing package manager: {}\n", .{err});
                std.process.exit(1);
            }
        }
    };

    // Exit with the return code from the main function
    const exit_code = switch (result) {
        .Integer => |i| @intCast(i),
        .Float => |f| @intFromFloat(f),
        else => 0,
    };

    std.process.exit(@intCast(exit_code));
}
