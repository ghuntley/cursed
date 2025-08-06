// WASM library build - exports functions properly
const std = @import("std");

// Simple allocator for WASM
var gpa = std.heap.GeneralPurposeAllocator(.{}){};

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

// Minimal lexer
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
    
    pub fn scanToken(self: *Self) TokenType {
        self.skipWhitespace();
        
        if (self.current >= self.source.len) {
            return .eof;
        }
        
        const c = self.advance();
        
        return switch (c) {
            'a'...'z', 'A'...'Z', '_' => self.scanIdentifierOrKeyword(),
            '0'...'9' => self.scanNumber(),
            '"' => self.scanString(),
            '(', ')', '{', '}', ',', ';', '.' => .delimiter,
            '+', '-', '*', '/', '=' => .operator,
            else => .invalid,
        };
    }
    
    fn advance(self: *Self) u8 {
        if (self.current >= self.source.len) return 0;
        const c = self.source[self.current];
        self.current += 1;
        return c;
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
        return .identifier;
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

// WASM exports
export fn cursed_wasm_tokenize(source_ptr: [*]const u8, source_len: usize) i32 {
    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(source);
    var token_count: i32 = 0;
    
    while (true) {
        const token = lexer.scanToken();
        if (token == .eof) break;
        token_count += 1;
        
        if (token_count > 10000) break; // Safety limit
    }
    
    return token_count;
}

export fn cursed_wasm_version() [*:0]const u8 {
    return "CURSED v1.0.0-wasm";
}

export fn cursed_wasm_check(source_ptr: [*]const u8, source_len: usize) i32 {
    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(source);
    var has_errors: i32 = 0;
    
    while (true) {
        const token = lexer.scanToken();
        if (token == .eof) break;
        if (token == .invalid) has_errors = 1;
    }
    
    return has_errors;
}

export fn cursed_wasm_test() i32 {
    const test_source = "vibez.spill(\"Hello WASM!\")";
    return cursed_wasm_tokenize(test_source.ptr, test_source.len);
}

// Memory management
export fn cursed_wasm_alloc(size: usize) ?[*]u8 {
    const allocator = gpa.allocator();
    const memory = allocator.alloc(u8, size) catch return null;
    return memory.ptr;
}

export fn cursed_wasm_free(ptr: [*]u8, size: usize) void {
    const allocator = gpa.allocator();
    const slice = ptr[0..size];
    allocator.free(slice);
}

// Initialize function
export fn cursed_wasm_init() i32 {
    return 0; // Success
}

// Cleanup function
export fn cursed_wasm_cleanup() void {
    _ = gpa.deinit();
}
