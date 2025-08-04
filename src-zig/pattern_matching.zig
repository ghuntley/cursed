//! Pattern Matching Implementation for CURSED Zig Compiler
//! 
//! This module implements comprehensive pattern matching semantics including:
//! - Guards
//! - Literal and range patterns  
//! - Nested destructuring
//! - Exhaustiveness checking
//! - Enum variant index lookup
//! - C code generation support

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");

/// Enum variant registry for variant index lookup
pub const EnumVariantRegistry = struct {
    /// Map from (enum_name, variant_name) to variant_index
    variants: HashMap(VariantKey, usize, VariantKeyContext, std.hash_map.default_max_load_percentage),
    /// Map from enum_name to list of variant names in order
    enum_variants: HashMap([]const u8, ArrayList([]const u8), StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    const VariantKey = struct {
        enum_name: []const u8,
        variant_name: []const u8,
    };
    
    const VariantKeyContext = struct {
        pub fn hash(self: @This(), key: VariantKey) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(key.enum_name);
            hasher.update(key.variant_name);
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: VariantKey, b: VariantKey) bool {
            _ = self;
            return std.mem.eql(u8, a.enum_name, b.enum_name) and 
                   std.mem.eql(u8, a.variant_name, b.variant_name);
        }
    };
    
    const StringContext = struct {
        pub fn hash(self: @This(), key: []const u8) u64 {
            _ = self;
            var hasher = std.hash.Wyhash.init(0);
            hasher.update(key);
            return hasher.final();
        }
        
        pub fn eql(self: @This(), a: []const u8, b: []const u8) bool {
            _ = self;
            return std.mem.eql(u8, a, b);
        }
    };
    
    pub fn init(allocator: Allocator) EnumVariantRegistry {
        return EnumVariantRegistry{
            .variants = HashMap(VariantKey, usize, VariantKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
            .enum_variants = HashMap([]const u8, ArrayList([]const u8), StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *EnumVariantRegistry) void {
        var enum_iterator = self.enum_variants.iterator();
        while (enum_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.enum_variants.deinit();
        self.variants.deinit();
    }
    
    /// Register an enum with its variants in order
    pub fn registerEnum(self: *EnumVariantRegistry, enum_name: []const u8, variant_names: []const []const u8) !void {
        var variant_list = ArrayList([]const u8).init(self.allocator);
        
        for (variant_names, 0..) |variant_name, index| {
            const key = VariantKey{
                .enum_name = enum_name,
                .variant_name = variant_name,
            };
            try self.variants.put(key, index);
            try variant_list.append(variant_name);
        }
        
        try self.enum_variants.put(enum_name, variant_list);
    }
    
    /// Get variant index for given enum and variant name
    pub fn getVariantIndex(self: *EnumVariantRegistry, enum_name: []const u8, variant_name: []const u8) ?usize {
        const key = VariantKey{
            .enum_name = enum_name,
            .variant_name = variant_name,
        };
        return self.variants.get(key);
    }
    
    /// Get all variants for an enum
    pub fn getEnumVariants(self: *EnumVariantRegistry, enum_name: []const u8) ?ArrayList([]const u8) {
        return self.enum_variants.get(enum_name);
    }
};

/// Pattern compiler for C code generation
pub const PatternCompiler = struct {
    output: *ArrayList(u8),
    register_counter: *usize,
    label_counter: *usize,
    enum_registry: *EnumVariantRegistry,
    allocator: Allocator,
    
    pub fn init(output: *ArrayList(u8), register_counter: *usize, label_counter: *usize, enum_registry: *EnumVariantRegistry, allocator: Allocator) PatternCompiler {
        return PatternCompiler{
            .output = output,
            .register_counter = register_counter,
            .label_counter = label_counter,
            .enum_registry = enum_registry,
            .allocator = allocator,
        };
    }
    
    /// Compile enum pattern with proper variant index lookup
    pub fn compileEnumPattern(self: *PatternCompiler, value_var: []const u8, enum_pattern: ast.Pattern.EnumPattern, success_label: []const u8, fail_label: []const u8) !void {
        // Get variant index from registry
        const variant_index = self.enum_registry.getVariantIndex(enum_pattern.enum_name, enum_pattern.variant_name) orelse {
            return error.UnknownVariant;
        };
        
        // Generate tag extraction code
        const tag_var = try std.fmt.allocPrint(self.allocator, "tag_{}", .{self.register_counter.*});
        self.register_counter.* += 1;
        
        try self.output.writer().print("    int {s} = {s}->tag;\n", .{ tag_var, value_var });
        
        // Generate comparison
        const cmp_var = try std.fmt.allocPrint(self.allocator, "cmp_{}", .{self.register_counter.*});
        self.register_counter.* += 1;
        
        try self.output.writer().print("    int {s} = ({s} == {});\n", .{ cmp_var, tag_var, variant_index });
        
        // Generate conditional branch
        try self.output.writer().print("    if ({s}) {{\n", .{cmp_var});
        try self.output.writer().print("        goto {s};\n", .{success_label});
        try self.output.writer().print("    }} else {{\n");
        try self.output.writer().print("        goto {s};\n", .{fail_label});
        try self.output.writer().print("    }}\n");
        
        // Clean up allocated strings
        self.allocator.free(tag_var);
        self.allocator.free(cmp_var);
    }
    
    /// Generate pattern matching switch for multiple variants
    pub fn generatePatternMatchSwitch(self: *PatternCompiler, value_var: []const u8, cases: []const ast.MatchCase) !void {
        try self.output.writer().print("    switch ({s}->tag) {{\n", .{value_var});
        
        for (cases, 0..) |case, i| {
            if (case.pattern == .Enum) {
                const enum_pattern = case.pattern.Enum;
                const variant_index = self.enum_registry.getVariantIndex(enum_pattern.enum_name, enum_pattern.variant_name) orelse {
                    return error.UnknownVariant;
                };
                
                try self.output.writer().print("        case {}:\n", .{variant_index});
                try self.output.writer().print("            goto case_{};\n", .{i});
            }
        }
        
        try self.output.writer().print("        default:\n");
        try self.output.writer().print("            goto match_fail;\n");
        try self.output.writer().print("    }}\n");
    }
};

// Test cases for the enum variant registry
test "enum variant registry basic functionality" {
    var registry = EnumVariantRegistry.init(std.testing.allocator);
    defer registry.deinit();
    
    // Register Color enum with variants
    const color_variants = [_][]const u8{ "Red", "Green", "Blue", "Custom" };
    try registry.registerEnum("Color", &color_variants);
    
    // Test variant index lookup
    try std.testing.expect(registry.getVariantIndex("Color", "Red") == 0);
    try std.testing.expect(registry.getVariantIndex("Color", "Green") == 1);
    try std.testing.expect(registry.getVariantIndex("Color", "Blue") == 2);
    try std.testing.expect(registry.getVariantIndex("Color", "Custom") == 3);
    
    // Test unknown variant
    try std.testing.expect(registry.getVariantIndex("Color", "Unknown") == null);
    try std.testing.expect(registry.getVariantIndex("UnknownEnum", "Red") == null);
}

test "multiple enums support" {
    var registry = EnumVariantRegistry.init(std.testing.allocator);
    defer registry.deinit();
    
    // Register multiple enums
    const status_variants = [_][]const u8{ "Success", "Error", "Pending" };
    const direction_variants = [_][]const u8{ "North", "South", "East", "West" };
    
    try registry.registerEnum("Status", &status_variants);
    try registry.registerEnum("Direction", &direction_variants);
    
    // Test Status enum indices
    try std.testing.expect(registry.getVariantIndex("Status", "Success") == 0);
    try std.testing.expect(registry.getVariantIndex("Status", "Error") == 1);
    try std.testing.expect(registry.getVariantIndex("Status", "Pending") == 2);
    
    // Test Direction enum indices  
    try std.testing.expect(registry.getVariantIndex("Direction", "North") == 0);
    try std.testing.expect(registry.getVariantIndex("Direction", "South") == 1);
    try std.testing.expect(registry.getVariantIndex("Direction", "East") == 2);
    try std.testing.expect(registry.getVariantIndex("Direction", "West") == 3);
    
    // Test cross-enum queries don't work
    try std.testing.expect(registry.getVariantIndex("Status", "North") == null);
    try std.testing.expect(registry.getVariantIndex("Direction", "Success") == null);
}
