const std = @import("std");
const parser = @import("src-zig/parser.zig");
const lexer = @import("src-zig/lexer.zig");
const interpreter = @import("src-zig/interpreter.zig");
const ast = @import("src-zig/ast.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source =
        \\fn test_func() -> number {
        \\    return 42;
        \\}
        \\
        \\fn main() {
        \\    const result = test_func();
        \\    yap(result);
        \\}
    ;

    std.debug.print("Source: {s}\n", .{source});

    var lex = lexer.Lexer.init(allocator, source);

    var tokens = try lex.tokenize();
    defer tokens.deinit(allocator);

    std.debug.print("Tokens: {d}\n", .{tokens.items.len});

    var p = parser.Parser.init(allocator, tokens.items);
    defer p.deinit();

    var program = try p.parseProgram();
    defer program.deinit(allocator);

    std.debug.print("Program parsed with {d} statements\n", .{program.statements.items.len});

    // Check if we have any Block statements
    for (program.statements.items) |stmt_ptr| {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        std.debug.print("Statement type: {s}\n", .{@tagName(stmt.*)});
        switch (stmt.*) {
            .Function => |func| {
                std.debug.print("Function {s} has body with {} statements\n", .{ func.name, func.body.items.len });
                for (func.body.items) |body_stmt_ptr| {
                    const body_stmt: *ast.Statement = @ptrCast(@alignCast(body_stmt_ptr));
                    std.debug.print("  Body statement type: {s}\n", .{@tagName(body_stmt.*)});
                }
            },
            else => {},
        }
    }
}
