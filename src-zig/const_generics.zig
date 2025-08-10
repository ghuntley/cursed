const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const print = std.debug.print;
const ast = @import("ast.zig");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
});

/// Const generic parameter types
pub const ConstGenericKind = enum {
    Integer,
    Boolean,
    String,
    Float,
    Array,
    
    pub fn format(self: ConstGenericKind, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self) {
            .Integer => try writer.print("integer"),
            .Boolean => try writer.print("boolean"),
            .String => try writer.print("string"),
            .Float => try writer.print("float"),
            .Array => try writer.print("array"),
        }
    }
};

/// Const generic value representation
pub const ConstGenericValue = union(ConstGenericKind) {
    Integer: i64,
    Boolean: bool,
    String: []const u8,
    Float: f64,
    Array: ArrayConstValue,
    
    pub const ArrayConstValue = struct {
        element_type: ConstGenericKind,
        length: usize,
        elements: []ConstGenericValue,
    };
    
    pub fn format(self: ConstGenericValue, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self) {
            .Integer => |val| try writer.print("{}", .{val}),
            .Boolean => |val| try writer.print("{}", .{val}),
            .String => |val| try writer.print("\"{}\"", .{val}),
            .Float => |val| try writer.print("{d}", .{val}),
            .Array => |arr| {
                try writer.print("[");
                for (arr.elements, 0..) |elem, i| {
                    if (i > 0) try writer.print(", ");
                    try elem.format("", .{}, writer);
                }
                try writer.print("]");
            },
        }
    }
    
    pub fn getKind(self: ConstGenericValue) ConstGenericKind {
        return @as(ConstGenericKind, self);
    }
};

/// Const generic bounds/constraints
pub const ConstGenericBounds = struct {
    min_value: ?ConstGenericValue = null,
    max_value: ?ConstGenericValue = null,
    allowed_values: ?ArrayList(ConstGenericValue) = null,
    must_be_positive: bool = false,
    must_be_power_of_two: bool = false,
    max_array_length: ?usize = null,
    
    pub fn init(allocator: Allocator) ConstGenericBounds {
        return ConstGenericBounds{
            .allowed_values = ArrayList(ConstGenericValue).init(allocator),
        };
    }
    
    pub fn deinit(self: *ConstGenericBounds) void {
        if (self.allowed_values) |*values| {
            values.deinit();
        }
    }
    
    /// Validate a const generic value against these bounds
    pub fn validate(self: ConstGenericBounds, value: ConstGenericValue) ConstGenericError!void {
        // Check allowed values first (most restrictive)
        if (self.allowed_values) |allowed| {
            var found = false;
            for (allowed.items) |allowed_val| {
                if (self.valuesEqual(value, allowed_val)) {
                    found = true;
                    break;
                }
            }
            if (!found) {
                return ConstGenericError.ValueNotInAllowedSet;
            }
        }
        
        switch (value) {
            .Integer => |int_val| {
                // Check bounds
                if (self.min_value) |min| {
                    if (min == .Integer and int_val < min.Integer) {
                        return ConstGenericError.ValueBelowMinimum;
                    }
                }
                if (self.max_value) |max| {
                    if (max == .Integer and int_val > max.Integer) {
                        return ConstGenericError.ValueAboveMaximum;
                    }
                }
                
                // Check positivity
                if (self.must_be_positive and int_val <= 0) {
                    return ConstGenericError.ValueMustBePositive;
                }
                
                // Check power of two
                if (self.must_be_power_of_two) {
                    if (int_val <= 0 or (int_val & (int_val - 1)) != 0) {
                        return ConstGenericError.ValueMustBePowerOfTwo;
                    }
                }
            },
            .Float => |float_val| {
                // Check bounds
                if (self.min_value) |min| {
                    if (min == .Float and float_val < min.Float) {
                        return ConstGenericError.ValueBelowMinimum;
                    }
                }
                if (self.max_value) |max| {
                    if (max == .Float and float_val > max.Float) {
                        return ConstGenericError.ValueAboveMaximum;
                    }
                }
                
                // Check positivity
                if (self.must_be_positive and float_val <= 0.0) {
                    return ConstGenericError.ValueMustBePositive;
                }
            },
            .Array => |arr_val| {
                // Check array length bounds
                if (self.max_array_length) |max_len| {
                    if (arr_val.length > max_len) {
                        return ConstGenericError.ArrayTooLong;
                    }
                }
                
                // Recursively validate array elements if bounds specify element constraints
                // This would need additional bounds structure for element-specific constraints
            },
            .Boolean, .String => {
                // These are always valid unless restricted by allowed_values
            },
        }
    }
    
    fn valuesEqual(self: ConstGenericBounds, a: ConstGenericValue, b: ConstGenericValue) bool {
        if (@as(ConstGenericKind, a) != @as(ConstGenericKind, b)) return false;
        
        switch (a) {
            .Integer => |a_val| return a_val == b.Integer,
            .Boolean => |a_val| return a_val == b.Boolean,
            .String => |a_val| return std.mem.eql(u8, a_val, b.String),
            .Float => |a_val| return a_val == b.Float,
            .Array => |a_arr| {
                const b_arr = b.Array;
                if (a_arr.length != b_arr.length) return false;
                if (a_arr.element_type != b_arr.element_type) return false;
                for (a_arr.elements, b_arr.elements) |a_elem, b_elem| {
                    if (!self.valuesEqual(a_elem, b_elem)) return false;
                }
                return true;
            },
        }
    }
};

/// Const generic parameter definition
pub const ConstGenericParam = struct {
    name: []const u8,
    kind: ConstGenericKind,
    bounds: ConstGenericBounds,
    default_value: ?ConstGenericValue = null,
    
    pub fn init(allocator: Allocator, name: []const u8, kind: ConstGenericKind) ConstGenericParam {
        return ConstGenericParam{
            .name = name,
            .kind = kind,
            .bounds = ConstGenericBounds.init(allocator),
        };
    }
    
    pub fn deinit(self: *ConstGenericParam) void {
        self.bounds.deinit();
    }
    
    pub fn validate(self: ConstGenericParam, value: ConstGenericValue) ConstGenericError!void {
        // First check if the value type matches the parameter type
        if (value.getKind() != self.kind) {
            return ConstGenericError.TypeMismatch;
        }
        
        // Then validate against bounds
        try self.bounds.validate(value);
    }
};

/// Const generic errors
pub const ConstGenericError = error{
    TypeMismatch,
    ValueBelowMinimum,
    ValueAboveMaximum,
    ValueNotInAllowedSet,
    ValueMustBePositive,
    ValueMustBePowerOfTwo,
    ArrayTooLong,
    ParameterNotFound,
    InvalidConstantExpression,
    OptimizerICE,
    BoundsCheckFailed,
    ConstantEvaluationFailed,
    CircularConstantDependency,
    ConstantOverflow,
    InvalidArrayIndex,
    DivisionByZeroInConstant,
};

/// Const generic instantiation context
pub const ConstGenericInstantiation = struct {
    allocator: Allocator,
    parameters: HashMap([]const u8, ConstGenericParam, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    values: HashMap([]const u8, ConstGenericValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator) ConstGenericInstantiation {
        return ConstGenericInstantiation{
            .allocator = allocator,
            .parameters = HashMap([]const u8, ConstGenericParam, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .values = HashMap([]const u8, ConstGenericValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *ConstGenericInstantiation) void {
        var param_iter = self.parameters.iterator();
        while (param_iter.next()) |entry| {
            var param = entry.value_ptr;
            param.deinit();
        }
        self.parameters.deinit();
        self.values.deinit();
    }
    
    /// Add a const generic parameter with bounds
    pub fn addParameter(self: *ConstGenericInstantiation, param: ConstGenericParam) !void {
        try self.parameters.put(param.name, param);
    }
    
    /// Set a const generic value with validation
    pub fn setValue(self: *ConstGenericInstantiation, name: []const u8, value: ConstGenericValue) ConstGenericError!void {
        const param = self.parameters.get(name) orelse {
            std.log.err("Const generic parameter '{}' not found", .{name});
            return ConstGenericError.ParameterNotFound;
        };
        
        // Validate the value against parameter constraints
        param.validate(value) catch |err| {
            std.log.err("Const generic validation failed for parameter '{}': {}", .{ name, err });
            std.log.err("  Parameter type: {}", .{param.kind});
            std.log.err("  Provided value: {}", .{value});
            return err;
        };
        
        try self.values.put(name, value);
    }
    
    /// Get a const generic value
    pub fn getValue(self: ConstGenericInstantiation, name: []const u8) ?ConstGenericValue {
        return self.values.get(name);
    }
    
    /// Validate all parameters have values and all values are valid
    pub fn validateComplete(self: ConstGenericInstantiation) ConstGenericError!void {
        var param_iter = self.parameters.iterator();
        while (param_iter.next()) |entry| {
            const param_name = entry.key_ptr.*;
            const param = entry.value_ptr.*;
            
            const value = self.values.get(param_name) orelse {
                // Check for default value
                if (param.default_value) |default| {
                    // Validate default value
                    try param.validate(default);
                    continue;
                }
                std.log.err("Const generic parameter '{}' has no value and no default", .{param_name});
                return ConstGenericError.ParameterNotFound;
            };
            
            // Validate the value
            try param.validate(value);
        }
    }
};

/// Const generic evaluator - handles compile-time constant evaluation
pub const ConstGenericEvaluator = struct {
    allocator: Allocator,
    instantiation: *ConstGenericInstantiation,
    
    pub fn init(allocator: Allocator, instantiation: *ConstGenericInstantiation) ConstGenericEvaluator {
        return ConstGenericEvaluator{
            .allocator = allocator,
            .instantiation = instantiation,
        };
    }
    
    /// Evaluate a const generic expression to a constant value
    pub fn evaluateExpression(self: *ConstGenericEvaluator, expr: ast.Expression) ConstGenericError!ConstGenericValue {
        return switch (expr) {
            .Literal => |literal| self.evaluateLiteral(literal),
            .Identifier => |ident| self.evaluateIdentifier(ident.name),
            .Binary => |binary| self.evaluateBinaryExpression(binary),
            .Unary => |unary| self.evaluateUnaryExpression(unary),
            .ArrayAccess => |access| self.evaluateArrayAccess(access),
            .FunctionCall => |call| self.evaluateFunctionCall(call),
            else => {
                std.log.err("Unsupported const generic expression type: {}", .{@as(ast.ExpressionTag, expr)});
                return ConstGenericError.InvalidConstantExpression;
            },
        };
    }
    
    fn evaluateLiteral(self: *ConstGenericEvaluator, literal: ast.Literal) ConstGenericError!ConstGenericValue {
        _ = self;
        return switch (literal) {
            .Integer => |val| ConstGenericValue{ .Integer = val },
            .Float => |val| ConstGenericValue{ .Float = val },
            .String => |val| ConstGenericValue{ .String = val },
            .Boolean => |val| ConstGenericValue{ .Boolean = val },
            else => {
                std.log.err("Unsupported literal type for const generic: {}", .{@as(ast.LiteralTag, literal)});
                return ConstGenericError.InvalidConstantExpression;
            },
        };
    }
    
    fn evaluateIdentifier(self: *ConstGenericEvaluator, name: []const u8) ConstGenericError!ConstGenericValue {
        return self.instantiation.getValue(name) orelse {
            std.log.err("Const generic parameter '{}' not found during evaluation", .{name});
            return ConstGenericError.ParameterNotFound;
        };
    }
    
    fn evaluateBinaryExpression(self: *ConstGenericEvaluator, binary: ast.BinaryExpression) ConstGenericError!ConstGenericValue {
        const left = try self.evaluateExpression(binary.left.*);
        const right = try self.evaluateExpression(binary.right.*);
        
        return switch (binary.operator) {
            .Plus => self.addValues(left, right),
            .Minus => self.subtractValues(left, right),
            .Multiply => self.multiplyValues(left, right),
            .Divide => self.divideValues(left, right),
            .Modulo => self.moduloValues(left, right),
            .Equal => ConstGenericValue{ .Boolean = self.compareValues(left, right, .Equal) },
            .NotEqual => ConstGenericValue{ .Boolean = self.compareValues(left, right, .NotEqual) },
            .LessThan => ConstGenericValue{ .Boolean = self.compareValues(left, right, .LessThan) },
            .LessThanOrEqual => ConstGenericValue{ .Boolean = self.compareValues(left, right, .LessThanOrEqual) },
            .GreaterThan => ConstGenericValue{ .Boolean = self.compareValues(left, right, .GreaterThan) },
            .GreaterThanOrEqual => ConstGenericValue{ .Boolean = self.compareValues(left, right, .GreaterThanOrEqual) },
            else => {
                std.log.err("Unsupported binary operator in const generic: {}", .{binary.operator});
                return ConstGenericError.InvalidConstantExpression;
            },
        };
    }
    
    fn evaluateUnaryExpression(self: *ConstGenericEvaluator, unary: ast.UnaryExpression) ConstGenericError!ConstGenericValue {
        const operand = try self.evaluateExpression(unary.operand.*);
        
        return switch (unary.operator) {
            .Minus => switch (operand) {
                .Integer => |val| ConstGenericValue{ .Integer = -val },
                .Float => |val| ConstGenericValue{ .Float = -val },
                else => {
                    std.log.err("Cannot apply unary minus to non-numeric const generic value");
                    return ConstGenericError.InvalidConstantExpression;
                },
            },
            .Not => switch (operand) {
                .Boolean => |val| ConstGenericValue{ .Boolean = !val },
                else => {
                    std.log.err("Cannot apply logical not to non-boolean const generic value");
                    return ConstGenericError.InvalidConstantExpression;
                },
            },
            else => {
                std.log.err("Unsupported unary operator in const generic: {}", .{unary.operator});
                return ConstGenericError.InvalidConstantExpression;
            },
        };
    }
    
    fn evaluateArrayAccess(self: *ConstGenericEvaluator, access: ast.ArrayAccessExpression) ConstGenericError!ConstGenericValue {
        const array = try self.evaluateExpression(access.array.*);
        const index = try self.evaluateExpression(access.index.*);
        
        switch (array) {
            .Array => |arr| {
                switch (index) {
                    .Integer => |idx| {
                        if (idx < 0 or idx >= @as(i64, @intCast(arr.length))) {
                            std.log.err("Array index {} out of bounds (length: {})", .{ idx, arr.length });
                            return ConstGenericError.InvalidArrayIndex;
                        }
                        return arr.elements[@intCast(idx)];
                    },
                    else => {
                        std.log.err("Array index must be an integer in const generic context");
                        return ConstGenericError.InvalidConstantExpression;
                    },
                }
            },
            else => {
                std.log.err("Cannot index non-array value in const generic context");
                return ConstGenericError.InvalidConstantExpression;
            },
        }
    }
    
    fn evaluateFunctionCall(self: *ConstGenericEvaluator, call: ast.CallExpression) ConstGenericError!ConstGenericValue {
        _ = self;
        _ = call;
        // For now, we don't support function calls in const generics
        // This could be extended to support built-in const functions like min, max, etc.
        std.log.err("Function calls not yet supported in const generic expressions");
        return ConstGenericError.InvalidConstantExpression;
    }
    
    // Arithmetic operations with overflow checking
    fn addValues(self: *ConstGenericEvaluator, left: ConstGenericValue, right: ConstGenericValue) ConstGenericError!ConstGenericValue {
        _ = self;
        return switch (left) {
            .Integer => |l_val| switch (right) {
                .Integer => |r_val| blk: {
                    const result = @addWithOverflow(l_val, r_val);
                    if (result[1] != 0) {
                        return ConstGenericError.ConstantOverflow;
                    }
                    break :blk ConstGenericValue{ .Integer = result[0] };
                },
                else => ConstGenericError.TypeMismatch,
            },
            .Float => |l_val| switch (right) {
                .Float => |r_val| ConstGenericValue{ .Float = l_val + r_val },
                else => ConstGenericError.TypeMismatch,
            },
            else => ConstGenericError.InvalidConstantExpression,
        };
    }
    
    fn subtractValues(self: *ConstGenericEvaluator, left: ConstGenericValue, right: ConstGenericValue) ConstGenericError!ConstGenericValue {
        _ = self;
        return switch (left) {
            .Integer => |l_val| switch (right) {
                .Integer => |r_val| blk: {
                    const result = @subWithOverflow(l_val, r_val);
                    if (result[1] != 0) {
                        return ConstGenericError.ConstantOverflow;
                    }
                    break :blk ConstGenericValue{ .Integer = result[0] };
                },
                else => ConstGenericError.TypeMismatch,
            },
            .Float => |l_val| switch (right) {
                .Float => |r_val| ConstGenericValue{ .Float = l_val - r_val },
                else => ConstGenericError.TypeMismatch,
            },
            else => ConstGenericError.InvalidConstantExpression,
        };
    }
    
    fn multiplyValues(self: *ConstGenericEvaluator, left: ConstGenericValue, right: ConstGenericValue) ConstGenericError!ConstGenericValue {
        _ = self;
        return switch (left) {
            .Integer => |l_val| switch (right) {
                .Integer => |r_val| blk: {
                    const result = @mulWithOverflow(l_val, r_val);
                    if (result[1] != 0) {
                        return ConstGenericError.ConstantOverflow;
                    }
                    break :blk ConstGenericValue{ .Integer = result[0] };
                },
                else => ConstGenericError.TypeMismatch,
            },
            .Float => |l_val| switch (right) {
                .Float => |r_val| ConstGenericValue{ .Float = l_val * r_val },
                else => ConstGenericError.TypeMismatch,
            },
            else => ConstGenericError.InvalidConstantExpression,
        };
    }
    
    fn divideValues(self: *ConstGenericEvaluator, left: ConstGenericValue, right: ConstGenericValue) ConstGenericError!ConstGenericValue {
        _ = self;
        return switch (left) {
            .Integer => |l_val| switch (right) {
                .Integer => |r_val| {
                    if (r_val == 0) {
                        return ConstGenericError.DivisionByZeroInConstant;
                    }
                    return ConstGenericValue{ .Integer = @divTrunc(l_val, r_val) };
                },
                else => ConstGenericError.TypeMismatch,
            },
            .Float => |l_val| switch (right) {
                .Float => |r_val| {
                    if (r_val == 0.0) {
                        return ConstGenericError.DivisionByZeroInConstant;
                    }
                    return ConstGenericValue{ .Float = l_val / r_val };
                },
                else => ConstGenericError.TypeMismatch,
            },
            else => ConstGenericError.InvalidConstantExpression,
        };
    }
    
    fn moduloValues(self: *ConstGenericEvaluator, left: ConstGenericValue, right: ConstGenericValue) ConstGenericError!ConstGenericValue {
        _ = self;
        return switch (left) {
            .Integer => |l_val| switch (right) {
                .Integer => |r_val| {
                    if (r_val == 0) {
                        return ConstGenericError.DivisionByZeroInConstant;
                    }
                    return ConstGenericValue{ .Integer = @mod(l_val, r_val) };
                },
                else => ConstGenericError.TypeMismatch,
            },
            else => ConstGenericError.InvalidConstantExpression,
        };
    }
    
    const ComparisonOp = enum { Equal, NotEqual, LessThan, LessThanOrEqual, GreaterThan, GreaterThanOrEqual };
    
    fn compareValues(self: *ConstGenericEvaluator, left: ConstGenericValue, right: ConstGenericValue, op: ComparisonOp) bool {
        const bounds = ConstGenericBounds.init(self.allocator);
        defer {
            var mut_bounds = bounds;
            mut_bounds.deinit();
        }
        
        return switch (left) {
            .Integer => |l_val| switch (right) {
                .Integer => |r_val| switch (op) {
                    .Equal => l_val == r_val,
                    .NotEqual => l_val != r_val,
                    .LessThan => l_val < r_val,
                    .LessThanOrEqual => l_val <= r_val,
                    .GreaterThan => l_val > r_val,
                    .GreaterThanOrEqual => l_val >= r_val,
                },
                else => false,
            },
            .Float => |l_val| switch (right) {
                .Float => |r_val| switch (op) {
                    .Equal => l_val == r_val,
                    .NotEqual => l_val != r_val,
                    .LessThan => l_val < r_val,
                    .LessThanOrEqual => l_val <= r_val,
                    .GreaterThan => l_val > r_val,
                    .GreaterThanOrEqual => l_val >= r_val,
                },
                else => false,
            },
            .Boolean => |l_val| switch (right) {
                .Boolean => |r_val| switch (op) {
                    .Equal => l_val == r_val,
                    .NotEqual => l_val != r_val,
                    else => false,
                },
                else => false,
            },
            .String => |l_val| switch (right) {
                .String => |r_val| switch (op) {
                    .Equal => std.mem.eql(u8, l_val, r_val),
                    .NotEqual => !std.mem.eql(u8, l_val, r_val),
                    .LessThan => std.mem.lessThan(u8, l_val, r_val),
                    .LessThanOrEqual => std.mem.lessThan(u8, l_val, r_val) or std.mem.eql(u8, l_val, r_val),
                    .GreaterThan => !std.mem.lessThan(u8, l_val, r_val) and !std.mem.eql(u8, l_val, r_val),
                    .GreaterThanOrEqual => !std.mem.lessThan(u8, l_val, r_val),
                },
                else => false,
            },
            .Array => |_| switch (right) {
                .Array => |_| switch (op) {
                    .Equal => bounds.valuesEqual(left, right),
                    .NotEqual => !bounds.valuesEqual(left, right),
                    else => false,
                },
                else => false,
            },
        };
    }
};

/// Integration with LLVM optimization to prevent ICE
pub const ConstGenericLLVMIntegration = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef) ConstGenericLLVMIntegration {
        return ConstGenericLLVMIntegration{
            .allocator = allocator,
            .context = context,
        };
    }
    
    /// Safely generate LLVM constant from const generic value
    pub fn generateLLVMConstant(self: *ConstGenericLLVMIntegration, value: ConstGenericValue) ConstGenericError!c.LLVMValueRef {
        return switch (value) {
            .Integer => |int_val| {
                // Validate integer is within safe bounds for LLVM
                if (int_val < std.math.minInt(i32) or int_val > std.math.maxInt(i32)) {
                    std.log.err("Integer const generic value {} exceeds LLVM i32 bounds", .{int_val});
                    return ConstGenericError.ConstantOverflow;
                }
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @intCast(int_val), 0);
            },
            .Boolean => |bool_val| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
            },
            .String => |str_val| {
                return c.LLVMConstStringInContext(self.context, str_val.ptr, @intCast(str_val.len), 0);
            },
            .Array => |arr_val| {
                var llvm_elements = ArrayList(c.LLVMValueRef).init(self.allocator);
                defer llvm_elements.deinit();
                
                for (arr_val.elements) |elem| {
                    const llvm_elem = try self.generateLLVMConstant(elem);
                    try llvm_elements.append(llvm_elem);
                }
                
                const element_type = try self.constGenericKindToLLVMType(arr_val.element_type);
                return c.LLVMConstArray(element_type, llvm_elements.items.ptr, @intCast(llvm_elements.items.len));
            },
        };
    }
    
    /// Convert const generic kind to LLVM type
    pub fn constGenericKindToLLVMType(self: *ConstGenericLLVMIntegration, kind: ConstGenericKind) ConstGenericError!c.LLVMTypeRef {
        return switch (kind) {
            .Integer => c.LLVMInt64TypeInContext(self.context),
            .Boolean => c.LLVMInt1TypeInContext(self.context),
            .Float => c.LLVMDoubleTypeInContext(self.context),
            .String => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
            .Array => {
                std.log.err("Array const generic types need element type specification");
                return ConstGenericError.InvalidConstantExpression;
            },
        };
    }
    
    /// Validate LLVM constant value before optimizer sees it
    pub fn validateLLVMConstant(self: *ConstGenericLLVMIntegration, value: c.LLVMValueRef) ConstGenericError!void {
        _ = self;
        
        // Check if the value is actually constant
        if (c.LLVMIsConstant(value) == 0) {
            std.log.err("Expected constant value but got non-constant in const generic context");
            return ConstGenericError.BoundsCheckFailed;
        }
        
        // Check for undefined values that could cause optimizer ICE
        if (c.LLVMIsUndef(value) != 0) {
            std.log.err("Undefined value detected in const generic - this would cause optimizer ICE");
            return ConstGenericError.OptimizerICE;
        }
        
        // Additional validation for specific value types
        const value_type = c.LLVMTypeOf(value);
        const type_kind = c.LLVMGetTypeKind(value_type);
        
        switch (type_kind) {
            c.LLVMIntegerTypeKind => {
                // Ensure integer constants are within bounds
                const bit_width = c.LLVMGetIntTypeWidth(value_type);
                if (bit_width > 64) {
                    std.log.warn("Large integer constant ({} bits) may cause optimizer issues", .{bit_width});
                }
            },
            c.LLVMArrayTypeKind => {
                const array_length = c.LLVMGetArrayLength(value_type);
                if (array_length > 1024) {
                    std.log.warn("Large array constant ({} elements) may cause optimizer issues", .{array_length});
                }
            },
            else => {},
        }
    }
};

/// Main const generics manager that prevents ICE
pub const ConstGenericsManager = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    instantiation: ConstGenericInstantiation,
    evaluator: ConstGenericEvaluator,
    llvm_integration: ConstGenericLLVMIntegration,
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef) ConstGenericsManager {
        var instantiation = ConstGenericInstantiation.init(allocator);
        const evaluator = ConstGenericEvaluator.init(allocator, &instantiation);
        const llvm_integration = ConstGenericLLVMIntegration.init(allocator, context);
        
        return ConstGenericsManager{
            .allocator = allocator,
            .context = context,
            .instantiation = instantiation,
            .evaluator = evaluator,
            .llvm_integration = llvm_integration,
        };
    }
    
    pub fn deinit(self: *ConstGenericsManager) void {
        self.instantiation.deinit();
    }
    
    /// Process const generic declaration with bounds checking
    pub fn processConstGenericDeclaration(
        self: *ConstGenericsManager,
        name: []const u8,
        kind: ConstGenericKind,
        bounds: ?ConstGenericBounds,
        default_value: ?ConstGenericValue,
    ) ConstGenericError!void {
        var param = ConstGenericParam.init(self.allocator, name, kind);
        
        if (bounds) |b| {
            param.bounds = b;
        }
        
        if (default_value) |default| {
            // Validate default value
            try param.validate(default);
            param.default_value = default;
        }
        
        try self.instantiation.addParameter(param);
        
        std.log.info("Registered const generic parameter '{}' of type {}", .{ name, kind });
    }
    
    /// Process const generic instantiation with full validation
    pub fn processConstGenericInstantiation(
        self: *ConstGenericsManager,
        name: []const u8,
        value_expr: ast.Expression,
    ) ConstGenericError!c.LLVMValueRef {
        std.log.info("Processing const generic instantiation for '{}'", .{name});
        
        // Evaluate the expression to a constant value
        const const_value = self.evaluator.evaluateExpression(value_expr) catch |err| {
            std.log.err("Failed to evaluate const generic expression for '{}': {}", .{ name, err });
            return err;
        };
        
        // Set the value with validation
        self.instantiation.setValue(name, const_value) catch |err| {
            std.log.err("Failed to set const generic value for '{}': {}", .{ name, err });
            return err;
        };
        
        // Generate LLVM constant safely
        const llvm_value = self.llvm_integration.generateLLVMConstant(const_value) catch |err| {
            std.log.err("Failed to generate LLVM constant for '{}': {}", .{ name, err });
            return err;
        };
        
        // Validate LLVM constant to prevent optimizer ICE
        self.llvm_integration.validateLLVMConstant(llvm_value) catch |err| {
            std.log.err("LLVM constant validation failed for '{}' - preventing optimizer ICE: {}", .{ name, err });
            return err;
        };
        
        std.log.info("Successfully processed const generic '{}' = {}", .{ name, const_value });
        return llvm_value;
    }
    
    /// Validate all const generics before code generation
    pub fn validateAllConstGenerics(self: *ConstGenericsManager) ConstGenericError!void {
        std.log.info("Validating all const generics before code generation...");
        
        try self.instantiation.validateComplete();
        
        // Additional checks for optimizer safety
        var value_iter = self.instantiation.values.iterator();
        while (value_iter.next()) |entry| {
            const name = entry.key_ptr.*;
            const value = entry.value_ptr.*;
            
            // Check for potentially problematic values
            switch (value) {
                .Integer => |int_val| {
                    if (int_val < -1000000 or int_val > 1000000) {
                        std.log.warn("Large integer const generic '{}' = {} may affect optimizer performance", .{ name, int_val });
                    }
                },
                .Array => |arr| {
                    if (arr.length > 1000) {
                        std.log.warn("Large array const generic '{}' with {} elements may affect optimizer performance", .{ name, arr.length });
                    }
                },
                else => {},
            }
        }
        
        std.log.info("All const generics validated successfully");
    }
};

// Tests
test "const generic bounds validation" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var bounds = ConstGenericBounds.init(allocator);
    defer bounds.deinit();
    
    bounds.min_value = ConstGenericValue{ .Integer = 0 };
    bounds.max_value = ConstGenericValue{ .Integer = 100 };
    bounds.must_be_positive = true;
    
    // Valid value
    try bounds.validate(ConstGenericValue{ .Integer = 50 });
    
    // Invalid values
    try std.testing.expectError(ConstGenericError.ValueBelowMinimum, bounds.validate(ConstGenericValue{ .Integer = -1 }));
    try std.testing.expectError(ConstGenericError.ValueAboveMaximum, bounds.validate(ConstGenericValue{ .Integer = 101 }));
    try std.testing.expectError(ConstGenericError.ValueMustBePositive, bounds.validate(ConstGenericValue{ .Integer = 0 }));
}

test "const generic expression evaluation" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var instantiation = ConstGenericInstantiation.init(allocator);
    defer instantiation.deinit();
    
    // Add a parameter
    const param = ConstGenericParam.init(allocator, "N", .Integer);
    try instantiation.addParameter(param);
    try instantiation.setValue("N", ConstGenericValue{ .Integer = 10 });
    
    var evaluator = ConstGenericEvaluator.init(allocator, &instantiation);
    
    // Test simple literal
    const literal_expr = ast.Expression{ .Literal = ast.Literal{ .Integer = 42 } };
    const result = try evaluator.evaluateExpression(literal_expr);
    try std.testing.expect(result == .Integer);
    try std.testing.expectEqual(@as(i64, 42), result.Integer);
}
