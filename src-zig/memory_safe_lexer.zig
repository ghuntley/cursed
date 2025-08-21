const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

const lexer_old = @import("lexer.zig");
pub const TokenKind = lexer_old.TokenKind;

/// Memory-safe Token with arena-managed strings
pub const Token = struct {
    kind: TokenKind,
    lexeme: []const u8,
    line: usize,
    column: usize,

    pub fn init(kind: TokenKind, lexeme: []const u8, line: usize, column: usize) Token {
        return Token{
            .kind = kind,
            .lexeme = lexeme,
            .line = line,
            .column = column,
        };
    }

    pub fn format(self: Token, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        try writer.print("Token{{.kind = {}, .lexeme = \"{s}\", .line = {}, .column = {}}}", .{ self.kind, self.lexeme, self.line, self.column });
    }
};

/// Memory-safe token collection with automatic cleanup
pub const TokenCollection = struct {
    tokens: ArrayList(Token),
    arena: ArenaAllocator,
    
    pub fn init(backing_allocator: Allocator) TokenCollection {
        var arena = ArenaAllocator.init(backing_allocator);
        const arena_allocator = arena.allocator();
        
        return TokenCollection{
            .tokens = .empty,
            .arena = arena,
        };
    }
    
    pub fn deinit(self: *TokenCollection) void {
        // Arena automatically cleans up all allocated memory including token lexemes
        self.arena.deinit();
    }
    
    pub fn append(self: *TokenCollection, token: Token) !void {
        try self.tokens.append(token);
    }
    
    pub fn toSlice(self: *TokenCollection) []const Token {
        return self.tokens.items;
    }
    
    pub fn items(self: *TokenCollection) []const Token {
        return self.tokens.items;
    }
};

/// Memory-safe lexer using arena allocator for token storage
pub const Lexer = struct {
    input: []const u8,
    position: usize,
    line: usize,
    column: usize,
    arena: *ArenaAllocator,

    pub fn init(arena: *ArenaAllocator, input: []const u8) Lexer {
        return Lexer{
            .input = input,
            .position = 0,
            .line = 1,
            .column = 1,
            .arena = arena,
        };
    }

    pub fn tokenize(self: *Lexer) !TokenCollection {
        var tokens = TokenCollection.init(self.arena.child_allocator);
        
        while (!self.isAtEnd()) {
            const token = try self.nextToken();
            // Skip comments and newlines like the Rust version
            if (token.kind != .Newline and token.kind != .LineComment and token.kind != .BlockComment) {
                try tokens.append(token);
            }
            if (token.kind == .Eof) break;
        }
        
        return tokens;
    }

    pub fn nextToken(self: *Lexer) !Token {
        self.skipWhitespace();

        if (self.isAtEnd()) {
            return Token.init(.Eof, "", self.line, self.column);
        }

        const c = self.advance();
        const start_line = self.line;
        const start_column = self.column - 1;

        switch (c) {
            '(' => return self.makeToken(.LeftParen, start_line, start_column),
            ')' => return self.makeToken(.RightParen, start_line, start_column),
            '{' => return self.makeToken(.LeftBrace, start_line, start_column),
            '}' => return self.makeToken(.RightBrace, start_line, start_column),
            '[' => return self.makeToken(.LeftBracket, start_line, start_column),
            ']' => return self.makeToken(.RightBracket, start_line, start_column),
            ',' => return self.makeToken(.Comma, start_line, start_column),
            ';' => return self.makeToken(.Semicolon, start_line, start_column),
            '@' => return self.makeToken(.At, start_line, start_column),
            '#' => {
                // Hash character - check if it's a line comment
                if (self.peek() == ' ' or self.peek() == '\t' or std.ascii.isAlphabetic(self.peek())) {
                    // Treat as line comment - consume until end of line
                    while (self.peek() != '\n' and !self.isAtEnd()) {
                        _ = self.advance();
                    }
                    return self.makeToken(.LineComment, start_line, start_column);
                } else {
                    // Standalone # token for other uses (preprocessor directives, etc.)
                    return self.makeToken(.Hash, start_line, start_column);
                }
            },
            '?' => return self.makeToken(.Question, start_line, start_column),
            '\n' => {
                self.line += 1;
                self.column = 1;
                return self.makeToken(.Newline, start_line, start_column);
            },

            '+' => {
                if (self.match('+')) return self.makeToken(.PlusPlus, start_line, start_column);
                if (self.match('=')) return self.makeToken(.PlusEqual, start_line, start_column);
                return self.makeToken(.Plus, start_line, start_column);
            },
            '-' => {
                if (self.match('-')) return self.makeToken(.MinusMinus, start_line, start_column);
                if (self.match('=')) return self.makeToken(.MinusEqual, start_line, start_column);
                if (self.match('>')) return self.makeToken(.Arrow, start_line, start_column);
                return self.makeToken(.Minus, start_line, start_column);
            },
            '*' => {
                if (self.match('=')) return self.makeToken(.StarEqual, start_line, start_column);
                return self.makeToken(.Star, start_line, start_column);
            },
            '/' => {
                if (self.match('=')) return self.makeToken(.SlashEqual, start_line, start_column);
                if (self.match('/')) {
                    // Line comment - consume until end of line
                    while (self.peek() != '\n' and !self.isAtEnd()) {
                        _ = self.advance();
                    }
                    return self.makeToken(.LineComment, start_line, start_column);
                }
                return self.makeToken(.Slash, start_line, start_column);
            },
            '%' => {
                if (self.match('=')) return self.makeToken(.PercentEqual, start_line, start_column);
                return self.makeToken(.Percent, start_line, start_column);
            },
            '=' => {
                if (self.match('=')) return self.makeToken(.EqualEqual, start_line, start_column);
                return self.makeToken(.Equal, start_line, start_column);
            },
            '!' => {
                if (self.match('=')) return self.makeToken(.BangEqual, start_line, start_column);
                return self.makeToken(.Bang, start_line, start_column);
            },
            '<' => {
                if (self.match('=')) return self.makeToken(.LessEqual, start_line, start_column);
                if (self.match('<')) return self.makeToken(.LeftShift, start_line, start_column);
                if (self.match('-')) return self.makeToken(.LeftArrow, start_line, start_column);
                return self.makeToken(.Less, start_line, start_column);
            },
            '>' => {
                if (self.match('=')) return self.makeToken(.GreaterEqual, start_line, start_column);
                if (self.match('>')) return self.makeToken(.RightShift, start_line, start_column);
                return self.makeToken(.Greater, start_line, start_column);
            },
            '&' => {
                if (self.match('&')) return self.makeToken(.AmpAmp, start_line, start_column);
                return self.makeToken(.Amp, start_line, start_column);
            },
            '|' => {
                if (self.match('|')) return self.makeToken(.PipePipe, start_line, start_column);
                return self.makeToken(.Pipe, start_line, start_column);
            },
            '^' => return self.makeToken(.Caret, start_line, start_column),
            ':' => {
                if (self.match('=')) return self.makeToken(.ColonEqual, start_line, start_column);
                if (self.match(':')) return self.makeToken(.DoubleColon, start_line, start_column);
                return self.makeToken(.Colon, start_line, start_column);
            },
            '.' => {
                if (self.match('.')) {
                    if (self.match('.')) return self.makeToken(.DotDotDot, start_line, start_column);
                    return self.makeToken(.DotDot, start_line, start_column);
                }
                return self.makeToken(.Dot, start_line, start_column);
            },

            '"' => return self.stringLiteral(start_line, start_column),
            '\'' => return self.charLiteral(start_line, start_column),

            else => {
                if (std.ascii.isDigit(c)) {
                    self.position -= 1; // Back up to re-read the digit
                    self.column -= 1;
                    return self.number(start_line, start_column);
                }
                if (std.ascii.isAlphabetic(c) or c == '_') {
                    self.position -= 1; // Back up to re-read the character
                    self.column -= 1;
                    return self.identifier(start_line, start_column);
                }
                return error.UnexpectedCharacter;
            },
        }
    }

    fn isAtEnd(self: *Lexer) bool {
        return self.position >= self.input.len;
    }

    fn advance(self: *Lexer) u8 {
        if (self.isAtEnd()) return 0;
        const c = self.input[self.position];
        self.position += 1;
        self.column += 1;
        return c;
    }

    fn peek(self: *Lexer) u8 {
        if (self.isAtEnd()) return 0;
        return self.input[self.position];
    }

    fn peekNext(self: *Lexer) u8 {
        if (self.position + 1 >= self.input.len) return 0;
        return self.input[self.position + 1];
    }
    
    // SECURITY FIX: Safe peek ahead function with bounds checking
    fn safePeekAhead(self: *Lexer, offset: usize) u8 {
        if (self.position + offset >= self.input.len) return 0;
        return self.input[self.position + offset];
    }

    fn match(self: *Lexer, expected: u8) bool {
        if (self.isAtEnd()) return false;
        if (self.input[self.position] != expected) return false;
        
        self.position += 1;
        self.column += 1;
        return true;
    }

    fn skipWhitespace(self: *Lexer) void {
        while (!self.isAtEnd()) {
            const c = self.peek();
            switch (c) {
                ' ', '\r', '\t' => {
                    _ = self.advance();
                },
                else => break,
            }
        }
    }

    fn makeToken(self: *Lexer, kind: TokenKind, line: usize, column: usize) !Token {
        const start = if (self.position > 0) self.position - 1 else 0;
        const lexeme_slice = self.input[start..self.position];
        
        // Use arena allocator to store lexeme copy
        const arena_allocator = self.arena.allocator();
        const lexeme = try arena_allocator.dupe(u8, lexeme_slice);
        
        return Token.init(kind, lexeme, line, column);
    }

    fn stringLiteral(self: *Lexer, line: usize, column: usize) !Token {
        const start = self.position - 1; // Include opening quote
        
        while (self.peek() != '"' and !self.isAtEnd()) {
            if (self.peek() == '\n') {
                self.line += 1;
                self.column = 0; // Will be incremented by advance()
            }
            if (self.peek() == '\\') {
                _ = self.advance(); // Skip escape character
                if (!self.isAtEnd()) _ = self.advance(); // Skip escaped character
            } else {
                _ = self.advance();
            }
        }

        if (self.isAtEnd()) return error.UnterminatedString;

        // Consume closing quote
        _ = self.advance();

        const lexeme_slice = self.input[start..self.position];
        
        // Use arena allocator to store lexeme copy
        const arena_allocator = self.arena.allocator();
        const lexeme = try arena_allocator.dupe(u8, lexeme_slice);
        
        return Token.init(.StringLiteral, lexeme, line, column);
    }

    fn charLiteral(self: *Lexer, line: usize, column: usize) !Token {
        const start = self.position - 1; // Include opening quote
        
        if (self.peek() == '\\') {
            _ = self.advance(); // Skip escape character
            if (!self.isAtEnd()) _ = self.advance(); // Skip escaped character
        } else if (!self.isAtEnd()) {
            _ = self.advance(); // Single character
        }

        if (self.isAtEnd() or self.peek() != '\'') return error.UnterminatedChar;

        // Consume closing quote
        _ = self.advance();

        const lexeme_slice = self.input[start..self.position];
        
        // Use arena allocator to store lexeme copy
        const arena_allocator = self.arena.allocator();
        const lexeme = try arena_allocator.dupe(u8, lexeme_slice);
        
        return Token.init(.Character, lexeme, line, column);
    }

    fn number(self: *Lexer, line: usize, column: usize) !Token {
        const start = self.position;
        
        while (std.ascii.isDigit(self.peek())) {
            _ = self.advance();
        }

        // Look for decimal point
        if (self.peek() == '.' and std.ascii.isDigit(self.peekNext())) {
            _ = self.advance(); // Consume '.'
            while (std.ascii.isDigit(self.peek())) {
                _ = self.advance();
            }
        }

        const lexeme_slice = self.input[start..self.position];
        
        // Use arena allocator to store lexeme copy
        const arena_allocator = self.arena.allocator();
        const lexeme = try arena_allocator.dupe(u8, lexeme_slice);
        
        return Token.init(.Number, lexeme, line, column);
    }

    fn identifier(self: *Lexer, line: usize, column: usize) !Token {
        const start = self.position;
        
        while (std.ascii.isAlphanumeric(self.peek()) or self.peek() == '_') {
            _ = self.advance();
        }

        const lexeme_slice = self.input[start..self.position];
        
        // Use arena allocator to store lexeme copy
        const arena_allocator = self.arena.allocator();
        const lexeme = try arena_allocator.dupe(u8, lexeme_slice);
        
        // Special handling for "fr" - check if it's "fr fr" comment
        if (std.mem.eql(u8, lexeme, "fr")) {
            // Look ahead for whitespace + "fr"
            const saved_pos = self.position;
            const saved_col = self.column;
            
            // Skip whitespace
            while (!self.isAtEnd() and (self.peek() == ' ' or self.peek() == '\t')) {
                _ = self.advance();
            }
            
            // Check for second "fr"
            if (self.position + 2 <= self.input.len and std.mem.eql(u8, self.input[self.position..self.position + 2], "fr")) {
                // This is "fr fr" comment - consume the rest of the line
                self.position += 2;
                self.column += 2;
                
                // Skip to end of line
                while (!self.isAtEnd() and self.peek() != '\n') {
                    _ = self.advance();
                }
                
                const comment_slice = self.input[start..self.position];
                const comment_lexeme = try arena_allocator.dupe(u8, comment_slice);
                return Token.init(.LineComment, comment_lexeme, line, column);
            } else {
                // Not a comment, restore position and treat as identifier
                self.position = saved_pos;
                self.column = saved_col;
            }
        }
        
        // Special handling for "no" - check if it's "no cap" block comment start
        if (std.mem.eql(u8, lexeme, "no")) {
            const saved_pos = self.position;
            const saved_col = self.column;
            
            // Skip whitespace
            while (!self.isAtEnd() and (self.peek() == ' ' or self.peek() == '\t')) {
                _ = self.advance();
            }
            
            // Check for "cap"
            if (self.position + 3 <= self.input.len and std.mem.eql(u8, self.input[self.position..self.position + 3], "cap")) {
                // This is "no cap" block comment start
                self.position += 3;
                self.column += 3;
                
                // Skip until "on god"
                while (self.position + 6 <= self.input.len) {
                    if (std.mem.eql(u8, self.input[self.position..self.position + 2], "on")) {
                        // Check for whitespace + "god"
                        var temp_pos = self.position + 2;
                        while (temp_pos < self.input.len and (self.input[temp_pos] == ' ' or self.input[temp_pos] == '\t')) {
                            temp_pos += 1;
                        }
                        if (temp_pos + 3 <= self.input.len and std.mem.eql(u8, self.input[temp_pos..temp_pos + 3], "god")) {
                            // Found "on god" - consume it and return comment token
                            self.position = temp_pos + 3;
                            // Update column/line tracking (simplified)
                            self.column += @intCast(temp_pos + 3 - self.position);
                            
                            const comment_slice = self.input[start..self.position];
                            const comment_lexeme = try arena_allocator.dupe(u8, comment_slice);
                            return Token.init(.BlockComment, comment_lexeme, line, column);
                        }
                    }
                    
                    if (self.peek() == '\n') {
                        self.line += 1;
                        self.column = 1;
                    }
                    _ = self.advance();
                }
                
                // Unterminated block comment
                return error.UnterminatedBlockComment;
            } else {
                // Not a comment, restore position and treat as identifier
                self.position = saved_pos;
                self.column = saved_col;
            }
        }
        
        const kind = getKeywordType(lexeme);
        return Token.init(kind, lexeme, line, column);
    }

    fn getKeywordType(text: []const u8) TokenKind {
        // CURSED Gen Z Keywords
        if (std.mem.eql(u8, text, "slay")) return .Slay;
        if (std.mem.eql(u8, text, "yolo")) return .Yolo;
        if (std.mem.eql(u8, text, "sus")) return .Sus;
        if (std.mem.eql(u8, text, "facts")) return .Facts;
        if (std.mem.eql(u8, text, "lowkey")) return .Lowkey;
        if (std.mem.eql(u8, text, "highkey")) return .Highkey;
        if (std.mem.eql(u8, text, "periodt")) return .Periodt;
        if (std.mem.eql(u8, text, "stan")) return .Stan;
        if (std.mem.eql(u8, text, "bestie")) return .Bestie;
        if (std.mem.eql(u8, text, "flex")) return .Flex;
        if (std.mem.eql(u8, text, "ghosted")) return .Ghosted;
        if (std.mem.eql(u8, text, "simp")) return .Simp;
        if (std.mem.eql(u8, text, "squad")) return .Squad;
        if (std.mem.eql(u8, text, "struct")) return .Struct;
        if (std.mem.eql(u8, text, "collab")) return .Collab;
        if (std.mem.eql(u8, text, "impl")) return .Impl;
        if (std.mem.eql(u8, text, "extends")) return .Extends;
        if (std.mem.eql(u8, text, "with")) return .With;
        if (std.mem.eql(u8, text, "as")) return .As;
        if (std.mem.eql(u8, text, "except")) return .Except;
        if (std.mem.eql(u8, text, "rename")) return .Rename;
        if (std.mem.eql(u8, text, "vibe")) return .Vibe;
        if (std.mem.eql(u8, text, "yeet")) return .Yeet;
        if (std.mem.eql(u8, text, "be_like")) return .BeLike;
        if (std.mem.eql(u8, text, "vibe_check")) return .VibeCheck;
        if (std.mem.eql(u8, text, "mood")) return .Mood;
        if (std.mem.eql(u8, text, "basic")) return .Basic;
        if (std.mem.eql(u8, text, "match")) return .Match;
        if (std.mem.eql(u8, text, "type_check")) return .TypeCheck;
        if (std.mem.eql(u8, text, "yeet_error")) return .YeetError;
        if (std.mem.eql(u8, text, "catch")) return .Catch;
        if (std.mem.eql(u8, text, "where")) return .Where;
        if (std.mem.eql(u8, text, "later")) return .Later;
        if (std.mem.eql(u8, text, "in")) return .In;

        // Types
        if (std.mem.eql(u8, text, "normie")) return .Normie;
        if (std.mem.eql(u8, text, "tea")) return .Tea;
        if (std.mem.eql(u8, text, "txt")) return .Txt;
        if (std.mem.eql(u8, text, "sip")) return .Sip;
        if (std.mem.eql(u8, text, "smol")) return .Smol;
        if (std.mem.eql(u8, text, "mid")) return .Mid;
        if (std.mem.eql(u8, text, "thicc")) return .Thicc;
        if (std.mem.eql(u8, text, "snack")) return .Snack;
        if (std.mem.eql(u8, text, "meal")) return .Meal;
        if (std.mem.eql(u8, text, "byte")) return .Byte;
        if (std.mem.eql(u8, text, "rune")) return .Rune;
        if (std.mem.eql(u8, text, "extra")) return .Extra;
        if (std.mem.eql(u8, text, "lit")) return .Lit;
        if (std.mem.eql(u8, text, "dm")) return .Dm;

        // Literals (canonical spec conformance)
        if (std.mem.eql(u8, text, "based")) return .Based;   // true literal
        if (std.mem.eql(u8, text, "cringe")) return .Cringe; // false literal  
        if (std.mem.eql(u8, text, "nah")) return .Nah;       // nil literal
        if (std.mem.eql(u8, text, "no_cap")) return .NoCap;
        
        // Deprecated forms - treated as identifiers to trigger parser errors
        if (std.mem.eql(u8, text, "cap")) return .Identifier;   // Use 'nah' instead
        if (std.mem.eql(u8, text, "truth")) return .Identifier; // Use 'based' instead
        if (std.mem.eql(u8, text, "lies")) return .Identifier;  // Use 'cringe' instead
        if (std.mem.eql(u8, text, "main_character")) return .MainCharacter;

        // Error handling
        if (std.mem.eql(u8, text, "yikes")) return .Yikes;
        if (std.mem.eql(u8, text, "shook")) return .Shook;
        if (std.mem.eql(u8, text, "fam")) return .Fam;
        if (std.mem.eql(u8, text, "panic")) return .Panic;
        if (std.mem.eql(u8, text, "recover")) return .Recover;

        // Visibility
        if (std.mem.eql(u8, text, "spill")) return .Spill;
        if (std.mem.eql(u8, text, "priv")) return .Priv;
        if (std.mem.eql(u8, text, "crew")) return .Crew;

        // Control flow
        if (std.mem.eql(u8, text, "select")) return .Select;
        if (std.mem.eql(u8, text, "ready")) return .Ready;

        // Traditional keywords (for compatibility)
        if (std.mem.eql(u8, text, "let")) return .Let;
        if (std.mem.eql(u8, text, "mut")) return .Mut;
        if (std.mem.eql(u8, text, "fn")) return .Fn;
        if (std.mem.eql(u8, text, "if")) return .If;
        if (std.mem.eql(u8, text, "else")) return .Else;
        if (std.mem.eql(u8, text, "while")) return .While;
        if (std.mem.eql(u8, text, "for")) return .For;
        if (std.mem.eql(u8, text, "return")) return .Return;
        if (std.mem.eql(u8, text, "async")) return .Async;
        if (std.mem.eql(u8, text, "unsafe")) return .Unsafe;
        if (std.mem.eql(u8, text, "public")) return .Public;
        if (std.mem.eql(u8, text, "private")) return .Private;

        return .Identifier;
    }
};

test "memory safe lexer" {
    const allocator = std.testing.allocator;
    
    var arena = ArenaAllocator.init(allocator);
    defer arena.deinit();
    
    var lexer = Lexer.init(&arena, "slay main_character() { }");
    var tokens = try lexer.tokenize();
    defer tokens.deinit();

    try std.testing.expect(tokens.items().len >= 5);
    try std.testing.expect(tokens.items()[0].kind == .Slay);
    try std.testing.expect(tokens.items()[1].kind == .MainCharacter);
    try std.testing.expect(tokens.items()[2].kind == .LeftParen);
    try std.testing.expect(tokens.items()[3].kind == .RightParen);
    try std.testing.expect(tokens.items()[4].kind == .LeftBrace);
}
