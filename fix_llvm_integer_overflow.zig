// LLVM Integer Overflow Bug Fixes
// This file contains fixes for integer overflow issues in LLVM codegen

const std = @import("std");

// Safe integer casting with overflow detection
pub fn safeIntCast(comptime T: type, value: anytype) !T {
    const ValueType = @TypeOf(value);
    const value_info = @typeInfo(ValueType);
    const target_info = @typeInfo(T);
    
    if (value_info != .int or target_info != .int) {
        return error.InvalidType;
    }
    
    const min_val = std.math.minInt(T);
    const max_val = std.math.maxInt(T);
    
    if (value < min_val or value > max_val) {
        return error.IntegerOverflow;
    }
    
    return @intCast(value);
}

// Safe truncation with bounds checking
pub fn safeTruncate(comptime T: type, value: anytype) !T {
    const ValueType = @TypeOf(value);
    if (@typeInfo(ValueType) != .int or @typeInfo(T) != .int) {
        return error.InvalidType;
    }
    
    const max_val = std.math.maxInt(T);
    if (value > max_val) {
        return error.IntegerOverflow;
    }
    
    return @truncate(value);
}

// Safe arithmetic operations with overflow detection
pub fn safeAdd(comptime T: type, a: T, b: T) !T {
    return std.math.add(T, a, b) catch error.ArithmeticOverflow;
}

pub fn safeSub(comptime T: type, a: T, b: T) !T {
    return std.math.sub(T, a, b) catch error.ArithmeticOverflow;
}

pub fn safeMul(comptime T: type, a: T, b: T) !T {
    return std.math.mul(T, a, b) catch error.ArithmeticOverflow;
}

// Safe array length handling
pub fn safeArrayLength(length: anytype) !u32 {
    const LengthType = @TypeOf(length);
    if (@typeInfo(LengthType) != .int) {
        return error.InvalidType;
    }
    
    // Check for reasonable array size limits
    const MAX_ARRAY_SIZE = 0x7FFFFFFF; // 2^31 - 1
    if (length < 0 or length > MAX_ARRAY_SIZE) {
        return error.ArraySizeOverflow;
    }
    
    return @intCast(length);
}

// Safe string length handling  
pub fn safeStringLength(length: anytype) !usize {
    const LengthType = @TypeOf(length);
    if (@typeInfo(LengthType) != .int) {
        return error.InvalidType;
    }
    
    // Check for reasonable string size limits (1GB max)
    const MAX_STRING_SIZE = 1024 * 1024 * 1024;
    if (length < 0 or length > MAX_STRING_SIZE) {
        return error.StringSizeOverflow;
    }
    
    return @intCast(length);
}

// Safe counter operations with overflow protection
pub const SafeCounter = struct {
    value: i64,
    max_value: i64,
    min_value: i64,
    
    pub fn init(initial: i64, min_val: i64, max_val: i64) SafeCounter {
        return SafeCounter{
            .value = initial,
            .max_value = max_val,
            .min_value = min_val,
        };
    }
    
    pub fn increment(self: *SafeCounter) !void {
        if (self.value >= self.max_value) {
            return error.CounterOverflow;
        }
        self.value += 1;
    }
    
    pub fn decrement(self: *SafeCounter) !void {
        if (self.value <= self.min_value) {
            return error.CounterUnderflow;
        }
        self.value -= 1;
    }
    
    pub fn add(self: *SafeCounter, amount: i64) !void {
        const new_value = try safeAdd(i64, self.value, amount);
        if (new_value > self.max_value) {
            return error.CounterOverflow;
        }
        self.value = new_value;
    }
    
    pub fn getValue(self: *const SafeCounter) i64 {
        return self.value;
    }
    
    pub fn getValueAsU32(self: *const SafeCounter) !u32 {
        return safeIntCast(u32, self.value);
    }
};

// Test function to validate overflow detection
test "Safe integer operations" {
    const testing = std.testing;
    
    // Test safe casting
    const large_val: i64 = 3000000000;
    const result = safeIntCast(i32, large_val);
    try testing.expectError(error.IntegerOverflow, result);
    
    // Test safe arithmetic
    const add_result = safeAdd(i32, 2000000000, 1000000000);
    try testing.expectError(error.ArithmeticOverflow, add_result);
    
    // Test safe counter
    var counter = SafeCounter.init(0, -10, 10);
    try counter.add(5);
    try testing.expect(counter.getValue() == 5);
    
    const overflow_result = counter.add(10);
    try testing.expectError(error.CounterOverflow, overflow_result);
}
