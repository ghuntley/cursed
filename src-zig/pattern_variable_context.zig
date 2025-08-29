//! Pattern Variable Context Management for CURSED
//!
//! Manages variable bindings and scope during pattern matching with guard evaluation.
//! Provides runtime context for guard clauses to access pattern-bound variables.

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");

/// Runtime value representation for pattern matching
pub const PatternValue = union(enum) {
    Integer: i64,
    Float: f64,
    Boolean: bool,
    String: []const u8,
    Array: []PatternValue,
    Struct: HashMap([]const u8, PatternValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    Pointer: *anyopaque,
    Null: void,
    
    pub fn init(allocator: Allocator) PatternValue {
        _ = allocator;
        return PatternValue{
            .Struct = HashMap([]const u8, PatternValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *PatternValue, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .Array => |arr| {
                for (arr) |*value| {
                    value.deinit();
                }
                allocator.free(arr);
            },
            .Struct => |*map| {
                var iterator = map.iterator();
                while (iterator.next()) |entry| {
                    allocator.free(entry.key_ptr.*);
                    entry.value_ptr.deinit();
                }
                map.deinit();
            },
            .String => |str| allocator.free(str),
            else => {},
        }
    }
    
    /// Convert to string representation for debugging
    pub fn toString(self: PatternValue, allocator: Allocator) ![]const u8 {
        _ = allocator;
        return switch (self) {
            .Integer => |val| try std.fmt.allocPrint(allocator, "{}", .{val}),
            .Float => |val| try std.fmt.allocPrint(allocator, "{d}", .{val}),
            .Boolean => |val| try std.fmt.allocPrint(allocator, "{}", .{val}),
            .String => |str| try allocator.dupe(u8, str),
            .Array => |arr| blk: {
                var result = ArrayList(u8){};
                defer result.deinit();
                
                try result.appendSlice("[");
                for (arr, 0..) |item, i| {
                    if (i > 0) try result.appendSlice(", ");
                    const item_str = try item.toString(allocator);
                    defer allocator.free(item_str);
                    try result.appendSlice(item_str);
                }
                try result.appendSlice("]");
                
                break :blk try result.toOwnedSlice();
            },
            .Struct => |_| try std.fmt.allocPrint(allocator, "{{struct}}", .{}),
            .Pointer => try std.fmt.allocPrint(allocator, "{{ptr}}", .{}),
            .Null => try std.fmt.allocPrint(allocator, "null", .{}),
        };
    }
    
    /// Type-safe value comparison
    pub fn equals(self: PatternValue, other: PatternValue) bool {
        return switch (self) {
            .Integer => |a| switch (other) {
                .Integer => |b| a == b,
                else => false,
            },
            .Float => |a| switch (other) {
                .Float => |b| @abs(a - b) < 1e-9,
                else => false,
            },
            .Boolean => |a| switch (other) {
                .Boolean => |b| a == b,
                else => false,
            },
            .String => |a| switch (other) {
                .String => |b| std.mem.eql(u8, a, b),
                else => false,
            },
            .Null => switch (other) {
                .Null => true,
                else => false,
            },
            else => false, // Complex types need deep comparison
        };
    }
};

/// Variable binding information during pattern matching
pub const VariableBinding = struct {
    name: []const u8,
    value: PatternValue,
    type_name: []const u8,
    is_mutable: bool,
    scope_depth: usize,
    
    pub fn init(name: []const u8, value: PatternValue, type_name: []const u8, is_mutable: bool, scope_depth: usize) VariableBinding {
        return VariableBinding{
            .name = name,
            .value = value,
            .type_name = type_name,
            .is_mutable = is_mutable,
            .scope_depth = scope_depth,
        };
    }
    
    pub fn deinit(self: *VariableBinding, allocator: Allocator) void {
        _ = allocator;
        allocator.free(self.name);
        allocator.free(self.type_name);
        self.value.deinit(self.allocator);
    }
};

/// Pattern matching context with variable scope management
pub const PatternContext = struct {
    allocator: Allocator,
    variable_stack: ArrayList(VariableBinding),
    scope_depth: usize,
    guard_evaluation_active: bool,
    
    // Pattern matching state
    current_match_value: ?PatternValue,
    match_success: bool,
    
    // Debugging and diagnostics
    debug_mode: bool,
    match_trace: ArrayList([]const u8),
    
    pub fn init(allocator: Allocator) PatternContext {
        _ = allocator;
        return PatternContext{
            .allocator = allocator,
            .variable_stack = ArrayList(VariableBinding){},
            .scope_depth = 0,
            .guard_evaluation_active = false,
            .current_match_value = null,
            .match_success = false,
            .debug_mode = false,
            .match_trace = ArrayList([]const u8){},
        };
    }
    
    pub fn deinit(self: *PatternContext) void {
        // Clean up all variable bindings
        for (self.variable_stack.items) |*binding| {
            binding.deinit(self.allocator);
        }
        self.variable_stack.deinit(self.allocator);
        
        // Clean up match value if present
        if (self.current_match_value) |*value| {
            value.deinit(self.allocator);
        }
        
        // Clean up debug trace
        for (self.match_trace.items) |trace| {
            self.allocator.free(trace);
        }
        self.match_trace.deinit(self.allocator);
    }
    
    /// Enter a new pattern matching scope
    pub fn enterScope(self: *PatternContext) void {
        self.scope_depth += 1;
        
        if (self.debug_mode) {
            const trace_msg = std.fmt.allocPrint(self.allocator, "Entered scope depth {}", .{self.scope_depth}) catch return;
            self.match_trace.append(allocator, trace_msg) catch {};
        }
    }
    
    /// Exit current pattern matching scope
    pub fn exitScope(self: *PatternContext) void {
        if (self.scope_depth == 0) return;
        
        // Remove variables from current scope
        var i: usize = self.variable_stack.items.len;
        while (i > 0) {
            i -= 1;
            if (self.variable_stack.items[i].scope_depth == self.scope_depth) {
                var binding = self.variable_stack.swapRemove(i);
                binding.deinit(self.allocator);
            }
        }
        
        self.scope_depth -= 1;
        
        if (self.debug_mode) {
            const trace_msg = std.fmt.allocPrint(self.allocator, "Exited to scope depth {}", .{self.scope_depth}) catch return;
            self.match_trace.append(allocator, trace_msg) catch {};
        }
    }
    
    /// Bind a variable in the current scope
    pub fn bindVariable(self: *PatternContext, name: []const u8, value: PatternValue, type_name: []const u8, is_mutable: bool) !void {
        // Check if variable already exists in current scope
        for (self.variable_stack.items) |binding| {
            if (binding.scope_depth == self.scope_depth and std.mem.eql(u8, binding.name, name)) {
                return error.VariableAlreadyBound;
            }
        }
        
        const owned_name = try self.allocator.dupe(u8, name);
        const owned_type = try self.allocator.dupe(u8, type_name);
        
        const binding = VariableBinding.init(owned_name, value, owned_type, is_mutable, self.scope_depth);
        try self.variable_stack.append(allocator, binding);
        
        if (self.debug_mode) {
            const value_str = try value.toString(self.allocator);
            defer self.allocator.free(value_str);
            
            const trace_msg = try std.fmt.allocPrint(self.allocator, "Bound variable '{s}': {s} (type: {s})", .{ name, value_str, type_name });
            try self.match_trace.append(allocator, trace_msg);
        }
    }
    
    /// Look up a variable by name (searches from current scope upward)
    pub fn lookupVariable(self: *PatternContext, name: []const u8) ?*VariableBinding {
        var i: usize = self.variable_stack.items.len;
        while (i > 0) {
            i -= 1;
            if (std.mem.eql(u8, self.variable_stack.items[i].name, name)) {
                return &self.variable_stack.items[i];
            }
        }
        return null;
    }
    
    /// Set the current value being matched against
    pub fn setMatchValue(self: *PatternContext, value: PatternValue) void {
        if (self.current_match_value) |*old_value| {
            old_value.deinit(self.allocator);
        }
        self.current_match_value = value;
    }
    
    /// Get the current match value
    pub fn getMatchValue(self: *PatternContext) ?PatternValue {
        return self.current_match_value;
    }
    
    /// Start guard evaluation context
    pub fn startGuardEvaluation(self: *PatternContext) void {
        self.guard_evaluation_active = true;
        
        if (self.debug_mode) {
            const trace_msg = std.fmt.allocPrint(self.allocator, "Started guard evaluation", .{}) catch return;
            self.match_trace.append(allocator, trace_msg) catch {};
        }
    }
    
    /// End guard evaluation context
    pub fn endGuardEvaluation(self: *PatternContext) void {
        self.guard_evaluation_active = false;
        
        if (self.debug_mode) {
            const trace_msg = std.fmt.allocPrint(self.allocator, "Ended guard evaluation", .{}) catch return;
            self.match_trace.append(allocator, trace_msg) catch {};
        }
    }
    
    /// Check if currently in guard evaluation
    pub fn isInGuardEvaluation(self: *PatternContext) bool {
        return self.guard_evaluation_active;
    }
    
    /// Set match result
    pub fn setMatchResult(self: *PatternContext, success: bool) void {
        self.match_success = success;
        
        if (self.debug_mode) {
            const result_str = if (success) "success" else "failure";
            const trace_msg = std.fmt.allocPrint(self.allocator, "Match result: {s}", .{result_str}) catch return;
            self.match_trace.append(allocator, trace_msg) catch {};
        }
    }
    
    /// Get match result
    pub fn getMatchResult(self: *PatternContext) bool {
        return self.match_success;
    }
    
    /// Enable debug tracing
    pub fn enableDebug(self: *PatternContext) void {
        self.debug_mode = true;
    }
    
    /// Get debug trace as string
    pub fn getDebugTrace(self: *PatternContext) ![]const u8 {
        var result = ArrayList(u8){};
        defer result.deinit();
        
        try result.appendSlice("Pattern Matching Trace:\n");
        for (self.match_trace.items, 0..) |trace, i| {
            try result.writer().print("  {s}. {s}\n", .{ i + 1, trace });
        }
        
        return result.toOwnedSlice();
    }
    
    /// Clear debug trace
    pub fn clearDebugTrace(self: *PatternContext) void {
        for (self.match_trace.items) |trace| {
            self.allocator.free(trace);
        }
        self.match_trace.clearRetainingCapacity();
    }
    
    /// Print current context state (debugging utility)
    pub fn printContextState(self: *PatternContext) void {
        std.debug.print("=== Pattern Context State ===\n", .{});
        std.debug.print("Scope depth: {s}\n", .{self.scope_depth});
        std.debug.print("Guard evaluation: {s}\n", .{self.guard_evaluation_active});
        std.debug.print("Match success: {s}\n", .{self.match_success});
        std.debug.print("Variables ({s}):\n", .{self.variable_stack.items.len});
        
        for (self.variable_stack.items) |binding| {
            const value_str = binding.value.toString(self.allocator) catch "<?>";
            defer self.allocator.free(value_str);
            
            const mutability = if (binding.is_mutable) "mut" else "immut";
            std.debug.print("  {s} {s}: {s} = {s} (scope: {s})\n", .{ mutability, binding.type_name, binding.name, value_str, binding.scope_depth });
        }
        
        if (self.current_match_value) |match_value| {
            const match_str = match_value.toString(self.allocator) catch "<?>";
            defer self.allocator.free(match_str);
            std.debug.print("Current match value: {s}\n", .{match_str});
        }
        
        std.debug.print("==============================\n", .{});
    }
};

/// Advanced pattern matcher with context support
pub const AdvancedPatternMatcher = struct {
    context: *PatternContext,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, context: *PatternContext) AdvancedPatternMatcher {
        return AdvancedPatternMatcher{
            .context = context,
            .allocator = allocator,
        };
    }
    
    /// Match a pattern against a value with context support
    pub fn matchPattern(self: *AdvancedPatternMatcher, pattern: ast.Pattern, value: PatternValue) !bool {
        self.context.enterScope();
        defer self.context.exitScope();
        
        self.context.setMatchValue(value);
        
        const result = try self.matchPatternInternal(pattern, value);
        self.context.setMatchResult(result);
        
        return result;
    }
    
    /// Internal pattern matching with variable binding
    fn matchPatternInternal(self: *AdvancedPatternMatcher, pattern: ast.Pattern, value: PatternValue) !bool {
        return switch (pattern) {
            .Literal => |literal| try self.matchLiteral(literal, value),
            .Variable => |var_pattern| try self.matchVariable(var_pattern, value),
            .Wildcard => true, // Always matches
             value),
            .Struct => |struct_pattern| try self.matchStruct(struct_pattern, value),
            .Array => |array| try self.matchArray(array, value),
            .Or => |or_pattern| try self.matchOr(or_pattern, value),
            .Range => |range| try self.matchRange(range, value),
            .Guard => |guard| try self.matchGuard(guard, value),
            .Enum => |enum_pattern| try self.matchEnum(enum_pattern, value),
            .Type => |type_pattern| try self.matchType(type_pattern, value),
            else => false, // Unimplemented patterns
        };
    }
    
    /// Match literal pattern
    fn matchLiteral(self: *AdvancedPatternMatcher, literal: ast.Pattern.LiteralPattern, value: PatternValue) !bool {
        _ = self;
        return switch (literal.value) {
            .Integer => |expected| switch (value) {
                .Integer => |actual| expected == actual,
                else => false,
            },
            .Float => |expected| switch (value) {
                .Float => |actual| @abs(expected - actual) < 1e-9,
                else => false,
            },
            .String => |expected| switch (value) {
                .String => |actual| std.mem.eql(u8, expected, actual),
                else => false,
            },
            .Boolean => |expected| switch (value) {
                .Boolean => |actual| expected == actual,
                else => false,
            },
        };
    }
    
    /// Match variable pattern (always succeeds, binds variable)
    fn matchVariable(self: *AdvancedPatternMatcher, var_pattern: ast.Pattern.VariablePattern, value: PatternValue) !bool {
        const type_name = try self.inferTypeName(value);
        defer self.allocator.free(type_name);
        
        try self.context.bindVariable(var_pattern.name, value, type_name, var_pattern.is_mutable);
        return true;
    }
    
    /// Match tuple pattern
    fn matchTuple(self: *AdvancedPatternMatcher, tuple: ast.Pattern.TuplePattern, value: PatternValue) !bool {
        switch (value) {
            .Array => |arr| {
                if (tuple.patterns.len != arr.len) return false;
                
                for (tuple.patterns, 0..) |sub_pattern, i| {
                    if (!try self.matchPatternInternal(sub_pattern, arr[i])) {
                        return false;
                    }
                }
                return true;
            },
            else => return false,
        }
    }
    
    /// Match struct pattern
    fn matchStruct(self: *AdvancedPatternMatcher, struct_pattern: ast.Pattern.StructPattern, value: PatternValue) !bool {
        switch (value) {
            .Struct => |struct_map| {
                for (struct_pattern.fields) |field| {
                    if (struct_map.get(field.name)) |field_value| {
                        if (!try self.matchPatternInternal(field.pattern, field_value)) {
                            return false;
                        }
                    } else {
                        return false; // Required field not found
                    }
                }
                return true;
            },
            else => return false,
        }
    }
    
    /// Match array pattern
    fn matchArray(self: *AdvancedPatternMatcher, array: ast.Pattern.ArrayPattern, value: PatternValue) !bool {
        switch (value) {
            .Array => |arr| {
                if (array.patterns.len > arr.len) return false;
                
                for (array.patterns, 0..) |sub_pattern, i| {
                    if (i >= arr.len) return false;
                    if (!try self.matchPatternInternal(sub_pattern, arr[i])) {
                        return false;
                    }
                }
                return true;
            },
            else => return false,
        }
    }
    
    /// Match OR pattern (any alternative succeeds)
    fn matchOr(self: *AdvancedPatternMatcher, or_pattern: ast.Pattern.OrPattern, value: PatternValue) !bool {
        for (or_pattern.patterns) |alternative| {
            if (try self.matchPatternInternal(alternative, value)) {
                return true;
            }
        }
        return false;
    }
    
    /// Match range pattern
    fn matchRange(self: *AdvancedPatternMatcher, range: ast.Pattern.RangePattern, value: PatternValue) !bool {
        _ = self;
        _ = range;
        
        return switch (value) {
            .Integer => |val| {
                // Simplified range matching - would need to evaluate range.start and range.end
                const start: i64 = 0; // Placeholder
                const end: i64 = 100; // Placeholder
                if (range.is_inclusive) {
                    return val >= start and val <= end;
                } else {
                    return val >= start and val < end;
                }
            },
            else => false,
        };
    }
    
    /// Match guard pattern
    fn matchGuard(self: *AdvancedPatternMatcher, guard: ast.Pattern.GuardPattern, value: PatternValue) !bool {
        // First match the base pattern
        if (!try self.matchPatternInternal(guard.pattern.*, value)) {
            return false;
        }
        
        // Then evaluate the guard condition with variable context
        self.context.startGuardEvaluation();
        defer self.context.endGuardEvaluation();
        
        // Simplified guard evaluation - would need expression evaluator
        return true; // Placeholder: assume guard passes
    }
    
    /// Match enum pattern
    fn matchEnum(self: *AdvancedPatternMatcher, enum_pattern: ast.Pattern.EnumPattern, value: PatternValue) !bool {
        _ = self;
        _ = enum_pattern;
        
        // Simplified enum matching - would need enum value representation
        return switch (value) {
            .Integer => |val| val == 0, // Placeholder: assume variant 0
            else => false,
        };
    }
    
    /// Match type pattern
    fn matchType(self: *AdvancedPatternMatcher, type_pattern: ast.Pattern.TypePattern, value: PatternValue) !bool {
        _ = type_pattern;
        
        const type_name = try self.inferTypeName(value);
        defer self.allocator.free(type_name);
        
        // Simplified type matching - would need type system integration
        _ = type_name;
        return true; // Placeholder
    }
    
    /// Infer type name from pattern value
    fn inferTypeName(self: *AdvancedPatternMatcher, value: PatternValue) ![]const u8 {
        return switch (value) {
            .Integer => try self.allocator.dupe(u8, "drip"),
            .Float => try self.allocator.dupe(u8, "spill"),
            .Boolean => try self.allocator.dupe(u8, "lit"),
            .String => try self.allocator.dupe(u8, "tea"),
            .Array => try self.allocator.dupe(u8, "array"),
            .Struct => try self.allocator.dupe(u8, "struct"),
            .Pointer => try self.allocator.dupe(u8, "pointer"),
            .Null => try self.allocator.dupe(u8, "null"),
        };
    }
};

// Test cases for pattern variable context
test "pattern context basic functionality" {
    var context = PatternContext.init(std.testing.allocator);
    defer context.deinit();
    
    // Test variable binding
    const value = PatternValue{ .Integer = 42 };
    try context.bindVariable("x", value, "drip", false);
    
    // Test variable lookup
    const binding = context.lookupVariable("x");
    try std.testing.expect(binding != null);
    try std.testing.expect(binding.?.value.Integer == 42);
    try std.testing.expectEqualStrings("x", binding.?.name);
}

test "pattern context scope management" {
    var context = PatternContext.init(std.testing.allocator);
    defer context.deinit();
    
    // Outer scope
    const value1 = PatternValue{ .Integer = 1 };
    try context.bindVariable("outer", value1, "drip", false);
    
    context.enterScope();
    
    // Inner scope
    const value2 = PatternValue{ .Integer = 2 };
    try context.bindVariable("inner", value2, "drip", false);
    
    // Both variables should be accessible
    try std.testing.expect(context.lookupVariable("outer") != null);
    try std.testing.expect(context.lookupVariable("inner") != null);
    
    context.exitScope();
    
    // Only outer variable should remain
    try std.testing.expect(context.lookupVariable("outer") != null);
    try std.testing.expect(context.lookupVariable("inner") == null);
}
