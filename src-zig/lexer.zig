const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub const TokenKind = enum {
    // Literals
    Number,
    Integer,
    StringLiteral,
    String,
    Boolean,
    Character,
    Based, // For 'based' literal

    // Identifiers
    Identifier,

    // Traditional Keywords (for compatibility)
    Let,
    Mut,
    Fn,
    If,
    Else,
    While,
    For,
    Return,

    // CURSED Gen Z Keywords
    Slay, // function definition
    Yolo, // return statement
    Sus, // mutable variable
    Facts, // immutable constant
    Lowkey, // if statement
    Highkey, // else statement
    Periodt, // while loop
    Stan, // goroutine
    Bestie, // for loop
    Flex, // while loop (alternative)
    Ghosted, // break
    Simp, // continue
    Squad, // struct
    Struct, // struct (alternative)
    Collab, // interface
    Impl, // implementation
    Extends, // interface inheritance
    With, // interface composition
    As, // alias in composition
    Except, // exclusion in composition
    Rename, // method renaming in composition
    ForImpl, // for (used in impl for)
    Vibe, // package
    Yeet, // import
    BeLike, // assignment operator
    VibeCheck, // switch statement
    Mood, // case
    Basic, // default case
    Match, // match expression
    TypeCheck, // type switch expression (match variable is)
    YeetError, // throw error
    Catch, // catch error
    Where, // where clause for generics
    Normie, // integer type (i32)
    Tea, // string type
    Txt, // string type (alias)
    Sip, // character type
    Smol, // small integer type (i8)
    Mid, // medium integer type (i16)
    Thicc, // large integer type (i64)
    Snack, // small float type (f32)
    Meal, // large float type (f64)
    Byte, // unsigned 8-bit integer (u8)
    Rune, // Unicode code point (i32 alias)
    Extra, // complex number type
    Lit, // boolean type
    Cap, // null/nil
    NoCap, // not null
    Truth, // true
    Lies, // false (NoTruth)
    MainCharacter, // main function
    Dm, // channel type
    Select, // select statement
    Ready, // ready (for select statements)
    LeftArrow, // <- channel operator
    Arrow, // -> return type arrow
    Later, // later (defer statement)
    In, // in (for-in loops)

    // Error handling tokens
    Yikes, // error type declarations
    Shook, // error propagation operator / panic function
    Fam, // panic recovery blocks
    Panic, // panic function
    Recover, // recover function

    // Visibility modifiers
    Spill, // pub (public)
    Priv, // private
    Crew, // pkg (package)

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent, // %
    PlusPlus, // ++
    MinusMinus, // --
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpAmp, // &&
    PipePipe, // ||
    Pipe, // |
    Amp, // &
    Caret, // ^
    LeftShift, // <<
    RightShift, // >>

    // Assignment operators
    Assign, // = (for assignment context)
    PlusEqual, // +=
    MinusEqual, // -=
    StarEqual, // *=
    SlashEqual, // /=
    PercentEqual, // %=
    ColonEqual, // :=

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle, // < (for generics)
    RightAngle, // > (for generics)
    Comma,
    Semicolon,
    Colon,
    DoubleColon, // :: (for paths and type annotations)
    Dot,
    DotDot, // .. (for range expressions)
    Question, // ?

    // Special
    At, // @ (for pointer types)
    Newline,
    Eof,

    // Advanced function signature features
    Async, // async keyword
    Unsafe, // unsafe keyword
    Public, // pub keyword
    Private, // priv keyword
    Comment, // Comment with content
    IntegerLiteral, // Integer literal token
    DotDotDot, // ... (variadic parameters)

    // Comments
    LineComment, // fr fr line comment
    BlockComment, // no cap ... on god block comment

    pub fn format(self: TokenKind, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        try writer.print("{s}", .{@tagName(self)});
    }
};

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

pub const Lexer = struct {
    input: []const u8,
    position: usize,
    line: usize,
    column: usize,
    allocator: Allocator,

    pub fn init(allocator: Allocator, input: []const u8) Lexer {
        return Lexer{
            .input = input,
            .position = 0,
            .line = 1,
            .column = 1,
            .allocator = allocator,
        };
    }

    pub fn tokenize(self: *Lexer) !ArrayList(Token) {
        var tokens = ArrayList(Token).init(self.allocator);
        
        while (!self.isAtEnd()) {
            const token = try self.nextToken();
            if (token.kind != .Newline) { // Skip newlines for now
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
                
                // Handle "fr fr" comment
                if (c == 'f' and self.position < self.input.len and self.input[self.position] == 'r') {
                    if (self.position + 1 < self.input.len and self.input[self.position + 1] == ' ') {
                        if (self.position + 2 < self.input.len and self.input[self.position + 2] == 'f') {
                            if (self.position + 3 < self.input.len and self.input[self.position + 3] == 'r') {
                                // This is a "fr fr" comment
                                self.position += 4; // Skip "r fr"
                                self.column += 4;
                                while (self.peek() != '\n' and !self.isAtEnd()) {
                                    _ = self.advance();
                                }
                                return self.makeToken(.LineComment, start_line, start_column);
                            }
                        }
                    }
                    // Not a comment, back up and handle as identifier
                    self.position -= 1;
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

    fn makeToken(self: *Lexer, kind: TokenKind, line: usize, column: usize) Token {
        const start = if (self.position > 0) self.position - 1 else 0;
        const lexeme = self.input[start..self.position];
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

        const lexeme = self.input[start..self.position];
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

        const lexeme = self.input[start..self.position];
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

        const lexeme = self.input[start..self.position];
        return Token.init(.Number, lexeme, line, column);
    }

    fn identifier(self: *Lexer, line: usize, column: usize) !Token {
        const start = self.position;
        
        while (std.ascii.isAlphanumeric(self.peek()) or self.peek() == '_') {
            _ = self.advance();
        }

        const lexeme = self.input[start..self.position];
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

        // Literals
        if (std.mem.eql(u8, text, "based")) return .Based;
        if (std.mem.eql(u8, text, "cap")) return .Cap;
        if (std.mem.eql(u8, text, "no_cap")) return .NoCap;
        if (std.mem.eql(u8, text, "truth")) return .Truth;
        if (std.mem.eql(u8, text, "lies")) return .Lies;
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

test "lexer basic tokens" {
    const allocator = std.testing.allocator;
    
    var lexer = Lexer.init(allocator, "slay main_character() { }");
    const tokens = try lexer.tokenize();
    defer tokens.deinit();

    try std.testing.expect(tokens.items.len >= 5);
    try std.testing.expect(tokens.items[0].kind == .Slay);
    try std.testing.expect(tokens.items[1].kind == .MainCharacter);
    try std.testing.expect(tokens.items[2].kind == .LeftParen);
    try std.testing.expect(tokens.items[3].kind == .RightParen);
    try std.testing.expect(tokens.items[4].kind == .LeftBrace);
}

test "lexer numbers" {
    const allocator = std.testing.allocator;
    
    var lexer = Lexer.init(allocator, "42 3.14");
    const tokens = try lexer.tokenize();
    defer tokens.deinit();

    try std.testing.expect(tokens.items.len >= 2);
    try std.testing.expect(tokens.items[0].kind == .Number);
    try std.testing.expect(tokens.items[1].kind == .Number);
}

test "lexer strings" {
    const allocator = std.testing.allocator;
    
    var lexer = Lexer.init(allocator, "\"hello world\"");
    const tokens = try lexer.tokenize();
    defer tokens.deinit();

    try std.testing.expect(tokens.items.len >= 1);
    try std.testing.expect(tokens.items[0].kind == .StringLiteral);
}

test "lexer bitwise operators" {
    const allocator = std.testing.allocator;
    
    var lexer = Lexer.init(allocator, "& | ^ << >>");
    const tokens = try lexer.tokenize();
    defer tokens.deinit();

    try std.testing.expect(tokens.items.len >= 5);
    try std.testing.expect(tokens.items[0].kind == .Amp);
    try std.testing.expect(tokens.items[1].kind == .Pipe);
    try std.testing.expect(tokens.items[2].kind == .Caret);
    try std.testing.expect(tokens.items[3].kind == .LeftShift);
    try std.testing.expect(tokens.items[4].kind == .RightShift);
}
