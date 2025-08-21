//! P26: Enhanced Exhaustive Pattern Checking for Enums
//! Ensures all enum variants are covered in pattern matches

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");
const pattern_matching = @import("pattern_matching.zig");

/// Enhanced enum exhaustiveness checker
pub const EnumExhaustivenessChecker = struct {
    allocator: Allocator,
    enum_registry: *pattern_matching.EnumVariantRegistry,
    coverage_cache: HashMap([]const u8, EnumCoverage, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const EnumCoverage = struct {
        enum_name: []const u8,
        covered_variants: std.bit_set.DynamicBitSet,
        has_wildcard: bool = false,
        pattern_count: usize = 0,
        
        pub fn init(allocator: Allocator, enum_name: []const u8, variant_count: usize) !EnumCoverage {
            return EnumCoverage{
                .enum_name = enum_name,
                .covered_variants = try std.bit_set.DynamicBitSet.initEmpty(allocator, variant_count),
            };
        }
        
        pub fn deinit(self: *EnumCoverage) void {
            self.covered_variants.deinit(allocator);
        }
    };
    
    pub fn init(allocator: Allocator, enum_registry: *pattern_matching.EnumVariantRegistry) EnumExhaustivenessChecker {
        return EnumExhaustivenessChecker{
            .allocator = allocator,
            .enum_registry = enum_registry,
            .coverage_cache = HashMap([]const u8, EnumCoverage, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *EnumExhaustivenessChecker) void {
        var iterator = self.coverage_cache.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.coverage_cache.deinit(allocator);
    }
    
    /// Check exhaustiveness for enum match patterns
    pub fn checkEnumExhaustiveness(self: *EnumExhaustivenessChecker, enum_name: []const u8, patterns: []const ast.Pattern) !ExhaustivenessResult {
        // Get enum variants from registry
        const variants = self.enum_registry.getEnumVariants(enum_name) orelse {
            return error.EnumNotFound;
        };
        
        // Initialize coverage tracking
        var coverage = try EnumCoverage.init(self.allocator, enum_name, variants.items.len);
        defer coverage.deinit(allocator);
        
        // Analyze each pattern
        for (patterns) |pattern| {
            try self.analyzePattern(pattern, enum_name, &coverage);
        }
        
        // Check if all variants are covered
        const is_exhaustive = coverage.has_wildcard or coverage.covered_variants.count() == variants.items.len;
        
        var missing_variants = .empty;
        
        if (!is_exhaustive) {
            for (variants.items, 0..) |variant_name, i| {
                if (!coverage.covered_variants.isSet(i)) {
                    try missing_variants.append(allocator, variant_name);
                }
            }
        }
        
        return ExhaustivenessResult{
            .is_exhaustive = is_exhaustive,
            .covered_count = coverage.covered_variants.count(),
            .total_count = variants.items.len,
            .missing_variants = missing_variants,
            .has_wildcard = coverage.has_wildcard,
        };
    }
    
    /// Analyze individual pattern for coverage
    fn analyzePattern(self: *EnumExhaustivenessChecker, pattern: ast.Pattern, enum_name: []const u8, coverage: *EnumCoverage) !void {
        switch (pattern) {
            .Enum => |enum_pattern| {
                // Check if this pattern matches our target enum
                if (std.mem.eql(u8, enum_pattern.enum_name, enum_name)) {
                    // Get variant index and mark as covered
                    if (self.enum_registry.getVariantIndex(enum_name, enum_pattern.variant_name)) |variant_index| {
                        coverage.covered_variants.set(variant_index);
                        coverage.pattern_count += 1;
                    }
                }
            },
            .Wildcard => {
                coverage.has_wildcard = true;
            },
            .Or => |or_pattern| {
                // Recursively analyze OR alternatives
                for (or_pattern.patterns) |sub_pattern| {
                    try self.analyzePattern(sub_pattern, enum_name, coverage);
                }
            },
            .Guard => |guard| {
                // Guards don't provide complete coverage - analyze base pattern
                try self.analyzePattern(guard.pattern.*, enum_name, coverage);
            },
            else => {
                // Other patterns don't contribute to enum coverage
            },
        }
    }
    
    /// Generate exhaustiveness error message
    pub fn generateExhaustivenessError(self: *EnumExhaustivenessChecker, enum_name: []const u8, result: ExhaustivenessResult) ![]const u8 {
        var error_msg = .empty;
        defer error_msg.deinit(allocator);
        
        try error_msg.writer().print("Non-exhaustive pattern match for enum '{s}'\n", .{enum_name});
        try error_msg.writer().print("Covered {d}/{d} variants\n", .{ result.covered_count, result.total_count });
        
        if (result.missing_variants.items.len > 0) {
            try error_msg.writer().print("Missing patterns:\n");
            for (result.missing_variants.items) |variant| {
                try error_msg.writer().print("  - {s}::{s}\n", .{ enum_name, variant });
            }
        }
        
        if (!result.has_wildcard) {
            try error_msg.writer().print("Consider adding a wildcard pattern (_) to handle remaining cases\n");
        }
        
        return error_msg.toOwnedSlice(allocator);
    }
    
    /// Generate fix suggestions
    pub fn generateFixSuggestions(self: *EnumExhaustivenessChecker, enum_name: []const u8, result: ExhaustivenessResult) ![]const u8 {
        var suggestions = .empty;
        defer suggestions.deinit(allocator);
        
        try suggestions.writer().print("To fix exhaustiveness:\n");
        
        if (result.missing_variants.items.len <= 3) {
            // Suggest specific patterns for small number of missing variants
            try suggestions.writer().print("Add missing patterns:\n");
            for (result.missing_variants.items) |variant| {
                try suggestions.writer().print("  when {s}::{s} -> {{ /* handle this case */ }}\n", .{ enum_name, variant });
            }
        } else {
            // Suggest wildcard for many missing variants
            try suggestions.writer().print("Add wildcard pattern:\n");
            try suggestions.writer().print("  when _ -> {{ /* handle remaining cases */ }}\n");
        }
        
        return suggestions.toOwnedSlice(allocator);
    }
    
    const ExhaustivenessResult = struct {
        is_exhaustive: bool,
        covered_count: usize,
        total_count: usize,
        missing_variants: ArrayList([]const u8),
        has_wildcard: bool,
    };
};

/// Enhanced pattern match compiler with exhaustiveness checking
pub const EnhancedPatternCompiler = struct {
    base_compiler: *pattern_matching.PatternCompiler,
    exhaustiveness_checker: *EnumExhaustivenessChecker,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, base_compiler: *pattern_matching.PatternCompiler, exhaustiveness_checker: *EnumExhaustivenessChecker) EnhancedPatternCompiler {
        return EnhancedPatternCompiler{
            .base_compiler = base_compiler,
            .exhaustiveness_checker = exhaustiveness_checker,
            .allocator = allocator,
        };
    }
    
    /// Compile match expression with exhaustiveness checking
    pub fn compileMatchWithExhaustiveness(self: *EnhancedPatternCompiler, match_expr: ast.MatchExpression) ![]const u8 {
        // Extract enum patterns for exhaustiveness checking
        var enum_patterns = HashMap([]const u8, ArrayList(ast.Pattern), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer {
            var iterator = enum_patterns.iterator();
            while (iterator.next()) |entry| {
                entry.value_ptr.deinit(allocator);
            }
            enum_patterns.deinit(allocator);
        }
        
        // Group patterns by enum type
        for (match_expr.cases.items) |case| {
            try self.collectEnumPatterns(case.pattern, &enum_patterns);
        }
        
        // Check exhaustiveness for each enum
        var enum_iterator = enum_patterns.iterator();
        while (enum_iterator.next()) |entry| {
            const enum_name = entry.key_ptr.*;
            const patterns = entry.value_ptr.items;
            
            const exhaustiveness_result = self.exhaustiveness_checker.checkEnumExhaustiveness(enum_name, patterns) catch continue;
            defer exhaustiveness_result.missing_variants.deinit(allocator);
            
            if (!exhaustiveness_result.is_exhaustive) {
                // Generate warning with fix suggestions
                const error_msg = try self.exhaustiveness_checker.generateExhaustivenessError(enum_name, exhaustiveness_result);
                defer self.allocator.free(error_msg);
                
                const suggestions = try self.exhaustiveness_checker.generateFixSuggestions(enum_name, exhaustiveness_result);
                defer self.allocator.free(suggestions);
                
                std.log.warn("Pattern exhaustiveness warning:\n{s}\n{s}", .{ error_msg, suggestions });
            }
        }
        
        // Delegate to base compiler for code generation
        return self.base_compiler.compileMatchExpression(match_expr);
    }
    
    /// Collect enum patterns from pattern tree
    fn collectEnumPatterns(self: *EnhancedPatternCompiler, pattern: ast.Pattern, enum_patterns: *HashMap([]const u8, ArrayList(ast.Pattern), std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !void {
        switch (pattern) {
            .Enum => |enum_pattern| {
                var patterns_list = enum_patterns.get(enum_pattern.enum_name) orelse blk: {
                    var new_list = .empty;
                    try enum_patterns.put(enum_pattern.enum_name, new_list);
                    break :blk enum_patterns.get(enum_pattern.enum_name).?;
                };
                try patterns_list.append(allocator, pattern);
            },
            .Or => |or_pattern| {
                for (or_pattern.patterns) |sub_pattern| {
                    try self.collectEnumPatterns(sub_pattern, enum_patterns);
                }
            },
            .Guard => |guard| {
                try self.collectEnumPatterns(guard.pattern.*, enum_patterns);
            },
            else => {},
        }
    }
};

// Test cases for exhaustiveness checking
test "enum exhaustiveness checking basic functionality" {
    var registry = pattern_matching.EnumVariantRegistry.init(std.testing.allocator);
    defer registry.deinit(allocator);
    
    // Register Color enum with variants
    const color_variants = [_][]const u8{ "Red", "Green", "Blue", "Custom" };
    try registry.registerEnum("Color", &color_variants);
    
    var checker = EnumExhaustivenessChecker.init(std.testing.allocator, &registry);
    defer checker.deinit(allocator);
    
    // Test non-exhaustive pattern (missing Custom variant)
    const patterns = [_]ast.Pattern{
        ast.Pattern{ .Enum = ast.Pattern.EnumPattern{ .enum_name = "Color", .variant_name = "Red" } },
        ast.Pattern{ .Enum = ast.Pattern.EnumPattern{ .enum_name = "Color", .variant_name = "Green" } },
        ast.Pattern{ .Enum = ast.Pattern.EnumPattern{ .enum_name = "Color", .variant_name = "Blue" } },
    };
    
    const result = try checker.checkEnumExhaustiveness("Color", &patterns);
    defer result.missing_variants.deinit(allocator);
    
    try std.testing.expect(!result.is_exhaustive);
    try std.testing.expect(result.covered_count == 3);
    try std.testing.expect(result.total_count == 4);
    try std.testing.expect(result.missing_variants.items.len == 1);
    try std.testing.expectEqualStrings("Custom", result.missing_variants.items[0]);
}

test "enum exhaustiveness with wildcard pattern" {
    var registry = pattern_matching.EnumVariantRegistry.init(std.testing.allocator);
    defer registry.deinit(allocator);
    
    // Register Status enum
    const status_variants = [_][]const u8{ "Success", "Error", "Pending", "Cancelled" };
    try registry.registerEnum("Status", &status_variants);
    
    var checker = EnumExhaustivenessChecker.init(std.testing.allocator, &registry);
    defer checker.deinit(allocator);
    
    // Test with wildcard pattern (should be exhaustive)
    const patterns = [_]ast.Pattern{
        ast.Pattern{ .Enum = ast.Pattern.EnumPattern{ .enum_name = "Status", .variant_name = "Success" } },
        ast.Pattern{ .Wildcard = {} },
    };
    
    const result = try checker.checkEnumExhaustiveness("Status", &patterns);
    defer result.missing_variants.deinit(allocator);
    
    try std.testing.expect(result.is_exhaustive);
    try std.testing.expect(result.has_wildcard);
    try std.testing.expect(result.missing_variants.items.len == 0);
}
