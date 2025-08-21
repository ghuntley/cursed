//! FFI Runtime Bridge Implementation
//! Provides the actual runtime support for C function calls from CURSED code

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// Mock C function implementations for testing
// In real implementation, these would call actual C libraries
var mock_pixel_colors: HashMap(u64, i32, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage) = undefined;
var mock_system_status: i32 = 0; // OK status
var mock_log_priority: i32 = 0; // Normal priority

pub fn initializeFfiRuntime(allocator: Allocator) !void {
    mock_pixel_colors = HashMap(u64, i32, std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator);
}

pub fn deinitializeFfiRuntime() void {
    mock_pixel_colors.deinit();
}

// Mock graphics library functions
pub export fn set_pixel_color(x: i32, y: i32, color: i8) void {
    const coord = (@as(u64, @intCast(x)) << 32) | @as(u64, @intCast(y));
    mock_pixel_colors.put(coord, color) catch {
        print("Warning: Failed to store pixel color\n", .{});
    };
    print("Set pixel at ({}, {}) to color {}\n", .{ x, y, color });
}

pub export fn get_pixel_color(x: i32, y: i32) i8 {
    const coord = (@as(u64, @intCast(x)) << 32) | @as(u64, @intCast(y));
    const color = mock_pixel_colors.get(coord) orelse 0;
    print("Get pixel at ({}, {}) returned color {}\n", .{ x, y, color });
    return @intCast(color);
}

// Mock system library functions  
pub export fn get_system_status() i8 {
    print("System status requested: {}\n", .{mock_system_status});
    return @intCast(mock_system_status);
}

pub export fn set_log_priority(priority: i8) void {
    mock_log_priority = priority;
    print("Log priority set to: {}\n", .{priority});
}

// FFI call dispatcher
pub fn cursed_ffi_call(library_name: []const u8, function_name: []const u8, args: anytype) !i32 {
    print("FFI call: {s}.{s}\n", .{ library_name, function_name });
    
    if (std.mem.eql(u8, library_name, "graphics")) {
        if (std.mem.eql(u8, function_name, "set_pixel_color")) {
            if (args.len == 3) {
                set_pixel_color(args[0], args[1], args[2]);
                return 0;
            }
        } else if (std.mem.eql(u8, function_name, "get_pixel_color")) {
            if (args.len == 2) {
                return get_pixel_color(args[0], args[1]);
            }
        }
    } else if (std.mem.eql(u8, library_name, "system")) {
        if (std.mem.eql(u8, function_name, "get_system_status")) {
            if (args.len == 0) {
                return get_system_status();
            }
        } else if (std.mem.eql(u8, function_name, "set_log_priority")) {
            if (args.len == 1) {
                set_log_priority(args[0]);
                return 0;
            }
        }
    }
    
    print("FFI function not found or wrong argument count\n", .{});
    return -1;
}

// Dynamic library loading stubs
pub export fn load_dynamic_library(name: [*:0]const u8) ?*anyopaque {
    _ = name;
    // Return a dummy handle for testing
    return @ptrFromInt(0x12345678);
}

pub export fn get_function_symbol(handle: ?*anyopaque, name: [*:0]const u8) ?*anyopaque {
    _ = handle;
    _ = name;
    // Return a dummy function pointer for testing
    return @ptrFromInt(0x87654321);
}

pub export fn call_c_function(func_ptr: ?*anyopaque) i32 {
    _ = func_ptr;
    // Dummy implementation
    return 0;
}

// Export functions for external use
pub export fn ffi_runtime_init() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    initializeFfiRuntime(allocator) catch {
        print("Failed to initialize FFI runtime\n", .{});
    };
}

pub export fn ffi_runtime_deinit() void {
    deinitializeFfiRuntime();
}

// Test utilities
pub fn simulateSystemError() void {
    mock_system_status = 255; // ERROR status
}

pub fn simulateSystemWarning() void {
    mock_system_status = 1; // WARNING status  
}

pub fn resetSystemStatus() void {
    mock_system_status = 0; // OK status
}
