//! P31: Macro Hygiene System
//! Prevents variable capture and ensures lexical scoping in macros

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");

/// Macro hygiene context for variable scope tracking
pub const MacroHygieneContext = struct {
    allocator: Allocator,
    
    // Scope management
    scope_stack: ArrayList(Scope),
    global_scope: *Scope,
    current_scope_id: u32,
    
    // Symbol renaming
    renamed_symbols: HashMap(SymbolKey, []const u8, SymbolKeyContext, std.hash_map.default_max_load_percentage),
    rename_counter: u32,
    
    // Macro expansion tracking
    expansion_stack: ArrayList(MacroExpansion),
    hygiene_violations: ArrayList(HygieneViolation),
    
    const Scope = struct {
        id: u32,
        parent: ?*Scope,
        symbols: HashMap([]const u8, SymbolInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        macro_generated: bool,
        expansion_id: ?u32,
        
        const SymbolInfo = struct {
            name: []const u8,
            symbol_type: SymbolType,
            original_name: []const u8,
            is_macro_generated: bool,
            definition_scope: u32,
            
            const SymbolType = enum {
                Variable,
                Function,
                Type,
                Constant,
            };
        };
        
        pub fn init(allocator: Allocator, id: u32, parent: ?*Scope, macro_generated: bool) Scope {
            return Scope{
                .id = id,
                .parent = parent,
                .symbols = HashMap([]const u8, SymbolInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .macro_generated = macro_generated,
                .expansion_id = null,
            };
        }
        
        pub fn deinit(self: *Scope) void {
            self.symbols.deinit();
        }
    };
    
    const SymbolKey = struct {
        scope_id: u32,
        symbol_name: []const u8,
    };
    
    const SymbolKeyContext = struct {
        pub fn hash(self: @This(), key: SymbolKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(std.mem.asBytes(&key.scope_id));
            hasher.update(key.symbol_name);
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: SymbolKey, b: SymbolKey) bool {
            _ = self;
            return a.scope_id == b.scope_id and std.mem.eql(u8, a.symbol_name, b.symbol_name);
        }
    };
    
    const MacroExpansion = struct {
        id: u32,
        macro_name: []const u8,
        expansion_location: []const u8,
        parent_expansion: ?u32,
        scope_before: u32,
        symbols_introduced: ArrayList([]const u8),
        
        pub fn init(allocator: Allocator, id: u32, macro_name: []const u8, location: []const u8) MacroExpansion {
            return MacroExpansion{
                .id = id,
                .macro_name = macro_name,
                .expansion_location = location,
                .parent_expansion = null,
                .scope_before = 0,
                .symbols_introduced = ArrayList([]const u8).init(allocator),
            };
        }
        
        pub fn deinit(self: *MacroExpansion) void {
            self.symbols_introduced.deinit();
        }
    };
    
    const HygieneViolation = struct {
        kind: ViolationKind,
        symbol_name: []const u8,
        macro_name: []const u8,
        location: []const u8,
        
        const ViolationKind = enum {
            VariableCapture,     // Macro accidentally captures outer variable
            SymbolShadowing,     // Macro introduces symbol that shadows outer scope
            UnintendedBinding,   // Macro binds to unintended symbol
            ScopeEscape,         // Macro-generated symbol escapes intended scope
        };
    };
    
    pub fn init(allocator: Allocator) !MacroHygieneContext {
        var context = MacroHygieneContext{
            .allocator = allocator,
            .scope_stack = ArrayList(Scope).init(allocator),
            .global_scope = undefined,
            .current_scope_id = 0,
            .renamed_symbols = HashMap(SymbolKey, []const u8, SymbolKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .rename_counter = 0,
            .expansion_stack = ArrayList(MacroExpansion).init(allocator),
            .hygiene_violations = ArrayList(HygieneViolation).init(allocator),
        };
        
        // Create global scope
        const global_scope = Scope.init(allocator, 0, null, false);
        try context.scope_stack.append(global_scope);
        context.global_scope = &context.scope_stack.items[0];
        
        return context;
    }
    
    pub fn deinit(self: *MacroHygieneContext) void {
        for (self.scope_stack.items) |*scope| {
            scope.deinit();
        }
        self.scope_stack.deinit();
        
        var renamed_iterator = self.renamed_symbols.iterator();
        while (renamed_iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.*);
        }
        self.renamed_symbols.deinit();
        
        for (self.expansion_stack.items) |*expansion| {
            expansion.deinit();
        }
        self.expansion_stack.deinit();
        
        self.hygiene_violations.deinit();
    }
    
    /// Begin macro expansion with hygiene tracking
    pub fn beginMacroExpansion(self: *MacroHygieneContext, macro_name: []const u8, location: []const u8) !u32 {
        const expansion_id = self.expansion_stack.items.len;
        var expansion = MacroExpansion.init(self.allocator, @intCast(expansion_id), macro_name, location);
        expansion.scope_before = self.current_scope_id;
        
        if (self.expansion_stack.items.len > 0) {
            expansion.parent_expansion = @intCast(self.expansion_stack.items.len - 1);
        }
        
        try self.expansion_stack.append(expansion);
        
        // Create new scope for macro expansion
        try self.pushScope(true);
        
        return @intCast(expansion_id);
    }
    
    /// End macro expansion and check for hygiene violations
    pub fn endMacroExpansion(self: *MacroHygieneContext) !void {
        if (self.expansion_stack.items.len == 0) {
            return error.NoActiveExpansion;
        }
        
        const expansion = self.expansion_stack.pop();
        defer expansion.deinit();
        
        // Check for hygiene violations in this expansion
        try self.checkHygieneViolations(expansion);
        
        // Pop the macro's scope
        self.popScope();
    }
    
    /// Push new scope
    fn pushScope(self: *MacroHygieneContext, macro_generated: bool) !void {
        self.current_scope_id += 1;
        const parent = if (self.scope_stack.items.len > 0) &self.scope_stack.items[self.scope_stack.items.len - 1] else null;
        var new_scope = Scope.init(self.allocator, self.current_scope_id, parent, macro_generated);
        
        if (self.expansion_stack.items.len > 0) {
            new_scope.expansion_id = @intCast(self.expansion_stack.items.len - 1);
        }
        
        try self.scope_stack.append(new_scope);
    }
    
    /// Pop current scope
    fn popScope(self: *MacroHygieneContext) void {
        if (self.scope_stack.items.len > 1) { // Don't pop global scope
            var scope = self.scope_stack.pop();
            scope.deinit();
        }
    }
    
    /// Declare symbol with hygiene checking
    pub fn declareSymbol(self: *MacroHygieneContext, name: []const u8, symbol_type: Scope.SymbolInfo.SymbolType) ![]const u8 {
        const current_scope = &self.scope_stack.items[self.scope_stack.items.len - 1];
        const is_macro_context = self.expansion_stack.items.len > 0;
        
        var final_name = name;
        
        // Check for hygiene violations
        if (is_macro_context) {
            // Check if this symbol would shadow an outer scope variable
            if (self.findSymbolInOuterScopes(name, current_scope)) |_| {
                // This is a potential shadowing violation
                try self.hygiene_violations.append(HygieneViolation{
                    .kind = .SymbolShadowing,
                    .symbol_name = name,
                    .macro_name = self.expansion_stack.items[self.expansion_stack.items.len - 1].macro_name,
                    .location = self.expansion_stack.items[self.expansion_stack.items.len - 1].expansion_location,
                });
                
                // Generate hygienic name
                final_name = try self.generateHygienicName(name);
            }
        }
        
        // Add symbol to current scope
        const symbol_info = Scope.SymbolInfo{
            .name = try self.allocator.dupe(u8, final_name),
            .symbol_type = symbol_type,
            .original_name = try self.allocator.dupe(u8, name),
            .is_macro_generated = is_macro_context,
            .definition_scope = current_scope.id,
        };
        
        try current_scope.symbols.put(try self.allocator.dupe(u8, final_name), symbol_info);
        
        // Track symbols introduced by macro
        if (is_macro_context) {
            try self.expansion_stack.items[self.expansion_stack.items.len - 1].symbols_introduced.append(try self.allocator.dupe(u8, final_name));
        }
        
        return final_name;
    }
    
    /// Resolve symbol reference with hygiene checking
    pub fn resolveSymbol(self: *MacroHygieneContext, name: []const u8) !?[]const u8 {
        const is_macro_context = self.expansion_stack.items.len > 0;
        
        // First, check if this symbol has been renamed in the current context
        if (is_macro_context) {
            const current_scope = &self.scope_stack.items[self.scope_stack.items.len - 1];
            const symbol_key = SymbolKey{
                .scope_id = current_scope.id,
                .symbol_name = name,
            };
            
            if (self.renamed_symbols.get(symbol_key)) |renamed| {
                return renamed;
            }
        }
        
        // Look up symbol in scope chain
        for (self.scope_stack.items) |*scope| {
            if (scope.symbols.get(name)) |symbol_info| {
                // Check for potential hygiene violation
                if (is_macro_context and !symbol_info.is_macro_generated) {
                    // Macro is referring to a symbol from outside its definition
                    const current_expansion = &self.expansion_stack.items[self.expansion_stack.items.len - 1];
                    
                    // Check if this is an intended capture or accidental
                    if (self.isAccidentalCapture(name, current_expansion)) {
                        try self.hygiene_violations.append(HygieneViolation{
                            .kind = .VariableCapture,
                            .symbol_name = name,
                            .macro_name = current_expansion.macro_name,
                            .location = current_expansion.expansion_location,
                        });
                    }
                }
                
                return symbol_info.name;
            }
        }
        
        return null; // Symbol not found
    }
    
    /// Generate hygienic name for symbol
    fn generateHygienicName(self: *MacroHygieneContext, original_name: []const u8) ![]const u8 {
        const unique_suffix = self.rename_counter;
        self.rename_counter += 1;
        
        const expansion_id = if (self.expansion_stack.items.len > 0) 
            self.expansion_stack.items[self.expansion_stack.items.len - 1].id 
        else 
            0;
        
        return std.fmt.allocPrint(self.allocator, "{s}__hyg_{d}_{d}", .{ original_name, expansion_id, unique_suffix });
    }
    
    /// Find symbol in outer scopes
    fn findSymbolInOuterScopes(self: *MacroHygieneContext, name: []const u8, current_scope: *Scope) ?*Scope.SymbolInfo {
        _ = self;
        var scope = current_scope.parent;
        while (scope) |s| {
            if (s.symbols.getPtr(name)) |symbol| {
                return symbol;
            }
            scope = s.parent;
        }
        return null;
    }
    
    /// Check if a symbol reference is an accidental capture
    fn isAccidentalCapture(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = self;
        _ = symbol_name;
        _ = expansion;
        
        // Heuristics for detecting accidental capture:
        // 1. Symbol was not explicitly marked as intended for capture
        // 2. Symbol is a common variable name (x, i, temp, etc.)
        // 3. Symbol was not passed as a parameter to the macro
        
        // For now, assume all external references are potentially accidental
        return true;
    }
    
    /// Check for hygiene violations after macro expansion
    fn checkHygieneViolations(self: *MacroHygieneContext, expansion: MacroExpansion) !void {
        // Check if any symbols introduced by the macro escape their intended scope
        for (expansion.symbols_introduced.items) |symbol_name| {
            if (self.symbolEscapesScope(symbol_name, &expansion)) {
                try self.hygiene_violations.append(HygieneViolation{
                    .kind = .ScopeEscape,
                    .symbol_name = symbol_name,
                    .macro_name = expansion.macro_name,
                    .location = expansion.expansion_location,
                });
            }
        }
    }
    
    /// Check if a symbol escapes its intended scope
    fn symbolEscapesScope(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = self;
        _ = symbol_name;
        _ = expansion;
        
        // For now, assume symbols don't escape
        // In a real implementation, this would check if the symbol is referenced
        // outside the macro's expansion scope
        return false;
    }
    
    /// Apply automatic hygiene fixes
    pub fn applyHygieneFixes(self: *MacroHygieneContext) !void {
        for (self.hygiene_violations.items) |*violation| {
            switch (violation.kind) {
                .VariableCapture => {
                    // Rename the captured variable to make the capture explicit
                    try self.fixVariableCapture(violation);
                },
                .SymbolShadowing => {
                    // Generate a unique name for the shadowing symbol
                    try self.fixSymbolShadowing(violation);
                },
                .UnintendedBinding => {
                    // Qualify the binding to make it explicit
                    try self.fixUnintendedBinding(violation);
                },
                .ScopeEscape => {
                    // Restrict the symbol's scope
                    try self.fixScopeEscape(violation);
                },
            }
        }
        
        // Clear violations after fixing
        self.hygiene_violations.clearRetainingCapacity();
    }
    
    /// Fix variable capture violation
    fn fixVariableCapture(self: *MacroHygieneContext, violation: *HygieneViolation) !void {
        // Generate a captured variable name that makes the capture explicit
        const captured_name = try std.fmt.allocPrint(self.allocator, "captured_{s}_from_{s}", .{ violation.symbol_name, violation.macro_name });
        
        // Record the renaming
        const symbol_key = SymbolKey{
            .scope_id = self.current_scope_id,
            .symbol_name = violation.symbol_name,
        };
        try self.renamed_symbols.put(symbol_key, captured_name);
    }
    
    /// Fix symbol shadowing violation
    fn fixSymbolShadowing(self: *MacroHygieneContext, violation: *HygieneViolation) !void {
        const unique_name = try self.generateHygienicName(violation.symbol_name);
        
        const symbol_key = SymbolKey{
            .scope_id = self.current_scope_id,
            .symbol_name = violation.symbol_name,
        };
        try self.renamed_symbols.put(symbol_key, unique_name);
    }
    
    /// Fix unintended binding violation
    fn fixUnintendedBinding(self: *MacroHygieneContext, violation: *HygieneViolation) !void {
        // Make the binding explicit by qualifying it
        const qualified_name = try std.fmt.allocPrint(self.allocator, "{s}::{s}", .{ violation.macro_name, violation.symbol_name });
        
        const symbol_key = SymbolKey{
            .scope_id = self.current_scope_id,
            .symbol_name = violation.symbol_name,
        };
        try self.renamed_symbols.put(symbol_key, qualified_name);
    }
    
    /// Fix scope escape violation
    fn fixScopeEscape(self: *MacroHygieneContext, violation: *HygieneViolation) !void {
        // Generate a name that's scoped to the macro
        const scoped_name = try std.fmt.allocPrint(self.allocator, "{s}_scoped_{s}", .{ violation.macro_name, violation.symbol_name });
        
        const symbol_key = SymbolKey{
            .scope_id = self.current_scope_id,
            .symbol_name = violation.symbol_name,
        };
        try self.renamed_symbols.put(symbol_key, scoped_name);
    }
    
    /// Generate hygiene report
    pub fn generateHygieneReport(self: *MacroHygieneContext) ![]const u8 {
        var report = ArrayList(u8).init(self.allocator);
        defer report.deinit();
        
        try report.writer().print("Macro Hygiene Report\n");
        try report.writer().print("===================\n\n");
        
        if (self.hygiene_violations.items.len == 0) {
            try report.writer().print("No hygiene violations detected.\n");
        } else {
            try report.writer().print("Found {d} hygiene violations:\n\n", .{self.hygiene_violations.items.len});
            
            for (self.hygiene_violations.items, 0..) |violation, i| {
                try report.writer().print("{}. {} in macro '{}' at {}\n", .{ 
                    i + 1, 
                    violation.kind, 
                    violation.macro_name, 
                    violation.location 
                });
                try report.writer().print("   Symbol: {}\n", .{violation.symbol_name});
                try report.writer().print("   Resolution: {}\n\n", .{self.getViolationResolution(violation.kind)});
            }
        }
        
        return report.toOwnedSlice();
    }
    
    /// Get resolution description for violation type
    fn getViolationResolution(self: *MacroHygieneContext, kind: HygieneViolation.ViolationKind) []const u8 {
        _ = self;
        return switch (kind) {
            .VariableCapture => "Renamed captured variable to make capture explicit",
            .SymbolShadowing => "Generated unique name to prevent shadowing",
            .UnintendedBinding => "Qualified binding to make intent clear",
            .ScopeEscape => "Restricted symbol scope to prevent escape",
        };
    }
};

// Test cases for macro hygiene
test "basic hygiene violation detection" {
    var hygiene_ctx = try MacroHygieneContext.init(std.testing.allocator);
    defer hygiene_ctx.deinit();
    
    // Declare a variable in global scope
    _ = try hygiene_ctx.declareSymbol("x", .Variable);
    
    // Begin macro expansion
    _ = try hygiene_ctx.beginMacroExpansion("test_macro", "test.csd:10");
    
    // Try to declare a variable with the same name (should detect shadowing)
    _ = try hygiene_ctx.declareSymbol("x", .Variable);
    
    // End macro expansion
    try hygiene_ctx.endMacroExpansion();
    
    // Check that hygiene violation was detected
    try std.testing.expect(hygiene_ctx.hygiene_violations.items.len > 0);
    try std.testing.expect(hygiene_ctx.hygiene_violations.items[0].kind == .SymbolShadowing);
}

test "symbol resolution with renaming" {
    var hygiene_ctx = try MacroHygieneContext.init(std.testing.allocator);
    defer hygiene_ctx.deinit();
    
    // Begin macro expansion
    _ = try hygiene_ctx.beginMacroExpansion("test_macro", "test.csd:5");
    
    // Declare a symbol
    const renamed = try hygiene_ctx.declareSymbol("temp", .Variable);
    
    // Resolve the symbol
    const resolved = try hygiene_ctx.resolveSymbol("temp");
    
    try std.testing.expect(resolved != null);
    try std.testing.expectEqualStrings(renamed, resolved.?);
    
    // End macro expansion
    try hygiene_ctx.endMacroExpansion();
}
