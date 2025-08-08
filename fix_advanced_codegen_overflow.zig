// Comprehensive integer overflow fixes for advanced_codegen.zig
// This module provides safe integer operations for LLVM codegen

const std = @import("std");

// Error types for overflow protection
pub const OverflowError = error{
    IntegerOverflow,
    ArrayIndexOverflow,
    CaseCountOverflow,
    FunctionParameterOverflow,
    StringLengthOverflow,
    MemoryOffsetOverflow,
};

// Safe conversion from usize to u32 with overflow checking
pub fn safeUsizeToU32(value: usize) OverflowError!u32 {
    if (value > std.math.maxInt(u32)) {
        return OverflowError.IntegerOverflow;
    }
    return @intCast(value);
}

// Safe conversion from any integer type to u32
pub fn safeIntToU32(value: anytype) OverflowError!u32 {
    const ValueType = @TypeOf(value);
    const value_info = @typeInfo(ValueType);
    
    if (value_info != .int) {
        return OverflowError.IntegerOverflow;
    }
    
    if (value < 0 or value > std.math.maxInt(u32)) {
        return OverflowError.IntegerOverflow;
    }
    
    return @intCast(value);
}

// Safe conversion from any integer type to u64
pub fn safeIntToU64(value: anytype) OverflowError!u64 {
    const ValueType = @TypeOf(value);
    const value_info = @typeInfo(ValueType);
    
    if (value_info != .int) {
        return OverflowError.IntegerOverflow;
    }
    
    if (value < 0 or value > std.math.maxInt(u64)) {
        return OverflowError.IntegerOverflow;
    }
    
    return @intCast(value);
}

// Safe array index validation
pub fn validateArrayIndex(index: anytype, array_len: usize) OverflowError!u32 {
    // First check if the index is within the array bounds using the original type
    if (index < 0 or index >= array_len) {
        return OverflowError.ArrayIndexOverflow;
    }
    // Then safely convert to u32
    const safe_index = try safeIntToU32(index);
    return safe_index;
}

// Safe case count validation for switch statements
pub fn validateCaseCount(cases_len: usize) OverflowError!u32 {
    // Reasonable limit for switch cases (64K cases should be enough)
    const MAX_SWITCH_CASES = 65536;
    if (cases_len > MAX_SWITCH_CASES) {
        return OverflowError.CaseCountOverflow;
    }
    return safeUsizeToU32(cases_len);
}

// Safe function parameter count validation
pub fn validateParameterCount(param_count: usize) OverflowError!u32 {
    // Reasonable limit for function parameters (1K params should be enough)
    const MAX_FUNCTION_PARAMS = 1024;
    if (param_count > MAX_FUNCTION_PARAMS) {
        return OverflowError.FunctionParameterOverflow;
    }
    return safeUsizeToU32(param_count);
}

// Safe string length validation
pub fn validateStringLength(str_len: usize) OverflowError!u64 {
    // Reasonable limit for string literals (1GB should be enough)
    const MAX_STRING_LENGTH = 1024 * 1024 * 1024;
    if (str_len > MAX_STRING_LENGTH) {
        return OverflowError.StringLengthOverflow;
    }
    return @intCast(str_len);
}

// Safe memory offset validation
pub fn validateMemoryOffset(offset: anytype) OverflowError!u64 {
    const safe_offset = try safeIntToU64(offset);
    // Reasonable limit for memory offsets (32GB should be enough)
    const MAX_MEMORY_OFFSET = 32 * 1024 * 1024 * 1024;
    if (safe_offset > MAX_MEMORY_OFFSET) {
        return OverflowError.MemoryOffsetOverflow;
    }
    return safe_offset;
}

// Test functions to ensure overflow detection works
test "Safe integer conversions" {
    const testing = std.testing;
    
    // Test usize to u32 conversion
    const large_usize: usize = 0xFFFFFFFF + 1;
    const result = safeUsizeToU32(large_usize);
    try testing.expectError(OverflowError.IntegerOverflow, result);
    
    // Test valid conversion
    const valid_usize: usize = 1000;
    const valid_result = try safeUsizeToU32(valid_usize);
    try testing.expect(valid_result == 1000);
}

test "Array index validation" {
    const testing = std.testing;
    
    // Test out of bounds index (array length exceeded)
    const result = validateArrayIndex(100, 50);
    try testing.expectError(OverflowError.ArrayIndexOverflow, result);
    
    // Test valid index within u32 range
    const valid_result = try validateArrayIndex(@as(u32, 25), 50);
    try testing.expect(valid_result == 25);
}

test "Case count validation" {
    const testing = std.testing;
    
    // Test too many cases
    const large_cases: usize = 100000;
    const result = validateCaseCount(large_cases);
    try testing.expectError(OverflowError.CaseCountOverflow, result);
    
    // Test valid case count
    const valid_cases: usize = 100;
    const valid_result = try validateCaseCount(valid_cases);
    try testing.expect(valid_result == 100);
}
