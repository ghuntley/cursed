const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const error_reporting = @import("enhanced_error_reporting.zig");
const ErrorReporter = error_reporting.ErrorReporter;
const ErrorCode = error_reporting.ErrorCode;
const SourceLocation = error_reporting.SourceLocation;

/// Enhanced lexer with comprehensive error reporting and recovery
pub const Token = struct {
    kind: TokenKind,
    lexeme: []const u8,
    location: SourceLocation,
    
    pub fn init(kind: TokenKind, lexeme: []const u8, location: SourceLocation) Token {
        return Token{
            .kind = kind,
            .lexeme = lexeme,
            .location = location,
        };
    }
};

pub const TokenKind = enum {
    // Literals
    Number,
    Integer,
    Float,
    StringLiteral,
    Character,
    Boolean,
    Based,  // true
    Cringe, // false
    
    // Identifiers
    Identifier,
    
    // CURSED Keywords
    Slay,     // function definition
    Sus,      // mutable variable
    Facts,    // immutable constant
    Lowkey,   // if statement
    Highkey,  // else statement
    Periodt,  // while loop
    Stan,     // goroutine
    Bestie,   // for loop
    Ghosted,  // break
    Simp,     // continue
    Squad,    // struct
    Collab,   // interface
    Flex,     // implementation
    Vibe,     // package
    Yeet,     // import
    As,       // alias keyword
    VibeCheck,// switch
    Mood,     // case
    Basic,    // default
    Match,    // match expression
    Yikes,    // error type
    Shook,    // error propagation
    Fam,      // error recovery
    Damn,     // return
    Vibes,    // break/continue context
    
    // Types
    Normie,   // i32
    Tea,      // string
    Lit,      // boolean
    Meal,     // f64
    Smol,     // i8
    Thicc,    // i64
    Spill,    // public modifier
    
    // Built-in functions and modules
    Vibez,    // print module identifier
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    ColonAssign,  // :=
    Arrow,        // ->
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Question,
    
    // Special
    Newline,
    Comment,
    EOF,
    Error,
    
    pub fn toString(self: TokenKind) []const u8 {
        return @tagName(self);
    }
    
    pub fn isKeyword(self: TokenKind) bool {
        return switch (self) {
            .Slay, .Sus, .Facts, .Lowkey, .Highkey, .Periodt, .Stan, .Bestie,
            .Ghosted, .Simp, .Squad, .Collab, .Flex, .Vibe, .Yeet, .VibeCheck,
            .Mood, .Basic, .Match, .Yikes, .Shook, .Fam, .Damn, .Vibes,
            .Normie, .Tea, .Lit, .Meal, .Smol, .Thicc, .Spill, .Based, .Cringe => true,
            else => false,
        };
    }
    
    pub fn isLiteral(self: TokenKind) bool {
        return switch (self) {
            .Number, .Integer, .Float, .StringLiteral, .Character, .Boolean, .Based, .Cringe => true,
            else => false,
        };
    }
    
    pub fn isOperator(self: TokenKind) bool {
        return switch (self) {
            .Plus, .Minus, .Star, .Slash, .Percent, .Equal, .NotEqual,
            .Less, .Greater, .LessEqual, .GreaterEqual, .And, .Or, .Not,
            .Assign, .PlusAssign, .MinusAssign, .StarAssign, .SlashAssign,
            .ColonAssign, .Arrow => true,
            else => false,
        };
    }
};

pub const Lexer = struct {
    source: []const u8,
    current: usize,
    line: u32,
    column: u32,
    char_offset: usize,
    file_path: []const u8,
    allocator: Allocator,
    error_reporter: *ErrorReporter,
    tokens: ArrayList(Token),
    
    // Keyword mapping
    keywords: std.HashMap([]const u8, TokenKind, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, source: []const u8, file_path: []const u8, error_reporter: *ErrorReporter) !Lexer {
        var lexer = Lexer{
            .source = source,
            .current = 0,
            .line = 1,
            .column = 1,
            .char_offset = 0,
            .file_path = file_path,
            .allocator = allocator,
            .error_reporter = error_reporter,
            .tokens = .empty,
            .keywords = std.HashMap([]const u8, TokenKind, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
        
        try lexer.initKeywords();
        return lexer;
    }
    
    pub fn deinit(self: *Lexer) void {
        self.tokens.deinit(self.allocator);
        self.keywords.deinit(self.allocator);
    }
    
    fn initKeywords(self: *Lexer) !void {
        // CURSED keywords
        try self.keywords.put("slay", .Slay);
        try self.keywords.put("sus", .Sus);
        try self.keywords.put("facts", .Facts);
        try self.keywords.put("lowkey", .Lowkey);
        try self.keywords.put("highkey", .Highkey);
        try self.keywords.put("periodt", .Periodt);
        try self.keywords.put("stan", .Stan);
        try self.keywords.put("bestie", .Bestie);
        try self.keywords.put("ghosted", .Ghosted);
        try self.keywords.put("simp", .Simp);
        try self.keywords.put("squad", .Squad);
        try self.keywords.put("collab", .Collab);
        try self.keywords.put("flex", .Flex);
        try self.keywords.put("vibe", .Vibe);
        try self.keywords.put("yeet", .Yeet);
        try self.keywords.put("as", .As);
        try self.keywords.put("vibecheck", .VibeCheck);
        try self.keywords.put("mood", .Mood);
        try self.keywords.put("basic", .Basic);
        try self.keywords.put("match", .Match);
        try self.keywords.put("yikes", .Yikes);
        try self.keywords.put("shook", .Shook);
        try self.keywords.put("fam", .Fam);
        try self.keywords.put("damn", .Damn);
        try self.keywords.put("vibes", .Vibes);
        
        // Types
        try self.keywords.put("normie", .Normie);
        try self.keywords.put("tea", .Tea);
        try self.keywords.put("lit", .Lit);
        try self.keywords.put("meal", .Meal);
        try self.keywords.put("smol", .Smol);
        try self.keywords.put("thicc", .Thicc);
        try self.keywords.put("spill", .Spill);
        
        // Boolean literals
        try self.keywords.put("based", .Based);
        try self.keywords.put("cringe", .Cringe);
        
        // Built-in modules
        try self.keywords.put("vibez", .Vibez);
    }
    
    pub fn tokenize(self: *Lexer) ![]Token {
        while (!self.isAtEnd()) {
            try self.scanToken();
        }
        
        // Add EOF token
        const eof_location = SourceLocation.init(self.file_path, self.line, self.column, @intCast(self.char_offset));
        try self.tokens.append(Token.init(.EOF, "", eof_location));
        
        return self.tokens.toOwnedSlice();
    }
    
    fn scanToken(self: *Lexer) !void {
        const start_column = self.column;
        const start_offset = self.char_offset;
        const char = self.advance();
        
        switch (char) {
            ' ', '\r', '\t' => {
                // Skip whitespace
            },
            '\n' => {
                const location = SourceLocation.init(self.file_path, self.line, start_column, @intCast(start_offset));
                try self.tokens.append(Token.init(.Newline, self.source[start_offset..self.char_offset], location));
                self.line += 1;
                self.column = 1;
            },
            
            // Single character tokens
            '(' => try self.addToken(.LeftParen, start_column, start_offset),
            ')' => try self.addToken(.RightParen, start_column, start_offset),
            '{' => try self.addToken(.LeftBrace, start_column, start_offset),
            '}' => try self.addToken(.RightBrace, start_column, start_offset),
            '[' => try self.addToken(.LeftBracket, start_column, start_offset),
            ']' => try self.addToken(.RightBracket, start_column, start_offset),
            ',' => try self.addToken(.Comma, start_column, start_offset),
            '.' => try self.addToken(.Dot, start_column, start_offset),
            ';' => try self.addToken(.Semicolon, start_column, start_offset),
            '?' => try self.addToken(.Question, start_column, start_offset),
            
            // Potentially multi-character tokens
            '+' => {
                if (self.match('=')) {
                    try self.addToken(.PlusAssign, start_column, start_offset);
                } else {
                    try self.addToken(.Plus, start_column, start_offset);
                }
            },
            '-' => {
                if (self.match('=')) {
                    try self.addToken(.MinusAssign, start_column, start_offset);
                } else if (self.match('>')) {
                    try self.addToken(.Arrow, start_column, start_offset);
                } else {
                    try self.addToken(.Minus, start_column, start_offset);
                }
            },
            '*' => {
                if (self.match('=')) {
                    try self.addToken(.StarAssign, start_column, start_offset);
                } else {
                    try self.addToken(.Star, start_column, start_offset);
                }
            },
            '/' => {
                if (self.match('=')) {
                    try self.addToken(.SlashAssign, start_column, start_offset);
                } else if (self.match('/')) {
                    try self.lineComment();
                } else if (self.match('*')) {
                    try self.blockComment(start_column, start_offset);
                } else {
                    try self.addToken(.Slash, start_column, start_offset);
                }
            },
            '%' => try self.addToken(.Percent, start_column, start_offset),
            '!' => {
                if (self.match('=')) {
                    try self.addToken(.NotEqual, start_column, start_offset);
                } else {
                    try self.addToken(.Not, start_column, start_offset);
                }
            },
            '=' => {
                if (self.match('=')) {
                    try self.addToken(.Equal, start_column, start_offset);
                } else {
                    try self.addToken(.Assign, start_column, start_offset);
                }
            },
            '<' => {
                if (self.match('=')) {
                    try self.addToken(.LessEqual, start_column, start_offset);
                } else {
                    try self.addToken(.Less, start_column, start_offset);
                }
            },
            '>' => {
                if (self.match('=')) {
                    try self.addToken(.GreaterEqual, start_column, start_offset);
                } else {
                    try self.addToken(.Greater, start_column, start_offset);
                }
            },
            '&' => {
                if (self.match('&')) {
                    try self.addToken(.And, start_column, start_offset);
                } else {
                    try self.reportError(.E002_InvalidCharacter, "Unexpected character '&'. Did you mean '&&'?", start_column, start_offset);
                }
            },
            '|' => {
                if (self.match('|')) {
                    try self.addToken(.Or, start_column, start_offset);
                } else {
                    try self.reportError(.E002_InvalidCharacter, "Unexpected character '|'. Did you mean '||'?", start_column, start_offset);
                }
            },
            ':' => {
                if (self.match('=')) {
                    try self.addToken(.ColonAssign, start_column, start_offset);
                } else {
                    try self.addToken(.Colon, start_column, start_offset);
                }
            },
            
            // String literals
            '"' => try self.string(start_column, start_offset),
            '\'' => try self.character(start_column, start_offset),
            
            else => {
                if (std.ascii.isDigit(char)) {
                    try self.number(start_column, start_offset);
                } else if (std.ascii.isAlphabetic(char) or char == '_') {
                    try self.identifier(start_column, start_offset);
                } else {
                    const error_msg = try std.fmt.allocPrint(self.allocator, "Unexpected character '{c}' (ASCII: {d})", .{ char, char });
                    defer self.allocator.free(error_msg);
                    try self.reportError(.E002_InvalidCharacter, error_msg, start_column, start_offset);
                }
            },
        }
    }
    
    fn addToken(self: *Lexer, kind: TokenKind, start_column: u32, start_offset: usize) !void {
        const lexeme = self.source[start_offset..self.char_offset];
        const location = SourceLocation.init(self.file_path, self.line, start_column, @intCast(start_offset));
        try self.tokens.append(self.allocator, Token.init(kind, lexeme, location));
    }
    
    fn string(self: *Lexer, start_column: u32, start_offset: usize) !void {
        const start_line = self.line;
        
        while (!self.isAtEnd() and self.peek() != '"') {
            if (self.peek() == '\n') {
                if (self.line < std.math.maxInt(u32)) {
                    self.line += 1;
                }
                self.column = 1;
            } else if (self.peek() == '\\') {
                // Handle escape sequences
                _ = self.advance(); // consume backslash
                if (!self.isAtEnd()) {
                    const escaped = self.peek();
                    switch (escaped) {
                        'n', 't', 'r', '\\', '"', '\'' => {
                            _ = self.advance(); // consume escaped character
                        },
                        'x' => {
                            // Hex escape \xFF
                            _ = self.advance(); // consume 'x'
                            if (!std.ascii.isHex(self.peek()) or self.isAtEnd()) {
                                try self.reportError(.E005_InvalidEscape, "Invalid hex escape sequence. Expected \\xFF format", start_column, start_offset);
                                return;
                            }
                            _ = self.advance();
                            if (!std.ascii.isHex(self.peek()) or self.isAtEnd()) {
                                try self.reportError(.E005_InvalidEscape, "Invalid hex escape sequence. Expected \\xFF format", start_column, start_offset);
                                return;
                            }
                            _ = self.advance();
                        },
                        'u' => {
                            // Unicode escape \u1234
                            _ = self.advance(); // consume 'u'
                            var i: u8 = 0;
                            while (i < 4) : (i += 1) {
                                if (!std.ascii.isHex(self.peek()) or self.isAtEnd()) {
                                    try self.reportError(.E005_InvalidEscape, "Invalid unicode escape sequence. Expected \\u1234 format", start_column, start_offset);
                                    return;
                                }
                                _ = self.advance();
                            }
                        },
                        else => {
                            const error_msg = try std.fmt.allocPrint(self.allocator, "Invalid escape sequence '\\{c}'. Valid escapes: \\n, \\t, \\r, \\\\, \\\", \\', \\xFF, \\u1234", .{escaped});
                            defer self.allocator.free(error_msg);
                            try self.reportError(.E005_InvalidEscape, error_msg, start_column, start_offset);
                        },
                    }
                } else {
                    try self.reportError(.E005_InvalidEscape, "Unterminated escape sequence at end of file", start_column, start_offset);
                }
            } else {
                _ = self.advance();
            }
        }
        
        if (self.isAtEnd()) {
            const error_msg = if (start_line == self.line) 
                "Unterminated string literal on same line"
            else 
                try std.fmt.allocPrint(self.allocator, "Unterminated string literal started on line {}", .{start_line});
            defer if (start_line != self.line) self.allocator.free(error_msg);
            
            try self.reportError(.E001_UnterminatedString, error_msg, start_column, start_offset);
            return;
        }
        
        // Consume closing quote
        _ = self.advance();
        
        try self.addToken(.StringLiteral, start_column, start_offset);
    }
    
    fn character(self: *Lexer, start_column: u32, start_offset: usize) !void {
        if (self.isAtEnd()) {
            try self.reportError(.E001_UnterminatedString, "Unterminated character literal", start_column, start_offset);
            return;
        }
        
        if (self.peek() == '\\') {
            // Escape sequence
            _ = self.advance(); // consume backslash
            if (self.isAtEnd()) {
                try self.reportError(.E005_InvalidEscape, "Unterminated escape sequence in character literal", start_column, start_offset);
                return;
            }
            _ = self.advance(); // consume escaped character
        } else {
            _ = self.advance(); // consume regular character
        }
        
        if (self.isAtEnd() or self.peek() != '\'') {
            try self.reportError(.E001_UnterminatedString, "Unterminated character literal. Expected closing '", start_column, start_offset);
            return;
        }
        
        // Consume closing quote
        _ = self.advance();
        
        try self.addToken(.Character, start_column, start_offset);
    }
    
    fn number(self: *Lexer, start_column: u32, start_offset: usize) !void {
        var is_float = false;
        
        // Consume digits
        while (std.ascii.isDigit(self.peek())) {
            _ = self.advance();
        }
        
        // Look for decimal point
        if (self.peek() == '.' and std.ascii.isDigit(self.peekNext())) {
            is_float = true;
            _ = self.advance(); // consume '.'
            
            while (std.ascii.isDigit(self.peek())) {
                _ = self.advance();
            }
        }
        
        // Look for exponent
        if (self.peek() == 'e' or self.peek() == 'E') {
            is_float = true;
            _ = self.advance(); // consume 'e'/'E'
            
            if (self.peek() == '+' or self.peek() == '-') {
                _ = self.advance(); // consume sign
            }
            
            if (!std.ascii.isDigit(self.peek())) {
                try self.reportError(.E003_InvalidNumber, "Invalid number format: expected digits after exponent", start_column, start_offset);
                return;
            }
            
            while (std.ascii.isDigit(self.peek())) {
                _ = self.advance();
            }
        }
        
        // Check for invalid suffix
        if (std.ascii.isAlphabetic(self.peek()) or self.peek() == '_') {
            try self.reportError(.E003_InvalidNumber, "Invalid number format: unexpected character after number", start_column, start_offset);
            return;
        }
        
        const token_kind = if (is_float) TokenKind.Float else TokenKind.Integer;
        try self.addToken(token_kind, start_column, start_offset);
    }
    
    fn identifier(self: *Lexer, start_column: u32, start_offset: usize) !void {
        while (std.ascii.isAlphanumeric(self.peek()) or self.peek() == '_') {
            _ = self.advance();
        }
        
        const text = self.source[start_offset..self.char_offset];
        const token_kind = self.keywords.get(text) orelse .Identifier;
        
        try self.addToken(token_kind, start_column, start_offset);
    }
    
    fn lineComment(self: *Lexer) !void {
        // Consume until end of line
        while (!self.isAtEnd() and self.peek() != '\n') {
            _ = self.advance();
        }
    }
    
    fn blockComment(self: *Lexer, start_column: u32, start_offset: usize) !void {
        const start_line = self.line;
        var nesting_level: u32 = 1;
        
        while (!self.isAtEnd() and nesting_level > 0) {
            if (self.peek() == '/' and self.peekNext() == '*') {
                _ = self.advance(); // consume '/'
                _ = self.advance(); // consume '*'
                nesting_level += 1;
            } else if (self.peek() == '*' and self.peekNext() == '/') {
                _ = self.advance(); // consume '*'
                _ = self.advance(); // consume '/'
                nesting_level -= 1;
            } else if (self.peek() == '\n') {
                self.line += 1;
                self.column = 1;
                _ = self.advance();
            } else {
                _ = self.advance();
            }
        }
        
        if (nesting_level > 0) {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Unterminated block comment started on line {}", .{start_line});
            defer self.allocator.free(error_msg);
            try self.reportError(.E004_UnterminatedComment, error_msg, start_column, start_offset);
        }
    }
    
    fn reportError(self: *Lexer, code: ErrorCode, message: []const u8, start_column: u32, start_offset: usize) !void {
        const location = SourceLocation.init(self.file_path, self.line, start_column, @intCast(start_offset));
        try self.error_reporter.reportError(code, message, location);
        
        // Add error token for recovery
        try self.addToken(.Error, start_column, start_offset);
    }
    
    // Helper methods
    fn isAtEnd(self: *Lexer) bool {
        return self.current >= self.source.len;
    }
    
    fn advance(self: *Lexer) u8 {
        if (self.isAtEnd()) return 0;
        
        const char = self.source[self.current];
        self.current += 1;
        self.char_offset += 1;
        self.column += 1;
        
        return char;
    }
    
    fn match(self: *Lexer, expected: u8) bool {
        if (self.isAtEnd() or self.source[self.current] != expected) {
            return false;
        }
        
        self.current += 1;
        self.char_offset += 1;
        self.column += 1;
        return true;
    }
    
    fn peek(self: *Lexer) u8 {
        if (self.isAtEnd()) return 0;
        return self.source[self.current];
    }
    
    fn peekNext(self: *Lexer) u8 {
        if (self.current + 1 >= self.source.len) return 0;
        return self.source[self.current + 1];
    }
};

// Testing
test "enhanced lexer with error reporting" {
    const allocator = std.testing.allocator;
    
    var error_reporter = ErrorReporter.init(allocator, 10);
    defer error_reporter.deinit();
    
    // Test valid tokens
    const source = "slay main() { sus x normie = 42; vibez.spill(\"Hello!\"); }";
    var lexer = try Lexer.init(allocator, source, "test.csd", &error_reporter);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenize();
    defer allocator.free(tokens);
    
    try std.testing.expect(tokens.len > 0);
    try std.testing.expect(tokens[0].kind == .Slay);
    try std.testing.expect(!error_reporter.hasErrors());
    
    // Test error recovery
    const invalid_source = "slay main() { sus x = \"unterminated string; }";
    var error_lexer = try Lexer.init(allocator, invalid_source, "error_test.csd", &error_reporter);
    defer error_lexer.deinit();
    
    const error_tokens = try error_lexer.tokenize();
    defer allocator.free(error_tokens);
    
    try std.testing.expect(error_reporter.hasErrors());
}
