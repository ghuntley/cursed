//! P30: Compile-time Reflection API
//! Provides type.fields and other compile-time introspection capabilities

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");
const type_system = @import("type_system_runtime.zig");

/// Compile-time type information
pub const CompileTimeTypeInfo = struct {
    name: []const u8,
    kind: TypeKind,
    size: ?usize = null,
    alignment: ?usize = null,
    fields: ?[]FieldInfo = null,
    methods: ?[]MethodInfo = null,
    interfaces: ?[][]const u8 = null,
    
    pub const TypeKind = enum {
        Primitive,
        Struct,
        Interface,
        Enum,
        Array,
        Slice,
        Pointer,
        Function,
        Generic,
    };
    
    pub const FieldInfo = struct {
        name: []const u8,
        field_type: []const u8,
        offset: usize,
        size: usize,
        is_public: bool,
        has_default: bool,
        default_value: ?[]const u8 = null,
    };
    
    pub const MethodInfo = struct {
        name: []const u8,
        return_type: []const u8,
        parameters: []ParameterInfo,
        is_public: bool,
        is_static: bool,
        is_async: bool,
        
        pub const ParameterInfo = struct {
            name: []const u8,
            param_type: []const u8,
        };
    };
    
    pub fn init(allocator: Allocator, name: []const u8, kind: TypeKind) !CompileTimeTypeInfo {
        return CompileTimeTypeInfo{
            .name = try allocator.dupe(u8, name),
            .kind = kind,
        };
    }
    
    pub fn deinit(self: *CompileTimeTypeInfo, allocator: Allocator) void {
        allocator.free(self.name);
        if (self.fields) |fields| {
            for (fields) |*field| {
                allocator.free(field.name);
                allocator.free(field.field_type);
                if (field.default_value) |default| {
                    allocator.free(default);
                }
            }
            allocator.free(fields);
        }
        if (self.methods) |methods| {
            for (methods) |*method| {
                allocator.free(method.name);
                allocator.free(method.return_type);
                for (method.parameters) |*param| {
                    allocator.free(param.name);
                    allocator.free(param.param_type);
                }
                allocator.free(method.parameters);
            }
            allocator.free(methods);
        }
        if (self.interfaces) |interfaces| {
            for (interfaces) |interface| {
                allocator.free(interface);
            }
            allocator.free(interfaces);
        }
    }
};

/// Compile-time reflection registry
pub const CompileTimeReflection = struct {
    allocator: Allocator,
    type_info_cache: HashMap([]const u8, CompileTimeTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    ast_cache: HashMap([]const u8, ASTTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const ASTTypeInfo = struct {
        struct_decl: ?*ast.StructStatement = null,
        interface_decl: ?*ast.InterfaceStatement = null,
        enum_decl: ?*ast.EnumStatement = null,
        function_decl: ?*ast.FunctionStatement = null,
    };
    
    pub fn init() CompileTimeReflection {
        return CompileTimeReflection{
            .allocator = allocator,
            .type_info_cache = HashMap([]const u8, CompileTimeTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .ast_cache = HashMap([]const u8, ASTTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *CompileTimeReflection) void {
        var type_iterator = self.type_info_cache.iterator();
        while (type_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.type_info_cache.deinit();
        self.ast_cache.deinit();
    }
    
    /// Register struct declaration for compile-time reflection
    pub fn registerStruct(self: *CompileTimeReflection, struct_decl: *ast.StructStatement) !void {
        const ast_info = ASTTypeInfo{ .struct_decl = struct_decl };
        try self.ast_cache.put(struct_decl.name, ast_info);
        
        // Generate compile-time type info
        var type_info = try CompileTimeTypeInfo.init(self.allocator, struct_decl.name, .Struct);
        
        // Analyze struct fields
        var fields = .empty;
        var offset: usize = 0;
        
        for (struct_decl.fields.items) |*field| {
            const field_size = try self.calculateTypeSize(field.field_type);
            try fields.append(self.allocator, CompileTimeTypeInfo.FieldInfo{
                .name = try self.allocator.dupe(u8, field.name),
                .field_type = try self.typeToString(field.field_type),
                .offset = offset,
                .size = field_size,
                .is_public = field.visibility == .Public,
                .has_default = field.default_value != null,
                .default_value = if (field.default_value) |default| try self.expressionToString(default) else null,
            });
            offset += field_size;
        }
        
        type_info.fields = try fields.toOwnedSlice(self.allocator);
        type_info.size = offset;
        type_info.alignment = 8; // Default alignment
        
        try self.type_info_cache.put(try self.allocator.dupe(u8, struct_decl.name), type_info);
    }
    
    /// Register interface declaration for compile-time reflection
    pub fn registerInterface(self: *CompileTimeReflection, interface_decl: *ast.InterfaceStatement) !void {
        const ast_info = ASTTypeInfo{ .interface_decl = interface_decl };
        try self.ast_cache.put(interface_decl.name, ast_info);
        
        var type_info = try CompileTimeTypeInfo.init(self.allocator, interface_decl.name, .Interface);
        
        // Analyze interface methods
        var methods = .empty;
        
        for (interface_decl.methods.items) |*method| {
            var parameters = .empty;
            
            for (method.parameters.items) |*param| {
                try parameters.append(self.allocator, CompileTimeTypeInfo.MethodInfo.ParameterInfo{
                    .name = try self.allocator.dupe(u8, param.name),
                    .param_type = try self.typeToString(param.param_type),
                });
            }
            
            try methods.append(self.allocator, CompileTimeTypeInfo.MethodInfo{
                .name = try self.allocator.dupe(u8, method.name),
                .return_type = if (method.return_type) |ret_type| try self.typeToString(ret_type) else try self.allocator.dupe(u8, "vibes"),
                .parameters = try parameters.toOwnedSlice(self.allocator),
                .is_public = true, // Interface methods are always public
                .is_static = false,
                .is_async = method.is_async,
            });
        }
        
        type_info.methods = try methods.toOwnedSlice(self.allocator);
        
        try self.type_info_cache.put(try self.allocator.dupe(u8, interface_decl.name), type_info);
    }
    
    /// Register enum declaration for compile-time reflection
    pub fn registerEnum(self: *CompileTimeReflection, enum_decl: *ast.EnumStatement) !void {
        const ast_info = ASTTypeInfo{ .enum_decl = enum_decl };
        try self.ast_cache.put(enum_decl.name, ast_info);
        
        var type_info = try CompileTimeTypeInfo.init(self.allocator, enum_decl.name, .Enum);
        
        // Enums have tag + largest variant size
        var max_variant_size: usize = 0;
        for (enum_decl.variants.items) |*variant| {
            const variant_size = if (variant.associated_type) |assoc_type| 
                try self.calculateTypeSize(assoc_type) 
            else 
                0;
            max_variant_size = @max(max_variant_size, variant_size);
        }
        
        type_info.size = 4 + max_variant_size; // 4 bytes for tag + variant data
        type_info.alignment = 8;
        
        try self.type_info_cache.put(try self.allocator.dupe(u8, enum_decl.name), type_info);
    }
    
    /// Get compile-time type information
    pub fn getTypeInfo(self: *CompileTimeReflection, type_name: []const u8) ?*CompileTimeTypeInfo {
        return self.type_info_cache.getPtr(type_name);
    }
    
    /// Generate type.fields information at compile time
    pub fn generateFieldsInfo(self: *CompileTimeReflection, type_name: []const u8) !?[]const u8 {
        const type_info = self.getTypeInfo(type_name) orelse return null;
        
        if (type_info.fields == null) return null;
        
        var result = .empty;
        defer result.deinit();
        
        try result.writer().print("[\n", .{});
        for (type_info.fields.?, 0..) |field, i| {
            if (i > 0) try result.writer().print(",\n", .{});
            try result.writer().print("  {{ .name = \"{s}\", .type = \"{s}\", .offset = {d}, .size = {d} }}", 
                .{ field.name, field.field_type, field.offset, field.size });
        }
        try result.writer().print("\n]", .{});
        
        return result.toOwnedSlice();
    }
    
    /// Generate type.methods information at compile time
    pub fn generateMethodsInfo(self: *CompileTimeReflection, type_name: []const u8) !?[]const u8 {
        const type_info = self.getTypeInfo(type_name) orelse return null;
        
        if (type_info.methods == null) return null;
        
        var result = .empty;
        defer result.deinit();
        
        try result.writer().print("[\n", .{});
        for (type_info.methods.?, 0..) |method, i| {
            if (i > 0) try result.writer().print(",\n", .{});
            try result.writer().print("  {{ .name = \"{s}\", .return_type = \"{s}\", .parameters = [", 
                .{ method.name, method.return_type });
            
            for (method.parameters, 0..) |param, j| {
                if (j > 0) try result.writer().print(", ", .{});
                try result.writer().print("{{ .name = \"{s}\", .type = \"{s}\" }}", 
                    .{ param.name, param.param_type });
            }
            
            try result.writer().print("] }}", .{});
        }
        try result.writer().print("\n]", .{});
        
        return result.toOwnedSlice(self.allocator);
    }
    
    /// Generate type.size information at compile time
    pub fn generateSizeInfo(self: *CompileTimeReflection, type_name: []const u8) !?[]const u8 {
        const type_info = self.getTypeInfo(type_name) orelse return null;
        
        if (type_info.size == null) return null;
        
        return std.fmt.allocPrint(self.allocator, "{d}", .{type_info.size.?});
    }
    
    /// Generate type.alignment information at compile time
    pub fn generateAlignmentInfo(self: *CompileTimeReflection, type_name: []const u8) !?[]const u8 {
        const type_info = self.getTypeInfo(type_name) orelse return null;
        
        if (type_info.alignment == null) return null;
        
        return std.fmt.allocPrint(self.allocator, "{d}", .{type_info.alignment.?});
    }
    
    /// Calculate type size for layout purposes
    fn calculateTypeSize(self: *CompileTimeReflection, type_expr: ast.Type) !usize {
        _ = self;
        return switch (type_expr) {
            .Primitive => |primitive| switch (primitive) {
                .Lit => 1,
                .Smol => 1,
                .Normie => 4,
                .Drip, .Thicc => 8,
                .Snack => 4,
                .Meal => 8,
                .Tea => 8, // Pointer to string
                .Vibes => 0,
            },
            .Identifier => 8, // Assume pointer size for user types
            .Array => |array_type| {
                const element_size = try self.calculateTypeSize(array_type.element_type.*);
                return element_size * array_type.size;
            },
            .Slice => 16, // Pointer + length
            .Pointer => 8,
            else => 8,
        };
    }
    
    /// Convert type to string representation
    fn typeToString(self: *CompileTimeReflection, type_expr: ast.Type) ![]const u8 {
        return switch (type_expr) {
            .Primitive => |primitive| switch (primitive) {
                .Lit => try self.allocator.dupe(u8, "lit"),
                .Smol => try self.allocator.dupe(u8, "smol"),
                .Normie => try self.allocator.dupe(u8, "normie"),
                .Drip => try self.allocator.dupe(u8, "drip"),
                .Thicc => try self.allocator.dupe(u8, "thicc"),
                .Snack => try self.allocator.dupe(u8, "snack"),
                .Meal => try self.allocator.dupe(u8, "meal"),
                .Tea => try self.allocator.dupe(u8, "tea"),
                .Vibes => try self.allocator.dupe(u8, "vibes"),
            },
            .Identifier => |name| try self.allocator.dupe(u8, name),
            .Array => |array_type| {
                const element_type_str = try self.typeToString(array_type.element_type.*);
                defer self.allocator.free(element_type_str);
                return std.fmt.allocPrint(self.allocator, "[{d}]{s}", .{ array_type.size, element_type_str });
            },
            .Slice => |slice_type| {
                const element_type_str = try self.typeToString(slice_type.element_type.*);
                defer self.allocator.free(element_type_str);
                return std.fmt.allocPrint(self.allocator, "[]{s}", .{element_type_str});
            },
            .Pointer => |pointer_type| {
                const target_type_str = try self.typeToString(pointer_type.target_type.*);
                defer self.allocator.free(target_type_str);
                return std.fmt.allocPrint(self.allocator, "*{s}", .{target_type_str});
            },
            else => try self.allocator.dupe(u8, "unknown"),
        };
    }
    
    /// Convert expression to string representation
    fn expressionToString(self: *CompileTimeReflection, expr: *ast.Expression) ![]const u8 {
        return switch (expr.*) {
            .Literal => |literal| switch (literal) {
                .Integer => |int_val| std.fmt.allocPrint(self.allocator, "{d}", .{int_val}),
                .Float => |float_val| std.fmt.allocPrint(self.allocator, "{d}", .{float_val}),
                .String => |str_val| std.fmt.allocPrint(self.allocator, "\"{s}\"", .{str_val}),
                .Boolean => |bool_val| std.fmt.allocPrint(self.allocator, "{}", .{bool_val}),
            },
            else => try self.allocator.dupe(u8, "complex_expression"),
        };
    }
};

/// Compile-time code generation from reflection
pub const CompileTimeCodeGen = struct {
    reflection: *CompileTimeReflection,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, reflection: *CompileTimeReflection) CompileTimeCodeGen {
        return CompileTimeCodeGen{
            .reflection = reflection,
            .allocator = allocator,
        };
    }
    
    /// Generate field accessor functions at compile time
    pub fn generateFieldAccessors(self: *CompileTimeCodeGen, type_name: []const u8) !?[]const u8 {
        const type_info = self.reflection.getTypeInfo(type_name) orelse return null;
        
        if (type_info.fields == null) return null;
        
        var code = .empty;
        defer code.deinit();
        
        try code.writer().print("// Auto-generated field accessors for {s}\n\n", .{type_name});
        
        for (type_info.fields.?) |field| {
            // Generate getter
            try code.writer().print("slay get_{s}_{s}(obj *{s}) {s} {{\n", .{ type_name, field.name, type_name, field.field_type });
            try code.writer().print("    damn obj.{s}\n", .{field.name});
            try code.writer().print("}}\n\n", .{});
            
            // Generate setter (if not immutable)
            try code.writer().print("slay set_{s}_{s}(obj *{s}, value {s}) {{\n", .{ type_name, field.name, type_name, field.field_type });
            try code.writer().print("    obj.{s} = value\n", .{field.name});
            try code.writer().print("}}\n\n", .{});
        }
        
        return code.toOwnedSlice();
    }
    
    /// Generate type information constants at compile time
    pub fn generateTypeConstants(self: *CompileTimeCodeGen, type_name: []const u8) !?[]const u8 {
        const type_info = self.reflection.getTypeInfo(type_name) orelse return null;
        
        var code = .empty;
        defer code.deinit();
        
        try code.writer().print("// Auto-generated type constants for {s}\n\n", .{type_name});
        
        if (type_info.size) |size| {
            try code.writer().print("sus {s}_SIZE normie = {d}\n", .{ type_name, size });
        }
        
        if (type_info.alignment) |alignment| {
            try code.writer().print("sus {s}_ALIGNMENT normie = {d}\n", .{ type_name, alignment });
        }
        
        if (type_info.fields) |fields| {
            try code.writer().print("sus {s}_FIELD_COUNT normie = {d}\n", .{ type_name, fields.len });
            
            for (fields, 0..) |field, i| {
                try code.writer().print("sus {s}_{s}_OFFSET normie = {d}\n", .{ type_name, field.name, field.offset });
                try code.writer().print("sus {s}_{s}_SIZE normie = {d}\n", .{ type_name, field.name, field.size });
                try code.writer().print("sus {s}_{s}_INDEX normie = {d}\n", .{ type_name, field.name, i });
            }
        }
        
        return code.toOwnedSlice();
    }
    
    /// Generate serialization functions at compile time
    pub fn generateSerialization(self: *CompileTimeCodeGen, type_name: []const u8) !?[]const u8 {
        const type_info = self.reflection.getTypeInfo(type_name) orelse return null;
        
        if (type_info.fields == null) return null;
        
        var code = .empty;
        defer code.deinit();
        
        try code.writer().print("// Auto-generated serialization for {s}\n\n", .{type_name});
        
        // Generate to_string function
        try code.writer().print("slay {s}_to_string(obj {s}) tea {{\n", .{ type_name, type_name });
        try code.writer().print("    sus result tea = \"{s} {{ \"\n", .{type_name});
        
        for (type_info.fields.?, 0..) |field, i| {
            if (i > 0) {
                try code.writer().print("    result = result + \", \"\n");
            }
            try code.writer().print("    result = result + \"{s}: \" + to_string(obj.{s})\n", .{ field.name, field.name });
        }
        
        try code.writer().print("    result = result + \" }}\"\n");
        try code.writer().print("    damn result\n", .{});
        try code.writer().print("}}\n\n", .{});
        
        return code.toOwnedSlice();
    }
};

/// Macro integration for compile-time reflection
pub const ReflectionMacros = struct {
    reflection: *CompileTimeReflection,
    codegen: *CompileTimeCodeGen,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, reflection: *CompileTimeReflection, codegen: *CompileTimeCodeGen) ReflectionMacros {
        return ReflectionMacros{
            .reflection = reflection,
            .codegen = codegen,
            .allocator = allocator,
        };
    }
    
    /// Expand type.fields macro at compile time
    pub fn expandTypeFields(self: *ReflectionMacros, type_name: []const u8) !?[]const u8 {
        return self.reflection.generateFieldsInfo(type_name);
    }
    
    /// Expand type.methods macro at compile time
    pub fn expandTypeMethods(self: *ReflectionMacros, type_name: []const u8) !?[]const u8 {
        return self.reflection.generateMethodsInfo(type_name);
    }
    
    /// Expand type.size macro at compile time
    pub fn expandTypeSize(self: *ReflectionMacros, type_name: []const u8) !?[]const u8 {
        return self.reflection.generateSizeInfo(type_name);
    }
    
    /// Expand generate_accessors macro at compile time
    pub fn expandGenerateAccessors(self: *ReflectionMacros, type_name: []const u8) !?[]const u8 {
        return self.codegen.generateFieldAccessors(type_name);
    }
};

// Test cases for compile-time reflection
test "compile-time type info registration" {
    var reflection = CompileTimeReflection.init(std.testing.allocator);
    defer reflection.deinit();
    
    // Create a mock struct declaration
    var fields = .empty;
    defer fields.deinit();
    
    try fields.append(ast.StructField{
        .name = "name",
        .field_type = ast.Type{ .Primitive = .Tea },
        .visibility = .Public,
        .default_value = null,
    });
    
    try fields.append(ast.StructField{
        .name = "age",
        .field_type = ast.Type{ .Primitive = .Normie },
        .visibility = .Public,
        .default_value = null,
    });
    
    var mock_struct = ast.StructStatement{
        .name = "Person",
        .fields = fields,
        .generic_params = .empty,
    };
    defer mock_struct.generic_params.deinit();
    
    // Register struct
    try reflection.registerStruct(&mock_struct);
    
    // Verify type info was generated
    const type_info = reflection.getTypeInfo("Person");
    try std.testing.expect(type_info != null);
    try std.testing.expect(type_info.?.kind == .Struct);
    try std.testing.expect(type_info.?.fields != null);
    try std.testing.expect(type_info.?.fields.?.len == 2);
}

test "field info generation" {
    var reflection = CompileTimeReflection.init(std.testing.allocator);
    defer reflection.deinit();
    
    // This test would require a complete struct registration
    // For now, just test the basic structure
    
    const fields_info = try reflection.generateFieldsInfo("NonExistentType");
    try std.testing.expect(fields_info == null);
}
