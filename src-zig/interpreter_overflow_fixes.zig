// CURSED Interpreter Integer Overflow Protection
// This file provides comprehensive overflow protection for the CURSED interpreter

const std = @import("std");
const Allocator = std.mem.Allocator;

/// Safe arithmetic operations with overflow protection
pub const SafeArithmetic = struct {
    
    /// Safe addition with overflow detection
    pub fn safeAdd(comptime T: type, a: T, b: T) ?T {
        return std.math.add(T, a, b) catch null;
    }
    
    /// Safe subtraction with overflow detection
    pub fn safeSub(comptime T: type, a: T, b: T) ?T {
        return std.math.sub(T, a, b) catch null;
    }
    
    /// Safe multiplication with overflow detection
    pub fn safeMul(comptime T: type, a: T, b: T) ?T {
        return std.math.mul(T, a, b) catch null;
    }
    
    /// Safe division with zero check
    pub fn safeDiv(comptime T: type, a: T, b: T) ?T {
        if (b == 0) return null;
        return @divTrunc(a, b);
    }
    
    /// Safe modulo with zero check
    pub fn safeMod(comptime T: type, a: T, b: T) ?T {
        if (b == 0) return null;
        return @mod(a, b);
    }
    
    /// Safe cast to usize with bounds checking
    pub fn safeToUsize(value: anytype) ?usize {
        const T = @TypeOf(value);
        if (T == usize) return value;
        
        if (@typeInfo(T).Int.signedness == .signed) {
            if (value < 0) return null;
        }
        
        if (value > std.math.maxInt(usize)) return null;
        return @intCast(value);
    }
    
    /// Safe array length validation
    pub fn validateArrayLength(len: anytype) bool {
        const actual_len = safeToUsize(len) orelse return false;
        // Reasonable maximum array length (1GB worth of pointers)
        const max_reasonable_len = 1024 * 1024 * 1024 / @sizeOf(usize);
        return actual_len <= max_reasonable_len;
    }
};

/// Safe memory operations with bounds checking
pub const SafeMemory = struct {
    
    /// Safe slice creation with length validation
    pub fn safeCreateSlice(comptime T: type, allocator: Allocator, len: anytype) ![]T {
        const safe_len = SafeArithmetic.safeToUsize(len) orelse {
            std.debug.print("ERROR: Invalid slice length: {}\n", .{len});
            return error.InvalidLength;
        };
        
        if (!SafeArithmetic.validateArrayLength(safe_len)) {
            std.debug.print("ERROR: Array length too large: {}\n", .{safe_len});
            return error.ArrayTooLarge;
        }
        
        return try allocator.alloc(T, safe_len);
    }
    
    /// Safe slice deallocation with validation
    pub fn safeFreeSlice(comptime T: type, allocator: Allocator, slice: []T) void {
        // Validate slice before freeing
        if (slice.len > 0 and slice.ptr != @ptrFromInt(0)) {
            // Valid slice with non-null pointer and length
        } else if (slice.len > 0) {
            std.debug.print("WARNING: Attempting to free null pointer with non-zero length\n", .{});
            return;
        }
        
        if (!SafeArithmetic.validateArrayLength(slice.len)) {
            std.debug.print("WARNING: Invalid slice length during free: {}\n", .{slice.len});
            return;
        }
        
        // Additional safety: check if length is reasonable
        if (slice.len > 0) {
            allocator.free(slice);
        }
    }
    
    /// Safe memory copy with bounds checking
    pub fn safeMemCopy(dst: []u8, src: []const u8) !void {
        if (dst.len < src.len) {
            return error.DestinationTooSmall;
        }
        
        if (!SafeArithmetic.validateArrayLength(src.len)) {
            return error.InvalidSourceLength;
        }
        
        @memcpy(dst[0..src.len], src);
    }
};

/// Safe string operations
pub const SafeString = struct {
    
    /// Safe string duplication with length validation
    pub fn safeDupe(allocator: Allocator, str: []const u8) ![]u8 {
        if (!SafeArithmetic.validateArrayLength(str.len)) {
            std.debug.print("ERROR: String too long to duplicate: {}\n", .{str.len});
            return error.StringTooLong;
        }
        
        return try allocator.dupe(u8, str);
    }
    
    /// Safe string formatting with overflow protection
    pub fn safeAllocPrint(allocator: Allocator, comptime fmt: []const u8, args: anytype) ![]u8 {
        return std.fmt.allocPrint(allocator, fmt, args) catch |err| switch (err) {
            error.OutOfMemory => return err,
        };
    }
};

/// Error handling utilities with overflow protection
pub const SafeErrorHandling = struct {
    
    /// Create error context safely
    pub fn createErrorContext(allocator: Allocator, message: []const u8, error_type: anytype) !*anyopaque {
        const safe_message = try SafeString.safeDupe(allocator, message);
        // Additional error context creation logic would go here
        _ = safe_message;
        _ = error_type;
        return undefined; // Placeholder
    }
    
    /// Safe error propagation without arithmetic in error paths
    pub fn propagateError(error_value: anytype) void {
        // Log error without doing arithmetic that could overflow
        std.debug.print("ERROR: Propagating error\n", .{});
        _ = error_value;
    }
};

/// Safe array operations
pub const SafeArray = struct {
    
    /// Safe array access with bounds checking
    pub fn safeGet(comptime T: type, array: []T, index: anytype) ?T {
        const safe_index = SafeArithmetic.safeToUsize(index) orelse return null;
        if (safe_index >= array.len) return null;
        return array[safe_index];
    }
    
    /// Safe array modification with bounds checking
    pub fn safeSet(comptime T: type, array: []T, index: anytype, value: T) bool {
        const safe_index = SafeArithmetic.safeToUsize(index) orelse return false;
        if (safe_index >= array.len) return false;
        array[safe_index] = value;
        return true;
    }
    
    /// Safe array resize with overflow protection
    pub fn safeResize(comptime T: type, allocator: Allocator, array: []T, new_len: anytype) ![]T {
        const safe_new_len = SafeArithmetic.safeToUsize(new_len) orelse {
            return error.InvalidLength;
        };
        
        if (!SafeArithmetic.validateArrayLength(safe_new_len)) {
            return error.ArrayTooLarge;
        }
        
        const new_array = try allocator.alloc(T, safe_new_len);
        const copy_len = @min(array.len, new_array.len);
        
        // Safe copy without overflow
        for (0..copy_len) |i| {
            new_array[i] = array[i];
        }
        
        allocator.free(array);
        return new_array;
    }
};

/// Integration helpers for the interpreter
pub const InterpreterSafety = struct {
    
    /// Validate Value structure integrity
    pub fn validateValue(value: *const anyopaque) bool {
        // Basic sanity checks for Value structures
        const ptr_val = @intFromPtr(value);
        
        // Check if pointer is in reasonable range (not null, not corrupted)
        if (ptr_val == 0) return false;
        if (ptr_val < 4096) return false; // Likely null pointer dereference
        if (ptr_val > 0x7FFFFFFFFFFF) return false; // Unreasonable high address
        
        return true;
    }
    
    /// Safe environment lookup with corruption detection
    pub fn safeEnvironmentLookup(env: *anyopaque, name: []const u8) bool {
        if (!validateValue(env)) {
            std.debug.print("ERROR: Corrupted environment during lookup\n", .{});
            return false;
        }
        
        if (!SafeArithmetic.validateArrayLength(name.len)) {
            std.debug.print("ERROR: Invalid variable name length\n", .{});
            return false;
        }
        
        return true;
    }
    
    /// Safe function call with stack overflow protection
    pub fn safeFunctionCall(stack_depth: usize, max_depth: usize) bool {
        return stack_depth < max_depth;
    }
};
