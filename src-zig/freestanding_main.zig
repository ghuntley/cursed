// CURSED Compiler for Freestanding/Embedded Targets
// No filesystem, threading, or OS-specific dependencies

const std = @import("std");
const builtin = @import("builtin");

// Simplified token types for freestanding
const TokenType = enum(u8) {
    Identifier,
    String, 
    Integer,
    Float,
    Keyword,
    Operator,
    Delimiter,
    EOF,
    Invalid,
};

const Token = struct {
    type: TokenType,
    value: []const u8,
    line: u32,
    column: u32,
};

// Freestanding lexer (no file I/O)
const FreestandingLexer = struct {
    source: []const u8,
    current: usize,
    line: u32,
    column: u32,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator, source: []const u8) @This() {
        return .{
            .source = source,
            .current = 0,
            .line = 1,
            .column = 1,
            .allocator = allocator,
        };
    }
    
    pub fn tokenize(self: *@This()) ![]Token {
        var tokens = std.ArrayList(Token){};
        defer tokens.deinit();
        
        while (!self.isAtEnd()) {
            const token = self.scanToken();
            try tokens.append(allocator, token);
            if (token.type == .EOF) break;
        }
        
        return tokens.toOwnedSlice();
    }
    
    fn scanToken(self: *@This()) Token {
        self.skipWhitespace();
        
        if (self.isAtEnd()) {
            return Token{
                .type = .EOF,
                .value = "",
                .line = self.line,
                .column = self.column,
            };
        }
        
        const start = self.current;
        const start_line = self.line;
        const start_column = self.column;
        const c = self.advance();
        
        return switch (c) {
            'a'...'z', 'A'...'Z', '_' => self.scanIdentifier(start, start_line, start_column),
            '0'...'9' => self.scanNumber(start, start_line, start_column),
            '"' => self.scanString(start, start_line, start_column),
            '(' => Token{ .type = .Delimiter, .value = self.source[start..self.current], .line = start_line, .column = start_column },
            ')' => Token{ .type = .Delimiter, .value = self.source[start..self.current], .line = start_line, .column = start_column },
            '{' => Token{ .type = .Delimiter, .value = self.source[start..self.current], .line = start_line, .column = start_column },
            '}' => Token{ .type = .Delimiter, .value = self.source[start..self.current], .line = start_line, .column = start_column },
            '+', '-', '*', '/', '=' => Token{ .type = .Operator, .value = self.source[start..self.current], .line = start_line, .column = start_column },
            else => Token{ .type = .Invalid, .value = self.source[start..self.current], .line = start_line, .column = start_column },
        };
    }
    
    fn advance(self: *@This()) u8 {
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
    
    fn isAtEnd(self: *@This()) bool {
        return self.current >= self.source.len;
    }
    
    fn skipWhitespace(self: *@This()) void {
        while (!self.isAtEnd()) {
            const c = self.source[self.current];
            if (c == ' ' or c == '\t' or c == '\r' or c == '\n') {
                _ = self.advance();
            } else {
                break;
            }
        }
    }
    
    fn scanIdentifier(self: *@This(), start: usize, start_line: u32, start_column: u32) Token {
        while (!self.isAtEnd()) {
            const c = self.source[self.current];
            if (std.ascii.isAlphanumeric(c) or c == '_') {
                _ = self.advance();
            } else {
                break;
            }
        }
        
        const value = self.source[start..self.current];
        const token_type = if (isKeyword(value)) TokenType.Keyword else TokenType.Identifier;
        return Token{ .type = token_type, .value = value, .line = start_line, .column = start_column };
    }
    
    fn scanNumber(self: *@This(), start: usize, start_line: u32, start_column: u32) Token {
        var has_dot = false;
        
        while (!self.isAtEnd()) {
            const c = self.source[self.current];
            if (std.ascii.isDigit(c)) {
                _ = self.advance();
            } else if (c == '.' and !has_dot) {
                has_dot = true;
                _ = self.advance();
            } else {
                break;
            }
        }
        
        const value = self.source[start..self.current];
        const token_type = if (has_dot) TokenType.Float else TokenType.Integer;
        return Token{ .type = token_type, .value = value, .line = start_line, .column = start_column };
    }
    
    fn scanString(self: *@This(), start: usize, start_line: u32, start_column: u32) Token {
        while (!self.isAtEnd() and self.source[self.current] != '"') {
            _ = self.advance();
        }
        
        if (!self.isAtEnd()) {
            _ = self.advance(); // consume closing quote
        }
        
        const value = self.source[start..self.current];
        return Token{ .type = .String, .value = value, .line = start_line, .column = start_column };
    }
};

fn isKeyword(text: []const u8) bool {
    const keywords = [_][]const u8{
        "sus", "vibez", "spill", "yeet", "damn", "slay", "ready",
        "bestie", "otherwise", "squad", "collab", "stan", "based", "cringe"
    };
    
    for (keywords) |keyword| {
        if (std.mem.eql(u8, text, keyword)) {
            return true;
        }
    }
    return false;
}

// Freestanding interpreter (no I/O)
const FreestandingInterpreter = struct {
    allocator: std.mem.Allocator,
    output: std.ArrayList(u8),
    
    pub fn init(allocator: std.mem.Allocator) @This() {
        return .{
            .allocator = allocator,
            .output = .{},
        };
    }
    
    pub fn deinit(self: *@This()) void {
        self.output.deinit(self.allocator);
    }
    
    pub fn execute(self: *@This(), tokens: []Token) !void {
        var i: usize = 0;
        while (i < tokens.len) {
            const token = tokens[i];
            
            if (token.type == .Identifier and std.mem.eql(u8, token.value, "vibez.spill")) {
                try self.executeVibezSpill(tokens, &i);
            } else {
                i += 1;
            }
        }
    }
    
    fn executeVibezSpill(self: *@This(), tokens: []Token, index: *usize) !void {
        index.* += 1; // Skip "vibez.spill"
        
        if (index.* < tokens.len and tokens[index.*].type == .Delimiter and 
            std.mem.eql(u8, tokens[index.*].value, "(")) {
            index.* += 1; // Skip "("
            
            while (index.* < tokens.len) {
                const token = tokens[index.*];
                
                if (token.type == .Delimiter and std.mem.eql(u8, token.value, ")")) {
                    index.* += 1; // Skip ")"
                    break;
                }
                
                if (token.type == .String) {
                    const str_content = if (token.value.len >= 2 and 
                                           token.value[0] == '"' and 
                                           token.value[token.value.len - 1] == '"')
                        token.value[1..token.value.len - 1]
                    else
                        token.value;
                    
                    try self.output.appendSlice(str_content);
                } else if (token.type == .Integer or token.type == .Float) {
                    try self.output.appendSlice(token.value);
                }
                
                index.* += 1;
            }
        }
    }
    
    pub fn getOutput(self: *@This()) []const u8 {
        return self.output.items;
    }
};

// Fixed-size buffer for embedded systems
var memory_buffer: [1024 * 64]u8 = undefined; // 64KB buffer
var fba: ?std.heap.FixedBufferAllocator = null;

// Global state for embedded/WASM exports
var global_interpreter: ?FreestandingInterpreter = null;

// Export functions for embedded systems
export fn cursed_init() i32 {
    fba = std.heap.FixedBufferAllocator.init(&memory_buffer);
    const allocator = fba.?.allocator();
    
    const interpreter = FreestandingInterpreter.init(allocator);
    global_interpreter = interpreter;
    
    return 0;
}

export fn cursed_deinit() void {
    if (global_interpreter) |*interpreter| {
        interpreter.deinit();
        global_interpreter = null;
    }
    fba = null;
}

export fn cursed_execute(source_ptr: [*]const u8, source_len: usize) i32 {
    if (fba == null or global_interpreter == null) {
        return -1;
    }
    
    const allocator = fba.?.allocator();
    var interpreter = &global_interpreter.?;
    const source = source_ptr[0..source_len];
    
    var lexer = FreestandingLexer.init(allocator, source);
    const tokens = lexer.tokenize() catch return -2;
    defer allocator.free(tokens);
    
    interpreter.execute(tokens) catch return -3;
    
    return 0;
}

export fn cursed_get_output(buffer_ptr: [*]u8, buffer_len: usize) i32 {
    if (global_interpreter == null) {
        return -1;
    }
    
    const interpreter = &global_interpreter.?;
    const output = interpreter.getOutput();
    
    if (output.len > buffer_len) {
        return @as(i32, @intCast(output.len));
    }
    
    @memcpy(buffer_ptr[0..output.len], output);
    return @as(i32, @intCast(output.len));
}

export fn cursed_version() [*:0]const u8 {
    return "CURSED v1.0.0-freestanding";
}

// Main function for testing
pub fn main() !void {
    // Test if not freestanding
    if (builtin.target.os.tag != .freestanding) {
        _ = cursed_init();
        defer cursed_deinit();
        
        const test_source = "vibez.spill(\"Hello Freestanding CURSED!\")";
        const result = cursed_execute(test_source.ptr, test_source.len);
        
        if (result == 0) {
            // Success - no output in freestanding mode
        }
    }
}
