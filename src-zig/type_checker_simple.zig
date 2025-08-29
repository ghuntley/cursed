// P0 Sprint 1: Type Checker "Simple Mode"
// Implements basic type checking for non-generic CURSED programs

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const ast = @import("ast.zig");
const error_handling = @import("error_handling.zig");
const CursedError = error_handling.CursedError;

// Simple type representation
pub const SimpleType = enum {
    // Primitive types
    DrIP,      // int/integer
    Normie,    // int
    Tea,       // string
    Lit,       // boolean
    Facts,     // boolean variant
    Cap,       // void/unit
    Vibes,     // float/double
    Sip,       // character
    
    // Complex types
    Array,     // []Type
    Struct,    // squad Name
    Pointer,   // *Type
    Function,  // slay signature
    Unknown,   // For unresolved types
    
    pub fn toString(self: SimpleType) []const u8 {
        return switch (self) {
            .DrIP => "drip",
            .Normie => "normie", 
            .Tea => "tea",
            .Lit => "lit",
            .Facts => "facts",
            .Cap => "cap",
            .Vibes => "vibes",
            .Sip => "sip",
            .Array => "array",
            .Struct => "struct",
            .Pointer => "pointer",
            .Function => "function",
            .Unknown => "unknown",
        };
    }
    
    pub fn fromString(name: []const u8) SimpleType {
        if (std.mem.eql(u8, name, "drip")) return .DrIP;
        if (std.mem.eql(u8, name, "normie")) return .Normie;
        if (std.mem.eql(u8, name, "tea")) return .Tea;
        if (std.mem.eql(u8, name, "lit")) return .Lit;
        if (std.mem.eql(u8, name, "facts")) return .Facts;
        if (std.mem.eql(u8, name, "cap")) return .Cap;
        if (std.mem.eql(u8, name, "vibes")) return .Vibes;
        if (std.mem.eql(u8, name, "sip")) return .Sip;
        return .Unknown;
    }
    
    pub fn isNumeric(self: SimpleType) bool {
        return switch (self) {
            .DrIP, .Normie, .Vibes => true,
            else => false,
        };
    }
    
    pub fn isBoolean(self: SimpleType) bool {
        return switch (self) {
            .Lit, .Facts => true,
            else => false,
        };
    }
};

// Type information with metadata
pub const TypeInfo = struct {
    base_type: SimpleType,
    name: []const u8,
    element_type: ?*TypeInfo, // For arrays and pointers
    struct_fields: ?ArrayList(StructField),
    function_signature: ?FunctionSignature,
    allocator: Allocator,
    
    pub const StructField = struct {
        name: []const u8,
        field_type: *TypeInfo,
        is_public: bool,
    };
    
    pub const FunctionSignature = struct {
        parameters: ArrayList(*TypeInfo),
        return_type: ?*TypeInfo,
    };
    
    pub fn init(allocator: Allocator, base_type: SimpleType, name: []const u8) !*TypeInfo {
        const type_info = try allocator.create(TypeInfo);
        type_info.* = TypeInfo{
            .base_type = base_type,
            .name = try allocator.dupe(u8, name),
            .element_type = null,
            .struct_fields = null,
            .function_signature = null,
            .allocator = allocator,
        };
        return type_info;
    }
    
    pub fn deinit(self: *TypeInfo) void {
        self.allocator.free(self.name);
        if (self.struct_fields) |*fields| {
            for (fields.items) |field| {
                self.allocator.free(field.name);
                field.field_type.deinit();
            }
            fields.deinit();
        }
        if (self.function_signature) |*sig| {
            for (sig.parameters.items) |param| {
                param.deinit();
            }
            sig.parameters.deinit();
            if (sig.return_type) |ret| {
                ret.deinit();
            }
        }
        if (self.element_type) |elem| {
            elem.deinit();
        }
        self.allocator.destroy(self);
    }
    
    pub fn makeArray(allocator: Allocator, element_type: *TypeInfo) !*TypeInfo {
        const array_info = try TypeInfo.init(allocator, .Array, "array");
        array_info.element_type = element_type;
        return array_info;
    }
    
    pub fn makeStruct(allocator: Allocator, name: []const u8) !*TypeInfo {
        const struct_info = try TypeInfo.init(allocator, .Struct, name);
        struct_info.struct_fields = ArrayList(StructField){};
        return struct_info;
    }
};

// Variable in symbol table
pub const Variable = struct {
    name: []const u8,
    type_info: *TypeInfo,
    is_mutable: bool,
    is_initialized: bool,
    scope_level: usize,
    
    pub fn init(name: []const u8, type_info: *TypeInfo, is_mutable: bool, scope_level: usize) Variable {
        return Variable{
            .name = name,
            .type_info = type_info,
            .is_mutable = is_mutable,
            .is_initialized = false,
            .scope_level = scope_level,
        };
    }
};

// Symbol table with scope management
pub const SymbolTable = struct {
    scopes: ArrayList(HashMap([]const u8, Variable, StringContext, std.hash_map.default_max_load_percentage)),
    current_scope_level: usize,
    allocator: Allocator,
    
    const StringContext = struct {
        pub fn hash(self: @This(), s: []const u8) u64 {
            _ = self;
            return std.hash_map.hashString(s);
        }
        pub fn eql(self: @This(), a: []const u8, b: []const u8) bool {
            _ = self;
            return std.mem.eql(u8, a, b);
        }
    };
    
    pub fn init(allocator: Allocator) SymbolTable {
        _ = allocator;
        var table = SymbolTable{
            .scopes = ArrayList(HashMap([]const u8, Variable, StringContext, std.hash_map.default_max_load_percentage)).init(allocator),
            .current_scope_level = 0,
            .allocator = allocator,
        };
        
        // Create global scope
        const global_scope = HashMap([]const u8, Variable, StringContext, std.hash_map.default_max_load_percentage){};
        table.scopes.append(allocator, global_scope) catch unreachable;
        
        return table;
    }
    
    pub fn deinit(self: *SymbolTable) void {
        for (self.scopes.items) |*scope| {
            var iter = scope.iterator();
            while (iter.next()) |entry| {
                self.allocator.free(entry.key_ptr.*);
                // Note: TypeInfo cleanup is handled by TypeChecker
            }
            scope.deinit();
        }
        self.scopes.deinit(self.allocator);
    }
    
    pub fn enterScope(self: *SymbolTable) !void {
        const new_scope = HashMap([]const u8, Variable, StringContext, std.hash_map.default_max_load_percentage){};
        try self.scopes.append(allocator, new_scope);
        self.current_scope_level += 1;
    }
    
    pub fn exitScope(self: *SymbolTable) void {
        if (self.current_scope_level > 0) {
            var scope = self.scopes.pop();
            var iter = scope.iterator();
            while (iter.next()) |entry| {
                self.allocator.free(entry.key_ptr.*);
            }
            scope.deinit();
            self.current_scope_level -= 1;
        }
    }
    
    pub fn declareVariable(self: *SymbolTable, name: []const u8, type_info: *TypeInfo, is_mutable: bool) !void {
        const current_scope = &self.scopes.items[self.current_scope_level];
        
        // Check if variable already exists in current scope
        if (current_scope.contains(name)) {
            return CursedError.VariableAlreadyDeclared;
        }
        
        const name_copy = try self.allocator.dupe(u8, name);
        const variable = Variable.init(name_copy, type_info, is_mutable, self.current_scope_level);
        try current_scope.put(name_copy, variable);
    }
    
    pub fn lookupVariable(self: *SymbolTable, name: []const u8) ?*Variable {
        // Search from current scope to global scope
        var level = self.current_scope_level;
        while (true) {
            const scope = &self.scopes.items[level];
            if (scope.getPtr(name)) |variable| {
                return variable;
            }
            if (level == 0) break;
            level -= 1;
        }
        return null;
    }
    
    pub fn setVariableInitialized(self: *SymbolTable, name: []const u8) !void {
        if (self.lookupVariable(name)) |variable| {
            variable.is_initialized = true;
        } else {
            return CursedError.UnknownVariable;
        }
    }
};

// Type checking errors
pub const TypeCheckError = struct {
    kind: ErrorKind,
    message: []const u8,
    location: ?SourceLocation,
    allocator: Allocator,
    
    pub const ErrorKind = enum {
        TypeMismatch,
        UnknownVariable,
        UnknownType,
        UnknownField,
        UnknownMethod,
        VariableAlreadyDeclared,
        UndeclaredVariable,
        InvalidOperation,
        ArgumentCountMismatch,
        InvalidFieldAccess,
        NotAStruct,
        NotAnArray,
        NotAFunction,
        AssignmentToImmutable,
    };
    
    pub const SourceLocation = struct {
        line: usize,
        column: usize,
        file: []const u8,
    };
    
    pub fn init(allocator: Allocator, kind: ErrorKind, message: []const u8) !TypeCheckError {
        return TypeCheckError{
            .kind = kind,
            .message = try allocator.dupe(u8, message),
            .location = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TypeCheckError) void {
        self.allocator.free(self.message);
    }
};

// Main simple type checker
pub const SimpleTypeChecker = struct {
    symbol_table: SymbolTable,
    type_registry: HashMap([]const u8, *TypeInfo, StringContext, std.hash_map.default_max_load_percentage),
    errors: ArrayList(TypeCheckError),
    allocator: Allocator,
    
    const StringContext = struct {
        pub fn hash(self: @This(), s: []const u8) u64 {
            _ = self;
            return std.hash_map.hashString(s);
        }
        pub fn eql(self: @This(), a: []const u8, b: []const u8) bool {
            _ = self;
            return std.mem.eql(u8, a, b);
        }
    };
    
    pub fn init(allocator: Allocator) SimpleTypeChecker {
        _ = allocator;
        var checker = SimpleTypeChecker{
            .symbol_table = SymbolTable.init(allocator),
            .type_registry = HashMap([]const u8, *TypeInfo, StringContext, std.hash_map.default_max_load_percentage){},
            .errors = ArrayList(TypeCheckError){},
            .allocator = allocator,
        };
        
        // Register builtin types
        checker.registerBuiltinTypes() catch |err| {
            std.log.err("Failed to register builtin types: {}", .{err});
        };
        
        return checker;
    }
    
    pub fn deinit(self: *SimpleTypeChecker) void {
        // Clean up type registry
        var type_iter = self.type_registry.iterator();
        while (type_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.*.deinit();
        }
        self.type_registry.deinit(self.allocator);
        
        // Clean up errors
        for (self.errors.items) |*error_item| {
            error_item.deinit();
        }
        self.errors.deinit(self.allocator);
        
        // Clean up symbol table
        self.symbol_table.deinit(self.allocator);
    }
    
    fn registerBuiltinTypes(self: *SimpleTypeChecker) !void {
        const builtin_types = [_]struct { name: []const u8, simple_type: SimpleType }{
            .{ .name = "drip", .simple_type = .DrIP },
            .{ .name = "normie", .simple_type = .Normie },
            .{ .name = "tea", .simple_type = .Tea },
            .{ .name = "lit", .simple_type = .Lit },
            .{ .name = "facts", .simple_type = .Facts },
            .{ .name = "cap", .simple_type = .Cap },
            .{ .name = "vibes", .simple_type = .Vibes },
            .{ .name = "sip", .simple_type = .Sip },
        };
        
        for (builtin_types) |builtin| {
            const type_info = try TypeInfo.init(self.allocator, builtin.simple_type, builtin.name);
            const name_copy = try self.allocator.dupe(u8, builtin.name);
            try self.type_registry.put(name_copy, type_info);
        }
    }
    
    pub fn registerStructType(self: *SimpleTypeChecker, name: []const u8, fields: []const struct { name: []const u8, type_name: []const u8 }) !void {
        const struct_info = try TypeInfo.makeStruct(self.allocator, name);
        
        for (fields) |field| {
            const field_type = self.getTypeByName(field.type_name) orelse {
                try self.addError(.UnknownType, try std.fmt.allocPrint(self.allocator, "Unknown type '{s}' for field '{s}'", .{ field.type_name, field.name }));
                return;
            };
            
            const struct_field = TypeInfo.StructField{
                .name = try self.allocator.dupe(u8, field.name),
                .field_type = field_type,
                .is_public = true, // Simplified: all fields public
            };
            
            try struct_info.struct_fields.?.append(allocator, struct_field);
        }
        
        const name_copy = try self.allocator.dupe(u8, name);
        try self.type_registry.put(name_copy, struct_info);
    }
    
    pub fn getTypeByName(self: *SimpleTypeChecker, name: []const u8) ?*TypeInfo {
        return self.type_registry.get(name);
    }
    
    pub fn enterScope(self: *SimpleTypeChecker) !void {
        try self.symbol_table.enterScope();
    }
    
    pub fn exitScope(self: *SimpleTypeChecker) void {
        self.symbol_table.exitScope();
    }
    
    pub fn checkVariableDeclaration(self: *SimpleTypeChecker, name: []const u8, type_name: []const u8, is_mutable: bool) !*TypeInfo {
        const type_info = self.getTypeByName(type_name) orelse {
            try self.addError(.UnknownType, try std.fmt.allocPrint(self.allocator, "Unknown type '{s}'", .{type_name}));
            return self.getTypeByName("unknown").?;
        };
        
        self.symbol_table.declareVariable(name, type_info, is_mutable) catch |err| {
            switch (err) {
                CursedError.VariableAlreadyDeclared => {
                    try self.addError(.VariableAlreadyDeclared, try std.fmt.allocPrint(self.allocator, "Variable '{s}' already declared", .{name}));
                },
                else => return err,
            }
        };
        
        return type_info;
    }
    
    pub fn checkVariableAccess(self: *SimpleTypeChecker, name: []const u8) !*TypeInfo {
        const variable = self.symbol_table.lookupVariable(name) orelse {
            try self.addError(.UndeclaredVariable, try std.fmt.allocPrint(self.allocator, "Undeclared variable '{s}'", .{name}));
            return self.getTypeByName("unknown").?;
        };
        
        return variable.type_info;
    }
    
    pub fn checkFieldAccess(self: *SimpleTypeChecker, struct_type: *TypeInfo, field_name: []const u8) !*TypeInfo {
        if (struct_type.base_type != .Struct) {
            try self.addError(.NotAStruct, try std.fmt.allocPrint(self.allocator, "Type '{s}' is not a struct", .{struct_type.name}));
            return self.getTypeByName("unknown").?;
        }
        
        if (struct_type.struct_fields) |fields| {
            for (fields.items) |field| {
                if (std.mem.eql(u8, field.name, field_name)) {
                    return field.field_type;
                }
            }
        }
        
        try self.addError(.UnknownField, try std.fmt.allocPrint(self.allocator, "Unknown field '{s}' in struct '{s}'", .{ field_name, struct_type.name }));
        return self.getTypeByName("unknown").?;
    }
    
    pub fn checkBinaryOperation(self: *SimpleTypeChecker, left_type: *TypeInfo, right_type: *TypeInfo, operator: []const u8) !*TypeInfo {
        // Check for type compatibility
        if (!self.areTypesCompatible(left_type, right_type)) {
            try self.addError(.TypeMismatch, try std.fmt.allocPrint(self.allocator, "Cannot apply operator '{s}' to types '{s}' and '{s}'", .{ operator, left_type.name, right_type.name }));
            return self.getTypeByName("unknown").?;
        }
        
        // Determine result type based on operator
        if (std.mem.eql(u8, operator, "==") or 
            std.mem.eql(u8, operator, "!=") or
            std.mem.eql(u8, operator, "<") or
            std.mem.eql(u8, operator, ">") or
            std.mem.eql(u8, operator, "<=") or
            std.mem.eql(u8, operator, ">=")) {
            return self.getTypeByName("lit").?; // Boolean result
        }
        
        // Arithmetic operations return the common type
        if (left_type.base_type.isNumeric() and right_type.base_type.isNumeric()) {
            // Use the "larger" type (simplified coercion rules)
            if (left_type.base_type == .Vibes or right_type.base_type == .Vibes) {
                return self.getTypeByName("vibes").?;
            }
            return left_type; // Return left type for simplicity
        }
        
        // String concatenation
        if (std.mem.eql(u8, operator, "+") and 
            left_type.base_type == .Tea and right_type.base_type == .Tea) {
            return self.getTypeByName("tea").?;
        }
        
        try self.addError(.InvalidOperation, try std.fmt.allocPrint(self.allocator, "Invalid operation '{s}' between '{s}' and '{s}'", .{ operator, left_type.name, right_type.name }));
        return self.getTypeByName("unknown").?;
    }
    
    pub fn checkAssignment(self: *SimpleTypeChecker, variable_name: []const u8, value_type: *TypeInfo) !void {
        const variable = self.symbol_table.lookupVariable(variable_name) orelse {
            try self.addError(.UndeclaredVariable, try std.fmt.allocPrint(self.allocator, "Undeclared variable '{s}'", .{variable_name}));
            return;
        };
        
        if (!variable.is_mutable) {
            try self.addError(.AssignmentToImmutable, try std.fmt.allocPrint(self.allocator, "Cannot assign to immutable variable '{s}'", .{variable_name}));
            return;
        }
        
        if (!self.areTypesCompatible(value_type, variable.type_info)) {
            try self.addError(.TypeMismatch, try std.fmt.allocPrint(self.allocator, "Cannot assign value of type '{s}' to variable of type '{s}'", .{ value_type.name, variable.type_info.name }));
            return;
        }
        
        try self.symbol_table.setVariableInitialized(variable_name);
    }
    
    pub fn checkArrayAccess(self: *SimpleTypeChecker, array_type: *TypeInfo, index_type: *TypeInfo) !*TypeInfo {
        if (array_type.base_type != .Array) {
            try self.addError(.NotAnArray, try std.fmt.allocPrint(self.allocator, "Type '{s}' is not an array", .{array_type.name}));
            return self.getTypeByName("unknown").?;
        }
        
        if (!index_type.base_type.isNumeric()) {
            try self.addError(.TypeMismatch, try std.fmt.allocPrint(self.allocator, "Array index must be numeric, got '{s}'", .{index_type.name}));
            return self.getTypeByName("unknown").?;
        }
        
        return array_type.element_type orelse self.getTypeByName("unknown").?;
    }
    
    pub fn checkFunctionCall(self: *SimpleTypeChecker, function_name: []const u8, arg_types: []*TypeInfo) !*TypeInfo {
        // Simplified function checking - in a real implementation this would check
        // function signatures from the symbol table
        _ = function_name;
        _ = arg_types;
        
        // For now, return a generic type
        return self.getTypeByName("cap").?; // Return void for simplicity
    }
    
    fn areTypesCompatible(self: *SimpleTypeChecker, source: *TypeInfo, target: *TypeInfo) bool {
        _ = self;
        
        // Exact match
        if (source.base_type == target.base_type) return true;
        
        // Numeric coercions
        if (source.base_type.isNumeric() and target.base_type.isNumeric()) {
            return true; // Allow all numeric coercions for simplicity
        }
        
        // Boolean compatibility
        if (source.base_type.isBoolean() and target.base_type.isBoolean()) {
            return true;
        }
        
        return false;
    }
    
    fn addError(self: *SimpleTypeChecker, kind: TypeCheckError.ErrorKind, message: []const u8) !void {
        const error_item = try TypeCheckError.init(self.allocator, kind, message);
        try self.errors.append(allocator, error_item);
    }
    
    pub fn hasErrors(self: *SimpleTypeChecker) bool {
        return self.errors.items.len > 0;
    }
    
    pub fn getErrors(self: *SimpleTypeChecker) []const TypeCheckError {
        return self.errors.items;
    }
    
    pub fn printErrors(self: *SimpleTypeChecker) void {
        for (self.errors.items) |error_item| {
            std.log.err("Type Error: {s}", .{error_item.message});
        }
    }
};

// Test functions
test "simple type checker basic functionality" {
    const allocator = std.testing.allocator;
    var checker = SimpleTypeChecker.init(allocator);
    defer checker.deinit();
    
    // Test variable declaration
    const int_type = try checker.checkVariableDeclaration("x", "drip", true);
    try std.testing.expect(int_type.base_type == .DrIP);
    
    // Test variable access
    const accessed_type = try checker.checkVariableAccess("x");
    try std.testing.expect(accessed_type.base_type == .DrIP);
    
    // Test type compatibility
    const string_type = checker.getTypeByName("tea").?;
    try std.testing.expect(!checker.areTypesCompatible(int_type, string_type));
    
    std.log.info("✅ Simple type checker basic functionality test passed", .{});
}

test "simple type checker struct field access" {
    const allocator = std.testing.allocator;
    var checker = SimpleTypeChecker.init(allocator);
    defer checker.deinit();
    
    // Register a struct type
    const fields = [_]struct { name: []const u8, type_name: []const u8 }{
        .{ .name = "value", .type_name = "drip" },
        .{ .name = "name", .type_name = "tea" },
    };
    try checker.registerStructType("Person", &fields);
    
    // Test struct field access
    const struct_type = checker.getTypeByName("Person").?;
    const field_type = try checker.checkFieldAccess(struct_type, "value");
    try std.testing.expect(field_type.base_type == .DrIP);
    
    std.log.info("✅ Simple type checker struct field access test passed", .{});
}

test "simple type checker binary operations" {
    const allocator = std.testing.allocator;
    var checker = SimpleTypeChecker.init(allocator);
    defer checker.deinit();
    
    const int_type = checker.getTypeByName("drip").?;
    const bool_type = checker.getTypeByName("lit").?;
    
    // Test arithmetic operation
    const add_result = try checker.checkBinaryOperation(int_type, int_type, "+");
    try std.testing.expect(add_result.base_type == .DrIP);
    
    // Test comparison operation
    const cmp_result = try checker.checkBinaryOperation(int_type, int_type, "==");
    try std.testing.expect(cmp_result.base_type == .Lit);
    
    std.log.info("✅ Simple type checker binary operations test passed", .{});
}
