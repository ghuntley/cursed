const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

/// Comprehensive error diagnostics system for CURSED compiler
/// Provides source location tracking, colorized output, multi-line highlighting,
/// helpful suggestions, and error codes for tooling integration

pub const SourceSpan = struct {
    file: []const u8,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
    start_offset: u32,
    end_offset: u32,
    
    pub fn init(file: []const u8, start_line: u32, start_column: u32, 
                end_line: u32, end_column: u32, start_offset: u32, end_offset: u32) SourceSpan {
        return SourceSpan{
            .file = file,
            .start_line = start_line,
            .start_column = start_column,
            .end_line = end_line,
            .end_column = end_column,
            .start_offset = start_offset,
            .end_offset = end_offset,
        };
    }
    
    pub fn fromSinglePosition(file: []const u8, line: u32, column: u32, offset: u32) SourceSpan {
        return init(file, line, column, line, column + 1, offset, offset + 1);
    }
    
    pub fn isMultiLine(self: SourceSpan) bool {
        return self.start_line != self.end_line;
    }
    
    pub fn length(self: SourceSpan) u32 {
        return self.end_offset - self.start_offset;
    }
};

pub const ErrorSeverity = enum {
    Note,
    Hint,
    Warning,
    Error,
    Fatal,
    
    pub fn toString(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Note => "note",
            .Hint => "hint",
            .Warning => "warning", 
            .Error => "error",
            .Fatal => "fatal error",
        };
    }
    
    pub fn color(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Note => "\x1b[96m",     // Bright Cyan
            .Hint => "\x1b[92m",     // Bright Green
            .Warning => "\x1b[93m",  // Bright Yellow
            .Error => "\x1b[91m",    // Bright Red
            .Fatal => "\x1b[95m",    // Bright Magenta
        };
    }
    
    pub fn icon(self: ErrorSeverity) []const u8 {
        return switch (self) {
            .Note => "💡",
            .Hint => "💡",
            .Warning => "⚠️ ",
            .Error => "❌",
            .Fatal => "💀",
        };
    }
};

pub const ErrorCode = enum {
    // Lexical errors (L001-L099)
    L001_UnterminatedString,
    L002_InvalidCharacter,
    L003_InvalidNumber,
    L004_UnterminatedComment,
    L005_InvalidEscape,
    L006_InvalidUnicode,
    L007_NumberOverflow,
    L008_InvalidFloatFormat,
    L009_UnexpectedCharacter,
    
    // Parse errors (P001-P199)
    P001_UnexpectedToken,
    P002_ExpectedToken,
    P003_UnexpectedEOF,
    P004_InvalidSyntax,
    P005_MissingExpression,
    P006_InvalidPattern,
    P007_InvalidType,
    P008_UnbalancedBraces,
    P009_InvalidFunction,
    P010_InvalidParameter,
    P011_InvalidStatement,
    P012_InvalidDeclaration,
    P013_MissingBrace,
    P014_MissingParen,
    P015_MissingSemicolon,
    P016_InvalidStructField,
    P017_InvalidInterfaceMethod,
    P018_InvalidGenericParameter,
    P019_InvalidConstraint,
    P020_InvalidImport,
    
    // Semantic errors (S001-S299)
    S001_UndefinedVariable,
    S002_UndefinedFunction,
    S003_TypeMismatch,
    S004_DuplicateDefinition,
    S005_CircularDependency,
    S006_InvalidAssignment,
    S007_UnreachableCode,
    S008_UndefinedField,
    S009_InterfaceNotImplemented,
    S010_InvalidCast,
    S011_InvalidOperation,
    S012_IncompatibleTypes,
    S013_MissingMethod,
    S014_InvalidMethodSignature,
    S015_RecursiveType,
    S016_InvalidGenericInstantiation,
    S017_ConstraintViolation,
    S018_ScopeError,
    S019_VisibilityError,
    S020_ImmutableAssignment,
    
    // Runtime errors (R001-R099)
    R001_DivisionByZero,
    R002_IndexOutOfBounds,
    R003_NullDereference,
    R004_StackOverflow,
    R005_OutOfMemory,
    R006_InvalidCast,
    R007_AssertionFailed,
    
    // Concurrency errors (C001-C099)
    C001_ChannelClosed,
    C002_Deadlock,
    C003_RaceCondition,
    C004_InvalidChannelOp,
    C005_TimeoutExpired,
    
    // Module system errors (M001-M099)
    M001_ModuleNotFound,
    M002_CyclicImport,
    M003_InvalidExport,
    M004_VersionConflict,
    M005_DependencyError,
    
    pub fn toString(self: ErrorCode) []const u8 {
        return @tagName(self);
    }
    
    pub fn getPrefix(self: ErrorCode) []const u8 {
        const name = @tagName(self);
        return name[0..4]; // L001, P001, etc.
    }
    
    pub fn description(self: ErrorCode) []const u8 {
        return switch (self) {
            .L001_UnterminatedString => "String literal is not properly terminated",
            .L002_InvalidCharacter => "Invalid character in source code",
            .L003_InvalidNumber => "Invalid number format",
            .L004_UnterminatedComment => "Comment block is not properly terminated",
            .L005_InvalidEscape => "Invalid escape sequence in string",
            .L006_InvalidUnicode => "Invalid Unicode escape sequence",
            .L007_NumberOverflow => "Number literal exceeds maximum value",
            .L008_InvalidFloatFormat => "Invalid floating-point number format",
            .L009_UnexpectedCharacter => "Unexpected character in input",
            
            .P001_UnexpectedToken => "Unexpected token encountered",
            .P002_ExpectedToken => "Expected specific token",
            .P003_UnexpectedEOF => "Unexpected end of file",
            .P004_InvalidSyntax => "Invalid syntax structure",
            .P005_MissingExpression => "Missing required expression",
            .P006_InvalidPattern => "Invalid pattern in match expression",
            .P007_InvalidType => "Invalid type specification",
            .P008_UnbalancedBraces => "Unbalanced braces, brackets, or parentheses",
            .P009_InvalidFunction => "Invalid function declaration",
            .P010_InvalidParameter => "Invalid function parameter",
            .P011_InvalidStatement => "Invalid statement syntax",
            .P012_InvalidDeclaration => "Invalid declaration syntax",
            .P013_MissingBrace => "Missing opening or closing brace",
            .P014_MissingParen => "Missing opening or closing parenthesis",
            .P015_MissingSemicolon => "Missing semicolon",
            .P016_InvalidStructField => "Invalid struct field declaration",
            .P017_InvalidInterfaceMethod => "Invalid interface method declaration",
            .P018_InvalidGenericParameter => "Invalid generic parameter",
            .P019_InvalidConstraint => "Invalid generic constraint",
            .P020_InvalidImport => "Invalid import statement",
            
            .S001_UndefinedVariable => "Variable is not defined in current scope",
            .S002_UndefinedFunction => "Function is not defined",
            .S003_TypeMismatch => "Type mismatch in expression",
            .S004_DuplicateDefinition => "Duplicate definition of symbol",
            .S005_CircularDependency => "Circular dependency detected",
            .S006_InvalidAssignment => "Invalid assignment operation",
            .S007_UnreachableCode => "Code is unreachable",
            .S008_UndefinedField => "Struct field is not defined",
            .S009_InterfaceNotImplemented => "Interface method not implemented",
            .S010_InvalidCast => "Invalid type cast operation",
            .S011_InvalidOperation => "Invalid operation for given types",
            .S012_IncompatibleTypes => "Types are not compatible",
            .S013_MissingMethod => "Required method is missing",
            .S014_InvalidMethodSignature => "Method signature does not match interface",
            .S015_RecursiveType => "Recursive type definition without indirection",
            .S016_InvalidGenericInstantiation => "Invalid generic type instantiation",
            .S017_ConstraintViolation => "Generic constraint violation",
            .S018_ScopeError => "Variable accessed outside its scope",
            .S019_VisibilityError => "Symbol is not visible in current context",
            .S020_ImmutableAssignment => "Cannot assign to immutable variable",
            
            .R001_DivisionByZero => "Division by zero error",
            .R002_IndexOutOfBounds => "Array index out of bounds",
            .R003_NullDereference => "Null pointer dereference",
            .R004_StackOverflow => "Stack overflow detected",
            .R005_OutOfMemory => "Out of memory error",
            .R006_InvalidCast => "Invalid runtime type cast",
            .R007_AssertionFailed => "Assertion failed",
            
            .C001_ChannelClosed => "Operation on closed channel",
            .C002_Deadlock => "Deadlock detected in concurrent code",
            .C003_RaceCondition => "Race condition detected",
            .C004_InvalidChannelOp => "Invalid channel operation",
            .C005_TimeoutExpired => "Operation timed out",
            
            .M001_ModuleNotFound => "Module not found",
            .M002_CyclicImport => "Cyclic import detected",
            .M003_InvalidExport => "Invalid export declaration",
            .M004_VersionConflict => "Version conflict in dependencies",
            .M005_DependencyError => "Dependency resolution error",
        };
    }
};

pub const Suggestion = struct {
    message: []const u8,
    replacement: ?[]const u8,
    span: ?SourceSpan,
    
    pub fn init(message: []const u8) Suggestion {
        return Suggestion{
            .message = message,
            .replacement = null,
            .span = null,
        };
    }
    
    pub fn initWithReplacement(message: []const u8, replacement: []const u8) Suggestion {
        return Suggestion{
            .message = message,
            .replacement = replacement,
            .span = null,
        };
    }
    
    pub fn initWithSpan(message: []const u8, span: SourceSpan) Suggestion {
        return Suggestion{
            .message = message,
            .replacement = null,
            .span = span,
        };
    }
};

pub const RelatedInfo = struct {
    span: SourceSpan,
    message: []const u8,
    
    pub fn init(span: SourceSpan, message: []const u8) RelatedInfo {
        return RelatedInfo{
            .span = span,
            .message = message,
        };
    }
};

pub const Diagnostic = struct {
    severity: ErrorSeverity,
    code: ErrorCode,
    message: []const u8,
    primary_span: SourceSpan,
    secondary_spans: ArrayList(SourceSpan),
    suggestions: ArrayList(Suggestion),
    related: ArrayList(RelatedInfo),
    allocator: Allocator,
    
    pub fn init(
        allocator: Allocator,
        severity: ErrorSeverity,
        code: ErrorCode,
        message: []const u8,
        span: SourceSpan
    ) !Diagnostic {
        return Diagnostic{
            .severity = severity,
            .code = code,
            .message = try allocator.dupe(u8, message),
            .primary_span = span,
            .secondary_spans = ArrayList(SourceSpan).init(allocator),
            .suggestions = ArrayList(Suggestion).init(allocator),
            .related = ArrayList(RelatedInfo).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Diagnostic) void {
        self.allocator.free(self.message);
        self.secondary_spans.deinit();
        self.suggestions.deinit();
        self.related.deinit();
    }
    
    pub fn addSecondarySpan(self: *Diagnostic, span: SourceSpan) !void {
        try self.secondary_spans.append(span);
    }
    
    pub fn addSuggestion(self: *Diagnostic, suggestion: Suggestion) !void {
        try self.suggestions.append(suggestion);
    }
    
    pub fn addRelated(self: *Diagnostic, related: RelatedInfo) !void {
        try self.related.append(related);
    }
};

pub const DiagnosticEngine = struct {
    diagnostics: ArrayList(Diagnostic),
    allocator: Allocator,
    max_errors: usize,
    error_count: usize,
    warning_count: usize,
    use_colors: bool,
    use_unicode: bool,
    verbose: bool,
    source_files: std.HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, max_errors: usize) DiagnosticEngine {
        return DiagnosticEngine{
            .diagnostics = ArrayList(Diagnostic).init(allocator),
            .allocator = allocator,
            .max_errors = max_errors,
            .error_count = 0,
            .warning_count = 0,
            .use_colors = std.posix.isatty(std.io.getStdErr().handle),
            .use_unicode = true,
            .verbose = false,
            .source_files = std.HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *DiagnosticEngine) void {
        for (self.diagnostics.items) |*diagnostic| {
            diagnostic.deinit();
        }
        self.diagnostics.deinit();
        
        var iterator = self.source_files.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.source_files.deinit();
    }
    
    pub fn addSourceFile(self: *DiagnosticEngine, file_path: []const u8, contents: []const u8) !void {
        const path_copy = try self.allocator.dupe(u8, file_path);
        const contents_copy = try self.allocator.dupe(u8, contents);
        try self.source_files.put(path_copy, contents_copy);
    }
    
    pub fn reportError(self: *DiagnosticEngine, code: ErrorCode, message: []const u8, span: SourceSpan) !void {
        return self.report(.Error, code, message, span);
    }
    
    pub fn reportWarning(self: *DiagnosticEngine, code: ErrorCode, message: []const u8, span: SourceSpan) !void {
        return self.report(.Warning, code, message, span);
    }
    
    pub fn reportNote(self: *DiagnosticEngine, code: ErrorCode, message: []const u8, span: SourceSpan) !void {
        return self.report(.Note, code, message, span);
    }
    
    pub fn reportHint(self: *DiagnosticEngine, code: ErrorCode, message: []const u8, span: SourceSpan) !void {
        return self.report(.Hint, code, message, span);
    }
    
    fn report(self: *DiagnosticEngine, severity: ErrorSeverity, code: ErrorCode, message: []const u8, span: SourceSpan) !void {
        if (severity == .Error or severity == .Fatal) {
            self.error_count += 1;
            if (self.error_count > self.max_errors) {
                return; // Stop reporting after max errors
            }
        } else if (severity == .Warning) {
            self.warning_count += 1;
        }
        
        var diagnostic = try Diagnostic.init(self.allocator, severity, code, message, span);
        
        // Add helpful suggestions based on error code
        try self.addSuggestionsForError(&diagnostic);
        
        try self.diagnostics.append(diagnostic);
    }
    
    fn addSuggestionsForError(self: *DiagnosticEngine, diagnostic: *Diagnostic) !void {
        _ = self; // unused
        
        switch (diagnostic.code) {
            .P001_UnexpectedToken => {
                try diagnostic.addSuggestion(Suggestion.init("Check for missing semicolons, commas, or brackets"));
                try diagnostic.addSuggestion(Suggestion.init("Verify proper CURSED keyword spelling (e.g., 'slay' for function)"));
            },
            .P002_ExpectedToken => {
                try diagnostic.addSuggestion(Suggestion.init("Add the missing token at the indicated position"));
            },
            .P003_UnexpectedEOF => {
                try diagnostic.addSuggestion(Suggestion.init("Check for unmatched braces, brackets, or quotes"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure all code blocks are properly closed"));
            },
            .S001_UndefinedVariable => {
                try diagnostic.addSuggestion(Suggestion.init("Check variable name spelling"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure variable is declared with 'sus' or 'facts'"));
                try diagnostic.addSuggestion(Suggestion.init("Check variable scope - variables are only accessible within their declaration scope"));
            },
            .S002_UndefinedFunction => {
                try diagnostic.addSuggestion(Suggestion.init("Check function name spelling"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure function is declared with 'slay'"));
                try diagnostic.addSuggestion(Suggestion.init("Check if the function is imported from another module"));
            },
            .S003_TypeMismatch => {
                try diagnostic.addSuggestion(Suggestion.init("Check type compatibility between assigned values"));
                try diagnostic.addSuggestion(Suggestion.init("Use explicit type conversion if needed"));
                try diagnostic.addSuggestion(Suggestion.init("CURSED types: normie (i32), tea (string), lit (bool), meal (f64)"));
            },
            .S004_DuplicateDefinition => {
                try diagnostic.addSuggestion(Suggestion.init("Use different names for variables/functions"));
                try diagnostic.addSuggestion(Suggestion.init("Check for redeclaration in the same scope"));
            },
            .P009_InvalidFunction => {
                try diagnostic.addSuggestion(Suggestion.init("Use 'slay' keyword for function declaration"));
                try diagnostic.addSuggestion(Suggestion.initWithReplacement("CURSED function syntax", "slay functionName(param type) returnType { ... }"));
            },
            .L001_UnterminatedString => {
                try diagnostic.addSuggestion(Suggestion.init("Add closing quote to string literal"));
                try diagnostic.addSuggestion(Suggestion.init("Check for unescaped quotes within the string"));
            },
            .P008_UnbalancedBraces => {
                try diagnostic.addSuggestion(Suggestion.init("Count opening and closing braces to find the mismatch"));
                try diagnostic.addSuggestion(Suggestion.init("Use proper indentation to visualize code structure"));
            },
            .S020_ImmutableAssignment => {
                try diagnostic.addSuggestion(Suggestion.init("Use 'sus' instead of 'facts' for mutable variables"));
                try diagnostic.addSuggestion(Suggestion.init("Create a new variable instead of reassigning"));
            },
            .P006_InvalidPattern => {
                try diagnostic.addSuggestion(Suggestion.init("Check pattern matching syntax: 'match value { pattern => action }'"));
                try diagnostic.addSuggestion(Suggestion.init("Ensure all patterns are exhaustive"));
            },
            .S009_InterfaceNotImplemented => {
                try diagnostic.addSuggestion(Suggestion.init("Implement all required interface methods"));
                try diagnostic.addSuggestion(Suggestion.init("Use 'impl Interface slay Type' to implement interface"));
            },
            else => {
                // Generic suggestions
                try diagnostic.addSuggestion(Suggestion.init("Check the CURSED language documentation for correct syntax"));
                try diagnostic.addSuggestion(Suggestion.init("Review similar working code examples"));
            }
        }
    }
    
    pub fn hasErrors(self: *DiagnosticEngine) bool {
        return self.error_count > 0;
    }
    
    pub fn hasWarnings(self: *DiagnosticEngine) bool {
        return self.warning_count > 0;
    }
    
    pub fn getErrorCount(self: *DiagnosticEngine) usize {
        return self.error_count;
    }
    
    pub fn getWarningCount(self: *DiagnosticEngine) usize {
        return self.warning_count;
    }
    
    pub fn setColors(self: *DiagnosticEngine, enabled: bool) void {
        self.use_colors = enabled;
    }
    
    pub fn setUnicode(self: *DiagnosticEngine, enabled: bool) void {
        self.use_unicode = enabled;
    }
    
    pub fn setVerbose(self: *DiagnosticEngine, enabled: bool) void {
        self.verbose = enabled;
    }
    
    pub fn printDiagnostics(self: *DiagnosticEngine, writer: anytype) !void {
        for (self.diagnostics.items) |diagnostic| {
            try self.printDiagnostic(writer, diagnostic);
        }
        
        try self.printSummary(writer);
    }
    
    fn printDiagnostic(self: *DiagnosticEngine, writer: anytype, diagnostic: Diagnostic) !void {
        const reset = if (self.use_colors) "\x1b[0m" else "";
        const bold = if (self.use_colors) "\x1b[1m" else "";
        
        // Error header with color and icon
        if (self.use_colors and self.use_unicode) {
            try writer.print("{s}{s}{s}{s}: {s}[{s}]{s} {s}\n", .{
                diagnostic.severity.color(),
                bold,
                diagnostic.severity.icon(),
                diagnostic.severity.toString(),
                reset,
                diagnostic.code.toString(),
                diagnostic.severity.color(),
                diagnostic.message
            });
        } else if (self.use_colors) {
            try writer.print("{s}{s}{s}:{s} {s}[{s}]{s} {s}\n", .{
                diagnostic.severity.color(),
                bold,
                diagnostic.severity.toString(),
                reset,
                bold,
                diagnostic.code.toString(),
                reset,
                diagnostic.message
            });
        } else {
            try writer.print("{s}: [{s}] {s}\n", .{
                diagnostic.severity.toString(),
                diagnostic.code.toString(),
                diagnostic.message
            });
        }
        
        // Primary span
        try self.printSourceSpan(writer, diagnostic.primary_span, true);
        
        // Secondary spans
        for (diagnostic.secondary_spans.items) |span| {
            try self.printSourceSpan(writer, span, false);
        }
        
        // Related information
        for (diagnostic.related.items) |related| {
            if (self.use_colors) {
                try writer.print("{s}note:{s} {s}\n", .{ "\x1b[96m", reset, related.message });
            } else {
                try writer.print("note: {s}\n", .{related.message});
            }
            try self.printSourceSpan(writer, related.span, false);
        }
        
        // Suggestions
        for (diagnostic.suggestions.items) |suggestion| {
            const help_color = if (self.use_colors) "\x1b[96m" else "";
            const try_color = if (self.use_colors) "\x1b[92m" else "";
            
            try writer.print("{s}help:{s} {s}\n", .{ help_color, reset, suggestion.message });
            
            if (suggestion.replacement) |replacement| {
                try writer.print("      {s}try:{s} {s}{s}{s}\n", .{ 
                    help_color, reset, try_color, replacement, reset 
                });
            }
        }
        
        try writer.print("\n", .{});
    }
    
    fn printSourceSpan(self: *DiagnosticEngine, writer: anytype, span: SourceSpan, is_primary: bool) !void {
        const source = self.source_files.get(span.file) orelse return;
        
        // Location header
        try writer.print("  {s}--> {s}:{d}:{d}\n", .{
            if (self.use_colors) "\x1b[36m" else "",
            span.file,
            span.start_line,
            span.start_column
        });
        
        if (span.isMultiLine()) {
            try self.printMultiLineSpan(writer, source, span, is_primary);
        } else {
            try self.printSingleLineSpan(writer, source, span, is_primary);
        }
    }
    
    fn printSingleLineSpan(self: *DiagnosticEngine, writer: anytype, source: []const u8, span: SourceSpan, is_primary: bool) !void {
        const line_content = self.extractSourceLine(source, span.start_line) catch return;
        defer self.allocator.free(line_content);
        
        const line_num_width = std.fmt.count("{d}", .{span.start_line});
        const padding = "   ";
        const reset = if (self.use_colors) "\x1b[0m" else "";
        const line_color = if (self.use_colors) "\x1b[36m" else "";
        const highlight_color = if (is_primary) 
            (if (self.use_colors) "\x1b[91m" else "") 
        else 
            (if (self.use_colors) "\x1b[94m" else "");
        
        // Empty line
        try writer.print("{s}{s}", .{ padding, line_color });
        var i: usize = 0;
        while (i < line_num_width) : (i += 1) {
            try writer.print(" ", .{});
        }
        try writer.print(" |{s}\n", .{reset});
        
        // Line with content
        try writer.print("{s}{s}{d} |{s} {s}\n", .{
            padding, line_color, span.start_line, reset, line_content
        });
        
        // Highlight line
        try writer.print("{s}{s}", .{ padding, line_color });
        i = 0;
        while (i < line_num_width) : (i += 1) {
            try writer.print(" ", .{});
        }
        try writer.print(" |{s} ", .{reset});
        
        // Spaces before highlight
        const spaces_before = @min(span.start_column - 1, line_content.len);
        i = 0;
        while (i < spaces_before) : (i += 1) {
            try writer.print(" ", .{});
        }
        
        // Highlight characters
        const highlight_len = @min(span.length(), line_content.len - spaces_before);
        try writer.print("{s}", .{highlight_color});
        
        if (highlight_len == 1) {
            try writer.print("^", .{});
        } else {
            i = 0;
            while (i < highlight_len) : (i += 1) {
                try writer.print("~", .{});
            }
        }
        
        try writer.print("{s}", .{reset});
        
        if (is_primary) {
            try writer.print(" {s}here{s}", .{ highlight_color, reset });
        }
        
        try writer.print("\n", .{});
    }
    
    fn printMultiLineSpan(self: *DiagnosticEngine, writer: anytype, source: []const u8, span: SourceSpan, is_primary: bool) !void {
        _ = is_primary; // TODO: implement multi-line highlighting
        
        const start_line_content = self.extractSourceLine(source, span.start_line) catch return;
        defer self.allocator.free(start_line_content);
        
        const end_line_content = self.extractSourceLine(source, span.end_line) catch return;
        defer self.allocator.free(end_line_content);
        
        const line_num_width = std.fmt.count("{d}", .{span.end_line});
        const padding = "   ";
        const reset = if (self.use_colors) "\x1b[0m" else "";
        const line_color = if (self.use_colors) "\x1b[36m" else "";
        const highlight_color = if (self.use_colors) "\x1b[91m" else "";
        
        // Start line
        try writer.print("{s}{s}{d} |{s} {s}\n", .{
            padding, line_color, span.start_line, reset, start_line_content
        });
        
        // Show middle lines if not too many
        if (span.end_line - span.start_line <= 5) {
            var line: u32 = span.start_line + 1;
            while (line < span.end_line) : (line += 1) {
                const line_content = self.extractSourceLine(source, line) catch continue;
                defer self.allocator.free(line_content);
                try writer.print("{s}{s}{d} |{s} {s}\n", .{
                    padding, line_color, line, reset, line_content
                });
            }
        } else {
            // Show ellipsis for too many lines
            try writer.print("{s}{s}...{s}\n", .{ padding, line_color, reset });
        }
        
        // End line
        try writer.print("{s}{s}{d} |{s} {s}\n", .{
            padding, line_color, span.end_line, reset, end_line_content
        });
        
        // Highlight indicator
        try writer.print("{s}{s}", .{ padding, line_color });
        var i: usize = 0;
        while (i < line_num_width) : (i += 1) {
            try writer.print(" ", .{});
        }
        try writer.print(" |{s} {s}^^^^ spanning multiple lines{s}\n", .{
            reset, highlight_color, reset
        });
    }
    
    fn extractSourceLine(self: *DiagnosticEngine, source: []const u8, line_num: u32) ![]const u8 {
        var current_line: u32 = 1;
        var line_start: usize = 0;
        
        for (source, 0..) |char, i| {
            if (char == '\n') {
                if (current_line == line_num) {
                    return try self.allocator.dupe(u8, source[line_start..i]);
                }
                current_line += 1;
                line_start = i + 1;
            }
        }
        
        // Handle last line without newline
        if (current_line == line_num) {
            return try self.allocator.dupe(u8, source[line_start..]);
        }
        
        return try self.allocator.dupe(u8, "");
    }
    
    fn printSummary(self: *DiagnosticEngine, writer: anytype) !void {
        if (self.error_count == 0 and self.warning_count == 0) {
            return;
        }
        
        const reset = if (self.use_colors) "\x1b[0m" else "";
        const error_color = if (self.use_colors) "\x1b[91m" else "";
        const warning_color = if (self.use_colors) "\x1b[93m" else "";
        const success_color = if (self.use_colors) "\x1b[92m" else "";
        
        if (self.error_count > 0) {
            try writer.print("{s}Compilation failed{s} with ", .{ error_color, reset });
            
            if (self.error_count == 1) {
                try writer.print("{s}1 error{s}", .{ error_color, reset });
            } else {
                try writer.print("{s}{d} errors{s}", .{ error_color, self.error_count, reset });
            }
            
            if (self.warning_count > 0) {
                if (self.warning_count == 1) {
                    try writer.print(" and {s}1 warning{s}", .{ warning_color, reset });
                } else {
                    try writer.print(" and {s}{d} warnings{s}", .{ warning_color, self.warning_count, reset });
                }
            }
        } else if (self.warning_count > 0) {
            try writer.print("{s}Compilation completed{s} with ", .{ success_color, reset });
            
            if (self.warning_count == 1) {
                try writer.print("{s}1 warning{s}", .{ warning_color, reset });
            } else {
                try writer.print("{s}{d} warnings{s}", .{ warning_color, self.warning_count, reset });
            }
        }
        
        try writer.print("\n", .{});
    }
};

// Testing functions
test "error diagnostics system" {
    const allocator = std.testing.allocator;
    
    var engine = DiagnosticEngine.init(allocator, 10);
    defer engine.deinit();
    
    // Add test source file
    const test_source = "slay main() {\n    sus x = 42\n    vibez.spill(y)\n}\n";
    try engine.addSourceFile("test.csd", test_source);
    
    // Test single-line error
    const span1 = SourceSpan.fromSinglePosition("test.csd", 3, 17, 35);
    try engine.reportError(.S001_UndefinedVariable, "Variable 'y' is not defined", span1);
    
    // Test multi-line span
    const span2 = SourceSpan.init("test.csd", 2, 5, 3, 20, 20, 40);
    try engine.reportWarning(.S003_TypeMismatch, "Type inference may be ambiguous", span2);
    
    try std.testing.expect(engine.hasErrors());
    try std.testing.expect(engine.hasWarnings());
    try std.testing.expect(engine.getErrorCount() == 1);
    try std.testing.expect(engine.getWarningCount() == 1);
}

// Example integration functions for lexer, parser, and semantic analysis
pub fn lexerError(engine: *DiagnosticEngine, code: ErrorCode, message: []const u8, file: []const u8, line: u32, column: u32, offset: u32) !void {
    const span = SourceSpan.fromSinglePosition(file, line, column, offset);
    try engine.reportError(code, message, span);
}

pub fn parserError(engine: *DiagnosticEngine, code: ErrorCode, message: []const u8, file: []const u8, start_line: u32, start_col: u32, end_line: u32, end_col: u32, start_offset: u32, end_offset: u32) !void {
    const span = SourceSpan.init(file, start_line, start_col, end_line, end_col, start_offset, end_offset);
    try engine.reportError(code, message, span);
}

pub fn semanticError(engine: *DiagnosticEngine, code: ErrorCode, message: []const u8, primary_span: SourceSpan, related_spans: ?[]SourceSpan) !void {
    try engine.reportError(code, message, primary_span);
    
    if (related_spans) |spans| {
        var diagnostic = &engine.diagnostics.items[engine.diagnostics.items.len - 1];
        for (spans) |span| {
            try diagnostic.addSecondarySpan(span);
        }
    }
}
