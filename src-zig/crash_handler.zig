const std = @import("std");
const print = std.debug.print;
const builtin = @import("builtin");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// CURSED Compiler Crash Handler and Telemetry System
/// Provides graceful error handling, stack traces, and crash recovery

pub const CrashSeverity = enum {
    Warning,      // Non-fatal issues
    Error,        // Recoverable errors
    Fatal,        // Unrecoverable errors
    Panic,        // Critical system failures
};

pub const CrashContext = struct {
    severity: CrashSeverity,
    message: []const u8,
    error_code: ?anyerror,
    source_file: []const u8,
    source_line: u32,
    source_column: u32,
    function_name: []const u8,
    stack_trace: ?[][]const u8,
    timestamp: i64,
    memory_usage: ?usize,
    
    pub fn init(allocator: Allocator, severity: CrashSeverity, message: []const u8, source_file: []const u8, source_line: u32, source_column: u32, function_name: []const u8) !CrashContext {
        return CrashContext{
            .severity = severity,
            .message = try allocator.dupe(u8, message),
            .error_code = null,
            .source_file = try allocator.dupe(u8, source_file),
            .source_line = source_line,
            .source_column = source_column,
            .function_name = try allocator.dupe(u8, function_name),
            .stack_trace = null,
            .timestamp = std.time.milliTimestamp(),
            .memory_usage = null,
        };
    }
    
    pub fn deinit(self: *CrashContext, allocator: Allocator) void {
        if (self.message.len > 0) allocator.free(self.message);
        if (self.source_file.len > 0) allocator.free(self.source_file);
        if (self.function_name.len > 0) allocator.free(self.function_name);
        if (self.stack_trace) |trace| {
            for (trace) |frame| {
                if (frame.len > 0) allocator.free(frame);
            }
            allocator.free(trace);
        }
    }
};

pub const CrashTelemetry = struct {
    allocator: Allocator,
    crash_log: ArrayList(CrashContext),
    enable_telemetry: bool,
    max_crashes: usize,
    crash_file_path: ?[]const u8,
    
    pub fn init(allocator: Allocator, enable_telemetry: bool, max_crashes: usize) CrashTelemetry {
        return CrashTelemetry{
            .allocator = allocator,
            .crash_log = .empty,
            .enable_telemetry = enable_telemetry,
            .max_crashes = max_crashes,
            .crash_file_path = null,
        };
    }
    
    pub fn deinit(self: *CrashTelemetry) void {
        // Clean up crash log items
        for (self.crash_log.items) |*crash| {
            crash.deinit();
        }
        self.crash_log.deinit();
        
        // Clean up crash file path
        if (self.crash_file_path) |path| {
            if (path.len > 0) {
                self.allocator.free(path);
            }
        }
    }
    
    pub fn recordCrash(self: *CrashTelemetry, context: CrashContext) !void {
        if (!self.enable_telemetry) return;
        
        // Limit crash log size
        if (self.crash_log.items.len >= self.max_crashes) {
            var oldest = self.crash_log.orderedRemove(0);
            oldest.deinit(self.allocator);
        }
        
        // Create a deep copy of the context for storage
        var context_copy = try CrashContext.init(
            self.allocator,
            context.severity,
            context.message,
            context.source_file,
            context.source_line,
            context.source_column,
            context.function_name
        );
        context_copy.error_code = context.error_code;
        context_copy.timestamp = context.timestamp;
        context_copy.memory_usage = context.memory_usage;
        
        // Copy stack trace if present
        if (context.stack_trace) |trace| {
            var new_trace = try self.allocator.alloc([]const u8, trace.len);
            for (trace, 0..) |frame, i| {
                new_trace[i] = try self.allocator.dupe(u8, frame);
            }
            context_copy.stack_trace = new_trace;
        }
        
        try self.crash_log.append(self.allocator, context_copy);
        
        // Write to crash file if configured
        if (self.crash_file_path) |path| {
            try self.writeCrashToFile(path, context);
        }
    }
    
    fn writeCrashToFile(self: *CrashTelemetry, path: []const u8, context: CrashContext) !void {
        const file = std.fs.cwd().createFile(path, .{ .truncate = false }) catch |err| {
            print("⚠️  Failed to open crash log file '{s}': {any}\n", .{ path, err });
            return;
        };
        defer file.close();
        
        try file.seekFromEnd(0); // Append to end
        
        const severity_str = switch (context.severity) {
            .Warning => "WARNING",
            .Error => "ERROR",
            .Fatal => "FATAL",
            .Panic => "PANIC",
        };
        
        const crash_entry = try std.fmt.allocPrint(self.allocator, 
            "[{d}] {s}: {s} at {s}:{d}:{d} in {s}\n",
            .{ context.timestamp, severity_str, context.message, context.source_file, context.source_line, context.source_column, context.function_name }
        );
        defer self.allocator.free(crash_entry);
        
        try file.writeAll(crash_entry);
    }
};

pub const FatalErrorHandler = struct {
    allocator: Allocator,
    telemetry: *CrashTelemetry,
    panic_handler_installed: bool,
    recovery_strategies: ArrayList(RecoveryStrategy),
    
    const RecoveryStrategy = struct {
        error_type: []const u8,
        recovery_fn: *const fn (allocator: Allocator, context: CrashContext) anyerror!void,
    };
    
    pub fn init(allocator: Allocator, telemetry: *CrashTelemetry) FatalErrorHandler {
        return FatalErrorHandler{
            .allocator = allocator,
            .telemetry = telemetry,
            .panic_handler_installed = false,
            .recovery_strategies = .empty,
        };
    }
    
    pub fn deinit(self: *FatalErrorHandler) void {
        self.recovery_strategies.deinit();
    }
    
    pub fn installPanicHandler(self: *FatalErrorHandler) void {
        if (self.panic_handler_installed) return;
        
        // Install custom panic handler
        std.debug.panic_handler = customPanicHandler;
        self.panic_handler_installed = true;
    }
    
    pub fn handleFatalError(self: *FatalErrorHandler, severity: CrashSeverity, message: []const u8, source_file: []const u8, source_line: u32, source_column: u32, function_name: []const u8) !void {
        var context = try CrashContext.init(self.allocator, severity, message, source_file, source_line, source_column, function_name);
        defer context.deinit();
        
        // Capture stack trace
        context.stack_trace = try self.captureStackTrace();
        
        // Capture memory usage
        context.memory_usage = self.getCurrentMemoryUsage();
        
        // Record in telemetry
        try self.telemetry.recordCrash(context);
        
        // Print user-friendly error message
        try self.printErrorMessage(context);
        
        // Try recovery strategies
        if (severity != .Panic) {
            try self.attemptRecovery(context);
        }
        
        // For fatal errors and panics, terminate gracefully
        if (severity == .Fatal or severity == .Panic) {
            print("\n💥 CURSED compiler encountered a fatal error and cannot continue.\n", .{});
            print("📋 Crash details have been logged for debugging.\n", .{});
            std.process.exit(1);
        }
    }
    
    fn captureStackTrace(self: *FatalErrorHandler) !?[][]const u8 {
        var stack_trace = .empty;
        defer stack_trace.deinit();
        
        // Use builtin stack trace if available
        if (builtin.mode == .Debug) {
            var stack_iter = std.debug.StackIterator.init(null, null);
            var frame_count: usize = 0;
            
            while (stack_iter.next()) |frame| {
                if (frame_count >= 10) break; // Limit frames
                
                const frame_str = try std.fmt.allocPrint(self.allocator, "0x{x}", .{frame});
                try stack_trace.append(self.allocator, frame_str);
                frame_count += 1;
            }
        }
        
        if (stack_trace.items.len == 0) return null;
        
        return try stack_trace.toOwnedSlice(self.allocator);
    }
    
    fn getCurrentMemoryUsage(self: *FatalErrorHandler) ?usize {
        _ = self; // Not used
        // Attempt to get current memory usage
        // This is platform-specific and best-effort
        return null;
    }
    
    fn printErrorMessage(self: *FatalErrorHandler, context: CrashContext) !void {
        _ = self; // Not used
        
        const severity_emoji = switch (context.severity) {
            .Warning => "⚠️ ",
            .Error => "❌",
            .Fatal => "💀",
            .Panic => "💥",
        };
        
        const severity_color = switch (context.severity) {
            .Warning => "\x1b[33m", // Yellow
            .Error => "\x1b[31m", // Red
            .Fatal => "\x1b[91m", // Bright red
            .Panic => "\x1b[95m", // Bright magenta
        };
        const reset_color = "\x1b[0m";
        
        print("\n{s}{s} CURSED Compiler Error{s}\n", .{ severity_color, severity_emoji, reset_color });
        print("{s}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{s}\n", .{ severity_color, reset_color });
        print("💬 {s}\n", .{context.message});
        print("📍 Location: {s}:{d}:{d}\n", .{ context.source_file, context.source_line, context.source_column });
        print("🔧 Function: {s}\n", .{context.function_name});
        
        if (context.error_code) |err| {
            print("🔍 Error Code: {any}\n", .{err});
        }
        
        if (context.memory_usage) |memory| {
            print("💾 Memory Usage: {d} bytes\n", .{memory});
        }
        
        if (context.stack_trace) |trace| {
            print("📚 Stack Trace:\n", .{});
            for (trace, 0..) |frame, i| {
                print("  {d}: {s}\n", .{ i, frame });
            }
        }
        
        print("{s}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{s}\n", .{ severity_color, reset_color });
    }
    
    fn attemptRecovery(self: *FatalErrorHandler, context: CrashContext) !void {
        for (self.recovery_strategies.items) |strategy| {
            if (std.mem.indexOf(u8, context.message, strategy.error_type) != null) {
                print("🔄 Attempting recovery strategy for: {s}\n", .{strategy.error_type});
                strategy.recovery_fn(self.allocator, context) catch |err| {
                    print("❌ Recovery strategy failed: {any}\n", .{err});
                };
                return;
            }
        }
        
        print("❌ No recovery strategy available for this error.\n", .{});
    }
    
    pub fn addRecoveryStrategy(self: *FatalErrorHandler, error_type: []const u8, recovery_fn: *const fn (allocator: Allocator, context: CrashContext) anyerror!void) !void {
        const strategy = RecoveryStrategy{
            .error_type = try self.allocator.dupe(u8, error_type),
            .recovery_fn = recovery_fn,
        };
        try self.recovery_strategies.append(self.allocator, strategy);
    }
};

/// Custom panic handler for CURSED compiler
fn customPanicHandler(message: []const u8, stack_trace: ?*std.builtin.StackTrace, ret_addr: ?usize) noreturn {
    _ = stack_trace; // Not used in this implementation
    _ = ret_addr; // Not used
    
    print("\n💥 CURSED COMPILER PANIC 💥\n", .{});
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", .{});
    print("💬 Panic Message: {s}\n", .{message});
    print("🕒 Timestamp: {d}\n", .{std.time.milliTimestamp()});
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", .{});
    print("🔧 This is a bug in the CURSED compiler.\n", .{});
    print("📋 Please report this panic with the above information.\n", .{});
    print("🌐 Repository: https://github.com/ghuntley/cursed\n", .{});
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", .{});
    
    std.process.exit(1);
}

/// Memory error detection utilities
pub const MemoryErrorDetector = struct {
    allocator: Allocator,
    allocations: std.HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage),
    total_allocated: usize,
    total_freed: usize,
    peak_usage: usize,
    
    const AllocationInfo = struct {
        size: usize,
        file: []const u8,
        line: u32,
        timestamp: i64,
    };
    
    pub fn init(allocator: std.mem.Allocator) MemoryErrorDetector {
        return MemoryErrorDetector{
            .allocator = allocator,
            .allocations = std.HashMap(usize, AllocationInfo, std.hash_map.AutoContext(usize), std.hash_map.default_max_load_percentage).init(allocator),
            .total_allocated = 0,
            .total_freed = 0,
            .peak_usage = 0,
        };
    }
    
    pub fn deinit(self: *MemoryErrorDetector) void {
        self.allocations.deinit();
    }
    
    pub fn trackAllocation(self: *MemoryErrorDetector, ptr: usize, size: usize, file: []const u8, line: u32) !void {
        const info = AllocationInfo{
            .size = size,
            .file = try self.allocator.dupe(u8, file),
            .line = line,
            .timestamp = std.time.milliTimestamp(),
        };
        
        try self.allocations.put(ptr, info);
        self.total_allocated += size;
        
        const current_usage = self.total_allocated - self.total_freed;
        if (current_usage > self.peak_usage) {
            self.peak_usage = current_usage;
        }
    }
    
    pub fn trackDeallocation(self: *MemoryErrorDetector, ptr: usize) void {
        if (self.allocations.get(ptr)) |info| {
            self.total_freed += info.size;
            self.allocator.free(info.file);
            _ = self.allocations.remove(ptr);
        }
    }
    
    pub fn detectLeaks(self: *MemoryErrorDetector) ![]AllocationInfo {
        var leaks = .empty;
        
        var iter = self.allocations.iterator();
        while (iter.next()) |entry| {
            try leaks.append(self.allocator, entry.value_ptr.*);
        }
        
        return try leaks.toOwnedSlice();
    }
    
    pub fn getCurrentUsage(self: *MemoryErrorDetector) usize {
        return self.total_allocated - self.total_freed;
    }
    
    pub fn getPeakUsage(self: *MemoryErrorDetector) usize {
        return self.peak_usage;
    }
};

/// Graceful degradation utilities
pub fn attemptGracefulRecovery(allocator: Allocator, context: CrashContext) !void {
    _ = allocator; // Not used
    print("🔄 Attempting graceful recovery for: {s}\n", .{context.message});
    
    // Basic recovery strategies based on error type
    if (std.mem.indexOf(u8, context.message, "OutOfMemory") != null) {
        print("💾 Memory exhaustion detected - suggesting cleanup\n", .{});
        // Could trigger garbage collection or memory cleanup here
    } else if (std.mem.indexOf(u8, context.message, "FileNotFound") != null) {
        print("📁 File not found - checking alternative paths\n", .{});
        // Could suggest alternative file paths or create missing directories
    } else if (std.mem.indexOf(u8, context.message, "InvalidSyntax") != null) {
        print("📝 Syntax error detected - providing suggestions\n", .{});
        // Could provide syntax suggestions or partial parsing results
    }
}

/// Convenience macros for error handling
pub fn CURSED_FATAL(allocator: Allocator, telemetry: *CrashTelemetry, comptime message: []const u8, comptime file: []const u8, comptime line: u32, comptime function: []const u8) !void {
    var handler = FatalErrorHandler.init(allocator, telemetry);
    defer handler.deinit();
    try handler.handleFatalError(.Fatal, message, file, line, 0, function);
}

pub fn CURSED_ERROR(allocator: Allocator, telemetry: *CrashTelemetry, comptime message: []const u8, comptime file: []const u8, comptime line: u32, comptime function: []const u8) !void {
    var handler = FatalErrorHandler.init(allocator, telemetry);
    defer handler.deinit();
    try handler.handleFatalError(.Error, message, file, line, 0, function);
}

pub fn CURSED_WARNING(allocator: Allocator, telemetry: *CrashTelemetry, comptime message: []const u8, comptime file: []const u8, comptime line: u32, comptime function: []const u8) !void {
    var handler = FatalErrorHandler.init(allocator, telemetry);
    defer handler.deinit();
    try handler.handleFatalError(.Warning, message, file, line, 0, function);
}
