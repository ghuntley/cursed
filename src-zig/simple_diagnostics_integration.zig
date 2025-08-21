const std = @import("std");
const diagnostics = @import("error_diagnostics.zig");

const DiagnosticEngine = diagnostics.DiagnosticEngine;
const SourceSpan = diagnostics.SourceSpan;
const ErrorCode = diagnostics.ErrorCode;

/// Simple utility functions for integrating diagnostics with the compiler
pub const DiagnosticUtils = struct {
    
    /// Report a lexical error at a specific position
    pub fn reportLexError(
        engine: *DiagnosticEngine,
        code: ErrorCode,
        message: []const u8,
        file_path: []const u8,
        line: u32,
        column: u32,
        offset: u32
    ) !void {
        const span = SourceSpan.fromSinglePosition(file_path, line, column, offset);
        try engine.reportError(code, message, span);
    }
    
    /// Report a parser error spanning multiple tokens
    pub fn reportParseError(
        engine: *DiagnosticEngine,
        code: ErrorCode,
        message: []const u8,
        file_path: []const u8,
        start_line: u32,
        start_column: u32,
        end_line: u32,
        end_column: u32,
        start_offset: u32,
        end_offset: u32
    ) !void {
        const span = SourceSpan.init(
            file_path,
            start_line,
            start_column,
            end_line,
            end_column,
            start_offset,
            end_offset
        );
        try engine.reportError(code, message, span);
    }
    
    /// Report a semantic error with optional related spans
    pub fn reportSemanticError(
        engine: *DiagnosticEngine,
        code: ErrorCode,
        message: []const u8,
        primary_span: SourceSpan,
        related_spans: ?[]const SourceSpan
    ) !void {
        try engine.reportError(code, message, primary_span);
        
        if (related_spans) |spans| {
            const diagnostic = &engine.diagnostics.items[engine.diagnostics.items.len - 1];
            for (spans) |span| {
                try diagnostic.addSecondarySpan(span);
            }
        }
    }
    
    /// Create a source span from simple parameters
    pub fn createSpan(
        file_path: []const u8,
        line: u32,
        column: u32,
        length: u32
    ) SourceSpan {
        return SourceSpan.init(
            file_path,
            line,
            column,
            line,
            column + length,
            0, // Offset calculation would need more context
            length
        );
    }
    
    /// Helper to report common CURSED syntax errors
    pub fn reportCursedSyntaxError(
        engine: *DiagnosticEngine,
        file_path: []const u8,
        line: u32,
        column: u32,
        found_token: []const u8,
        expected_token: []const u8
    ) !void {
        const span = SourceSpan.fromSinglePosition(file_path, line, column, 0);
        const message = try std.fmt.allocPrint(
            engine.allocator,
            "Expected '{s}' but found '{s}'",
            .{ expected_token, found_token }
        );
        defer engine.allocator.free(message);
        
        try engine.reportError(.P002_ExpectedToken, message, span);
    }
    
    /// Helper to report CURSED type mismatch errors
    pub fn reportTypeError(
        engine: *DiagnosticEngine,
        file_path: []const u8,
        line: u32,
        column: u32,
        expected_type: []const u8,
        actual_type: []const u8
    ) !void {
        const span = SourceSpan.fromSinglePosition(file_path, line, column, 0);
        const message = try std.fmt.allocPrint(
            engine.allocator,
            "Type mismatch: expected '{s}' but got '{s}'",
            .{ expected_type, actual_type }
        );
        defer engine.allocator.free(message);
        
        try engine.reportError(.S003_TypeMismatch, message, span);
    }
    
    /// Helper to report undefined variable errors
    pub fn reportUndefinedVariable(
        engine: *DiagnosticEngine,
        file_path: []const u8,
        line: u32,
        column: u32,
        variable_name: []const u8
    ) !void {
        const span = SourceSpan.fromSinglePosition(file_path, line, column, 0);
        const message = try std.fmt.allocPrint(
            engine.allocator,
            "Variable '{s}' is not defined in current scope",
            .{variable_name}
        );
        defer engine.allocator.free(message);
        
        try engine.reportError(.S001_UndefinedVariable, message, span);
    }
    
    /// Helper to report function not found errors
    pub fn reportUndefinedFunction(
        engine: *DiagnosticEngine,
        file_path: []const u8,
        line: u32,
        column: u32,
        function_name: []const u8
    ) !void {
        const span = SourceSpan.fromSinglePosition(file_path, line, column, 0);
        const message = try std.fmt.allocPrint(
            engine.allocator,
            "Function '{s}' is not defined",
            .{function_name}
        );
        defer engine.allocator.free(message);
        
        try engine.reportError(.S002_UndefinedFunction, message, span);
    }
};

// Testing
test "diagnostic utils integration" {
    const allocator = std.testing.allocator;
    
    var engine = DiagnosticEngine.init(allocator, 10);
    defer engine.deinit(allocator);
    
    const test_source = "sus x normie = \"string value\"\n";
    try engine.addSourceFile("test.csd", test_source);
    
    // Test various error types
    try DiagnosticUtils.reportLexError(&engine, .L001_UnterminatedString, "Test lexical error", "test.csd", 1, 15, 14);
    try DiagnosticUtils.reportTypeError(&engine, "test.csd", 1, 16, "normie", "tea");
    try DiagnosticUtils.reportUndefinedVariable(&engine, "test.csd", 2, 5, "undefinedVar");
    
    try std.testing.expect(engine.hasErrors());
    try std.testing.expect(engine.getErrorCount() == 3);
}
