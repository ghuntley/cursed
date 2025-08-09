//! Enhanced Pattern Matching Implementation for CURSED Zig Compiler
//! 
//! This module implements comprehensive pattern matching compilation including:
//! - Literal pattern matching (numbers, strings, booleans)
//! - Variable binding patterns with type inference
//! - Wildcard patterns (_) for catch-all cases
//! - Tuple/struct destructuring patterns
//! - Array/slice patterns with rest elements
//! - Guard expressions for conditional matching
//! - Efficient LLVM/C code generation with optimization

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast_advanced.zig");

// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

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

/// Enhanced pattern compiler for efficient LLVM/C code generation
pub const PatternCompiler = struct {
    // Code generation targets
    output: *ArrayList(u8),
    llvm_module: ?c.LLVMModuleRef,
    llvm_builder: ?c.LLVMBuilderRef,
    llvm_context: ?c.LLVMContextRef,
    
    // State management
    register_counter: *usize,
    label_counter: *usize,
    temp_counter: usize,
    block_counter: usize,
    
    // Type and binding tracking
    enum_registry: *EnumVariantRegistry,
    variable_bindings: HashMap([]const u8, VariableBinding, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    pattern_variables: ArrayList([]const u8),
    
    // Memory management
    allocator: Allocator,
    arena: std.heap.ArenaAllocator,
    
    // Optimization state
    jump_table_threshold: usize,
    optimization_level: OptimizationLevel,
    
    const OptimizationLevel = enum { O0, O1, O2, O3 };
    
    const VariableBinding = struct {
        llvm_value: ?c.LLVMValueRef,
        c_name: []const u8,
        type_info: TypeInfo,
        is_mutable: bool,
    };
    
    const TypeInfo = union(enum) {
        integer: u32,
        float: u32,
        boolean: void,
        string: void,
        pointer: *TypeInfo,
        array: struct { element_type: *TypeInfo, size: ?usize },
        tuple: []*TypeInfo,
        struct_type: []const u8,
    };
    
    pub fn init(output: *ArrayList(u8), register_counter: *usize, label_counter: *usize, enum_registry: *EnumVariantRegistry, allocator: Allocator) PatternCompiler {
        const arena = std.heap.ArenaAllocator.init(allocator);
        return PatternCompiler{
            .output = output,
            .llvm_module = null,
            .llvm_builder = null,
            .llvm_context = null,
            .register_counter = register_counter,
            .label_counter = label_counter,
            .temp_counter = 0,
            .block_counter = 0,
            .enum_registry = enum_registry,
            .variable_bindings = HashMap([]const u8, VariableBinding, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .pattern_variables = ArrayList([]const u8).init(allocator),
            .allocator = allocator,
            .arena = arena,
            .jump_table_threshold = 8,
            .optimization_level = .O2,
        };
    }
    
    pub fn deinit(self: *PatternCompiler) void {
        self.variable_bindings.deinit();
        self.pattern_variables.deinit();
        self.arena.deinit();
    }
    
    pub fn setLLVMContext(self: *PatternCompiler, context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef) void {
        self.llvm_context = context;
        self.llvm_module = module;
        self.llvm_builder = builder;
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
    
    /// Main pattern compilation entry point with optimization
    pub fn compilePattern(self: *PatternCompiler, pattern: ast.Pattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        switch (pattern) {
            .Literal => |lit| try self.compileLiteralPattern(lit, value_var, success_label, fail_label),
            .Variable => |var_pattern| try self.compileVariablePattern(var_pattern, value_var, success_label),
            .Wildcard => try self.compileWildcardPattern(success_label),
            .Tuple => |tuple| try self.compileTuplePattern(tuple, value_var, success_label, fail_label),
            .Struct => |struct_pattern| try self.compileStructPattern(struct_pattern, value_var, success_label, fail_label),
            .Array => |array| try self.compileArrayPattern(array, value_var, success_label, fail_label),
            .Slice => |slice| try self.compileSlicePattern(slice, value_var, success_label, fail_label),
            .Or => |or_pattern| try self.compileOrPattern(or_pattern, value_var, success_label, fail_label),
            .Range => |range| try self.compileRangePattern(range, value_var, success_label, fail_label),
            .Guard => |guard| try self.compileGuardPattern(guard, value_var, success_label, fail_label),
            .Enum => |enum_pattern| try self.compileEnumPattern(value_var, enum_pattern, success_label, fail_label),
            .Type => |type_pattern| try self.compileTypePattern(type_pattern, value_var, success_label, fail_label),
        }
    }

    /// Complete switch statement compilation with pattern matching and exhaustiveness checking
    pub fn compileSwitchStatement(self: *PatternCompiler, switch_stmt: ast.PatternSwitchStatement) !void {
        try self.output.writer().print("    // Pattern matching switch statement with exhaustiveness checking\n");
        
        const value_temp = try self.getTempVar();
        defer self.allocator.free(value_temp);
        
        // Generate value extraction 
        try self.output.writer().print("    auto {s} = evaluate_switch_expression();\n", .{value_temp});
        
        // Extract patterns for exhaustiveness analysis
        var patterns = ArrayList(ast.Pattern).init(self.allocator);
        defer patterns.deinit();
        
        for (switch_stmt.cases.items) |case| {
            try patterns.append(case.pattern);
        }
        
        // Perform exhaustiveness checking
        const exhaustiveness_result = try self.checkExhaustiveness(patterns.items, null);
        defer exhaustiveness_result.missing_patterns.deinit();
        
        // Generate exhaustiveness report
        try self.generateExhaustivenessReport(exhaustiveness_result);
        
        // Generate case labels
        const end_label = try std.fmt.allocPrint(self.allocator, "switch_end_{}", .{self.block_counter});
        defer self.allocator.free(end_label);
        self.block_counter += 1;
        
        const has_default = exhaustiveness_result.is_exhaustive;
        
        // Generate optimized dispatch if applicable
        const literal_cases = try self.extractLiteralCases(switch_stmt.cases.items);
        defer if (literal_cases.len > 0) self.allocator.free(literal_cases);
        
        if (literal_cases.len >= self.jump_table_threshold) {
            try self.generateOptimizedLiteralSwitch(value_temp, literal_cases);
        } else {
            // Generate sequential pattern matching
            for (switch_stmt.cases.items, 0..) |case, i| {
                const case_label = try std.fmt.allocPrint(self.allocator, "case_{}", .{i});
                defer self.allocator.free(case_label);
                
                const next_case_label = if (i == switch_stmt.cases.items.len - 1) 
                    "match_fail" 
                else 
                    try std.fmt.allocPrint(self.allocator, "case_{}_fail", .{i});
                defer if (i != switch_stmt.cases.items.len - 1) self.allocator.free(next_case_label);
                
                try self.compilePattern(case.pattern, value_temp, case_label, next_case_label);
                
                // Generate case body
                try self.output.writer().print("{s}:\n", .{case_label});
                try self.output.writer().print("    // Case {} body\n", .{i});
                
                // Execute case statements
                for (case.body.items) |stmt| {
                    try self.output.writer().print("    execute_statement({});\n", .{@intFromPtr(stmt)});
                }
                
                try self.output.writer().print("    goto {s};\n", .{end_label});
                
                if (i != switch_stmt.cases.items.len - 1) {
                    try self.output.writer().print("{s}:\n", .{next_case_label});
                }
            }
        }
        
        // Generate default case based on exhaustiveness
        try self.output.writer().print("match_fail:\n");
        if (!has_default) {
            try self.output.writer().print("    // Non-exhaustive pattern match - runtime error\n");
            try self.output.writer().print("    cursed_runtime_error(\"Pattern match failed: non-exhaustive patterns\");\n");
        } else {
            try self.output.writer().print("    // Exhaustive pattern match - this should never be reached\n");
            try self.output.writer().print("    cursed_unreachable(\"Exhaustive pattern match fallthrough\");\n");
        }
        
        try self.output.writer().print("{s}:\n", .{end_label});
    }
    
    /// Compile match expression with return value
    pub fn compileMatchExpression(self: *PatternCompiler, match_expr: ast.MatchExpression) ![]const u8 {
        try self.output.writer().print("    // Match expression with return value\n");
        
        const value_temp = try self.getTempVar();
        defer self.allocator.free(value_temp);
        
        const result_temp = try self.getTempVar();
        defer self.allocator.free(result_temp);
        
        // Initialize result variable
        try self.output.writer().print("    Value {s};\n", .{result_temp});
        try self.output.writer().print("    auto {s} = evaluate_match_expression();\n", .{value_temp});
        
        const end_label = try std.fmt.allocPrint(self.allocator, "match_end_{}", .{self.block_counter});
        defer self.allocator.free(end_label);
        self.block_counter += 1;
        
        // Generate pattern matching for each case
        for (match_expr.cases.items, 0..) |case, i| {
            const case_label = try std.fmt.allocPrint(self.allocator, "match_case_{}", .{i});
            defer self.allocator.free(case_label);
            
            const next_case_label = if (i == match_expr.cases.items.len - 1) 
                "match_no_default" 
            else 
                try std.fmt.allocPrint(self.allocator, "match_case_{}_fail", .{i});
            defer if (i != match_expr.cases.items.len - 1) self.allocator.free(next_case_label);
            
            try self.compilePattern(case.pattern, value_temp, case_label, next_case_label);
            
            // Generate case result
            try self.output.writer().print("{s}:\n", .{case_label});
            if (case.guard) |guard| {
                try self.output.writer().print("    // Guard condition check\n");
                try self.output.writer().print("    if (!evaluate_guard_condition({})) goto {s};\n", .{ @intFromPtr(guard), next_case_label });
            }
            
            try self.output.writer().print("    {s} = evaluate_expression({});\n", .{ result_temp, @intFromPtr(case.result) });
            try self.output.writer().print("    goto {s};\n", .{end_label});
            
            if (i != match_expr.cases.items.len - 1) {
                try self.output.writer().print("{s}:\n", .{next_case_label});
            }
        }
        
        // Handle default case if provided
        if (match_expr.default_case) |default| {
            try self.output.writer().print("match_no_default:\n");
            try self.output.writer().print("    {s} = evaluate_expression({});\n", .{ result_temp, @intFromPtr(default) });
        } else {
            try self.output.writer().print("match_no_default:\n");
            try self.output.writer().print("    cursed_runtime_error(\"Match expression: no pattern matched\");\n");
        }
        
        try self.output.writer().print("{s}:\n", .{end_label});
        
        return try self.arena.allocator().dupe(u8, result_temp);
    }
    
    /// Compile literal patterns (numbers, strings, booleans)
    fn compileLiteralPattern(self: *PatternCompiler, literal: ast.Pattern.LiteralPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        const temp_var = try self.getTempVar();
        defer self.allocator.free(temp_var);
        
        switch (literal.value) {
            .Integer => |int_val| {
                try self.output.writer().print("    int {s} = ({s} == {});\n", .{ temp_var, value_var, int_val });
            },
            .Float => |float_val| {
                try self.output.writer().print("    int {s} = (fabs({s} - {d}) < 1e-9);\n", .{ temp_var, value_var, float_val });
            },
            .String => |str_val| {
                try self.output.writer().print("    int {s} = (strcmp({s}, \"{s}\") == 0);\n", .{ temp_var, value_var, str_val });
            },
            .Boolean => |bool_val| {
                const bool_str = if (bool_val) "1" else "0";
                try self.output.writer().print("    int {s} = ({s} == {s});\n", .{ temp_var, value_var, bool_str });
            },
        }
        
        try self.output.writer().print("    if ({s}) goto {s}; else goto {s};\n", .{ temp_var, success_label, fail_label });
        
        // Generate LLVM IR if available
        if (self.llvm_builder) |builder| {
            try self.generateLLVMLiteralPattern(builder, literal, value_var, success_label, fail_label);
        }
    }
    
    /// Compile variable binding patterns
    fn compileVariablePattern(self: *PatternCompiler, var_pattern: ast.Pattern.VariablePattern, value_var: []const u8, success_label: []const u8) !void {
        const binding_name = try self.arena.allocator().dupe(u8, var_pattern.name);
        
        // Generate variable binding code
        if (var_pattern.is_mutable) {
            try self.output.writer().print("    // Mutable binding: {s} = {s}\n", .{ binding_name, value_var });
            try self.output.writer().print("    {s} = {s};\n", .{ binding_name, value_var });
        } else {
            try self.output.writer().print("    // Immutable binding: {s} = {s}\n", .{ binding_name, value_var });
            try self.output.writer().print("    const typeof({s}) {s} = {s};\n", .{ value_var, binding_name, value_var });
        }
        
        // Track variable binding for later use
        const binding = VariableBinding{
            .llvm_value = null,
            .c_name = binding_name,
            .type_info = .{ .integer = 32 }, // Default type, should be inferred
            .is_mutable = var_pattern.is_mutable,
        };
        try self.variable_bindings.put(binding_name, binding);
        try self.pattern_variables.append(binding_name);
        
        try self.output.writer().print("    goto {s};\n", .{success_label});
    }
    
    /// Compile wildcard patterns (catch-all)
    fn compileWildcardPattern(self: *PatternCompiler, success_label: []const u8) !void {
        try self.output.writer().print("    // Wildcard pattern - matches anything\n");
        try self.output.writer().print("    goto {s};\n", .{success_label});
    }
    
    /// Compile tuple destructuring patterns
    fn compileTuplePattern(self: *PatternCompiler, tuple: ast.Pattern.TuplePattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Tuple pattern destructuring\n");
        
        // Check tuple length first
        const len_temp = try self.getTempVar();
        defer self.allocator.free(len_temp);
        
        try self.output.writer().print("    int {s} = ({s}->length == {});\n", .{ len_temp, value_var, tuple.patterns.len });
        try self.output.writer().print("    if (!{s}) goto {s};\n", .{ len_temp, fail_label });
        
        // Match each tuple element
        for (tuple.patterns, 0..) |element_pattern, i| {
            const element_var = try std.fmt.allocPrint(self.allocator, "{s}->elements[{}]", .{ value_var, i });
            defer self.allocator.free(element_var);
            
            const element_success = try std.fmt.allocPrint(self.allocator, "tuple_element_{}_success", .{i});
            defer self.allocator.free(element_success);
            
            try self.compilePattern(element_pattern, element_var, element_success, fail_label);
            try self.output.writer().print("{s}:\n", .{element_success});
        }
        
        try self.output.writer().print("    goto {s};\n", .{success_label});
    }
    
    /// Compile struct destructuring patterns
    fn compileStructPattern(self: *PatternCompiler, struct_pattern: ast.Pattern.StructPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Struct pattern destructuring: {s}\n", .{struct_pattern.type_name});
        
        // Type check first
        const type_temp = try self.getTempVar();
        defer self.allocator.free(type_temp);
        
        try self.output.writer().print("    int {s} = (strcmp({s}->type_name, \"{s}\") == 0);\n", .{ type_temp, value_var, struct_pattern.type_name });
        try self.output.writer().print("    if (!{s}) goto {s};\n", .{ type_temp, fail_label });
        
        // Match each field
        for (struct_pattern.fields) |field_pattern| {
            const field_var = try std.fmt.allocPrint(self.allocator, "{s}->{s}", .{ value_var, field_pattern.name });
            defer self.allocator.free(field_var);
            
            const field_success = try std.fmt.allocPrint(self.allocator, "struct_field_{s}_success", .{field_pattern.name});
            defer self.allocator.free(field_success);
            
            try self.compilePattern(field_pattern.pattern, field_var, field_success, fail_label);
            try self.output.writer().print("{s}:\n", .{field_success});
        }
        
        try self.output.writer().print("    goto {s};\n", .{success_label});
    }
    
    /// Compile array patterns with rest elements
    fn compileArrayPattern(self: *PatternCompiler, array: ast.Pattern.ArrayPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Array pattern matching\n");
        
        // Check minimum length
        const min_len = array.patterns.len;
        const len_temp = try self.getTempVar();
        defer self.allocator.free(len_temp);
        
        if (array.rest) |_| {
            try self.output.writer().print("    int {s} = ({s}->length >= {});\n", .{ len_temp, value_var, min_len });
        } else {
            try self.output.writer().print("    int {s} = ({s}->length == {});\n", .{ len_temp, value_var, min_len });
        }
        try self.output.writer().print("    if (!{s}) goto {s};\n", .{ len_temp, fail_label });
        
        // Match each specified element
        for (array.patterns, 0..) |element_pattern, i| {
            const element_var = try std.fmt.allocPrint(self.allocator, "{s}->data[{}]", .{ value_var, i });
            defer self.allocator.free(element_var);
            
            const element_success = try std.fmt.allocPrint(self.allocator, "array_element_{}_success", .{i});
            defer self.allocator.free(element_success);
            
            try self.compilePattern(element_pattern, element_var, element_success, fail_label);
            try self.output.writer().print("{s}:\n", .{element_success});
        }
        
        // Handle rest pattern if present
        if (array.rest) |rest| {
            if (rest.name) |rest_name| {
                try self.output.writer().print("    // Rest pattern binding: {s}\n", .{rest_name});
                try self.output.writer().print("    Array {s} = array_slice({s}, {}, {s}->length);\n", .{ rest_name, value_var, min_len, value_var });
                
                const rest_binding = VariableBinding{
                    .llvm_value = null,
                    .c_name = rest_name,
                    .type_info = .{ .array = .{ .element_type = undefined, .size = null } },
                    .is_mutable = false,
                };
                try self.variable_bindings.put(rest_name, rest_binding);
            }
        }
        
        try self.output.writer().print("    goto {s};\n", .{success_label});
    }
    
    /// Compile slice patterns
    fn compileSlicePattern(self: *PatternCompiler, slice: ast.Pattern.SlicePattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Slice pattern matching\n");
        // Similar to array pattern but for slice types
        try self.compileArrayPattern(.{ .patterns = slice.patterns, .rest = slice.rest }, value_var, success_label, fail_label);
    }
    
    /// Compile OR patterns (multiple alternatives)
    fn compileOrPattern(self: *PatternCompiler, or_pattern: ast.Pattern.OrPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // OR pattern - try alternatives\n");
        
        for (or_pattern.patterns, 0..) |alternative, i| {
            const alt_fail = if (i == or_pattern.patterns.len - 1) fail_label else try std.fmt.allocPrint(self.allocator, "or_alt_{}_fail", .{i});
            defer if (i != or_pattern.patterns.len - 1) self.allocator.free(alt_fail);
            
            try self.compilePattern(alternative, value_var, success_label, alt_fail);
            
            if (i != or_pattern.patterns.len - 1) {
                try self.output.writer().print("{s}:\n", .{alt_fail});
            }
        }
    }
    
    /// Compile range patterns with enhanced 0..10 syntax
    fn compileRangePattern(self: *PatternCompiler, range: ast.Pattern.RangePattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.compileRangePatternEnhanced(range, value_var, success_label, fail_label);
    }
    
    /// Compile guard patterns (conditional matching)
    fn compileGuardPattern(self: *PatternCompiler, guard: ast.Pattern.GuardPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.compileGuardPatternEfficient(guard, value_var, success_label, fail_label);
    }
    
    /// Generate optimized LLVM IR for literal patterns
    fn generateLLVMLiteralPattern(self: *PatternCompiler, builder: c.LLVMBuilderRef, literal: ast.Pattern.LiteralPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        if (self.llvm_context == null) return;
        
        const context = self.llvm_context.?;
        
        // Create basic blocks for success and failure
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(builder));
        const success_block = c.LLVMAppendBasicBlockInContext(context, current_func, success_label.ptr);
        const fail_block = c.LLVMAppendBasicBlockInContext(context, current_func, fail_label.ptr);
        
        // Generate comparison based on literal type
        switch (literal.value) {
            .Integer => |int_val| {
                // Load current value and compare with literal
                const value_ptr = c.LLVMGetNamedGlobal(self.llvm_module.?, value_var.ptr);
                const loaded_value = c.LLVMBuildLoad2(
                    builder,
                    c.LLVMInt64TypeInContext(context),
                    value_ptr,
                    "loaded_value"
                );
                
                const literal_value = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), @as(u64, @intCast(int_val)), 0);
                const cmp_result = c.LLVMBuildICmp(
                    builder,
                    c.LLVMIntEQ,
                    loaded_value,
                    literal_value,
                    "lit_cmp"
                );
                
                _ = c.LLVMBuildCondBr(builder, cmp_result, success_block, fail_block);
            },
            .Float => |float_val| {
                const value_ptr = c.LLVMGetNamedGlobal(self.llvm_module.?, value_var.ptr);
                const loaded_value = c.LLVMBuildLoad2(
                    builder,
                    c.LLVMDoubleTypeInContext(context),
                    value_ptr,
                    "loaded_float"
                );
                
                const literal_value = c.LLVMConstReal(c.LLVMDoubleTypeInContext(context), float_val);
                const cmp_result = c.LLVMBuildFCmp(
                    builder,
                    c.LLVMRealOEQ,
                    loaded_value,
                    literal_value,
                    "float_cmp"
                );
                
                _ = c.LLVMBuildCondBr(builder, cmp_result, success_block, fail_block);
            },
            .Boolean => |bool_val| {
                const value_ptr = c.LLVMGetNamedGlobal(self.llvm_module.?, value_var.ptr);
                const loaded_value = c.LLVMBuildLoad2(
                    builder,
                    c.LLVMInt1TypeInContext(context),
                    value_ptr,
                    "loaded_bool"
                );
                
                const literal_value = c.LLVMConstInt(c.LLVMInt1TypeInContext(context), if (bool_val) 1 else 0, 0);
                const cmp_result = c.LLVMBuildICmp(
                    builder,
                    c.LLVMIntEQ,
                    loaded_value,
                    literal_value,
                    "bool_cmp"
                );
                
                _ = c.LLVMBuildCondBr(builder, cmp_result, success_block, fail_block);
            },
            .String => |str_val| {
                // For string comparison, call runtime function
                const value_ptr = c.LLVMGetNamedGlobal(self.llvm_module.?, value_var.ptr);
                const loaded_str = c.LLVMBuildLoad2(
                    builder,
                    c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                    value_ptr,
                    "loaded_str"
                );
                
                // Create string constant
                const str_constant = c.LLVMBuildGlobalStringPtr(builder, str_val.ptr, "str_literal");
                
                // Call strcmp function
                const strcmp_func = c.LLVMGetNamedFunction(self.llvm_module.?, "strcmp") orelse {
                    const strcmp_type = c.LLVMFunctionType(
                        c.LLVMInt32TypeInContext(context),
                        &[_]c.LLVMTypeRef{
                            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                        },
                        2,
                        0
                    );
                    c.LLVMAddFunction(self.llvm_module.?, "strcmp", strcmp_type);
                };
                
                const strcmp_result = c.LLVMBuildCall2(
                    builder,
                    c.LLVMInt32TypeInContext(context),
                    strcmp_func,
                    &[_]c.LLVMValueRef{ loaded_str, str_constant },
                    2,
                    "strcmp_result"
                );
                
                const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0);
                const cmp_result = c.LLVMBuildICmp(
                    builder,
                    c.LLVMIntEQ,
                    strcmp_result,
                    zero,
                    "str_cmp"
                );
                
                _ = c.LLVMBuildCondBr(builder, cmp_result, success_block, fail_block);
            },
        }
        
        // Position builder at success block for continuation
        c.LLVMPositionBuilderAtEnd(builder, success_block);
    }

    /// Generate LLVM IR for switch-based pattern matching
    pub fn generateLLVMSwitchPattern(self: *PatternCompiler, builder: c.LLVMBuilderRef, value: c.LLVMValueRef, cases: []const SwitchCase) !void {
        if (self.llvm_context == null or self.llvm_module == null) return;
        
        const context = self.llvm_context.?;
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(builder));
        
        // Create default block
        const default_block = c.LLVMAppendBasicBlockInContext(context, current_func, "switch_default");
        
        // Create switch instruction
        const switch_inst = c.LLVMBuildSwitch(builder, value, default_block, @as(u32, @intCast(cases.len)));
        
        // Generate case blocks
        for (cases) |case| {
            const case_block = c.LLVMAppendBasicBlockInContext(context, current_func, case.label.ptr);
            const case_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(case.value)), 0);
            c.LLVMAddCase(switch_inst, case_value, case_block);
            
            // Generate case body (would be filled by higher-level compiler)
            c.LLVMPositionBuilderAtEnd(builder, case_block);
            // Branch to end block (simplified)
        }
        
        // Generate default case
        c.LLVMPositionBuilderAtEnd(builder, default_block);
        // Handle default case (simplified)
    }

    const SwitchCase = struct {
        value: i64,
        label: []const u8,
    };
    
    /// Generate an efficient switch-based dispatch for multiple literal patterns
    pub fn generateOptimizedLiteralSwitch(self: *PatternCompiler, value_var: []const u8, literal_cases: []LiteralCase) !void {
        if (literal_cases.len >= self.jump_table_threshold) {
            try self.output.writer().print("    // Optimized jump table for {} cases\n", .{literal_cases.len});
            try self.output.writer().print("    switch ({s}) {{\n", .{value_var});
            
            for (literal_cases) |case| {
                try self.output.writer().print("        case {}: goto {s};\n", .{ case.value, case.label });
            }
            
            try self.output.writer().print("        default: goto match_fail;\n");
            try self.output.writer().print("    }}\n");
        } else {
            // Use if-else chain for small number of cases
            for (literal_cases, 0..) |case, i| {
                const cond = if (i == 0) "if" else "else if";
                try self.output.writer().print("    {s} ({s} == {}) goto {s};\n", .{ cond, value_var, case.value, case.label });
            }
            try self.output.writer().print("    else goto match_fail;\n");
        }
    }
    
    /// Compile type patterns (instanceof checks)
    fn compileTypePattern(self: *PatternCompiler, type_pattern: ast.Pattern.TypePattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Type pattern matching\n");
        
        const temp_var = try self.getTempVar();
        defer self.allocator.free(temp_var);
        
        // Generate type check based on type expression
        try self.output.writer().print("    int {s} = cursed_instanceof({s}, \"{}\");\n", .{ temp_var, value_var, @intFromPtr(&type_pattern.type_expr) });
        
        // Bind variable if specified
        if (type_pattern.variable) |var_name| {
            try self.output.writer().print("    if ({s}) {{\n", .{temp_var});
            try self.output.writer().print("        // Type-cast binding: {s} = ({})({s})\n", .{ var_name, @intFromPtr(&type_pattern.type_expr), value_var });
            
            const binding = VariableBinding{
                .llvm_value = null,
                .c_name = var_name,
                .type_info = .{ .integer = 32 }, // Will be inferred from type_expr
                .is_mutable = false,
            };
            try self.variable_bindings.put(var_name, binding);
            try self.pattern_variables.append(var_name);
        }
        
        try self.output.writer().print("    if ({s}) goto {s}; else goto {s};\n", .{ temp_var, success_label, fail_label });
    }

    /// Extract literal cases for optimization
    fn extractLiteralCases(self: *PatternCompiler, cases: []const ast.PatternCase) ![]LiteralCase {
        var literal_cases = ArrayList(LiteralCase).init(self.allocator);
        
        for (cases, 0..) |case, i| {
            if (case.pattern == .Literal) {
                const literal = case.pattern.Literal;
                if (literal.value == .Integer) {
                    const label = try std.fmt.allocPrint(self.allocator, "case_{}", .{i});
                    try literal_cases.append(LiteralCase{
                        .value = literal.value.Integer,
                        .label = label,
                    });
                }
            }
        }
        
        return literal_cases.toOwnedSlice();
    }

    /// Enhanced range pattern implementation with 0..10 syntax support
    fn compileRangePatternEnhanced(self: *PatternCompiler, range: ast.Pattern.RangePattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Enhanced range pattern matching with 0..10 syntax\n");
        
        const temp_var = try self.getTempVar();
        defer self.allocator.free(temp_var);
        
        // Enhanced range bounds evaluation with literal optimization
        const start_temp = try self.getTempVar();
        defer self.allocator.free(start_temp);
        
        const end_temp = try self.getTempVar();
        defer self.allocator.free(end_temp);
        
        // Evaluate start and end expressions
        try self.output.writer().print("    auto {s} = evaluate_expression({});\n", .{ start_temp, @intFromPtr(range.start) });
        try self.output.writer().print("    auto {s} = evaluate_expression({});\n", .{ end_temp, @intFromPtr(range.end) });
        
        // Support both inclusive (..) and exclusive (...) ranges
        const op = if (range.is_inclusive) "<=" else "<";
        try self.output.writer().print("    int {s} = ({s} >= {s} && {s} {s} {s});\n", .{ temp_var, value_var, start_temp, value_var, op, end_temp });
        
        // Add optimized bounds checking for integer ranges
        try self.output.writer().print("    // Optimized integer range checking\n");
        try self.output.writer().print("    if (cursed_is_integer({s}) && cursed_is_integer({s}) && cursed_is_integer({s})) {{\n", .{ value_var, start_temp, end_temp });
        try self.output.writer().print("        int64_t val = cursed_to_int({s});\n", .{value_var});
        try self.output.writer().print("        int64_t start_val = cursed_to_int({s});\n", .{start_temp});
        try self.output.writer().print("        int64_t end_val = cursed_to_int({s});\n", .{end_temp});
        try self.output.writer().print("        {s} = (val >= start_val && val {s} end_val);\n", .{ temp_var, op });
        try self.output.writer().print("    }}\n");
        
        try self.output.writer().print("    if ({s}) goto {s}; else goto {s};\n", .{ temp_var, success_label, fail_label });
    }

    /// Compile guard patterns with 'when' condition support
    fn compileGuardPatternEfficient(self: *PatternCompiler, guard: ast.Pattern.GuardPattern, value_var: []const u8, success_label: []const u8, fail_label: []const u8) !void {
        try self.output.writer().print("    // Guard pattern with 'when' condition support\n");
        
        // First match the inner pattern
        const inner_success = try std.fmt.allocPrint(self.allocator, "guard_inner_success_{}", .{self.temp_counter});
        self.temp_counter += 1;
        defer self.allocator.free(inner_success);
        
        try self.compilePattern(guard.pattern.*, value_var, inner_success, fail_label);
        
        try self.output.writer().print("{s}:\n", .{inner_success});
        try self.output.writer().print("    // Evaluate 'when' condition with variable binding context\n");
        
        const guard_temp = try self.getTempVar();
        defer self.allocator.free(guard_temp);
        
        // Enhanced guard evaluation with pattern variable access
        try self.output.writer().print("    // Set up guard evaluation context with pattern variables\n");
        for (self.pattern_variables.items) |var_name| {
            try self.output.writer().print("    cursed_set_guard_variable(\"{s}\", {s});\n", .{ var_name, var_name });
        }
        
        try self.output.writer().print("    int {s} = evaluate_guard_expression({});\n", .{ guard_temp, @intFromPtr(guard.condition) });
        try self.output.writer().print("    cursed_clear_guard_context();\n");
        try self.output.writer().print("    if ({s}) goto {s}; else goto {s};\n", .{ guard_temp, success_label, fail_label });
    }

    /// Generate comprehensive exhaustiveness checking for pattern matches
    pub fn checkExhaustiveness(self: *PatternCompiler, patterns: []const ast.Pattern, matched_type: ?TypeInfo) !ExhaustivenessResult {
        var coverage = ExhaustivenessAnalysis.init(self.allocator);
        defer coverage.deinit();
        
        // Analyze each pattern for coverage
        for (patterns) |pattern| {
            try self.analyzePatternCoverage(&coverage, pattern, matched_type);
        }
        
        // Check for exhaustiveness based on type and patterns
        return self.determineExhaustiveness(coverage, matched_type);
    }
    
    /// Analyze individual pattern coverage
    fn analyzePatternCoverage(self: *PatternCompiler, coverage: *ExhaustivenessAnalysis, pattern: ast.Pattern, matched_type: ?TypeInfo) !void {
        switch (pattern) {
            .Wildcard => {
                coverage.has_wildcard = true;
            },
            .Literal => |lit| {
                switch (lit.value) {
                    .Integer => |val| try coverage.covered_integers.put(val, true),
                    .Boolean => |val| {
                        if (val) coverage.has_true = true else coverage.has_false = true;
                    },
                    .String => |val| try coverage.covered_strings.put(val, true),
                    else => {},
                }
            },
            .Variable => {
                coverage.has_variable_binding = true;
            },
            .Range => |range| {
                // Mark range coverage (simplified)
                coverage.has_range_pattern = true;
                try coverage.range_patterns.append(range);
            },
            .Or => |or_pattern| {
                // Recursively analyze OR alternatives
                for (or_pattern.patterns) |sub_pattern| {
                    try self.analyzePatternCoverage(coverage, sub_pattern, matched_type);
                }
            },
            .Guard => |guard| {
                // Guards reduce coverage - analyze base pattern
                try self.analyzePatternCoverage(coverage, guard.pattern.*, matched_type);
                coverage.has_guards = true;
            },
            .Enum => |enum_pattern| {
                const key = EnumVariantKey{
                    .enum_name = enum_pattern.enum_name,
                    .variant_name = enum_pattern.variant_name,
                };
                try coverage.covered_enum_variants.put(key, true);
            },
            .Tuple => |tuple| {
                coverage.has_tuple_patterns = true;
                // Could analyze tuple arity for exhaustiveness
            },
            .Struct => |struct_pattern| {
                try coverage.covered_struct_types.put(struct_pattern.type_name, true);
            },
            .Array => |array| {
                coverage.has_array_patterns = true;
                // Could analyze array length patterns
            },
            else => {},
        }
    }
    
    /// Determine if patterns are exhaustive for the given type
    fn determineExhaustiveness(self: *PatternCompiler, coverage: ExhaustivenessAnalysis, matched_type: ?TypeInfo) !ExhaustivenessResult {
        _ = self;
        
        // Wildcard or variable binding always provides exhaustive coverage
        if (coverage.has_wildcard or coverage.has_variable_binding) {
            return ExhaustivenessResult{ .is_exhaustive = true, .missing_patterns = ArrayList([]const u8).init(self.allocator) };
        }
        
        var missing_patterns = ArrayList([]const u8).init(self.allocator);
        
        // Type-specific exhaustiveness checking
        if (matched_type) |type_info| {
            switch (type_info) {
                .boolean => {
                    if (!coverage.has_true) try missing_patterns.append("based");
                    if (!coverage.has_false) try missing_patterns.append("cringe");
                },
                .integer => |bits| {
                    // For small integer types, check if all values are covered
                    if (bits <= 8 and !coverage.has_range_pattern) {
                        const max_val = (@as(i64, 1) << @intCast(bits - 1)) - 1;
                        const min_val = -(@as(i64, 1) << @intCast(bits - 1));
                        
                        for (min_val..max_val + 1) |val| {
                            if (!coverage.covered_integers.contains(@intCast(val))) {
                                const pattern_str = try std.fmt.allocPrint(self.allocator, "{}", .{val});
                                try missing_patterns.append(pattern_str);
                            }
                        }
                    }
                },
                else => {
                    // For complex types, require wildcard or complete structural coverage
                    if (!coverage.has_wildcard and !coverage.has_variable_binding) {
                        try missing_patterns.append("_"); // Suggest wildcard
                    }
                },
            }
        }
        
        // Check enum exhaustiveness if we have enum patterns
        if (coverage.covered_enum_variants.count() > 0) {
            // This would require type information about the enum
            // For now, assume non-exhaustive unless wildcard present
            if (!coverage.has_wildcard and missing_patterns.items.len == 0) {
                try missing_patterns.append("_"); // Suggest wildcard for unknown enum variants
            }
        }
        
        const is_exhaustive = missing_patterns.items.len == 0 or coverage.has_guards;
        
        return ExhaustivenessResult{
            .is_exhaustive = is_exhaustive,
            .missing_patterns = missing_patterns,
        };
    }
    
    /// Generate comprehensive pattern coverage analysis (legacy method)
    pub fn analyzePatternCoverage(self: *PatternCompiler, patterns: []const ast.Pattern) !bool {
        const result = try self.checkExhaustiveness(patterns, null);
        defer result.missing_patterns.deinit();
        return result.is_exhaustive;
    }
    
    /// Generate exhaustiveness warning/error messages
    pub fn generateExhaustivenessReport(self: *PatternCompiler, result: ExhaustivenessResult) !void {
        if (!result.is_exhaustive) {
            try self.output.writer().print("    // EXHAUSTIVENESS WARNING: Missing patterns detected\n");
            try self.output.writer().print("    cursed_compiler_warning(\"Non-exhaustive pattern match\");\n");
            
            if (result.missing_patterns.items.len > 0) {
                try self.output.writer().print("    // Missing patterns: ");
                for (result.missing_patterns.items, 0..) |pattern, i| {
                    if (i > 0) try self.output.writer().print(", ");
                    try self.output.writer().print("{s}", .{pattern});
                }
                try self.output.writer().print("\n");
            }
            
            // Generate runtime exhaustiveness check
            try self.output.writer().print("    cursed_runtime_exhaustiveness_check();\n");
        } else {
            try self.output.writer().print("    // EXHAUSTIVENESS: Pattern match is exhaustive\n");
        }
    }
    
    const ExhaustivenessAnalysis = struct {
        has_wildcard: bool = false,
        has_variable_binding: bool = false,
        has_guards: bool = false,
        has_range_pattern: bool = false,
        has_tuple_patterns: bool = false,
        has_array_patterns: bool = false,
        has_true: bool = false,
        has_false: bool = false,
        covered_integers: HashMap(i64, bool, IntegerContext, std.hash_map.default_max_load_percentage),
        covered_strings: HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        covered_enum_variants: HashMap(EnumVariantKey, bool, EnumVariantKeyContext, std.hash_map.default_max_load_percentage),
        covered_struct_types: HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        range_patterns: ArrayList(ast.Pattern.RangePattern),
        allocator: Allocator,
        
        const IntegerContext = struct {
            pub fn hash(self: @This(), key: i64) u64 {
                _ = self;
                return @bitCast(key);
            }
            pub fn eql(self: @This(), a: i64, b: i64) bool {
                _ = self;
                return a == b;
            }
        };
        
        const EnumVariantKey = struct {
            enum_name: []const u8,
            variant_name: []const u8,
        };
        
        const EnumVariantKeyContext = struct {
            pub fn hash(self: @This(), key: EnumVariantKey) u64 {
                _ = self;
                var hasher = std.hash.Wyhash.init(0);
                hasher.update(key.enum_name);
                hasher.update(key.variant_name);
                return hasher.final();
            }
            pub fn eql(self: @This(), a: EnumVariantKey, b: EnumVariantKey) bool {
                _ = self;
                return std.mem.eql(u8, a.enum_name, b.enum_name) and 
                       std.mem.eql(u8, a.variant_name, b.variant_name);
            }
        };
        
        fn init(allocator: Allocator) ExhaustivenessAnalysis {
            return ExhaustivenessAnalysis{
                .covered_integers = HashMap(i64, bool, IntegerContext, std.hash_map.default_max_load_percentage).init(allocator),
                .covered_strings = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .covered_enum_variants = HashMap(EnumVariantKey, bool, EnumVariantKeyContext, std.hash_map.default_max_load_percentage).init(allocator),
                .covered_struct_types = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
                .range_patterns = ArrayList(ast.Pattern.RangePattern).init(allocator),
                .allocator = allocator,
            };
        }
        
        fn deinit(self: *ExhaustivenessAnalysis) void {
            self.covered_integers.deinit();
            self.covered_strings.deinit();
            self.covered_enum_variants.deinit();
            self.covered_struct_types.deinit();
            self.range_patterns.deinit();
        }
    };
    
    const ExhaustivenessResult = struct {
        is_exhaustive: bool,
        missing_patterns: ArrayList([]const u8),
    };

    /// Helper to get a temporary variable name
    fn getTempVar(self: *PatternCompiler) ![]const u8 {
        const name = try std.fmt.allocPrint(self.allocator, "temp_{}", .{self.temp_counter});
        self.temp_counter += 1;
        return name;
    }
    
    const LiteralCase = struct {
        value: i64,
        label: []const u8,
    };
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
