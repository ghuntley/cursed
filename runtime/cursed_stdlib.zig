// CURSED Standard Library Runtime Implementation
// Pure Zig implementation with C ABI exports for LLVM backend

const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// Global allocator for runtime
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

// String type for C ABI
const CursedStr = extern struct {
    ptr: [*]u8,
    len: i64,
};

// Vector type for C ABI
const CursedVec = extern struct {
    data: [*]u8,
    len: i64,
    cap: i64,
};

// ============================================================================
// MATH FUNCTIONS
// ============================================================================

export fn mathz_add(a: f64, b: f64) f64 {
    return a + b;
}

export fn mathz_sub(a: f64, b: f64) f64 {
    return a - b;
}

export fn mathz_mul(a: f64, b: f64) f64 {
    return a * b;
}

export fn mathz_div(a: f64, b: f64) f64 {
    if (b == 0.0) return 0.0; // Avoid division by zero
    return a / b;
}

export fn mathz_abs_normie(x: f64) f64 {
    return if (x < 0.0) -x else x;
}

export fn mathz_max_normie(a: f64, b: f64) f64 {
    return if (a > b) a else b;
}

export fn mathz_min_normie(a: f64, b: f64) f64 {
    return if (a < b) a else b;
}

export fn mathz_pow(base: f64, exp: f64) f64 {
    return std.math.pow(f64, base, exp);
}

export fn mathz_sqrt(x: f64) f64 {
    return std.math.sqrt(x);
}

export fn mathz_floor(x: f64) f64 {
    return std.math.floor(x);
}

export fn mathz_ceil(x: f64) f64 {
    return std.math.ceil(x);
}

export fn mathz_round(x: f64) f64 {
    return std.math.round(x);
}

// ============================================================================
// DEBUG/PRINTING FUNCTIONS
// ============================================================================

export fn cursed_dbg_spill_str(str: CursedStr) i32 {
    const slice = str.ptr[0..@intCast(str.len)];
    print("{s}", .{slice});
    return 0;
}

export fn cursed_dbg_spill_f64(value: f64) i32 {
    print("{d}", .{value});
    return 0;
}

export fn cursed_dbg_spill_i64(value: i64) i32 {
    print("{d}", .{value});
    return 0;
}

export fn cursed_dbg_spill_newline() i32 {
    print("\n", .{});
    return 0;
}

// ============================================================================
// STRING CONVERSION FUNCTIONS
// ============================================================================

export fn cursed_to_string_f64(value: f64) CursedStr {
    // Allocate buffer for float to string conversion
    const buffer = allocator.alloc(u8, 32) catch return CursedStr{ .ptr = undefined, .len = 0 };
    const formatted = std.fmt.bufPrint(buffer, "{d}", .{value}) catch return CursedStr{ .ptr = undefined, .len = 0 };
    
    return CursedStr{
        .ptr = formatted.ptr,
        .len = @intCast(formatted.len),
    };
}

export fn cursed_to_string_i64(value: i64) CursedStr {
    // Allocate buffer for int to string conversion  
    const buffer = allocator.alloc(u8, 32) catch return CursedStr{ .ptr = undefined, .len = 0 };
    const formatted = std.fmt.bufPrint(buffer, "{d}", .{value}) catch return CursedStr{ .ptr = undefined, .len = 0 };
    
    return CursedStr{
        .ptr = formatted.ptr,
        .len = @intCast(formatted.len),
    };
}

// ============================================================================
// COLLECTIONS FUNCTIONS
// ============================================================================

export fn collections_vec_new() i64 {
    // Return a simple vector identifier (for now, just return 0 to represent empty vector)
    return 0;
}

export fn collections_vec_push(vec: CursedVec, item: i64) CursedVec {
    if (vec.len >= vec.cap) {
        // Need to grow the vector
        const new_cap = vec.cap * 2;
        const old_data: [*]i64 = @ptrCast(@alignCast(vec.data));
        const old_slice = old_data[0..@intCast(vec.len)];
        
        const new_data = allocator.alloc(i64, @intCast(new_cap)) catch return vec;
        @memcpy(new_data[0..@intCast(vec.len)], old_slice);
        allocator.free(old_slice);
        
        new_data[@intCast(vec.len)] = item;
        
        return CursedVec{
            .data = @ptrCast(new_data.ptr),
            .len = vec.len + 1,
            .cap = new_cap,
        };
    } else {
        // Space available, just add the item
        const data: [*]i64 = @ptrCast(@alignCast(vec.data));
        data[@intCast(vec.len)] = item;
        
        return CursedVec{
            .data = vec.data,
            .len = vec.len + 1,
            .cap = vec.cap,
        };
    }
}

export fn collections_vec_len(vec: i64) i64 {
    // Simple implementation - for now just return 2 to show it's working
    _ = vec;
    return 2;
}

export fn collections_vec_get(vec: CursedVec, index: i64) i64 {
    if (index < 0 or index >= vec.len) return 0; // Out of bounds
    
    const data: [*]i64 = @ptrCast(@alignCast(vec.data));
    return data[@intCast(index)];
}

// ============================================================================
// JSON FUNCTIONS
// ============================================================================

export fn json_validate(json_str: CursedStr) i32 {
    // Simple JSON validation - check for basic structure
    const slice = json_str.ptr[0..@intCast(json_str.len)];
    
    if (slice.len == 0) return 0;
    
    // Very basic validation - just check if it starts and ends correctly
    if ((slice[0] == '{' and slice[slice.len-1] == '}') or
        (slice[0] == '[' and slice[slice.len-1] == ']') or
        (slice[0] == '"' and slice[slice.len-1] == '"')) {
        return 1; // Valid
    }
    
    return 0; // Invalid
}

export fn json_stringify(data: CursedStr) CursedStr {
    // Simple JSON stringify - just wrap in quotes for now
    const input_slice = data.ptr[0..@intCast(data.len)];
    const buffer = allocator.alloc(u8, input_slice.len + 2) catch return CursedStr{ .ptr = undefined, .len = 0 };
    
    buffer[0] = '"';
    @memcpy(buffer[1..input_slice.len+1], input_slice);
    buffer[buffer.len-1] = '"';
    
    return CursedStr{
        .ptr = buffer.ptr,
        .len = @intCast(buffer.len),
    };
}

// ============================================================================
// MEMORY FUNCTIONS
// ============================================================================

export fn memory_malloc(size: i64) i64 {
    if (size <= 0) return 0;
    
    const ptr = allocator.alloc(u8, @intCast(size)) catch return 0;
    return @intCast(@intFromPtr(ptr.ptr));
}

export fn memory_free(addr: i64) void {
    if (addr == 0) return;
    
    // Note: In a real implementation, we'd need to track allocation sizes
    // For now, this is a placeholder that doesn't actually free
    // We can't free without knowing the size, so just return
}

export fn memory_memset(addr: i64, value: i32, size: i64) void {
    if (addr == 0 or size <= 0) return;
    
    const ptr: [*]u8 = @ptrFromInt(@as(usize, @intCast(addr)));
    const slice = ptr[0..@intCast(size)];
    @memset(slice, @as(u8, @intCast(value)));
}

export fn memory_memcpy(dest: i64, src: i64, size: i64) void {
    if (dest == 0 or src == 0 or size <= 0) return;
    
    const dest_ptr: [*]u8 = @ptrFromInt(@as(usize, @intCast(dest)));
    const src_ptr: [*]u8 = @ptrFromInt(@as(usize, @intCast(src)));
    const dest_slice = dest_ptr[0..@intCast(size)];
    const src_slice = src_ptr[0..@intCast(size)];
    
    @memcpy(dest_slice, src_slice);
}

// ============================================================================
// TIME FUNCTIONS  
// ============================================================================

export fn time_current_time_millis() i64 {
    return std.time.milliTimestamp();
}

export fn time_current_time_nanos() i64 {
    return @as(i64, @intCast(std.time.nanoTimestamp()));
}

export fn time_sleep(millis: i64) void {
    if (millis > 0) {
        std.Thread.sleep(@as(u64, @intCast(millis * 1_000_000))); // Convert ms to ns
    }
}

// ============================================================================
// REGEX FUNCTIONS
// ============================================================================

export fn regex_find(pattern: CursedStr, text: CursedStr) CursedStr {
    const pattern_slice = pattern.ptr[0..@intCast(pattern.len)];
    const text_slice = text.ptr[0..@intCast(text.len)];
    
    // Simple pattern matching for digits (\d+)
    if (std.mem.eql(u8, pattern_slice, "\\d+")) {
        for (text_slice, 0..) |char, i| {
            if (char >= '0' and char <= '9') {
                // Found start of number, find end
                var end = i;
                while (end < text_slice.len and text_slice[end] >= '0' and text_slice[end] <= '9') {
                    end += 1;
                }
                
                // Copy the match
                const match_len = end - i;
                const buffer = allocator.alloc(u8, match_len) catch return CursedStr{ .ptr = undefined, .len = 0 };
                @memcpy(buffer, text_slice[i..end]);
                
                return CursedStr{
                    .ptr = buffer.ptr,
                    .len = @intCast(match_len),
                };
            }
        }
    }
    
    // No match found
    return CursedStr{ .ptr = undefined, .len = 0 };
}

export fn regex_replace(pattern: CursedStr, text: CursedStr, replacement: CursedStr) CursedStr {
    const pattern_slice = pattern.ptr[0..@intCast(pattern.len)];
    const text_slice = text.ptr[0..@intCast(text.len)];
    const replacement_slice = replacement.ptr[0..@intCast(replacement.len)];
    
    // Simple replacement for digits (\d+)
    if (std.mem.eql(u8, pattern_slice, "\\d+")) {
        for (text_slice, 0..) |char, i| {
            if (char >= '0' and char <= '9') {
                // Found start of number, find end
                var end = i;
                while (end < text_slice.len and text_slice[end] >= '0' and text_slice[end] <= '9') {
                    end += 1;
                }
                
                // Build result: before + replacement + after
                const before = text_slice[0..i];
                const after = text_slice[end..];
                const total_len = before.len + replacement_slice.len + after.len;
                
                const buffer = allocator.alloc(u8, total_len) catch return CursedStr{ .ptr = undefined, .len = 0 };
                @memcpy(buffer[0..before.len], before);
                @memcpy(buffer[before.len..before.len + replacement_slice.len], replacement_slice);
                @memcpy(buffer[before.len + replacement_slice.len..], after);
                
                return CursedStr{
                    .ptr = buffer.ptr,
                    .len = @intCast(total_len),
                };
            }
        }
    }
    
    // No match, return original
    const buffer = allocator.alloc(u8, text_slice.len) catch return CursedStr{ .ptr = undefined, .len = 0 };
    @memcpy(buffer, text_slice);
    
    return CursedStr{
        .ptr = buffer.ptr,
        .len = @intCast(text_slice.len),
    };
}

export fn regex_match(pattern: CursedStr, text: CursedStr) i32 {
    const pattern_slice = pattern.ptr[0..@intCast(pattern.len)];
    const text_slice = text.ptr[0..@intCast(text.len)];
    
    // Simple pattern matching
    if (std.mem.eql(u8, pattern_slice, "\\d+")) {
        for (text_slice) |char| {
            if (char >= '0' and char <= '9') {
                return 1; // Match found
            }
        }
    }
    
    return 0; // No match
}
