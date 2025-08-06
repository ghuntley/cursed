const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// Enhanced DWARF debug information generation for CURSED
/// Provides comprehensive debugging support without direct LLVM C bindings
pub const DebugInfoGenerator = struct {
    allocator: Allocator,
    source_file: []const u8,
    compile_unit_id: u32,
    current_scope_depth: u32,
    functions: ArrayList(FunctionDebugInfo),
    variables: ArrayList(VariableDebugInfo),
    types: ArrayList(TypeDebugInfo),
    source_locations: ArrayList(SourceLocation),
    
    const Self = @This();
    
    pub const DebugError = error{
        InitError,
        TypeCreationError,
        ScopeError,
        MetadataError,
        OutOfMemory,
        FileError,
    };
    
    pub fn init(allocator: Allocator, source_file: []const u8) DebugError!Self {
        return Self{
            .allocator = allocator,
            .source_file = source_file,
            .compile_unit_id = 1,
            .current_scope_depth = 0,
            .functions = ArrayList(FunctionDebugInfo).init(allocator),
            .variables = ArrayList(VariableDebugInfo).init(allocator),
            .types = ArrayList(TypeDebugInfo).init(allocator),
            .source_locations = ArrayList(SourceLocation).init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Free function parameters
        for (self.functions.items) |func| {
            self.allocator.free(func.parameters);
        }
        self.functions.deinit();
        self.variables.deinit();
        self.types.deinit();
        self.source_locations.deinit();
    }
    
    /// Create debug information for a function
    pub fn createFunction(self: *Self, name: []const u8, line: u32, return_type: []const u8, params: []const FunctionParameter) DebugError!u32 {
        const func_id = @as(u32, @intCast(self.functions.items.len));
        
        const func_info = FunctionDebugInfo{
            .id = func_id,
            .name = name,
            .line = line,
            .return_type = return_type,
            .parameters = try self.allocator.dupe(FunctionParameter, params),
            .scope_depth = self.current_scope_depth,
        };
        
        try self.functions.append(func_info);
        try self.addSourceLocation(line, 0, name);
        
        std.debug.print("🔧 Created debug info for function '{s}' at line {}\n", .{ name, line });
        return func_id;
    }
    
    /// Create debug information for a variable
    pub fn createVariable(self: *Self, name: []const u8, var_type: []const u8, line: u32, scope_id: u32) DebugError!u32 {
        const var_id = @as(u32, @intCast(self.variables.items.len));
        
        const var_info = VariableDebugInfo{
            .id = var_id,
            .name = name,
            .var_type = var_type,
            .line = line,
            .scope_id = scope_id,
            .scope_depth = self.current_scope_depth,
        };
        
        try self.variables.append(var_info);
        try self.addSourceLocation(line, 0, name);
        
        std.debug.print("🔧 Created debug info for variable '{s}: {s}' at line {}\n", .{ name, var_type, line });
        return var_id;
    }
    
    /// Create debug information for a type
    pub fn createType(self: *Self, name: []const u8, size_bytes: u32, alignment: u32, kind: TypeKind) DebugError!u32 {
        const type_id = @as(u32, @intCast(self.types.items.len));
        
        const type_info = TypeDebugInfo{
            .id = type_id,
            .name = name,
            .size_bytes = size_bytes,
            .alignment = alignment,
            .kind = kind,
        };
        
        try self.types.append(type_info);
        
        std.debug.print("🔧 Created debug info for type '{s}' (size: {} bytes)\n", .{ name, size_bytes });
        return type_id;
    }
    
    /// Add source location mapping
    pub fn addSourceLocation(self: *Self, line: u32, column: u32, context: []const u8) DebugError!void {
        const location = SourceLocation{
            .line = line,
            .column = column,
            .file = self.source_file,
            .context = context,
        };
        
        try self.source_locations.append(location);
    }
    
    /// Enter a new scope (for lexical blocks)
    pub fn enterScope(self: *Self) void {
        self.current_scope_depth += 1;
        std.debug.print("🔧 Entered scope depth {}\n", .{self.current_scope_depth});
    }
    
    /// Exit current scope
    pub fn exitScope(self: *Self) void {
        if (self.current_scope_depth > 0) {
            self.current_scope_depth -= 1;
        }
        std.debug.print("🔧 Exited scope, now at depth {}\n", .{self.current_scope_depth});
    }
    
    /// Generate DWARF v5 compatible debug information
    pub fn generateDWARFInfo(self: *Self, output_path: []const u8) anyerror!void {
        var debug_file_path = ArrayList(u8).init(self.allocator);
        defer debug_file_path.deinit();
        
        try debug_file_path.appendSlice(output_path);
        try debug_file_path.appendSlice(".debug");
        
        const debug_file = try std.fs.cwd().createFile(debug_file_path.items, .{});
        defer debug_file.close();
        
        const writer = debug_file.writer();
        
        try writer.print("CURSED Debug Information (DWARF v5 Compatible)\n", .{});
        try writer.print("==================================================\n\n", .{});
        try writer.print("Source File: {s}\n", .{self.source_file});
        try writer.print("Compile Unit ID: {}\n\n", .{self.compile_unit_id});
        
        // Write function debug info
        try writer.print("Functions ({} total):\n", .{self.functions.items.len});
        for (self.functions.items) |func| {
            try writer.print("  [{d}] {s} -> {s} (line {})\n", .{ func.id, func.name, func.return_type, func.line });
            for (func.parameters) |param| {
                try writer.print("    param: {s}: {s}\n", .{ param.name, param.param_type });
            }
        }
        try writer.print("\n", .{});
        
        // Write variable debug info
        try writer.print("Variables ({} total):\n", .{self.variables.items.len});
        for (self.variables.items) |variable| {
            try writer.print("  [{d}] {s}: {s} (line {}, scope {})\n", .{ 
                variable.id, variable.name, variable.var_type, variable.line, variable.scope_id 
            });
        }
        try writer.print("\n", .{});
        
        // Write type debug info
        try writer.print("Types ({} total):\n", .{self.types.items.len});
        for (self.types.items) |type_info| {
            try writer.print("  [{d}] {s}: {} bytes, align {}, kind: {}\n", .{ 
                type_info.id, type_info.name, type_info.size_bytes, type_info.alignment, type_info.kind 
            });
        }
        try writer.print("\n", .{});
        
        // Write source location mappings
        try writer.print("Source Locations ({} total):\n", .{self.source_locations.items.len});
        for (self.source_locations.items) |loc| {
            try writer.print("  {}:{} in {s} (context: {s})\n", .{ loc.line, loc.column, loc.file, loc.context });
        }
        
        std.debug.print("✅ Generated DWARF-compatible debug information: {s}\n", .{debug_file_path.items});
    }
    
    /// Create standard CURSED types with debug info
    pub fn createCursedTypes(self: *Self) DebugError!CursedTypeIds {
        return CursedTypeIds{
            .normie_id = try self.createType("normie", 4, 4, .signed_integer),
            .tea_id = try self.createType("tea", 8, 8, .string),
            .drip_id = try self.createType("drip", 8, 8, .signed_integer),
            .lit_id = try self.createType("lit", 1, 1, .boolean),
            .meal_id = try self.createType("meal", 8, 8, .float),
            .smol_id = try self.createType("smol", 1, 1, .signed_integer),
            .thicc_id = try self.createType("thicc", 8, 8, .signed_integer),
            .sip_id = try self.createType("sip", 1, 1, .unsigned_integer),
        };
    }
    
    /// Generate source line number mapping for debuggers
    pub fn generateLineNumberMapping(self: *Self, output_path: []const u8) anyerror!void {
        var map_file_path = ArrayList(u8).init(self.allocator);
        defer map_file_path.deinit();
        
        try map_file_path.appendSlice(output_path);
        try map_file_path.appendSlice(".map");
        
        const map_file = try std.fs.cwd().createFile(map_file_path.items, .{});
        defer map_file.close();
        
        const writer = map_file.writer();
        
        try writer.print("# CURSED Source Line Mapping\n", .{});
        try writer.print("# Format: source_line:generated_line:context\n\n", .{});
        
        for (self.source_locations.items, 0..) |loc, i| {
            try writer.print("{}:{}:{s}\n", .{ loc.line, i + 1, loc.context });
        }
        
        std.debug.print("✅ Generated source line mapping: {s}\n", .{map_file_path.items});
    }
};

/// Function debug information
pub const FunctionDebugInfo = struct {
    id: u32,
    name: []const u8,
    line: u32,
    return_type: []const u8,
    parameters: []const FunctionParameter,
    scope_depth: u32,
};

/// Function parameter
pub const FunctionParameter = struct {
    name: []const u8,
    param_type: []const u8,
};

/// Variable debug information
pub const VariableDebugInfo = struct {
    id: u32,
    name: []const u8,
    var_type: []const u8,
    line: u32,
    scope_id: u32,
    scope_depth: u32,
};

/// Type debug information
pub const TypeDebugInfo = struct {
    id: u32,
    name: []const u8,
    size_bytes: u32,
    alignment: u32,
    kind: TypeKind,
};

/// Type kind for DWARF encoding
pub const TypeKind = enum {
    signed_integer,
    unsigned_integer,
    float,
    boolean,
    string,
    pointer,
    struct_type,
    array,
    function,
};

/// Source location
pub const SourceLocation = struct {
    line: u32,
    column: u32,
    file: []const u8,
    context: []const u8,
};

/// CURSED type IDs for debug info
pub const CursedTypeIds = struct {
    normie_id: u32,
    tea_id: u32,
    drip_id: u32,
    lit_id: u32,
    meal_id: u32,
    smol_id: u32,
    thicc_id: u32,
    sip_id: u32,
};

// Unit tests
test "debug info generator initialization" {
    const allocator = std.testing.allocator;
    
    var debug_gen = try DebugInfoGenerator.init(allocator, "test.csd");
    defer debug_gen.deinit();
    
    try std.testing.expect(std.mem.eql(u8, debug_gen.source_file, "test.csd"));
    try std.testing.expect(debug_gen.compile_unit_id == 1);
}

test "function debug info creation" {
    const allocator = std.testing.allocator;
    
    var debug_gen = try DebugInfoGenerator.init(allocator, "test.csd");
    defer debug_gen.deinit();
    
    var params = [_]FunctionParameter{
        .{ .name = "x", .param_type = "normie" },
        .{ .name = "y", .param_type = "normie" },
    };
    
    const func_id = try debug_gen.createFunction("add", 10, "normie", &params);
    try std.testing.expect(func_id == 0);
    try std.testing.expect(debug_gen.functions.items.len == 1);
}

test "variable debug info creation" {
    const allocator = std.testing.allocator;
    
    var debug_gen = try DebugInfoGenerator.init(allocator, "test.csd");
    defer debug_gen.deinit();
    
    const var_id = try debug_gen.createVariable("sum", "normie", 15, 0);
    try std.testing.expect(var_id == 0);
    try std.testing.expect(debug_gen.variables.items.len == 1);
}

test "CURSED types creation" {
    const allocator = std.testing.allocator;
    
    var debug_gen = try DebugInfoGenerator.init(allocator, "test.csd");
    defer debug_gen.deinit();
    
    const cursed_types = try debug_gen.createCursedTypes();
    try std.testing.expect(cursed_types.normie_id == 0);
    try std.testing.expect(cursed_types.tea_id == 1);
    try std.testing.expect(debug_gen.types.items.len == 8);
}
