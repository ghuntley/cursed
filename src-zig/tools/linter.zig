// CURSED Code Linter
// Provides code quality analysis and suggestions for CURSED syntax

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import CURSED compiler components
const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const ast = @import("../ast.zig");

// Lint Rule Severity
pub const Severity = enum {
    Error,
    Warning,
    Info,
    Hint,
    
    pub fn toString(self: Severity) []const u8 {
        return switch (self) {
            .Error => "error",
            .Warning => "warning",
            .Info => "info",
            .Hint => "hint",
        };
    }
};

// Lint Rule Categories
pub const RuleCategory = enum {
    Style,
    Performance,
    Security,
    Correctness,
    BestPractice,
    GenZSyntax,
    
    pub fn toString(self: RuleCategory) []const u8 {
        return switch (self) {
            .Style => "style",
            .Performance => "performance",
            .Security => "security",
            .Correctness => "correctness",
            .BestPractice => "best-practice",
            .GenZSyntax => "gen-z-syntax",
        };
    }
};

// Lint Issue
pub const LintIssue = struct {
    rule_id: []const u8,
    severity: Severity,
    category: RuleCategory,
    message: []const u8,
    file: []const u8,
    line: u32,
    column: u32,
    suggestion: ?[]const u8 = null,
};

// Linter Configuration
pub const LinterConfig = struct {
    enabled_rules: std.HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    severity_overrides: std.HashMap([]const u8, Severity, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    max_line_length: u32 = 100,
    max_function_length: u32 = 50,
    enforce_gen_z_syntax: bool = true,
    
    pub fn init(allocator: Allocator) LinterConfig {
        return LinterConfig{
            .enabled_rules = std.HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .severity_overrides = std.HashMap([]const u8, Severity, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *LinterConfig) void {
        self.enabled_rules.deinit();
        self.severity_overrides.deinit();
    }
};

// CURSED Linter
pub const Linter = struct {
    allocator: Allocator,
    config: LinterConfig,
    issues: ArrayList(LintIssue),
    
    pub fn init(allocator: Allocator, config: LinterConfig) Linter {
        return Linter{
            .allocator = allocator,
            .config = config,
            .issues = ArrayList(LintIssue).init(allocator),
        };
    }
    
    pub fn deinit(self: *Linter) void {
        self.issues.deinit();
    }
    
    pub fn lintFile(self: *Linter, file_path: []const u8) !void {
        // Read file
        const file = try std.fs.cwd().openFile(file_path, .{});
        defer file.close();
        
        const source = try file.readToEndAlloc(self.allocator, 1024 * 1024); // 1MB max
        defer self.allocator.free(source);
        
        try self.lintSource(file_path, source);
    }
    
    pub fn lintSource(self: *Linter, file_path: []const u8, source: []const u8) !void {
        // Tokenize source
        var token_lexer = lexer.Lexer.init(self.allocator, source);
        
        const tokens = try token_lexer.tokenize();
        defer tokens.deinit();
        
        // Parse tokens
        var cursed_parser = parser.Parser.init(self.allocator, tokens.items);
        
        const ast_tree = cursed_parser.parseProgram() catch {
            try self.addIssue(LintIssue{
                .rule_id = "parse-error",
                .severity = .Error,
                .category = .Correctness,
                .message = "Failed to parse CURSED code",
                .file = file_path,
                .line = 1,
                .column = 1,
            });
            return;
        };
        
        // Run lint rules
        try self.runStyleRules(file_path, source, tokens.items);
        try self.runPerformanceRules(file_path, ast_tree);
        try self.runSecurityRules(file_path, ast_tree);
        try self.runCorrectnessRules(file_path, ast_tree);
        try self.runGenZSyntaxRules(file_path, tokens.items);
    }
    
    pub fn getIssues(self: *const Linter) []const LintIssue {
        return self.issues.items;
    }
    
    fn addIssue(self: *Linter, issue: LintIssue) !void {
        // Check if rule is enabled
        if (self.config.enabled_rules.get(issue.rule_id)) |enabled| {
            if (!enabled) return;
        }
        
        // Apply severity overrides
        var final_issue = issue;
        if (self.config.severity_overrides.get(issue.rule_id)) |severity| {
            final_issue.severity = severity;
        }
        
        try self.issues.append(final_issue);
    }
    
    // Style Rules
    fn runStyleRules(self: *Linter, file_path: []const u8, source: []const u8, tokens: []const lexer.Token) !void {
        try self.checkLineLength(file_path, source);
        try self.checkIndentation(file_path, source);
        try self.checkTrailingWhitespace(file_path, source);
        try self.checkNamingConventions(file_path, tokens);
    }
    
    fn checkLineLength(self: *Linter, file_path: []const u8, source: []const u8) !void {
        var line_number: u32 = 1;
        var line_start: usize = 0;
        
        for (source, 0..) |char, i| {
            if (char == '\n') {
                const line_length = i - line_start;
                if (line_length > self.config.max_line_length) {
                    try self.addIssue(LintIssue{
                        .rule_id = "line-too-long",
                        .severity = .Warning,
                        .category = .Style,
                        .message = try std.fmt.allocPrint(self.allocator, "Line too long ({} > {})", .{ line_length, self.config.max_line_length }),
                        .file = file_path,
                        .line = line_number,
                        .column = @as(u32, @intCast(line_length + 1)),
                        .suggestion = "Consider breaking this line into multiple lines",
                    });
                }
                line_number += 1;
                line_start = i + 1;
            }
        }
    }
    
    fn checkIndentation(self: *Linter, file_path: []const u8, source: []const u8) !void {
        var line_number: u32 = 1;
        var line_start: usize = 0;
        
        for (source, 0..) |char, i| {
            if (char == '\n') {
                // Check next line's indentation
                if (i + 1 < source.len) {
                    var spaces: u32 = 0;
                    var tabs: u32 = 0;
                    var j = i + 1;
                    
                    while (j < source.len and (source[j] == ' ' or source[j] == '\t')) {
                        if (source[j] == ' ') spaces += 1;
                        if (source[j] == '\t') tabs += 1;
                        j += 1;
                    }
                    
                    // Mixed indentation
                    if (spaces > 0 and tabs > 0) {
                        try self.addIssue(LintIssue{
                            .rule_id = "mixed-indentation",
                            .severity = .Warning,
                            .category = .Style,
                            .message = "Mixed spaces and tabs for indentation",
                            .file = file_path,
                            .line = line_number + 1,
                            .column = 1,
                            .suggestion = "Use either spaces or tabs consistently",
                        });
                    }
                    
                    // Inconsistent space indentation
                    if (spaces > 0 and spaces % 4 != 0) {
                        try self.addIssue(LintIssue{
                            .rule_id = "inconsistent-indentation",
                            .severity = .Info,
                            .category = .Style,
                            .message = "Indentation should be multiples of 4 spaces",
                            .file = file_path,
                            .line = line_number + 1,
                            .column = 1,
                            .suggestion = "Use 4-space indentation",
                        });
                    }
                }
                
                line_number += 1;
                line_start = i + 1;
            }
        }
    }
    
    fn checkTrailingWhitespace(self: *Linter, file_path: []const u8, source: []const u8) !void {
        var line_number: u32 = 1;
        var line_start: usize = 0;
        
        for (source, 0..) |char, i| {
            if (char == '\n') {
                // Check for trailing whitespace
                if (i > line_start and (source[i - 1] == ' ' or source[i - 1] == '\t')) {
                    try self.addIssue(LintIssue{
                        .rule_id = "trailing-whitespace",
                        .severity = .Info,
                        .category = .Style,
                        .message = "Trailing whitespace",
                        .file = file_path,
                        .line = line_number,
                        .column = @as(u32, @intCast(i - line_start)),
                        .suggestion = "Remove trailing whitespace",
                    });
                }
                line_number += 1;
                line_start = i + 1;
            }
        }
    }
    
    fn checkNamingConventions(self: *Linter, file_path: []const u8, tokens: []const lexer.Token) !void {
        for (tokens) |token| {
            if (token.kind == .Identifier) {
                const name = token.lexeme;
                
                // Check for snake_case in function names
                if (self.isAfterKeyword(tokens, token, "slay")) {
                    if (!self.isSnakeCase(name)) {
                        try self.addIssue(LintIssue{
                            .rule_id = "function-naming",
                            .severity = .Warning,
                            .category = .Style,
                            .message = "Function names should use snake_case",
                            .file = file_path,
                            .line = @intCast(token.line),
                            .column = @intCast(token.column),
                            .suggestion = try self.toSnakeCase(name),
                        });
                    }
                }
                
                // Check for PascalCase in struct names
                if (self.isAfterKeyword(tokens, token, "squad")) {
                    if (!self.isPascalCase(name)) {
                        try self.addIssue(LintIssue{
                            .rule_id = "struct-naming",
                            .severity = .Warning,
                            .category = .Style,
                            .message = "Struct names should use PascalCase",
                            .file = file_path,
                            .line = @intCast(token.line),
                            .column = @intCast(token.column),
                            .suggestion = try self.toPascalCase(name),
                        });
                    }
                }
            }
        }
    }
    
    // Performance Rules
    fn runPerformanceRules(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        try self.checkUnusedVariables(file_path, ast_tree);
        try self.checkInefficiientLoops(file_path, ast_tree);
        try self.checkStringConcatenation(file_path, ast_tree);
    }
    
    fn checkUnusedVariables(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Walk AST to find unused variables
        // This would be a full AST traversal implementation
    }
    
    fn checkInefficiientLoops(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Check for common loop anti-patterns
    }
    
    fn checkStringConcatenation(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Check for inefficient string concatenation patterns
    }
    
    // Security Rules
    fn runSecurityRules(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        try self.checkHardcodedSecrets(file_path, ast_tree);
        try self.checkUnsafeOperations(file_path, ast_tree);
    }
    
    fn checkHardcodedSecrets(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Check for hardcoded passwords, API keys, etc.
    }
    
    fn checkUnsafeOperations(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Check for potentially unsafe operations
    }
    
    // Correctness Rules
    fn runCorrectnessRules(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        try self.checkUnreachableCode(file_path, ast_tree);
        try self.checkInfiniteLoops(file_path, ast_tree);
    }
    
    fn checkUnreachableCode(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Check for unreachable code after returns
    }
    
    fn checkInfiniteLoops(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // Check for potential infinite loops
    }
    
    // Gen Z Syntax Rules
    fn runGenZSyntaxRules(self: *Linter, file_path: []const u8, tokens: []const lexer.Token) !void {
        if (!self.config.enforce_gen_z_syntax) return;
        
        try self.checkDeprecatedKeywords(file_path, tokens);
        try self.checkGenZConsistency(file_path, tokens);
    }
    
    fn checkDeprecatedKeywords(self: *Linter, file_path: []const u8, tokens: []const lexer.Token) !void {
        const deprecated_mappings = [_]struct { old: []const u8, new: []const u8 }{
            .{ .old = "function", .new = "slay" },
            .{ .old = "var", .new = "sus" },
            .{ .old = "return", .new = "yolo" },
            .{ .old = "if", .new = "lowkey" },
            .{ .old = "while", .new = "bestie" },
            .{ .old = "struct", .new = "squad" },
            .{ .old = "interface", .new = "collab" },
        };
        
        for (tokens) |token| {
            // Check traditional keywords that should be converted
            for (deprecated_mappings) |mapping| {
                if (std.mem.eql(u8, token.lexeme, mapping.old)) {
                    try self.addIssue(LintIssue{
                        .rule_id = "deprecated-keyword",
                        .severity = .Warning,
                        .category = .GenZSyntax,
                        .message = try std.fmt.allocPrint(self.allocator, "Use '{s}' instead of '{s}'", .{ mapping.new, mapping.old }),
                        .file = file_path,
                        .line = @intCast(token.line),
                        .column = @intCast(token.column),
                        .suggestion = mapping.new,
                    });
                }
            }
        }
    }
    
    fn checkGenZConsistency(self: *Linter, file_path: []const u8, tokens: []const lexer.Token) !void {
        // Check for consistent use of Gen Z terminology
        var has_gen_z: bool = false;
        var has_traditional: bool = false;
        
        for (tokens) |token| {
            const gen_z_keywords = [_][]const u8{ "sus", "slay", "yolo", "vibes", "bestie", "based", "cringe", "yeet", "stan" };
            const traditional_keywords = [_][]const u8{ "var", "function", "return", "if", "while", "true", "false" };
            
            for (gen_z_keywords) |keyword| {
                if (std.mem.eql(u8, token.lexeme, keyword)) {
                    has_gen_z = true;
                    break;
                }
            }
            
            for (traditional_keywords) |keyword| {
                if (std.mem.eql(u8, token.lexeme, keyword)) {
                    has_traditional = true;
                    break;
                }
            }
        }
        
        if (has_gen_z and has_traditional) {
            try self.addIssue(LintIssue{
                .rule_id = "mixed-syntax-style",
                .severity = .Info,
                .category = .GenZSyntax,
                .message = "Mixed Gen Z and traditional syntax",
                .file = file_path,
                .line = 1,
                .column = 1,
                .suggestion = "Use consistent Gen Z syntax throughout",
            });
        }
    }
    
    // Helper Functions
    fn isAfterKeyword(self: *Linter, tokens: []const lexer.Token, current: lexer.Token, keyword: []const u8) bool {
        _ = self;
        for (tokens, 0..) |token, i| {
            if (std.mem.eql(u8, token.lexeme, current.lexeme)) {
                if (i > 0 and std.mem.eql(u8, tokens[i - 1].lexeme, keyword)) {
                    return true;
                }
                break;
            }
        }
        return false;
    }
    
    fn isSnakeCase(self: *Linter, name: []const u8) bool {
        _ = self;
        for (name) |char| {
            if (char >= 'A' and char <= 'Z') return false;
        }
        return true;
    }
    
    fn isPascalCase(self: *Linter, name: []const u8) bool {
        _ = self;
        if (name.len == 0) return false;
        return name[0] >= 'A' and name[0] <= 'Z';
    }
    
    fn toSnakeCase(self: *Linter, name: []const u8) ![]const u8 {
        // Convert to snake_case
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        for (name, 0..) |char, i| {
            if (char >= 'A' and char <= 'Z') {
                if (i > 0) try result.append('_');
                try result.append(char + 32); // Convert to lowercase
            } else {
                try result.append(char);
            }
        }
        
        return try result.toOwnedSlice();
    }
    
    fn toPascalCase(self: *Linter, name: []const u8) ![]const u8 {
        // Convert to PascalCase
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        var capitalize_next = true;
        for (name) |char| {
            if (char == '_') {
                capitalize_next = true;
            } else if (capitalize_next and char >= 'a' and char <= 'z') {
                try result.append(char - 32); // Convert to uppercase
                capitalize_next = false;
            } else {
                try result.append(char);
                capitalize_next = false;
            }
        }
        
        return try result.toOwnedSlice();
    }
};

// Output formatters
pub fn printIssues(allocator: Allocator, issues: []const LintIssue, format: []const u8) !void {
    if (std.mem.eql(u8, format, "json")) {
        try printIssuesJSON(allocator, issues);
    } else {
        try printIssuesHuman(allocator, issues);
    }
}

fn printIssuesHuman(allocator: Allocator, issues: []const LintIssue) !void {
    _ = allocator;
    const stdout = std.io.getStdOut().writer();
    
    for (issues) |issue| {
        try stdout.print("{s}:{}:{}: {s}: {s} [{s}]\n", .{
            issue.file,
            issue.line,
            issue.column,
            issue.severity.toString(),
            issue.message,
            issue.rule_id,
        });
        
        if (issue.suggestion) |suggestion| {
            try stdout.print("  suggestion: {s}\n", .{suggestion});
        }
    }
    
    try stdout.print("\nFound {} issues\n", .{issues.len});
}

fn printIssuesJSON(allocator: Allocator, issues: []const LintIssue) !void {
    _ = allocator;
    const stdout = std.io.getStdOut().writer();
    
    try stdout.writeAll("{\n  \"issues\": [\n");
    
    for (issues, 0..) |issue, i| {
        try stdout.print("    {{\n");
        try stdout.print("      \"rule_id\": \"{s}\",\n", .{issue.rule_id});
        try stdout.print("      \"severity\": \"{s}\",\n", .{issue.severity.toString()});
        try stdout.print("      \"category\": \"{s}\",\n", .{issue.category.toString()});
        try stdout.print("      \"message\": \"{s}\",\n", .{issue.message});
        try stdout.print("      \"file\": \"{s}\",\n", .{issue.file});
        try stdout.print("      \"line\": {},\n", .{issue.line});
        try stdout.print("      \"column\": {}", .{issue.column});
        
        if (issue.suggestion) |suggestion| {
            try stdout.print(",\n      \"suggestion\": \"{s}\"", .{suggestion});
        }
        
        try stdout.writeAll("\n    }");
        if (i < issues.len - 1) try stdout.writeAll(",");
        try stdout.writeAll("\n");
    }
    
    try stdout.writeAll("  ],\n");
    try stdout.print("  \"total\": {}\n", .{issues.len});
    try stdout.writeAll("}\n");
}

// Main linter entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-lint <file> [--format json]", .{});
        return;
    }
    
    var config = LinterConfig.init(allocator);
    defer config.deinit();
    
    var linter = Linter.init(allocator, config);
    defer linter.deinit();
    
    const file_path = args[1];
    const format = if (args.len > 2 and std.mem.eql(u8, args[2], "--format") and args.len > 3) args[3] else "human";
    
    try linter.lintFile(file_path);
    const issues = linter.getIssues();
    
    try printIssues(allocator, issues, format);
}
