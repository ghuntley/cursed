const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// Production-quality stack trace capture for CURSED error handling
/// Provides real stack frame capture with source line mapping and symbol resolution

pub const StackFrame = struct {
    function_name: []const u8,
    file_name: []const u8,
    line_number: u32,
    column_number: u32,
    address: usize,
    
    pub fn deinit(self: *StackFrame, allocator: Allocator) void {
        _ = allocator;
        allocator.free(self.function_name);
        allocator.free(self.file_name);
    }
    
    pub fn format(self: StackFrame, writer: anytype) !void {
        try writer.print("  at {s}() in {s}:{s}:{s} (0x{x})\n", .{
            self.function_name,
            self.file_name,
            self.line_number,
            self.column_number,
            self.address
        });
    }
};

pub const StackTraceCapture = struct {
    frames: ArrayList(StackFrame),
    allocator: Allocator,
    max_depth: usize,
    
    pub fn init(allocator: Allocator, max_depth: usize) StackTraceCapture {
        return StackTraceCapture{
            .frames = ArrayList(StackFrame){},
            .allocator = allocator,
            .max_depth = max_depth,
        };
    }
    
    pub fn deinit(self: *StackTraceCapture) void {
        for (self.frames.items) |*frame| {
            frame.deinit(self.allocator);
        }
        self.frames.deinit(self.allocator);
    }
    
    /// Capture current stack trace with debug information
    pub fn capture(allocator: Allocator) !StackTraceCapture {
        _ = allocator;
        var trace_capture = StackTraceCapture.init(allocator, 50); // Limit to 50 frames
        
        // Use Zig's built-in stack trace functionality
        var stack_trace = std.debug.StackIterator{
            .instruction_addresses = undefined,
            .index = 0,
        };
        
        std.debug.captureStackTrace(null, &stack_trace);
        
        // Skip first few frames (runtime internals)
        const skip_frames = 2;
        var i: usize = skip_frames;
        
        while (i < stack_trace.index and i < trace_capture.max_depth) : (i += 1) {
            const addr = stack_trace.instruction_addresses[i];
            const frame = try trace_capture.addressToFrame(addr);
            try trace_capture.frames.append(allocator, frame);
        }
        
        return trace_capture;
    }
    
    /// Convert memory address to stack frame with debug information
    fn addressToFrame(self: *StackTraceCapture, address: usize) !StackFrame {
        // Try to get debug information
        if (std.debug.getSelfDebugInfo()) |debug_info| {
            if (debug_info.getSymbolAtAddress(self.allocator, address)) |symbol_info| {
                defer symbol_info.deinit();
                
                const function_name = if (symbol_info.symbol_name) |name|
                    try self.allocator.dupe(u8, name)
                else
                    try self.allocator.dupe(u8, "<unknown_function>");
                
                const file_name = if (symbol_info.file_name) |name|
                    try self.allocator.dupe(u8, std.fs.path.basename(name)) // Just basename
                else
                    try self.allocator.dupe(u8, "<unknown_file>");
                
                const line_number: u32 = if (symbol_info.line_info) |line_info|
                    @as(u32, @intCast(line_info.line))
                else
                    0;
                
                const column_number: u32 = if (symbol_info.line_info) |line_info|
                    @as(u32, @intCast(line_info.column))
                else
                    0;
                
                return StackFrame{
                    .function_name = function_name,
                    .file_name = file_name,
                    .line_number = line_number,
                    .column_number = column_number,
                    .address = address,
                };
            } else |_| {
                // Symbol resolution failed, return generic frame
            }
        } else |_| {
            // No debug info available
        }
        
        // Fallback for when debug info is unavailable
        return StackFrame{
            .function_name = try self.allocator.dupe(u8, "<native>"),
            .file_name = try self.allocator.dupe(u8, "<native>"),
            .line_number = 0,
            .column_number = 0,
            .address = address,
        };
    }
    
    /// Format stack trace as a string
    pub fn toString(self: *StackTraceCapture) ![]u8 {
        var buffer = ArrayList(u8){};
        defer buffer.deinit();
        
        const writer = buffer.writer();
        try writer.print("Stack trace ({s} frames):\n", .{self.frames.items.len});
        
        for (self.frames.items) |frame| {
            try frame.format(writer);
        }
        
        return try self.allocator.dupe(u8, buffer.items);
    }
    
    /// Convert to null-terminated C string for CURSED runtime
    pub fn toCString(self: *StackTraceCapture) ![:0]u8 {
        const str = try self.toString();
        defer self.allocator.free(str);
        
        const c_str = try self.allocator.allocSentinel(u8, str.len, 0);
        @memcpy(c_str[0..str.len], str);
        return c_str;
    }
};

/// Global stack trace capture for C exports
var global_allocator: ?Allocator = null;

pub fn setGlobalAllocator(allocator: Allocator) void {
        _ = allocator;
    global_allocator = allocator;
}

/// C-compatible API for LLVM integration
export fn cursed_runtime_get_stack_trace() callconv(.c) [*:0]const u8 {
    const allocator = global_allocator orelse std.heap.c_allocator;
    
    var capture = StackTraceCapture.capture(allocator) catch {
        const fallback = allocator.dupeZ(u8, "Stack trace capture failed") catch {
            return "Stack trace unavailable";
        };
        return fallback.ptr;
    };
    
    const c_str = capture.toCString() catch {
        capture.deinit();
        const fallback = allocator.dupeZ(u8, "Stack trace formatting failed") catch {
            return "Stack trace unavailable";
        };
        return fallback.ptr;
    };
    
    capture.deinit();
    return c_str.ptr;
}

/// Initialize stack trace system
export fn cursed_stack_trace_init() callconv(.c) void {
    // Initialize debug information parsing
    // Note: openSelfDebugInfo is deprecated, debug info is loaded automatically
    print("Stack trace system initialized\n", .{});
}

/// Cleanup stack trace system
export fn cursed_stack_trace_cleanup() callconv(.c) void {
    // Cleanup would go here if needed
}

/// Capture stack trace at specific context (for error handling)
export fn cursed_capture_error_stack_trace(
    function_name: [*:0]const u8,
    file_name: [*:0]const u8,
    line: u32,
    column: u32
) callconv(.c) [*:0]const u8 {
    const allocator = global_allocator orelse std.heap.c_allocator;
    
    var capture = StackTraceCapture.capture(allocator) catch {
        const fallback = allocator.dupeZ(u8, "Stack trace capture failed") catch {
            return "Stack trace unavailable";
        };
        return fallback.ptr;
    };
    defer capture.deinit();
    
    // Prepend current error location to stack trace
    const func_span = std.mem.span(function_name);
    const file_span = std.mem.span(file_name);
    
    const current_frame = StackFrame{
        .function_name = allocator.dupe(u8, func_span) catch return "Memory error",
        .file_name = allocator.dupe(u8, std.fs.path.basename(file_span)) catch return "Memory error",
        .line_number = line,
        .column_number = column,
        .address = 0,
    };
    
    // Insert at beginning
    capture.frames.insert(0, current_frame) catch {
        const fallback = allocator.dupeZ(u8, "Stack trace insert failed") catch {
            return "Stack trace unavailable";
        };
        return fallback.ptr;
    };
    
    const c_str = capture.toCString() catch {
        const fallback = allocator.dupeZ(u8, "Stack trace formatting failed") catch {
            return "Stack trace unavailable";
        };
        return fallback.ptr;
    };
    
    return c_str.ptr;
}

/// Free stack trace string memory
export fn cursed_free_stack_trace(stack_trace_ptr: [*:0]const u8) callconv(.c) void {
    const allocator = global_allocator orelse std.heap.c_allocator;
    const slice = std.mem.span(stack_trace_ptr);
    if (slice.len > 0) {
        allocator.free(slice);
    }
}

/// Test function to verify stack trace functionality
fn testFunction3() void {
    const allocator = std.testing.allocator;
    var capture = StackTraceCapture.capture(allocator) catch return;
    defer capture.deinit();
    
    const trace_str = capture.toString() catch return;
    defer allocator.free(trace_str);
    
    print("Test stack trace:\n{s}\n", .{trace_str});
}

fn testFunction2() void {
    testFunction3();
}

fn testFunction1() void {
    testFunction2();
}

test "stack trace capture" {
    const allocator = std.testing.allocator;
    
    // Set global allocator for testing
    setGlobalAllocator(allocator);
    
    // Test basic capture
    var capture = try StackTraceCapture.capture(allocator);
    defer capture.deinit();
    
    try std.testing.expect(capture.frames.items.len > 0);
    
    // Test string conversion
    const trace_str = try capture.toString();
    defer allocator.free(trace_str);
    
    try std.testing.expect(trace_str.len > 0);
    try std.testing.expect(std.mem.indexOf(u8, trace_str, "Stack trace") != null);
    
    // Test C string conversion
    const c_str = try capture.toCString();
    defer allocator.free(c_str);
    
    try std.testing.expect(c_str.len > 0);
    try std.testing.expect(c_str[c_str.len] == 0); // Null terminated
}

test "nested function stack trace" {
    testFunction1();
}
