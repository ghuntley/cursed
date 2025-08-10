const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const Lexer = lexer.Lexer;

const ast = @import("ast.zig");
const FunctionStatement = ast.FunctionStatement;
const StructStatement = ast.StructStatement;
const Statement = ast.Statement;

const attribute_system = @import("attribute_system.zig");
const AttributeList = attribute_system.AttributeList;

const attribute_parser = @import("attribute_parser.zig");
const AttributeParser = attribute_parser.AttributeParser;

const attribute_codegen = @import("attribute_codegen.zig");
const AttributeCodeGen = attribute_codegen.AttributeCodeGen;

const advanced_codegen = @import("advanced_codegen.zig");
const AdvancedCodeGen = advanced_codegen.AdvancedCodeGen;

/// Integration layer that connects attributes to the main compilation pipeline
/// Provides hooks for parsing attributes and applying them during code generation
pub const AttributeIntegration = struct {
    allocator: Allocator,
    attribute_codegen: ?*AttributeCodeGen = null,
    
    pub fn init(allocator: Allocator) AttributeIntegration {
        return AttributeIntegration{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *AttributeIntegration) void {
        if (self.attribute_codegen) |codegen| {
            codegen.deinit();
            self.allocator.destroy(codegen);
        }
    }
    
    /// Initialize attribute code generation with base codegen
    pub fn initializeCodeGen(self: *AttributeIntegration, base_codegen: *AdvancedCodeGen) !void {
        const attr_codegen = try self.allocator.create(AttributeCodeGen);
        attr_codegen.* = AttributeCodeGen.init(self.allocator, base_codegen);
        self.attribute_codegen = attr_codegen;
    }
    
    /// Parse attributes before a function declaration
    /// Called by the main parser when it encounters @ tokens before function declarations
    pub fn parseAttributesForFunction(self: *AttributeIntegration, lex: *Lexer, func: *FunctionStatement) !void {
        const attrs = try attribute_parser.parseAttributesForFunction(self.allocator, lex);
        if (attrs) |parsed_attrs| {
            func.attributes = parsed_attrs;
            
            // Validate attributes are applicable to functions
            try self.validateFunctionAttributes(&parsed_attrs);
        }
    }
    
    /// Parse attributes before a struct declaration
    /// Called by the main parser when it encounters @ tokens before struct declarations
    pub fn parseAttributesForStruct(self: *AttributeIntegration, lex: *Lexer, struct_stmt: *StructStatement) !void {
        const attrs = try attribute_parser.parseAttributesForStruct(self.allocator, lex);
        if (attrs) |parsed_attrs| {
            struct_stmt.attributes = parsed_attrs;
            
            // Validate attributes are applicable to structs
            try self.validateStructAttributes(&parsed_attrs);
        }
    }
    
    /// Process function attributes during code generation
    pub fn processFunctionCodeGen(self: *AttributeIntegration, func: *const FunctionStatement, llvm_function: anytype) !void {
        if (self.attribute_codegen) |codegen| {
            try attribute_codegen.preprocessFunctionAttributes(codegen, func);
            try attribute_codegen.postprocessFunctionAttributes(codegen, func, llvm_function);
        }
    }
    
    /// Process struct attributes during code generation
    pub fn processStructCodeGen(self: *AttributeIntegration, struct_stmt: *const StructStatement, llvm_struct_type: anytype) !void {
        if (self.attribute_codegen) |codegen| {
            try attribute_codegen.processStructLayoutAttributes(codegen, struct_stmt, llvm_struct_type);
        }
    }
    
    /// Finalize attribute-driven optimizations
    pub fn finalizeOptimizations(self: *AttributeIntegration, module: anytype, pass_manager: anytype) !void {
        if (self.attribute_codegen) |codegen| {
            try attribute_codegen.finalizeAttributeOptimizations(codegen, module, pass_manager);
        }
    }
    
    /// Validate that attributes are applicable to functions
    fn validateFunctionAttributes(self: *AttributeIntegration, attrs: *const AttributeList) !void {
        _ = self; // Suppress unused warning
        
        for (attrs.attributes.items) |*attr| {
            switch (attr.type) {
                // Function-applicable attributes
                .Performance, .Inline, .Optimize, .Unroll, .Vectorize,
                .Debug, .NoDebug, .ProfileGuided, .Export, .Import, .Extern,
                .Unsafe, .Bounds, .Overflow, .Atomic, .ThreadSafe, .Lock,
                .Test, .Benchmark, .Fuzz, .Doc, .Deprecated, .Since, .Custom => {
                    // These are valid for functions
                },
                
                // Struct-only attributes
                .MemoryLayout, .Align, .Pack, .Cache => {
                    return error.AttributeNotApplicableToFunction;
                },
                
                // Linkage attributes that need special validation
                .LinkSection => {
                    if (!attr.hasParameter("name")) {
                        return error.MissingLinkSectionName;
                    }
                },
            }
        }
        
        // Validate attribute combinations
        try self.validateAttributeCombinations(attrs);
    }
    
    /// Validate that attributes are applicable to structs
    fn validateStructAttributes(self: *AttributeIntegration, attrs: *const AttributeList) !void {
        _ = self; // Suppress unused warning
        
        for (attrs.attributes.items) |*attr| {
            switch (attr.type) {
                // Struct-applicable attributes
                .MemoryLayout, .Align, .Pack, .Cache, .Debug, .NoDebug,
                .Export, .Doc, .Deprecated, .Since, .Custom => {
                    // These are valid for structs
                },
                
                // Function-only attributes
                .Performance, .Inline, .Optimize, .Unroll, .Vectorize,
                .ProfileGuided, .Import, .Extern, .Unsafe, .Bounds, .Overflow,
                .Atomic, .ThreadSafe, .Lock, .Test, .Benchmark, .Fuzz, .LinkSection => {
                    return error.AttributeNotApplicableToStruct;
                },
            }
        }
    }
    
    /// Validate attribute combinations for conflicts
    fn validateAttributeCombinations(self: *AttributeIntegration, attrs: *const AttributeList) !void {
        _ = self; // Suppress unused warning
        
        // Check for conflicting inline attributes
        const has_inline = attrs.hasAttribute(.Inline);
        const has_never_inline = blk: {
            if (attrs.findByType(.Inline)) |inline_attr| {
                const hint = inline_attr.getStringParameter("hint") orelse "hint";
                break :blk std.mem.eql(u8, hint, "never");
            }
            break :blk false;
        };
        
        if (has_inline and has_never_inline) {
            return error.ConflictingInlineAttributes;
        }
        
        // Check for conflicting optimization attributes
        const has_optimize_speed = blk: {
            if (attrs.findByType(.Optimize)) |opt_attr| {
                const target = opt_attr.getStringParameter("target") orelse "speed";
                break :blk std.mem.eql(u8, target, "speed");
            }
            break :blk false;
        };
        
        const has_optimize_size = blk: {
            if (attrs.findByType(.Optimize)) |opt_attr| {
                const target = opt_attr.getStringParameter("target") orelse "speed";
                break :blk std.mem.eql(u8, target, "size");
            }
            break :blk false;
        };
        
        if (has_optimize_speed and has_optimize_size) {
            return error.ConflictingOptimizationTargets;
        }
        
        // Check for conflicting debug attributes
        if (attrs.hasAttribute(.Debug) and attrs.hasAttribute(.NoDebug)) {
            return error.ConflictingDebugAttributes;
        }
    }
};

/// Parser hooks that integrate with the main CURSED parser

/// Check if the current token position has attributes
pub fn hasAttributesAtCurrentPosition(lex: *Lexer) bool {
    // Look ahead for @ token
    const current = lex.peek();
    return current.type == .At;
}

/// Parse attributes at current position and attach to statement
pub fn parseAndAttachAttributes(integration: *AttributeIntegration, lex: *Lexer, stmt: *Statement) !void {
    switch (stmt.*) {
        .Function => |*func| {
            try integration.parseAttributesForFunction(lex, func);
        },
        .Struct => |*struct_stmt| {
            try integration.parseAttributesForStruct(lex, struct_stmt);
        },
        else => {
            // For other statement types, just skip attributes for now
            // Could be extended to support attributes on variables, etc.
            var attr_parser = AttributeParser.init(integration.allocator, lex);
            attr_parser.skipAttributes();
        }
    }
}

/// Code generation hooks that integrate with the main codegen

/// Pre-process attributes before code generation
pub fn preprocessStatement(integration: *AttributeIntegration, stmt: *const Statement) !void {
    if (integration.attribute_codegen == null) return;
    
    switch (stmt.*) {
        .Function => |*func| {
            if (func.attributes) |_| {
                try attribute_codegen.preprocessFunctionAttributes(integration.attribute_codegen.?, func);
            }
        },
        else => {
            // Other statement types don't need preprocessing yet
        }
    }
}

/// Post-process attributes after code generation
pub fn postprocessStatement(integration: *AttributeIntegration, stmt: *const Statement, llvm_value: anytype) !void {
    if (integration.attribute_codegen == null) return;
    
    switch (stmt.*) {
        .Function => |*func| {
            try integration.processFunctionCodeGen(func, llvm_value);
        },
        .Struct => |*struct_stmt| {
            try integration.processStructCodeGen(struct_stmt, llvm_value);
        },
        else => {
            // Other statement types don't need post-processing yet
        }
    }
}

/// Example CURSED code with attributes for testing
pub const example_attributed_code =
    \\@performance(level=high)
    \\@inline(hint=always)
    \\slay fast_multiply(a drip, b drip) drip {
    \\    damn a * b
    \\}
    \\
    \\@memory_layout(packed)
    \\@align(bytes=16)
    \\squad OptimizedStruct {
    \\    spill x drip
    \\    spill y drip
    \\    spill data []drip
    \\}
    \\
    \\@export(name="cursed_compute")
    \\@optimize(target=speed)
    \\slay compute_heavy(data []drip) drip {
    \\    sus result drip = 0
    \\    for value in data {
    \\        result = result + fast_multiply(value, 2)
    \\    }
    \\    damn result
    \\}
    \\
    \\@debug(enable=false)
    \\@unsafe
    \\slay low_level_operation(ptr *drip) drip {
    \\    damn ptr.*
    \\}
;

/// Error types for attribute integration
pub const AttributeIntegrationError = error{
    AttributeNotApplicableToFunction,
    AttributeNotApplicableToStruct,
    MissingLinkSectionName,
    ConflictingInlineAttributes,
    ConflictingOptimizationTargets,
    ConflictingDebugAttributes,
    InvalidAttributeForContext,
};

/// Testing utilities
pub fn createTestIntegration(allocator: Allocator) AttributeIntegration {
    return AttributeIntegration.init(allocator);
}

test "attribute integration basic functionality" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var integration = createTestIntegration(allocator);
    defer integration.deinit();
    
    // Test that integration can be created and destroyed without issues
    try std.testing.expect(integration.attribute_codegen == null);
}

test "attribute validation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var integration = createTestIntegration(allocator);
    defer integration.deinit();
    
    // Create a valid function attribute list
    var attrs = AttributeList.init(allocator);
    defer attrs.deinit(allocator);
    
    var perf_attr = try attribute_system.createPerformanceAttribute(allocator, "high", ast.SourceLocation.unknown());
    try attrs.addAttribute(perf_attr);
    
    // This should pass validation
    try integration.validateFunctionAttributes(&attrs);
    
    // Test struct attributes - performance should fail for structs
    const result = integration.validateStructAttributes(&attrs);
    try std.testing.expectError(AttributeIntegrationError.AttributeNotApplicableToStruct, result);
}
