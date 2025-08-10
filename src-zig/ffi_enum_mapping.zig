//! Enhanced FFI Enum Type Mapping System
//! Provides comprehensive C enum type mapping with proper size handling and marshaling

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const extern_abi = @import("extern_abi.zig");
const ast = @import("ast_advanced.zig");

/// C enum size variants
pub const CEnumSize = enum {
    Char,        // 8-bit
    Short,       // 16-bit
    Int,         // 32-bit (default)
    Long,        // 64-bit
    LongLong,    // 64-bit
    
    pub fn getBitWidth(self: CEnumSize) u8 {
        return switch (self) {
            .Char => 8,
            .Short => 16,
            .Int => 32,
            .Long => 64,
            .LongLong => 64,
        };
    }
    
    pub fn getByteSize(self: CEnumSize) u8 {
        return self.getBitWidth() / 8;
    }
    
    pub fn toCType(self: CEnumSize, is_signed: bool) []const u8 {
        return switch (self) {
            .Char => if (is_signed) "signed char" else "unsigned char",
            .Short => if (is_signed) "short" else "unsigned short", 
            .Int => if (is_signed) "int" else "unsigned int",
            .Long => if (is_signed) "long" else "unsigned long",
            .LongLong => if (is_signed) "long long" else "unsigned long long",
        };
    }
    
    pub fn toCursedType(self: CEnumSize, is_signed: bool) []const u8 {
        _ = is_signed; // CURSED doesn't distinguish signed/unsigned in type names
        return switch (self) {
            .Char => "smol",
            .Short => "smol", 
            .Int => "normie",
            .Long, .LongLong => "drip",
        };
    }
};

/// C enum definition with size and signedness information
pub const CEnumDefinition = struct {
    name: []const u8,
    underlying_size: CEnumSize,
    is_signed: bool,
    values: ArrayList(CEnumValue),
    allocator: Allocator,
    
    pub const CEnumValue = struct {
        name: []const u8,
        value: i64,
        explicit: bool, // Whether value was explicitly set
    };
    
    pub fn init(allocator: Allocator, name: []const u8) CEnumDefinition {
        return CEnumDefinition{
            .name = name,
            .underlying_size = .Int, // Default to int
            .is_signed = true, // Default to signed
            .values = ArrayList(CEnumValue).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *CEnumDefinition) void {
        self.values.deinit();
    }
    
    /// Add enum value with auto-increment
    pub fn addValue(self: *CEnumDefinition, name: []const u8, value: ?i64) !void {
        const actual_value = if (value) |v| v else blk: {
            if (self.values.items.len == 0) {
                break :blk 0;
            } else {
                break :blk self.values.items[self.values.items.len - 1].value + 1;
            }
        };
        
        try self.values.append(CEnumValue{
            .name = try self.allocator.dupe(u8, name),
            .value = actual_value,
            .explicit = value != null,
        });
    }
    
    /// Set underlying type from C attribute or type specification
    pub fn setUnderlyingType(self: *CEnumDefinition, type_spec: []const u8) void {
        if (std.mem.eql(u8, type_spec, "char")) {
            self.underlying_size = .Char;
            self.is_signed = true;
        } else if (std.mem.eql(u8, type_spec, "unsigned char")) {
            self.underlying_size = .Char;
            self.is_signed = false;
        } else if (std.mem.eql(u8, type_spec, "short")) {
            self.underlying_size = .Short;
            self.is_signed = true;
        } else if (std.mem.eql(u8, type_spec, "unsigned short")) {
            self.underlying_size = .Short;
            self.is_signed = false;
        } else if (std.mem.eql(u8, type_spec, "int")) {
            self.underlying_size = .Int;
            self.is_signed = true;
        } else if (std.mem.eql(u8, type_spec, "unsigned int") or std.mem.eql(u8, type_spec, "unsigned")) {
            self.underlying_size = .Int;
            self.is_signed = false;
        } else if (std.mem.eql(u8, type_spec, "long")) {
            self.underlying_size = .Long;
            self.is_signed = true;
        } else if (std.mem.eql(u8, type_spec, "unsigned long")) {
            self.underlying_size = .Long;
            self.is_signed = false;
        } else if (std.mem.eql(u8, type_spec, "long long")) {
            self.underlying_size = .LongLong;
            self.is_signed = true;
        } else if (std.mem.eql(u8, type_spec, "unsigned long long")) {
            self.underlying_size = .LongLong;
            self.is_signed = false;
        }
    }
    
    /// Validate enum value fits in underlying type
    pub fn validateValue(self: *const CEnumDefinition, value: i64) bool {
        const max_val: i64 = switch (self.underlying_size) {
            .Char => if (self.is_signed) 127 else 255,
            .Short => if (self.is_signed) 32767 else 65535,
            .Int => if (self.is_signed) 2147483647 else 4294967295,
            .Long, .LongLong => if (self.is_signed) std.math.maxInt(i63) else std.math.maxInt(u63),
        };
        
        const min_val: i64 = switch (self.underlying_size) {
            .Char => if (self.is_signed) -128 else 0,
            .Short => if (self.is_signed) -32768 else 0,
            .Int => if (self.is_signed) -2147483648 else 0,
            .Long, .LongLong => if (self.is_signed) std.math.minInt(i63) else 0,
        };
        
        return value >= min_val and value <= max_val;
    }
};

/// Enhanced FFI enum type mapper
pub const FFIEnumMapper = struct {
    allocator: Allocator,
    enum_definitions: HashMap([]const u8, CEnumDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    type_mappings: HashMap([]const u8, extern_abi.CABISignature.CABIType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator) FFIEnumMapper {
        return FFIEnumMapper{
            .allocator = allocator,
            .enum_definitions = HashMap([]const u8, CEnumDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_mappings = HashMap([]const u8, extern_abi.CABISignature.CABIType, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *FFIEnumMapper) void {
        var iterator = self.enum_definitions.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.enum_definitions.deinit();
        self.type_mappings.deinit();
    }
    
    /// Parse C enum declaration with size attributes
    pub fn parseCEnumDeclaration(self: *FFIEnumMapper, enum_text: []const u8) !*CEnumDefinition {
        var lines = std.mem.splitSequence(u8, enum_text, "\n");
        var enum_def: ?*CEnumDefinition = null;
        var in_enum_body = false;
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t");
            if (trimmed.len == 0 or trimmed[0] == '/') continue;
            
            if (std.mem.startsWith(u8, trimmed, "enum")) {
                // Parse enum declaration: enum Name : type { or enum Name {
                const enum_name = try self.parseEnumName(trimmed);
                var new_enum = CEnumDefinition.init(self.allocator, enum_name);
                
                // Check for type specification
                if (std.mem.indexOf(u8, trimmed, ":")) |colon_pos| {
                    const type_start = colon_pos + 1;
                    const type_end = std.mem.indexOf(u8, trimmed[type_start..], "{") orelse trimmed.len - type_start;
                    const type_spec = std.mem.trim(u8, trimmed[type_start..type_start + type_end], " \t");
                    new_enum.setUnderlyingType(type_spec);
                }
                
                // Check for attributes like __attribute__((packed))
                if (std.mem.indexOf(u8, trimmed, "__attribute__")) |_| {
                    try self.parseEnumAttributes(trimmed, &new_enum);
                }
                
                try self.enum_definitions.put(try self.allocator.dupe(u8, enum_name), new_enum);
                enum_def = self.enum_definitions.getPtr(enum_name);
                in_enum_body = std.mem.indexOf(u8, trimmed, "{") != null;
                
            } else if (in_enum_body and enum_def != null) {
                if (std.mem.indexOf(u8, trimmed, "}") != null) {
                    in_enum_body = false;
                } else {
                    // Parse enum value: NAME = value, or just NAME,
                    try self.parseEnumValue(trimmed, enum_def.?);
                }
            }
        }
        
        return enum_def orelse error.InvalidEnumDeclaration;
    }
    
    /// Extract enum name from declaration
    fn parseEnumName(self: *FFIEnumMapper, decl_line: []const u8) ![]const u8 {
        _ = self;
        
        // Find "enum" keyword
        const enum_pos = std.mem.indexOf(u8, decl_line, "enum") orelse return error.InvalidEnumDeclaration;
        const after_enum = decl_line[enum_pos + 4..];
        
        // Find name (skip whitespace)
        var name_start: usize = 0;
        while (name_start < after_enum.len and std.ascii.isWhitespace(after_enum[name_start])) {
            name_start += 1;
        }
        
        // Find end of name (whitespace, colon, or brace)
        var name_end = name_start;
        while (name_end < after_enum.len and 
               !std.ascii.isWhitespace(after_enum[name_end]) and 
               after_enum[name_end] != ':' and 
               after_enum[name_end] != '{') {
            name_end += 1;
        }
        
        if (name_start >= name_end) return error.InvalidEnumDeclaration;
        
        return after_enum[name_start..name_end];
    }
    
    /// Parse enum attributes like __attribute__((packed))
    fn parseEnumAttributes(self: *FFIEnumMapper, decl_line: []const u8, enum_def: *CEnumDefinition) !void {
        _ = self;
        
        if (std.mem.indexOf(u8, decl_line, "packed")) |_| {
            // Packed enums use minimal size
            enum_def.underlying_size = .Char;
        }
        
        // Parse other attributes as needed
        if (std.mem.indexOf(u8, decl_line, "aligned(")) |aligned_pos| {
            // Parse alignment specification if needed
            _ = aligned_pos;
        }
    }
    
    /// Parse individual enum value
    fn parseEnumValue(self: *FFIEnumMapper, value_line: []const u8, enum_def: *CEnumDefinition) !void {
        _ = self;
        
        const trimmed = std.mem.trim(u8, value_line, " \t,");
        if (trimmed.len == 0) return;
        
        if (std.mem.indexOf(u8, trimmed, "=")) |eq_pos| {
            // Explicit value: NAME = value
            const name = std.mem.trim(u8, trimmed[0..eq_pos], " \t");
            const value_str = std.mem.trim(u8, trimmed[eq_pos + 1..], " \t");
            
            const value = std.fmt.parseInt(i64, value_str, 0) catch |err| switch (err) {
                error.InvalidCharacter => blk: {
                    // Could be hex (0x...) or expression, try hex
                    if (std.mem.startsWith(u8, value_str, "0x")) {
                        break :blk std.fmt.parseInt(i64, value_str[2..], 16) catch 0;
                    } else {
                        break :blk 0; // Default if we can't parse
                    }
                },
                else => 0,
            };
            
            try enum_def.addValue(name, value);
        } else {
            // Auto-increment value: NAME
            try enum_def.addValue(trimmed, null);
        }
    }
    
    /// Generate CURSED enum from C enum
    pub fn generateCursedEnum(self: *FFIEnumMapper, c_enum: *const CEnumDefinition) ![]const u8 {
        var output = ArrayList(u8).init(self.allocator);
        defer output.deinit();
        
        try output.writer().print("// Generated from C enum {s}\n", .{c_enum.name});
        try output.writer().print("// Underlying type: {s} ({} bits)\n", .{
            c_enum.underlying_size.toCType(c_enum.is_signed),
            c_enum.underlying_size.getBitWidth()
        });
        try output.writer().print("enum {s} {{\n", .{c_enum.name});
        
        for (c_enum.values.items) |value| {
            try output.writer().print("    {s} = {},\n", .{ value.name, value.value });
        }
        
        try output.writer().print("}}\n\n", .{});
        
        // Generate type alias for marshaling
        const cursed_type = c_enum.underlying_size.toCursedType(c_enum.is_signed);
        try output.writer().print("// Type alias for C interop\n", .{});
        try output.writer().print("type {s}_Raw = {s}\n\n", .{ c_enum.name, cursed_type });
        
        // Generate conversion functions
        try output.writer().print("// Conversion functions\n", .{});
        try output.writer().print("slay {s}_to_raw(value {s}) {s}_Raw {{\n", .{ c_enum.name, c_enum.name, c_enum.name });
        try output.writer().print("    damn @intFromEnum(value)\n", .{});
        try output.writer().print("}}\n\n", .{});
        
        try output.writer().print("slay raw_to_{s}(value {s}_Raw) {s} {{\n", .{ c_enum.name, c_enum.name, c_enum.name });
        try output.writer().print("    damn @enumFromInt(value)\n", .{});
        try output.writer().print("}}\n\n", .{});
        
        return output.toOwnedSlice();
    }
    
    /// Generate C header for CURSED enum
    pub fn generateCHeader(self: *FFIEnumMapper, cursed_enum_name: []const u8) ![]const u8 {
        const enum_def = self.enum_definitions.get(cursed_enum_name) orelse return error.EnumNotFound;
        
        var output = ArrayList(u8).init(self.allocator);
        defer output.deinit();
        
        try output.writer().print("// Generated C header for CURSED enum {s}\n", .{cursed_enum_name});
        try output.writer().print("typedef enum {{\n", .{});
        
        for (enum_def.values.items) |value| {
            try output.writer().print("    {s}_{s} = {},\n", .{ cursed_enum_name, value.name, value.value });
        }
        
        try output.writer().print("}} {s}_t;\n\n", .{cursed_enum_name});
        
        // Generate conversion functions
        try output.writer().print("// Conversion functions\n", .{});
        try output.writer().print("{s} cursed_enum_to_{s}({s}_t value);\n", .{
            enum_def.underlying_size.toCType(enum_def.is_signed),
            cursed_enum_name,
            cursed_enum_name
        });
        try output.writer().print("{s}_t {s}_to_cursed_enum({s} value);\n", .{
            cursed_enum_name,
            cursed_enum_name,
            enum_def.underlying_size.toCType(enum_def.is_signed)
        });
        
        return output.toOwnedSlice();
    }
    
    /// Register enum type mapping for FFI
    pub fn registerEnumMapping(self: *FFIEnumMapper, enum_name: []const u8) !void {
        const enum_def = self.enum_definitions.get(enum_name) orelse return error.EnumNotFound;
        
        const cabi_type = switch (enum_def.underlying_size) {
            .Char => if (enum_def.is_signed) extern_abi.CABISignature.CABIType.Int8 else extern_abi.CABISignature.CABIType.UInt8,
            .Short => if (enum_def.is_signed) extern_abi.CABISignature.CABIType.Int16 else extern_abi.CABISignature.CABIType.UInt16,
            .Int => if (enum_def.is_signed) extern_abi.CABISignature.CABIType.Int32 else extern_abi.CABISignature.CABIType.UInt32,
            .Long, .LongLong => if (enum_def.is_signed) extern_abi.CABISignature.CABIType.Int64 else extern_abi.CABISignature.CABIType.UInt64,
        };
        
        try self.type_mappings.put(try self.allocator.dupe(u8, enum_name), cabi_type);
    }
    
    /// Marshall enum value between CURSED and C
    pub fn marshallToCValue(self: *FFIEnumMapper, enum_name: []const u8, cursed_value: i64) !i64 {
        const enum_def = self.enum_definitions.get(enum_name) orelse return error.EnumNotFound;
        
        // Validate value fits in target type
        if (!enum_def.validateValue(cursed_value)) {
            return error.ValueOutOfRange;
        }
        
        return cursed_value; // Direct mapping for now
    }
    
    /// Marshall enum value from C to CURSED
    pub fn marshallFromCValue(self: *FFIEnumMapper, enum_name: []const u8, c_value: i64) !i64 {
        const enum_def = self.enum_definitions.get(enum_name) orelse return error.EnumNotFound;
        
        // Validate C value is a valid enum value
        for (enum_def.values.items) |value| {
            if (value.value == c_value) {
                return c_value;
            }
        }
        
        return error.InvalidEnumValue;
    }
};

// Test cases
test "basic enum parsing" {
    var mapper = FFIEnumMapper.init(std.testing.allocator);
    defer mapper.deinit();
    
    const enum_text = 
        \\enum Color {
        \\    Red = 0,
        \\    Green = 1,
        \\    Blue = 2
        \\}
    ;
    
    const enum_def = try mapper.parseCEnumDeclaration(enum_text);
    
    try std.testing.expectEqualStrings("Color", enum_def.name);
    try std.testing.expect(enum_def.values.items.len == 3);
    try std.testing.expectEqualStrings("Red", enum_def.values.items[0].name);
    try std.testing.expect(enum_def.values.items[0].value == 0);
}

test "enum with size specification" {
    var mapper = FFIEnumMapper.init(std.testing.allocator);
    defer mapper.deinit();
    
    const enum_text = 
        \\enum Status : unsigned char {
        \\    OK,
        \\    ERROR = 255
        \\}
    ;
    
    const enum_def = try mapper.parseCEnumDeclaration(enum_text);
    
    try std.testing.expect(enum_def.underlying_size == .Char);
    try std.testing.expect(!enum_def.is_signed);
    try std.testing.expect(enum_def.values.items.len == 2);
    try std.testing.expect(enum_def.values.items[1].value == 255);
}

test "enum marshalling" {
    var mapper = FFIEnumMapper.init(std.testing.allocator);
    defer mapper.deinit();
    
    const enum_text = 
        \\enum Color {
        \\    Red = 0,
        \\    Green = 1,
        \\    Blue = 2
        \\}
    ;
    
    _ = try mapper.parseCEnumDeclaration(enum_text);
    try mapper.registerEnumMapping("Color");
    
    const c_value = try mapper.marshallToCValue("Color", 1);
    try std.testing.expect(c_value == 1);
    
    const cursed_value = try mapper.marshallFromCValue("Color", 1);
    try std.testing.expect(cursed_value == 1);
}
