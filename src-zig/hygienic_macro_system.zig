//! Hygienic Macro System for CURSED
//! Provides `slay_macro!` syntax with pattern matching, code generation, and compile-time execution
//! Features: declarative macros, procedural macros, built-in macros, and comprehensive debugging

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const macro_hygiene = @import("macro_hygiene.zig");
const macro_expansion_order = @import("macro_expansion_order.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Parser = parser.Parser;
const MacroHygieneContext = macro_hygiene.MacroHygieneContext;
const MacroExpansionContext = macro_expansion_order.MacroExpansionContext;

/// Enhanced hygienic macro system with pattern matching
pub const HygienicMacroSystem = struct {
    allocator: Allocator,
    
    // Core macro infrastructure
    hygiene_context: MacroHygieneContext,
    expansion_context: MacroExpansionContext,
    
    // Pattern matching system
    pattern_matcher: PatternMatcher,
    
    // Code generation
    code_generator: CodeGenerator,
    
    // Macro definitions
    declarative_macros: HashMap([]const u8, DeclarativeMacro, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    procedural_macros: HashMap([]const u8, ProceduralMacro, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    builtin_macros: HashMap([]const u8, BuiltinMacro, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Debugging and error reporting
    debug_context: MacroDebugContext,
    error_reporter: MacroErrorReporter,
    
    // Compile-time execution
    compile_time_executor: CompileTimeExecutor,
    
    pub fn init(allocator: Allocator) !HygienicMacroSystem {
        var hygiene_context = try MacroHygieneContext.init(allocator);
        const expansion_context = try MacroExpansionContext.init(allocator, &hygiene_context);
        
        var system = HygienicMacroSystem{
            .allocator = allocator,
            .hygiene_context = hygiene_context,
            .expansion_context = expansion_context,
            .pattern_matcher = try PatternMatcher.init(allocator),
            .code_generator = try CodeGenerator.init(allocator),
            .declarative_macros = HashMap([]const u8, DeclarativeMacro, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .procedural_macros = HashMap([]const u8, ProceduralMacro, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .builtin_macros = HashMap([]const u8, BuiltinMacro, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .debug_context = try MacroDebugContext.init(allocator),
            .error_reporter = try MacroErrorReporter.init(allocator),
            .compile_time_executor = try CompileTimeExecutor.init(allocator),
        };
        
        // Register built-in macros
        try system.registerBuiltinMacros();
        
        return system;
    }
    
    pub fn deinit(self: *HygienicMacroSystem) void {
        // Clean up declarative macros
        var decl_iter = self.declarative_macros.iterator();
        while (decl_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.declarative_macros.deinit(self.allocator);
        
        // Clean up procedural macros
        var proc_iter = self.procedural_macros.iterator();
        while (proc_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.procedural_macros.deinit(self.allocator);
        
        // Clean up builtin macros
        var builtin_iter = self.builtin_macros.iterator();
        while (builtin_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.builtin_macros.deinit(self.allocator);
        
        self.compile_time_executor.deinit(self.allocator);
        self.error_reporter.deinit(self.allocator);
        self.debug_context.deinit(self.allocator);
        self.code_generator.deinit(self.allocator);
        self.pattern_matcher.deinit(self.allocator);
        self.expansion_context.deinit(self.allocator);
        self.hygiene_context.deinit(self.allocator);
    }
    
    /// Parse and register a `slay_macro!` definition
    pub fn parseSlayMacro(self: *HygienicMacroSystem, tokens: []const Token) !void {
        var parser_ctx = MacroParseContext.init(self.allocator, tokens);
        defer parser_ctx.deinit();
        
        // Expected syntax: slay_macro! macro_name { pattern => expansion, ... }
        if (tokens.len < 6) return error.InvalidMacroSyntax;
        
        // Check for slay_macro! keyword
        if (!self.isSlayMacroKeyword(tokens[0])) {
            return error.ExpectedSlayMacro;
        }
        
        const macro_name = tokens[1].lexeme;
        
        // Parse macro body
        const body_start = self.findBodyStart(tokens) orelse return error.MissingMacroBody;
        const body_end = self.findBodyEnd(tokens, body_start) orelse return error.UnterminatedMacroBody;
        
        const body_tokens = tokens[body_start + 1..body_end];
        const patterns = try self.parsePatternRules(body_tokens);
        
        // Create declarative macro
        var macro_def = DeclarativeMacro.init(self.allocator, macro_name);
        macro_def.patterns = patterns;
        macro_def.hygiene_enabled = true;
        macro_def.debug_enabled = true;
        
        try self.declarative_macros.put(try self.allocator.dupe(u8, macro_name), macro_def);
        
        // Log macro registration
        try self.debug_context.logMacroRegistration(macro_name, .Declarative);
    }
    
    /// Expand a macro call with full hygiene and debugging
    pub fn expandMacro(self: *HygienicMacroSystem, call: MacroCall) ![]Token {
        const start_time = std.time.milliTimestamp();
        
        // Start hygiene tracking
        const expansion_id = try self.hygiene_context.beginMacroExpansion(call.name, call.location.file);
        defer self.hygiene_context.endMacroExpansion() catch {};
        
        // Log expansion start
        try self.debug_context.logExpansionStart(call, expansion_id);
        
        var result: []Token = undefined;
        
        // Try different macro types
        if (self.declarative_macros.get(call.name)) |*macro_def| {
            result = try self.expandDeclarativeMacro(macro_def, call);
        } else if (self.procedural_macros.get(call.name)) |*macro_def| {
            result = try self.expandProceduralMacro(macro_def, call);
        } else if (self.builtin_macros.get(call.name)) |*macro_def| {
            result = try self.expandBuiltinMacro(macro_def, call);
        } else {
            try self.error_reporter.reportUndefinedMacro(call.name, call.location);
            return error.UndefinedMacro;
        }
        
        // Apply hygiene fixes
        try self.hygiene_context.applyHygieneFixes();
        
        // Log expansion completion
        const duration = std.time.milliTimestamp() - start_time;
        try self.debug_context.logExpansionComplete(call.name, expansion_id, duration, result.len);
        
        return result;
    }
    
    /// Expand declarative macro with pattern matching
    fn expandDeclarativeMacro(self: *HygienicMacroSystem, macro_def: *DeclarativeMacro, call: MacroCall) ![]Token {
        // Find matching pattern
        for (macro_def.patterns) |*pattern| {
            if (try self.pattern_matcher.matches(pattern.input_pattern, call.arguments)) {
                // Extract pattern captures
                const captures = try self.pattern_matcher.extractCaptures(pattern.input_pattern, call.arguments);
                defer self.allocator.free(captures);
                
                // Generate expansion
                const expansion = try self.code_generator.expandPattern(pattern.output_template, captures);
                
                // Apply hygiene if enabled
                if (macro_def.hygiene_enabled) {
                    return try self.applyHygiene(expansion, call);
                }
                
                return expansion;
            }
        }
        
        try self.error_reporter.reportNoMatchingPattern(call.name, call.location);
        return error.NoMatchingPattern;
    }
    
    /// Expand procedural macro with compile-time execution
    fn expandProceduralMacro(self: *HygienicMacroSystem, macro_def: *ProceduralMacro, call: MacroCall) ![]Token {
        // Execute procedural macro function
        const input_ast = try self.parseTokensToAST(call.arguments);
        defer self.deallocateAST(input_ast);
        
        const output_ast = try self.compile_time_executor.executeProcMacro(macro_def.function, input_ast);
        defer self.deallocateAST(output_ast);
        
        // Convert AST back to tokens
        const expansion = try self.code_generator.astToTokens(output_ast);
        
        // Apply hygiene
        return try self.applyHygiene(expansion, call);
    }
    
    /// Expand built-in macro
    fn expandBuiltinMacro(self: *HygienicMacroSystem, macro_def: *BuiltinMacro, call: MacroCall) ![]Token {
        return try macro_def.expansion_function(self, call);
    }
    
    /// Apply hygiene transformations to expansion
    fn applyHygiene(self: *HygienicMacroSystem, tokens: []Token, call: MacroCall) ![]Token {
        _ = call; // Mark as intentionally unused
        var hygienic_tokens = ArrayList(Token){};
        defer hygienic_tokens.deinit();
        
        for (tokens) |token| {
            if (token.kind == .Identifier) {
                // Check if this identifier needs hygiene renaming
                const resolved = try self.hygiene_context.resolveSymbol(token.lexeme);
                if (resolved) |renamed| {
                    var hygienic_token = token;
                    hygienic_token.lexeme = renamed;
                    try hygienic_tokens.append(self.allocator, hygienic_token);
                } else {
                    try hygienic_tokens.append(self.allocator, token);
                }
            } else {
                try hygienic_tokens.append(self.allocator, token);
            }
        }
        
        return hygienic_tokens.toOwnedSlice();
    }
    
    /// Register built-in macros
    fn registerBuiltinMacros(self: *HygienicMacroSystem) !void {
        // debug_print! macro
        var debug_print = BuiltinMacro.init("debug_print", debugPrintExpansion);
        debug_print.description = "Conditional debug printing based on DEBUG_MODE";
        try self.builtin_macros.put(try self.allocator.dupe(u8, "debug_print"), debug_print);
        
        // derive_json! macro
        var derive_json = BuiltinMacro.init("derive_json", deriveJsonExpansion);
        derive_json.description = "Generate JSON serialization code for structs";
        try self.builtin_macros.put(try self.allocator.dupe(u8, "derive_json"), derive_json);
        
        // format! macro
        var format_macro = BuiltinMacro.init("format", formatExpansion);
        format_macro.description = "Compile-time string formatting";
        try self.builtin_macros.put(try self.allocator.dupe(u8, "format"), format_macro);
        
        // assert! macro
        var assert_macro = BuiltinMacro.init("assert", assertExpansion);
        assert_macro.description = "Debug assertions with optional messages";
        try self.builtin_macros.put(try self.allocator.dupe(u8, "assert"), assert_macro);
        
        // vec! macro
        var vec_macro = BuiltinMacro.init("vec", vecExpansion);
        vec_macro.description = "Array literal syntax sugar";
        try self.builtin_macros.put(try self.allocator.dupe(u8, "vec"), vec_macro);
    }
    
    // Helper functions
    fn isSlayMacroKeyword(self: *HygienicMacroSystem, token: Token) bool {
        _ = self;
        return token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "slay_macro!");
    }
    
    fn findBodyStart(self: *HygienicMacroSystem, tokens: []const Token) ?usize {
        _ = self;
        for (tokens, 0..) |token, i| {
            if (token.kind == .LeftBrace) return i;
        }
        return null;
    }
    
    fn findBodyEnd(self: *HygienicMacroSystem, tokens: []const Token, start: usize) ?usize {
        _ = self;
        var brace_count: i32 = 1;
        var i = start + 1;
        
        while (i < tokens.len and brace_count > 0) {
            switch (tokens[i].kind) {
                .LeftBrace => brace_count += 1,
                .RightBrace => brace_count -= 1,
                else => {}
        }
            i += 1;
        }
        
        return if (brace_count == 0) i - 1 else null;
    }
    
    fn parseTokensToAST(self: *HygienicMacroSystem, tokens: []const Token) !*ast.Expression {
        var parser_instance = Parser.init(self.allocator, tokens);
        defer parser_instance.deinit();
        
        return try parser_instance.parseExpression();
    }
    
    fn deallocateAST(self: *HygienicMacroSystem, ast_node: *ast.Expression) void {
        // In a full implementation, this would properly deallocate the AST
        _ = self;
        _ = ast_node;
    }
    
    fn parsePatternRules(self: *HygienicMacroSystem, tokens: []const Token) ![]PatternRule {
        var rules = ArrayList(PatternRule){};
        defer rules.deinit();
        
        var i: usize = 0;
        while (i < tokens.len) {
            // Parse pattern => template format
            const pattern_start = i;
            
            // Find => separator
            var arrow_pos: ?usize = null;
            while (i < tokens.len) {
                if (tokens[i].kind == .Equal and i + 1 < tokens.len and tokens[i + 1].kind == .Greater) {
                    arrow_pos = i;
                    break;
                }
                i += 1;
            }
            
            if (arrow_pos == null) break;
            
            // Find end of template (comma or end)
            var template_end = arrow_pos.? + 2;
            var brace_count: i32 = 0;
            
            while (template_end < tokens.len) {
                switch (tokens[template_end].kind) {
                    .LeftBrace => brace_count += 1,
                    .RightBrace => brace_count -= 1,
                    .Comma => if (brace_count == 0) break,
                    else => {}
        }
                template_end += 1;
            }
            
            // Create pattern rule
            const input_pattern = try self.pattern_matcher.parsePattern(tokens[pattern_start..arrow_pos.?]);
            const output_template = try self.code_generator.parseTemplate(tokens[arrow_pos.? + 2..template_end]);
            
            try rules.append(PatternRule{
                .input_pattern = input_pattern,
                .output_template = output_template,
            });
            
            i = template_end + 1; // Skip comma
        }
        
        return rules.toOwnedSlice();
    }
    
    /// Generate macro debugging report
    pub fn generateDebugReport(self: *HygienicMacroSystem) ![]const u8 {
        return try self.debug_context.generateReport();
    }
    
    /// Get macro statistics
    pub fn getStatistics(self: *HygienicMacroSystem) MacroStatistics {
        return MacroStatistics{
            .declarative_macros_defined = self.declarative_macros.count(),
            .procedural_macros_defined = self.procedural_macros.count(),
            .builtin_macros_available = self.builtin_macros.count(),
            .total_expansions = self.debug_context.total_expansions,
            .hygiene_violations_detected = self.hygiene_context.hygiene_violations.items.len,
            .expansion_cache_hits = self.expansion_context.expansion_cache.count(),
        };
    }
};

/// Declarative macro definition with pattern matching
const DeclarativeMacro = struct {
    name: []const u8,
    patterns: []PatternRule,
    hygiene_enabled: bool,
    debug_enabled: bool,
    max_expansion_depth: usize,
    
    pub fn init(allocator: Allocator, name: []const u8) DeclarativeMacro {
        _ = allocator;
        return DeclarativeMacro{
            .name = name,
            .patterns = &[_]PatternRule{},
            .hygiene_enabled = true,
            .debug_enabled = false,
            .max_expansion_depth = 32,
        };
    }
    
    pub fn deinit(self: *DeclarativeMacro) void {
        // Free patterns
        for (self.patterns) |*pattern| {
            pattern.deinit();
        }
    }
};

/// Pattern rule for declarative macros
const PatternRule = struct {
    input_pattern: Pattern,
    output_template: Template,
    
    pub fn deinit(self: *PatternRule) void {
        self.input_pattern.deinit(self.allocator);
        self.output_template.deinit(self.allocator);
    }
};

/// Procedural macro definition
const ProceduralMacro = struct {
    name: []const u8,
    function: ProceduralMacroFunction,
    signature: ProcMacroSignature,
    
    pub fn init(name: []const u8, function: ProceduralMacroFunction) ProceduralMacro {
        return ProceduralMacro{
            .name = name,
            .function = function,
            .signature = ProcMacroSignature.init(),
        };
    }
    
    pub fn deinit(self: *ProceduralMacro) void {
        _ = self;
        // Nothing to clean up for now
    }
};

/// Built-in macro definition
const BuiltinMacro = struct {
    name: []const u8,
    expansion_function: BuiltinMacroFunction,
    description: []const u8,
    
    pub fn init(name: []const u8, expansion_function: BuiltinMacroFunction) BuiltinMacro {
        return BuiltinMacro{
            .name = name,
            .expansion_function = expansion_function,
            .description = "",
        };
    }
    
    pub fn deinit(self: *BuiltinMacro) void {
        _ = self;
        // Nothing to clean up for now
    }
};

/// Pattern matcher for declarative macros
const PatternMatcher = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) !PatternMatcher {
        return PatternMatcher{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *PatternMatcher) void {
        _ = self;
    }
    
    pub fn parsePattern(self: *PatternMatcher, tokens: []const Token) !Pattern {
        return Pattern.parse(self.allocator, tokens);
    }
    
    pub fn matches(self: *PatternMatcher, pattern: Pattern, tokens: []const Token) !bool {
        _ = self;
        return pattern.matches(tokens);
    }
    
    pub fn extractCaptures(self: *PatternMatcher, pattern: Pattern, tokens: []const Token) ![]PatternCapture {
        return pattern.extractCaptures(self.allocator, tokens);
    }
};

/// Code generator for macro expansions
const CodeGenerator = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) !CodeGenerator {
        return CodeGenerator{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *CodeGenerator) void {
        _ = self;
    }
    
    pub fn parseTemplate(self: *CodeGenerator, tokens: []const Token) !Template {
        return Template.parse(self.allocator, tokens);
    }
    
    pub fn expandPattern(self: *CodeGenerator, template: Template, captures: []PatternCapture) ![]Token {
        return template.expand(self.allocator, captures);
    }
    
    pub fn astToTokens(self: *CodeGenerator, ast_node: *ast.Expression) ![]Token {
        _ = self;
        _ = ast_node;
        // In a full implementation, this would convert AST back to tokens
        return &[_]Token{};
    }
};

/// Pattern matching types
const Pattern = struct {
    elements: []PatternElement,
    allocator: Allocator,
    
    const PatternElement = union(enum) {
        Literal: Token,
        Capture: PatternCapture,
        Repetition: struct {
            pattern: *Pattern,
            min: usize,
            max: ?usize,
            separator: ?Token,
        },
        Alternative: []Pattern,
    };
    
    pub fn parse(allocator: Allocator, tokens: []const Token) !Pattern {
        var elements = ArrayList(PatternElement){};
        defer elements.deinit();
        
        var i: usize = 0;
        while (i < tokens.len) {
            const token = tokens[i];
            
            if (token.kind == .Dollar) {
                // Pattern capture: $name:type
                if (i + 1 < tokens.len and tokens[i + 1].kind == .Identifier) {
                    const capture_name = tokens[i + 1].lexeme;
                    var capture_type = CaptureType.Expression;
                    
                    if (i + 3 < tokens.len and tokens[i + 2].kind == .Colon and tokens[i + 3].kind == .Identifier) {
                        capture_type = parseCaptureType(tokens[i + 3].lexeme);
                        i += 4;
                    } else {
                        i += 2;
                    }
                    
                    try elements.append(PatternElement{
                        .Capture = PatternCapture{
                            .name = try allocator.dupe(u8, capture_name),
                            .capture_type = capture_type,
                            .value = null,
                        }
        });
                } else {
                    return error.InvalidPatternCapture;
                }
            } else {
                // Literal token
                try elements.append(PatternElement{
                    .Literal = token,
                });
                i += 1;
            }
        }
        
        return Pattern{
            .elements = try elements.toOwnedSlice(),
            .allocator = allocator,
        };
    }
    
    pub fn matches(self: Pattern, tokens: []const Token) bool {
        return self.matchesRecursive(tokens, 0, 0) != null;
    }
    
    fn matchesRecursive(self: Pattern, tokens: []const Token, token_index: usize, pattern_index: usize) ?usize {
        if (pattern_index >= self.elements.len) {
            return if (token_index == tokens.len) token_index else null;
        }
        
        const element = self.elements[pattern_index];
        
        switch (element) {
            .Literal => |literal| {
                if (token_index >= tokens.len) return null;
                const token = tokens[token_index];
                
                if (literal.kind == token.kind and std.mem.eql(u8, literal.lexeme, token.lexeme)) {
                    return self.matchesRecursive(tokens, token_index + 1, pattern_index + 1);
                }
                return null;
            },
            .Capture => |capture| {
                const consumed = self.consumeCapture(tokens[token_index..], capture.capture_type);
                if (consumed > 0) {
                    return self.matchesRecursive(tokens, token_index + consumed, pattern_index + 1);
                }
                return null;
            },
            .Repetition => {
                // Handle repetition patterns
                return null; // Simplified for now
            },
            .Alternative => {
                // Handle alternative patterns
                return null; // Simplified for now
            }
        }
    }
    
    fn consumeCapture(self: Pattern, tokens: []const Token, capture_type: CaptureType) usize {
        _ = self;
        if (tokens.len == 0) return 0;
        
        return switch (capture_type) {
            .Expression => 1, // Simplified: consume one token
            .Statement => 1,
            .Type => 1,
            .Identifier => if (tokens[0].kind == .Identifier) 1 else 0,
            .Literal => if (tokens[0].kind == .Number or tokens[0].kind == .String) 1 else 0,
            .Block => 1, // Would need to count braces
        };
    }
    
    pub fn extractCaptures(self: Pattern, allocator: Allocator, tokens: []const Token) ![]PatternCapture {
        var captures = ArrayList(PatternCapture){};
        defer captures.deinit();
        
        var token_index: usize = 0;
        
        for (self.elements) |element| {
            switch (element) {
                .Capture => |capture| {
                    const consumed = self.consumeCapture(tokens[token_index..], capture.capture_type);
                    if (consumed > 0) {
                        var new_capture = capture;
                        new_capture.value = try allocator.dupe(Token, tokens[token_index..token_index + consumed]);
                        try captures.append(self.allocator, new_capture);
                        token_index += consumed;
                    }
                },
                .Literal => {
                    token_index += 1;
                },
                else => {
                    // Handle other pattern types
                }
        }
        }
        
        return captures.toOwnedSlice();
    }
    
    pub fn deinit(self: *Pattern) void {
        for (self.elements) |*element| {
            switch (element.*) {
                .Capture => |*capture| {
                    self.allocator.free(capture.name);
                    if (capture.value) |value| {
                        self.allocator.free(value);
                    }
                },
                else => {}
        }
        }
        self.allocator.free(self.elements);
    }
};

const PatternCapture = struct {
    name: []const u8,
    capture_type: CaptureType,
    value: ?[]Token,
};

const CaptureType = enum {
    Expression,
    Statement,
    Type,
    Identifier,
    Literal,
    Block,
};

fn parseCaptureType(type_name: []const u8) CaptureType {
    if (std.mem.eql(u8, type_name, "expr")) return .Expression;
    if (std.mem.eql(u8, type_name, "stmt")) return .Statement;
    if (std.mem.eql(u8, type_name, "ty")) return .Type;
    if (std.mem.eql(u8, type_name, "ident")) return .Identifier;
    if (std.mem.eql(u8, type_name, "literal")) return .Literal;
    if (std.mem.eql(u8, type_name, "block")) return .Block;
    return .Expression; // Default
}

/// Template for code generation
const Template = struct {
    elements: []TemplateElement,
    allocator: Allocator,
    
    const TemplateElement = union(enum) {
        Literal: Token,
        Substitution: []const u8, // Capture name
        Conditional: struct {
            condition: []const u8,
            then_template: *Template,
            else_template: ?*Template,
        },
        Repetition: struct {
            capture_name: []const u8,
            template: *Template,
            separator: ?Token,
        }
        };
    
    pub fn parse(allocator: Allocator, tokens: []const Token) !Template {
        var elements = ArrayList(TemplateElement){};
        defer elements.deinit();
        
        var i: usize = 0;
        while (i < tokens.len) {
            const token = tokens[i];
            
            if (token.kind == .Dollar) {
                // Template substitution: $capture_name
                if (i + 1 < tokens.len and tokens[i + 1].kind == .Identifier) {
                    const capture_name = try allocator.dupe(u8, tokens[i + 1].lexeme);
                    try elements.append(TemplateElement{
                        .Substitution = capture_name,
                    });
                    i += 2;
                } else {
                    return error.InvalidTemplateSubstitution;
                }
            } else {
                // Literal token
                try elements.append(TemplateElement{
                    .Literal = token,
                });
                i += 1;
            }
        }
        
        return Template{
            .elements = try elements.toOwnedSlice(),
            .allocator = allocator,
        };
    }
    
    pub fn expand(self: Template, allocator: Allocator, captures: []PatternCapture) ![]Token {
        _ = allocator;
        var result = ArrayList(Token){};
        defer result.deinit();
        
        for (self.elements) |element| {
            switch (element) {
                .Literal => |literal| {
                    try result.append(self.allocator, literal);
                },
                .Substitution => |capture_name| {
                    // Find capture and substitute
                    for (captures) |capture| {
                        if (std.mem.eql(u8, capture.name, capture_name)) {
                            if (capture.value) |tokens| {
                                try result.appendSlice(tokens);
                            }
                            break;
                        }
                    }
                },
                else => {
                    // Handle other template elements
                }
        }
        }
        
        return result.toOwnedSlice();
    }
    
    pub fn deinit(self: *Template) void {
        for (self.elements) |*element| {
            switch (element.*) {
                .Substitution => |capture_name| {
                    self.allocator.free(capture_name);
                },
                else => {}
        }
        }
        self.allocator.free(self.elements);
    }
};

/// Built-in macro expansions
fn debugPrintExpansion(system: *HygienicMacroSystem, call: MacroCall) ![]Token {
    _ = system;
    // Generate: ready (DEBUG_MODE) { vibez.spill("[DEBUG]", $msg) }
    var result = ArrayList(Token){};
    defer result.deinit();
    
    // ready (DEBUG_MODE) {
    try result.append(Token{ .kind = .Identifier, .lexeme = "ready", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .LeftParen, .lexeme = "(", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .Identifier, .lexeme = "DEBUG_MODE", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .RightParen, .lexeme = ")", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .LeftBrace, .lexeme = "{", .line = call.location.line, .column = call.location.column, .offset = 0 });
    
    // vibez.spill("[DEBUG]", $msg)
    try result.append(Token{ .kind = .Identifier, .lexeme = "vibez", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .Dot, .lexeme = ".", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .Identifier, .lexeme = "spill", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .LeftParen, .lexeme = "(", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .String, .lexeme = "\"[DEBUG]\"", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .Comma, .lexeme = ",", .line = call.location.line, .column = call.location.column, .offset = 0 });
    
    // Add arguments
    try result.appendSlice(call.arguments);
    
    // Close the call and block
    try result.append(Token{ .kind = .RightParen, .lexeme = ")", .line = call.location.line, .column = call.location.column, .offset = 0 });
    try result.append(Token{ .kind = .RightBrace, .lexeme = "}", .line = call.location.line, .column = call.location.column, .offset = 0 });
    
    return result.toOwnedSlice();
}

fn deriveJsonExpansion(system: *HygienicMacroSystem, call: MacroCall) ![]Token {
    // Generate JSON serialization code for structs
    _ = system;
    _ = call;
    return &[_]Token{}; // Simplified implementation
}

fn formatExpansion(system: *HygienicMacroSystem, call: MacroCall) ![]Token {
    // Compile-time string formatting
    _ = system;
    _ = call;
    return &[_]Token{}; // Simplified implementation
}

fn assertExpansion(system: *HygienicMacroSystem, call: MacroCall) ![]Token {
    // Debug assertions
    _ = system;
    _ = call;
    return &[_]Token{}; // Simplified implementation
}

fn vecExpansion(system: *HygienicMacroSystem, call: MacroCall) ![]Token {
    // Array literal syntax sugar
    _ = system;
    _ = call;
    return &[_]Token{}; // Simplified implementation
}

// Supporting types and structures
const MacroCall = macro_expansion_order.MacroExpansionContext.MacroCall;
const ProceduralMacroFunction = *const fn (*ast.Expression) *ast.Expression;
const BuiltinMacroFunction = *const fn (*HygienicMacroSystem, MacroCall) anyerror![]Token;

const ProcMacroSignature = struct {
    input_type: type,
    output_type: type,
    
    pub fn init() ProcMacroSignature {
        return ProcMacroSignature{
            .input_type = *ast.Expression,
            .output_type = *ast.Expression,
        };
    }
};

const MacroDebugContext = struct {
    allocator: Allocator,
    expansion_log: ArrayList(ExpansionLogEntry),
    total_expansions: usize,
    
    const ExpansionLogEntry = struct {
        macro_name: []const u8,
        expansion_id: u32,
        start_time: i64,
        duration_ms: i64,
        result_tokens: usize,
        location: []const u8,
    };
    
    pub fn init(allocator: Allocator) !MacroDebugContext {
        return MacroDebugContext{
            .allocator = allocator,
            .expansion_log = ArrayList(ExpansionLogEntry){},
            .total_expansions = 0,
        };
    }
    
    pub fn deinit(self: *MacroDebugContext) void {
        self.expansion_log.deinit(self.allocator);
    }
    
    pub fn logMacroRegistration(self: *MacroDebugContext, name: []const u8, macro_type: MacroType) !void {
        _ = self;
        _ = name;
        _ = macro_type;
        // Log macro registration
    }
    
    pub fn logExpansionStart(self: *MacroDebugContext, call: MacroCall, expansion_id: u32) !void {
        _ = self;
        _ = call;
        _ = expansion_id;
        // Log expansion start
    }
    
    pub fn logExpansionComplete(self: *MacroDebugContext, name: []const u8, expansion_id: u32, duration: i64, result_tokens: usize) !void {
        try self.expansion_log.append(ExpansionLogEntry{
            .macro_name = try self.allocator.dupe(u8, name),
            .expansion_id = expansion_id,
            .start_time = std.time.milliTimestamp(),
            .duration_ms = duration,
            .result_tokens = result_tokens,
            .location = "",
        });
        self.total_expansions += 1;
    }
    
    pub fn generateReport(self: *MacroDebugContext) ![]const u8 {
        var report = ArrayList(u8){};
        defer report.deinit();
        
        try report.writer().print("Macro Debug Report\n");
        try report.writer().print("Total expansions: {d}\n", .{self.total_expansions});
        
        for (self.expansion_log.items) |entry| {
            try report.writer().print("  {s}: {d}ms, {d} tokens\n", .{ entry.macro_name, entry.duration_ms, entry.result_tokens });
        }
        
        return report.toOwnedSlice();
    }
};

const MacroErrorReporter = struct {
    allocator: Allocator,
    errors: ArrayList(MacroError),
    
    const MacroError = struct {
        kind: ErrorKind,
        message: []const u8,
        location: []const u8,
        
        const ErrorKind = enum {
            UndefinedMacro,
            NoMatchingPattern,
            ExpansionError,
            HygieneViolation,
            RecursionLimit,
        };
    };
    
    pub fn init(allocator: Allocator) !MacroErrorReporter {
        return MacroErrorReporter{
            .allocator = allocator,
            .errors = ArrayList(MacroError){}
        };
    }
    
    pub fn deinit(self: *MacroErrorReporter) void {
        self.errors.deinit(self.allocator);
    }
    
    pub fn reportUndefinedMacro(self: *MacroErrorReporter, name: []const u8, location: MacroCall.SourceLocation) !void {
        try self.errors.append(MacroError{
            .kind = .UndefinedMacro,
            .message = try std.fmt.allocPrint(self.allocator, "Undefined macro: {s}", .{name}),
            .location = try self.allocator.dupe(u8, location.file),
        });
    }
    
    pub fn reportNoMatchingPattern(self: *MacroErrorReporter, name: []const u8, location: MacroCall.SourceLocation) !void {
        try self.errors.append(MacroError{
            .kind = .NoMatchingPattern,
            .message = try std.fmt.allocPrint(self.allocator, "No matching pattern for macro: {s}", .{name}),
            .location = try self.allocator.dupe(u8, location.file),
        });
    }
};

const CompileTimeExecutor = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) !CompileTimeExecutor {
        return CompileTimeExecutor{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *CompileTimeExecutor) void {
        _ = self;
    }
    
    pub fn executeProcMacro(self: *CompileTimeExecutor, function: ProceduralMacroFunction, input: *ast.Expression) !*ast.Expression {
        _ = self;
        return function(input);
    }
};

const MacroParseContext = struct {
    allocator: Allocator,
    tokens: []const Token,
    
    pub fn init(allocator: Allocator, tokens: []const Token) MacroParseContext {
        return MacroParseContext{
            .allocator = allocator,
            .tokens = tokens,
        };
    }
    
    pub fn deinit(self: *MacroParseContext) void {
        _ = self;
    }
};

const MacroType = enum {
    Declarative,
    Procedural,
    Builtin,
};

const MacroStatistics = struct {
    declarative_macros_defined: u32,
    procedural_macros_defined: u32,
    builtin_macros_available: u32,
    total_expansions: usize,
    hygiene_violations_detected: usize,
    expansion_cache_hits: u32,
};

// Test cases
test "basic declarative macro parsing" {
    const test_tokens = [_]Token{
        Token{ .kind = .Identifier, .lexeme = "slay_macro!", .line = 1, .column = 1, .offset = 0 },
        Token{ .kind = .Identifier, .lexeme = "test_macro", .line = 1, .column = 12, .offset = 11 },
        Token{ .kind = .LeftBrace, .lexeme = "{", .line = 1, .column = 23, .offset = 22 },
        Token{ .kind = .Dollar, .lexeme = "$", .line = 1, .column = 25, .offset = 24 },
        Token{ .kind = .Identifier, .lexeme = "x", .line = 1, .column = 26, .offset = 25 },
        Token{ .kind = .Equal, .lexeme = "=", .line = 1, .column = 28, .offset = 27 },
        Token{ .kind = .Greater, .lexeme = ">", .line = 1, .column = 29, .offset = 28 },
        Token{ .kind = .Dollar, .lexeme = "$", .line = 1, .column = 31, .offset = 30 },
        Token{ .kind = .Identifier, .lexeme = "x", .line = 1, .column = 32, .offset = 31 },
        Token{ .kind = .RightBrace, .lexeme = "}", .line = 1, .column = 34, .offset = 33 }
        };
    
    var system = try HygienicMacroSystem.init(std.testing.allocator);
    defer system.deinit();
    
    try system.parseSlayMacro(&test_tokens);
    
    // Verify macro was registered
    try std.testing.expect(system.declarative_macros.contains("test_macro"));
}

test "built-in macro expansion" {
    var system = try HygienicMacroSystem.init(std.testing.allocator);
    defer system.deinit();
    
    const call = MacroCall{
        .name = "debug_print",
        .arguments = &[_]Token{
            Token{ .kind = .String, .lexeme = "\"Hello\"", .line = 1, .column = 1, .offset = 0 }
        },
        .location = .{
            .file = "test.💀",
            .line = 1,
            .column = 1,
            .byte_offset = 0,
        },
        .context_tokens = &[_]Token{}
        };
    
    const result = try system.expandMacro(call);
    defer system.allocator.free(result);
    
    // Verify expansion contains expected tokens
    try std.testing.expect(result.len > 0);
}
