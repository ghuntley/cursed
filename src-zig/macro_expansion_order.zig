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
            _ = allocator;
            return PendingExpansion{
                .macro_id = macro_id,
                .call_site = call_site,
                .priority = priority,
                .dependencies = .empty,
            };
        }
        
        pub fn deinit(self: *PendingExpansion) void {
            self.dependencies.deinit(self.allocator);
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
            _ = allocator;
            return ActiveExpansion{
                .macro_id = macro_id,
                .start_time = std.time.milliTimestamp(),
                .recursion_depth = 0,
                .parent_expansion = parent,
                .child_expansions = .empty,
                .expansion_state = .Queued,
            };
        }
        
        pub fn deinit(self: *ActiveExpansion) void {
            self.child_expansions.deinit(self.allocator);
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
    
    /// Context for nested macro expansion with hygiene tracking
    const NestedExpansionContext = struct {
        parent_expansion_id: u64,
        nested_hygiene_id: u32,
        nesting_depth: usize,
        symbol_capture_context: SymbolCaptureContext,
        
        fn deinit(self: *const NestedExpansionContext, allocator: Allocator) void {
            _ = allocator;
            self.symbol_capture_context.deinit(self.allocator);
        }
    };
    
    /// Symbol capture context for hygiene tracking
    const SymbolCaptureContext = struct {
        captured_symbols: ArrayList([]const u8),
        scope_boundaries: ArrayList(ScopeBoundary),
        
        const ScopeBoundary = struct {
            scope_id: u32,
            symbol_count: usize,
            is_macro_scope: bool,
        };
        
        fn init(allocator: Allocator) SymbolCaptureContext {
            _ = allocator;
            return SymbolCaptureContext{
                .captured_symbols = .empty,
                .scope_boundaries = .empty,
            };
        }
        
        fn deinit(self: *const SymbolCaptureContext, allocator: Allocator) void {
            for (self.captured_symbols.items) |symbol| {
                allocator.free(symbol);
            }
            self.captured_symbols.deinit(self.allocator);
            self.scope_boundaries.deinit(self.allocator);
        }
    };
    
    pub fn init(allocator: Allocator, hygiene_context: *MacroHygieneContext) !MacroExpansionContext {
        return MacroExpansionContext{
            .allocator = allocator,
            .expansion_queue = .empty,
            .current_expansions = HashMap(MacroId, ActiveExpansion, MacroIdContext, std.hash_map.default_max_load_percentage){},
            .expansion_counter = 0,
            .call_stack = .empty,
            .recursion_limit = 100,
            .dependency_graph = HashMap(MacroId, ArrayList(MacroId), MacroIdContext, std.hash_map.default_max_load_percentage).init(allocator),
            .hygiene_context = hygiene_context,
            .macro_definitions = HashMap([]const u8, MacroDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .expansion_cache = HashMap(ExpansionKey, []Token, ExpansionKeyContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *MacroExpansionContext) void {
        // Clean up pending expansions
        for (self.expansion_queue.items) |*expansion| {
            expansion.deinit();
        }
        self.expansion_queue.deinit(self.allocator);
        
        // Clean up active expansions
        var active_iterator = self.current_expansions.iterator();
        while (active_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.current_expansions.deinit(self.allocator);
        
        // Clean up dependency graph
        var dep_iterator = self.dependency_graph.iterator();
        while (dep_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.dependency_graph.deinit(self.allocator);
        
        self.call_stack.deinit(self.allocator);
        
        // Clean up cached expansions
        var cache_iterator = self.expansion_cache.iterator();
        while (cache_iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.*);
        }
        self.expansion_cache.deinit(self.allocator);
        
        self.macro_definitions.deinit(self.allocator);
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
        var result_tokens = std.ArrayList(u8){};
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
        var expanded_tokens = std.ArrayList(u8){};
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
        var substitutions = HashMap([]const u8, []Token, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){};
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
                    try result.append(self.allocator, token);
                }
            } else {
                try result.append(self.allocator, token);
            }
        }
    }
    
    /// Substitute object-like macro
    fn substituteObjectLikeMacro(self: *MacroExpansionContext, result: *ArrayList(Token), definition: *const MacroDefinition) !void {
        _ = self;
        try result.appendSlice(definition.body);
    }
    
    /// Process nested macro calls in expanded tokens with comprehensive hygiene
    fn processNestedMacros(self: *MacroExpansionContext, tokens: []Token) ![]Token {
        var result = std.ArrayList(u8){};
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
                    
                    // Critical P1 Fix: Enhanced hygiene for nested macro calls
                    const current_expansion_id = if (self.current_expansions.count() > 0) 
                        self.expansion_counter - 1 
                    else 
                        self.expansion_counter;
                    
                    // Begin nested expansion with hygiene tracking
                    const nested_hygiene_id = try self.hygiene_context.beginMacroExpansion(
                        macro_name, 
                        macro_call.location.file
                    );
                    
                    // Track nested expansion context for hygiene
                    const nested_expansion_context = NestedExpansionContext{
                        .parent_expansion_id = current_expansion_id,
                        .nested_hygiene_id = nested_hygiene_id,
                        .nesting_depth = self.call_stack.items.len,
                        .symbol_capture_context = try self.captureSymbolContext(tokens[0..i]),
                    };
                    
                    // Queue nested expansion with enhanced context
                    const nested_id = try self.queueNestedMacroExpansion(macro_call, nested_expansion_context);
                    
                    // Apply pre-expansion hygiene checks
                    try self.performPreExpansionHygieneChecks(nested_id, &nested_expansion_context);
                    
                    // Get expanded result with hygiene tracking
                    const nested_result = try self.expandSpecificMacroWithHygiene(nested_id);
                    
                    // Apply post-expansion hygiene fixes
                    const sanitized_result = try self.applyScopeRenaming(nested_result, nested_hygiene_id);
                    
                    try result.appendSlice(sanitized_result);
                    
                    // End nested expansion hygiene tracking
                    try self.hygiene_context.endMacroExpansion();
                    
                    // Clean up
                    self.allocator.free(sanitized_result);
                    nested_expansion_context.deinit();
                    
                    i = call_end;
                } else {
                    try result.append(self.allocator, tokens[i]);
                    i += 1;
                }
            } else {
                try result.append(self.allocator, tokens[i]);
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
        var visited = HashMap(MacroId, void, MacroIdContext, std.hash_map.default_max_load_percentage){};
        defer visited.deinit();
        
        var recursion_stack = HashMap(MacroId, void, MacroIdContext, std.hash_map.default_max_load_percentage){};
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
                    
                    try expansion.dependencies.append(self.allocator, dep_id);
                    
                    // Add to dependency graph
                    if (!self.dependency_graph.contains(expansion.macro_id)) {
                        try self.dependency_graph.put(expansion.macro_id, .empty);
                    }
                    
                    try self.dependency_graph.getPtr(expansion.macro_id).?.append(self.allocator, dep_id);
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
    
    // ============================================================================
    // Enhanced nested macro processing functions for P1 hygiene fix
    // ============================================================================
    
    /// Capture symbol context from preceding tokens
    fn captureSymbolContext(self: *MacroExpansionContext, preceding_tokens: []const Token) !SymbolCaptureContext {
        var context = SymbolCaptureContext.init(self.allocator);
        
        // Scan preceding tokens for symbol references
        for (preceding_tokens) |token| {
            if (token.kind == .Identifier) {
                // Check if this identifier is a symbol that might be captured
                if (self.hygiene_context.resolveSymbol(token.lexeme)) |resolved_name| {
                    if (resolved_name.ptr != token.lexeme.ptr) {
                        // Symbol has been renamed, indicating potential capture
                        try context.captured_symbols.append(self.allocator, try self.allocator.dupe(u8, token.lexeme));
                    }
                }
            }
        }
        
        // Track current scope boundaries
        for (self.hygiene_context.scope_stack.items) |*scope| {
            try context.scope_boundaries.append(self.allocator, SymbolCaptureContext.ScopeBoundary{
                .scope_id = scope.id,
                .symbol_count = scope.symbols.count(),
                .is_macro_scope = scope.macro_generated,
            });
        }
        
        return context;
    }
    
    /// Queue nested macro expansion with enhanced context
    fn queueNestedMacroExpansion(self: *MacroExpansionContext, macro_call: MacroCall, context: NestedExpansionContext) !MacroId {
        const macro_id = MacroId{
            .name = macro_call.name,
            .expansion_id = self.expansion_counter,
            .call_site_hash = self.hashCallSite(&macro_call),
        };
        
        // Create pending expansion with nested context
        var pending = PendingExpansion{
            .macro_id = macro_id,
            .call = macro_call,
            .dependencies = .empty,
            .priority = .Normal, // Could be adjusted based on nesting depth
            .estimated_complexity = @intCast(context.nesting_depth * 10), // Penalty for nesting
        };
        
        // Analyze dependencies considering nested context
        if (self.macro_definitions.getPtr(macro_call.name)) |definition| {
            try self.analyzeDependenciesWithContext(&pending, definition, &context);
        }
        
        try self.insertIntoQueue(pending);
        self.expansion_counter += 1;
        
        return macro_id;
    }
    
    /// Perform pre-expansion hygiene checks
    fn performPreExpansionHygieneChecks(self: *MacroExpansionContext, macro_id: MacroId, context: *const NestedExpansionContext) !void {
        // Check for potential variable capture issues before expansion
        for (context.symbol_capture_context.captured_symbols.items) |captured_symbol| {
            // Verify this capture is intentional
            const current_expansion = &self.hygiene_context.expansion_stack.items[self.hygiene_context.expansion_stack.items.len - 1];
            
            if (self.hygiene_context.isAccidentalCapture(captured_symbol, current_expansion)) {
                // Add a warning or error about potential hygiene violation
                try self.hygiene_context.hygiene_violations.append(macro_hygiene.MacroHygieneContext.HygieneViolation{
                    .kind = .VariableCapture,
                    .symbol_name = captured_symbol,
                    .macro_name = macro_id.name,
                    .location = "pre-expansion check",
                });
            }
        }
        
        // Check for scope boundary violations
        if (context.nesting_depth > 5) { // Arbitrary deep nesting threshold
            std.log.warn("Deep macro nesting detected (depth: {}), potential performance impact", .{context.nesting_depth});
        }
    }
    
    /// Expand specific macro with hygiene tracking
    fn expandSpecificMacroWithHygiene(self: *MacroExpansionContext, macro_id: MacroId) ![]Token {
        // Find the macro in pending expansions
        var expansion_index: ?usize = null;
        for (self.expansion_queue.items, 0..) |*expansion, i| {
            if (std.mem.eql(u8, expansion.macro_id.name, macro_id.name) and 
                expansion.macro_id.expansion_id == macro_id.expansion_id) {
                expansion_index = i;
                break;
            }
        }
        
        if (expansion_index == null) {
            return error.MacroNotFound;
        }
        
        const expansion = self.expansion_queue.swapRemove(expansion_index.?);
        defer expansion.deinit();
        
        // Get macro definition
        const definition = self.macro_definitions.get(expansion.macro_id.name) orelse return error.MacroNotDefined;
        
        // Perform expansion with hygiene tracking
        var result = std.ArrayList(u8){};
        defer result.deinit();
        
        if (definition.is_function_like) {
            try self.substituteFunctionLikeMacroWithHygiene(&result, &definition, expansion.call.arguments);
        } else {
            try self.substituteObjectLikeMacroWithHygiene(&result, &definition);
        }
        
        return result.toOwnedSlice(self.allocator);
    }
    
    /// Apply scope renaming for hygiene
    fn applyScopeRenaming(self: *MacroExpansionContext, tokens: []Token, hygiene_id: u32) ![]Token {
        _ = hygiene_id;
        var result = try self.allocator.alloc(Token, tokens.len);
        
        for (tokens, 0..) |token, i| {
            if (token.kind == .Identifier) {
                // Check if this identifier needs renaming for hygiene
                if (try self.hygiene_context.resolveSymbol(token.lexeme)) |renamed| {
                    // Create new token with renamed identifier
                    result[i] = Token{
                        .kind = token.kind,
                        .lexeme = renamed,
                        .line = token.line,
                        .column = token.column,
                        .offset = token.offset,
                    };
                } else {
                    result[i] = token;
                }
            } else {
                result[i] = token;
            }
        }
        
        return result;
    }
    
    /// Analyze dependencies with nested context
    fn analyzeDependenciesWithContext(self: *MacroExpansionContext, expansion: *PendingExpansion, definition: *const MacroDefinition, context: *const NestedExpansionContext) !void {
        // Standard dependency analysis
        try self.analyzeDependencies(expansion, definition);
        
        // Additional analysis for nested context
        if (context.nesting_depth > 0) {
            // Check for circular dependencies in nested context
            const parent_id = MacroId{
                .name = "parent", // Would be actual parent name
                .expansion_id = context.parent_expansion_id,
                .call_site_hash = 0,
            };
            
            // Ensure we don't create circular dependency with parent
            for (expansion.dependencies.items) |dep| {
                if (dep.expansion_id == parent_id.expansion_id) {
                    // Potential circular dependency detected
                    expansion.priority = .Low; // Reduce priority to break cycles
                }
            }
        }
    }
    
    /// Substitute function-like macro with hygiene
    fn substituteFunctionLikeMacroWithHygiene(self: *MacroExpansionContext, result: *ArrayList(Token), definition: *const MacroDefinition, arguments: []Token) !void {
        // Enhanced substitution with hygiene considerations
        const parsed_args = try self.parseArguments(arguments);
        defer {
            for (parsed_args) |arg| {
                self.allocator.free(arg);
            }
            self.allocator.free(parsed_args);
        }
        
        for (definition.body) |token| {
            if (token.kind == .Identifier) {
                // Check if this is a parameter
                var is_parameter = false;
                for (definition.parameters, 0..) |param, param_idx| {
                    if (std.mem.eql(u8, token.lexeme, param)) {
                        // Substitute with argument
                        if (param_idx < parsed_args.len) {
                            // Apply hygiene renaming to argument tokens
                            for (parsed_args[param_idx]) |arg_token| {
                                const renamed_token = try self.applyHygieneToToken(arg_token);
                                try result.append(self.allocator, renamed_token);
                            }
                        }
                        is_parameter = true;
                        break;
                    }
                }
                
                if (!is_parameter) {
                    // Not a parameter, apply hygiene renaming
                    const renamed_token = try self.applyHygieneToToken(token);
                    try result.append(self.allocator, renamed_token);
                }
            } else {
                try result.append(self.allocator, token);
            }
        }
    }
    
    /// Substitute object-like macro with hygiene
    fn substituteObjectLikeMacroWithHygiene(self: *MacroExpansionContext, result: *ArrayList(Token), definition: *const MacroDefinition) !void {
        for (definition.body) |token| {
            const renamed_token = try self.applyHygieneToToken(token);
            try result.append(self.allocator, renamed_token);
        }
    }
    
    /// Apply hygiene to individual token
    fn applyHygieneToToken(self: *MacroExpansionContext, token: Token) !Token {
        if (token.kind == .Identifier) {
            if (try self.hygiene_context.resolveSymbol(token.lexeme)) |renamed| {
                return Token{
                    .kind = token.kind,
                    .lexeme = renamed,
                    .line = token.line,
                    .column = token.column,
                    .offset = token.offset,
                };
            }
        }
        return token;
    }
    
    fn parseArguments(self: *MacroExpansionContext, tokens: []const Token) ![][]Token {
        // Simple argument parsing - split on commas at top level
        var args = std.ArrayList(u8){};
        defer args.deinit();
        
        var current_arg = std.ArrayList(u8){};
        defer current_arg.deinit();
        
        var paren_depth: i32 = 0;
        
        for (tokens) |token| {
            switch (token.kind) {
                .LeftParen => {
                    paren_depth += 1;
                    try current_arg.append(self.allocator, token);
                },
                .RightParen => {
                    paren_depth -= 1;
                    try current_arg.append(self.allocator, token);
                },
                .Comma => {
                    if (paren_depth == 0) {
                        // End of current argument
                        try args.append(self.allocator, try self.allocator.dupe(Token, current_arg.items));
                        current_arg.clearRetainingCapacity();
                    } else {
                        try current_arg.append(self.allocator, token);
                    }
                },
                else => {
                    try current_arg.append(self.allocator, token);
                },
            }
        }
        
        // Add final argument
        if (current_arg.items.len > 0) {
            try args.append(self.allocator, try self.allocator.dupe(Token, current_arg.items));
        }
        
        return args.toOwnedSlice(self.allocator);
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
