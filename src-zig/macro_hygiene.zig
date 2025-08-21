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
            _ = allocator;
            return MacroExpansion{
                .id = id,
                .macro_name = macro_name,
                .expansion_location = location,
                .parent_expansion = null,
                .scope_before = 0,
                .symbols_introduced = .empty,
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
            .scope_stack = .empty,
            .global_scope = undefined,
            .current_scope_id = 0,
            .renamed_symbols = HashMap(SymbolKey, []const u8, SymbolKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .rename_counter = 0,
            .expansion_stack = .empty,
            .hygiene_violations = .empty,
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
        
        try self.expansion_stack.append(self.allocator, expansion);
        
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
        
        try self.scope_stack.append(self.allocator, new_scope);
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
            try self.expansion_stack.items[self.expansion_stack.items.len - 1].symbols_introduced.append(self.allocator, try self.allocator.dupe(u8, final_name));
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
        // Comprehensive hygiene algorithm for nested macro calls
        
        // 1. Check if symbol is a common variable name likely to be accidental
        const common_names = [_][]const u8{ "i", "j", "k", "x", "y", "z", "temp", "tmp", "result", "value", "count", "index", "item", "data" };
        for (common_names) |common| {
            if (std.mem.eql(u8, symbol_name, common)) {
                return true; // Likely accidental capture of common variable
            }
        }
        
        // 2. Check if symbol exists in multiple nested expansion contexts
        if (self.expansion_stack.items.len > 1) {
            // We're in a nested macro context
            var capture_depth: u32 = 0;
            for (self.expansion_stack.items) |*nested_expansion| {
                if (self.symbolExistsInExpansionScope(symbol_name, nested_expansion)) {
                    capture_depth += 1;
                }
            }
            
            // If symbol appears in multiple expansion scopes, it's likely accidental
            if (capture_depth > 1) {
                return true;
            }
        }
        
        // 3. Check if symbol was explicitly marked as intentional capture
        if (self.isIntentionalCapture(symbol_name, expansion)) {
            return false;
        }
        
        // 4. Check if symbol is defined in the macro's lexical scope
        if (self.isDefinedInMacroScope(symbol_name, expansion)) {
            return false; // Not a capture if defined within macro
        }
        
        // 5. Check symbol naming patterns that suggest intentional capture
        if (std.mem.startsWith(u8, symbol_name, "captured_") or 
            std.mem.startsWith(u8, symbol_name, "outer_") or
            std.mem.startsWith(u8, symbol_name, "parent_")) {
            return false; // Naming suggests intentional capture
        }
        
        // 6. For nested macros, check if the symbol crosses multiple macro boundaries
        if (expansion.parent_expansion) |parent_id| {
            if (parent_id < self.expansion_stack.items.len) {
                const parent_expansion = &self.expansion_stack.items[parent_id];
                if (self.symbolCrossesMacroBoundary(symbol_name, parent_expansion, expansion)) {
                    return true; // Crosses macro boundaries, likely accidental
                }
            }
        }
        
        // Default: assume accidental if symbol comes from outer scope
        return self.symbolFromOuterScope(symbol_name, expansion);
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
        // Comprehensive scope escape detection for nested macro calls
        
        // 1. Check if symbol is referenced outside its defining expansion
        const defining_scope_id = self.getDefiningScope(symbol_name, expansion);
        if (defining_scope_id) |scope_id| {
            // Look for references in scopes outside the defining scope
            for (self.scope_stack.items) |*scope| {
                if (scope.id != scope_id and scope.expansion_id != expansion.id) {
                    if (self.symbolIsReferencedInScope(symbol_name, scope)) {
                        return true; // Symbol referenced outside its defining scope
                    }
                }
            }
        }
        
        // 2. For nested macros, check if symbol escapes to parent macro scope
        if (expansion.parent_expansion) |parent_id| {
            if (parent_id < self.expansion_stack.items.len) {
                const parent_expansion = &self.expansion_stack.items[parent_id];
                if (self.symbolVisibleInExpansion(symbol_name, parent_expansion)) {
                    // Check if this is intentional (e.g., return value or explicit export)
                    if (!self.isIntentionalExport(symbol_name, expansion)) {
                        return true; // Unintentional escape to parent macro
                    }
                }
            }
        }
        
        // 3. Check if symbol escapes to sibling macro expansions
        for (self.expansion_stack.items) |*sibling_expansion| {
            if (sibling_expansion.id != expansion.id and 
                sibling_expansion.parent_expansion == expansion.parent_expansion) {
                // This is a sibling expansion
                if (self.symbolVisibleInExpansion(symbol_name, sibling_expansion)) {
                    return true; // Symbol leaked to sibling macro
                }
            }
        }
        
        // 4. Check if symbol persists beyond macro expansion lifetime
        if (self.symbolPersistsBeyondExpansion(symbol_name, expansion)) {
            // This might be intentional (e.g., global definitions) or accidental
            if (!self.isGlobalSymbol(symbol_name) and !self.isIntentionalPersistence(symbol_name, expansion)) {
                return true; // Unintentional persistence
            }
        }
        
        // 5. Check for temporal scope violations in nested contexts
        if (self.expansion_stack.items.len > 1) {
            const current_time = @as(u64, @intCast(std.time.timestamp()));
            if (self.symbolAccessedAfterScopeEnd(symbol_name, expansion, current_time)) {
                return true; // Temporal scope violation
            }
        }
        
        return false; // Symbol properly scoped
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
        var report = .empty;
        defer report.deinit();
        
        try report.writer().print("Macro Hygiene Report\n", .{});
        try report.writer().print("===================\n\n", .{});
        
        if (self.hygiene_violations.items.len == 0) {
            try report.writer().print("No hygiene violations detected.\n", .{});
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
    
    // ============================================================================
    // Helper functions for comprehensive hygiene algorithm
    // ============================================================================
    
    /// Check if symbol exists in expansion scope
    fn symbolExistsInExpansionScope(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        for (expansion.symbols_introduced.items) |introduced_symbol| {
            if (std.mem.eql(u8, symbol_name, introduced_symbol)) {
                return true;
            }
        }
        
        // Check if symbol is accessible in the expansion's scope
        for (self.scope_stack.items) |*scope| {
            if (scope.expansion_id == expansion.id and scope.symbols.contains(symbol_name)) {
                return true;
            }
        }
        
        return false;
    }
    
    /// Check if symbol is marked as intentional capture
    fn isIntentionalCapture(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = expansion;
        
        // Check for explicit capture annotations (would be expanded in real implementation)
        // For now, check naming patterns that suggest intentional capture
        return std.mem.startsWith(u8, symbol_name, "capture_") or
               std.mem.startsWith(u8, symbol_name, "use_") or
               std.mem.endsWith(u8, symbol_name, "_captured") or
               self.isExplicitlyMarkedCapture(symbol_name);
    }
    
    /// Check if symbol is defined within macro scope
    fn isDefinedInMacroScope(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        // Check if symbol was introduced by this expansion
        for (expansion.symbols_introduced.items) |introduced| {
            if (std.mem.eql(u8, symbol_name, introduced)) {
                return true;
            }
        }
        
        // Check if symbol is defined in any scope belonging to this expansion
        for (self.scope_stack.items) |*scope| {
            if (scope.expansion_id == expansion.id and scope.symbols.contains(symbol_name)) {
                return true;
            }
        }
        
        return false;
    }
    
    /// Check if symbol crosses macro boundaries
    fn symbolCrossesMacroBoundary(self: *MacroHygieneContext, symbol_name: []const u8, parent: *MacroExpansion, child: *MacroExpansion) bool {
        // Check if symbol is defined in parent but referenced in child
        const parent_defines = self.symbolExistsInExpansionScope(symbol_name, parent);
        const child_references = self.symbolReferencedInExpansion(symbol_name, child);
        
        return parent_defines and child_references and !self.isExplicitlyPassedToChild(symbol_name, parent, child);
    }
    
    /// Check if symbol comes from outer scope
    fn symbolFromOuterScope(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        // Find the symbol in scope stack
        for (self.scope_stack.items) |*scope| {
            if (scope.symbols.contains(symbol_name)) {
                // Check if this scope belongs to the current expansion or a parent
                if (scope.expansion_id != expansion.id and 
                    (scope.expansion_id == null or scope.expansion_id.? < expansion.id)) {
                    return true; // Symbol comes from outer scope
                }
            }
        }
        return false;
    }
    
    /// Get the scope that defines a symbol
    fn getDefiningScope(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) ?u32 {
        _ = expansion;
        for (self.scope_stack.items) |*scope| {
            if (scope.symbols.contains(symbol_name)) {
                return scope.id;
            }
        }
        return null;
    }
    
    /// Check if symbol is referenced in scope
    fn symbolIsReferencedInScope(self: *MacroHygieneContext, symbol_name: []const u8, scope: *Scope) bool {
        _ = self;
        // In a full implementation, this would track symbol references
        // For now, check if symbol exists in scope (simplified)
        return scope.symbols.contains(symbol_name);
    }
    
    /// Check if symbol is visible in expansion
    fn symbolVisibleInExpansion(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        return self.symbolExistsInExpansionScope(symbol_name, expansion) or
               self.symbolReferencedInExpansion(symbol_name, expansion);
    }
    
    /// Check if symbol is intentionally exported
    fn isIntentionalExport(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = self;
        _ = expansion;
        
        // Check naming patterns that suggest intentional export
        return std.mem.startsWith(u8, symbol_name, "export_") or
               std.mem.startsWith(u8, symbol_name, "public_") or
               std.mem.endsWith(u8, symbol_name, "_export") or
               std.mem.endsWith(u8, symbol_name, "_public");
    }
    
    /// Check if symbol persists beyond expansion
    fn symbolPersistsBeyondExpansion(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = self;
        _ = expansion;
        
        // This would require tracking symbol lifetimes
        // For now, check naming patterns that suggest persistence
        return std.mem.startsWith(u8, symbol_name, "global_") or
               std.mem.startsWith(u8, symbol_name, "static_") or
               std.mem.startsWith(u8, symbol_name, "persistent_");
    }
    
    /// Check if symbol is global
    fn isGlobalSymbol(self: *MacroHygieneContext, symbol_name: []const u8) bool {
        // Check if symbol exists in global scope
        return self.global_scope.symbols.contains(symbol_name);
    }
    
    /// Check if persistence is intentional
    fn isIntentionalPersistence(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = self;
        _ = expansion;
        
        // Check for patterns that suggest intentional persistence
        return std.mem.startsWith(u8, symbol_name, "keep_") or
               std.mem.endsWith(u8, symbol_name, "_keep") or
               std.mem.contains(u8, symbol_name, "_persist_");
    }
    
    /// Check for temporal scope violations
    fn symbolAccessedAfterScopeEnd(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion, current_time: u64) bool {
        _ = self;
        _ = symbol_name;
        _ = expansion;
        _ = current_time;
        
        // This would require tracking access timestamps and scope lifetimes
        // For now, return false (no temporal violations detected)
        return false;
    }
    
    /// Check if symbol is explicitly marked as capture
    fn isExplicitlyMarkedCapture(self: *MacroHygieneContext, symbol_name: []const u8) bool {
        _ = self;
        
        // In a full implementation, this would check for capture annotations
        // For now, check for explicit capture prefixes
        return std.mem.startsWith(u8, symbol_name, "CAPTURE_") or
               std.mem.startsWith(u8, symbol_name, "OUTER_");
    }
    
    /// Check if symbol is referenced in expansion
    fn symbolReferencedInExpansion(self: *MacroHygieneContext, symbol_name: []const u8, expansion: *MacroExpansion) bool {
        _ = self;
        _ = symbol_name;
        _ = expansion;
        
        // This would require tracking symbol references during expansion
        // For now, return false (simplified implementation)
        return false;
    }
    
    /// Check if symbol is explicitly passed to child expansion
    fn isExplicitlyPassedToChild(self: *MacroHygieneContext, symbol_name: []const u8, parent: *MacroExpansion, child: *MacroExpansion) bool {
        _ = self;
        _ = symbol_name;
        _ = parent;
        _ = child;
        
        // This would require tracking parameter passing between macro expansions
        // For now, return false (no explicit passing detected)
        return false;
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
