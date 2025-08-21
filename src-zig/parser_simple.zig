const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const ast = @import("ast_simple.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;

pub const ParserError = error{
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,
    OutOfMemory,
};

pub const Parser = struct {
    tokens: []const Token,
    current: usize,
    allocator: Allocator,

    pub fn init(allocator: Allocator, tokens: []const Token) Parser {
        return Parser{
            .tokens = tokens,
            .current = 0,
            .allocator = allocator,
        };
    }

    pub fn parseProgram(self: *Parser) ParserError!ast.Program {
        var program = ast.Program.init(self.allocator);
        
        while (!self.isAtEnd()) {
            // Skip newlines and semicolons
            if (self.check(.Newline) or self.check(.Semicolon)) {
                _ = self.advance();
                continue;
            }

            // Parse package declaration
            if (self.check(.Vibe)) {
                program.package = try self.parsePackageDeclaration();
                continue;
            }

            // Parse import statement
            if (self.check(.Yeet)) {
                const import_stmt = try self.parseImportStatement();
                try program.imports.append(allocator, import_stmt);
                continue;
            }

            // Parse regular statements
            const stmt = try self.parseStatement();
            try program.statements.append(allocator, stmt);
        }

        return program;
    }

    fn parsePackageDeclaration(self: *Parser) ParserError!ast.PackageDeclaration {
        _ = try self.consume(.Vibe, "Expected 'vibe'");
        
        if (!self.check(.Identifier)) {
            return ParserError.UnexpectedToken;
        }
        
        const name = self.previous().lexeme;
        _ = self.advance();
        
        return ast.PackageDeclaration{
            .name = name,
            .version = null,
        };
    }

    fn parseImportStatement(self: *Parser) ParserError!ast.ImportStatement {
        _ = try self.consume(.Yeet, "Expected 'yeet'");
        
        if (!self.check(.StringLiteral) and !self.check(.String)) {
            return ParserError.UnexpectedToken;
        }
        
        const path_token = self.advance();
        const path = if (path_token.lexeme.len >= 2 and 
                        path_token.lexeme[0] == '"' and 
                        path_token.lexeme[path_token.lexeme.len-1] == '"')
                     path_token.lexeme[1..path_token.lexeme.len-1] // Remove quotes
                     else path_token.lexeme;
        
        var import_stmt = ast.ImportStatement.init(self.allocator, path);
        
        // Handle alias (as name)
        if (self.match(.As)) {
            if (self.check(.Identifier)) {
                import_stmt.alias = self.advance().lexeme;
            }
        }
        
        return import_stmt;
    }

    fn parseStatement(self: *Parser) ParserError!ast.Statement {
        // Function declaration (slay)
        if (self.check(.Slay)) {
            return ast.Statement.Function;
        }
        
        // Variable declaration (sus/facts)
        if (self.check(.Sus) or self.check(.Facts)) {
            _ = self.advance();
            return ast.Statement.Let;
        }
        
        // Return statement (yolo/damn)
        if (self.check(.Yolo) or (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "damn"))) {
            _ = self.advance();
            return ast.Statement.Return;
        }
        
        // If statement (lowkey)
        if (self.check(.Lowkey)) {
            _ = self.advance();
            return ast.Statement.If;
        }
        
        // While statement (periodt/flex)
        if (self.check(.Periodt) or self.check(.Flex)) {
            _ = self.advance();
            return ast.Statement.While;
        }
        
        // For statement (bestie)
        if (self.check(.Bestie)) {
            _ = self.advance();
            return ast.Statement.For;
        }
        
        // Break/continue
        if (self.check(.Ghosted)) {
            _ = self.advance();
            return ast.Statement.Break;
        }
        
        if (self.check(.Simp)) {
            _ = self.advance();
            return ast.Statement.Continue;
        }
        
        // Defer statement (later)
        if (self.check(.Later)) {
            _ = self.advance();
            return ast.Statement.Defer;
        }
        
        // Struct declaration (squad)
        if (self.check(.Squad) or self.check(.Struct)) {
            _ = self.advance();
            return ast.Statement.Struct;
        }
        
        // Interface declaration (collab)
        if (self.check(.Collab)) {
            _ = self.advance();
            return ast.Statement.Interface;
        }

        // Implementation statement (flex)
        if (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "flex")) {
            _ = self.advance();
            return ast.Statement.Implementation;
        }
        
        // Type alias
        if (self.check(.BeLike) or (self.check(.Identifier) and std.mem.eql(u8, self.peek().lexeme, "be"))) {
            _ = self.advance();
            return ast.Statement.TypeAlias;
        }

        // Goroutine statement (stan)
        if (self.check(.Stan)) {
            _ = self.advance();
            return ast.Statement.Goroutine;
        }

        // Match expression (match)
        if (self.check(.Match)) {
            _ = self.advance();
            return ast.Statement.Expression;
        }

        // Vibe check (switch)
        if (self.check(.VibeCheck)) {
            _ = self.advance();
            return ast.Statement.PatternSwitch;
        }

        // Select statement
        if (self.check(.Select) or self.check(.Ready)) {
            _ = self.advance();
            return ast.Statement.Select;
        }
        
        // Error handling statements
        if (self.check(.Yikes)) {
            _ = self.advance();
            return ast.Statement.Yikes;
        }
        
        if (self.check(.Fam)) {
            _ = self.advance();
            return ast.Statement.Fam;
        }

        // Constants (facts at top level)
        if (self.check(.Facts)) {
            _ = self.advance();
            return ast.Statement.Const;
        }
        
        // Short variable declaration or assignment
        if (self.isShortDeclaration()) {
            _ = self.advance();
            return ast.Statement.ShortDeclaration;
        }

        if (self.isAssignment()) {
            _ = self.advance();
            return ast.Statement.Assignment;
        }
        
        // Expression statement - just consume tokens until we reach a statement boundary
        self.consumeExpression();
        return ast.Statement.Expression;
    }

    fn consumeExpression(self: *Parser) void {
        var depth: usize = 0;
        
        while (!self.isAtEnd()) {
            const token = self.peek();
            
            switch (token.kind) {
                .LeftParen, .LeftBrace, .LeftBracket => {
                    depth += 1;
                    _ = self.advance();
                },
                .RightParen, .RightBrace, .RightBracket => {
                    if (depth == 0) break;
                    depth -= 1;
                    _ = self.advance();
                },
                .Semicolon, .Newline => {
                    if (depth == 0) break;
                    _ = self.advance();
                },
                .Slay, .Sus, .Facts, .Lowkey, .Bestie, .Periodt, .Squad, .Collab => {
                    if (depth == 0) break;
                    _ = self.advance();
                },
                else => {
                    _ = self.advance();
                }
            }
        }
    }

    // Helper methods
    fn match(self: *Parser, kind: TokenKind) bool {
        if (self.check(kind)) {
            _ = self.advance();
            return true;
        }
        return false;
    }

    fn check(self: *Parser, kind: TokenKind) bool {
        if (self.isAtEnd()) return false;
        return self.peek().kind == kind;
    }

    fn advance(self: *Parser) Token {
        if (!self.isAtEnd()) self.current += 1;
        return self.previous();
    }

    fn isAtEnd(self: *Parser) bool {
        return self.peek().kind == .Eof;
    }

    fn peek(self: *Parser) Token {
        if (self.current >= self.tokens.len) {
            return Token.init(.Eof, "", 0, 0);
        }
        return self.tokens[self.current];
    }

    fn previous(self: *Parser) Token {
        if (self.current == 0) return self.tokens[0];
        return self.tokens[self.current - 1];
    }

    fn consume(self: *Parser, kind: TokenKind, message: []const u8) ParserError!Token {
        if (self.check(kind)) return self.advance();
        
        std.debug.print("Parser error: {s}. Expected {}, got {}\n", .{ message, kind, self.peek().kind });
        return ParserError.UnexpectedToken;
    }

    // Helper predicates for parsing decisions
    fn isShortDeclaration(self: *Parser) bool {
        // Simple check for := pattern
        var pos = self.current;
        
        while (pos < self.tokens.len) {
            const token_kind = self.tokens[pos].kind;
            if (token_kind == .ColonEqual) {
                return true;
            }
            if (token_kind == .Semicolon or token_kind == .Newline or 
               token_kind == .LeftBrace or token_kind == .RightBrace) {
                return false;
            }
            pos += 1;
        }
        
        return false;
    }

    fn isAssignment(self: *Parser) bool {
        // Simple assignment detection
        var pos = self.current;
        
        while (pos < self.tokens.len) {
            const token_kind = self.tokens[pos].kind;
            if (token_kind == .Equal or token_kind == .PlusEqual or 
               token_kind == .MinusEqual or token_kind == .StarEqual or
               token_kind == .SlashEqual or token_kind == .PercentEqual) {
                return true;
            }
            if (token_kind == .Semicolon or token_kind == .Newline or 
               token_kind == .LeftBrace or token_kind == .RightBrace) {
                return false;
            }
            pos += 1;
        }
        
        return false;
    }
};
