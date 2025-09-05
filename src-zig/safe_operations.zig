const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const crash_handler = @import("crash_handler.zig");
const error_handling = @import("error_handling.zig");

/// Safe Operations Module for CURSED Compiler
/// Provides crash-safe wrappers for critical operations that might fail

pub const SafeOperationResult = union(enum) {
    Success: void,
    PartialSuccess: []const u8, // Partial result with explanation
    Failure: []const u8,       // Error message
    
    pub fn isOk(self: SafeOperationResult) bool {
        return switch (self) {
            .Success => true,
            .PartialSuccess => true,
            .Failure => false,
        };
    }
    
    pub fn getMessage(self: SafeOperationResult) ?[]const u8 {
        return switch (self) {
            .Success => null,
            .PartialSuccess => |msg| msg,
            .Failure => |msg| msg,
        };
    }
};

pub const SafeMemoryManager = struct {
    allocator: Allocator,
    telemetry: *crash_handler.CrashTelemetry,
    memory_detector: crash_handler.MemoryErrorDetector,
    
    pub fn init(allocator: Allocator, telemetry: *crash_handler.CrashTelemetry) SafeMemoryManager {
        return SafeMemoryManager{
            .allocator = allocator,
            .telemetry = telemetry,
            .memory_detector = crash_handler.MemoryErrorDetector.init(allocator),
        };
    }
    
    pub fn deinit(self: *SafeMemoryManager) void {
        self.memory_detector.deinit(self.allocator);
    }
    
    /// Safe memory allocation with error tracking
    pub fn safeAlloc(self: *SafeMemoryManager, comptime T: type, count: usize, comptime file: []const u8, comptime line: u32) ![]T {
        const bytes_needed = @sizeOf(T) * count;
        
        // Check for reasonable allocation size
        if (bytes_needed > 1024 * 1024 * 100) { // 100MB limit
            try self.recordError("Large allocation attempted", file, line);
            return error.AllocationTooLarge;
        }
        
        const memory = self.allocator.alloc(T, count) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Memory allocation failed: {any}", .{err});
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return err;
        };
        
        // Track allocation
        try self.memory_detector.trackAllocation(@intFromPtr(memory.ptr), bytes_needed, file, line);
        
        return memory;
    }
    
    /// Safe memory deallocation with tracking
    pub fn safeFree(self: *SafeMemoryManager, memory: anytype) void {
        const ptr_int = @intFromPtr(memory.ptr);
        self.memory_detector.trackDeallocation(ptr_int);
        self.allocator.free(memory);
    }
    
    /// Safe string duplication
    pub fn safeDupeString(self: *SafeMemoryManager, str: []const u8, comptime file: []const u8, comptime line: u32) ![]u8 {
        if (str.len == 0) return try self.allocator.dupe(u8, "");
        
        if (str.len > 1024 * 1024) { // 1MB string limit
            try self.recordError("String too large for duplication", file, line);
            return error.StringTooLarge;
        }
        
        const result = self.allocator.dupe(u8, str) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "String duplication failed: {any}", .{err});
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return err;
        };
        
        try self.memory_detector.trackAllocation(@intFromPtr(result.ptr), result.len, file, line);
        return result;
    }
    
    fn recordError(self: *SafeMemoryManager, message: []const u8, file: []const u8, line: u32) !void {
        var context = try crash_handler.CrashContext.init(
            self.allocator,
            .Error,
            message,
            file,
            line,
            0,
            "SafeMemoryManager"
        );
        defer context.deinit();
        
        try self.telemetry.recordCrash(context);
    }
    
    pub fn getMemoryStats(self: *SafeMemoryManager) struct { current: usize, peak: usize } {
        return .{
            .current = self.memory_detector.getCurrentUsage(),
            .peak = self.memory_detector.getPeakUsage(),
        };
    }
};

pub const SafeFileOperations = struct {
    allocator: Allocator,
    telemetry: *crash_handler.CrashTelemetry,
    
    pub fn init(allocator: Allocator, telemetry: *crash_handler.CrashTelemetry) SafeFileOperations {
        return SafeFileOperations{
            .allocator = allocator,
            .telemetry = telemetry,
        };
    }
    
    /// Safe file reading with error recovery
    pub fn safeReadFile(self: *SafeFileOperations, path: []const u8, comptime file: []const u8, comptime line: u32) ![]u8 {
        // Check if file exists first
        std.fs.cwd().access(path, .{}) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "File access failed for '{s}': {any}", .{ path, err });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return err;
        };
        
        // Get file size to check if reasonable
        const file_stat = std.fs.cwd().statFile(path) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "File stat failed for '{s}': {any}", .{ path, err });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return err;
        };
        
        if (file_stat.size > 10 * 1024 * 1024) { // 10MB limit
            const error_msg = try std.fmt.allocPrint(self.allocator, "File too large: {s} ({d} bytes)", .{ path, file_stat.size });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return error.FileTooLarge;
        }
        
        const contents = std.fs.cwd().readFileAlloc(self.allocator, path, file_stat.size) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "File read failed for '{s}': {any}", .{ path, err });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return err;
        };
        
        return contents;
    }
    
    /// Safe file writing with backup
    pub fn safeWriteFile(self: *SafeFileOperations, path: []const u8, contents: []const u8, comptime file: []const u8, comptime line: u32) !void {
        // Create backup if file exists
        var backup_created = false;
        if (std.fs.cwd().access(path, .{})) |_| {
            const backup_path = try std.fmt.allocPrint(self.allocator, "{s}.backup", .{path});
            defer self.allocator.free(backup_path);
            
            std.fs.cwd().copyFile(path, std.fs.cwd(), backup_path, .{}) catch |err| {
                const error_msg = try std.fmt.allocPrint(self.allocator, "Backup creation failed for '{s}': {any}", .{ path, err });
                defer self.allocator.free(error_msg);
                try self.recordWarning(error_msg, file, line);
            };
            backup_created = true;
        } else |_| {
            // File doesn't exist, that's ok
        }
        
        // Write file
        std.fs.cwd().writeFile(path, contents) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "File write failed for '{s}': {any}", .{ path, err });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            
            // Restore backup if we created one
            if (backup_created) {
                const backup_path = try std.fmt.allocPrint(self.allocator, "{s}.backup", .{path});
                defer self.allocator.free(backup_path);
                std.fs.cwd().copyFile(backup_path, std.fs.cwd(), path, .{}) catch {};
            }
            
            return err;
        };
        
        // Clean up backup on success
        if (backup_created) {
            const backup_path = try std.fmt.allocPrint(self.allocator, "{s}.backup", .{path});
            defer self.allocator.free(backup_path);
            std.fs.cwd().deleteFile(backup_path) catch {};
        }
    }
    
    fn recordError(self: *SafeFileOperations, message: []const u8, file: []const u8, line: u32) !void {
        var context = try crash_handler.CrashContext.init(
            self.allocator,
            .Error,
            message,
            file,
            line,
            0,
            "SafeFileOperations"
        );
        defer context.deinit();
        
        try self.telemetry.recordCrash(context);
    }
    
    fn recordWarning(self: *SafeFileOperations, message: []const u8, file: []const u8, line: u32) !void {
        var context = try crash_handler.CrashContext.init(
            self.allocator,
            .Warning,
            message,
            file,
            line,
            0,
            "SafeFileOperations"
        );
        defer context.deinit();
        
        try self.telemetry.recordCrash(context);
    }
};

pub const SafeParserOperations = struct {
    allocator: Allocator,
    telemetry: *crash_handler.CrashTelemetry,
    
    pub fn init(allocator: Allocator, telemetry: *crash_handler.CrashTelemetry) SafeParserOperations {
        return SafeParserOperations{
            .allocator = allocator,
            .telemetry = telemetry,
        };
    }
    
    /// Safe AST node allocation with validation
    pub fn safeAllocateNode(self: *SafeParserOperations, comptime T: type, comptime file: []const u8, comptime line: u32) !*T {
        // Validate alignment before allocation
        if (@alignOf(T) > @alignOf(*anyopaque)) {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Invalid alignment for type {s}: {d}", .{ @typeName(T), @alignOf(T) });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return error.InvalidAlignment;
        }
        
        const node = self.allocator.create(T) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "AST node allocation failed for {s}: {any}", .{ @typeName(T), err });
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            return err;
        };
        
        // Validate pointer alignment
        const ptr_int = @intFromPtr(node);
        if (ptr_int % @alignOf(T) != 0) {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Pointer not properly aligned for {s}", .{@typeName(T)});
            defer self.allocator.free(error_msg);
            try self.recordError(error_msg, file, line);
            self.allocator.destroy(node);
            return error.BadAlignment;
        }
        
        return node;
    }
    
    /// Safe token validation
    pub fn validateToken(self: *SafeParserOperations, token: anytype, expected: []const u8, comptime file: []const u8, comptime line: u32) !void {
        _ = token; // Type checking would go here
        _ = expected;
        
        // Placeholder for token validation
        // In a real implementation, this would check token types, values, etc.
        
        // For demonstration, let's say validation passed
        _ = self;
        _ = file;
        _ = line;
    }
    
    fn recordError(self: *SafeParserOperations, message: []const u8, file: []const u8, line: u32) !void {
        var context = try crash_handler.CrashContext.init(
            self.allocator,
            .Error,
            message,
            file,
            line,
            0,
            "SafeParserOperations"
        );
        defer context.deinit();
        
        try self.telemetry.recordCrash(context);
    }
};

/// Safe module loading with error recovery
pub const SafeModuleLoader = struct {
    allocator: Allocator,
    telemetry: *crash_handler.CrashTelemetry,
    file_ops: SafeFileOperations,
    
    pub fn init(allocator: Allocator, telemetry: *crash_handler.CrashTelemetry) SafeModuleLoader {
        return SafeModuleLoader{
            .allocator = allocator,
            .telemetry = telemetry,
            .file_ops = SafeFileOperations.init(allocator, telemetry),
        };
    }
    
    /// Safe module loading with fallback strategies
    pub fn safeLoadModule(self: *SafeModuleLoader, module_name: []const u8, comptime file: []const u8, comptime line: u32) SafeOperationResult {
        // Try primary module path
        const primary_path = std.fmt.allocPrint(self.allocator, "stdlib/{s}/mod.💀", .{module_name}) catch {
            return SafeOperationResult{ .Failure = "Failed to construct module path" };
        };
        defer self.allocator.free(primary_path);
        
        if (self.file_ops.safeReadFile(primary_path, file, line)) |contents| {
            defer self.allocator.free(contents);
            // Module loaded successfully
            return SafeOperationResult.Success;
        } else |_| {
            // Try fallback paths
            const fallback_paths = [_][]const u8{
                std.fmt.allocPrint(self.allocator, "stdlib/{s}.💀", .{module_name}) catch return SafeOperationResult{ .Failure = "Fallback path construction failed" },
std.fmt.allocPrint(self.allocator, "src/{s}.💀", .{module_name}) catch return SafeOperationResult{ .Failure = "Fallback path construction failed" },
            };
            
            for (fallback_paths) |fallback_path| {
                defer self.allocator.free(fallback_path);
                
                if (self.file_ops.safeReadFile(fallback_path, file, line)) |contents| {
                    defer self.allocator.free(contents);
                    const partial_msg = std.fmt.allocPrint(self.allocator, "Module loaded from fallback path: {s}", .{fallback_path}) catch return SafeOperationResult{ .Failure = "Partial message creation failed" };
                    return SafeOperationResult{ .PartialSuccess = partial_msg };
                } else |_| {
                    continue;
                }
            }
            
            // All paths failed
            const error_msg = std.fmt.allocPrint(self.allocator, "Module '{s}' not found in any search path", .{module_name}) catch return SafeOperationResult{ .Failure = "Error message creation failed" };
            return SafeOperationResult{ .Failure = error_msg };
        }
    }
};

/// Convenience functions for safe operations
pub fn SAFE_ALLOC(memory_manager: *SafeMemoryManager, comptime T: type, count: usize, comptime file: []const u8, comptime line: u32) ![]T {
    return try memory_manager.safeAlloc(T, count, file, line);
}

pub fn SAFE_FREE(memory_manager: *SafeMemoryManager, memory: anytype) void {
    memory_manager.safeFree(memory);
}

pub fn SAFE_READ_FILE(file_ops: *SafeFileOperations, path: []const u8, comptime file: []const u8, comptime line: u32) ![]u8 {
    return try file_ops.safeReadFile(path, file, line);
}

pub fn SAFE_WRITE_FILE(file_ops: *SafeFileOperations, path: []const u8, contents: []const u8, comptime file: []const u8, comptime line: u32) !void {
    return try file_ops.safeWriteFile(path, contents, file, line);
}
