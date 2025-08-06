const std = @import("std");
const lexer = @import("src-zig/lexer.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.debug.print("Usage: simple_format_demo <file.csd>\n", .{});
        return;
    }
    
    const file_path = args[1];
    const source = std.fs.cwd().readFileAlloc(allocator, file_path, 1024 * 1024) catch |err| {
        std.debug.print("Error reading file: {}\n", .{err});
        return;
    };
    defer allocator.free(source);
    
    std.debug.print("=== Original Code ===\n{s}\n\n", .{source});
    
    // Tokenize
    var token_lexer = lexer.Lexer.init(allocator, source);
    const tokens = try token_lexer.tokenize();
    defer tokens.deinit();
    
    std.debug.print("=== Tokens ===\n", .{});
    for (tokens.items) |token| {
        std.debug.print("{}: '{s}'\n", .{ token.kind, token.lexeme });
    }
    
    std.debug.print("\n=== Formatted Code ===\n", .{});
    var formatted = std.ArrayList(u8).init(allocator);
    defer formatted.deinit();
    
    var line_length: u32 = 0;
    const max_line_length: u32 = 100;
    
    for (tokens.items) |token| {
        switch (token.kind) {
            .Slay => {
                try formatted.appendSlice("slay ");
                line_length += 5;
            },
            .Sus => {
                try formatted.appendSlice("sus ");
                line_length += 4;
            },
            .Squad => {
                try formatted.appendSlice("squad ");
                line_length += 6;
            },
            .Yeet => {
                try formatted.appendSlice("yeet ");
                line_length += 5;
            },
            .LeftBrace => {
                try formatted.appendSlice(" {\n");
                line_length = 0;
            },
            .RightBrace => {
                try formatted.appendSlice("}\n");
                line_length = 0;
            },
            .Equal => {
                try formatted.appendSlice(" = ");
                line_length += 3;
            },
            .Identifier, .StringLiteral, .Number, .Integer => {
                try formatted.appendSlice(token.lexeme);
                line_length += @as(u32, @intCast(token.lexeme.len));
            },
            .Comment => {
                try formatted.appendSlice(token.lexeme);
                try formatted.append('\n');
                line_length = 0;
            },
            .Newline => {
                try formatted.append('\n');
                line_length = 0;
            },
            else => {
                try formatted.appendSlice(token.lexeme);
                line_length += @as(u32, @intCast(token.lexeme.len));
            },
        }
        
        if (line_length > max_line_length) {
            try formatted.append('\n');
            line_length = 0;
        }
    }
    
    std.debug.print("{s}\n", .{formatted.items});
    
    // Basic linting
    std.debug.print("\n=== Lint Issues ===\n", .{});
    
    var issues: u32 = 0;
    for (tokens.items) |token| {
        if (std.mem.eql(u8, token.lexeme, "function")) {
            std.debug.print("Warning: Use 'slay' instead of 'function'\n", .{});
            issues += 1;
        }
        if (std.mem.eql(u8, token.lexeme, "var")) {
            std.debug.print("Warning: Use 'sus' instead of 'var'\n", .{});
            issues += 1;
        }
        if (std.mem.eql(u8, token.lexeme, "return")) {
            std.debug.print("Warning: Use 'yolo' instead of 'return'\n", .{});
            issues += 1;
        }
    }
    
    if (issues == 0) {
        std.debug.print("No lint issues found!\n", .{});
    } else {
        std.debug.print("Found {} lint issues\n", .{issues});
    }
}
