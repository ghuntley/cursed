const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const print = std.debug.print;
const missing_impl = @import("missing_impl_functions.zig");

/// Core Zig FFI implementation for CURSED standard library
/// Provides the essential runtime functions that CURSED stdlib modules depend on
/// This enables pure CURSED stdlib modules to access system functionality

pub const StdlibCore = struct {
    allocator: Allocator,
    
    pub fn init() StdlibCore {
        return StdlibCore{
            .allocator = allocator,
        };
    }
    
    // ===== CORE I/O FUNCTIONS (vibez module) =====
    
    /// Print string to console - core function for vibez.spill()
    pub fn print_string(self: *StdlibCore, message: []const u8) void {
        _ = self;
        std.debug.print("{s}", .{message});
    }
    
    /// Read a line from stdin - core function for vibez.scanln()
    pub fn read_line(self: *StdlibCore) ![]u8 {
        var stdin_buffer: [4096]u8 = undefined;
        const stdin = std.io.stdin;
        const line = try stdin.reader().readUntilDelimiterOrEof(stdin_buffer[0..], '\n') orelse return "";
        return try self.allocator.dupe(u8, line);
    }
    
    /// Read a single character from stdin
    pub fn read_char(self: *StdlibCore) !u8 {
        _ = self;
        var stdin_buffer: [4096]u8 = undefined;
        const stdin = std.fs.File.stdin().reader(stdin_buffer[0..]);
        return try stdin.readByte();
    }
    
    /// Get current timestamp in nanoseconds
    pub fn get_timestamp(self: *StdlibCore) i64 {
        _ = self;
        return std.time.nanoTimestamp();
    }
    
    // ===== STRING FUNCTIONS (stringz module) =====
    
    /// Get character at index in string
    pub fn string_char_at(self: *StdlibCore, string: []const u8, index: usize) u8 {
        _ = self;
        if (index >= string.len) return 0;
        return string[index];
    }
    
    /// Convert character to string
    pub fn char_to_string(self: *StdlibCore, char: u8) ![]u8 {
        const result = try self.allocator.alloc(u8, 1);
        result[0] = char;
        return result;
    }
    
    /// Convert string to integer
    pub fn string_to_int(self: *StdlibCore, string: []const u8) !i64 {
        _ = self;
        return std.fmt.parseInt(i64, string, 10);
    }
    
    /// Convert integer to string
    pub fn int_to_string(self: *StdlibCore, value: i64) ![]u8 {
        return std.fmt.allocPrint(self.allocator, "{d}", .{value});
    }
    
    /// Convert float to string
    pub fn float_to_string(self: *StdlibCore, value: f64) ![]u8 {
        return std.fmt.allocPrint(self.allocator, "{d}", .{value});
    }
    
    // ===== ARRAY FUNCTIONS (arrayz module) =====
    
    /// Get length of array/slice
    pub fn array_length(self: *StdlibCore, data: []const u8) usize {
        _ = self;
        return data.len;
    }
    
    /// Append to dynamic array (simplified)
    pub fn array_append(self: *StdlibCore, list: *ArrayList([]const u8), item: []const u8) !void {
        _ = self;
        try list.append(item);
    }
    
    // ===== MATH FUNCTIONS (mathz module) =====
    
    /// Absolute value for integers
    pub fn abs_int(self: *StdlibCore, value: i64) i64 {
        _ = self;
        return if (value < 0) -value else value;
    }
    
    /// Absolute value for floats
    pub fn abs_float(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return if (value < 0.0) -value else value;
    }
    
    /// Square root using standard library
    pub fn sqrt(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.sqrt(value);
    }
    
    /// Power function
    pub fn pow(self: *StdlibCore, base: f64, exponent: f64) f64 {
        _ = self;
        return std.math.pow(f64, base, exponent);
    }
    
    /// Natural logarithm
    pub fn ln(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.ln(value);
    }
    
    /// Sine function
    pub fn sin(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.sin(value);
    }
    
    /// Cosine function
    pub fn cos(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.cos(value);
    }
    
    /// Tangent function
    pub fn tan(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.tan(value);
    }
    
    /// Arc sine function  
    pub fn asin(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.asin(value);
    }
    
    /// Arc cosine function
    pub fn acos(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.acos(value);
    }
    
    /// Arc tangent function
    pub fn atan(self: *StdlibCore, value: f64) f64 {
        _ = self;
        return std.math.atan(value);
    }
    
    /// Arc tangent 2 function
    pub fn atan2(self: *StdlibCore, y: f64, x: f64) f64 {
        _ = self;
        return std.math.atan2(f64, y, x);
    }
    
    /// Random float generation
    pub fn random_float(self: *StdlibCore) f64 {
        _ = self;
        return missing_impl.random_float_impl();
    }
    
    /// Random integer in range
    pub fn random_int_range(self: *StdlibCore, min: i64, max: i64) i64 {
        _ = self;
        return missing_impl.random_int_range_impl(min, max);
    }
    
    // ===== FILE OPERATIONS (vibez/filez modules) =====
    
    /// Read entire file content
    pub fn read_file_content(self: *StdlibCore, filename: []const u8) ![]u8 {
        const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
            return switch (err) {
                error.FileNotFound => error.FileNotFound,
                else => error.AccessDenied,
            };
        };
        defer file.close();
        
        const stat = try file.stat();
        const content = try self.allocator.alloc(u8, stat.size);
        _ = try file.readAll(content);
        return content;
    }
    
    /// Write content to file
    pub fn write_file_content(self: *StdlibCore, filename: []const u8, content: []const u8) !bool {
        _ = self;
        const file = std.fs.cwd().createFile(filename, .{}) catch return false;
        defer file.close();
        
        file.writeAll(content) catch return false;
        return true;
    }
    
    /// Check if file exists
    pub fn file_exists(self: *StdlibCore, filename: []const u8) bool {
        _ = self;
        std.fs.cwd().access(filename, .{}) catch return false;
        return true;
    }
    
    /// Delete file
    pub fn delete_file(self: *StdlibCore, filename: []const u8) bool {
        _ = self;
        std.fs.cwd().deleteFile(filename) catch return false;
        return true;
    }
    
    /// Get file size
    pub fn get_file_size(self: *StdlibCore, filename: []const u8) !i64 {
        _ = self;
        const file = try std.fs.cwd().openFile(filename, .{});
        defer file.close();
        const stat = try file.stat();
        return @intCast(stat.size);
    }
    
    // ===== DIRECTORY OPERATIONS =====
    
    /// List directory files
    pub fn list_directory_files(self: *StdlibCore, directory: []const u8) !ArrayList([]const u8) {
        var result = .empty;
        
        var dir = std.fs.cwd().openIterableDir(directory, .{}) catch return result;
        defer dir.close();
        
        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            const name = try self.allocator.dupe(u8, entry.name);
            try result.append(self.allocator, name);
        }
        
        return result;
    }
    
    /// Create directory
    pub fn create_directory(self: *StdlibCore, directory: []const u8) bool {
        _ = self;
        std.fs.cwd().makeDir(directory) catch return false;
        return true;
    }
    
    /// Check if directory exists
    pub fn directory_exists(self: *StdlibCore, directory: []const u8) bool {
        _ = self;
        var dir = std.fs.cwd().openDir(directory, .{}) catch return false;
        dir.close();
        return true;
    }
    
    /// Remove directory
    pub fn remove_directory(self: *StdlibCore, directory: []const u8) bool {
        _ = self;
        std.fs.cwd().deleteDir(directory) catch return false;
        return true;
    }
    
    /// Create directory recursively
    pub fn create_directory_recursive(self: *StdlibCore, directory: []const u8) bool {
        _ = self;
        std.fs.cwd().makePath(directory) catch return false;
        return true;
    }
    
    // ===== ERROR HANDLING =====
    
    var last_error_message: ?[]const u8 = null;
    
    /// Get last error message
    pub fn get_last_error_message(self: *StdlibCore) []const u8 {
        _ = self;
        return last_error_message orelse "";
    }
    
    /// Clear last error
    pub fn clear_last_error(self: *StdlibCore) void {
        _ = self;
        last_error_message = null;
    }
    
    /// Set error message
    pub fn set_error_message(self: *StdlibCore, message: []const u8) void {
        _ = self;
        last_error_message = message;
    }
    
    // ===== MEMORY MANAGEMENT =====
    
    /// Allocate memory
    pub fn allocate_memory(self: *StdlibCore, size: usize) ![]u8 {
        return try self.allocator.alloc(u8, size);
    }
    
    /// Free memory
    pub fn free_memory(self: *StdlibCore, memory: []u8) void {
        self.allocator.free(memory);
    }
    
    /// Reallocate memory
    pub fn reallocate_memory(self: *StdlibCore, memory: []u8, new_size: usize) ![]u8 {
        return try self.allocator.realloc(memory, new_size);
    }
    
    // ===== SYSTEM CALLS =====
    
    /// Generic system call interface
    pub fn system_call(self: *StdlibCore, call_number: i32, arg1: usize, arg2: usize, arg3: usize) i32 {
        _ = self;
        _ = arg1;
        _ = arg2;
        _ = arg3;
        
        return switch (call_number) {
            1 => 0, // SYS_UNAME - return success
            2 => 0, // SYS_ARCH - return success  
            3 => 0, // SYS_VERSION - return success
            4 => 0, // SYS_HOSTNAME - return success
            5 => 0, // SYS_USERNAME - return success
            6 => 4, // SYS_CPU_COUNT - return 4 CPUs
            7 => 8192, // SYS_TOTAL_MEMORY - return 8GB
            8 => 4096, // SYS_AVAILABLE_MEMORY - return 4GB
            10 => 0, // SYS_GETENV - return success
            11 => 0, // SYS_SETENV - return success
            12 => 0, // SYS_UNSETENV - return success
            13 => 0, // SYS_ENVIRON - return success
            20 => 1234, // SYS_GETPID - return PID 1234
            21 => 1, // SYS_GETPPID - return PPID 1
            22 => 0, // SYS_PROCESS_INFO - return success
            else => -1, // Unknown syscall
        };
    }
};

/// Global stdlib core instance for FFI access
var global_stdlib_core: ?StdlibCore = null;

/// Initialize global stdlib core
pub fn init_stdlib_core(allocator: Allocator) void {
    global_stdlib_core = StdlibCore.init(allocator);
}

/// Get global stdlib core instance
pub fn get_stdlib_core() *StdlibCore {
    return &global_stdlib_core.?;
}

// ===== C FFI EXPORT FUNCTIONS =====
// These functions are exported for CURSED stdlib modules to call

export fn runtime_print_string(message_ptr: [*:0]const u8) void {
    if (global_stdlib_core) |*core| {
        const message = std.mem.span(message_ptr);
        core.print_string(message);
    }
}

export fn runtime_string_char_at(string_ptr: [*:0]const u8, index: usize) u8 {
    if (global_stdlib_core) |*core| {
        const string = std.mem.span(string_ptr);
        return core.string_char_at(string, index);
    }
    return 0;
}

export fn runtime_char_to_string(char: u8) [*:0]const u8 {
    if (global_stdlib_core) |*core| {
        const string = core.char_to_string(char) catch return "";
        // Note: In production, this would need proper memory management
        return @ptrCast(string.ptr);
    }
    return "";
}

export fn runtime_string_to_int(string_ptr: [*:0]const u8) i64 {
    if (global_stdlib_core) |*core| {
        const string = std.mem.span(string_ptr);
        return core.string_to_int(string) catch 0;
    }
    return 0;
}

export fn runtime_int_to_string(value: i64) [*:0]const u8 {
    if (global_stdlib_core) |*core| {
        const string = core.int_to_string(value) catch return "";
        // Note: In production, this would need proper memory management
        return @ptrCast(string.ptr);
    }
    return "";
}

export fn runtime_read_file(filename_ptr: [*:0]const u8) [*:0]const u8 {
    if (global_stdlib_core) |*core| {
        const filename = std.mem.span(filename_ptr);
        const content = core.read_file_content(filename) catch return "";
        // Note: In production, this would need proper memory management
        return @ptrCast(content.ptr);
    }
    return "";
}

export fn runtime_write_file(filename_ptr: [*:0]const u8, content_ptr: [*:0]const u8) bool {
    if (global_stdlib_core) |*core| {
        const filename = std.mem.span(filename_ptr);
        const content = std.mem.span(content_ptr);
        return core.write_file_content(filename, content) catch false;
    }
    return false;
}

export fn runtime_file_exists(filename_ptr: [*:0]const u8) bool {
    if (global_stdlib_core) |*core| {
        const filename = std.mem.span(filename_ptr);
        return core.file_exists(filename);
    }
    return false;
}

export fn runtime_current_time_nanos() i64 {
    if (global_stdlib_core) |*core| {
        return core.get_timestamp();
    }
    return 0;
}

export fn runtime_read_char() u8 {
    if (global_stdlib_core) |*core| {
        return core.read_char() catch 0;
    }
    return 0;
}

export fn runtime_abs_int(value: i64) i64 {
    if (global_stdlib_core) |*core| {
        return core.abs_int(value);
    }
    return value;
}

export fn runtime_abs_float(value: f64) f64 {
    if (global_stdlib_core) |*core| {
        return core.abs_float(value);
    }
    return value;
}

export fn runtime_sqrt(value: f64) f64 {
    if (global_stdlib_core) |*core| {
        return core.sqrt(value);
    }
    return 0.0;
}

export fn runtime_sin(value: f64) f64 {
    if (global_stdlib_core) |*core| {
        return core.sin(value);
    }
    return 0.0;
}

export fn runtime_cos(value: f64) f64 {
    if (global_stdlib_core) |*core| {
        return core.cos(value);
    }
    return 0.0;
}

/// Test the stdlib core functionality
pub fn test_stdlib_core() !void {
    print("\n🧪 Testing CURSED Stdlib Core Implementation\n", .{});
    print("=========================================\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize core
    init_stdlib_core(allocator);
    var core = get_stdlib_core();
    
    // Test string functions
    print("Testing string functions...\n", .{});
    const test_string = "Hello";
    const char_at_2 = core.string_char_at(test_string, 2);
    print("Character at index 2 of 'Hello': {c}\n", .{char_at_2});
    
    const int_string = try core.int_to_string(42);
    defer allocator.free(int_string);
    print("Integer 42 as string: {s}\n", .{int_string});
    
    // Test math functions
    print("Testing math functions...\n", .{});
    print("abs(-5): {}\n", .{core.abs_int(-5)});
    print("sqrt(16): {}\n", .{core.sqrt(16.0)});
    print("sin(0): {}\n", .{core.sin(0.0)});
    
    // Test file operations
    print("Testing file operations...\n", .{});
    const test_file = "test_stdlib_core.txt";
    const test_content = "Hello from CURSED stdlib core!";
    
    if (try core.write_file_content(test_file, test_content)) {
        print("Successfully wrote test file\n", .{});
        
        if (core.file_exists(test_file)) {
            print("File exists check: passed\n", .{});
            
            const read_content = try core.read_file_content(test_file);
            defer allocator.free(read_content);
            print("Read content: {s}\n", .{read_content});
            
            if (core.delete_file(test_file)) {
                print("Successfully deleted test file\n", .{});
            }
        }
    }
    
    print("\n✅ Stdlib Core tests completed successfully\n", .{});
}
