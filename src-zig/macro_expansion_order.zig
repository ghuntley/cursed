//! Macro Expansion Order System
//! Ensures deterministic macro expansion with proper ordering, recursion detection, and nested expansion handling

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const macro_hygiene = @import("macro_hygiene.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const MacroHygieneContext = macro_hygiene.MacroHygieneContext;

/// Macro expansion context with ordering guarantees
pub const MacroExpansionContext = struct {
    allocator: Allocator,
    
    // Expansion order tracking
    expansion_queue: ArrayList(PendingExpansion),
    current_expansions: HashMap(MacroId, ActiveExpansion, MacroIdContext, std.hash_map.default_max_load_percentage),
    expansion_counter: u64,
    
    // Recursion detection
    call_stack: ArrayList(MacroCall),
    recursion_limit: usize,
    dependency_graph: HashMap(MacroId, ArrayList(MacroId), MacroIdContext, std.hash_map.default_max_load_percentage),
    
    // Integration with hygiene system
    hygiene_context: *MacroHygieneContext,
    
    // Macro definitions
    macro_definitions: HashMap([]const u8, MacroDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Expansion results cache
    expansion_cache: HashMap(ExpansionKey, []Token, ExpansionKeyContext, std.hash_map.default_max_load_percentage),
    
    const MacroId = struct {
        name: []const u8,
        expansion_id: u64,
        call_site_hash: u64,
    };
    
    const MacroIdContext = struct {
        pub fn hash(self: @This(), key: MacroId) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(key.name);
            hasher.update(std.mem.asBytes(&key.expansion_id));
            hasher.update(std.mem.asBytes(&key.call_site_hash));
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: MacroId, b: MacroId) bool {
            _ = self;
            return std.mem.eql(u8, a.name, b.name) and 
                   a.expansion_id == b.expansion_id and 
                   a.call_site_hash == b.call_site_hash;
        }
    };
    
    const PendingExpansion = struct {
        macro_id: MacroId,
        call_site: MacroCall,
        priority: ExpansionPriority,
        dependencies: ArrayList(MacroId),
        
        const ExpansionPriority = enum(u8) {
            Immediate = 0,    // Must expand before any other processing
            High = 1,         // Type-level macros, compile-time constants
            Normal = 2,       // Regular function-like macros
            Low = 3,          // Code generation macros
            Deferred = 4,     // Cleanup and finalization macros
        };
        
        pub fn init(allocator: Allocator, macro_id: MacroId, call_site: MacroCall, priority: ExpansionPriority) PendingExpansion {
            return PendingExpansion{
                .macro_id = macro_id,
                .call_site = call_site,
                .priority = priority,
                .dependencies = ArrayList(MacroId).init(allocator),
            };
        }
        
        pub fn deinit(self: *PendingExpansion) void {
            self.dependencies.deinit();
        }
    };
    
    const ActiveExpansion = struct {
        macro_id: MacroId,
        start_time: i64,
        recursion_depth: usize,
        parent_expansion: ?MacroId,
        child_expansions: ArrayList(MacroId),
        expansion_state: ExpansionState,
        
        const ExpansionState = enum {
            Queued,
            Analyzing,
            Expanding,
            HygieneCheck,
            Complete,
            Failed,
        };
        
        pub fn init(allocator: Allocator, macro_id: MacroId, parent: ?MacroId) ActiveExpansion {
            return ActiveExpansion{
                .macro_id = macro_id,
                .start_time = std.time.milliTimestamp(),
                .recursion_depth = 0,
                .parent_expansion = parent,
                .child_expansions = ArrayList(MacroId).init(allocator),
                .expansion_state = .Queued,
            };
        }
        
        pub fn deinit(self: *ActiveExpansion) void {
            self.child_expansions.deinit();
        }
    };
    
    pub const MacroCall = struct {
        name: []const u8,
        arguments: []Token,
        location: SourceLocation,
        context_tokens: []Token,  // Surrounding tokens for context
        
        const SourceLocation = struct {
            file: []const u8,
            line: u32,
            column: u32,
            byte_offset: u32,
        };
    };
    
    pub const MacroDefinition = struct {
        name: []const u8,
        parameters: [][]const u8,
        body: []Token,
        expansion_priority: PendingExpansion.ExpansionPriority,
        is_function_like: bool,
        requires_hygiene: bool,
        max_recursion: usize,
        
        pub fn init(allocator: Allocator, name: []const u8) MacroDefinition {
            _ = allocator;
            return MacroDefinition{
                .name = name,
                .parameters = &[_][]const u8{},
                .body = &[_]Token{},
                .expansion_priority = .Normal,
                .is_function_like = false,
                .requires_hygiene = true,
                .max_recursion = 50,
            };
        }
    };
    
    const ExpansionKey = struct {
        macro_name: []const u8,
        arguments_hash: u64,
        context_hash: u64,
    };
    
    const ExpansionKeyContext = struct {
        pub fn hash(self: @This(), key: ExpansionKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(key.macro_name);
            hasher.update(std.mem.asBytes(&key.arguments_hash));
            hasher.update(std.mem.asBytes(&key.context_hash));
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: ExpansionKey, b: ExpansionKey) bool {
            _ = self;
            return std.mem.eql(u8, a.macro_name, b.macro_name) and 
                   a.arguments_hash == b.arguments_hash and 
                   a.context_hash == b.context_hash;
        }
    };
    
    pub fn init(allocator: Allocator, hygiene_context: *MacroHygieneContext) !MacroExpansionContext {
        return MacroExpansionContext{
            .allocator = allocator,
            .expansion_queue = ArrayList(PendingExpansion).init(allocator),
            .current_expansions = HashMap(MacroId, ActiveExpansion, MacroIdContext, std.hash_map.default_max_load_percentage).init(allocator),
            .expansion_counter = 0,
            .call_stack = ArrayList(MacroCall).init(allocator),
            .recursion_limit = 100,
            .dependency_graph = HashMap(MacroId, ArrayList(MacroId), MacroIdContext, std.hash_map.default_max_load_percentage).init(allocator),
            .hygiene_context = hygiene_context,
            .macro_definitions = HashMap([]const u8, MacroDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .expansion_cache = HashMap(ExpansionKey, []Token, ExpansionKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *MacroExpansionContext) void {
        // Clean up pending expansions
        for (self.expansion_queue.items) |*expansion| {
            expansion.deinit();
        }
        self.expansion_queue.deinit();
        
        // Clean up active expansions
        var active_iterator = self.current_expansions.iterator();
        while (active_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.current_expansions.deinit();
        
        // Clean up dependency graph
        var dep_iterator = self.dependency_graph.iterator();
        while (dep_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.dependency_graph.deinit();
        
        self.call_stack.deinit();
        
        // Clean up cached expansions
        var cache_iterator = self.expansion_cache.iterator();
        while (cache_iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.*);
        }
        self.expansion_cache.deinit();
        
        self.macro_definitions.deinit();
    }
    
    /// Register a macro definition
    pub fn defineMacro(self: *MacroExpansionContext, definition: MacroDefinition) !void {
        try self.macro_definitions.put(try self.allocator.dupe(u8, definition.name), definition);
    }
    
    /// Queue a macro for expansion with proper ordering
    pub fn queueMacroExpansion(self: *MacroExpansionContext, call: MacroCall) !MacroId {
        // Check if macro is defined
        const definition = self.macro_definitions.get(call.name) orelse {
            return error.UndefinedMacro;
        };
        
        // Generate unique macro ID
        const call_site_hash = self.hashCallSite(&call);
        const macro_id = MacroId{
            .name = call.name,
            .expansion_id = self.expansion_counter,
            .call_site_hash = call_site_hash,
        };
        self.expansion_counter += 1;
        
        // Check for cached expansion
        const cache_key = ExpansionKey{
            .macro_name = call.name,
            .arguments_hash = self.hashTokens(call.arguments),
            .context_hash = self.hashTokens(call.context_tokens),
        };
        
        if (self.expansion_cache.contains(cache_key)) {
            // Return cached expansion immediately
            return macro_id;
        }
        
        // Create pending expansion
        var pending = PendingExpansion.init(self.allocator, macro_id, call, definition.expansion_priority);
        
        // Analyze dependencies
        try self.analyzeDependencies(&pending, &definition);
        
        // Check for recursion
        if (try self.detectRecursion(macro_id)) {
            return error.MacroRecursion;
        }
        
        // Insert into queue with proper ordering
        try self.insertIntoQueue(pending);
        
        return macro_id;
    }
    
    /// Process all queued macro expansions in correct order
    pub fn processExpansions(self: *MacroExpansionContext) ![]Token {
        var result_tokens = ArrayList(Token).init(self.allocator);
        defer result_tokens.deinit();
        
        // Process expansions by priority and dependency order
        while (self.expansion_queue.items.len > 0) {
            // Find next expansion to process
            const next_index = try self.findNextExpansion();
            if (next_index == null) {
                // Circular dependency detected
                return error.CircularDependency;
            }
            
            var expansion = self.expansion_queue.swapRemove(next_index.?);
            defer expansion.deinit();
            
            // Create active expansion
            var active = ActiveExpansion.init(self.allocator, expansion.macro_id, self.getCurrentParentExpansion());
            defer active.deinit();
            
            try self.current_expansions.put(expansion.macro_id, active);
            
            // Process the expansion
            const expanded_tokens = try self.expandMacro(&expansion);
            try result_tokens.appendSlice(expanded_tokens);
            
            // Remove from active expansions
            _ = self.current_expansions.remove(expansion.macro_id);
        }
        
        return result_tokens.toOwnedSlice();
    }
    
    /// Expand a single macro with hygiene checking
    fn expandMacro(self: *MacroExpansionContext, expansion: *PendingExpansion) ![]Token {
        const definition = self.macro_definitions.get(expansion.macro_id.name).?;
        
        // Update expansion state
        if (self.current_expansions.getPtr(expansion.macro_id)) |active| {
            active.expansion_state = .Expanding;
        }
        
        // Begin hygiene context if required
        const hygiene_id = if (definition.requires_hygiene) 
            try self.hygiene_context.beginMacroExpansion(definition.name, expansion.call_site.location.file)
        else 
            null;
        
        defer if (hygiene_id != null) {
            self.hygiene_context.endMacroExpansion() catch {};
        };
        
        // Perform macro expansion
        var expanded_tokens = ArrayList(Token).init(self.allocator);
        defer expanded_tokens.deinit();
        
        // Substitute parameters
        if (definition.is_function_like) {
            try self.substituteFunctionLikeMacro(&expanded_tokens, &definition, &expansion.call_site);
        } else {
            try self.substituteObjectLikeMacro(&expanded_tokens, &definition);
        }
        
        // Process nested macro calls in expansion
        const final_tokens = try self.processNestedMacros(expanded_tokens.items);
        
        // Cache the expansion
        const cache_key = ExpansionKey{
            .macro_name = expansion.macro_id.name,
            .arguments_hash = self.hashTokens(expansion.call_site.arguments),
            .context_hash = self.hashTokens(expansion.call_site.context_tokens),
        };
        try self.expansion_cache.put(cache_key, try self.allocator.dupe(Token, final_tokens));
        
        return final_tokens;
    }
    
    /// Substitute function-like macro parameters
    fn substituteFunctionLikeMacro(self: *MacroExpansionContext, result: *ArrayList(Token), definition: *const MacroDefinition, call: *const MacroCall) !void {
        // Create parameter substitution map
        var substitutions = HashMap([]const u8, []Token, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer {
            var iterator = substitutions.iterator();
            while (iterator.next()) |entry| {
                self.allocator.free(entry.value_ptr.*);
            }
            substitutions.deinit();
        }
        
        // Parse arguments and create substitutions
        const arg_lists = try self.parseArguments(call.arguments);
        defer {
            for (arg_lists) |arg_list| {
                self.allocator.free(arg_list);
            }
            self.allocator.free(arg_lists);
        }
        
        if (arg_lists.len != definition.parameters.len) {
            return error.MacroArgumentMismatch;
        }
        
        for (definition.parameters, arg_lists) |param, arg_tokens| {
            try substitutions.put(param, try self.allocator.dupe(Token, arg_tokens));
        }
        
        // Substitute tokens in macro body
        for (definition.body) |token| {
            if (token.kind == .Identifier) {
                if (substitutions.get(token.lexeme)) |replacement| {
                    try result.appendSlice(replacement);
                } else {
                    try result.append(token);
                }
            } else {
                try result.append(token);
            }
        }
    }
    
    /// Substitute object-like macro
    fn substituteObjectLikeMacro(self: *MacroExpansionContext, result: *ArrayList(Token), definition: *const MacroDefinition) !void {
        _ = self;
        try result.appendSlice(definition.body);
    }
    
    /// Process nested macro calls in expanded tokens
    fn processNestedMacros(self: *MacroExpansionContext, tokens: []Token) ![]Token {
        var result = ArrayList(Token).init(self.allocator);
        defer result.deinit();
        
        var i: usize = 0;
        while (i < tokens.len) {
            // Look for macro calls (@ followed by identifier)
            if (i + 1 < tokens.len and 
                tokens[i].kind == .At and 
                tokens[i + 1].kind == .Identifier) {
                
                const macro_name = tokens[i + 1].lexeme;
                
                // Check if this is a defined macro
                if (self.macro_definitions.contains(macro_name)) {
                    // Extract macro call
                    const call_end = try self.findMacroCallEnd(tokens, i);
                    const call_tokens = tokens[i..call_end];
                    
                    // Parse macro call
                    const macro_call = try self.parseMacroCall(call_tokens);
                    
                    // Queue nested expansion
                    const nested_id = try self.queueMacroExpansion(macro_call);
                    
                    // Get expanded result (recursive processing)
                    const nested_result = try self.expandSpecificMacro(nested_id);
                    try result.appendSlice(nested_result);
                    
                    i = call_end;
                } else {
                    try result.append(tokens[i]);
                    i += 1;
                }
            } else {
                try result.append(tokens[i]);
                i += 1;
            }
        }
        
        return result.toOwnedSlice();
    }
    
    /// Detect recursion in macro expansion
    fn detectRecursion(self: *MacroExpansionContext, macro_id: MacroId) !bool {
        // Check call stack depth
        if (self.call_stack.items.len >= self.recursion_limit) {
            return true;
        }
        
        // Check for direct recursion (same macro in call stack)
        for (self.call_stack.items) |call| {
            if (std.mem.eql(u8, call.name, macro_id.name)) {
                return true;
            }
        }
        
        // Check for circular dependencies
        return self.hasCircularDependency(macro_id);
    }
    
    /// Check for circular dependencies in dependency graph
    fn hasCircularDependency(self: *MacroExpansionContext, start_macro: MacroId) bool {
        var visited = HashMap(MacroId, void, MacroIdContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer visited.deinit();
        
        var recursion_stack = HashMap(MacroId, void, MacroIdContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer recursion_stack.deinit();
        
        return self.dfsHasCycle(start_macro, &visited, &recursion_stack);
    }
    
    /// Depth-first search for cycles in dependency graph
    fn dfsHasCycle(self: *MacroExpansionContext, macro_id: MacroId, visited: *HashMap(MacroId, void, MacroIdContext, std.hash_map.default_max_load_percentage), recursion_stack: *HashMap(MacroId, void, MacroIdContext, std.hash_map.default_max_load_percentage)) bool {
        try visited.put(macro_id, {});
        try recursion_stack.put(macro_id, {});
        
        if (self.dependency_graph.get(macro_id)) |dependencies| {
            for (dependencies.items) |dep_id| {
                if (!visited.contains(dep_id)) {
                    if (self.dfsHasCycle(dep_id, visited, recursion_stack)) {
                        return true;
                    }
                } else if (recursion_stack.contains(dep_id)) {
                    return true;
                }
            }
        }
        
        _ = recursion_stack.remove(macro_id);
        return false;
    }
    
    /// Find next expansion to process based on dependencies and priority
    fn findNextExpansion(self: *MacroExpansionContext) !?usize {
        for (self.expansion_queue.items, 0..) |*expansion, i| {
            // Check if all dependencies are satisfied
            var all_satisfied = true;
            for (expansion.dependencies.items) |dep_id| {
                if (self.current_expansions.contains(dep_id)) {
                    // Dependency is still being processed
                    all_satisfied = false;
                    break;
                }
            }
            
            if (all_satisfied) {
                return i;
            }
        }
        
        // No expansion can be processed - might be circular dependency
        return null;
    }
    
    /// Analyze dependencies for a macro expansion
    fn analyzeDependencies(self: *MacroExpansionContext, expansion: *PendingExpansion, definition: *const MacroDefinition) !void {
        // Scan macro body for nested macro calls
        var i: usize = 0;
        while (i + 1 < definition.body.len) {
            if (definition.body[i].kind == .At and definition.body[i + 1].kind == .Identifier) {
                const nested_macro = definition.body[i + 1].value;
                
                // Check if this is a defined macro
                if (self.macro_definitions.contains(nested_macro)) {
                    // Create dependency
                    const dep_id = MacroId{
                        .name = nested_macro,
                        .expansion_id = self.expansion_counter,
                        .call_site_hash = 0, // Will be set when actually queued
                    };
                    
                    try expansion.dependencies.append(dep_id);
                    
                    // Add to dependency graph
                    if (!self.dependency_graph.contains(expansion.macro_id)) {
                        try self.dependency_graph.put(expansion.macro_id, ArrayList(MacroId).init(self.allocator));
                    }
                    
                    try self.dependency_graph.getPtr(expansion.macro_id).?.append(dep_id);
                }
            }
            i += 1;
        }
    }
    
    /// Insert expansion into queue maintaining priority order
    fn insertIntoQueue(self: *MacroExpansionContext, expansion: PendingExpansion) !void {
        // Find insertion point based on priority
        var insert_index: usize = 0;
        for (self.expansion_queue.items, 0..) |*queued, i| {
            if (@intFromEnum(expansion.priority) < @intFromEnum(queued.priority)) {
                insert_index = i;
                break;
            }
            insert_index = i + 1;
        }
        
        try self.expansion_queue.insert(insert_index, expansion);
    }
    
    /// Helper functions
    fn hashCallSite(self: *MacroExpansionContext, call: *const MacroCall) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(call.name);
        hasher.update(std.mem.asBytes(&call.location.line));
        hasher.update(std.mem.asBytes(&call.location.column));
        hasher.update(call.location.file);
        return hasher.final();
    }
    
    fn hashTokens(self: *MacroExpansionContext, tokens: []const Token) u64 {
        _ = self;
        var hasher = std.hash.Wyhash.init(0);
        for (tokens) |token| {
            hasher.update(std.mem.asBytes(&token.kind));
            hasher.update(token.lexeme);
        }
        return hasher.final();
    }
    
    fn parseArguments(self: *MacroExpansionContext, tokens: []const Token) ![][]Token {
        // Simple argument parsing - split on commas at top level
        var args = ArrayList([]Token).init(self.allocator);
        defer args.deinit();
        
        var current_arg = ArrayList(Token).init(self.allocator);
        defer current_arg.deinit();
        
        var paren_depth: i32 = 0;
        
        for (tokens) |token| {
            switch (token.kind) {
                .LeftParen => {
                    paren_depth += 1;
                    try current_arg.append(token);
                },
                .RightParen => {
                    paren_depth -= 1;
                    try current_arg.append(token);
                },
                .Comma => {
                    if (paren_depth == 0) {
                        // End of current argument
                        try args.append(try self.allocator.dupe(Token, current_arg.items));
                        current_arg.clearRetainingCapacity();
                    } else {
                        try current_arg.append(token);
                    }
                },
                else => {
                    try current_arg.append(token);
                },
            }
        }
        
        // Add final argument
        if (current_arg.items.len > 0) {
            try args.append(try self.allocator.dupe(Token, current_arg.items));
        }
        
        return args.toOwnedSlice();
    }
    
    fn findMacroCallEnd(self: *MacroExpansionContext, tokens: []const Token, start: usize) !usize {
        _ = self;
        var i = start + 2; // Skip @ and identifier
        
        // Look for opening parenthesis for function-like macro
        if (i < tokens.len and tokens[i].kind == .LeftParen) {
            var paren_count: i32 = 1;
            i += 1;
            
            while (i < tokens.len and paren_count > 0) {
                switch (tokens[i].kind) {
                    .LeftParen => paren_count += 1,
                    .RightParen => paren_count -= 1,
                    else => {},
                }
                i += 1;
            }
        }
        
        return i;
    }
    
    fn parseMacroCall(self: *MacroExpansionContext, tokens: []const Token) !MacroCall {
        if (tokens.len < 2) return error.InvalidMacroCall;
        
        const name = tokens[1].lexeme;
        var arguments: []Token = &[_]Token{};
        
        if (tokens.len > 3 and tokens[2].kind == .LeftParen) {
            arguments = tokens[3..tokens.len-1]; // Exclude parens
        }
        
        return MacroCall{
            .name = try self.allocator.dupe(u8, name),
            .arguments = try self.allocator.dupe(Token, arguments),
            .location = .{
                .file = "current",
                .line = tokens[0].line,
                .column = tokens[0].column,
                .byte_offset = 0,
            },
            .context_tokens = &[_]Token{},
        };
    }
    
    fn expandSpecificMacro(self: *MacroExpansionContext, macro_id: MacroId) ![]Token {
        // Find expansion in cache
        for (self.expansion_queue.items) |*expansion| {
            if (std.meta.eql(expansion.macro_id, macro_id)) {
                return self.expandMacro(expansion);
            }
        }
        
        return error.MacroNotFound;
    }
    
    fn getCurrentParentExpansion(self: *MacroExpansionContext) ?MacroId {
        if (self.current_expansions.count() == 0) return null;
        
        var iterator = self.current_expansions.iterator();
        if (iterator.next()) |entry| {
            return entry.key_ptr.*;
        }
        
        return null;
    }
};

// Test cases for macro expansion order
test "basic macro expansion order" {
    var hygiene_ctx = try MacroHygieneContext.init(std.testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(std.testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define a simple macro
    var definition = MacroExpansionContext.MacroDefinition.init(std.testing.allocator, "TEST_MACRO");
    var body = [_]Token{
        Token{ .kind = .Identifier, .lexeme = "result", .line = 1, .column = 1, .offset = 0 },
    };
    definition.body = body[0..];
    try expansion_ctx.defineMacro(definition);
    
    // Queue macro expansion
    const call = MacroExpansionContext.MacroCall{
        .name = "TEST_MACRO",
        .arguments = &[_]Token{},
        .location = .{
            .file = "test.csd",
            .line = 1,
            .column = 1,
            .byte_offset = 0,
        },
        .context_tokens = &[_]Token{},
    };
    
    _ = try expansion_ctx.queueMacroExpansion(call);
    
    // Process expansions
    const result = try expansion_ctx.processExpansions();
    defer std.testing.allocator.free(result);
    
    try std.testing.expect(result.len > 0);
}

test "recursion detection" {
    var hygiene_ctx = try MacroHygieneContext.init(std.testing.allocator);
    defer hygiene_ctx.deinit();
    
    var expansion_ctx = try MacroExpansionContext.init(std.testing.allocator, &hygiene_ctx);
    defer expansion_ctx.deinit();
    
    // Define recursive macro
    var definition = MacroExpansionContext.MacroDefinition.init(std.testing.allocator, "RECURSIVE");
    var body = [_]Token{
        Token{ .kind = .At, .lexeme = "@", .line = 1, .column = 1, .offset = 0 },
        Token{ .kind = .Identifier, .lexeme = "RECURSIVE", .line = 1, .column = 2, .offset = 1 },
    };
    definition.body = body[0..];
    try expansion_ctx.defineMacro(definition);
    
    // Try to expand recursive macro
    const call = MacroExpansionContext.MacroCall{
        .name = "RECURSIVE",
        .arguments = &[_]Token{},
        .location = .{
            .file = "test.csd",
            .line = 1,
            .column = 1,
            .byte_offset = 0,
        },
        .context_tokens = &[_]Token{},
    };
    
    // Should detect recursion
    const result = expansion_ctx.queueMacroExpansion(call);
    try std.testing.expectError(error.MacroRecursion, result);
}
