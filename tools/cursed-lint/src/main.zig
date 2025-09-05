//! CURSED Language Linter
//! Advanced static analysis and style checking for CURSED code

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

/// Lint severity levels
const Severity = enum {
    error,
    warning,
    info,
    hint,
    
    pub fn toString(self: Severity) []const u8 {
        return switch (self) {
            .error => "error",
            .warning => "warning",
            .info => "info",
            .hint => "hint",
        };
    }
    
    pub fn getColor(self: Severity) []const u8 {
        return switch (self) {
            .error => "\x1b[31m",      // Red
            .warning => "\x1b[33m",    // Yellow
            .info => "\x1b[36m",       // Cyan
            .hint => "\x1b[37m",       // White
        };
    }
};

/// Lint rule categories
const Category = enum {
    style,
    performance,
    security,
    correctness,
    complexity,
    naming,
    
    pub fn toString(self: Category) []const u8 {
        return switch (self) {
            .style => "style",
            .performance => "performance",
            .security => "security",
            .correctness => "correctness",
            .complexity => "complexity",
            .naming => "naming",
        };
    }
};

/// Position in source code
const Position = struct {
    line: u32,
    column: u32,
    
    pub fn format(self: Position, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        try writer.print("{}:{}", .{ self.line, self.column });
    }
};

/// Source code range
const Range = struct {
    start: Position,
    end: Position,
};

/// Lint diagnostic message
const Diagnostic = struct {
    severity: Severity,
    category: Category,
    rule: []const u8,
    message: []const u8,
    range: Range,
    suggestion: ?[]const u8,
    
    pub fn format(self: Diagnostic, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        try writer.print("{s}{s}\x1b[0m: {} [{}:{}] {s}", .{
            self.severity.getColor(),
            self.severity.toString(),
            self.range.start,
            self.category.toString(),
            self.rule,
            self.message,
        });
        
        if (self.suggestion) |suggestion| {
            try writer.print("\n  \x1b[32mSuggestion:\x1b[0m {s}", .{suggestion});
        }
    }
};

/// Lint configuration
const LintConfig = struct {
    max_line_length: u32 = 100,
    max_function_length: u32 = 50,
    max_complexity: u32 = 10,
    enforce_naming_conventions: bool = true,
    require_documentation: bool = false,
    check_unused_variables: bool = true,
    check_unused_imports: bool = true,
    check_security_issues: bool = true,
    
    pub fn fromFile(allocator: Allocator, path: []const u8) !LintConfig {
        _ = allocator;
        _ = path;
        // TODO: Parse configuration file
        return LintConfig{};
    }
};

/// Main linter structure
const CursedLinter = struct {
    allocator: Allocator,
    config: LintConfig,
    diagnostics: ArrayList(Diagnostic),
    source_code: []const u8,
    file_path: []const u8,
    
    pub fn init(allocator: Allocator, config: LintConfig) CursedLinter {
        return CursedLinter{
            .allocator = allocator,
            .config = config,
            .diagnostics = ArrayList(Diagnostic).init(allocator),
            .source_code = "",
            .file_path = "",
        };
    }
    
    pub fn deinit(self: *CursedLinter) void {
        for (self.diagnostics.items) |diag| {
            self.allocator.free(diag.message);
            if (diag.suggestion) |suggestion| {
                self.allocator.free(suggestion);
            }
        }
        self.diagnostics.deinit();
    }
    
    /// Lint a source file
    pub fn lintFile(self: *CursedLinter, file_path: []const u8) !void {
        self.file_path = file_path;
        
        // Read source file
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
            try self.addDiagnostic(.error, .correctness, "file-read", 
                try std.fmt.allocPrint(self.allocator, "Cannot read file: {}", .{err}), 
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} }, 
                null);
            return;
        };
        defer file.close();
        
        const file_size = try file.getEndPos();
        self.source_code = try self.allocator.alloc(u8, file_size);
        _ = try file.readAll(self.source_code);
        
        // Run all lint checks
        try self.checkBasicSyntax();
        try self.checkStyleIssues();
        try self.checkPerformanceIssues();
        try self.checkSecurityIssues();
        try self.checkComplexity();
        try self.checkNamingConventions();
        try self.checkUnusedCode();
        try self.checkBestPractices();
    }
    
    /// Check basic syntax and parsing
    fn checkBasicSyntax(self: *CursedLinter) !void {
        var lex = lexer.Lexer.init(self.allocator, self.source_code);
        defer lex.deinit();
        
        const tokens = lex.tokenize() catch |err| {
            try self.addDiagnostic(.error, .correctness, "syntax-error",
                try std.fmt.allocPrint(self.allocator, "Syntax error: {}", .{err}),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 10} },
                null);
            return;
        };
        defer tokens.deinit();
        
        var parse = parser.Parser.init(self.allocator, tokens.items);
        defer parse.deinit();
        
        const program = parse.parseProgram() catch |err| {
            try self.addDiagnostic(.error, .correctness, "parse-error",
                try std.fmt.allocPrint(self.allocator, "Parse error: {}", .{err}),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 10} },
                null);
            return;
        };
        
        _ = program; // Successfully parsed
    }
    
    /// Check style-related issues
    fn checkStyleIssues(self: *CursedLinter) !void {
        const lines = std.mem.split(u8, self.source_code, "\n");
        var line_num: u32 = 1;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            defer line_num += 1;
            
            // Check line length
            if (line.len > self.config.max_line_length) {
                try self.addDiagnostic(.warning, .style, "line-too-long",
                    try std.fmt.allocPrint(self.allocator, "Line is {} characters long (max {})", .{ line.len, self.config.max_line_length }),
                    Range{ 
                        .start = Position{.line = line_num, .column = 1}, 
                        .end = Position{.line = line_num, .column = @as(u32, @intCast(line.len))} 
                    },
                    try std.fmt.allocPrint(self.allocator, "Break this line into multiple lines"));
            }
            
            // Check trailing whitespace
            if (line.len > 0 and std.ascii.isWhitespace(line[line.len - 1])) {
                try self.addDiagnostic(.info, .style, "trailing-whitespace",
                    try std.fmt.allocPrint(self.allocator, "Line has trailing whitespace"),
                    Range{ 
                        .start = Position{.line = line_num, .column = @as(u32, @intCast(line.len))}, 
                        .end = Position{.line = line_num, .column = @as(u32, @intCast(line.len))} 
                    },
                    try std.fmt.allocPrint(self.allocator, "Remove trailing whitespace"));
            }
            
            // Check indentation consistency
            var indent_count: u32 = 0;
            var has_tabs = false;
            var has_spaces = false;
            
            for (line) |char| {
                if (char == ' ') {
                    has_spaces = true;
                    indent_count += 1;
                } else if (char == '\t') {
                    has_tabs = true;
                    indent_count += 4; // Assume tab = 4 spaces
                } else {
                    break;
                }
            }
            
            if (has_tabs and has_spaces) {
                try self.addDiagnostic(.warning, .style, "mixed-indentation",
                    try std.fmt.allocPrint(self.allocator, "Mixed tabs and spaces for indentation"),
                    Range{ 
                        .start = Position{.line = line_num, .column = 1}, 
                        .end = Position{.line = line_num, .column = indent_count + 1} 
                    },
                    try std.fmt.allocPrint(self.allocator, "Use consistent indentation (either tabs or spaces)"));
            }
        }
    }
    
    /// Check performance-related issues
    fn checkPerformanceIssues(self: *CursedLinter) !void {
        // Check for inefficient string concatenation
        if (std.mem.indexOf(u8, self.source_code, "+ \"") != null) {
            try self.addDiagnostic(.hint, .performance, "string-concatenation",
                try std.fmt.allocPrint(self.allocator, "Consider using string interpolation instead of concatenation"),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                try std.fmt.allocPrint(self.allocator, "Use string interpolation: `\"text {{variable}}\"`"));
        }
        
        // Check for unnecessary allocations in loops
        const loop_keywords = [_][]const u8{ "bestie", "periodt" };
        for (loop_keywords) |keyword| {
            var search_start: usize = 0;
            while (std.mem.indexOfPos(u8, self.source_code, search_start, keyword)) |pos| {
                // Look for allocations within the loop
                const loop_end = std.mem.indexOfPos(u8, self.source_code, pos, "}") orelse self.source_code.len;
                const loop_body = self.source_code[pos..loop_end];
                
                if (std.mem.indexOf(u8, loop_body, "sus ") != null) {
                    try self.addDiagnostic(.warning, .performance, "allocation-in-loop",
                        try std.fmt.allocPrint(self.allocator, "Variable allocation inside loop may impact performance"),
                        Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                        try std.fmt.allocPrint(self.allocator, "Consider moving allocations outside the loop"));
                }
                
                search_start = pos + keyword.len;
            }
        }
    }
    
    /// Check security-related issues
    fn checkSecurityIssues(self: *CursedLinter) !void {
        if (!self.config.check_security_issues) return;
        
        // Check for potential buffer overflows
        if (std.mem.indexOf(u8, self.source_code, "unsafe") != null) {
            try self.addDiagnostic(.error, .security, "unsafe-code",
                try std.fmt.allocPrint(self.allocator, "Unsafe code blocks should be carefully reviewed"),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                try std.fmt.allocPrint(self.allocator, "Ensure memory safety and bounds checking"));
        }
        
        // Check for hardcoded secrets (basic pattern matching)
        const secret_patterns = [_][]const u8{
            "password", "secret", "token", "key", "api_key",
            "auth_token", "private_key", "credential"
        };
        
        for (secret_patterns) |pattern| {
            var search_start: usize = 0;
            while (std.mem.indexOfPos(u8, self.source_code, search_start, pattern)) |pos| {
                // Check if it's in a string literal
                var in_string = false;
                var quote_pos = pos;
                while (quote_pos > 0) : (quote_pos -= 1) {
                    if (self.source_code[quote_pos] == '"') {
                        in_string = true;
                        break;
                    }
                    if (self.source_code[quote_pos] == '\n') break;
                }
                
                if (in_string) {
                    try self.addDiagnostic(.warning, .security, "potential-secret",
                        try std.fmt.allocPrint(self.allocator, "Potential hardcoded secret: {s}", .{pattern}),
                        Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                        try std.fmt.allocPrint(self.allocator, "Use environment variables or secure configuration"));
                }
                
                search_start = pos + pattern.len;
            }
        }
    }
    
    /// Check code complexity
    fn checkComplexity(self: *CursedLinter) !void {
        // Simple cyclomatic complexity check
        const complexity_keywords = [_][]const u8{ "ready", "otherwise", "bestie", "sick", "when" };
        var complexity: u32 = 1; // Base complexity
        
        for (complexity_keywords) |keyword| {
            var search_start: usize = 0;
            while (std.mem.indexOfPos(u8, self.source_code, search_start, keyword)) |pos| {
                complexity += 1;
                search_start = pos + keyword.len;
            }
        }
        
        if (complexity > self.config.max_complexity) {
            try self.addDiagnostic(.warning, .complexity, "high-complexity",
                try std.fmt.allocPrint(self.allocator, "Code complexity is {} (max {})", .{ complexity, self.config.max_complexity }),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                try std.fmt.allocPrint(self.allocator, "Consider breaking this into smaller functions"));
        }
    }
    
    /// Check naming conventions
    fn checkNamingConventions(self: *CursedLinter) !void {
        if (!self.config.enforce_naming_conventions) return;
        
        // Check for snake_case in function names
        var search_start: usize = 0;
        while (std.mem.indexOfPos(u8, self.source_code, search_start, "slay ")) |pos| {
            const after_slay = self.source_code[pos + 5..];
            const name_end = std.mem.indexOf(u8, after_slay, "(") orelse continue;
            const func_name = std.mem.trim(u8, after_slay[0..name_end], " \t");
            
            if (func_name.len > 0) {
                // Check for camelCase (should be snake_case)
                var has_upper = false;
                for (func_name) |char| {
                    if (std.ascii.isUpper(char)) {
                        has_upper = true;
                        break;
                    }
                }
                
                if (has_upper and !std.mem.eql(u8, func_name, "main")) {
                    try self.addDiagnostic(.info, .naming, "function-naming",
                        try std.fmt.allocPrint(self.allocator, "Function '{s}' should use snake_case", .{func_name}),
                        Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                        try std.fmt.allocPrint(self.allocator, "Use snake_case: {s}", .{func_name})); // TODO: Convert to snake_case
                }
            }
            
            search_start = pos + 5;
        }
    }
    
    /// Check for unused code
    fn checkUnusedCode(self: *CursedLinter) !void {
        if (!self.config.check_unused_variables and !self.config.check_unused_imports) return;
        
        // Simple unused import check
        if (self.config.check_unused_imports) {
            const lines = std.mem.split(u8, self.source_code, "\n");
            var line_iter = lines;
            
            while (line_iter.next()) |line| {
                const trimmed = std.mem.trim(u8, line, " \t");
                if (std.mem.startsWith(u8, trimmed, "yeet \"")) {
                    const quote_start = std.mem.indexOf(u8, trimmed, "\"") orelse continue;
                    const quote_end = std.mem.indexOfPos(u8, trimmed, quote_start + 1, "\"") orelse continue;
                    const module_name = trimmed[quote_start + 1..quote_end];
                    
                    // Check if module is used
                    const usage_pattern = try std.fmt.allocPrint(self.allocator, "{s}.", .{module_name});
                    defer self.allocator.free(usage_pattern);
                    
                    if (std.mem.indexOf(u8, self.source_code, usage_pattern) == null and !std.mem.eql(u8, module_name, "vibez")) {
                        try self.addDiagnostic(.warning, .style, "unused-import",
                            try std.fmt.allocPrint(self.allocator, "Unused import: {s}", .{module_name}),
                            Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                            try std.fmt.allocPrint(self.allocator, "Remove unused import"));
                    }
                }
            }
        }
    }
    
    /// Check best practices
    fn checkBestPractices(self: *CursedLinter) !void {
        // Check for missing main function
        if (std.mem.indexOf(u8, self.source_code, "slay main(") == null) {
            try self.addDiagnostic(.info, .correctness, "missing-main",
                try std.fmt.allocPrint(self.allocator, "No main function found"),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                try std.fmt.allocPrint(self.allocator, "Add a main function: slay main() drip {{ damn 0 }}"));
        }
        
        // Check for empty blocks
        if (std.mem.indexOf(u8, self.source_code, "{ }") != null or std.mem.indexOf(u8, self.source_code, "{\n}") != null) {
            try self.addDiagnostic(.info, .style, "empty-block",
                try std.fmt.allocPrint(self.allocator, "Empty code block found"),
                Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                try std.fmt.allocPrint(self.allocator, "Add implementation or remove empty block"));
        }
        
        // Check for TODO/FIXME comments
        const todo_patterns = [_][]const u8{ "TODO", "FIXME", "HACK", "XXX" };
        for (todo_patterns) |pattern| {
            if (std.mem.indexOf(u8, self.source_code, pattern) != null) {
                try self.addDiagnostic(.hint, .style, "todo-comment",
                    try std.fmt.allocPrint(self.allocator, "{s} comment found", .{pattern}),
                    Range{ .start = Position{.line = 1, .column = 1}, .end = Position{.line = 1, .column = 1} },
                    try std.fmt.allocPrint(self.allocator, "Address the {s} item", .{pattern}));
            }
        }
    }
    
    /// Add a diagnostic message
    fn addDiagnostic(self: *CursedLinter, severity: Severity, category: Category, rule: []const u8, message: []const u8, range: Range, suggestion: ?[]const u8) !void {
        try self.diagnostics.append(Diagnostic{
            .severity = severity,
            .category = category,
            .rule = rule,
            .message = message,
            .range = range,
            .suggestion = suggestion,
        });
    }
    
    /// Get diagnostics sorted by severity
    pub fn getDiagnostics(self: *CursedLinter) []Diagnostic {
        // Sort by severity (errors first)
        std.sort.sort(Diagnostic, self.diagnostics.items, {}, struct {
            fn lessThan(context: void, a: Diagnostic, b: Diagnostic) bool {
                _ = context;
                const severity_order = [_]Severity{ .error, .warning, .info, .hint };
                const a_index = for (severity_order, 0..) |sev, i| {
                    if (sev == a.severity) break i;
                } else severity_order.len;
                const b_index = for (severity_order, 0..) |sev, i| {
                    if (sev == b.severity) break i;
                } else severity_order.len;
                return a_index < b_index;
            }
        }.lessThan);
        
        return self.diagnostics.items;
    }
    
    /// Print diagnostics in various formats
    pub fn printDiagnostics(self: *CursedLinter, format: enum { default, json, xml }) !void {
        switch (format) {
            .default => {
                print("\n\x1b[1mLint Results for {s}:\x1b[0m\n", .{self.file_path});
                print("{'─'[0]}" ** 50);
                print("\n");
                
                for (self.getDiagnostics()) |diag| {
                    print("{}\n", .{diag});
                }
                
                // Summary
                var error_count: u32 = 0;
                var warning_count: u32 = 0;
                var info_count: u32 = 0;
                var hint_count: u32 = 0;
                
                for (self.diagnostics.items) |diag| {
                    switch (diag.severity) {
                        .error => error_count += 1,
                        .warning => warning_count += 1,
                        .info => info_count += 1,
                        .hint => hint_count += 1,
                    }
                }
                
                print("\n\x1b[1mSummary:\x1b[0m {} errors, {} warnings, {} info, {} hints\n", 
                    .{ error_count, warning_count, info_count, hint_count });
            },
            .json => {
                print("{{\"file\": \"{s}\", \"diagnostics\": [", .{self.file_path});
                for (self.getDiagnostics(), 0..) |diag, i| {
                    if (i > 0) print(",");
                    print("{{\"severity\": \"{s}\", \"category\": \"{s}\", \"rule\": \"{s}\", \"message\": \"{s}\"}}", 
                        .{ diag.severity.toString(), diag.category.toString(), diag.rule, diag.message });
                }
                print("]}\n");
            },
            .xml => {
                print("<lint file=\"{s}\">\n", .{self.file_path});
                for (self.getDiagnostics()) |diag| {
                    print("  <diagnostic severity=\"{s}\" category=\"{s}\" rule=\"{s}\" message=\"{s}\"/>\n", 
                        .{ diag.severity.toString(), diag.category.toString(), diag.rule, diag.message });
                }
                print("</lint>\n");
            },
        }
    }
};

/// Command line interface
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("CURSED Linter v1.0.0\n");
        print("Usage: cursed-lint [options] <file.💀.💀>\n");
        print("\nOptions:\n");
        print("  --config <file>     Use configuration file\n");
        print("  --format <fmt>      Output format (default, json, xml)\n");
        print("  --max-line-length <n>  Maximum line length (default: 100)\n");
        print("  --help              Show this help\n");
        return;
    }
    
    var config = LintConfig{};
    var file_path: ?[]const u8 = null;
    var output_format: enum { default, json, xml } = .default;
    
    var i: usize = 1;
    while (i < args.len) : (i += 1) {
        if (std.mem.eql(u8, args[i], "--help")) {
            // Help already printed above
            return;
        } else if (std.mem.eql(u8, args[i], "--config")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --config requires a file path\n");
                return;
            }
            config = LintConfig.fromFile(allocator, args[i]) catch |err| {
                print("Error reading config file: {}\n", .{err});
                return;
            };
        } else if (std.mem.eql(u8, args[i], "--format")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --format requires a format (default, json, xml)\n");
                return;
            }
            if (std.mem.eql(u8, args[i], "json")) {
                output_format = .json;
            } else if (std.mem.eql(u8, args[i], "xml")) {
                output_format = .xml;
            } else if (std.mem.eql(u8, args[i], "default")) {
                output_format = .default;
            } else {
                print("Error: Unknown format '{s}'\n", .{args[i]});
                return;
            }
        } else if (std.mem.eql(u8, args[i], "--max-line-length")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --max-line-length requires a number\n");
                return;
            }
            config.max_line_length = std.fmt.parseInt(u32, args[i], 10) catch |err| {
                print("Error parsing max line length: {}\n", .{err});
                return;
            };
        } else if (!std.mem.startsWith(u8, args[i], "--")) {
            file_path = args[i];
        } else {
            print("Error: Unknown option '{s}'\n", .{args[i]});
            return;
        }
    }
    
    if (file_path == null) {
        print("Error: No input file specified\n");
        return;
    }
    
    var linter = CursedLinter.init(allocator, config);
    defer linter.deinit();
    
    try linter.lintFile(file_path.?);
    try linter.printDiagnostics(output_format);
    
    // Exit with non-zero code if errors found
    const diagnostics = linter.getDiagnostics();
    for (diagnostics) |diag| {
        if (diag.severity == .error) {
            std.process.exit(1);
        }
    }
}
