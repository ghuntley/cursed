// WASM-compatible implementation that completely avoids POSIX dependencies
const std = @import("std");
const builtin = @import("builtin");

// Completely standalone implementation - no imports that could pull in POSIX

// Basic lexer that works without filesystem dependencies
const TokenType = enum {
    // Literals
    identifier,
    string_literal,
    integer_literal,
    float_literal,
    
    // Keywords
    slay,        // function
    sus,         // variable
    vibez,       // namespace/module
    based,       // true
    cringe,      // false
    yeet,        // import
    damn,        // return
    
    // Operators
    plus,
    minus,
    star,
    slash,
    equal,
    
    // Delimiters
    left_paren,
    right_paren,
    left_brace,
    right_brace,
    comma,
    semicolon,
    dot,
    
    // Special
    eof,
    invalid,
};

const Token = struct {
    type: TokenType,
    lexeme: []const u8,
    line: u32,
    column: u32,
};

const WasmLexer = struct {
    source: []const u8,
    current: usize,
    line: u32,
    column: u32,
    allocator: std.mem.Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator, source: []const u8) Self {
        return Self{
            .source = source,
            .current = 0,
            .line = 1,
            .column = 1,
            .allocator = allocator,
        };
    }
    
    pub fn tokenize(self: *Self) !std.ArrayList(Token) {
        var tokens = std.ArrayList(Token).init(self.allocator);
        
        while (!self.isAtEnd()) {
            self.skipWhitespace();
            if (self.isAtEnd()) break;
            
            const start = self.current;
            const start_line = self.line;
            const start_column = self.column;
            
            const token_type = self.scanToken();
            const lexeme = self.source[start..self.current];
            
            try tokens.append(Token{
                .type = token_type,
                .lexeme = lexeme,
                .line = start_line,
                .column = start_column,
            });
        }
        
        try tokens.append(Token{
            .type = .eof,
            .lexeme = "",
            .line = self.line,
            .column = self.column,
        });
        
        return tokens;
    }
    
    fn isAtEnd(self: *Self) bool {
        return self.current >= self.source.len;
    }
    
    fn advance(self: *Self) u8 {
        if (self.isAtEnd()) return 0;
        const c = self.source[self.current];
        self.current += 1;
        if (c == '\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        return c;
    }
    
    fn peek(self: *Self) u8 {
        if (self.isAtEnd()) return 0;
        return self.source[self.current];
    }
    
    fn skipWhitespace(self: *Self) void {
        while (!self.isAtEnd()) {
            const c = self.peek();
            if (c == ' ' or c == '\t' or c == '\r' or c == '\n') {
                _ = self.advance();
            } else {
                break;
            }
        }
    }
    
    fn scanToken(self: *Self) TokenType {
        const c = self.advance();
        
        return switch (c) {
            '(' => .left_paren,
            ')' => .right_paren,
            '{' => .left_brace,
            '}' => .right_brace,
            ',' => .comma,
            ';' => .semicolon,
            '.' => .dot,
            '+' => .plus,
            '-' => .minus,
            '*' => .star,
            '/' => .slash,
            '=' => .equal,
            '"' => self.scanString(),
            else => {
                if (self.isDigit(c)) {
                    return self.scanNumber();
                } else if (self.isAlpha(c)) {
                    return self.scanIdentifier();
                } else {
                    return .invalid;
                }
            },
        };
    }
    
    fn scanString(self: *Self) TokenType {
        while (!self.isAtEnd() and self.peek() != '"') {
            _ = self.advance();
        }
        
        if (self.isAtEnd()) return .invalid;
        
        // Consume closing quote
        _ = self.advance();
        return .string_literal;
    }
    
    fn scanNumber(self: *Self) TokenType {
        while (self.isDigit(self.peek())) {
            _ = self.advance();
        }
        
        // Look for decimal point
        if (self.peek() == '.' and self.isDigit(self.peekNext())) {
            _ = self.advance(); // consume '.'
            while (self.isDigit(self.peek())) {
                _ = self.advance();
            }
            return .float_literal;
        }
        
        return .integer_literal;
    }
    
    fn scanIdentifier(self: *Self) TokenType {
        while (self.isAlphaNumeric(self.peek())) {
            _ = self.advance();
        }
        
        // Check for keywords
        const start = self.current - (self.current - self.findStart());
        const text = self.source[start..self.current];
        
        return self.getKeywordType(text);
    }
    
    fn findStart(self: *Self) usize {
        var i = self.current - 1;
        while (i > 0 and self.isAlphaNumeric(self.source[i - 1])) {
            i -= 1;
        }
        return i;
    }
    
    fn getKeywordType(self: *Self, text: []const u8) TokenType {
        _ = self;
        
        if (std.mem.eql(u8, text, "slay")) return .slay;
        if (std.mem.eql(u8, text, "sus")) return .sus;
        if (std.mem.eql(u8, text, "vibez")) return .vibez;
        if (std.mem.eql(u8, text, "based")) return .based;
        if (std.mem.eql(u8, text, "cringe")) return .cringe;
        if (std.mem.eql(u8, text, "yeet")) return .yeet;
        if (std.mem.eql(u8, text, "damn")) return .damn;
        
        return .identifier;
    }
    
    fn peekNext(self: *Self) u8 {
        if (self.current + 1 >= self.source.len) return 0;
        return self.source[self.current + 1];
    }
    
    fn isDigit(self: *Self, c: u8) bool {
        _ = self;
        return c >= '0' and c <= '9';
    }
    
    fn isAlpha(self: *Self, c: u8) bool {
        _ = self;
        return (c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z') or c == '_';
    }
    
    fn isAlphaNumeric(self: *Self, c: u8) bool {
        return self.isAlpha(c) or self.isDigit(c);
    }
};

// WASM exports with no dependencies
export fn cursed_wasm_compile(source_ptr: [*]const u8, source_len: usize) i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(allocator, source);
    const tokens = lexer.tokenize() catch return -1;
    defer tokens.deinit();

    return @intCast(tokens.items.len);
}

export fn cursed_wasm_version() [*:0]const u8 {
    return "CURSED v1.0.0-wasm-minimal";
}

export fn cursed_wasm_test() i32 {
    const test_source = "vibez.spill(\"Hello WASM!\")";
    return cursed_wasm_compile(test_source.ptr, test_source.len);
}

// Memory management for WASM host
export fn cursed_wasm_alloc(size: usize) ?[*]u8 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const memory = allocator.alloc(u8, size) catch return null;
    return memory.ptr;
}

export fn cursed_wasm_free(ptr: [*]u8, size: usize) void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const slice = ptr[0..size];
    allocator.free(slice);
}

// Entry point (won't be called in WASM)
pub fn main() void {}
