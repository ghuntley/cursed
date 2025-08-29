const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Windows-compatible minimal CURSED compiler
// Avoids LLVM dependencies to ensure cross-compilation works

// Import only basic, LLVM-free modules
const Token = struct {
    type: TokenType,
    value: []const u8,
    line: u32,
    column: u32,
};

const TokenType = enum {
    Identifier,
    StringLiteral, 
    NumberLiteral,
    Dot,
    LeftParen,
    RightParen,
    Comma,
    Semicolon,
    EOF,
};

fn tokenizeBasic(content: []const u8, tokens: *ArrayList(Token)) !void {
    var i: usize = 0;
    var line: u32 = 1;
    var column: u32 = 1;
    
    while (i < content.len) {
        const c = content[i];
        
        // Skip whitespace
        if (c == ' ' or c == '\t' or c == '\r') {
            i += 1;
            column += 1;
            continue;
        }
        
        if (c == '\n') {
            i += 1;
            line += 1;
            column = 1;
            continue;
        }
        
        // Handle string literals
        if (c == '"') {
            const start = i;
            i += 1; // Skip opening quote
            while (i < content.len and content[i] != '"') {
                i += 1;
            }
            if (i < content.len) i += 1; // Skip closing quote
            
            try tokens.append(Token{
                .type = .StringLiteral,
                .value = content[start..i],
                .line = line,
                .column = column,
            });
            column += @intCast(i - start);
            continue;
        }
        
        // Handle numbers
        if (std.ascii.isDigit(c)) {
            const start = i;
            while (i < content.len and (std.ascii.isDigit(content[i]) or content[i] == '.')) {
                i += 1;
            }
            
            try tokens.append(Token{
                .type = .NumberLiteral,
                .value = content[start..i],
                .line = line,
                .column = column,
            });
            column += @intCast(i - start);
            continue;
        }
        
        // Handle identifiers
        if (std.ascii.isAlphabetic(c) or c == '_') {
            const start = i;
            while (i < content.len and (std.ascii.isAlphanumeric(content[i]) or content[i] == '_')) {
                i += 1;
            }
            
            try tokens.append(Token{
                .type = .Identifier,
                .value = content[start..i],
                .line = line,
                .column = column,
            });
            column += @intCast(i - start);
            continue;
        }
        
        // Handle single-character tokens
        const token_type: TokenType = switch (c) {
            '.' => .Dot,
            '(' => .LeftParen,
            ')' => .RightParen,
            ',' => .Comma,
            ';' => .Semicolon,
            else => {
                i += 1;
                column += 1;
                continue; // Skip unknown characters
            },
        };
        
        try tokens.append(Token{
            .type = token_type,
            .value = content[i..i+1],
            .line = line,
            .column = column,
        });
        
        i += 1;
        column += 1;
    }
    
    try tokens.append(Token{
        .type = .EOF,
        .value = "",
        .line = line,
        .column = column,
    });
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.print("Usage: cursed <file.csd>\n", .{});
        return;
    }

    const filename = args[1];
    
    // Handle basic commands
    if (std.mem.eql(u8, filename, "--version")) {
        std.debug.print("CURSED Compiler v1.0.0 (Windows minimal build)\n", .{});
        return;
    }
    
    if (std.mem.eql(u8, filename, "--help")) {
        std.debug.print("CURSED Compiler - Windows Minimal Build\n", .{});
        std.debug.print("Usage: cursed <file.csd>\n", .{});
        std.debug.print("       cursed --version\n", .{});
        std.debug.print("       cursed --help\n", .{});
        std.debug.print("\nNote: This Windows build has limited functionality.\n", .{});
        return;
    }

    // Read and execute CURSED file
    const file_content = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        std.debug.print("Error reading file {s}: {s}\n", .{ filename, err });
        return;
    };
    defer allocator.free(file_content);

    // Basic lexer
    var tokens = std.ArrayList(u8){};
    defer tokens.deinit();
    
    tokenizeBasic(file_content, &tokens) catch |err| {
        std.debug.print("Lexing error: {s}\n", .{err});
        return;
    };

    // Basic interpreter
    interpretBasic(allocator, &tokens) catch |err| {
        std.debug.print("Interpretation error: {s}\n", .{err});
        return;
    };
}

fn interpretBasic(_: Allocator, tokens: *ArrayList(Token)) !void {
    var i: usize = 0;
    
    while (i < tokens.items.len) {
        const token = tokens.items[i];
        
        // Handle basic vibez.spill statements
        if (token.type == .Identifier and std.mem.eql(u8, token.value, "vibez")) {
            if (i + 4 < tokens.items.len and 
                tokens.items[i + 1].type == .Dot and
                tokens.items[i + 2].type == .Identifier and 
                std.mem.eql(u8, tokens.items[i + 2].value, "spill")) {
                
                i += 4; // Skip to opening paren
                
                // Print arguments
                while (i < tokens.items.len and tokens.items[i].type != .RightParen) {
                    const arg_token = tokens.items[i];
                    if (arg_token.type == .StringLiteral) {
                        // Remove quotes and print
                        var content = arg_token.value;
                        if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                            content = content[1..content.len - 1];
                        }
                        std.debug.print("{s}", .{content});
                    } else if (arg_token.type == .NumberLiteral) {
                        std.debug.print("{s}", .{arg_token.value});
                    } else if (arg_token.type == .Comma) {
                        std.debug.print(" ", .{});
                    }
                    i += 1;
                }
                std.debug.print("\n", .{});
            }
        }
        i += 1;
    }
}
