const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Complete Variable Scope Management (Priority #22)
// Block scoping and variable shadowing with lexical scoping

pub const ScopeError = error{
    VariableNotFound,
    VariableAlreadyDeclared,
    ScopeStackEmpty,
    InvalidScopeOperation,
    CircularReference,
    ShadowingNotAllowed,
};

pub const VariableState = enum {
    Declared,
    Initialized,
    ReadOnly,
    Captured,    // For closures
    Moved,       // For move semantics
};

pub const Variable = struct {
    name: []const u8,
    type_name: []const u8,
    state: VariableState,
    scope_depth: u32,
    declaration_line: u32,
    declaration_column: u32,
    is_mutable: bool,
    is_parameter: bool,
    is_captured: bool,
    initial_value: ?[]const u8,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8, type_name: []const u8, 
               scope_depth: u32, line: u32, column: u32) !Variable {
        return Variable{
            .name = try allocator.dupe(u8, name),
            .type_name = try allocator.dupe(u8, type_name),
            .state = .Declared,
            .scope_depth = scope_depth,
            .declaration_line = line,
            .declaration_column = column,
            .is_mutable = true,
            .is_parameter = false,
            .is_captured = false,
            .initial_value = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Variable) void {
        self.allocator.free(self.name);
        self.allocator.free(self.type_name);
        if (self.initial_value) |value| {
            self.allocator.free(value);
        }
    }
    
    pub fn setState(self: *Variable, new_state: VariableState) void {
        self.state = new_state;
    }
    
    pub fn setInitialValue(self: *Variable, value: []const u8) !void {
        if (self.initial_value) |old_value| {
            self.allocator.free(old_value);
        }
        self.initial_value = try self.allocator.dupe(u8, value);
        self.state = .Initialized;
    }
    
    pub fn canShadow(self: *const Variable, new_scope_depth: u32) bool {
        return new_scope_depth > self.scope_depth;
    }
    
    pub fn format(self: *const Variable, writer: anytype) !void {
        try writer.print("Variable{{ name: {s}, type: {s}, scope: {}, state: {} }}", 
                        .{ self.name, self.type_name, self.scope_depth, self.state });
    }
};

pub const ScopeType = enum {
    Global,
    Function,
    Block,
    Loop,
    Conditional,
    Struct,
    Interface,
    Module,
};

pub const Scope = struct {
    scope_type: ScopeType,
    depth: u32,
    parent: ?*Scope,
    children: ArrayList(*Scope),
    variables: HashMap([]const u8, Variable),
    captured_variables: ArrayList([]const u8),
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
    is_function_scope: bool,
    allows_shadowing: bool,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, scope_type: ScopeType, depth: u32, 
               parent: ?*Scope, start_line: u32, start_column: u32) Scope {
        return Scope{
            .scope_type = scope_type,
            .depth = depth,
            .parent = parent,
            .children = ArrayList(*Scope).init(allocator),
            .variables = HashMap([]const u8, Variable).init(allocator),
            .captured_variables = ArrayList([]const u8).init(allocator),
            .start_line = start_line,
            .start_column = start_column,
            .end_line = 0,
            .end_column = 0,
            .is_function_scope = scope_type == .Function,
            .allows_shadowing = scope_type != .Global,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Scope) void {
        // Clean up variables
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.variables.deinit();
        
        // Clean up children
        for (self.children.items) |child| {
            child.deinit();
            self.allocator.destroy(child);
        }
        self.children.deinit();
        
        self.captured_variables.deinit();
    }
    
    pub fn addChild(self: *Scope, child: *Scope) !void {
        try self.children.append(child);
    }
    
    pub fn declareVariable(self: *Scope, variable: Variable) ScopeError!void {
        // Check for redeclaration in current scope
        if (self.variables.contains(variable.name)) {
            return ScopeError.VariableAlreadyDeclared;
        }
        
        // Check for shadowing rules
        if (!self.allows_shadowing) {
            if (self.findVariableInParentScopes(variable.name)) |existing| {
                if (!existing.canShadow(self.depth)) {
                    return ScopeError.ShadowingNotAllowed;
                }
            }
        }
        
        try self.variables.put(variable.name, variable);
    }
    
    pub fn findVariable(self: *Scope, name: []const u8) ?*Variable {
        // First check current scope
        if (self.variables.getPtr(name)) |variable| {
            return variable;
        }
        
        // Then check parent scopes
        return self.findVariableInParentScopes(name);
    }
    
    pub fn findVariableInParentScopes(self: *Scope, name: []const u8) ?*Variable {
        var current_scope = self.parent;
        while (current_scope) |scope| {
            if (scope.variables.getPtr(name)) |variable| {
                // Mark as captured if accessed from child scope
                if (self.depth > scope.depth and scope.is_function_scope) {
                    variable.is_captured = true;
                    scope.captured_variables.append(name) catch {};
                }
                return variable;
            }
            current_scope = scope.parent;
        }
        return null;
    }
    
    pub fn updateVariable(self: *Scope, name: []const u8, new_state: VariableState) ScopeError!void {
        if (self.findVariable(name)) |variable| {
            variable.setState(new_state);
        } else {
            return ScopeError.VariableNotFound;
        }
    }
    
    pub fn setVariableValue(self: *Scope, name: []const u8, value: []const u8) ScopeError!void {
        if (self.findVariable(name)) |variable| {
            if (!variable.is_mutable and variable.state == .Initialized) {
                return ScopeError.InvalidScopeOperation;
            }
            try variable.setInitialValue(value);
        } else {
            return ScopeError.VariableNotFound;
        }
    }
    
    pub fn getVariablesAtDepth(self: *Scope, target_depth: u32, result: *ArrayList(Variable)) !void {
        if (self.depth == target_depth) {
            var var_iter = self.variables.iterator();
            while (var_iter.next()) |entry| {
                try result.append(entry.value_ptr.*);
            }
        }
        
        for (self.children.items) |child| {
            try child.getVariablesAtDepth(target_depth, result);
        }
    }
    
    pub fn close(self: *Scope, end_line: u32, end_column: u32) void {
        self.end_line = end_line;
        self.end_column = end_column;
    }
    
    pub fn format(self: *const Scope, writer: anytype) !void {
        try writer.print("Scope{{ type: {}, depth: {}, vars: {} }}", 
                        .{ self.scope_type, self.depth, self.variables.count() });
    }
};

pub const ScopeManager = struct {
    global_scope: *Scope,
    current_scope: *Scope,
    scope_stack: ArrayList(*Scope),
    current_depth: u32,
    max_depth: u32,
    function_scopes: ArrayList(*Scope),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) !ScopeManager {
        const global_scope = try allocator.create(Scope);
        global_scope.* = Scope.init(allocator, .Global, 0, null, 0, 0);
        
        var scope_stack = ArrayList(*Scope).init(allocator);
        try scope_stack.append(global_scope);
        
        return ScopeManager{
            .global_scope = global_scope,
            .current_scope = global_scope,
            .scope_stack = scope_stack,
            .current_depth = 0,
            .max_depth = 0,
            .function_scopes = ArrayList(*Scope).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ScopeManager) void {
        self.global_scope.deinit();
        self.allocator.destroy(self.global_scope);
        self.scope_stack.deinit();
        self.function_scopes.deinit();
    }
    
    pub fn enterScope(self: *ScopeManager, scope_type: ScopeType, start_line: u32, start_column: u32) !*Scope {
        self.current_depth += 1;
        if (self.current_depth > self.max_depth) {
            self.max_depth = self.current_depth;
        }
        
        const new_scope = try self.allocator.create(Scope);
        new_scope.* = Scope.init(self.allocator, scope_type, self.current_depth, 
                                self.current_scope, start_line, start_column);
        
        try self.current_scope.addChild(new_scope);
        try self.scope_stack.append(new_scope);
        self.current_scope = new_scope;
        
        if (scope_type == .Function) {
            try self.function_scopes.append(new_scope);
        }
        
        return new_scope;
    }
    
    pub fn exitScope(self: *ScopeManager, end_line: u32, end_column: u32) ScopeError!void {
        if (self.scope_stack.items.len <= 1) {
            return ScopeError.ScopeStackEmpty;
        }
        
        self.current_scope.close(end_line, end_column);
        _ = self.scope_stack.pop();
        self.current_scope = self.scope_stack.items[self.scope_stack.items.len - 1];
        self.current_depth -= 1;
    }
    
    pub fn declareVariable(self: *ScopeManager, name: []const u8, type_name: []const u8, 
                          line: u32, column: u32) ScopeError!void {
        const variable = try Variable.init(self.allocator, name, type_name, 
                                         self.current_depth, line, column);
        try self.current_scope.declareVariable(variable);
    }
    
    pub fn declareParameter(self: *ScopeManager, name: []const u8, type_name: []const u8,
                           line: u32, column: u32) ScopeError!void {
        var variable = try Variable.init(self.allocator, name, type_name,
                                       self.current_depth, line, column);
        variable.is_parameter = true;
        variable.is_mutable = false; // Parameters are immutable by default
        try self.current_scope.declareVariable(variable);
    }
    
    pub fn findVariable(self: *ScopeManager, name: []const u8) ?*Variable {
        return self.current_scope.findVariable(name);
    }
    
    pub fn updateVariable(self: *ScopeManager, name: []const u8, new_state: VariableState) ScopeError!void {
        try self.current_scope.updateVariable(name, new_state);
    }
    
    pub fn setVariableValue(self: *ScopeManager, name: []const u8, value: []const u8) ScopeError!void {
        try self.current_scope.setVariableValue(name, value);
    }
    
    pub fn getCurrentScope(self: *const ScopeManager) *Scope {
        return self.current_scope;
    }
    
    pub fn getCurrentFunctionScope(self: *const ScopeManager) ?*Scope {
        if (self.function_scopes.items.len == 0) return null;
        return self.function_scopes.items[self.function_scopes.items.len - 1];
    }
    
    pub fn checkShadowing(self: *ScopeManager, name: []const u8) bool {
        // Returns true if variable would shadow an existing variable
        if (self.current_scope.variables.contains(name)) {
            return false; // Not shadowing, it's a redeclaration
        }
        
        return self.current_scope.findVariableInParentScopes(name) != null;
    }
    
    pub fn getVariableInfo(self: *ScopeManager, name: []const u8) ?struct { 
        variable: *Variable, 
        scope_distance: u32,
        is_captured: bool,
    } {
        var distance: u32 = 0;
        var current_scope = self.current_scope;
        
        while (current_scope) |scope| {
            if (scope.variables.getPtr(name)) |variable| {
                return .{
                    .variable = variable,
                    .scope_distance = distance,
                    .is_captured = variable.is_captured,
                };
            }
            distance += 1;
            current_scope = scope.parent;
        }
        
        return null;
    }
    
    pub fn generateScopeReport(self: *ScopeManager, writer: anytype) !void {
        try writer.print("Scope Report:\n");
        try writer.print("Max depth: {}\n", .{self.max_depth});
        try writer.print("Current depth: {}\n", .{self.current_depth});
        try writer.print("Function scopes: {}\n", .{self.function_scopes.items.len});
        
        var all_variables = ArrayList(Variable).init(self.allocator);
        defer all_variables.deinit();
        
        for (0..self.max_depth + 1) |depth| {
            try self.global_scope.getVariablesAtDepth(@intCast(depth), &all_variables);
        }
        
        try writer.print("Total variables: {}\n", .{all_variables.items.len});
        
        // Check for captured variables
        var captured_count: u32 = 0;
        for (all_variables.items) |variable| {
            if (variable.is_captured) {
                captured_count += 1;
            }
        }
        try writer.print("Captured variables: {}\n", .{captured_count});
    }
};

// Global scope manager for runtime use
var global_scope_manager: ?ScopeManager = null;

pub fn initScopeManagement(allocator: Allocator) !void {
    global_scope_manager = try ScopeManager.init(allocator);
}

pub fn deinitScopeManagement() void {
    if (global_scope_manager) |*manager| {
        manager.deinit();
        global_scope_manager = null;
    }
}

// Export functions for parser integration
export fn cursed_enter_scope(scope_type: u32, line: u32, column: u32) void {
    if (global_scope_manager == null) return;
    
    const parsed_type: ScopeType = @enumFromInt(scope_type);
    _ = global_scope_manager.?.enterScope(parsed_type, line, column) catch return;
}

export fn cursed_exit_scope(line: u32, column: u32) void {
    if (global_scope_manager == null) return;
    
    global_scope_manager.?.exitScope(line, column) catch return;
}

export fn cursed_declare_variable(name_ptr: [*]const u8, name_len: usize,
                                 type_ptr: [*]const u8, type_len: usize,
                                 line: u32, column: u32) u32 {
    if (global_scope_manager == null) return 0;
    
    const name = name_ptr[0..name_len];
    const type_name = type_ptr[0..type_len];
    
    global_scope_manager.?.declareVariable(name, type_name, line, column) catch return 0;
    return 1;
}

export fn cursed_find_variable(name_ptr: [*]const u8, name_len: usize) u32 {
    if (global_scope_manager == null) return 0;
    
    const name = name_ptr[0..name_len];
    return if (global_scope_manager.?.findVariable(name) != null) 1 else 0;
}

export fn cursed_check_shadowing(name_ptr: [*]const u8, name_len: usize) u32 {
    if (global_scope_manager == null) return 0;
    
    const name = name_ptr[0..name_len];
    return if (global_scope_manager.?.checkShadowing(name)) 1 else 0;
}

// Testing
pub fn testScopeManagement() !void {
    print("Testing variable scope management...\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var manager = try ScopeManager.init(allocator);
    defer manager.deinit();
    
    // Test global scope
    try manager.declareVariable("global_var", "normie", 1, 1);
    
    // Test function scope
    _ = try manager.enterScope(.Function, 5, 1);
    try manager.declareParameter("param1", "tea", 5, 10);
    try manager.declareVariable("local_var", "lit", 6, 5);
    
    // Test block scope with shadowing
    _ = try manager.enterScope(.Block, 10, 1);
    try manager.declareVariable("local_var", "meal", 11, 5); // Should shadow
    
    // Test variable resolution
    const var1 = manager.findVariable("global_var");
    const var2 = manager.findVariable("param1");
    const var3 = manager.findVariable("local_var");
    
    std.testing.expect(var1 != null) catch return error.TestFailed;
    std.testing.expect(var2 != null) catch return error.TestFailed;
    std.testing.expect(var3 != null) catch return error.TestFailed;
    std.testing.expect(std.mem.eql(u8, var3.?.type_name, "meal")) catch return error.TestFailed;
    
    // Test scope exit
    try manager.exitScope(15, 1);
    const var4 = manager.findVariable("local_var");
    std.testing.expect(std.mem.eql(u8, var4.?.type_name, "lit")) catch return error.TestFailed;
    
    try manager.exitScope(20, 1);
    
    print("Scope management tests passed!\n");
}
