const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const StringContext = std.hash_map.StringContext;

const ast = @import("ast.zig");
const Expression = ast.Expression;
const Type = ast.Type;
const SourceLocation = ast.SourceLocation;

/// Attribute system for CURSED compiler
/// Supports compile-time directives that modify code generation behavior
/// Examples: @performance, @inline, @memory_layout, @optimize, @debug

/// Available attribute types that can be applied to declarations
pub const AttributeType = enum {
    // Performance attributes
    Performance,    // @performance(level: high|medium|low)
    Inline,         // @inline(always|never|hint)
    Optimize,       // @optimize(speed|size|debug)
    Unroll,         // @unroll(count: usize)
    Vectorize,      // @vectorize(enable|disable)
    
    // Memory layout attributes
    MemoryLayout,   // @memory_layout(packed|aligned|native)
    Align,          // @align(bytes: usize)
    Pack,           // @pack(enable|disable)
    Cache,          // @cache(hot|cold|prefetch)
    
    // Debugging attributes
    Debug,          // @debug(enable|disable|verbose)
    NoDebug,        // @no_debug
    ProfileGuided,  // @profile_guided(enable|disable)
    
    // Code generation attributes
    Export,         // @export(name: string)
    Import,         // @import(library: string, symbol: string)
    Extern,         // @extern(abi: c|cursed|system)
    LinkSection,    // @link_section(name: string)
    
    // Safety attributes
    Unsafe,         // @unsafe
    Bounds,         // @bounds(check|no_check)
    Overflow,       // @overflow(check|wrap|trap)
    
    // Concurrency attributes
    Atomic,         // @atomic(ordering: relaxed|acquire|release|seq_cst)
    ThreadSafe,     // @thread_safe
    Lock,           // @lock(type: mutex|spinlock|rw)
    
    // Testing attributes
    Test,           // @test
    Benchmark,      // @benchmark(iterations: usize)
    Fuzz,           // @fuzz(duration: usize)
    
    // Documentation attributes
    Doc,            // @doc(string)
    Deprecated,     // @deprecated(since: string, reason: string)
    Since,          // @since(version: string)
    
    // Custom attributes are no longer supported - all attributes must be predefined
};

/// Attribute parameter value types
pub const AttributeValue = union(enum) {
    String: []const u8,
    Integer: i64,
    Float: f64,
    Boolean: bool,
    Identifier: []const u8,
    Expression: *Expression,
    
    pub fn deinit(self: *AttributeValue, allocator: Allocator) void {
        switch (self.*) {
            .Expression => |expr| {
                expr.deinit();
                allocator.destroy(expr);
            },
            else => {},
        }
    }
};

/// Individual attribute parameter
pub const AttributeParameter = struct {
    name: []const u8,
    value: AttributeValue,
    
    pub fn deinit(self: *AttributeParameter, allocator: Allocator) void {
        self.value.deinit();
    }
};

/// Complete attribute declaration
pub const Attribute = struct {
    type: AttributeType,
    name: []const u8, // Raw attribute name for custom attributes
    parameters: ArrayList(AttributeParameter),
    location: SourceLocation,
    
    pub fn init(allocator: Allocator, attr_type: AttributeType, name: []const u8, location: SourceLocation) Attribute {
        return Attribute{
            .type = attr_type,
            .name = name,
            .parameters = .empty,
            .location = location,
        };
    }
    
    pub fn deinit(self: *Attribute, allocator: Allocator) void {
        for (self.parameters.items) |*param| {
            param.deinit();
        }
        self.parameters.deinit();
    }
    
    /// Add a parameter to this attribute
    pub fn addParameter(self: *Attribute, allocator: Allocator, name: []const u8, value: AttributeValue) !void {
                try self.parameters.append(AttributeParameter{
            .name = name,
            .value = value,
        });
    }
    
    /// Get parameter value by name
    pub fn getParameter(self: *const Attribute, name: []const u8) ?AttributeValue {
        for (self.parameters.items) |param| {
            if (std.mem.eql(u8, param.name, name)) {
                return param.value;
            }
        }
        return null;
    }
    
    /// Check if attribute has a specific parameter
    pub fn hasParameter(self: *const Attribute, name: []const u8) bool {
        return self.getParameter(name) != null;
    }
    
    /// Get string parameter value
    pub fn getStringParameter(self: *const Attribute, name: []const u8) ?[]const u8 {
        if (self.getParameter(name)) |value| {
            switch (value) {
                .String => |str| return str,
                .Identifier => |id| return id,
                else => return null,
            }
        }
        return null;
    }
    
    /// Get integer parameter value
    pub fn getIntegerParameter(self: *const Attribute, name: []const u8) ?i64 {
        if (self.getParameter(name)) |value| {
            switch (value) {
                .Integer => |int| return int,
                else => return null,
            }
        }
        return null;
    }
    
    /// Get boolean parameter value
    pub fn getBooleanParameter(self: *const Attribute, name: []const u8) ?bool {
        if (self.getParameter(name)) |value| {
            switch (value) {
                .Boolean => |bool_val| return bool_val,
                else => return null,
            }
        }
        return null;
    }
};

/// Collection of attributes that can be applied to any AST node
pub const AttributeList = struct {
    attributes: ArrayList(Attribute),
    
    pub fn init() AttributeList {
        return AttributeList{
            .attributes = .empty,
        };
    }
    
    pub fn deinit(self: *AttributeList, allocator: Allocator) void {
        for (self.attributes.items) |*attr| {
            attr.deinit();
        }
        self.attributes.deinit();
    }
    
    /// Add an attribute to the list
    pub fn addAttribute(self: *AttributeList, attr: Attribute) !void {
        try self.attributes.append(attr);
    }
    
    /// Find attribute by type
    pub fn findByType(self: *const AttributeList, attr_type: AttributeType) ?*const Attribute {
        for (self.attributes.items) |*attr| {
            if (attr.type == attr_type) {
                return attr;
            }
        }
        return null;
    }
    
    /// Find attribute by name (for custom attributes)
    pub fn findByName(self: *const AttributeList, name: []const u8) ?*const Attribute {
        for (self.attributes.items) |*attr| {
            if (std.mem.eql(u8, attr.name, name)) {
                return attr;
            }
        }
        return null;
    }
    
    /// Check if attribute list contains specific type
    pub fn hasAttribute(self: *const AttributeList, attr_type: AttributeType) bool {
        return self.findByType(attr_type) != null;
    }
    
    /// Get all attributes of a specific type
    pub fn getAttributesByType(self: *const AttributeList, allocator: Allocator, attr_type: AttributeType) !ArrayList(*const Attribute) {
        var result = .empty;
        for (self.attributes.items) |*attr| {
            if (attr.type == attr_type) {
                try result.append(attr);
            }
        }
        return result;
    }
    
    /// Validate that all attributes are properly formed and have valid parameters
    pub fn validate(self: *const AttributeList) !void {
        for (self.attributes.items) |*attr| {
            try validateAttribute(attr);
        }
    }
};

/// Attribute validation functions
fn validateAttribute(attr: *const Attribute) !void {
    switch (attr.type) {
        .Performance => {
            if (attr.hasParameter("level")) {
                const level = attr.getStringParameter("level") orelse return error.InvalidAttributeParameter;
                if (!std.mem.eql(u8, level, "high") and 
                    !std.mem.eql(u8, level, "medium") and 
                    !std.mem.eql(u8, level, "low")) {
                    return error.InvalidPerformanceLevel;
                }
            }
        },
        .Inline => {
            if (attr.hasParameter("hint")) {
                const hint = attr.getStringParameter("hint") orelse return error.InvalidAttributeParameter;
                if (!std.mem.eql(u8, hint, "always") and 
                    !std.mem.eql(u8, hint, "never") and 
                    !std.mem.eql(u8, hint, "hint")) {
                    return error.InvalidInlineHint;
                }
            }
        },
        .Align => {
            if (attr.hasParameter("bytes")) {
                const bytes = attr.getIntegerParameter("bytes") orelse return error.InvalidAttributeParameter;
                if (bytes <= 0 or (bytes & (bytes - 1)) != 0) {
                    return error.InvalidAlignment; // Must be power of 2
                }
            }
        },
        .Export => {
            if (!attr.hasParameter("name")) {
                return error.MissingExportName;
            }
        },
        .Import => {
            if (!attr.hasParameter("library") or !attr.hasParameter("symbol")) {
                return error.MissingImportParameters;
            }
        },
        else => {
            // Other attributes have more flexible validation
        }
    }
}

/// Utility functions for working with common attribute patterns

/// Create a performance attribute with specified level
pub fn createPerformanceAttribute(allocator: Allocator, level: []const u8, location: SourceLocation) !Attribute {
    var attr = Attribute.init(allocator, .Performance, "performance", location);
    try attr.addParameter(allocator, "level", AttributeValue{ .String = level });
    return attr;
}

/// Create an inline attribute with specified hint
pub fn createInlineAttribute(allocator: Allocator, hint: []const u8, location: SourceLocation) !Attribute {
    var attr = Attribute.init(allocator, .Inline, "inline", location);
    try attr.addParameter(allocator, "hint", AttributeValue{ .String = hint });
    return attr;
}

/// Create an alignment attribute with specified byte alignment
pub fn createAlignAttribute(allocator: Allocator, bytes: i64, location: SourceLocation) !Attribute {
    var attr = Attribute.init(allocator, .Align, "align", location);
    try attr.addParameter(allocator, "bytes", AttributeValue{ .Integer = bytes });
    return attr;
}

/// Create an export attribute with specified name
pub fn createExportAttribute(allocator: Allocator, name: []const u8, location: SourceLocation) !Attribute {
    var attr = Attribute.init(allocator, .Export, "export", location);
    try attr.addParameter(allocator, "name", AttributeValue{ .String = name });
    return attr;
}

/// Create a debug attribute
pub fn createDebugAttribute(allocator: Allocator, enable: bool, location: SourceLocation) !Attribute {
    var attr = Attribute.init(allocator, .Debug, "debug", location);
    try attr.addParameter(allocator, "enable", AttributeValue{ .Boolean = enable });
    return attr;
}

/// Parse attribute type from string name
pub fn parseAttributeType(name: []const u8) ?AttributeType {
    const attr_map = std.StaticStringMap(AttributeType).initComptime(.{
        .{ "performance", .Performance },
        .{ "inline", .Inline },
        .{ "optimize", .Optimize },
        .{ "unroll", .Unroll },
        .{ "vectorize", .Vectorize },
        .{ "memory_layout", .MemoryLayout },
        .{ "align", .Align },
        .{ "pack", .Pack },
        .{ "cache", .Cache },
        .{ "debug", .Debug },
        .{ "no_debug", .NoDebug },
        .{ "profile_guided", .ProfileGuided },
        .{ "export", .Export },
        .{ "import", .Import },
        .{ "extern", .Extern },
        .{ "link_section", .LinkSection },
        .{ "unsafe", .Unsafe },
        .{ "bounds", .Bounds },
        .{ "overflow", .Overflow },
        .{ "atomic", .Atomic },
        .{ "thread_safe", .ThreadSafe },
        .{ "lock", .Lock },
        .{ "test", .Test },
        .{ "benchmark", .Benchmark },
        .{ "fuzz", .Fuzz },
        .{ "doc", .Doc },
        .{ "deprecated", .Deprecated },
        .{ "since", .Since },
    });
    
    return attr_map.get(name);
}

/// Error types for attribute system
pub const AttributeError = error{
    InvalidAttributeParameter,
    InvalidPerformanceLevel,
    InvalidInlineHint,
    InvalidAlignment,
    MissingExportName,
    MissingImportParameters,
    DuplicateAttribute,
    UnknownAttribute,
    AttributeNotApplicable,
};

/// Testing utilities
pub fn createTestAttributes(allocator: Allocator) !AttributeList {
    var attrs = AttributeList.init(allocator);
    
    // Add a performance attribute
    const perf_attr = try createPerformanceAttribute(allocator, "high", SourceLocation.unknown());
    try attrs.addAttribute(perf_attr);
    
    // Add an inline attribute
    const inline_attr = try createInlineAttribute(allocator, "always", SourceLocation.unknown());
    try attrs.addAttribute(inline_attr);
    
    // Add a debug attribute
    const debug_attr = try createDebugAttribute(allocator, true, SourceLocation.unknown());
    try attrs.addAttribute(debug_attr);
    
    return attrs;
}

test "attribute system basic functionality" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test creating attributes
    var attrs = try createTestAttributes(allocator);
    defer attrs.deinit();
    
    // Test finding attributes
    const perf_attr = attrs.findByType(.Performance);
    try std.testing.expect(perf_attr != null);
    try std.testing.expect(std.mem.eql(u8, perf_attr.?.getStringParameter("level").?, "high"));
    
    // Test validation
    try attrs.validate();
    
    // Test has attribute
    try std.testing.expect(attrs.hasAttribute(.Performance));
    try std.testing.expect(attrs.hasAttribute(.Inline));
    try std.testing.expect(!attrs.hasAttribute(.Export));
}

test "attribute parameter access" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var attr = try createAlignAttribute(allocator, 16, SourceLocation.unknown());
    defer attr.deinit();
    
    const bytes = attr.getIntegerParameter("bytes");
    try std.testing.expect(bytes != null);
    try std.testing.expect(bytes.? == 16);
    
    const nonexistent = attr.getStringParameter("nonexistent");
    try std.testing.expect(nonexistent == null);
}
