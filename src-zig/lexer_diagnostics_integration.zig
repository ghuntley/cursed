const std = @import("std");
const lexer = @import("lexer.zig");
const diagnostics = @import("error_diagnostics.zig");

const TokenKind = lexer.TokenKind;
const Token = lexer.Token;
const Lexer = lexer.Lexer;
const DiagnosticEngine = diagnostics.DiagnosticEngine;
const SourceSpan = diagnostics.SourceSpan;
const ErrorCode = diagnostics.ErrorCode;

/// Enhanced Lexer with integrated error diagnostics
pub const DiagnosticLexer = struct {
    lexer: Lexer,
    diagnostics: *DiagnosticEngine,
    file_path: []const u8,
    
    pub fn init(allocator: std.mem.Allocator, source: []const u8, file_path: []const u8, diagnostics_engine: *DiagnosticEngine) !DiagnosticLexer {
        const lex = Lexer.init(allocator, source);
        
        // Add source file to diagnostics engine
        try diagnostics_engine.addSourceFile(file_path, source);
        
        return DiagnosticLexer{
            .lexer = lex,
            .diagnostics = diagnostics_engine,
            .file_path = file_path,
        };
    }
    
    pub fn deinit(self: *DiagnosticLexer) void {
        self.lexer.deinit(allocator);
    }
    
    pub fn nextToken(self: *DiagnosticLexer) !?Token {
        const result = self.lexer.nextToken();
        
        if (result) |token_or_error| {
            return token_or_error;
        } else |err| {
            // Convert lexer errors to diagnostic errors
            try self.reportLexerError(err);
            return null;
        }
    }
    
    fn reportLexerError(self: *DiagnosticLexer, err: anyerror) !void {
        const current_line = @as(u32, @intCast(self.lexer.line));
        const current_column = @as(u32, @intCast(self.lexer.column));
        const current_offset = @as(u32, @intCast(self.lexer.position));
        
        const span = SourceSpan.fromSinglePosition(
            self.file_path,
            current_line,
            current_column,
            current_offset
        );
        
        switch (err) {
            error.UnterminatedString => {
                try self.diagnostics.reportError(
                    .L001_UnterminatedString,
                    "String literal is missing closing quote",
                    span
                );
            },
            error.UnterminatedChar => {
                try self.diagnostics.reportError(
                    .L001_UnterminatedString, // Reuse string error code for character
                    "Character literal is missing closing quote",
                    span
                );
            },
            error.UnterminatedBlockComment => {
                try self.diagnostics.reportError(
                    .L004_UnterminatedComment,
                    "Block comment is not properly terminated",
                    span
                );
            },
            error.UnexpectedCharacter => {
                try self.diagnostics.reportError(
                    .L009_UnexpectedCharacter,
                    "Unexpected character in source code",
                    span
                );
            },
            error.InvalidNumber => {
                try self.diagnostics.reportError(
                    .L003_InvalidNumber,
                    "Invalid number format",
                    span
                );
            },
            error.NumberOverflow => {
                try self.diagnostics.reportError(
                    .L007_NumberOverflow,
                    "Number literal exceeds maximum value",
                    span
                );
            },
            error.InvalidFloat => {
                try self.diagnostics.reportError(
                    .L008_InvalidFloatFormat,
                    "Invalid floating-point number format",
                    span
                );
            },
            error.InvalidEscape => {
                try self.diagnostics.reportError(
                    .L005_InvalidEscape,
                    "Invalid escape sequence in string literal",
                    span
                );
            },
            error.InvalidUnicodeEscape => {
                try self.diagnostics.reportError(
                    .L006_InvalidUnicode,
                    "Invalid Unicode escape sequence",
                    span
                );
            },
            else => {
                try self.diagnostics.reportError(
                    .L002_InvalidCharacter,
                    "Lexical analysis error",
                    span
                );
            }
        }
    }
    
    pub fn reportCustomError(self: *DiagnosticLexer, code: ErrorCode, message: []const u8, start_offset: u32, end_offset: u32) !void {
        const span = SourceSpan.init(
            self.file_path,
            @as(u32, @intCast(self.lexer.line)),
            @as(u32, @intCast(self.lexer.column)),
            @as(u32, @intCast(self.lexer.line)),
            @as(u32, @intCast(self.lexer.column)) + (end_offset - start_offset),
            start_offset,
            end_offset
        );
        
        try self.diagnostics.reportError(code, message, span);
    }
    
    pub fn reportWarning(self: *DiagnosticLexer, code: ErrorCode, message: []const u8) !void {
        const span = SourceSpan.fromSinglePosition(
            self.file_path,
            @as(u32, @intCast(self.lexer.line)),
            @as(u32, @intCast(self.lexer.column)),
            @as(u32, @intCast(self.lexer.position))
        );
        
        try self.diagnostics.reportWarning(code, message, span);
    }
    
    /// Check for common lexical issues and emit warnings/hints
    pub fn performLintChecks(self: *DiagnosticLexer) !void {
        // Reset lexer position for full scan
        const original_position = self.lexer.position;
        const original_line = self.lexer.line;
        const original_column = self.lexer.column;
        
        defer {
            self.lexer.position = original_position;
            self.lexer.line = original_line;
            self.lexer.column = original_column;
        }
        
        self.lexer.position = 0;
        self.lexer.line = 1;
        self.lexer.column = 1;
        
        var brace_depth: i32 = 0;
        var paren_depth: i32 = 0;
        var bracket_depth: i32 = 0;
        
        while (self.lexer.nextToken() catch null) |token| {
            switch (token.kind) {
                .LeftBrace => brace_depth += 1,
                .RightBrace => {
                    brace_depth -= 1;
                    if (brace_depth < 0) {
                        const span = SourceSpan.fromSinglePosition(
                            self.file_path,
                            @as(u32, @intCast(token.line)),
                            @as(u32, @intCast(token.column)),
                            @as(u32, @intCast(token.start))
                        );
                        try self.diagnostics.reportError(
                            .P008_UnbalancedBraces,
                            "Unexpected closing brace",
                            span
                        );
                    }
                },
                .LeftParen => paren_depth += 1,
                .RightParen => {
                    paren_depth -= 1;
                    if (paren_depth < 0) {
                        const span = SourceSpan.fromSinglePosition(
                            self.file_path,
                            token.line,
                            token.column,
                            token.start
                        );
                        try self.diagnostics.reportError(
                            .P014_MissingParen,
                            "Unexpected closing parenthesis",
                            span
                        );
                    }
                },
                .LeftBracket => bracket_depth += 1,
                .RightBracket => {
                    bracket_depth -= 1;
                    if (bracket_depth < 0) {
                        const span = SourceSpan.fromSinglePosition(
                            self.file_path,
                            token.line,
                            token.column,
                            token.start
                        );
                        try self.diagnostics.reportError(
                            .P008_UnbalancedBraces,
                            "Unexpected closing bracket",
                            span
                        );
                    }
                },
                .Number => {
                    // Check for potential overflow issues
                    const token_text = self.lexer.source[token.start..token.end];
                    if (token_text.len > 10) { // Potential overflow for 32-bit int
                        const span = SourceSpan.init(
                            self.file_path,
                            token.line,
                            token.column,
                            token.line,
                            token.column + @as(u32, @intCast(token_text.len)),
                            token.start,
                            token.end
                        );
                        try self.diagnostics.reportWarning(
                            .L007_NumberOverflow,
                            "Large number literal may cause overflow",
                            span
                        );
                    }
                },
                else => {},
            }
        }
        
        // Check for unclosed delimiters at end of file
        if (brace_depth > 0) {
            const span = SourceSpan.fromSinglePosition(
                self.file_path,
                self.lexer.line,
                self.lexer.column,
                @intCast(self.lexer.current - self.lexer.start)
            );
            try self.diagnostics.reportError(
                .P013_MissingBrace,
                "Missing closing brace at end of file",
                span
            );
        }
        
        if (paren_depth > 0) {
            const span = SourceSpan.fromSinglePosition(
                self.file_path,
                self.lexer.line,
                self.lexer.column,
                @intCast(self.lexer.current - self.lexer.start)
            );
            try self.diagnostics.reportError(
                .P014_MissingParen,
                "Missing closing parenthesis at end of file",
                span
            );
        }
    }
};

// Testing
test "diagnostic lexer integration" {
    const allocator = std.testing.allocator;
    
    var engine = DiagnosticEngine.init(allocator, 10);
    defer engine.deinit(allocator);
    
    // Test with error-prone source
    const bad_source = "sus x tea = \"unterminated string\nsus y normie = 999999999999999999999";
    
    var diagnostic_lexer = try DiagnosticLexer.init(allocator, bad_source, "test.csd", &engine);
    defer diagnostic_lexer.deinit(allocator);
    
    // Tokenize and collect errors
    while (try diagnostic_lexer.nextToken()) |_| {
        // Continue tokenizing
    }
    
    // Run lint checks
    try diagnostic_lexer.performLintChecks();
    
    try std.testing.expect(engine.hasErrors());
}
