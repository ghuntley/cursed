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

// Variable tracking for unused variable detection
const VariableInfo = struct {
    name: []const u8,
    line: u32,
    column: u32,
    used: bool = false,
    is_parameter: bool = false,
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
        _ = ast_tree; // TODO: Use this when implementing proper AST traversal
        var declared_vars = std.StringHashMap(VariableInfo).init(self.allocator);
        defer declared_vars.deinit();
        
        // TODO: Implement proper AST traversal with new structure
        // First pass: collect all variable declarations  
        // for (ast_tree.statements.items) |stmt_ptr| {
        //     const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        //     try self.collectVariableDeclarations(&declared_vars, stmt);
        // }
        
        // Second pass: mark variables as used
        // for (ast_tree.statements.items) |stmt_ptr| {
        //     const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        //     // Need to traverse expressions within statements
        // }
        
        // Report unused variables
        var iterator = declared_vars.iterator();
        while (iterator.next()) |entry| {
            const var_name = entry.key_ptr.*;
            const var_info = entry.value_ptr.*;
            
            if (!var_info.used) {
                const message = try std.fmt.allocPrint(self.allocator, "Variable '{s}' is declared but never used", .{var_name});
                defer self.allocator.free(message);
                
                try self.addIssue(LintIssue{
                    .rule_id = "unused-variable",
                    .severity = .Warning,
                    .category = .Performance,
                    .message = message,
                    .file = file_path,
                    .line = var_info.line,
                    .column = var_info.column,
                    .suggestion = "Remove unused variable or prefix with '_' if intentional",
                });
            }
        }
    }
    
    fn checkInefficiientLoops(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path; 
        _ = ast_tree;
        // TODO: Implement with new AST structure
    }
    
    // TODO: Reimplement visitor functions with new AST structure  
    fn visitLoops(self: *Linter, stmt: *ast.Statement, file_path: []const u8) !void {
        _ = self;
        _ = stmt;
        _ = file_path;
        // Placeholder implementation
    }
    
    fn checkStringConcatenation(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        _ = self;
        _ = file_path;
        _ = ast_tree;
        // TODO: Implement with new AST structure
    }
    
    fn visitStringOperations(self: *Linter, stmt: *ast.Statement, file_path: []const u8) !void {
        _ = self;
        _ = stmt;
        _ = file_path;
        // Placeholder implementation
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
        // TODO: Implement with new AST structure
    }
    
    fn visitForSecrets(self: *Linter, stmt: *ast.Statement, file_path: []const u8) !void {
        _ = self;
        _ = stmt; 
        _ = file_path;
        // Placeholder implementation
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
    
    // Helper methods for AST analysis
    fn collectVariableDeclarations(self: *Linter, vars: *std.StringHashMap(VariableInfo), stmt: *ast.Statement) !void {
        _ = self; // Mark unused parameter
        switch (stmt.*) {
            .Let => |let_stmt| {
                try vars.put(let_stmt.name, VariableInfo{
                    .name = let_stmt.name,
                    .line = 0, // TODO: Add source location to Statement
                    .column = 0,
                    .used = false,
                    .is_parameter = false,
                });
            },
            .ShortDeclaration => |short_decl| {
                for (short_decl.names.items) |name| {
                    try vars.put(name, VariableInfo{
                        .name = name,
                        .line = 0, // TODO: Add source location to Statement
                        .column = 0,
                        .used = false,
                        .is_parameter = false,
                    });
                }
            },
            .Function => |func_stmt| {
                // Collect function parameters
                for (func_stmt.parameters.items) |param| {
                    try vars.put(param.name, VariableInfo{
                        .name = param.name,
                        .line = 0, // TODO: Add source location to Statement
                        .column = 0,
                        .used = false,
                        .is_parameter = true,
                    });
                }
            },
            else => {},
        }
    }
    
    fn markVariableUsage(self: *Linter, vars: *std.StringHashMap(VariableInfo), expr: *ast.Expression) !void {
        _ = self; // Mark unused parameter
        switch (expr.*) {
            .Identifier => |identifier| {
                if (vars.getPtr(identifier)) |var_info| {
                    var_info.used = true;
                }
            },
            .Variable => |variable| {
                if (vars.getPtr(variable)) |var_info| {
                    var_info.used = true;
                }
            },
            else => {
                // TODO: Recursively analyze other expression types
            },
        }
    }
    
    fn isInfiniteLoop(self: *Linter, condition: *ast.Expression) bool {
        _ = self;
        switch (condition.*) {
            .Boolean => |bool_val| {
                return bool_val == true;
            },
            .Integer => |int_val| {
                return int_val != 0;
            },
            else => return false,
        }
    }
    
    // TODO: Fix these functions to use new AST structure
    fn hasInefficiientStringOperation(self: *Linter, node: *ast.Expression) bool {
        _ = self;
        _ = node;
        return false; // Placeholder implementation
    }
    
    // TODO: Reimplement these functions with new AST structure
    fn checkForLoopEfficiency(self: *Linter, for_loop: ast.ForLoop, file_path: []const u8, node: *ast.Expression) !void {
        _ = self;
        _ = for_loop;
        _ = file_path;
        _ = node;
        // Placeholder implementation
    }
    
    fn hasArrayLengthInCondition(self: *Linter, condition: *ast.Expression) bool {
        _ = self;
        _ = condition;
        return false; // Placeholder implementation
    }
    
    fn isStringType(_: *Linter, node: *ast.Expression) bool {
        _ = node;
        return false; // Placeholder implementation
    }
    
    fn isInLoop(_: *Linter, node: *ast.Expression) bool {
        _ = node;
        return false; // Placeholder implementation
    }
    
    fn countStringConcatenations(self: *Linter, node: *ast.Expression) u32 {
        _ = self;
        _ = node;
        return 0; // Placeholder implementation
    }
    
    // Security pattern detection
    fn looksLikeApiKey(self: *Linter, value: []const u8) bool {
        if (value.len < 16) return false;
        
        // Check for common API key patterns
        const api_prefixes = [_][]const u8{ "sk_", "pk_", "ak_", "key_", "api_", "token_" };
        for (api_prefixes) |prefix| {
            if (std.mem.startsWith(u8, value, prefix)) {
                return true;
            }
        }
        
        // Check for long alphanumeric strings (likely API keys)
        if (value.len > 20 and self.isAlphanumeric(value)) {
            return true;
        }
        
        return false;
    }
    
    fn looksLikePassword(self: *Linter, value: []const u8) bool {
        const password_patterns = [_][]const u8{ "password", "passwd", "pwd", "secret", "pass" };
        const lower_value = std.ascii.allocLowerString(self.allocator, value) catch return false;
        defer self.allocator.free(lower_value);
        
        for (password_patterns) |pattern| {
            if (std.mem.indexOf(u8, lower_value, pattern) != null) {
                return true;
            }
        }
        return false;
    }
    
    fn looksLikePrivateKey(_: *Linter, value: []const u8) bool {
        return std.mem.indexOf(u8, value, "-----BEGIN") != null and 
               std.mem.indexOf(u8, value, "PRIVATE KEY") != null;
    }
    
    fn isAlphanumeric(_: *Linter, value: []const u8) bool {
        for (value) |char| {
            if (!std.ascii.isAlphanumeric(char)) {
                return false;
            }
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
        try stdout.print("    {{\n", .{});
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
