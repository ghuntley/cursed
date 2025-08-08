const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
});

/// Variable information for LLVM compilation
pub const VariableInfo = struct {
    name: []const u8,
    llvm_value: c.LLVMValueRef,  // The alloca'd value
    llvm_type: c.LLVMTypeRef,    // LLVM type
    cursed_type: []const u8,     // Original CURSED type ("drip", "tea", etc.)
    is_parameter: bool = false,  // True if this is a function parameter
    scope_id: u32 = 0,          // Scope where variable was declared
    
    pub fn deinit(self: *VariableInfo, allocator: Allocator) void {
        allocator.free(self.name);
        allocator.free(self.cursed_type);
    }
};

/// Scope information for variable resolution
pub const Scope = struct {
    id: u32,
    parent: ?*Scope,
    variables: HashMap([]const u8, VariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, id: u32, parent: ?*Scope) Scope {
        return Scope{
            .id = id,
            .parent = parent,
            .variables = HashMap([]const u8, VariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *Scope) void {
        var iterator = self.variables.iterator();
        while (iterator.next()) |entry| {
            var var_info = entry.value_ptr;
            var_info.deinit(self.variables.allocator);
        }
        self.variables.deinit();
    }
    
    /// Look up a variable in this scope or parent scopes
    pub fn lookup(self: *Scope, name: []const u8) ?VariableInfo {
        if (self.variables.get(name)) |var_info| {
            return var_info;
        }
        
        if (self.parent) |parent| {
            return parent.lookup(name);
        }
        
        return null;
    }
    
    /// Define a variable in this scope
    pub fn define(self: *Scope, name: []const u8, var_info: VariableInfo) !void {
        // Duplicate the name and type for storage
        const allocator = self.variables.allocator;
        const name_copy = try allocator.dupe(u8, name);
        const type_copy = try allocator.dupe(u8, var_info.cursed_type);
        
        var stored_info = var_info;
        stored_info.name = name_copy;
        stored_info.cursed_type = type_copy;
        stored_info.scope_id = self.id;
        
        try self.variables.put(name_copy, stored_info);
    }
};

/// Variable scope manager for LLVM compilation
pub const VariableScopeManager = struct {
    allocator: Allocator,
    scopes: ArrayList(*Scope),
    current_scope: ?*Scope,
    next_scope_id: u32,
    
    pub fn init(allocator: Allocator) VariableScopeManager {
        return VariableScopeManager{
            .allocator = allocator,
            .scopes = ArrayList(*Scope).init(allocator),
            .current_scope = null,
            .next_scope_id = 0,
        };
    }
    
    pub fn deinit(self: *VariableScopeManager) void {
        // Clean up all scopes
        for (self.scopes.items) |scope| {
            scope.deinit();
            self.allocator.destroy(scope);
        }
        self.scopes.deinit();
    }
    
    /// Enter a new scope
    pub fn enterScope(self: *VariableScopeManager) !*Scope {
        const scope = try self.allocator.create(Scope);
        scope.* = Scope.init(self.allocator, self.next_scope_id, self.current_scope);
        self.next_scope_id += 1;
        
        try self.scopes.append(scope);
        self.current_scope = scope;
        
        return scope;
    }
    
    /// Exit the current scope
    pub fn exitScope(self: *VariableScopeManager) void {
        if (self.current_scope) |scope| {
            self.current_scope = scope.parent;
        }
    }
    
    /// Look up a variable in the current scope chain
    pub fn lookup(self: *VariableScopeManager, name: []const u8) ?VariableInfo {
        if (self.current_scope) |scope| {
            return scope.lookup(name);
        }
        return null;
    }
    
    /// Define a variable in the current scope
    pub fn define(self: *VariableScopeManager, name: []const u8, var_info: VariableInfo) !void {
        if (self.current_scope) |scope| {
            try scope.define(name, var_info);
        } else {
            return error.NoCurrentScope;
        }
    }
    
    /// Get the current scope ID
    pub fn getCurrentScopeId(self: *VariableScopeManager) u32 {
        if (self.current_scope) |scope| {
            return scope.id;
        }
        return 0;
    }
}

/// Convert CURSED types to LLVM types
pub fn cursedTypeToLLVMType(context: c.LLVMContextRef, cursed_type: []const u8) c.LLVMTypeRef {
    if (std.mem.eql(u8, cursed_type, "drip")) return c.LLVMInt64TypeInContext(context);
    if (std.mem.eql(u8, cursed_type, "normie")) return c.LLVMInt32TypeInContext(context);
    if (std.mem.eql(u8, cursed_type, "lit")) return c.LLVMInt1TypeInContext(context);
    if (std.mem.eql(u8, cursed_type, "meal")) return c.LLVMDoubleTypeInContext(context);
    if (std.mem.eql(u8, cursed_type, "tea")) return c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
    
    // Default to i32
    return c.LLVMInt32TypeInContext(context);
}

/// Get LLVM type alignment for alloca
pub fn getLLVMTypeAlignment(llvm_type: c.LLVMTypeRef) u32 {
    const type_kind = c.LLVMGetTypeKind(llvm_type);
    return switch (type_kind) {
        c.LLVMIntegerTypeKind => {
            const width = c.LLVMGetIntTypeWidth(llvm_type);
            return if (width == 64) 8 else if (width == 32) 4 else if (width == 16) 2 else 1;
        },
        c.LLVMDoubleTypeKind => 8,
        c.LLVMFloatTypeKind => 4,
        c.LLVMPointerTypeKind => 8, // Assuming 64-bit pointers
        else => 4, // Default alignment
    };
}
