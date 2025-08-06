// Minimal WASM-compatible CURSED compiler with zero dependencies
// Only uses core Zig features that work in WASM

// Simple token types for minimal lexing
const TokenType = enum(u8) {
    identifier = 0,
    string_literal = 1,
    integer_literal = 2,
    keyword = 3,
    operator = 4,
    delimiter = 5,
    eof = 6,
    invalid = 7,
};

// Minimal token structure
const Token = packed struct {
    type: TokenType,
    start: u16,
    length: u16,
};

// Minimal lexer that works entirely in WASM without allocations
const WasmLexer = struct {
    source: []const u8,
    current: usize,
    
    const Self = @This();
    
    pub fn init(source: []const u8) Self {
        return Self{
            .source = source,
            .current = 0,
        };
    }
    
    pub fn scanToken(self: *Self) Token {
        self.skipWhitespace();
        
        if (self.current >= self.source.len) {
            return Token{
                .type = .eof,
                .start = @intCast(self.current),
                .length = 0,
            };
        }
        
        const start = self.current;
        const c = self.advance();
        
        const token_type: TokenType = switch (c) {
            'a'...'z', 'A'...'Z', '_' => self.scanIdentifierOrKeyword(),
            '0'...'9' => self.scanNumber(),
            '"' => self.scanString(),
            '(', ')', '{', '}', ',', ';', '.' => .delimiter,
            '+', '-', '*', '/', '=' => .operator,
            else => .invalid,
        };
        
        return Token{
            .type = token_type,
            .start = @intCast(start),
            .length = @intCast(self.current - start),
        };
    }
    
    fn advance(self: *Self) u8 {
        if (self.current >= self.source.len) return 0;
        const c = self.source[self.current];
        self.current += 1;
        return c;
    }
    
    fn peek(self: *Self) u8 {
        if (self.current >= self.source.len) return 0;
        return self.source[self.current];
    }
    
    fn skipWhitespace(self: *Self) void {
        while (self.current < self.source.len) {
            const c = self.source[self.current];
            if (c == ' ' or c == '\t' or c == '\r' or c == '\n') {
                self.current += 1;
            } else {
                break;
            }
        }
    }
    
    fn scanIdentifierOrKeyword(self: *Self) TokenType {
        while (self.current < self.source.len) {
            const c = self.source[self.current];
            if ((c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z') or (c >= '0' and c <= '9') or c == '_') {
                self.current += 1;
            } else {
                break;
            }
        }
        
        // Simple keyword detection - could check for "slay", "sus", "vibez", etc.
        return .identifier; // Simplified - treat everything as identifier
    }
    
    fn scanNumber(self: *Self) TokenType {
        while (self.current < self.source.len) {
            const c = self.source[self.current];
            if (c >= '0' and c <= '9') {
                self.current += 1;
            } else {
                break;
            }
        }
        return .integer_literal;
    }
    
    fn scanString(self: *Self) TokenType {
        while (self.current < self.source.len and self.source[self.current] != '"') {
            self.current += 1;
        }
        
        if (self.current < self.source.len) {
            self.current += 1; // consume closing quote
        }
        
        return .string_literal;
    }
};

// WASM exports - minimal interface
export fn cursed_wasm_tokenize(source_ptr: [*]const u8, source_len: usize) i32 {
    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(source);
    var token_count: i32 = 0;
    
    // Count tokens without allocating memory
    while (true) {
        const token = lexer.scanToken();
        if (token.type == .eof) break;
        token_count += 1;
        
        // Safety limit to prevent infinite loops
        if (token_count > 10000) break;
    }
    
    return token_count;
}

export fn cursed_wasm_version() [*:0]const u8 {
    return "CURSED v1.0.0-wasm-minimal";
}

export fn cursed_wasm_check(source_ptr: [*]const u8, source_len: usize) i32 {
    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(source);
    var has_errors: i32 = 0;
    
    // Basic syntax check - count invalid tokens
    while (true) {
        const token = lexer.scanToken();
        if (token.type == .eof) break;
        if (token.type == .invalid) has_errors = 1;
    }
    
    return has_errors;
}

// Simple test function
export fn cursed_wasm_test() i32 {
    const test_source = "vibez.spill(\"Hello WASM!\")";
    return cursed_wasm_tokenize(test_source.ptr, test_source.len);
}

// Entry point (not used in WASM)
pub fn main() void {}
