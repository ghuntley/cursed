// Minimal CURSED WASM compiler - completely self-contained
// No external module dependencies that might pull in Thread/fs

const std = @import("std");
const builtin = @import("builtin");

// Only use WASM-compatible std features
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple token types for WASM
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
    
    pub fn toString(self: TokenType) []const u8 {
        return switch (self) {
            .Identifier => "identifier",
            .String => "string",
            .Integer => "integer", 
            .Float => "float",
            .Keyword => "keyword",
            .Operator => "operator",
            .Delimiter => "delimiter",
            .EOF => "EOF",
            .Invalid => "invalid",
        };
    }
};

// Minimal token structure
const Token = struct {
    type: TokenType,
    value: []const u8,
    line: u32,
    column: u32,
    
    pub fn init(token_type: TokenType, value: []const u8, line: u32, column: u32) Token {
        return Token{
            .type = token_type,
            .value = value,
            .line = line,
            .column = column,
        };
    }
};

// Minimal lexer for WASM
const WasmLexer = struct {
    source: []const u8,
    current: usize,
    line: u32,
    column: u32,
    allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, source: []const u8) Self {
        return Self{
            .source = source,
            .current = 0,
            .line = 1,
            .column = 1,
            .allocator = allocator,
        };
    }
    
    pub fn tokenize(self: *Self) ![]Token {
        var tokens = ArrayList(Token).init(self.allocator);
        
        while (!self.isAtEnd()) {
            const token = self.scanToken();
            if (token.type != .Invalid) {
                try tokens.append(token);
            }
            if (token.type == .EOF) break;
        }
        
        return tokens.toOwnedSlice();
    }
    
    fn scanToken(self: *Self) Token {
        self.skipWhitespace();
        
        if (self.isAtEnd()) {
            return Token.init(.EOF, "", self.line, self.column);
        }
        
        const start = self.current;
        const start_line = self.line;
        const start_column = self.column;
        const c = self.advance();
        
        return switch (c) {
            'a'...'z', 'A'...'Z', '_' => self.scanIdentifierOrKeyword(start, start_line, start_column),
            '0'...'9' => self.scanNumber(start, start_line, start_column),
            '"' => self.scanString(start, start_line, start_column),
            '(' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            ')' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            '{' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            '}' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            ',' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            ';' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            '.' => Token.init(.Delimiter, self.source[start..self.current], start_line, start_column),
            '+' => Token.init(.Operator, self.source[start..self.current], start_line, start_column),
            '-' => Token.init(.Operator, self.source[start..self.current], start_line, start_column),
            '*' => Token.init(.Operator, self.source[start..self.current], start_line, start_column),
            '/' => Token.init(.Operator, self.source[start..self.current], start_line, start_column),
            '=' => Token.init(.Operator, self.source[start..self.current], start_line, start_column),
            else => Token.init(.Invalid, self.source[start..self.current], start_line, start_column),
        };
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
    
    fn isAtEnd(self: *Self) bool {
        return self.current >= self.source.len;
    }
    
    fn skipWhitespace(self: *Self) void {
        while (!self.isAtEnd()) {
            const c = self.source[self.current];
            if (c == ' ' or c == '\t' or c == '\r' or c == '\n') {
                _ = self.advance();
            } else {
                break;
            }
        }
    }
    
    fn scanIdentifierOrKeyword(self: *Self, start: usize, start_line: u32, start_column: u32) Token {
        while (!self.isAtEnd()) {
            const c = self.source[self.current];
            if (std.ascii.isAlphanumeric(c) or c == '_' or c == '.') {
                _ = self.advance();
            } else {
                break;
            }
        }
        
        const value = self.source[start..self.current];
        const token_type = if (self.isKeyword(value)) TokenType.Keyword else TokenType.Identifier;
        return Token.init(token_type, value, start_line, start_column);
    }
    
    fn scanNumber(self: *Self, start: usize, start_line: u32, start_column: u32) Token {
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
        return Token.init(token_type, value, start_line, start_column);
    }
    
    fn scanString(self: *Self, start: usize, start_line: u32, start_column: u32) Token {
        while (!self.isAtEnd() and self.source[self.current] != '"') {
            _ = self.advance();
        }
        
        if (!self.isAtEnd()) {
            _ = self.advance(); // consume closing quote
        }
        
        const value = self.source[start..self.current];
        return Token.init(.String, value, start_line, start_column);
    }
    
    fn isKeyword(self: *Self, text: []const u8) bool {
        _ = self;
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
};

// Simple variable type for WASM
const WasmVariable = union(enum) {
    String: []const u8,
    Integer: i64,
    Float: f64,
    Boolean: bool,
    Null,
    
    pub fn toString(self: WasmVariable, allocator: Allocator) ![]const u8 {
        return switch (self) {
            .String => |str| try allocator.dupe(u8, str),
            .Integer => |val| try std.fmt.allocPrint(allocator, "{d}", .{val}),
            .Float => |val| try std.fmt.allocPrint(allocator, "{d}", .{val}),
            .Boolean => |val| try allocator.dupe(u8, if (val) "true" else "false"),
            .Null => try allocator.dupe(u8, "null"),
        };
    }
};

// WASM interpreter
const WasmInterpreter = struct {
    allocator: Allocator,
    output: ArrayList(u8),
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) Self {
        return Self{
            .allocator = allocator,
            .output = ArrayList(u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.output.deinit();
    }
    
    pub fn execute(self: *Self, tokens: []Token) !void {
        var i: usize = 0;
        while (i < tokens.len) {
            const token = tokens[i];
            
            // Simple pattern matching for basic CURSED syntax
            if (token.type == .Identifier and std.mem.eql(u8, token.value, "vibez.spill")) {
                try self.executeVibezSpill(tokens, &i);
            } else {
                i += 1;
            }
        }
    }
    
    fn executeVibezSpill(self: *Self, tokens: []Token, index: *usize) !void {
        index.* += 1; // Skip "vibez.spill"
        
        // Look for opening parenthesis
        if (index.* < tokens.len and tokens[index.*].type == .Delimiter and 
            std.mem.eql(u8, tokens[index.*].value, "(")) {
            index.* += 1; // Skip "("
            
            // Process arguments until closing parenthesis
            while (index.* < tokens.len) {
                const token = tokens[index.*];
                
                if (token.type == .Delimiter and std.mem.eql(u8, token.value, ")")) {
                    index.* += 1; // Skip ")"
                    try self.output.append('\n');
                    break;
                }
                
                if (token.type == .String) {
                    // Remove quotes from string
                    const str_content = if (token.value.len >= 2 and 
                                           token.value[0] == '"' and 
                                           token.value[token.value.len - 1] == '"')
                        token.value[1..token.value.len - 1]
                    else
                        token.value;
                    
                    try self.output.appendSlice(str_content);
                } else if (token.type == .Integer) {
                    try self.output.appendSlice(token.value);
                } else if (token.type == .Float) {
                    try self.output.appendSlice(token.value);
                } else if (token.type == .Identifier) {
                    // For simplicity, just output the identifier
                    try self.output.appendSlice(token.value);
                }
                
                index.* += 1;
                
                // Skip commas
                if (index.* < tokens.len and tokens[index.*].type == .Delimiter and 
                    std.mem.eql(u8, tokens[index.*].value, ",")) {
                    try self.output.appendSlice(" ");
                    index.* += 1;
                }
            }
        }
    }
    
    pub fn getOutput(self: *Self) []const u8 {
        return self.output.items;
    }
    
    pub fn clearOutput(self: *Self) void {
        self.output.clearRetainingCapacity();
    }
};

// Global state for WASM
var global_allocator: ?std.heap.ArenaAllocator = null;
var global_interpreter: ?WasmInterpreter = null;

// WASM exports
export fn wasm_init() i32 {
    const base_allocator = std.heap.page_allocator;
    
    // Initialize garbage collector for WASM target
    if (builtin.target.os.tag == .freestanding) {
        _ = @import("gc.zig").cursed_gc_init(1024 * 1024); // 1MB initial heap
    }
    
    var arena = std.heap.ArenaAllocator.init(base_allocator);
    global_allocator = arena;
    
    const interpreter = WasmInterpreter.init(arena.allocator());
    global_interpreter = interpreter;
    
    return 0; // Success
}

export fn wasm_deinit() void {
    if (global_interpreter) |*interpreter| {
        interpreter.deinit();
        global_interpreter = null;
    }
    if (global_allocator) |*arena| {
        arena.deinit();
        global_allocator = null;
    }
}

export fn wasm_execute_source(source_ptr: [*]const u8, source_len: usize) i32 {
    if (global_allocator == null or global_interpreter == null) {
        return -1; // Not initialized
    }
    
    const allocator = global_allocator.?.allocator();
    const interpreter = &global_interpreter.?;
    const source = source_ptr[0..source_len];
    
    // Tokenize
    var lexer = WasmLexer.init(allocator, source);
    const tokens = lexer.tokenize() catch return -2;
    defer allocator.free(tokens);
    
    // Execute
    interpreter.execute(tokens) catch return -3;
    
    return 0; // Success
}

export fn wasm_get_output(buffer_ptr: [*]u8, buffer_len: usize) i32 {
    if (global_interpreter == null) {
        return -1;
    }
    
    const interpreter = &global_interpreter.?;
    const output = interpreter.getOutput();
    
    if (output.len > buffer_len) {
        return @as(i32, @intCast(output.len)); // Return required size
    }
    
    @memcpy(buffer_ptr[0..output.len], output);
    return @as(i32, @intCast(output.len));
}

export fn wasm_clear_output() void {
    if (global_interpreter) |*interpreter| {
        interpreter.clearOutput();
    }
}

export fn wasm_tokenize(source_ptr: [*]const u8, source_len: usize) i32 {
    if (global_allocator == null) {
        return -1;
    }
    
    const allocator = global_allocator.?.allocator();
    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(allocator, source);
    const tokens = lexer.tokenize() catch return -2;
    defer allocator.free(tokens);
    
    return @as(i32, @intCast(tokens.len));
}

export fn wasm_check_syntax(source_ptr: [*]const u8, source_len: usize) i32 {
    if (global_allocator == null) {
        return -1;
    }
    
    const allocator = global_allocator.?.allocator();
    const source = source_ptr[0..source_len];
    
    var lexer = WasmLexer.init(allocator, source);
    const tokens = lexer.tokenize() catch return -2;
    defer allocator.free(tokens);
    
    // Basic syntax validation - check for balanced parentheses
    var paren_count: i32 = 0;
    for (tokens) |token| {
        if (token.type == .Delimiter) {
            if (std.mem.eql(u8, token.value, "(")) {
                paren_count += 1;
            } else if (std.mem.eql(u8, token.value, ")")) {
                paren_count -= 1;
                if (paren_count < 0) return -3; // Unbalanced
            }
        }
    }
    
    if (paren_count != 0) return -4; // Unbalanced
    return 0; // Success - syntax is valid
}

export fn wasm_version() [*:0]const u8 {
    return "CURSED v1.0.0-wasm-minimal";
}

// WASM memory management
export fn wasm_alloc(size: usize) ?[*]u8 {
    const allocator = std.heap.page_allocator;
    const memory = allocator.alloc(u8, size) catch return null;
    return memory.ptr;
}

export fn wasm_free(ptr: [*]u8, size: usize) void {
    const allocator = std.heap.page_allocator;
    const memory = ptr[0..size];
    allocator.free(memory);
}

// Main function for executable compatibility
pub fn main() !void {
    // For WASM, we don't need main() to do anything
    // All functionality is through exports
    
    // Test basic functionality if not in WASM environment
    if (builtin.target.os.tag != .freestanding) {
        // Initialize for testing
        _ = wasm_init();
        defer wasm_deinit();
        
        // Test basic functionality
        const test_source = "vibez.spill(\"Hello from WASM CURSED!\")";
        const result = wasm_execute_source(test_source.ptr, test_source.len);
        
        if (result == 0) {
            std.debug.print("WASM CURSED test: SUCCESS\n", .{});
            
            // Get and print output
            var output_buffer: [1024]u8 = undefined;
            const output_len = wasm_get_output(&output_buffer, output_buffer.len);
            if (output_len > 0) {
                std.debug.print("Output: {s}\n", .{output_buffer[0..@as(usize, @intCast(output_len))]});
            }
        } else {
            std.debug.print("WASM CURSED test: FAILED with code {d}\n", .{result});
        }
    }
}
