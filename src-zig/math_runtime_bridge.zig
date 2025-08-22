// =============================================================================
// CURSED Math Runtime Bridge - IEEE 754 Floating Point Operations
// Provides native floating-point arithmetic with full IEEE 754 compliance
// =============================================================================

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const math = std.math;

// IEEE 754 double precision floating point support
const Float64 = f64;
const Float32 = f32;

// String to float conversion with error handling
fn parseFloat(str: []const u8) !Float64 {
    return std.fmt.parseFloat(Float64, str) catch |err| switch (err) {
        error.InvalidCharacter => math.nan(Float64),
        else => math.nan(Float64),
    };
}

// Float to string conversion with precision
fn formatFloat(allocator: Allocator, value: Float64) ![]const u8 {
    if (math.isNan(value)) {
        return try allocator.dupe(u8, "NaN");
    }
    if (math.isPositiveInf(value)) {
        return try allocator.dupe(u8, "Infinity");
    }
    if (math.isNegativeInf(value)) {
        return try allocator.dupe(u8, "-Infinity");
    }
    
    // Format with 15 decimal places for double precision
    return try std.fmt.allocPrint(allocator, "{d:.15}", .{value});
}

// IEEE 754 compliant addition
pub fn runtime_float_add(allocator: Allocator, a_str: []const u8, b_str: []const u8) ![]const u8 {
    const a = parseFloat(a_str) catch return try allocator.dupe(u8, "NaN");
    const b = parseFloat(b_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = a + b;
    return formatFloat(allocator, result);
}

// IEEE 754 compliant subtraction
pub fn runtime_float_subtract(allocator: Allocator, a_str: []const u8, b_str: []const u8) ![]const u8 {
    const a = parseFloat(a_str) catch return try allocator.dupe(u8, "NaN");
    const b = parseFloat(b_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = a - b;
    return formatFloat(allocator, result);
}

// IEEE 754 compliant multiplication
pub fn runtime_float_multiply(allocator: Allocator, a_str: []const u8, b_str: []const u8) ![]const u8 {
    const a = parseFloat(a_str) catch return try allocator.dupe(u8, "NaN");
    const b = parseFloat(b_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = a * b;
    return formatFloat(allocator, result);
}

// IEEE 754 compliant division
pub fn runtime_float_divide(allocator: Allocator, a_str: []const u8, b_str: []const u8) ![]const u8 {
    const a = parseFloat(a_str) catch return try allocator.dupe(u8, "NaN");
    const b = parseFloat(b_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = a / b;
    return formatFloat(allocator, result);
}

// Check if float is zero
pub fn runtime_float_is_zero(value_str: []const u8) bool {
    const value = parseFloat(value_str) catch return false;
    return value == 0.0;
}

// Check if float is negative
pub fn runtime_float_is_negative(value_str: []const u8) bool {
    const value = parseFloat(value_str) catch return false;
    return math.signbit(value);
}

// Absolute value
pub fn runtime_float_abs(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.fabs(value);
    return formatFloat(allocator, result);
}

// Factorial computation with high precision
pub fn runtime_factorial(allocator: Allocator, n: i64) ![]const u8 {
    if (n < 0) {
        return try allocator.dupe(u8, "NaN");
    }
    if (n > 170) {
        // Factorial(171) overflows double precision
        return try allocator.dupe(u8, "Infinity");
    }
    
    var result: Float64 = 1.0;
    var i: i64 = 2;
    while (i <= n) : (i += 1) {
        result *= @as(Float64, @floatFromInt(i));
    }
    
    return formatFloat(allocator, result);
}

// Integer to float conversion
pub fn runtime_int_to_float(allocator: Allocator, n: i64) ![]const u8 {
    const result = @as(Float64, @floatFromInt(n));
    return formatFloat(allocator, result);
}

// Floor function
pub fn runtime_float_floor(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.floor(value);
    return formatFloat(allocator, result);
}

// Ceiling function
pub fn runtime_float_ceil(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.ceil(value);
    return formatFloat(allocator, result);
}

// Round function
pub fn runtime_float_round(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.round(value);
    return formatFloat(allocator, result);
}

// Truncate function
pub fn runtime_float_trunc(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.trunc(value);
    return formatFloat(allocator, result);
}

// Next representable float after x in direction of y
pub fn runtime_float_next_after(allocator: Allocator, x_str: []const u8, y_str: []const u8) ![]const u8 {
    const x = parseFloat(x_str) catch return try allocator.dupe(u8, "NaN");
    const y = parseFloat(y_str) catch return try allocator.dupe(u8, "NaN");
    
    // Zig doesn't have nextafter builtin, implement manually
    if (math.isNan(x) or math.isNan(y)) {
        return try allocator.dupe(u8, "NaN");
    }
    if (x == y) {
        return try allocator.dupe(u8, y_str);
    }
    
    // Use bit manipulation for nextafter implementation
    const x_bits = @as(u64, @bitCast(x));
    var result_bits: u64 = undefined;
    
    if (x == 0.0) {
        // Smallest positive or negative subnormal
        result_bits = if (y > 0.0) 1 else 0x8000000000000001;
    } else if ((x > 0.0) == (y > x)) {
        // Same sign and y is further from zero
        result_bits = x_bits + 1;
    } else {
        // Opposite signs or y is closer to zero
        result_bits = x_bits - 1;
    }
    
    const result = @as(Float64, @bitCast(result_bits));
    return formatFloat(allocator, result);
}

// Unit in Last Place (ULP) - precision measure
pub fn runtime_float_ulp(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (math.isNan(value)) {
        return try allocator.dupe(u8, "NaN");
    }
    if (math.isInf(value)) {
        return try allocator.dupe(u8, "Infinity");
    }
    
    const abs_value = math.fabs(value);
    if (abs_value == 0.0) {
        // Smallest subnormal number
        const result = math.floatMin(Float64);
        return formatFloat(allocator, result);
    }
    
    // Get next representable value and subtract
    const next_val_str = try runtime_float_next_after(allocator, value_str, 
        if (value >= 0.0) "Infinity" else "-Infinity");
    defer allocator.free(next_val_str);
    
    const next_val = parseFloat(next_val_str) catch return try allocator.dupe(u8, "NaN");
    const ulp_val = math.fabs(next_val - value);
    return formatFloat(allocator, ulp_val);
}

// Copy sign from one float to another
pub fn runtime_float_copy_sign(allocator: Allocator, magnitude_str: []const u8, sign_str: []const u8) ![]const u8 {
    const magnitude = parseFloat(magnitude_str) catch return try allocator.dupe(u8, "NaN");
    const sign_value = parseFloat(sign_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = math.copysign(magnitude, sign_value);
    return formatFloat(allocator, result);
}

// Floating point remainder (IEEE 754 remainder)
pub fn runtime_float_mod(allocator: Allocator, x_str: []const u8, y_str: []const u8) ![]const u8 {
    const x = parseFloat(x_str) catch return try allocator.dupe(u8, "NaN");
    const y = parseFloat(y_str) catch return try allocator.dupe(u8, "NaN");
    
    // IEEE 754 remainder: x - n*y where n = round(x/y)
    if (math.isNan(x) or math.isNan(y) or math.isInf(x) or y == 0.0) {
        return try allocator.dupe(u8, "NaN");
    }
    if (math.isInf(y)) {
        return try allocator.dupe(u8, x_str);
    }
    
    const quotient = x / y;
    const n = math.round(quotient);
    const result = x - n * y;
    
    return formatFloat(allocator, result);
}

// Check if two floats are close to each other within epsilon
pub fn runtime_float_close_to(a_str: []const u8, b_str: []const u8, epsilon_str: []const u8) bool {
    const a = parseFloat(a_str) catch return false;
    const b = parseFloat(b_str) catch return false;
    const epsilon = parseFloat(epsilon_str) catch return false;
    
    if (math.isNan(a) or math.isNan(b) or math.isNan(epsilon)) {
        return false;
    }
    
    return math.fabs(a - b) <= epsilon;
}

// Check if a float represents an integer
pub fn runtime_float_is_integer(value_str: []const u8) bool {
    const value = parseFloat(value_str) catch return false;
    
    if (math.isNan(value) or math.isInf(value)) {
        return false;
    }
    
    return value == math.trunc(value);
}

// Check if a float represents an odd integer
pub fn runtime_float_is_odd_integer(value_str: []const u8) bool {
    if (!runtime_float_is_integer(value_str)) {
        return false;
    }
    
    const value = parseFloat(value_str) catch return false;
    const int_value = @as(i64, @intFromFloat(math.trunc(value)));
    return @mod(int_value, 2) == 1;
}

// Negate a float
pub fn runtime_float_negate(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = -value;
    return formatFloat(allocator, result);
}

// Greater than comparison
pub fn runtime_float_greater_than(a_str: []const u8, b_str: []const u8) bool {
    const a = parseFloat(a_str) catch return false;
    const b = parseFloat(b_str) catch return false;
    
    if (math.isNan(a) or math.isNan(b)) {
        return false; // NaN comparisons are always false
    }
    
    return a > b;
}

// Less than comparison
pub fn runtime_float_less_than(a_str: []const u8, b_str: []const u8) bool {
    const a = parseFloat(a_str) catch return false;
    const b = parseFloat(b_str) catch return false;
    
    if (math.isNan(a) or math.isNan(b)) {
        return false;
    }
    
    return a < b;
}

// Less than or equal comparison
pub fn runtime_float_less_than_or_equal(a_str: []const u8, b_str: []const u8) bool {
    const a = parseFloat(a_str) catch return false;
    const b = parseFloat(b_str) catch return false;
    
    if (math.isNan(a) or math.isNan(b)) {
        return false;
    }
    
    return a <= b;
}

// Equal comparison
pub fn runtime_float_equal(a_str: []const u8, b_str: []const u8) bool {
    const a = parseFloat(a_str) catch return false;
    const b = parseFloat(b_str) catch return false;
    
    if (math.isNan(a) or math.isNan(b)) {
        return false; // NaN is not equal to anything, including itself
    }
    
    return a == b;
}

// Check if float is positive
pub fn runtime_float_is_positive(value_str: []const u8) bool {
    const value = parseFloat(value_str) catch return false;
    return value > 0.0 and !math.isNan(value);
}

// Advanced mathematical functions using standard library

// Sine function with high precision
pub fn runtime_sin(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.sin(value);
    return formatFloat(allocator, result);
}

// Cosine function with high precision
pub fn runtime_cos(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.cos(value);
    return formatFloat(allocator, result);
}

// Tangent function with high precision
pub fn runtime_tan(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.tan(value);
    return formatFloat(allocator, result);
}

// Natural exponential function
pub fn runtime_exp(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.exp(value);
    return formatFloat(allocator, result);
}

// Natural logarithm
pub fn runtime_ln(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (value < 0.0) {
        return try allocator.dupe(u8, "NaN");
    }
    if (value == 0.0) {
        return try allocator.dupe(u8, "-Infinity");
    }
    
    const result = math.log(value);
    return formatFloat(allocator, result);
}

// Base-10 logarithm
pub fn runtime_log10(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (value < 0.0) {
        return try allocator.dupe(u8, "NaN");
    }
    if (value == 0.0) {
        return try allocator.dupe(u8, "-Infinity");
    }
    
    const result = math.log10(value);
    return formatFloat(allocator, result);
}

// Base-2 logarithm
pub fn runtime_log2(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (value < 0.0) {
        return try allocator.dupe(u8, "NaN");
    }
    if (value == 0.0) {
        return try allocator.dupe(u8, "-Infinity");
    }
    
    const result = math.log2(value);
    return formatFloat(allocator, result);
}

// Power function
pub fn runtime_pow(allocator: Allocator, base_str: []const u8, exp_str: []const u8) ![]const u8 {
    const base = parseFloat(base_str) catch return try allocator.dupe(u8, "NaN");
    const exp = parseFloat(exp_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = math.pow(Float64, base, exp);
    return formatFloat(allocator, result);
}

// Square root
pub fn runtime_sqrt(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (value < 0.0) {
        return try allocator.dupe(u8, "NaN");
    }
    
    const result = math.sqrt(value);
    return formatFloat(allocator, result);
}

// Hyperbolic sine
pub fn runtime_sinh(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.sinh(value);
    return formatFloat(allocator, result);
}

// Hyperbolic cosine
pub fn runtime_cosh(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.cosh(value);
    return formatFloat(allocator, result);
}

// Hyperbolic tangent
pub fn runtime_tanh(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.tanh(value);
    return formatFloat(allocator, result);
}

// Arc sine
pub fn runtime_asin(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (value < -1.0 or value > 1.0) {
        return try allocator.dupe(u8, "NaN");
    }
    
    const result = math.asin(value);
    return formatFloat(allocator, result);
}

// Arc cosine
pub fn runtime_acos(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    
    if (value < -1.0 or value > 1.0) {
        return try allocator.dupe(u8, "NaN");
    }
    
    const result = math.acos(value);
    return formatFloat(allocator, result);
}

// Arc tangent
pub fn runtime_atan(allocator: Allocator, value_str: []const u8) ![]const u8 {
    const value = parseFloat(value_str) catch return try allocator.dupe(u8, "NaN");
    const result = math.atan(value);
    return formatFloat(allocator, result);
}

// Two-argument arc tangent
pub fn runtime_atan2(allocator: Allocator, y_str: []const u8, x_str: []const u8) ![]const u8 {
    const y = parseFloat(y_str) catch return try allocator.dupe(u8, "NaN");
    const x = parseFloat(x_str) catch return try allocator.dupe(u8, "NaN");
    
    const result = math.atan2(Float64, y, x);
    return formatFloat(allocator, result);
}

// Test all IEEE 754 compliance
pub fn test_ieee754_bridge() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test basic arithmetic
    const add_result = try runtime_float_add(allocator, "1.5", "2.5");
    defer allocator.free(add_result);
    std.testing.expect(runtime_float_close_to(add_result, "4.0", "1e-10")) catch unreachable;
    
    // Test NaN propagation
    const nan_result = try runtime_float_add(allocator, "NaN", "1.0");
    defer allocator.free(nan_result);
    std.testing.expectEqualStrings("NaN", nan_result) catch unreachable;
    
    // Test infinity arithmetic
    const inf_result = try runtime_float_divide(allocator, "1.0", "0.0");
    defer allocator.free(inf_result);
    std.testing.expectEqualStrings("Infinity", inf_result) catch unreachable;
    
    // Test transcendental functions
    const sin_result = try runtime_sin(allocator, "1.5707963267948966"); // π/2
    defer allocator.free(sin_result);
    std.testing.expect(runtime_float_close_to(sin_result, "1.0", "1e-12")) catch unreachable;
    
    std.debug.print("All IEEE 754 compliance tests passed!\n");
}

// =============================================================================
// END OF IEEE 754 RUNTIME BRIDGE
// Full double precision floating point with special value handling
// =============================================================================
