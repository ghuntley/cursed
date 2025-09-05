const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;
const print = std.debug.print;

/// Memory-safe enhanced error reporting system for CURSED Zig compiler
/// Uses arena allocators and proper cleanup patterns to prevent memory leaks

pub const SourceLocation = struct {
    file: []const u8,
    line: u32,
    column: u32,
    char_offset: u32,
    
    pub fn init(file: []const u8, line: u32, column: u32, char_offset: u32) SourceLocation {
        return SourceLocation{
            .file = file,
            .line = line,
            .column = column,
            .char_offset = char_offset,
        };
    }
    
    pub fn format(self: SourceLocation, writer: anytype) !void {
        try writer.print("{s}:{s}:{s}", .{ self.file, self.line, self.column });
    }
};

pub const ErrorSeverity = enum {
    Note,
    Warning,
    Error,
    Fatal,
    
    pub fn toString(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Note => "note",
            .Warning => "warning", 
            .Error => "error",
            .Fatal => "fatal",
        };
    }
    
    pub fn color(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Note => "\x1b[36m",     // Cyan
            .Warning => "\x1b[33m",  // Yellow
            .Error => "\x1b[31m",    // Red
            .Fatal => "\x1b[35m",    // Magenta
        };
    }
};

pub const ErrorCode = enum {
    // Lexical errors
    E001_UnterminatedString,
    E002_InvalidCharacter,
    E003_InvalidNumber,
    E004_UnterminatedComment,
    E005_InvalidEscape,
    
    // Parse errors
    E101_UnexpectedToken,
    E102_ExpectedToken,
    E103_UnexpectedEOF,
    E104_InvalidSyntax,
    E105_MissingExpression,
    E106_InvalidPattern,
    E107_InvalidType,
    E108_UnbalancedBraces,
    E109_InvalidFunction,
    E110_InvalidParameter,
    
    // Semantic errors
    E201_UndefinedVariable,
    E202_UndefinedFunction,
    E203_TypeMismatch,
    E204_DuplicateDefinition,
    E205_CircularDependency,
    E206_InvalidAssignment,
    E207_UnreachableCode,
    E208_UndefinedField,
    E209_InterfaceNotImplemented,
    E210_InvalidCast,
    
    // Runtime errors
    E301_DivisionByZero,
    E302_IndexOutOfBounds,
    E303_NullDereference,
    E304_StackOverflow,
    E305_OutOfMemory,
    
    // Concurrency errors
    E401_ChannelClosed,
    E402_Deadlock,
    E403_RaceCondition,
    
    pub fn toString(self: ErrorCode) []const u8 {
        return @tagName(self);
    }
    
    pub fn description(self: ErrorCode) []const u8 {
        return switch (self) {
            .E001_UnterminatedString => "String literal is not properly terminated",
            .E002_InvalidCharacter => "Invalid character in source code",
            .E003_InvalidNumber => "Invalid number format",
            .E004_UnterminatedComment => "Comment block is not properly terminated",
            .E005_InvalidEscape => "Invalid escape sequence in string",
            
            .E101_UnexpectedToken => "Unexpected token encountered",
            .E102_ExpectedToken => "Expected specific token",
            .E103_UnexpectedEOF => "Unexpected end of file",
            .E104_InvalidSyntax => "Invalid syntax structure",
            .E105_MissingExpression => "Missing required expression",
            .E106_InvalidPattern => "Invalid pattern in match expression",
            .E107_InvalidType => "Invalid type specification",
            .E108_UnbalancedBraces => "Unbalanced braces, brackets, or parentheses",
            .E109_InvalidFunction => "Invalid function declaration",
            .E110_InvalidParameter => "Invalid function parameter",
            
            .E201_UndefinedVariable => "Variable is not defined in current scope",
            .E202_UndefinedFunction => "Function is not defined",
            .E203_TypeMismatch => "Type mismatch in expression",
            .E204_DuplicateDefinition => "Duplicate definition of symbol",
            .E205_CircularDependency => "Circular dependency detected",
            .E206_InvalidAssignment => "Invalid assignment operation",
            .E207_UnreachableCode => "Code is unreachable",
            .E208_UndefinedField => "Struct field is not defined",
            .E209_InterfaceNotImplemented => "Interface method not implemented",
            .E210_InvalidCast => "Invalid type cast operation",
            
            .E301_DivisionByZero => "Division by zero error",
            .E302_IndexOutOfBounds => "Array index out of bounds",
            .E303_NullDereference => "Null pointer dereference",
            .E304_StackOverflow => "Stack overflow detected",
            .E305_OutOfMemory => "Out of memory error",
            
            .E401_ChannelClosed => "Operation on closed channel",
            .E402_Deadlock => "Deadlock detected in concurrent code",
            .E403_RaceCondition => "Race condition detected",
        };
    }
};

pub const Suggestion = struct {
    message: []const u8,
    replacement: ?[]const u8,
    location: ?SourceLocation,
    
    pub fn init(message: []const u8) Suggestion {
        return Suggestion{
            .message = message,
            .replacement = null,
            .location = null,
        };
    }
    
    pub fn initWithReplacement(message: []const u8, replacement: []const u8) Suggestion {
        return Suggestion{
            .message = message,
            .replacement = replacement,
            .location = null,
        };
    }
};

/// Memory-safe diagnostic message using arena allocator
pub const DiagnosticMessage = struct {
    severity: ErrorSeverity,
    code: ErrorCode,
    message: []const u8,
    location: SourceLocation,
    suggestions: ArrayList(Suggestion),
    source_snippet: ?[]const u8,
    arena: *ArenaAllocator,
    
    pub fn init(
        arena: *ArenaAllocator,
        severity: ErrorSeverity,
        code: ErrorCode,
        message: []const u8,
        location: SourceLocation
    ) !DiagnosticMessage {
        const arena_allocator = arena.allocator();
        return DiagnosticMessage{
            .severity = severity,
            .code = code,
            .message = try arena_allocator.dupe(u8, message),
            .location = location,
            .suggestions = .empty,
            .source_snippet = null,
            .arena = arena,
        };
    }
    
    // No manual deinit needed - arena handles cleanup
    
    pub fn addSuggestion(self: *DiagnosticMessage, suggestion: Suggestion) !void {
        try self.suggestions.append(allocator, suggestion);
    }
    
    pub fn setSourceSnippet(self: *DiagnosticMessage, snippet: []const u8) !void {
        const arena_allocator = self.arena.allocator();
        self.source_snippet = try arena_allocator.dupe(u8, snippet);
    }
    
    pub fn format(self: DiagnosticMessage, writer: anytype, use_colors: bool) !void {
        // Error header with color
        if (use_colors) {
            try writer.print("{s}{s}:{s} {s}\x1b[0m: {s}\n", .{
                self.severity.color(),
                self.severity.toString(),
                self.code.toString(),
                self.location.file,
                self.message
            });
        } else {
            try writer.print("{s}:{s} {s}: {s}\n", .{
                self.severity.toString(),
                self.code.toString(),
                self.location.file,
                self.message
            });
        }
        
        // Location information
        try writer.print("  --> {s}:{d}:{d}\n", .{
            self.location.file,
            self.location.line,
            self.location.column
        });
        
        // Source snippet with error highlighting
        if (self.source_snippet) |snippet| {
            try self.formatSourceSnippet(writer, snippet, use_colors);
        }
        
        // Suggestions
        for (self.suggestions.items) |suggestion| {
            if (use_colors) {
                try writer.print("\x1b[36mhelp:\x1b[0m {s}\n", .{suggestion.message});
            } else {
                try writer.print("help: {s}\n", .{suggestion.message});
            }
            
            if (suggestion.replacement) |replacement| {
                if (use_colors) {
                    try writer.print("      try: \x1b[32m{s}\x1b[0m\n", .{replacement});
                } else {
                    try writer.print("      try: {s}\n", .{replacement});
                }
            }
        }
        
        try writer.print("\n", .{});
    }
    
    fn formatSourceSnippet(self: DiagnosticMessage, writer: anytype, snippet: []const u8, use_colors: bool) !void {
        const line_num_width = std.fmt.count("{d}", .{self.location.line});
        const padding = "    "; // 4 spaces
        
        // Line number and source
        try writer.print("{s}{d} | {s}\n", .{ padding, self.location.line, snippet });
        
        // Error pointer/caret
        const spaces_before_caret = self.location.column - 1;
        try writer.print("{s}", .{padding});
        
        // Print spaces for line number width
        var i: usize = 0;
        while (i < line_num_width) : (i += 1) {
            try writer.print(" ", .{});
        }
        try writer.print(" | ", .{});
        
        // Print spaces before caret
        i = 0;
        while (i < spaces_before_caret) : (i += 1) {
            try writer.print(" ", .{});
        }
        
        if (use_colors) {
            try writer.print("\x1b[31m^\x1b[0m error here\n", .{});
        } else {
            try writer.print("^ error here\n", .{});
        }
    }
};

/// Memory-safe error reporter using arena allocator
pub const ErrorReporter = struct {
    arena: ArenaAllocator,
    diagnostics: ArrayList(DiagnosticMessage),
    max_errors: usize,
    error_count: usize,
    warning_count: usize,
    use_colors: bool,
    verbose: bool,
    source_files: std.HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(backing_allocator: Allocator, max_errors: usize) ErrorReporter {
        var arena = ArenaAllocator.init(backing_allocator);
        const arena_allocator = arena.allocator();
        
        return ErrorReporter{
            .arena = arena,
            .diagnostics = .empty,
            .max_errors = max_errors,
            .error_count = 0,
            .warning_count = 0,
            .use_colors = true,
            .verbose = false,
            .source_files = std.HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *ErrorReporter) void {
        // Arena automatically cleans up all allocated memory
        self.arena.deinit(self.allocator);
    }
    
    pub fn addSourceFile(self: *ErrorReporter, file_path: []const u8, contents: []const u8) !void {
        const arena_allocator = self.arena.allocator();
        const path_copy = try arena_allocator.dupe(u8, file_path);
        const contents_copy = try arena_allocator.dupe(u8, contents);
        try self.source_files.put(path_copy, contents_copy);
    }
    
    pub fn reportError(
        self: *ErrorReporter,
        code: ErrorCode,
        message: []const u8,
        location: SourceLocation
    ) !void {
        try self.reportDiagnostic(.Error, code, message, location);
    }
    
    pub fn reportWarning(
        self: *ErrorReporter,
        code: ErrorCode,
        message: []const u8,
        location: SourceLocation
    ) !void {
        try self.reportDiagnostic(.Warning, code, message, location);
    }
    
    pub fn reportNote(
        self: *ErrorReporter,
        code: ErrorCode,
        message: []const u8,
        location: SourceLocation
    ) !void {
        try self.reportDiagnostic(.Note, code, message, location);
    }
    
    fn reportDiagnostic(
        self: *ErrorReporter,
        severity: ErrorSeverity,
        code: ErrorCode,
        message: []const u8,
        location: SourceLocation
    ) !void {
        if (severity == .Error) {
            self.error_count += 1;
            if (self.error_count > self.max_errors) {
                return; // Stop reporting after max errors
            }
        } else if (severity == .Warning) {
            self.warning_count += 1;
        }
        
        var diagnostic = try DiagnosticMessage.init(&self.arena, severity, code, message, location);
        
        // Add source snippet if available
        if (self.source_files.get(location.file)) |source| {
            const snippet = try self.extractSourceLine(source, location.line);
            try diagnostic.setSourceSnippet(snippet);
        }
        
        // Add helpful suggestions based on error code
        try self.addSuggestionsForError(&diagnostic);
        
        try self.diagnostics.append(allocator, diagnostic);
    }
    
    fn extractSourceLine(self: *ErrorReporter, source: []const u8, line_num: u32) ![]const u8 {
        const arena_allocator = self.arena.allocator();
        var current_line: u32 = 1;
        var line_start: usize = 0;
        
        for (source, 0..) |char, i| {
            if (char == '\n') {
                if (current_line == line_num) {
                    return try arena_allocator.dupe(u8, source[line_start..i]);
                }
                current_line += 1;
                line_start = i + 1;
            }
        }
        
        // Handle last line without newline
        if (current_line == line_num) {
            return try arena_allocator.dupe(u8, source[line_start..]);
        }
        
        return try arena_allocator.dupe(u8, "");
    }
    
    fn addSuggestionsForError(_: *ErrorReporter, diagnostic: *DiagnosticMessage) !void {
        switch (diagnostic.code) {
            .E101_UnexpectedToken => {
                try diagnostic.addSuggestion(Suggestion.init("Check for missing semicolons, commas, or brackets"));
                try diagnostic.addSuggestion(Suggestion.init("Verify proper CURSED keyword spelling (e.g., 'slay' for function)"));
            },
            .E102_ExpectedToken => {
                try diagnostic.addSuggestion(Suggestion.init("Add the missing token at the indicated position"));
            },
            .E103_UnexpectedEOF => {
                try diagnostic.addSuggestion(Suggestion.init("Check for unmatched braces, brackets, or quotes"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure all code blocks are properly closed"));
            },
            .E201_UndefinedVariable => {
                try diagnostic.addSuggestion(Suggestion.init("Check variable name spelling"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure variable is declared with 'sus' or 'facts'"));
                try diagnostic.addSuggestion(Suggestion.init("Check variable scope - variables are only accessible within their declaration scope"));
            },
            .E202_UndefinedFunction => {
                try diagnostic.addSuggestion(Suggestion.init("Check function name spelling"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure function is declared with 'slay'"));
                try diagnostic.addSuggestion(Suggestion.init("Check if the function is imported from another module"));
            },
            .E203_TypeMismatch => {
                try diagnostic.addSuggestion(Suggestion.init("Check type compatibility between assigned values"));
                try diagnostic.addSuggestion(Suggestion.init("Use explicit type conversion if needed"));
                try diagnostic.addSuggestion(Suggestion.init("CURSED types: normie (i32), tea (string), lit (bool), meal (f64)"));
            },
            .E204_DuplicateDefinition => {
                try diagnostic.addSuggestion(Suggestion.init("Use different names for variables/functions"));
                try diagnostic.addSuggestion(Suggestion.init("Check for redeclaration in the same scope"));
            },
            .E109_InvalidFunction => {
                try diagnostic.addSuggestion(Suggestion.init("Use 'slay' keyword for function declaration"));
                try diagnostic.addSuggestion(Suggestion.initWithReplacement("CURSED function syntax", "slay functionName(param type) returnType { ... }"));
            },
            .E001_UnterminatedString => {
                try diagnostic.addSuggestion(Suggestion.init("Add closing quote to string literal"));
                try diagnostic.addSuggestion(Suggestion.init("Check for unescaped quotes within the string"));
            },
            .E108_UnbalancedBraces => {
                try diagnostic.addSuggestion(Suggestion.init("Count opening and closing braces to find the mismatch"));
                try diagnostic.addSuggestion(Suggestion.init("Use proper indentation to visualize code structure"));
            },
            else => {
                // Generic suggestions
                try diagnostic.addSuggestion(Suggestion.init("Check the CURSED language documentation for correct syntax"));
                try diagnostic.addSuggestion(Suggestion.init("Review similar working code examples"));
            }
        }
    }
    
    pub fn hasErrors(self: *ErrorReporter) bool {
        return self.error_count > 0;
    }
    
    pub fn hasWarnings(self: *ErrorReporter) bool {
        return self.warning_count > 0;
    }
    
    pub fn getErrorCount(self: *ErrorReporter) usize {
        return self.error_count;
    }
    
    pub fn getWarningCount(self: *ErrorReporter) usize {
        return self.warning_count;
    }
    
    pub fn printDiagnostics(self: *ErrorReporter, writer: anytype) !void {
        for (self.diagnostics.items) |diagnostic| {
            try diagnostic.format(writer, self.use_colors);
        }
        
        // Summary
        if (self.error_count > 0 or self.warning_count > 0) {
            if (self.use_colors) {
                if (self.error_count > 0) {
                    try writer.print("\x1b[31mCompilation failed\x1b[0m with {d} error(s)", .{self.error_count});
                } else {
                    try writer.print("\x1b[33mCompilation completed\x1b[0m with {d} warning(s)", .{self.warning_count});
                }
            } else {
                if (self.error_count > 0) {
                    try writer.print("Compilation failed with {d} error(s)", .{self.error_count});
                } else {
                    try writer.print("Compilation completed with {d} warning(s)", .{self.warning_count});
                }
            }
            
            if (self.warning_count > 0 and self.error_count > 0) {
                try writer.print(" and {d} warning(s)", .{self.warning_count});
            }
            try writer.print("\n", .{});
        }
    }
    
    pub fn setColors(self: *ErrorReporter, enabled: bool) void {
        self.use_colors = enabled;
    }
    
    pub fn setVerbose(self: *ErrorReporter, enabled: bool) void {
        self.verbose = enabled;
    }
};

/// Memory-safe debug information using arena allocator
pub const DebugInfo = struct {
    pub const DebugLevel = enum {
        None,
        Minimal,
        Full,
    };
    
    level: DebugLevel,
    line_table: ArrayList(LineInfo),
    scope_table: ArrayList(ScopeInfo),
    variable_table: ArrayList(VariableInfo),
    arena: *ArenaAllocator,
    
    pub const LineInfo = struct {
        line: u32,
        column: u32,
        file_path: []const u8,
        instruction_offset: u32,
    };
    
    pub const ScopeInfo = struct {
        start_offset: u32,
        end_offset: u32,
        parent_scope: ?u32,
        scope_type: ScopeType,
    };
    
    pub const ScopeType = enum {
        Function,
        Block,
        Loop,
        Conditional,
    };
    
    pub const VariableInfo = struct {
        name: []const u8,
        type_name: []const u8,
        scope_id: u32,
        line_declared: u32,
        is_parameter: bool,
    };
    
    pub fn init(arena: *ArenaAllocator, level: DebugLevel) DebugInfo {
        const arena_allocator = arena.allocator();
        return DebugInfo{
            .level = level,
            .line_table = .empty,
            .scope_table = .empty,
            .variable_table = .empty,
            .arena = arena,
        };
    }
    
    // No manual deinit needed - arena handles cleanup
    
    pub fn addLineInfo(self: *DebugInfo, line: u32, column: u32, file_path: []const u8, offset: u32) !void {
        if (self.level == .None) return;
        
        try self.line_table.append(LineInfo{
            .line = line,
            .column = column,
            .file_path = file_path,
            .instruction_offset = offset,
        });
    }
    
    pub fn addVariableInfo(self: *DebugInfo, name: []const u8, type_name: []const u8, scope_id: u32, line: u32, is_param: bool) !void {
        if (self.level != .Full) return;
        
        const arena_allocator = self.arena.allocator();
        try self.variable_table.append(VariableInfo{
            .name = try arena_allocator.dupe(u8, name),
            .type_name = try arena_allocator.dupe(u8, type_name),
            .scope_id = scope_id,
            .line_declared = line,
            .is_parameter = is_param,
        });
    }
    
    pub fn enterScope(self: *DebugInfo, scope_type: ScopeType, start_offset: u32, parent: ?u32) !u32 {
        if (self.level == .None) return 0;
        
        const scope_id = self.scope_table.items.len;
        try self.scope_table.append(ScopeInfo{
            .start_offset = start_offset,
            .end_offset = 0, // Will be set when scope ends
            .parent_scope = parent,
            .scope_type = scope_type,
        });
        
        return @intCast(scope_id);
    }
    
    pub fn exitScope(self: *DebugInfo, scope_id: u32, end_offset: u32) void {
        if (self.level == .None) return;
        if (scope_id < self.scope_table.items.len) {
            self.scope_table.items[scope_id].end_offset = end_offset;
        }
    }
};

/// Logger with built-in memory safety
pub const Logger = struct {
    level: LogLevel,
    writer: std.fs.File.Writer,
    use_colors: bool,
    
    pub const LogLevel = enum(u8) {
        Silent = 0,
        Error = 1,
        Warning = 2,
        Info = 3,
        Debug = 4,
        Trace = 5,
        
        pub fn toString(self: LogLevel) []const u8 {
            return switch (self) {
                .Silent => "SILENT",
                .Error => "ERROR",
                .Warning => "WARN ",
                .Info => "INFO ",
                .Debug => "DEBUG",
                .Trace => "TRACE",
            };
        }
        
        pub fn color(self: LogLevel) []const u8 {
            return switch (self) {
                .Silent => "",
                .Error => "\x1b[31m",   // Red
                .Warning => "\x1b[33m", // Yellow
                .Info => "\x1b[32m",    // Green
                .Debug => "\x1b[36m",   // Cyan
                .Trace => "\x1b[37m",   // White
            };
        }
    };
    
    pub fn init(writer: std.fs.File.Writer, level: LogLevel, use_colors: bool) Logger {
        return Logger{
            .level = level,
            .writer = writer,
            .use_colors = use_colors,
        };
    }
    
    pub fn log(self: *Logger, level: LogLevel, comptime format: []const u8, args: anytype) void {
        if (@intFromEnum(level) > @intFromEnum(self.level)) return;
        
        const timestamp = std.time.timestamp();
        
        if (self.use_colors) {
            self.writer.print("{s}[{s}]\x1b[0m {d}: ", .{ level.color(), level.toString(), timestamp }) catch return;
        } else {
            self.writer.print("[{s}] {d}: ", .{ level.toString(), timestamp }) catch return;
        }
        
        self.writer.print(format ++ "\n", args) catch return;
    }
    
    pub fn err(self: *Logger, comptime format: []const u8, args: anytype) void {
        self.log(.Error, format, args);
    }
    
    pub fn warning(self: *Logger, comptime format: []const u8, args: anytype) void {
        self.log(.Warning, format, args);
    }
    
    pub fn info(self: *Logger, comptime format: []const u8, args: anytype) void {
        self.log(.Info, format, args);
    }
    
    pub fn debug(self: *Logger, comptime format: []const u8, args: anytype) void {
        self.log(.Debug, format, args);
    }
    
    pub fn trace(self: *Logger, comptime format: []const u8, args: anytype) void {
        self.log(.Trace, format, args);
    }
};

// Testing
test "memory safe error reporting system" {
    const allocator = std.testing.allocator;
    
    // Test error reporter
    var reporter = ErrorReporter.init(allocator, 10);
    defer reporter.deinit();
    
    const location = SourceLocation.init("test.💀", 1, 5, 4);
    
    try reporter.reportError(.E201_UndefinedVariable, "Variable 'x' is not defined", location);
    try reporter.reportWarning(.E203_TypeMismatch, "Implicit conversion may lose precision", location);
    
    try std.testing.expect(reporter.hasErrors());
    try std.testing.expect(reporter.hasWarnings());
    try std.testing.expect(reporter.getErrorCount() == 1);
    try std.testing.expect(reporter.getWarningCount() == 1);
    
    // Test debug info
    var debug_arena = ArenaAllocator.init(allocator);
    defer debug_arena.deinit();
    
    var debug_info = DebugInfo.init(&debug_arena, .Full);
    
    try debug_info.addLineInfo(1, 5, "test.💀", 100);
    const scope_id = try debug_info.enterScope(.Function, 100, null);
    debug_info.exitScope(scope_id, 200);
    
    try std.testing.expect(debug_info.line_table.items.len == 1);
    try std.testing.expect(debug_info.scope_table.items.len == 1);
}
