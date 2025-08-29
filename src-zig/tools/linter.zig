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
    
    pub fn init() LinterConfig {
        return LinterConfig{
            .enabled_rules = std.HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .severity_overrides = std.HashMap([]const u8, Severity, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *LinterConfig) void {
        self.enabled_rules.deinit(self.allocator);
        self.severity_overrides.deinit(self.allocator);
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
            .issues = ArrayList(LintIssue){},
        };
    }
    
    pub fn deinit(self: *Linter) void {
        self.issues.deinit(self.allocator);
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
        
        try self.issues.append(allocator, final_issue);
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
        var declared_vars = std.StringHashMap(VariableInfo){};
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
    
    // Security Rules - Production Implementation
    fn runSecurityRules(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        try self.checkHardcodedSecrets(file_path, ast_tree);
        try self.checkUnsafeOperations(file_path, ast_tree);
        try self.checkBufferOverflows(file_path, ast_tree);
        try self.checkInsecureCrypto(file_path, ast_tree);
        try self.checkMemorySafety(file_path, ast_tree);
        try self.checkErrorHandling(file_path, ast_tree);
        try self.checkChannelSafety(file_path, ast_tree);
    }
    
    fn checkHardcodedSecrets(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForSecrets(stmt_ptr, file_path);
        }
    }
    
    fn visitForSecrets(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Let => |let_stmt| {
                if (let_stmt.initializer) |init_ptr| {
                    try self.checkExpressionForSecrets(init_ptr, file_path, let_stmt.name);
                }
            },
            .Assignment => |assign_stmt| {
                try self.checkExpressionForSecrets(assign_stmt.value, file_path, "assignment");
            },
            .Function => |func_stmt| {
                for (func_stmt.body.items) |body_stmt| {
                    try self.visitForSecrets(body_stmt, file_path);
                }
            },
            .ShortDeclaration => |short_decl| {
                for (short_decl.values.items) |value| {
                    try self.checkExpressionForSecrets(value, file_path, "variable");
                }
            },
            else => {},
        }
    }
    
    fn checkExpressionForSecrets(self: *Linter, expr_ptr: *anyopaque, file_path: []const u8, context: []const u8) !void {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .String => |string_val| {
                try self.analyzeStringForSecrets(string_val, file_path, context);
            },
            .Call => |call| {
                const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
                if (func_expr.* == .Identifier) {
                    const func_name = func_expr.Identifier;
                    
                    // Check for insecure crypto function calls
                    if (std.mem.eql(u8, func_name, "md5") or std.mem.eql(u8, func_name, "sha1")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "insecure-hash",
                            .severity = .Error,
                            .category = .Security,
                            .message = try std.fmt.allocPrint(self.allocator, "Insecure hash function '{s}' should not be used", .{func_name}),
                            .file = file_path,
                            .line = 1, // TODO: Get actual line from source location
                            .column = 1,
                            .suggestion = "Use sha256 or stronger hash functions",
                        });
                    }
                    
                    // Check for dangerous system calls
                    if (std.mem.eql(u8, func_name, "system") or std.mem.eql(u8, func_name, "exec")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "dangerous-system-call",
                            .severity = .Error,
                            .category = .Security,
                            .message = try std.fmt.allocPrint(self.allocator, "Dangerous system call '{s}' detected", .{func_name}),
                            .file = file_path,
                            .line = 1,
                            .column = 1,
                            .suggestion = "Validate input and use safer alternatives",
                        });
                    }
                }
                
                // Check arguments for secrets
                for (call.arguments.items) |arg| {
                    try self.checkExpressionForSecrets(arg, file_path, "function_argument");
                }
            },
            else => {},
        }
    }
    
    fn analyzeStringForSecrets(self: *Linter, value: []const u8, file_path: []const u8, context: []const u8) !void {
        // Check for API keys
        if (self.looksLikeApiKey(value)) {
            try self.addIssue(LintIssue{
                .rule_id = "hardcoded-api-key",
                .severity = .Error,
                .category = .Security,
                .message = try std.fmt.allocPrint(self.allocator, "Potential hardcoded API key in {s}", .{context}),
                .file = file_path,
                .line = 1,
                .column = 1,
                .suggestion = "Use environment variables or secure configuration",
            });
        }
        
        // Check for passwords
        if (self.looksLikePassword(value)) {
            try self.addIssue(LintIssue{
                .rule_id = "hardcoded-password",
                .severity = .Error,
                .category = .Security,
                .message = try std.fmt.allocPrint(self.allocator, "Potential hardcoded password in {s}", .{context}),
                .file = file_path,
                .line = 1,
                .column = 1,
                .suggestion = "Use secure credential storage",
            });
        }
        
        // Check for private keys
        if (self.looksLikePrivateKey(value)) {
            try self.addIssue(LintIssue{
                .rule_id = "hardcoded-private-key",
                .severity = .Error,
                .category = .Security,
                .message = "Private key detected in source code",
                .file = file_path,
                .line = 1,
                .column = 1,
                .suggestion = "Move private keys to secure key management",
            });
        }
        
        // Check for database connections
        if (std.mem.indexOf(u8, value, "://") != null and 
           (std.mem.indexOf(u8, value, "mysql") != null or 
            std.mem.indexOf(u8, value, "postgres") != null or
            std.mem.indexOf(u8, value, "mongodb") != null)) {
            try self.addIssue(LintIssue{
                .rule_id = "hardcoded-db-connection",
                .severity = .Warning,
                .category = .Security,
                .message = "Database connection string may contain credentials",
                .file = file_path,
                .line = 1,
                .column = 1,
                .suggestion = "Use environment variables for database configuration",
            });
        }
    }
    
    fn checkUnsafeOperations(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForUnsafeOps(stmt_ptr, file_path);
        }
    }
    
    fn visitForUnsafeOps(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Function => |func_stmt| {
                for (func_stmt.body.items) |body_stmt| {
                    try self.visitForUnsafeOps(body_stmt, file_path);
                }
            },
            .Expression => |expr_ptr| {
                try self.checkUnsafeExpression(expr_ptr, file_path);
            },
            else => {},
        }
    }
    
    fn checkUnsafeExpression(self: *Linter, expr_ptr: *anyopaque, file_path: []const u8) !void {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .Call => |call| {
                const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
                if (func_expr.* == .Identifier) {
                    const func_name = func_expr.Identifier;
                    
                    // Check for unsafe memory operations
                    if (std.mem.eql(u8, func_name, "malloc") or 
                       std.mem.eql(u8, func_name, "free") or
                       std.mem.eql(u8, func_name, "memcpy")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "unsafe-memory-operation",
                            .severity = .Warning,
                            .category = .Security,
                            .message = try std.fmt.allocPrint(self.allocator, "Unsafe memory operation '{s}' - use CURSED's memory management", .{func_name}),
                            .file = file_path,
                            .line = 1,
                            .column = 1,
                            .suggestion = "Use CURSED's built-in memory safety features",
                        });
                    }
                    
                    // Check for SQL injection risks
                    if (std.mem.eql(u8, func_name, "query") or std.mem.eql(u8, func_name, "execute")) {
                        // Check if any argument is string concatenation
                        for (call.arguments.items) |arg| {
                            if (self.containsStringConcatenation(arg)) {
                                try self.addIssue(LintIssue{
                                    .rule_id = "sql-injection-risk",
                                    .severity = .Error,
                                    .category = .Security,
                                    .message = "Potential SQL injection - avoid string concatenation in queries",
                                    .file = file_path,
                                    .line = 1,
                                    .column = 1,
                                    .suggestion = "Use parameterized queries",
                                });
                            }
                        }
                    }
                }
            },
            else => {},
        }
    }
    
    fn checkBufferOverflows(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForBufferOverflows(stmt_ptr, file_path);
        }
    }
    
    fn visitForBufferOverflows(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Function => |func_stmt| {
                for (func_stmt.body.items) |body_stmt| {
                    try self.visitForBufferOverflows(body_stmt, file_path);
                }
            },
            .Expression => |expr_ptr| {
                try self.checkBufferOverflowExpression(expr_ptr, file_path);
            },
            else => {},
        }
    }
    
    fn checkBufferOverflowExpression(self: *Linter, expr_ptr: *anyopaque, file_path: []const u8) !void {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .ArrayAccess => {
                // Check for array access without bounds checking
                try self.addIssue(LintIssue{
                    .rule_id = "unchecked-array-access",
                    .severity = .Warning,
                    .category = .Security,
                    .message = "Array access should include bounds checking",
                    .file = file_path,
                    .line = 1,
                    .column = 1,
                    .suggestion = "Add bounds check: ready (index < len(array))",
                });
            },
            .Call => |call| {
                const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
                if (func_expr.* == .Identifier) {
                    const func_name = func_expr.Identifier;
                    
                    // Check for unsafe string operations
                    if (std.mem.eql(u8, func_name, "strcpy") or 
                       std.mem.eql(u8, func_name, "strcat") or
                       std.mem.eql(u8, func_name, "sprintf")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "buffer-overflow-risk",
                            .severity = .Error,
                            .category = .Security,
                            .message = try std.fmt.allocPrint(self.allocator, "Function '{s}' is prone to buffer overflows", .{func_name}),
                            .file = file_path,
                            .line = 1,
                            .column = 1,
                            .suggestion = "Use safe string functions with bounds checking",
                        });
                    }
                }
            },
            else => {},
        }
    }
    
    fn checkInsecureCrypto(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForInsecureCrypto(stmt_ptr, file_path);
        }
    }
    
    fn visitForInsecureCrypto(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Function => |func_stmt| {
                for (func_stmt.body.items) |body_stmt| {
                    try self.visitForInsecureCrypto(body_stmt, file_path);
                }
            },
            .Expression => |expr_ptr| {
                try self.checkCryptoExpression(expr_ptr, file_path);
            },
            else => {},
        }
    }
    
    fn checkCryptoExpression(self: *Linter, expr_ptr: *anyopaque, file_path: []const u8) !void {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .Call => |call| {
                const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
                if (func_expr.* == .Identifier) {
                    const func_name = func_expr.Identifier;
                    
                    // Check for weak encryption
                    if (std.mem.eql(u8, func_name, "des_encrypt") or 
                       std.mem.eql(u8, func_name, "rc4_encrypt")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "weak-encryption",
                            .severity = .Error,
                            .category = .Security,
                            .message = try std.fmt.allocPrint(self.allocator, "Weak encryption algorithm '{s}' should not be used", .{func_name}),
                            .file = file_path,
                            .line = 1,
                            .column = 1,
                            .suggestion = "Use AES-GCM or ChaCha20-Poly1305",
                        });
                    }
                    
                    // Check for random number generation
                    if (std.mem.eql(u8, func_name, "rand") or std.mem.eql(u8, func_name, "srand")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "weak-random",
                            .severity = .Warning,
                            .category = .Security,
                            .message = "Standard random functions are not cryptographically secure",
                            .file = file_path,
                            .line = 1,
                            .column = 1,
                            .suggestion = "Use cryptographically secure random from stdlib/cryptz",
                        });
                    }
                    
                    // Check for hardcoded IV/salt
                    if (std.mem.eql(u8, func_name, "aes_encrypt") or std.mem.eql(u8, func_name, "encrypt")) {
                        for (call.arguments.items) |arg| {
                            const arg_expr: *ast.Expression = @ptrCast(@alignCast(arg));
                            if (arg_expr.* == .String) {
                                try self.addIssue(LintIssue{
                                    .rule_id = "hardcoded-crypto-key",
                                    .severity = .Error,
                                    .category = .Security,
                                    .message = "Encryption key/IV should not be hardcoded",
                                    .file = file_path,
                                    .line = 1,
                                    .column = 1,
                                    .suggestion = "Generate random IV and store keys securely",
                                });
                            }
                        }
                    }
                }
            },
            else => {},
        }
    }
    
    fn checkMemorySafety(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForMemorySafety(stmt_ptr, file_path);
        }
    }
    
    fn visitForMemorySafety(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Function => |func_stmt| {
                // Check for missing defer statements for cleanup
                var has_resource_allocation = false;
                var has_defer = false;
                
                for (func_stmt.body.items) |body_stmt| {
                    const body_statement: *ast.Statement = @ptrCast(@alignCast(body_stmt));
                    switch (body_statement.*) {
                        .Expression => |expr_ptr| {
                            if (self.allocatesResources(expr_ptr)) {
                                has_resource_allocation = true;
                            }
                        },
                        .Defer => {
                            has_defer = true;
                        },
                        else => {},
                    }
                    try self.visitForMemorySafety(body_stmt, file_path);
                }
                
                if (has_resource_allocation and !has_defer) {
                    try self.addIssue(LintIssue{
                        .rule_id = "missing-defer-cleanup",
                        .severity = .Warning,
                        .category = .Security,
                        .message = "Function allocates resources but lacks defer cleanup",
                        .file = file_path,
                        .line = 1,
                        .column = 1,
                        .suggestion = "Add defer statements for resource cleanup",
                    });
                }
            },
            else => {},
        }
    }
    
    fn checkErrorHandling(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForErrorHandling(stmt_ptr, file_path);
        }
    }
    
    fn visitForErrorHandling(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Function => |func_stmt| {
                for (func_stmt.body.items) |body_stmt| {
                    try self.visitForErrorHandling(body_stmt, file_path);
                }
            },
            .Expression => |expr_ptr| {
                try self.checkErrorHandlingExpression(expr_ptr, file_path);
            },
            else => {},
        }
    }
    
    fn checkErrorHandlingExpression(self: *Linter, expr_ptr: *anyopaque, file_path: []const u8) !void {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .Call => |call| {
                const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
                if (func_expr.* == .Identifier) {
                    const func_name = func_expr.Identifier;
                    
                    // Check for error-prone operations without error handling
                    if (std.mem.eql(u8, func_name, "file_open") or 
                       std.mem.eql(u8, func_name, "network_connect") or
                       std.mem.eql(u8, func_name, "parse_json")) {
                        try self.addIssue(LintIssue{
                            .rule_id = "unhandled-error",
                            .severity = .Warning,
                            .category = .Security,
                            .message = try std.fmt.allocPrint(self.allocator, "Function '{s}' can fail but error handling not visible", .{func_name}),
                            .file = file_path,
                            .line = 1,
                            .column = 1,
                            .suggestion = "Check error return values or use try/catch",
                        });
                    }
                }
            },
            else => {},
        }
    }
    
    fn checkChannelSafety(self: *Linter, file_path: []const u8, ast_tree: ast.AST) !void {
        for (ast_tree.statements.items) |stmt_ptr| {
            try self.visitForChannelSafety(stmt_ptr, file_path);
        }
    }
    
    fn visitForChannelSafety(self: *Linter, stmt_ptr: *anyopaque, file_path: []const u8) !void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        
        switch (stmt.*) {
            .Function => |func_stmt| {
                for (func_stmt.body.items) |body_stmt| {
                    try self.visitForChannelSafety(body_stmt, file_path);
                }
            },
            .Expression => |expr_ptr| {
                try self.checkChannelSafetyExpression(expr_ptr, file_path);
            },
            else => {},
        }
    }
    
    fn checkChannelSafetyExpression(self: *Linter, expr_ptr: *anyopaque, file_path: []const u8) !void {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .ChannelSend => {
                try self.addIssue(LintIssue{
                    .rule_id = "channel-deadlock-risk",
                    .severity = .Info,
                    .category = .Security,
                    .message = "Channel send operation could block indefinitely",
                    .file = file_path,
                    .line = 1,
                    .column = 1,
                    .suggestion = "Consider using buffered channels or timeouts",
                });
            },
            .ChannelReceive => {
                try self.addIssue(LintIssue{
                    .rule_id = "channel-deadlock-risk",
                    .severity = .Info,
                    .category = .Security,
                    .message = "Channel receive operation could block indefinitely",
                    .file = file_path,
                    .line = 1,
                    .column = 1,
                    .suggestion = "Consider using select statements with timeouts",
                });
            },
            else => {},
        }
    }
    
    // Helper functions for security analysis
    fn containsStringConcatenation(self: *Linter, expr_ptr: *anyopaque) bool {
        _ = self;
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .Binary => {
                // Check for string concatenation operations
                return true; // Simplified - would check operator type
            },
            else => return false,
        }
    }
    
    fn allocatesResources(self: *Linter, expr_ptr: *anyopaque) bool {
        _ = self;
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        
        switch (expr.*) {
            .Call => |call| {
                const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
                if (func_expr.* == .Identifier) {
                    const func_name = func_expr.Identifier;
                    return std.mem.eql(u8, func_name, "file_open") or
                           std.mem.eql(u8, func_name, "malloc") or
                           std.mem.eql(u8, func_name, "connect") or
                           std.mem.eql(u8, func_name, "allocate");
                }
            },
            else => {},
        }
        return false;
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
    
    // Enhanced security pattern detection
    fn looksLikeApiKey(self: *Linter, value: []const u8) bool {
        if (value.len < 16) return false;
        
        // Check for common API key patterns
        const api_prefixes = [_][]const u8{ 
            "sk_", "pk_", "ak_", "key_", "api_", "token_", "bearer_", "auth_",
            "secret_", "access_", "client_", "app_", "dev_", "prod_", "test_"
        };
        for (api_prefixes) |prefix| {
            if (std.mem.startsWith(u8, value, prefix)) {
                return true;
            }
        }
        
        // Check for AWS-style keys
        if ((std.mem.startsWith(u8, value, "AKIA") or std.mem.startsWith(u8, value, "ASIA")) and value.len >= 20) {
            return true;
        }
        
        // Check for GitHub tokens
        if (std.mem.startsWith(u8, value, "ghp_") or std.mem.startsWith(u8, value, "gho_") or 
            std.mem.startsWith(u8, value, "ghu_") or std.mem.startsWith(u8, value, "ghs_")) {
            return true;
        }
        
        // Check for long alphanumeric strings (likely API keys)
        if (value.len > 20 and value.len < 200 and self.isAlphanumeric(value)) {
            return true;
        }
        
        // Check for hex-encoded keys
        if (value.len >= 32 and value.len <= 128 and self.isHexString(value)) {
            return true;
        }
        
        return false;
    }
    
    fn looksLikePassword(self: *Linter, value: []const u8) bool {
        if (value.len < 4) return false;
        
        const password_patterns = [_][]const u8{ 
            "password", "passwd", "pwd", "secret", "pass", "auth", "credential",
            "login", "user", "admin", "root", "key", "token", "hash", "salt"
        };
        const lower_value = std.ascii.allocLowerString(self.allocator, value) catch return false;
        defer self.allocator.free(lower_value);
        
        for (password_patterns) |pattern| {
            if (std.mem.indexOf(u8, lower_value, pattern) != null) {
                // Additional checks for common password patterns
                if (value.len >= 8 and (self.containsDigits(value) or self.containsSpecialChars(value))) {
                    return true;
                }
                // Simple passwords like "password123"
                if (std.mem.eql(u8, lower_value, pattern) or 
                    std.mem.startsWith(u8, lower_value, pattern)) {
                    return true;
                }
            }
        }
        return false;
    }
    
    fn looksLikePrivateKey(_: *Linter, value: []const u8) bool {
        // PEM format keys
        if ((std.mem.indexOf(u8, value, "-----BEGIN") != null and 
             std.mem.indexOf(u8, value, "PRIVATE KEY") != null) or
            (std.mem.indexOf(u8, value, "-----BEGIN") != null and 
             std.mem.indexOf(u8, value, "RSA PRIVATE KEY") != null) or
            (std.mem.indexOf(u8, value, "-----BEGIN") != null and 
             std.mem.indexOf(u8, value, "EC PRIVATE KEY") != null)) {
            return true;
        }
        
        // SSH private keys
        if (std.mem.indexOf(u8, value, "-----BEGIN OPENSSH PRIVATE KEY-----") != null) {
            return true;
        }
        
        return false;
    }
    
    fn isAlphanumeric(_: *Linter, value: []const u8) bool {
        for (value) |char| {
            if (!std.ascii.isAlphanumeric(char) and char != '_' and char != '-') {
                return false;
            }
        }
        return true;
    }
    
    fn isHexString(_: *Linter, value: []const u8) bool {
        for (value) |char| {
            if (!std.ascii.isHex(char)) {
                return false;
            }
        }
        return true;
    }
    
    fn containsDigits(_: *Linter, value: []const u8) bool {
        for (value) |char| {
            if (std.ascii.isDigit(char)) {
                return true;
            }
        }
        return false;
    }
    
    fn containsSpecialChars(_: *Linter, value: []const u8) bool {
        const special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        for (value) |char| {
            for (special_chars) |special| {
                if (char == special) {
                    return true;
                }
            }
        }
        return false;
    }
    
    fn isPascalCase(self: *Linter, name: []const u8) bool {
        _ = self;
        if (name.len == 0) return false;
        return name[0] >= 'A' and name[0] <= 'Z';
    }
    
    fn toSnakeCase(self: *Linter, name: []const u8) ![]const u8 {
        // Convert to snake_case
        var result = ArrayList(u8){};
        defer result.deinit();
        
        for (name, 0..) |char, i| {
            if (char >= 'A' and char <= 'Z') {
                if (i > 0) try result.append(allocator, '_');
                try result.append(allocator, char + 32); // Convert to lowercase
            } else {
                try result.append(allocator, char);
            }
        }
        
        return try result.toOwnedSlice();
    }
    
    fn toPascalCase(self: *Linter, name: []const u8) ![]const u8 {
        // Convert to PascalCase
        var result = ArrayList(u8){};
        defer result.deinit();
        
        var capitalize_next = true;
        for (name) |char| {
            if (char == '_') {
                capitalize_next = true;
            } else if (capitalize_next and char >= 'a' and char <= 'z') {
                try result.append(allocator, char - 32); // Convert to uppercase
                capitalize_next = false;
            } else {
                try result.append(allocator, char);
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
        var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    
    for (issues) |issue| {
        try stdout.writer().print("{s}:{s}:{s}: {s}: {s} [{s}]\n", .{{
            issue.file,
            issue.line,
            issue.column,
            issue.severity.toString(),
            issue.message,
            issue.rule_id,
        });
        
        if (issue.suggestion) |suggestion| {
            try stdout.writer().print("  suggestion: {s}\n", .{suggestion});
        }
    }
    
    try stdout.writer().print("\nFound {s} issues\n", .{{issues.len});
}

fn printIssuesJSON(allocator: Allocator, issues: []const LintIssue) !void {
        var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    
    try stdout.writer().writeAll("{\n  \"issues\": [\n");
    
    for (issues, 0..) |issue, i| {
        try stdout.writer().print("    {{\n", .{});
        try stdout.writer().print("      \"rule_id\": \"{s}\",\n", .{issue.rule_id});
        try stdout.writer().print("      \"severity\": \"{s}\",\n", .{issue.severity.toString()});
        try stdout.writer().print("      \"category\": \"{s}\",\n", .{issue.category.toString()});
        try stdout.writer().print("      \"message\": \"{s}\",\n", .{issue.message});
        try stdout.writer().print("      \"file\": \"{s}\",\n", .{issue.file});
        try stdout.writer().print("      \"line\": {},\n", .{issue.line});
        try stdout.writer().print("      \"column\": {}", .{issue.column});
        
        if (issue.suggestion) |suggestion| {
            try stdout.writer().print(",\n      \"suggestion\": \"{s}\"", .{suggestion});
        }
        
        try stdout.writer().writeAll("\n    }");
        if (i < issues.len - 1) try stdout.writer().writeAll(",");
        try stdout.writer().writeAll("\n");
    }
    
    try stdout.writer().writeAll("  ],\n");
    try stdout.writer().print("  \"total\": {}\n", .{issues.len});
    try stdout.writer().writeAll("}\n");
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
