const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// ========== RUNTIME FUNCTION IMPLEMENTATIONS ==========
// 
// These functions provide the runtime bridge between CURSED stdlib
// modules and Zig system operations. They implement the core runtime
// functions that stdlib modules call via FFI.

// === STRING RUNTIME FUNCTIONS ===

pub fn runtime_string_char_at(s: []const u8, index: i64) u8 {
    if (index < 0 or index >= s.len) return 0;
    return s[@intCast(index)];
}

pub fn runtime_char_to_string(allocator: Allocator, c: u8) ![]u8 {
    const result = try allocator.alloc(u8, 1);
    result[0] = c;
    return result;
}

pub fn runtime_char_to_ascii(c: u8) i64 {
    return @as(i64, c);
}

pub fn runtime_ascii_to_char(ascii: i64) u8 {
    return @as(u8, @intCast(ascii));
}

pub fn runtime_string_length(s: []const u8) i64 {
    return @as(i64, @intCast(s.len));
}

pub fn len_str(s: []const u8) i64 {
    return runtime_string_length(s);
}

pub fn substring(allocator: Allocator, s: []const u8, start: i64, length: i64) ![]u8 {
    if (start < 0 or start >= s.len or length <= 0) {
        return allocator.dupe(u8, "");
    }
    
    const start_idx: usize = @intCast(start);
    const end_idx = @min(start_idx + @as(usize, @intCast(length)), s.len);
    
    return allocator.dupe(u8, s[start_idx..end_idx]);
}

pub fn string_contains(s: []const u8, substr: []const u8) bool {
    return std.mem.indexOf(u8, s, substr) != null;
}

pub fn string_concat(allocator: Allocator, a: []const u8, b: []const u8) ![]u8 {
    return std.fmt.allocPrint(allocator, "{s}{s}", .{ a, b });
}

// === ARRAY RUNTIME FUNCTIONS ===

// Note: These work with generic Variable arrays
pub const Variable = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Array: ArrayList(Variable),
    
    pub fn deinit(self: *Variable, allocator: Allocator) void {
        switch (self.*) {
            .String => |str| allocator.free(str),
            .Array => |*arr| arr.deinit(),
            else => {},
        }
    }
    
    pub fn clone(self: Variable, allocator: Allocator) !Variable {
        return switch (self) {
            .Integer => |val| Variable{ .Integer = val },
            .Float => |val| Variable{ .Float = val },
            .String => |str| Variable{ .String = try allocator.dupe(u8, str) },
            .Boolean => |val| Variable{ .Boolean = val },
            .Array => |arr| {
                var new_array = ArrayList(Variable).init(allocator);
                for (arr.items) |item| {
                    try new_array.append(try item.clone(allocator));
                }
                return Variable{ .Array = new_array };
            },
        };
    }
};

pub fn array_length(array: []const Variable) i64 {
    return @intCast(array.len);
}

pub fn array_push(array: *ArrayList(Variable), item: Variable) !void {
    try array.append(item);
}

pub fn array_pop(array: *ArrayList(Variable)) ?Variable {
    return array.popOrNull();
}

pub fn array_sort(allocator: Allocator, array: *ArrayList(Variable)) !void {
    // Simple sorting for integers and floats
    std.mem.sort(Variable, array.items, {}, compareVariables);
}

fn compareVariables(_: void, a: Variable, b: Variable) bool {
    return switch (a) {
        .Integer => |a_val| switch (b) {
            .Integer => |b_val| a_val < b_val,
            else => false,
        },
        .Float => |a_val| switch (b) {
            .Float => |b_val| a_val < b_val,
            else => false,
        },
        else => false,
    };
}

// === MATH RUNTIME FUNCTIONS ===

pub fn power(base: f64, exponent: f64) f64 {
    return std.math.pow(f64, base, exponent);
}

pub fn sqrt(value: f64) f64 {
    return @sqrt(value);
}

pub fn sin(value: f64) f64 {
    return @sin(value);
}

pub fn cos(value: f64) f64 {
    return @cos(value);
}

pub fn random() f64 {
    var prng = std.rand.DefaultPrng.init(@intCast(std.time.milliTimestamp()));
    return prng.random().float(f64);
}

pub fn runtime_abs_normie(value: i64) i64 {
    return if (value < 0) -value else value;
}

pub fn runtime_abs_meal(value: f64) f64 {
    return @abs(value);
}

// === FILE RUNTIME FUNCTIONS ===

pub fn read_file(allocator: Allocator, filename: []const u8) ![]u8 {
    return std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024);
}

pub fn write_file(filename: []const u8, content: []const u8) !bool {
    const file = std.fs.cwd().createFile(filename, .{}) catch return false;
    defer file.close();
    file.writeAll(content) catch return false;
    return true;
}

pub fn file_exists(filename: []const u8) bool {
    std.fs.cwd().access(filename, .{}) catch return false;
    return true;
}

pub fn runtime_delete_file(filename: []const u8) bool {
    std.fs.cwd().deleteFile(filename) catch return false;
    return true;
}

pub fn runtime_file_size(filename: []const u8) i64 {
    const file = std.fs.cwd().openFile(filename, .{}) catch return -1;
    defer file.close();
    const stat = file.stat() catch return -1;
    return @intCast(stat.size);
}

// === TIME RUNTIME FUNCTIONS ===

pub fn current_time() i64 {
    return std.time.milliTimestamp();
}

pub fn runtime_current_time_millis() i64 {
    return std.time.milliTimestamp();
}

pub fn runtime_current_time_nanos() i64 {
    const timestamp = std.time.milliTimestamp();
    return timestamp * 1000000; // Convert milliseconds to nanoseconds
}

pub fn format_time(allocator: Allocator, timestamp: i64) ![]u8 {
    // Basic ISO format
    const seconds = @divTrunc(timestamp, 1000);
    const epoch = std.time.epoch.EpochSeconds{ .secs = @intCast(seconds) };
    const day_seconds = epoch.getDaySeconds();
    const year_day = epoch.getEpochDay().calculateYearDay();
    const month_day = year_day.calculateMonthDay();
    
    return std.fmt.allocPrint(allocator, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", .{
        year_day.year, @intFromEnum(month_day.month), month_day.day_index + 1,
        day_seconds.getHoursIntoDay(), day_seconds.getMinutesIntoHour(), day_seconds.getSecondsIntoMinute()
    });
}

pub fn parse_time(time_str: []const u8) i64 {
    // Simple parsing - for demo purposes, return current time
    _ = time_str;
    return std.time.milliTimestamp();
}

pub fn runtime_sleep_millis(duration_millis: i64) void {
    const duration_ms = @max(1, duration_millis);
    std.time.sleep(@intCast(duration_ms * 1000000));
}

pub fn runtime_sleep_nanos(duration_nanos: i64) void {
    const duration_ms = @max(1, @divTrunc(duration_nanos, 1000000));
    std.time.sleep(@intCast(duration_ms * 1000000));
}

// === MEMORY RUNTIME FUNCTIONS ===

pub fn runtime_allocate_memory(size: i64) i64 {
    // For now, return a placeholder address
    // In a real implementation, this would interface with the GC
    return @intCast(size);
}

pub fn runtime_free_memory(ptr: i64) void {
    _ = ptr; // Placeholder - would interface with GC
}

pub fn runtime_reallocate_memory(ptr: i64, new_size: i64) i64 {
    _ = ptr;
    return @intCast(new_size); // Placeholder
}

// === I/O RUNTIME FUNCTIONS ===

pub fn runtime_print_string(message: []const u8) void {
    print("{s}", .{message});
}

pub fn runtime_read_file(allocator: Allocator, filename: []const u8) ![]u8 {
    return std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024);
}

pub fn runtime_write_file(filename: []const u8, content: []const u8) !bool {
    const file = std.fs.cwd().createFile(filename, .{}) catch return false;
    defer file.close();
    file.writeAll(content) catch return false;
    return true;
}

pub fn runtime_file_exists(filename: []const u8) bool {
    std.fs.cwd().access(filename, .{}) catch return false;
    return true;
}

// === STDLIB FUNCTION REGISTRY ===

pub const StdlibFunction = struct {
    name: []const u8,
    module: []const u8,
    implementation: *const fn() callconv(.C) void,
};

pub fn registerStdlibFunctions() void {
    // This would register all the runtime functions for stdlib modules
    // Implementation would depend on the runtime function dispatch system
}

// Test function to verify runtime functions work
pub fn test_runtime_functions() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Test string functions
    const test_str = "Hello, CURSED!";
    const length = len_str(test_str);
    print("String length: {}\n", .{length});
    
    const substr = try substring(allocator, test_str, 0, 5);
    print("Substring: {s}\n", .{substr});
    
    const contains_result = string_contains(test_str, "CURSED");
    print("Contains 'CURSED': {}\n", .{contains_result});
    
    // Test math functions
    const sqrt_result = sqrt(16.0);
    print("sqrt(16) = {}\n", .{sqrt_result});
    
    const power_result = power(2.0, 3.0);
    print("2^3 = {}\n", .{power_result});
    
    print("✅ Runtime functions test completed\n", .{});
}
