// P0 Sprint 1: Standalone Type Checker Demo
// Self-contained implementation with no external dependencies

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Simple CURSED type system
pub const CursedType = enum {
    DrIP,      // drip - integer
    Normie,    // normie - integer
    Tea,       // tea - string
    Lit,       // lit - boolean
    Facts,     // facts - boolean
    Cap,       // cap - void
    Vibes,     // vibes - float
    Array,     // []Type
    Struct,    // struct
    Unknown,   // unknown/error type
    
    pub fn toString(self: CursedType) []const u8 {
        return switch (self) {
            .DrIP => "drip",
            .Normie => "normie",
            .Tea => "tea",
            .Lit => "lit",
            .Facts => "facts",
            .Cap => "cap",
            .Vibes => "vibes",
            .Array => "array",
            .Struct => "struct",
            .Unknown => "unknown",
        };
    }
    
    pub fn fromString(name: []const u8) CursedType {
        if (std.mem.eql(u8, name, "drip")) return .DrIP;
        if (std.mem.eql(u8, name, "normie")) return .Normie;
        if (std.mem.eql(u8, name, "tea")) return .Tea;
        if (std.mem.eql(u8, name, "lit")) return .Lit;
        if (std.mem.eql(u8, name, "facts")) return .Facts;
        if (std.mem.eql(u8, name, "cap")) return .Cap;
        if (std.mem.eql(u8, name, "vibes")) return .Vibes;
        if (std.mem.eql(u8, name, "array")) return .Array;
        return .Unknown;
    }
    
    pub fn isNumeric(self: CursedType) bool {
        return switch (self) {
            .DrIP, .Normie, .Vibes => true,
            else => false,
        };
    }
    
    pub fn isBoolean(self: CursedType) bool {
        return switch (self) {
            .Lit, .Facts => true,
            else => false,
        };
    }
};

// Variable in symbol table
pub const Variable = struct {
    name: []const u8,
    var_type: CursedType,
    is_mutable: bool,
    is_initialized: bool,
};

// Struct field definition
pub const StructField = struct {
    name: []const u8,
    field_type: CursedType,
};

// Struct type definition
pub const StructType = struct {
    name: []const u8,
    fields: std.ArrayList(StructField),
    
    pub fn init(allocator: Allocator, name: []const u8) StructType {
        _ = allocator;
        return StructType{
            .name = name,
            .fields = std.ArrayList(StructField){},
        };
    }
    
    pub fn deinit(self: *StructType, allocator: Allocator) void {
        self.fields.deinit(allocator);
    }
    
    pub fn addField(self: *StructType, allocator: Allocator, name: []const u8, field_type: CursedType) !void {
        const field = StructField{
            .name = name,
            .field_type = field_type,
        };
        try self.fields.append(allocator, field);
    }
    
    pub fn getField(self: *const StructType, name: []const u8) ?StructField {
        for (self.fields.items) |field| {
            if (std.mem.eql(u8, field.name, name)) {
                return field;
            }
        }
        return null;
    }
};

// Type checking error
pub const TypeCheckError = struct {
    message: []const u8,
    kind: ErrorKind,
    
    pub const ErrorKind = enum {
        UnknownType,
        UnknownVariable,
        UnknownField,
        TypeMismatch,
        InvalidOperation,
        VariableAlreadyDeclared,
    };
};

// String context for hashmaps
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

// Main type checker  
pub const TypeChecker = struct {
    variables: std.HashMap([]const u8, Variable, StringContext, std.hash_map.default_max_load_percentage),
    struct_types: std.HashMap([]const u8, StructType, StringContext, std.hash_map.default_max_load_percentage), 
    errors: std.ArrayList(TypeCheckError),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) TypeChecker {
        return TypeChecker{
            .variables = std.HashMap([]const u8, Variable, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .struct_types = std.HashMap([]const u8, StructType, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .errors = std.ArrayList(TypeCheckError){},
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TypeChecker) void {
        // Clean up variables
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
        }
        self.variables.deinit();
        
        // Clean up struct types
        var struct_iter = self.struct_types.iterator();
        while (struct_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.struct_types.deinit();
        
        // Clean up errors
        for (self.errors.items) |error_item| {
            self.allocator.free(error_item.message);
        }
        self.errors.deinit(self.allocator);
    }
    
    // Declare a variable
    pub fn declareVariable(self: *TypeChecker, name: []const u8, type_name: []const u8, is_mutable: bool) !CursedType {
        const var_type = CursedType.fromString(type_name);
        
        if (var_type == .Unknown) {
            try self.addError(.UnknownType, try std.fmt.allocPrint(self.allocator, "Unknown type '{s}'", .{type_name}));
            return .Unknown;
        }
        
        if (self.variables.contains(name)) {
            try self.addError(.VariableAlreadyDeclared, try std.fmt.allocPrint(self.allocator, "Variable '{s}' already declared", .{name}));
            return .Unknown;
        }
        
        const name_copy = try self.allocator.dupe(u8, name);
        const variable = Variable{
            .name = name_copy,
            .var_type = var_type,
            .is_mutable = is_mutable,
            .is_initialized = false,
        };
        
        try self.variables.put(name_copy, variable);
        return var_type;
    }
    
    // Look up a variable
    pub fn getVariable(self: *TypeChecker, name: []const u8) !CursedType {
        if (self.variables.get(name)) |variable| {
            return variable.var_type;
        }
        
        try self.addError(.UnknownVariable, try std.fmt.allocPrint(self.allocator, "Unknown variable '{s}'", .{name}));
        return .Unknown;
    }
    
    // Register a struct type
    pub fn registerStructType(self: *TypeChecker, name: []const u8, fields: anytype) !void {
        var struct_type = StructType.init(self.allocator, name);
        
        for (fields) |field| {
            const field_type = CursedType.fromString(field.type_name);
            if (field_type == .Unknown) {
                try self.addError(.UnknownType, try std.fmt.allocPrint(self.allocator, "Unknown type '{s}' for field '{s}'", .{ field.type_name, field.name }));
                continue;
            }
            try struct_type.addField(self.allocator, field.name, field_type);
        }
        
        const name_copy = try self.allocator.dupe(u8, name);
        try self.struct_types.put(name_copy, struct_type);
    }
    
    // Get struct type
    pub fn getStructType(self: *TypeChecker, name: []const u8) ?*StructType {
        return self.struct_types.getPtr(name);
    }
    
    // Check field access
    pub fn checkFieldAccess(self: *TypeChecker, struct_name: []const u8, field_name: []const u8) !CursedType {
        const struct_type = self.getStructType(struct_name) orelse {
            try self.addError(.UnknownType, try std.fmt.allocPrint(self.allocator, "Unknown struct '{s}'", .{struct_name}));
            return .Unknown;
        };
        
        const field = struct_type.getField(field_name) orelse {
            try self.addError(.UnknownField, try std.fmt.allocPrint(self.allocator, "Unknown field '{s}' in struct '{s}'", .{ field_name, struct_name }));
            return .Unknown;
        };
        
        return field.field_type;
    }
    
    // Check binary operation
    pub fn checkBinaryOperation(self: *TypeChecker, left_type: CursedType, right_type: CursedType, operator: []const u8) !CursedType {
        // Comparison operations
        if (std.mem.eql(u8, operator, "==") or 
            std.mem.eql(u8, operator, "!=") or
            std.mem.eql(u8, operator, "<") or
            std.mem.eql(u8, operator, ">") or
            std.mem.eql(u8, operator, "<=") or
            std.mem.eql(u8, operator, ">=")) {
            
            if (self.areCompatible(left_type, right_type)) {
                return .Lit; // boolean result
            } else {
                try self.addError(.TypeMismatch, try std.fmt.allocPrint(self.allocator, "Cannot compare '{s}' and '{s}'", .{ left_type.toString(), right_type.toString() }));
                return .Unknown;
            }
        }
        
        // Arithmetic operations
        if (std.mem.eql(u8, operator, "+") or 
            std.mem.eql(u8, operator, "-") or
            std.mem.eql(u8, operator, "*") or
            std.mem.eql(u8, operator, "/")) {
            
            // String concatenation
            if (std.mem.eql(u8, operator, "+") and left_type == .Tea and right_type == .Tea) {
                return .Tea;
            }
            
            // Numeric operations
            if (left_type.isNumeric() and right_type.isNumeric()) {
                // Return the "larger" type (simplified coercion)
                if (left_type == .Vibes or right_type == .Vibes) {
                    return .Vibes;
                }
                return left_type;
            }
            
            try self.addError(.InvalidOperation, try std.fmt.allocPrint(self.allocator, "Cannot apply '{s}' to '{s}' and '{s}'", .{ operator, left_type.toString(), right_type.toString() }));
            return .Unknown;
        }
        
        try self.addError(.InvalidOperation, try std.fmt.allocPrint(self.allocator, "Unknown operator '{s}'", .{operator}));
        return .Unknown;
    }
    
    // Check if types are compatible for assignment/comparison
    fn areCompatible(self: *TypeChecker, source: CursedType, target: CursedType) bool {
        _ = self;
        
        // Exact match
        if (source == target) return true;
        
        // Numeric types are compatible with each other
        if (source.isNumeric() and target.isNumeric()) return true;
        
        // Boolean types are compatible with each other
        if (source.isBoolean() and target.isBoolean()) return true;
        
        return false;
    }
    
    // Add an error
    fn addError(self: *TypeChecker, kind: TypeCheckError.ErrorKind, message: []const u8) !void {
        const error_item = TypeCheckError{
            .kind = kind,
            .message = message,
        };
        try self.errors.append(self.allocator, error_item);
    }
    
    // Check if there are errors
    pub fn hasErrors(self: *const TypeChecker) bool {
        return self.errors.items.len > 0;
    }
    
    // Print all errors
    pub fn printErrors(self: *const TypeChecker) void {
        for (self.errors.items) |error_item| {
            std.log.err("{s}: {s}", .{ @tagName(error_item.kind), error_item.message });
        }
    }
};

// Demo program
pub fn runTypeCheckerDemo(allocator: Allocator) !void {
    std.log.info("🚀 P0 Sprint 1: CURSED Type Checker Simple Mode Demo", .{});
    std.log.info("=================================================", .{});
    
    var checker = TypeChecker.init(allocator);
    defer checker.deinit();
    
    std.log.info("", .{});
    std.log.info("📋 Testing Basic Variable Declarations", .{});
    
    // Test basic variable declarations
    _ = try checker.declareVariable("number", "drip", true);
    _ = try checker.declareVariable("text", "tea", false);
    _ = try checker.declareVariable("flag", "lit", true);
    _ = try checker.declareVariable("decimal", "vibes", false);
    
    std.log.info("✅ Declared variables: number (drip), text (tea), flag (lit), decimal (vibes)", .{});
    
    std.log.info("", .{});
    std.log.info("📋 Testing Variable Access", .{});
    
    // Test variable access
    const number_type = try checker.getVariable("number");
    const text_type = try checker.getVariable("text");
    std.log.info("✅ Accessed number: {s}, text: {s}", .{ number_type.toString(), text_type.toString() });
    
    std.log.info("", .{});
    std.log.info("📋 Testing Struct Definitions", .{});
    
    // Test struct definition
    const person_fields = [_]struct { name: []const u8, type_name: []const u8 }{
        .{ .name = "name", .type_name = "tea" },
        .{ .name = "age", .type_name = "drip" },
        .{ .name = "active", .type_name = "lit" },
    };
    
    try checker.registerStructType("Person", &person_fields);
    std.log.info("✅ Registered struct: Person (name: tea, age: drip, active: lit)", .{});
    
    std.log.info("", .{});
    std.log.info("📋 Testing Struct Field Access", .{});
    
    // Test struct field access
    const name_field_type = try checker.checkFieldAccess("Person", "name");
    const age_field_type = try checker.checkFieldAccess("Person", "age");
    std.log.info("✅ Person.name: {s}, Person.age: {s}", .{ name_field_type.toString(), age_field_type.toString() });
    
    std.log.info("", .{});
    std.log.info("📋 Testing Binary Operations", .{});
    
    // Test binary operations
    const add_result = try checker.checkBinaryOperation(.DrIP, .DrIP, "+");
    const cmp_result = try checker.checkBinaryOperation(.DrIP, .DrIP, "==");
    const concat_result = try checker.checkBinaryOperation(.Tea, .Tea, "+");
    
    std.log.info("✅ drip + drip = {s}", .{add_result.toString()});
    std.log.info("✅ drip == drip = {s}", .{cmp_result.toString()});
    std.log.info("✅ tea + tea = {s}", .{concat_result.toString()});
    
    std.log.info("", .{});
    std.log.info("📋 Testing Error Detection", .{});
    
    // Test error cases
    _ = try checker.getVariable("unknown_var");
    _ = try checker.checkFieldAccess("Person", "unknown_field");
    _ = try checker.checkBinaryOperation(.Tea, .DrIP, "+");
    
    std.log.info("📊 Generated {d} type checking errors (expected):", .{checker.errors.items.len});
    checker.printErrors();
    
    std.log.info("", .{});
    std.log.info("🎯 Type Checker Features Implemented:", .{});
    std.log.info("• ✅ Basic primitive type checking (drip, tea, lit, vibes)", .{});
    std.log.info("• ✅ Variable declaration and access", .{});
    std.log.info("• ✅ Struct definition and field access validation", .{});
    std.log.info("• ✅ Binary operation type checking with coercions", .{});
    std.log.info("• ✅ Type compatibility checking", .{});
    std.log.info("• ✅ Error detection and reporting", .{});
    std.log.info("• ✅ Symbol table management", .{});
    
    std.log.info("", .{});
    std.log.info("🎉 P0 Sprint 1 Type Checker implementation complete!", .{});
    std.log.info("Ready to handle basic CURSED programs with type safety.", .{});
}

// Main function for demo
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    try runTypeCheckerDemo(allocator);
}

// Unit tests
test "type checker basic functionality" {
    const allocator = std.testing.allocator;
    var checker = TypeChecker.init(allocator);
    defer checker.deinit();
    
    // Test variable declaration
    const int_type = try checker.declareVariable("x", "drip", true);
    try std.testing.expect(int_type == .DrIP);
    
    // Test variable access
    const accessed_type = try checker.getVariable("x");
    try std.testing.expect(accessed_type == .DrIP);
    
    // Test unknown variable
    const unknown_type = try checker.getVariable("unknown");
    try std.testing.expect(unknown_type == .Unknown);
    try std.testing.expect(checker.hasErrors());
}

test "struct field access" {
    const allocator = std.testing.allocator;
    var checker = TypeChecker.init(allocator);
    defer checker.deinit();
    
    const fields = [_]struct { name: []const u8, type_name: []const u8 }{
        .{ .name = "value", .type_name = "drip" },
        .{ .name = "name", .type_name = "tea" },
    };
    
    try checker.registerStructType("Test", &fields);
    
    const field_type = try checker.checkFieldAccess("Test", "value");
    try std.testing.expect(field_type == .DrIP);
}

test "binary operations" {
    const allocator = std.testing.allocator;
    var checker = TypeChecker.init(allocator);
    defer checker.deinit();
    
    // Test arithmetic
    const add_result = try checker.checkBinaryOperation(.DrIP, .DrIP, "+");
    try std.testing.expect(add_result == .DrIP);
    
    // Test comparison
    const cmp_result = try checker.checkBinaryOperation(.DrIP, .DrIP, "==");
    try std.testing.expect(cmp_result == .Lit);
    
    // Test string concatenation
    const concat_result = try checker.checkBinaryOperation(.Tea, .Tea, "+");
    try std.testing.expect(concat_result == .Tea);
}
