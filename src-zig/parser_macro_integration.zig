//! Parser Macro Integration
//! Integrates macro expansion system with the main CURSED parser

const std = @import("std");
const ArrayList = std.ArrayList;
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
const MacroCall = macro_expansion_order.MacroExpansionContext.MacroCall;
const MacroDefinition = macro_expansion_order.MacroExpansionContext.MacroDefinition;

/// Enhanced parser with macro expansion capabilities
pub const MacroAwareParser = struct {
    base_parser: Parser,
    hygiene_context: MacroHygieneContext,
    expansion_context: MacroExpansionContext,
    original_tokens: []const Token,
    expanded_tokens: ?[]Token,
    macro_pass_complete: bool,
    
    // Macro processing state
    pending_macro_calls: ArrayList(PendingMacroCall),
    macro_definition_mode: bool,
    current_macro_def: ?MacroDefinition,
    
    const PendingMacroCall = struct {
        call: MacroCall,
        token_range: struct { start: usize, end: usize },
        replacement_needed: bool,
    };
    
    pub fn init(allocator: Allocator, tokens: []const Token) !MacroAwareParser {
        var hygiene_context = try MacroHygieneContext.init(allocator);
        const expansion_context = try MacroExpansionContext.init(allocator, &hygiene_context);
        
        return MacroAwareParser{
            .base_parser = Parser.init(allocator, tokens),
            .hygiene_context = hygiene_context,
            .expansion_context = expansion_context,
            .original_tokens = tokens,
            .expanded_tokens = null,
            .macro_pass_complete = false,
            .pending_macro_calls = .empty,
            .macro_definition_mode = false,
            .current_macro_def = null,
        };
    }
    
    pub fn deinit(self: *MacroAwareParser) void {
        if (self.expanded_tokens) |tokens| {
            self.base_parser.allocator.free(tokens);
        }
        
        for (self.pending_macro_calls.items) |*call| {
            self.base_parser.allocator.free(call.call.name);
            self.base_parser.allocator.free(call.call.arguments);
        }
        self.pending_macro_calls.deinit(self.allocator);
        
        self.expansion_context.deinit(self.allocator);
        self.hygiene_context.deinit(self.allocator);
        self.base_parser.deinit(self.allocator);
    }
    
    /// Parse with macro expansion preprocessing
    pub fn parseProgram(self: *MacroAwareParser) !*ast.Program {
        // First pass: scan for macro definitions and calls
        try self.scanForMacros();
        
        // Second pass: expand macros in correct order
        try self.expandMacros();
        
        // Third pass: parse the expanded token stream
        return self.parseExpandedProgram();
    }
    
    /// Scan tokens for macro definitions and calls
    fn scanForMacros(self: *MacroAwareParser) !void {
        var i: usize = 0;
        while (i < self.original_tokens.len) {
            const token = self.original_tokens[i];
            
            switch (token.kind) {
                .At => {
                    // Potential macro call: @macro_name or @macro
                    if (i + 1 < self.original_tokens.len and 
                        self.original_tokens[i + 1].kind == .Identifier) {
                        
                        const call_end = try self.findMacroCallEnd(i);
                        const macro_call = try self.parseMacroCallFromTokens(self.original_tokens[i..call_end]);
                        
                        try self.pending_macro_calls.append(PendingMacroCall{
                            .call = macro_call,
                            .token_range = .{ .start = i, .end = call_end },
                            .replacement_needed = true,
                        });
                        
                        i = call_end;
                    } else {
                        i += 1;
                    }
                },
                .Hash => {
                    // Potential macro definition: #macro or #define
                    if (i + 1 < self.original_tokens.len and 
                        self.original_tokens[i + 1].kind == .Identifier) {
                        
                        const directive = self.original_tokens[i + 1].lexeme;
                        if (std.mem.eql(u8, directive, "macro") or std.mem.eql(u8, directive, "define")) {
                            const def_end = try self.findMacroDefinitionEnd(i);
                            const macro_def = try self.parseMacroDefinitionFromTokens(self.original_tokens[i..def_end]);
                            
                            try self.expansion_context.defineMacro(macro_def);
                            
                            // Mark these tokens for removal in expanded stream
                            try self.pending_macro_calls.append(PendingMacroCall{
                                .call = MacroCall{
                                    .name = "",
                                    .arguments = &[_]Token{},
                                    .location = .{
                                        .file = self.base_parser.file_path,
                                        .line = token.line,
                                        .column = token.column,
                                        .byte_offset = 0,
                                    },
                                    .context_tokens = &[_]Token{},
                                },
                                .token_range = .{ .start = i, .end = def_end },
                                .replacement_needed = false, // Remove, don't replace
                            });
                            
                            i = def_end;
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                },
                else => {
                    i += 1;
                },
            }
        }
    }
    
    /// Expand all discovered macros in proper order
    fn expandMacros(self: *MacroAwareParser) !void {
        if (self.pending_macro_calls.items.len == 0) {
            // No macros to expand, use original tokens
            self.expanded_tokens = try self.base_parser.allocator.dupe(Token, self.original_tokens);
            self.macro_pass_complete = true;
            return;
        }
        
        // Queue all macro calls for expansion
        for (self.pending_macro_calls.items) |*pending| {
            if (pending.replacement_needed) {
                _ = try self.expansion_context.queueMacroExpansion(pending.call);
            }
        }
        
        // Process expansions in order
        const expansion_results = try self.expansion_context.processExpansions();
        defer self.base_parser.allocator.free(expansion_results);
        
        // Build final token stream with replacements
        self.expanded_tokens = try self.buildExpandedTokenStream(expansion_results);
        self.macro_pass_complete = true;
        
        // Update base parser to use expanded tokens
        self.base_parser.tokens = self.expanded_tokens.?;
        self.base_parser.current = 0;
    }
    
    /// Parse the macro-expanded program
    fn parseExpandedProgram(self: *MacroAwareParser) !*ast.Program {
        if (!self.macro_pass_complete) {
            return error.MacroExpansionIncomplete;
        }
        
        // Use base parser to parse expanded tokens
        return self.base_parser.parseProgram();
    }
    
    /// Find end of macro call starting at position
    fn findMacroCallEnd(self: *MacroAwareParser, start: usize) !usize {
        var i = start + 1; // Skip @
        
        // Skip macro name
        if (i < self.original_tokens.len and self.original_tokens[i].kind == .Identifier) {
            i += 1;
        }
        
        // Check for function-like macro call
        if (i < self.original_tokens.len and self.original_tokens[i].kind == .LeftParen) {
            var paren_count: i32 = 1;
            i += 1;
            
            while (i < self.original_tokens.len and paren_count > 0) {
                switch (self.original_tokens[i].kind) {
                    .LeftParen => paren_count += 1,
                    .RightParen => paren_count -= 1,
                    else => {},
                }
                i += 1;
            }
        }
        
        return i;
    }
    
    /// Find end of macro definition starting at position
    fn findMacroDefinitionEnd(self: *MacroAwareParser, start: usize) !usize {
        var i = start;
        
        // Skip to end of line or block
        while (i < self.original_tokens.len) {
            const token = self.original_tokens[i];
            
            switch (token.kind) {
                .Newline => {
                    return i + 1;
                },
                .LeftBrace => {
                    // Multi-line macro definition
                    var brace_count: i32 = 1;
                    i += 1;
                    
                    while (i < self.original_tokens.len and brace_count > 0) {
                        switch (self.original_tokens[i].kind) {
                            .LeftBrace => brace_count += 1,
                            .RightBrace => brace_count -= 1,
                            else => {},
                        }
                        i += 1;
                    }
                    return i;
                },
                else => {
                    i += 1;
                },
            }
        }
        
        return self.original_tokens.len;
    }
    
    /// Parse macro call from token sequence
    fn parseMacroCallFromTokens(self: *MacroAwareParser, tokens: []const Token) !MacroCall {
        if (tokens.len < 2) return error.InvalidMacroCall;
        
        const name = tokens[1].lexeme;
        var arguments: []Token = &[_]Token{};
        var context_start: usize = 0;
        var context_end: usize = 0;
        
        // Extract arguments if function-like
        if (tokens.len > 3 and tokens[2].kind == .LeftParen) {
            // Find matching closing paren
            var paren_count: i32 = 1;
            const arg_start: usize = 3;
            var arg_end: usize = 3;
            
            for (tokens[3..], 3..) |token, i| {
                switch (token.kind) {
                    .LeftParen => paren_count += 1,
                    .RightParen => {
                        paren_count -= 1;
                        if (paren_count == 0) {
                            arg_end = i;
                            break;
                        }
                    },
                    else => {},
                }
            }
            
            if (arg_end > arg_start) {
                arguments = try self.base_parser.allocator.dupe(Token, tokens[arg_start..arg_end]);
            }
        }
        
        // Extract context tokens (surrounding tokens for better analysis)
        if (self.original_tokens.len > 0) {
            const start_offset = @as(isize, @intCast(@intFromPtr(tokens.ptr))) - @as(isize, @intCast(@intFromPtr(self.original_tokens.ptr)));
            const token_start = @divExact(start_offset, @sizeOf(Token));
            
            context_start = if (token_start >= 5) token_start - 5 else 0;
            context_end = if (token_start + tokens.len + 5 < self.original_tokens.len) 
                token_start + tokens.len + 5 else self.original_tokens.len;
        }
        
        return MacroCall{
            .name = try self.base_parser.allocator.dupe(u8, name),
            .arguments = arguments,
            .location = .{
                .file = self.base_parser.file_path,
                .line = tokens[0].line,
                .column = tokens[0].column,
                .byte_offset = 0,
            },
            .context_tokens = if (context_end > context_start) 
                try self.base_parser.allocator.dupe(Token, self.original_tokens[context_start..context_end]) 
                else &[_]Token{},
        };
    }
    
    /// Parse macro definition from token sequence
    fn parseMacroDefinitionFromTokens(self: *MacroAwareParser, tokens: []const Token) !MacroDefinition {
        if (tokens.len < 3) return error.InvalidMacroDefinition;
        
        // Expected format: #macro name(params...) { body... }
        // or: #define name value...
        
        const directive = tokens[1].lexeme;
        const name = tokens[2].lexeme;
        
        var definition = MacroDefinition.init(self.base_parser.allocator, name);
        definition.name = try self.base_parser.allocator.dupe(u8, name);
        
        if (std.mem.eql(u8, directive, "macro")) {
            // Function-like macro with parameters
            definition.is_function_like = true;
            
            // Parse parameters
            if (tokens.len > 3 and tokens[3].kind == .LeftParen) {
                const params = try self.parseParameterList(tokens[3..]);
                definition.parameters = params;
            }
            
            // Find body (everything in braces)
            var body_start: usize = 0;
            var body_end: usize = tokens.len;
            
            for (tokens, 0..) |token, i| {
                if (token.kind == .LeftBrace) {
                    body_start = i + 1;
                    break;
                }
            }
            
            for (tokens[body_start..], body_start..) |token, i| {
                if (token.kind == .RightBrace) {
                    body_end = i;
                    break;
                }
            }
            
            if (body_end > body_start) {
                definition.body = try self.base_parser.allocator.dupe(Token, tokens[body_start..body_end]);
            }
        } else if (std.mem.eql(u8, directive, "define")) {
            // Object-like macro
            definition.is_function_like = false;
            
            // Body is everything after the name
            if (tokens.len > 3) {
                definition.body = try self.base_parser.allocator.dupe(Token, tokens[3..]);
            }
        }
        
        return definition;
    }
    
    /// Parse parameter list from tokens
    fn parseParameterList(self: *MacroAwareParser, tokens: []const Token) ![][]const u8 {
        var params = std.ArrayList(u8){};
        defer params.deinit();
        
        if (tokens.len < 2 or tokens[0].kind != .LeftParen) {
            return &[_][]const u8{};
        }
        
        var i: usize = 1;
        while (i < tokens.len and tokens[i].kind != .RightParen) {
            if (tokens[i].kind == .Identifier) {
                try params.append(try self.base_parser.allocator.dupe(u8, tokens[i].value));
            }
            
            i += 1;
            
            // Skip comma
            if (i < tokens.len and tokens[i].kind == .Comma) {
                i += 1;
            }
        }
        
        return params.toOwnedSlice();
    }
    
    /// Build final token stream with macro expansions
    fn buildExpandedTokenStream(self: *MacroAwareParser, expansion_results: []const Token) ![]Token {
        var result = std.ArrayList(u8){};
        defer result.deinit();
        
        var original_index: usize = 0;
        var expansion_index: usize = 0;
        
        // Sort pending calls by start position
        std.sort.insertion(PendingMacroCall, self.pending_macro_calls.items, {}, compareCallsByPosition);
        
        for (self.pending_macro_calls.items) |pending| {
            // Add tokens before this macro call/definition
            while (original_index < pending.token_range.start) {
                try result.append(self.allocator, self.original_tokens[original_index]);
                original_index += 1;
            }
            
            // Add expansion result or skip definition
            if (pending.replacement_needed) {
                // Add expanded tokens
                const expansion_size = self.estimateExpansionSize(pending.call.name);
                if (expansion_index + expansion_size <= expansion_results.len) {
                    try result.appendSlice(expansion_results[expansion_index..expansion_index + expansion_size]);
                    expansion_index += expansion_size;
                }
            }
            // If not replacement_needed, we skip (for macro definitions)
            
            // Skip original macro call/definition tokens
            original_index = pending.token_range.end;
        }
        
        // Add remaining original tokens
        while (original_index < self.original_tokens.len) {
            try result.append(self.allocator, self.original_tokens[original_index]);
            original_index += 1;
        }
        
        return result.toOwnedSlice();
    }
    
    /// Estimate size of expansion for a macro (simple heuristic)
    fn estimateExpansionSize(self: *MacroAwareParser, macro_name: []const u8) usize {
        if (self.expansion_context.macro_definitions.get(macro_name)) |definition| {
            return definition.body.len;
        }
        return 1; // Default to 1 token
    }
    
    /// Compare function for sorting macro calls
    fn compareCallsByPosition(context: void, a: PendingMacroCall, b: PendingMacroCall) bool {
        _ = context;
        return a.token_range.start < b.token_range.start;
    }
    
    /// Register built-in macros
    pub fn registerBuiltinMacros(self: *MacroAwareParser) !void {
        // @line macro - expands to current line number
        var line_macro = MacroDefinition.init(self.base_parser.allocator, "line");
        line_macro.name = try self.base_parser.allocator.dupe(u8, "line");
        line_macro.is_function_like = false;
        line_macro.expansion_priority = .Immediate;
        line_macro.body = &[_]Token{
            Token{ .kind = .Number, .value = "0", .line = 1, .column = 1 },
        };
        try self.expansion_context.defineMacro(line_macro);
        
        // @file macro - expands to current file name
        var file_macro = MacroDefinition.init(self.base_parser.allocator, "file");
        file_macro.name = try self.base_parser.allocator.dupe(u8, "file");
        file_macro.is_function_like = false;
        file_macro.expansion_priority = .Immediate;
        file_macro.body = &[_]Token{
            Token{ .kind = .String, .value = self.base_parser.file_path, .line = 1, .column = 1 },
        };
        try self.expansion_context.defineMacro(file_macro);
        
        // @sizeof macro - expands to size of type
        var sizeof_macro = MacroDefinition.init(self.base_parser.allocator, "sizeof");
        sizeof_macro.name = try self.base_parser.dupe(u8, "sizeof");
        sizeof_macro.is_function_like = true;
        sizeof_macro.expansion_priority = .High;
        sizeof_macro.parameters = try self.base_parser.allocator.alloc([]const u8, 1);
        sizeof_macro.parameters[0] = try self.base_parser.allocator.dupe(u8, "type");
        try self.expansion_context.defineMacro(sizeof_macro);
    }
    
    /// Enable/disable hygiene checking
    pub fn setHygieneEnabled(self: *MacroAwareParser, enabled: bool) void {
        _ = self;
        _ = enabled;
        // Could modify expansion context to enable/disable hygiene
    }
    
    /// Get expansion statistics
    pub fn getExpansionStats(self: *MacroAwareParser) ExpansionStats {
        return ExpansionStats{
            .total_macros_defined = self.expansion_context.macro_definitions.count(),
            .total_expansions_performed = self.pending_macro_calls.items.len,
            .hygiene_violations_detected = self.hygiene_context.hygiene_violations.items.len,
        };
    }
    
    const ExpansionStats = struct {
        total_macros_defined: u32,
        total_expansions_performed: usize,
        hygiene_violations_detected: usize,
    };
};

// Test integration
test "macro aware parser basic functionality" {
    const test_tokens = [_]Token{
        Token{ .kind = .Hash, .lexeme = "#", .line = 1, .column = 1, .offset = 0 },
        Token{ .kind = .Identifier, .lexeme = "define", .line = 1, .column = 2, .offset = 1 },
        Token{ .kind = .Identifier, .lexeme = "MAX", .line = 1, .column = 3, .offset = 2 },
        Token{ .kind = .Number, .lexeme = "100", .line = 1, .column = 4, .offset = 3 },
        Token{ .kind = .Newline, .lexeme = "\n", .line = 1, .column = 5, .offset = 4 },
        Token{ .kind = .At, .lexeme = "@", .line = 2, .column = 1, .offset = 5 },
        Token{ .kind = .Identifier, .lexeme = "MAX", .line = 2, .column = 2, .offset = 6 },
    };
    
    var macro_parser = try MacroAwareParser.init(std.testing.allocator, &test_tokens);
    defer macro_parser.deinit();
    
    try macro_parser.registerBuiltinMacros();
    
    // Test scanning
    try macro_parser.scanForMacros();
    try std.testing.expect(macro_parser.pending_macro_calls.items.len > 0);
    
    // Test stats
    const stats = macro_parser.getExpansionStats();
    try std.testing.expect(stats.total_macros_defined > 0);
}
