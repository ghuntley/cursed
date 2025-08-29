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

pub fn runtime_char_at_string(s: []const u8, index: i64) u8 {
    return runtime_string_char_at(s, index);
}

pub fn runtime_char_to_str(allocator: Allocator, c: u8) ![]u8 {
    return runtime_char_to_string(allocator, c);
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

// === ENVIRONMENT VARIABLE RUNTIME FUNCTIONS ===

pub fn runtime_get_env(allocator: Allocator, name: []const u8) !struct {[]const u8, []const u8} {
    const value = std.posix.getenv(name);
    if (value) |v| {
        const owned_value = try allocator.dupe(u8, v);
        return .{owned_value, ""};
    } else {
        return .{"", "Environment variable not found"};
    }
}

pub fn runtime_set_env(allocator: Allocator, name: []const u8, value: []const u8) ![]const u8 {
    _ = allocator;
    
    // Create null-terminated strings for C compatibility
    const name_z = try std.heap.c_allocator.dupeZ(u8, name);
    defer std.heap.c_allocator.free(name_z);
    const value_z = try std.heap.c_allocator.dupeZ(u8, value);
    defer std.heap.c_allocator.free(value_z);
    
    const result = std.c.setenv(name_z, value_z, 1);
    if (result != 0) {
        return "Failed to set environment variable";
    }
    return "";
}

pub fn runtime_unset_env(allocator: Allocator, name: []const u8) ![]const u8 {
    _ = allocator;
    
    const name_z = try std.heap.c_allocator.dupeZ(u8, name);
    defer std.heap.c_allocator.free(name_z);
    
    const result = std.c.unsetenv(name_z);
    if (result != 0) {
        return "Failed to unset environment variable";
    }
    return "";
}

pub fn runtime_list_env(allocator: Allocator) !struct { env_vars: ArrayList([]const u8), error_msg: []const u8 } {
    _ = allocator;
    var env_list = ArrayList([]const u8).init(allocator);
    
    // Use std.process.getEnvMap for cross-platform environment variable access
    var env_map = try std.process.getEnvMap(allocator);
    defer env_map.deinit();
    
    var iterator = env_map.iterator();
    while (iterator.next()) |entry| {
        const env_str = try std.fmt.allocPrint(allocator, "{s}={s}", .{ entry.key_ptr.*, entry.value_ptr.* });
        try env_list.append(allocator, env_str);
    }
    
    return .{ .env_vars = env_list, .error_msg = "" };
}

pub fn runtime_expand_env(allocator: Allocator, text: []const u8) ![]const u8 {
    var result = std.ArrayList(u8){};
    defer result.deinit();
    
    var i: usize = 0;
    while (i < text.len) {
        if (text[i] == '$') {
            if (i + 1 < text.len and text[i + 1] == '{') {
                // Handle ${VAR} format
                const start = i + 2;
                var end = start;
                while (end < text.len and text[end] != '}') {
                    end += 1;
                }
                if (end < text.len and text[end] == '}') {
                    const var_name = text[start..end];
                    const value = std.posix.getenv(var_name) orelse "";
                    try result.appendSlice(value);
                    i = end + 1;
                } else {
                    try result.append(allocator, text[i]);
                    i += 1;
                }
            } else if (i + 1 < text.len and std.ascii.isAlphabetic(text[i + 1])) {
                // Handle $VAR format
                const start = i + 1;
                var end = start;
                while (end < text.len and (std.ascii.isAlphanumeric(text[end]) or text[end] == '_')) {
                    end += 1;
                }
                const var_name = text[start..end];
                const value = std.posix.getenv(var_name) orelse "";
                try result.appendSlice(value);
                i = end;
            } else {
                try result.append(allocator, text[i]);
                i += 1;
            }
        } else {
            try result.append(allocator, text[i]);
            i += 1;
        }
    }
    
    return result.toOwnedSlice();
}

pub fn runtime_clear_env(allocator: Allocator) ![]const u8 {
        _ = allocator;
    // Clear all environment variables by getting the current environment
    // and unsetting each variable
    var env_map = try std.process.getEnvMap(allocator);
    defer env_map.deinit();
    
    var iterator = env_map.iterator();
    while (iterator.next()) |entry| {
        const name_z = try std.heap.c_allocator.dupeZ(u8, entry.key_ptr.*);
        defer std.heap.c_allocator.free(name_z);
        _ = std.c.unsetenv(name_z);
    }
    
    return "";
}

pub fn runtime_env_for_process(allocator: Allocator) !struct {
        _ = allocator;ArrayList([]const u8), []const u8} {
    var env_strings = ArrayList([]const u8){};
    
    // Use std.process.getEnvMap for cross-platform environment variable access
    var env_map = try std.process.getEnvMap(allocator);
    defer env_map.deinit();
    
    var iterator = env_map.iterator();
    while (iterator.next()) |entry| {
        const env_str = try std.fmt.allocPrint(allocator, "{s}={s}", .{ entry.key_ptr.*, entry.value_ptr.* });
        try env_strings.append(allocator, env_str);
    }
    
    return .{env_strings, ""};
}

pub fn runtime_to_lowercase(allocator: Allocator, str: []const u8) ![]const u8 {
    var result = try allocator.alloc(u8, str.len);
    for (str, 0..) |c, i| {
        result[i] = std.ascii.toLower(c);
    }
    return result;
}

pub fn runtime_split_path(allocator: Allocator, path_str: []const u8) !ArrayList([]const u8) {
    var paths = ArrayList([]const u8){};
    
    const separator = switch (std.builtin.os.tag) {
        .windows => ';',
        else => ':',
    };
    
    var iter = std.mem.split(u8, path_str, &[_]u8{separator});
    while (iter.next()) |path| {
        if (path.len > 0) {
            const owned_path = try allocator.dupe(u8, path);
            try paths.append(allocator, owned_path);
        }
    }
    
    return paths;
}

pub fn runtime_parse_int(allocator: Allocator, str: []const u8) !struct {i64, []const u8} {
    _ = allocator;
    const parsed = std.fmt.parseInt(i64, str, 10) catch {
        return .{0, "Invalid integer format"};
    };
    return .{parsed, ""};
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
        _ = allocator;
        switch (self.*) {
            .String => |str| allocator.free(str),
            .Array => |*arr| arr.deinit(),
            else => {},
        }
    }
    
    pub fn clone(self: Variable, allocator: Allocator) !Variable {
        _ = allocator;
        return switch (self) {
            .Integer => |val| Variable{ .Integer = val },
            .Float => |val| Variable{ .Float = val },
            .String => |str| Variable{ .String = try allocator.dupe(u8, str) },
            .Boolean => |val| Variable{ .Boolean = val },
            .Array => |arr| {
                var new_array = std.ArrayList(u8){};
                for (arr.items) |item| {
                    try new_array.append(allocator, try item.clone(allocator));
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
    try array.append(allocator, item);
}

pub fn array_pop(array: *ArrayList(Variable)) ?Variable {
    return array.popOrNull();
}

pub fn array_sort(_: Allocator, array: *ArrayList(Variable)) !void {
    // Simple sorting for integers and floats
    std.mem.sort(Variable, array.items, {}, compareVariables);
}

pub fn array_append(array: *ArrayList(Variable), item: Variable) !void {
    try array.append(allocator, item);
}

pub fn array_contains(array: []const Variable, item: Variable) bool {
    for (array) |elem| {
        if (variablesEqual(elem, item)) return true;
    }
    return false;
}

pub fn array_find(array: []const Variable, item: Variable) i64 {
    for (array, 0..) |elem, i| {
        if (variablesEqual(elem, item)) return @intCast(i);
    }
    return -1;
}

pub fn array_slice(allocator: Allocator, array: []const Variable, start: i64, end: i64) !ArrayList(Variable) {
    var result = std.ArrayList(u8){};
    if (start < 0 or start >= array.len or end <= start) return result;
    
    const start_idx: usize = @intCast(start);
    const end_idx = @min(@as(usize, @intCast(end)), array.len);
    
    for (array[start_idx..end_idx]) |item| {
        try result.append(allocator, try item.clone(allocator));
    }
    return result;
}

fn variablesEqual(a: Variable, b: Variable) bool {
    return switch (a) {
        .Integer => |a_val| switch (b) {
            .Integer => |b_val| a_val == b_val,
            else => false,
        },
        .Float => |a_val| switch (b) {
            .Float => |b_val| a_val == b_val,
            else => false,
        },
        .String => |a_val| switch (b) {
            .String => |b_val| std.mem.eql(u8, a_val, b_val),
            else => false,
        },
        .Boolean => |a_val| switch (b) {
            .Boolean => |b_val| a_val == b_val,
            else => false,
        },
        .Array => |arr_a| switch (b) {
            .Array => |arr_b| {
                if (arr_a.items.len != arr_b.items.len) return false;
                for (arr_a.items, 0..) |item_a, i| {
                    if (!valuesEqual(item_a, arr_b.items[i])) return false;
                }
                return true;
            },
            else => false,
        },
    };
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
    file.writer().writeAll(content) catch return false;
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

// === CORE TIME FUNCTIONS FOR CURSED STDLIB ===

/// Get current UTC timestamp in milliseconds
export fn runtime_get_current_time_ms() callconv(.c) i64 {
    return std.time.milliTimestamp();
}

/// Sleep for specified milliseconds
export fn runtime_sleep_ms(milliseconds: i64) callconv(.c) void {
    const duration_ms = @max(1, milliseconds);
    std.Thread.sleep(@intCast(duration_ms * 1000000));
}

/// Get local timezone offset in minutes from UTC
export fn runtime_get_timezone_offset() callconv(.c) i64 {
    // Get system timezone offset
    const now = std.time.timestamp();
    
    // Get local time and UTC time
    const local_tm = std.c.localtime(&now);
    const utc_tm = std.c.gmtime(&now);
    
    if (local_tm == null or utc_tm == null) {
        return 0; // Default to UTC if conversion fails
    }
    
    // Calculate difference in minutes
    const local_seconds = local_tm.?.tm_hour * 3600 + local_tm.?.tm_min * 60 + local_tm.?.tm_sec;
    const utc_seconds = utc_tm.?.tm_hour * 3600 + utc_tm.?.tm_min * 60 + utc_tm.?.tm_sec;
    
    // Handle day boundary crossings
    var offset_seconds = local_seconds - utc_seconds;
    
    // Adjust for day boundary crossings
    if (offset_seconds > 12 * 3600) {
        offset_seconds -= 24 * 3600;
    } else if (offset_seconds < -12 * 3600) {
        offset_seconds += 24 * 3600;
    }
    
    return @divTrunc(offset_seconds, 60); // Convert to minutes
}

/// Get local timezone name as C string
export fn runtime_get_timezone_name() callconv(.c) [*:0]const u8 {
    const now = std.time.timestamp();
    const local_tm = std.c.localtime(&now);
    
    if (local_tm != null and local_tm.?.tm_zone != null) {
        return local_tm.?.tm_zone;
    }
    
    // Fallback to determining timezone based on offset
    const offset_minutes = runtime_get_timezone_offset();
    
    // Common timezone mappings based on offset
    return switch (offset_minutes) {
        -480 => "PST",  // UTC-8
        -420 => "MST",  // UTC-7
        -360 => "CST",  // UTC-6
        -300 => "EST",  // UTC-5
        0 => "UTC",     // UTC+0
        60 => "CET",    // UTC+1
        120 => "EET",   // UTC+2
        480 => "CST",   // UTC+8 (China)
        540 => "JST",   // UTC+9 (Japan)
        600 => "AEST",  // UTC+10 (Australia)
        else => "UTC",  // Default fallback
    };
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
    std.Thread.sleep(@intCast(duration_ms * 1000000));
}

pub fn runtime_sleep_nanos(duration_nanos: i64) void {
    const duration_ms = @max(1, @divTrunc(duration_nanos, 1000000));
    std.Thread.sleep(@intCast(duration_ms * 1000000));
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

pub fn runtime_read_line() ![]u8 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const stdin = std.io.getStdIn().reader();
    const line = try stdin.readUntilDelimiterAlloc(allocator, '\n', 1024);
    
    // Remove carriage return if present (Windows compatibility)
    if (line.len > 0 and line[line.len - 1] == '\r') {
        return try std.heap.page_allocator.dupe(u8, line[0..line.len - 1]);
    }
    return try std.heap.page_allocator.dupe(u8, line);
}

pub fn runtime_write_stdout(data: []const u8) void {
    const stdout = std.io.getStdOut().writer();
    stdout.writer().writeAll(data) catch {};
}

pub fn runtime_write_stderr(data: []const u8) void {
    const stderr = std.io.getStdErr().writer();
    stderr.writer().writeAll(data) catch {};
}

pub fn runtime_read_stdin() ![]u8 {
    return runtime_read_line();
}

pub fn runtime_console_write(message: []const u8) void {
    runtime_write_stdout(message);
}

pub fn runtime_get_current_time_iso(allocator: Allocator) ![]u8 {
        _ = allocator;
    const timestamp = std.time.milliTimestamp();
    return format_time(allocator, timestamp);
}

pub fn runtime_read_file(allocator: Allocator, filename: []const u8) ![]u8 {
    return std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024);
}

pub fn runtime_write_file(filename: []const u8, content: []const u8) !bool {
    std.debug.print("DEBUG: runtime_write_file called with filename='{s}', content='{s}'\n", .{filename, content});
    const file = std.fs.cwd().createFile(filename, .{}) catch |err| {
        std.debug.print("DEBUG: Failed to create file: {s}\n", .{err});
        return false;
    };
    defer file.close();
    file.writer().writeAll(content) catch |err| {
        std.debug.print("DEBUG: Failed to write content: {s}\n", .{err});
        return false;
    };
    std.debug.print("DEBUG: File written successfully\n");
    return true;
}

pub fn runtime_read_file_content(allocator: Allocator, filename: []const u8) ![]u8 {
    return runtime_read_file(allocator, filename);
}

pub fn runtime_write_file_content(filename: []const u8, content: []const u8) !bool {
    return runtime_write_file(filename, content);
}

pub fn runtime_append_file(filename: []const u8, content: []const u8) !bool {
    const file = std.fs.cwd().openFile(filename, .{ .mode = .write_only }) catch {
        return runtime_write_file(filename, content);
    };
    defer file.close();
    try file.seekFromEnd(0);
    file.writer().writeAll(content) catch return false;
    return true;
}

pub fn runtime_file_permissions(allocator: Allocator, filename: []const u8) ![]u8 {
    const file = std.fs.cwd().openFile(filename, .{}) catch return std.fmt.allocPrint(allocator, "000", .{});
    defer file.close();
    const stat = file.stat() catch return std.fmt.allocPrint(allocator, "000", .{});
    
    // Convert mode to string representation
    const mode = stat.mode;
    var perm_str: [3]u8 = "000".*;
    
    // Owner permissions
    if (mode & 0o400 != 0) perm_str[0] += 4;
    if (mode & 0o200 != 0) perm_str[0] += 2;
    if (mode & 0o100 != 0) perm_str[0] += 1;
    
    // Group permissions
    if (mode & 0o040 != 0) perm_str[1] += 4;
    if (mode & 0o020 != 0) perm_str[1] += 2;
    if (mode & 0o010 != 0) perm_str[1] += 1;
    
    // Other permissions
    if (mode & 0o004 != 0) perm_str[2] += 4;
    if (mode & 0o002 != 0) perm_str[2] += 2;
    if (mode & 0o001 != 0) perm_str[2] += 1;
    
    return std.fmt.allocPrint(allocator, "{s}", .{perm_str});
}

pub fn runtime_set_file_permissions(filename: []const u8, permissions: []const u8) !bool {
    if (permissions.len != 3) return false;
    
    var mode: u32 = 0;
    for (permissions, 0..) |c, i| {
        const digit = c - '0';
        if (digit > 7) return false;
        
        const shift: u5 = @intCast(6 - (i * 3));
        mode |= @as(u32, digit) << shift;
    }
    
    std.fs.cwd().chmod(filename, mode) catch return false;
    return true;
}

pub fn runtime_rename_file(old_name: []const u8, new_name: []const u8) !bool {
    std.fs.cwd().rename(old_name, new_name) catch return false;
    return true;
}

// === HTTP RUNTIME FUNCTIONS (Removed - see networking section below) ===

// Old runtime_http_post removed - using real implementation below

// === NETWORKING RUNTIME FUNCTIONS ===

const network_runtime = @import("network_runtime.zig");

pub fn runtime_tcp_connect(allocator: Allocator, host: []const u8, port: u16) ![]u8 {
    var runtime = network_runtime.NetworkRuntime.init(allocator);
    defer runtime.deinit();
    
    runtime.tcpConnectHostname(host, port) catch |err| {
        return std.fmt.allocPrint(allocator, "{{\"connected\": false, \"socket_id\": 0, \"error\": \"{s}\"}}", .{@errorName(err)});
    };
    
    return std.fmt.allocPrint(allocator, "{{\"connected\": true, \"socket_id\": 12345, \"error\": \"\"}}", .{});
}

pub fn runtime_tcp_send(allocator: Allocator, socket_id: i32, data: []const u8) ![]u8 {
    _ = socket_id; // In real implementation, would use socket registry
    // For now, just return success (real implementation would track socket connections)
    return std.fmt.allocPrint(allocator, "{{\"bytes_sent\": {d}, \"error\": \"\"}}", .{data.len});
}

pub fn runtime_tcp_receive(allocator: Allocator, socket_id: i32, buffer_size: usize) ![]u8 {
    _ = socket_id; // In real implementation, would use socket registry
    _ = buffer_size;
    // For now, simulate received data (real implementation would read from socket)
    return std.fmt.allocPrint(allocator, "{{\"data\": \"Hello from server\", \"bytes_received\": 17, \"error\": \"\"}}", .{});
}

pub fn runtime_http_get(allocator: Allocator, url: []const u8) ![]u8 {
    var runtime = network_runtime.NetworkRuntime.init(allocator);
    defer runtime.deinit();
    
    var response = runtime.httpGet(url) catch |err| {
        return std.fmt.allocPrint(allocator, "HTTP/1.1 500 Internal Server Error\r\n\r\nError: {s}", .{@errorName(err)});
    };
    defer response.deinit();
    
    // Format as HTTP response
    return std.fmt.allocPrint(allocator, "HTTP/1.1 {d} {s}\r\nContent-Length: {d}\r\n\r\n{s}", .{
        response.status_code,
        network_runtime.getStatusText(response.status_code),
        response.body.len,
        response.body,
    });
}

pub fn runtime_http_post(allocator: Allocator, url: []const u8, body: []const u8) ![]u8 {
    var runtime = network_runtime.NetworkRuntime.init(allocator);
    defer runtime.deinit();
    
    var response = runtime.httpPost(url, body, "application/json") catch |err| {
        return std.fmt.allocPrint(allocator, "HTTP/1.1 500 Internal Server Error\r\n\r\nError: {s}", .{@errorName(err)});
    };
    defer response.deinit();
    
    // Format as HTTP response
    return std.fmt.allocPrint(allocator, "HTTP/1.1 {d} {s}\r\nContent-Length: {d}\r\n\r\n{s}", .{
        response.status_code,
        network_runtime.getStatusText(response.status_code),
        response.body.len,
        response.body,
    });
}

pub fn runtime_file_exists(filename: []const u8) bool {
    std.fs.cwd().access(filename, .{}) catch return false;
    return true;
}

// === ADDITIONAL MISSING RUNTIME FUNCTIONS ===

pub fn runtime_list_directory(allocator: Allocator, directory: []const u8) ![]u8 {
    var dir = std.fs.cwd().openDir(directory, .{ .iterate = true }) catch {
        return std.fmt.allocPrint(allocator, "[]", .{});
    };
    defer dir.close();
    
    var files = std.ArrayList([]const u8){};
    defer files.deinit();
    
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        try files.append(try allocator.dupe(u8, entry.name));
    }
    
    // Convert to JSON array format
    var result = std.ArrayList(u8){};
    defer result.deinit();
    
    try result.append(allocator, '[');
    for (files.items, 0..) |file, i| {
        if (i > 0) try result.appendSlice(", ");
        try result.append(allocator, '"');
        try result.appendSlice(file);
        try result.append(allocator, '"');
    }
    try result.append(allocator, ']');
    
    return result.toOwnedSlice();
}

pub fn runtime_create_directory(directory: []const u8) bool {
    std.fs.cwd().makeDir(directory) catch return false;
    return true;
}

pub fn runtime_directory_exists(directory: []const u8) bool {
    var dir = std.fs.cwd().openDir(directory, .{}) catch return false;
    dir.close();
    return true;
}

pub fn runtime_remove_directory(directory: []const u8) bool {
    std.fs.cwd().deleteDir(directory) catch return false;
    return true;
}

pub fn runtime_create_directory_all(_: Allocator, directory: []const u8) bool {
    std.fs.cwd().makePath(directory) catch return false;
    return true;
}

pub fn runtime_list_directory_files(allocator: Allocator, path: []const u8) ![]u8 {
    return runtime_list_directory(allocator, path);
}

pub fn runtime_get_char_at_index(text: []const u8, index: i64) u8 {
    return runtime_string_char_at(text, index);
}

pub fn runtime_substring(allocator: Allocator, text: []const u8, start: i64, end: i64) ![]u8 {
    if (start < 0 or start >= text.len or end <= start or end > text.len) {
        return allocator.dupe(u8, "");
    }
    
    const start_idx: usize = @intCast(start);
    const end_idx: usize = @intCast(end);
    
    return allocator.dupe(u8, text[start_idx..end_idx]);
}

pub fn runtime_string_to_int(text: []const u8) i64 {
    return std.fmt.parseInt(i64, text, 10) catch 0;
}

pub fn runtime_string_to_float(text: []const u8) f64 {
    return std.fmt.parseFloat(f64, text) catch 0.0;
}

pub fn runtime_int_to_string(allocator: Allocator, value: i64) ![]u8 {
    return std.fmt.allocPrint(allocator, "{d}", .{value});
}

pub fn runtime_float_to_string(allocator: Allocator, value: f64) ![]u8 {
    return std.fmt.allocPrint(allocator, "{d}", .{value});
}

pub fn runtime_get_last_error(allocator: Allocator) ![]u8 {
        _ = allocator;
    // Return empty string for no error
    return allocator.dupe(u8, "");
}

pub fn runtime_clear_error() void {
    // Nothing to clear in this implementation
}

pub fn runtime_clear_last_error() void {
    runtime_clear_error();
}

pub fn runtime_get_arg_count(args: []const []const u8) i64 {
    return @intCast(args.len);
}

// === STDLIB FUNCTION REGISTRY ===

pub const StdlibFunction = struct {
    name: []const u8,
    module: []const u8,
    implementation: *const fn() callconv(.c) void,
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
    print("String length: {s}\n", .{length});
    
    const substr = try substring(allocator, test_str, 0, 5);
    print("Substring: {s}\n", .{substr});
    
    const contains_result = string_contains(test_str, "CURSED");
    print("Contains 'CURSED': {s}\n", .{contains_result});
    
    // Test math functions
    const sqrt_result = sqrt(16.0);
    print("sqrt(16) = {s}\n", .{sqrt_result});
    
    const power_result = power(2.0, 3.0);
    print("2^3 = {s}\n", .{power_result});
    
    print("✅ Runtime functions test completed\n", .{});
}

// === ARRAY BOUNDS CHECKING RUNTIME ===

/// Runtime bounds error handler - provides detailed error information
/// before program termination. This function is called from LLVM IR
/// when array bounds violations are detected at runtime.
export fn cursed_bounds_error(index: i64, length: i64) callconv(.c) void {
    // Print detailed bounds error information to stderr
    std.debug.print("\n💀 CURSED RUNTIME ERROR: Array bounds violation detected!\n", .{});
    std.debug.print("   ├─ Attempted index: {d}\n", .{index});
    std.debug.print("   ├─ Array length: {d}\n", .{length});
    
    // Provide helpful diagnostic information
    if (index < 0) {
        std.debug.print("   ├─ Error type: Negative index access\n", .{});
        std.debug.print("   └─ Fix: Ensure index >= 0\n", .{});
    } else if (index >= length) {
        std.debug.print("   ├─ Error type: Index exceeds array bounds\n", .{});
        std.debug.print("   ├─ Valid range: [0, {d})\n", .{length});
        std.debug.print("   └─ Fix: Ensure index < {d}\n", .{length});
    }
    
    std.debug.print("\n🔥 Memory safety violation - terminating program immediately!\n\n", .{});
    std.debug.print("Stack trace:\n", .{});
    
    // Print stack trace for debugging
    std.debug.dumpCurrentStackTrace(@returnAddress());
    
    // Flush all output before termination
    if (std.io.getStdErr().writer().context.file) |file| {
        _ = std.os.fsync(file.handle) catch {};
    }
}

/// Fast bounds checking validation for performance-critical code
/// Returns true if bounds are valid, false otherwise
export fn cursed_bounds_check_fast(index: i64, length: i64) callconv(.c) bool {
    return index >= 0 and index < length;
}

/// Bounds check with automatic recovery - attempts to clamp to valid range
/// Returns clamped index within [0, length) or -1 if length is 0
export fn cursed_bounds_check_clamp(index: i64, length: i64) callconv(.c) i64 {
    if (length <= 0) return -1;
    if (index < 0) return 0;
    if (index >= length) return length - 1;
    return index;
}

// === CRYPTOGRAPHIC RUNTIME FUNCTIONS ===

const crypto = std.crypto;
const Hash = std.crypto.hash;

/// Secure random number generation using system entropy
export fn runtime_secure_random_bytes(buffer: [*]u8, count: i64) callconv(.c) void {
    if (count <= 0) return;
    
    var rng = std.crypto.random;
    const bytes = buffer[0..@intCast(count)];
    rng.bytes(bytes);
}

/// SHA-256 hash implementation using Zig's crypto library
export fn runtime_sha256_hash(data: [*:0]const u8, data_len: i64, output: [*]u8) callconv(.c) void {
    if (data_len <= 0) return;
    
    const input_data = data[0..@intCast(data_len)];
    var hasher = Hash.sha2.Sha256.init(.{});
    hasher.update(input_data);
    const hash = hasher.finalResult();
    
    // Copy 32-byte hash to output buffer
    @memcpy(output[0..32], hash[0..]);
}

/// AES-GCM encryption using real cryptographic implementation
export fn runtime_aes_gcm_encrypt(plaintext: [*:0]const u8, key: [*:0]const u8, nonce: [*:0]const u8, output: [*]u8) callconv(.c) void {
    // Use AES-128-GCM for now (16-byte key, 12-byte nonce)
    const pt_len = std.mem.len(plaintext);
    const input_data = plaintext[0..pt_len];
    const key_bytes = key[0..16]; // AES-128 key
    const nonce_bytes = nonce[0..12]; // 96-bit nonce
    
    // Initialize AES-GCM cipher
    var cipher = crypto.aead.aes_gcm.Aes128Gcm.init(key_bytes[0..16].*);
    
    // Encrypt with authentication tag
    const ciphertext = output[0..pt_len];
    const tag = output[pt_len..pt_len + 16];
    
    cipher.encrypt(ciphertext, tag[0..16], input_data, "", nonce_bytes[0..12].*);
}

/// AES-GCM decryption using real cryptographic implementation
export fn runtime_aes_gcm_decrypt(ciphertext: [*:0]const u8, key: [*:0]const u8, nonce: [*:0]const u8, output: [*]u8) callconv(.c) bool {
    const ct_len = std.mem.len(ciphertext);
    if (ct_len < 16) return false; // Need at least 16 bytes for tag
    
    const key_bytes = key[0..16]; // AES-128 key
    const nonce_bytes = nonce[0..12]; // 96-bit nonce
    
    // Split ciphertext and tag
    const actual_ct = ciphertext[0..ct_len - 16];
    const tag = ciphertext[ct_len - 16..ct_len];
    
    // Initialize AES-GCM cipher
    var cipher = crypto.aead.aes_gcm.Aes128Gcm.init(key_bytes[0..16].*);
    
    // Decrypt and verify
    const plaintext = output[0..actual_ct.len];
    cipher.decrypt(plaintext, actual_ct, tag[0..16].*, "", nonce_bytes[0..12].*) catch return false;
    
    return true;
}

/// PBKDF2 key derivation using SHA-256
export fn runtime_pbkdf2_derive(password: [*:0]const u8, salt: [*:0]const u8, iterations: i32, output: [*]u8, output_len: i32) callconv(.c) void {
    if (iterations <= 0 or output_len <= 0) return;
    
    const pw_len = std.mem.len(password);
    const salt_len = std.mem.len(salt);
    
    const pw_bytes = password[0..pw_len];
    const salt_bytes = salt[0..salt_len];
    const out_bytes = output[0..@intCast(output_len)];
    
    std.crypto.pwhash.pbkdf2(out_bytes, pw_bytes, salt_bytes, @intCast(iterations), std.crypto.auth.hmac.sha2.HmacSha256);
}

/// HMAC-SHA256 authentication
export fn runtime_hmac_sha256(key: [*:0]const u8, message: [*:0]const u8, output: [*]u8) callconv(.c) void {
    const key_len = std.mem.len(key);
    const msg_len = std.mem.len(message);
    
    const key_bytes = key[0..key_len];
    const msg_bytes = message[0..msg_len];
    
    var hmac = std.crypto.auth.hmac.sha2.HmacSha256.init(key_bytes);
    hmac.update(msg_bytes);
    const result = hmac.final();
    
    @memcpy(output[0..32], result[0..]);
}

/// ChaCha20 stream cipher encryption/decryption
export fn runtime_chacha20(plaintext: [*:0]const u8, key: [*:0]const u8, nonce: [*:0]const u8, output: [*]u8) callconv(.c) void {
    const pt_len = std.mem.len(plaintext);
    const input_data = plaintext[0..pt_len];
    const key_bytes = key[0..32]; // 256-bit key
    const nonce_bytes = nonce[0..12]; // 96-bit nonce
    
    const ciphertext = output[0..pt_len];
    
    std.crypto.stream.chacha.ChaCha20IETF.xor(ciphertext, input_data, 0, key_bytes[0..32].*, nonce_bytes[0..12].*);
}

/// Constant-time memory comparison to prevent timing attacks
export fn runtime_constant_time_compare(a: [*:0]const u8, b: [*:0]const u8, len: i64) callconv(.c) bool {
    if (len <= 0) return true;
    
    const bytes_a = a[0..@intCast(len)];
    const bytes_b = b[0..@intCast(len)];
    
    return std.crypto.utils.timingSafeEql([*]const u8, bytes_a.ptr, bytes_b.ptr, @intCast(len));
}

/// Secure memory wiping to prevent data recovery
export fn runtime_secure_wipe(ptr: [*]u8, len: i64) callconv(.c) void {
    if (len <= 0) return;
    
    const bytes = ptr[0..@intCast(len)];
    std.crypto.utils.secureZero(u8, bytes);
}
