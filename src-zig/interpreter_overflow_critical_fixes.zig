// CURSED Interpreter Critical Overflow Fixes
// This file provides targeted fixes for immediate integer overflow panics

const std = @import("std");

/// Critical overflow protection for memory operations
pub fn safeArrayLengthCheck(len: anytype) bool {
    const T = @TypeOf(len);
    if (T == usize) {
        // Check for reasonable maximum array length (1GB worth of pointers)
        const max_reasonable_len = 1024 * 1024 * 1024 / @sizeOf(usize);
        return len <= max_reasonable_len;
    }
    
    // Convert to usize safely
    if (@typeInfo(T).Int.signedness == .signed and len < 0) {
        return false;
    }
    
    if (len > std.math.maxInt(usize)) {
        return false;
    }
    
    const usize_len: usize = @intCast(len);
    const max_reasonable_len = 1024 * 1024 * 1024 / @sizeOf(usize);
    return usize_len <= max_reasonable_len;
}

/// Safe integer operations with overflow protection
pub fn safeAddI64(a: i64, b: i64) ?i64 {
    return std.math.add(i64, a, b) catch null;
}

pub fn safeSubI64(a: i64, b: i64) ?i64 {
    return std.math.sub(i64, a, b) catch null;
}

pub fn safeMulI64(a: i64, b: i64) ?i64 {
    return std.math.mul(i64, a, b) catch null;
}

pub fn safeDivI64(a: i64, b: i64) ?i64 {
    if (b == 0) return null;
    return @divTrunc(a, b);
}

pub fn safeModI64(a: i64, b: i64) ?i64 {
    if (b == 0) return null;
    return @mod(a, b);
}

/// Basic pointer validation
pub fn isValidPointer(ptr: *const anyopaque) bool {
    const ptr_val = @intFromPtr(ptr);
    
    // Check if pointer is in reasonable range (not null, not corrupted)
    if (ptr_val == 0) return false;
    if (ptr_val < 4096) return false; // Likely null pointer dereference
    if (ptr_val > 0x7FFFFFFFFFFF) return false; // Unreasonable high address
    
    return true;
}
