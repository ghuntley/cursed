const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

/// Advanced error diagnostics and stack trace system for CURSED
/// Provides comprehensive error reporting with source location and context

pub const StackFrame = struct {
    function_name: []const u8,
    file_path: []const u8,
    line_number: u32,
    column_number: u32,
    source_line: ?[]const u8,
    
    pub fn init(allocator: Allocator, func_name: []const u8, file: []const u8, line: u32, col: u32, source: ?[]const u8) !StackFrame {
        return StackFrame{
            .function_name = try allocator.dupe(u8, func_name),
            .file_path = try allocator.dupe(u8, file),
            .line_number = line,
            .column_number = col,
            .source_line = if (source) |s| try allocator.dupe(u8, s) else null,
        };
    }
    
    pub fn deinit(self: *StackFrame, allocator: Allocator) void {
        allocator.free(self.function_name);
        allocator.free(self.file_path);
        if (self.source_line) |line| {
            allocator.free(line);
        }
    }
    
    pub fn format(self: StackFrame, writer: anytype) !void {
        try writer.print("  at {s} ({s}:{}:{})\n", .{ 
            self.function_name, 
            self.file_path, 
            self.line_number, 
            self.column_number 
        });
        
        if (self.source_line) |line| {
            try writer.print("    {}\n", .{self.line_number});
            try writer.print("    {s}\n", .{line});
            
            // Add visual indicator for column position
            var i: u32 = 0;
            try writer.print("    ", .{});
            while (i < self.column_number - 1) : (i += 1) {
                try writer.print(" ", .{});
            }
            try writer.print("^\n", .{});
        }
    }
};

pub const ErrorSeverity = enum {
    Info,
    Warning, 
    Error,
    Fatal,
    
    pub fn toString(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Info => "INFO",
            .Warning => "WARNING", 
            .Error => "ERROR",
            .Fatal => "FATAL",
        };
    }
    
    pub fn getColor(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Info => "\x1b[36m",    // Cyan
            .Warning => "\x1b[33m", // Yellow
            .Error => "\x1b[31m",   // Red
            .Fatal => "\x1b[91m",   // Bright Red
        };
    }
};

pub const ErrorDiagnostic = struct {
    severity: ErrorSeverity,
    message: []const u8,
    error_code: []const u8,
    file_path: []const u8,
    line_number: u32,
    column_number: u32,
    source_line: ?[]const u8,
    stack_trace: ArrayList(StackFrame),
    suggestions: ArrayList([]const u8),
    related_errors: ArrayList(*ErrorDiagnostic),
    allocator: Allocator,
    
    pub fn init(
        allocator: Allocator,
        severity: ErrorSeverity,
        message: []const u8,
        error_code: []const u8,
        file: []const u8,
        line: u32,
        col: u32
    ) !ErrorDiagnostic {
        return ErrorDiagnostic{
            .severity = severity,
            .message = try allocator.dupe(u8, message),
            .error_code = try allocator.dupe(u8, error_code),
            .file_path = try allocator.dupe(u8, file),
            .line_number = line,
            .column_number = col,
            .source_line = null,
            .stack_trace = ArrayList(StackFrame).init(allocator),
            .suggestions = ArrayList([]const u8).init(allocator),
            .related_errors = ArrayList(*ErrorDiagnostic).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ErrorDiagnostic) void {
        self.allocator.free(self.message);
        self.allocator.free(self.error_code);
        self.allocator.free(self.file_path);
        
        if (self.source_line) |line| {
            self.allocator.free(line);
        }
        
        for (self.stack_trace.items) |*frame| {
            frame.deinit(self.allocator);
        }
        self.stack_trace.deinit();
        
        for (self.suggestions.items) |suggestion| {
            self.allocator.free(suggestion);
        }
        self.suggestions.deinit();
        
        // Note: Don't free related_errors items - they're owned elsewhere
        self.related_errors.deinit();
    }
    
    pub fn setSourceLine(self: *ErrorDiagnostic, source: []const u8) !void {
        if (self.source_line) |old_line| {
            self.allocator.free(old_line);
        }
        self.source_line = try self.allocator.dupe(u8, source);
    }
    
    pub fn addStackFrame(self: *ErrorDiagnostic, frame: StackFrame) !void {
        try self.stack_trace.append(frame);
    }
    
    pub fn addSuggestion(self: *ErrorDiagnostic, suggestion: []const u8) !void {
        try self.suggestions.append(try self.allocator.dupe(u8, suggestion));
    }
    
    pub fn addRelatedError(self: *ErrorDiagnostic, related: *ErrorDiagnostic) !void {
        try self.related_errors.append(related);
    }
    
    pub fn format(self: ErrorDiagnostic, writer: anytype, use_colors: bool) !void {
        const reset_color = if (use_colors) "\x1b[0m" else "";
        const severity_color = if (use_colors) self.severity.getColor() else "";
        
        // Main error header
        try writer.print("{s}[{s}] {s}: {s}{s}\n", .{
            severity_color,
            self.error_code,
            self.severity.toString(),
            self.message,
            reset_color
        });
        
        // Location information
        try writer.print("  → {s}:{}:{}\n", .{ self.file_path, self.line_number, self.column_number });
        
        // Source line with visual indicator
        if (self.source_line) |line| {
            try writer.print("   |\n", .{});
            try writer.print("{d:3} | {s}\n", .{ self.line_number, line });
            try writer.print("   | ", .{});
            
            var i: u32 = 0;
            while (i < self.column_number - 1) : (i += 1) {
                try writer.print(" ", .{});
            }
            try writer.print("{s}^{s}\n", .{ severity_color, reset_color });
            try writer.print("   |\n", .{});
        }
        
        // Stack trace
        if (self.stack_trace.items.len > 0) {
            try writer.print("\nStack trace:\n", .{});
            for (self.stack_trace.items) |frame| {
                try frame.format(writer);
            }
        }
        
        // Suggestions
        if (self.suggestions.items.len > 0) {
            try writer.print("\nSuggestions:\n", .{});
            for (self.suggestions.items) |suggestion| {
                try writer.print("  💡 {s}\n", .{suggestion});
            }
        }
        
        // Related errors
        if (self.related_errors.items.len > 0) {
            try writer.print("\nRelated errors:\n", .{});
            for (self.related_errors.items) |related| {
                try writer.print("  ", .{});
                try related.format(writer, use_colors);
            }
        }
        
        try writer.print("\n", .{});
    }
    
    pub fn toString(self: ErrorDiagnostic, use_colors: bool) ![]u8 {
        var buffer = ArrayList(u8).init(self.allocator);
        defer buffer.deinit();
        
        const writer = buffer.writer();
        try self.format(writer, use_colors);
        
        return try self.allocator.dupe(u8, buffer.items);
    }
};

pub const ErrorHandler = struct {
    allocator: Allocator,
    diagnostics: ArrayList(ErrorDiagnostic),
    current_file: ?[]const u8,
    source_lines: ?ArrayList([]const u8),
    function_stack: ArrayList(StackFrame),
    max_errors: usize,
    use_colors: bool,
    
    pub fn init(allocator: Allocator, max_errors: usize) ErrorHandler {
        return ErrorHandler{
            .allocator = allocator,
            .diagnostics = ArrayList(ErrorDiagnostic).init(allocator),
            .current_file = null,
            .source_lines = null,
            .function_stack = ArrayList(StackFrame).init(allocator),
            .max_errors = max_errors,
            .use_colors = std.io.tty.detectConfig(std.io.getStdErr()) != .no_color,
        };
    }
    
    pub fn deinit(self: *ErrorHandler) void {
        for (self.diagnostics.items) |*diagnostic| {
            diagnostic.deinit();
        }
        self.diagnostics.deinit();
        
        if (self.current_file) |file| {
            self.allocator.free(file);
        }
        
        if (self.source_lines) |*lines| {
            for (lines.items) |line| {
                self.allocator.free(line);
            }
            lines.deinit();
        }
        
        for (self.function_stack.items) |*frame| {
            frame.deinit(self.allocator);
        }
        self.function_stack.deinit();
    }
    
    pub fn setCurrentFile(self: *ErrorHandler, file_path: []const u8, source: []const u8) !void {
        if (self.current_file) |old_file| {
            self.allocator.free(old_file);
        }
        self.current_file = try self.allocator.dupe(u8, file_path);
        
        // Parse source into lines for better error reporting
        if (self.source_lines) |*old_lines| {
            for (old_lines.items) |line| {
                self.allocator.free(line);
            }
            old_lines.deinit();
        }
        
        self.source_lines = ArrayList([]const u8).init(self.allocator);
        var lines = std.mem.split(u8, source, "\n");
        while (lines.next()) |line| {
            try self.source_lines.?.append(try self.allocator.dupe(u8, line));
        }
    }
    
    pub fn pushFunction(self: *ErrorHandler, func_name: []const u8, line: u32, col: u32) !void {
        const frame = try StackFrame.init(
            self.allocator,
            func_name,
            self.current_file orelse "unknown",
            line,
            col,
            self.getSourceLine(line)
        );
        try self.function_stack.append(frame);
    }
    
    pub fn popFunction(self: *ErrorHandler) void {
        if (self.function_stack.items.len > 0) {
            var frame = self.function_stack.pop();
            frame.deinit(self.allocator);
        }
    }
    
    fn getSourceLine(self: *ErrorHandler, line_number: u32) ?[]const u8 {
        if (self.source_lines) |lines| {
            if (line_number > 0 and line_number <= lines.items.len) {
                return lines.items[line_number - 1];
            }
        }
        return null;
    }
    
    pub fn reportError(
        self: *ErrorHandler,
        severity: ErrorSeverity,
        message: []const u8,
        error_code: []const u8,
        line: u32,
        col: u32
    ) !void {
        if (self.diagnostics.items.len >= self.max_errors) {
            return; // Max errors reached
        }
        
        var diagnostic = try ErrorDiagnostic.init(
            self.allocator,
            severity,
            message,
            error_code,
            self.current_file orelse "unknown",
            line,
            col
        );
        
        // Set source line if available
        if (self.getSourceLine(line)) |source_line| {
            try diagnostic.setSourceLine(source_line);
        }
        
        // Copy current stack trace
        for (self.function_stack.items) |*frame| {
            const frame_copy = try StackFrame.init(
                self.allocator,
                frame.function_name,
                frame.file_path,
                frame.line_number,
                frame.column_number,
                frame.source_line
            );
            try diagnostic.addStackFrame(frame_copy);
        }
        
        try self.diagnostics.append(diagnostic);
    }
    
    pub fn reportYikesError(self: *ErrorHandler, message: []const u8, line: u32, col: u32) !void {
        try self.reportError(.Error, message, "YIKES", line, col);
        
        // Add relevant suggestions for yikes errors
        if (self.diagnostics.items.len > 0) {
            var last_diagnostic = &self.diagnostics.items[self.diagnostics.items.len - 1];
            try last_diagnostic.addSuggestion("Consider using 'shook' to propagate this error");
            try last_diagnostic.addSuggestion("Wrap in 'fam' block to handle the error");
        }
    }
    
    pub fn reportShookError(self: *ErrorHandler, message: []const u8, line: u32, col: u32) !void {
        try self.reportError(.Error, message, "SHOOK", line, col);
        
        if (self.diagnostics.items.len > 0) {
            var last_diagnostic = &self.diagnostics.items[self.diagnostics.items.len - 1];
            try last_diagnostic.addSuggestion("Ensure the expression can return an error");
            try last_diagnostic.addSuggestion("Use 'fam' block to catch propagated errors");
        }
    }
    
    pub fn reportFamError(self: *ErrorHandler, message: []const u8, line: u32, col: u32) !void {
        try self.reportError(.Error, message, "FAM", line, col);
        
        if (self.diagnostics.items.len > 0) {
            var last_diagnostic = &self.diagnostics.items[self.diagnostics.items.len - 1];
            try last_diagnostic.addSuggestion("Check that the error variable type matches the caught error");
            try last_diagnostic.addSuggestion("Ensure all error paths are handled");
        }
    }
    
    pub fn hasErrors(self: *ErrorHandler) bool {
        for (self.diagnostics.items) |diagnostic| {
            if (diagnostic.severity == .Error or diagnostic.severity == .Fatal) {
                return true;
            }
        }
        return false;
    }
    
    pub fn hasWarnings(self: *ErrorHandler) bool {
        for (self.diagnostics.items) |diagnostic| {
            if (diagnostic.severity == .Warning) {
                return true;
            }
        }
        return false;
    }
    
    pub fn getErrorCount(self: *ErrorHandler) usize {
        var count: usize = 0;
        for (self.diagnostics.items) |diagnostic| {
            if (diagnostic.severity == .Error or diagnostic.severity == .Fatal) {
                count += 1;
            }
        }
        return count;
    }
    
    pub fn getWarningCount(self: *ErrorHandler) usize {
        var count: usize = 0;
        for (self.diagnostics.items) |diagnostic| {
            if (diagnostic.severity == .Warning) {
                count += 1;
            }
        }
        return count;
    }
    
    pub fn printAllDiagnostics(self: *ErrorHandler) !void {
        const stderr = std.io.getStdErr().writer();
        
        for (self.diagnostics.items) |diagnostic| {
            try diagnostic.format(stderr, self.use_colors);
        }
        
        // Summary
        const error_count = self.getErrorCount();
        const warning_count = self.getWarningCount();
        
        if (error_count > 0 or warning_count > 0) {
            try stderr.print("Compilation summary: {} error(s), {} warning(s)\n", .{ error_count, warning_count });
        }
    }
    
    pub fn clear(self: *ErrorHandler) void {
        for (self.diagnostics.items) |*diagnostic| {
            diagnostic.deinit();
        }
        self.diagnostics.clearAndFree();
    }
};

test "error diagnostics system" {
    const allocator = std.testing.allocator;
    
    var handler = ErrorHandler.init(allocator, 100);
    defer handler.deinit();
    
    // Test setting current file
    const test_source = "slay test_func() {\n    yikes \"test error\"\n}";
    try handler.setCurrentFile("test.csd", test_source);
    
    // Test pushing function context
    try handler.pushFunction("test_func", 1, 1);
    
    // Test reporting an error
    try handler.reportYikesError("Test yikes error", 2, 5);
    
    // Verify error was recorded
    try std.testing.expect(handler.hasErrors());
    try std.testing.expect(handler.getErrorCount() == 1);
    
    // Test stack trace
    const diagnostic = &handler.diagnostics.items[0];
    try std.testing.expect(diagnostic.stack_trace.items.len == 1);
    try std.testing.expect(std.mem.eql(u8, diagnostic.stack_trace.items[0].function_name, "test_func"));
    
    // Test suggestions
    try std.testing.expect(diagnostic.suggestions.items.len > 0);
}
