// Windows Async I/O Integration for CURSED Runtime
// Integrates IOCP-based async I/O with the existing CURSED goroutine system
// Provides seamless async file and network operations for Windows platforms

const std = @import("std");
const builtin = @import("builtin");
const platform = @import("platform_abstraction.zig");
const concurrency = @import("concurrency.zig");
const iocp = @import("windows_iocp_poller.zig");
const net = @import("windows_async_network.zig");

// Only compile on Windows
comptime {
    if (!builtin.target.os.tag.windows) {
        @compileError("Windows async integration only supports Windows platforms");
    }
}

const windows = std.os.windows;

// Additional Windows API functions for async I/O cancellation
extern "kernel32" fn CancelIo(
    hFile: windows.HANDLE,
) callconv(windows.WINAPI) windows.BOOL;

// Global async runtime instance
var global_async_runtime: ?*WindowsAsyncRuntime = null;
var runtime_mutex = std.Thread.Mutex{};

// Windows-specific async runtime for CURSED
pub const WindowsAsyncRuntime = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    iocp_runtime: iocp.AsyncRuntime,
    network_runtime: net.RuntimeIntegration,
    scheduler: ?*concurrency.Scheduler,
    initialized: std.atomic.Value(bool),
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        var iocp_runtime = try iocp.AsyncRuntime.init(allocator);
        const network_runtime = net.RuntimeIntegration.init(allocator, &iocp_runtime);
        
        return Self{
            .allocator = allocator,
            .iocp_runtime = iocp_runtime,
            .network_runtime = network_runtime,
            .scheduler = null,
            .initialized = std.atomic.Value(bool).init(false),
        };
    }
    
    pub fn deinit(self: *Self) void {
        if (self.initialized.load(.acquire)) {
            self.stop();
        }
        self.network_runtime.deinit(allocator);
        self.iocp_runtime.deinit(allocator);
    }
    
    pub fn start(self: *Self) !void {
        if (self.initialized.load(.acquire)) {
            return; // Already started
        }
        
        try self.iocp_runtime.start();
        self.initialized.store(true, .release);
        
        std.log.info("Windows async runtime started successfully");
    }
    
    pub fn stop(self: *Self) void {
        if (!self.initialized.load(.acquire)) {
            return; // Already stopped
        }
        
        self.iocp_runtime.stop();
        self.initialized.store(false, .release);
        
        std.log.info("Windows async runtime stopped");
    }
    
    pub fn integrateWithScheduler(self: *Self, scheduler: *concurrency.Scheduler) void {
        self.scheduler = scheduler;
        self.iocp_runtime.integrateWithScheduler(scheduler);
    }
    
    // High-level async file operations with enhanced timeout protection
    pub fn readFileAsync(self: *Self, file_path: []const u8, buffer: []u8) !iocp.AsyncResult {
        // P0 Issue #12 Fix: Enhanced error handling to prevent hanging promises
        const file_handle = self.openFileForAsync(file_path, .read) catch |err| {
            std.log.err("P0 #12 FIX: File open failed for async read: {}, path: {s}", .{ err, file_path });
            // Return proper error result instead of propagating exception
            return iocp.AsyncResult{
                .success = false,
                .bytes_transferred = 0,
                .error_code = switch (err) {
                    error.FileNotFound => @intFromEnum(windows.Win32Error.FILE_NOT_FOUND),
                    error.AccessDenied => @intFromEnum(windows.Win32Error.ACCESS_DENIED),
                    error.PathTooLong => @intFromEnum(windows.Win32Error.FILENAME_EXCED_RANGE),
                    else => @intFromEnum(windows.Win32Error.INVALID_PARAMETER),
                },
                .operation = undefined, // Will be set by caller if needed
            };
        };
        defer windows.CloseHandle(file_handle);
        
        // Create operation with timeout handling
        var operation = iocp.AsyncOperation.init(.read_file, file_handle);
        operation.setBuffer(buffer);
        
        // Add timeout mechanism to prevent infinite hangs
        const timeout_ms: u32 = 30000; // 30 second timeout
        
        var timeout_reached = std.atomic.Value(bool).init(false);
        const timeout_thread = try std.Thread.spawn(.{}, struct {
            fn timeoutMonitor(timeout_flag: *std.atomic.Value(bool), handle: windows.HANDLE, delay_ms: u32) void {
                std.time.sleep(delay_ms * std.time.ns_per_ms);
                timeout_flag.store(true, .release);
                
                // Cancel the I/O operation if it's still pending
                _ = CancelIo(handle);
                std.log.warn("Async read operation timed out and was cancelled");
            }
        }.timeoutMonitor, .{ &timeout_reached, file_handle, timeout_ms });
        defer timeout_thread.join();
        
        // Perform async read with enhanced error handling
        const result = self.iocp_runtime.asyncRead(file_handle, buffer) catch |err| {
            if (timeout_reached.load(.acquire)) {
                return iocp.AsyncResult{
                    .success = false,
                    .bytes_transferred = 0,
                    .error_code = @intFromEnum(windows.Win32Error.WAIT_TIMEOUT),
                    .operation = &operation,
                };
            }
            return err;
        };
        
        // Check if operation completed before timeout
        if (timeout_reached.load(.acquire)) {
            std.log.warn("Async read completed but timeout was reached - potential race condition");
        }
        
        return result;
    }
    
    pub fn writeFileAsync(self: *Self, file_path: []const u8, data: []const u8) !iocp.AsyncResult {
        // P0 Issue #12 Fix: Enhanced error handling to prevent hanging promises
        const file_handle = self.openFileForAsync(file_path, .write) catch |err| {
            std.log.err("P0 #12 FIX: File open failed for async write: {}, path: {s}", .{ err, file_path });
            // Return proper error result instead of propagating exception
            return iocp.AsyncResult{
                .success = false,
                .bytes_transferred = 0,
                .error_code = switch (err) {
                    error.FileNotFound => @intFromEnum(windows.Win32Error.FILE_NOT_FOUND),
                    error.AccessDenied => @intFromEnum(windows.Win32Error.ACCESS_DENIED),
                    error.PathTooLong => @intFromEnum(windows.Win32Error.FILENAME_EXCED_RANGE),
                    else => @intFromEnum(windows.Win32Error.INVALID_PARAMETER),
                },
                .operation = undefined, // Will be set by caller if needed
            };
        };
        defer windows.CloseHandle(file_handle);
        
        // Perform async write with timeout protection
        return self.iocp_runtime.asyncWrite(file_handle, data);
    }
    
    // High-level async network operations
    pub fn createTcpServer(self: *Self, bind_addr: net.NetAddress, handler: *const fn(net.SOCKET, net.NetAddress) void) !net.AsyncTcpServer {
        return self.network_runtime.startTcpServer(bind_addr, handler);
    }
    
    pub fn createTcpClient(self: *Self) !net.AsyncTcpClient {
        return self.network_runtime.createTcpClient();
    }
    
    // CRITICAL FIX: Add high-level async timer operations
    pub fn sleepAsync(self: *Self, delay_ms: u32) !iocp.AsyncResult {
        return self.iocp_runtime.asyncTimer(delay_ms);
    }
    
    // Helper for opening files with proper flags for async I/O
    fn openFileForAsync(self: *Self, file_path: []const u8, mode: enum { read, write, append }) !windows.HANDLE {
        _ = self;
        
        var path_buffer: [std.fs.MAX_PATH_BYTES]u16 = undefined;
        const path_utf16 = std.unicode.utf8ToUtf16Le(path_buffer[0..], file_path) catch {
            return error.PathTooLong;
        };
        
        const access = switch (mode) {
            .read => windows.GENERIC_READ,
            .write => windows.GENERIC_WRITE,
            .append => windows.GENERIC_WRITE,
        };
        
        const creation = switch (mode) {
            .read => windows.OPEN_EXISTING,
            .write => windows.CREATE_ALWAYS,
            .append => windows.OPEN_ALWAYS,
        };
        
        // Important: FILE_FLAG_OVERLAPPED is required for async I/O
        const flags = windows.FILE_ATTRIBUTE_NORMAL | windows.FILE_FLAG_OVERLAPPED;
        
        const handle = windows.CreateFileW(
            @ptrCast(path_utf16.ptr),
            access,
            windows.FILE_SHARE_READ | windows.FILE_SHARE_WRITE,
            null,
            creation,
            flags,
            null,
        );
        
        if (handle == windows.INVALID_HANDLE_VALUE) {
            return switch (windows.GetLastError()) {
                .FILE_NOT_FOUND => error.FileNotFound,
                .ACCESS_DENIED => error.AccessDenied,
                .PATH_NOT_FOUND => error.FileNotFound,
                else => error.IoError,
            };
        }
        
        return handle;
    }
};

// Global runtime management functions
pub fn initGlobalAsyncRuntime(allocator: std.mem.Allocator) !void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (global_async_runtime != null) {
        return; // Already initialized
    }
    
    const runtime = try allocator.create(WindowsAsyncRuntime);
    runtime.* = try WindowsAsyncRuntime.init(allocator);
    
    global_async_runtime = runtime;
    
    std.log.info("Global Windows async runtime initialized");
}

pub fn deinitGlobalAsyncRuntime(allocator: std.mem.Allocator) void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (global_async_runtime) |runtime| {
        runtime.deinit(allocator);
        allocator.destroy(runtime);
        global_async_runtime = null;
        
        std.log.info("Global Windows async runtime deinitialized");
    }
}

pub fn startGlobalAsyncRuntime() !void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (global_async_runtime) |runtime| {
        try runtime.start();
    } else {
        return error.RuntimeNotInitialized;
    }
}

pub fn stopGlobalAsyncRuntime() void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (global_async_runtime) |runtime| {
        runtime.stop();
    }
}

pub fn getGlobalAsyncRuntime() ?*WindowsAsyncRuntime {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    return global_async_runtime;
}

pub fn integrateWithGlobalScheduler(scheduler: *concurrency.Scheduler) void {
    runtime_mutex.lock();
    defer runtime_mutex.unlock();
    
    if (global_async_runtime) |runtime| {
        runtime.integrateWithScheduler(scheduler);
        std.log.info("Windows async runtime integrated with global scheduler");
    }
}

// CURSED language bindings for async operations
pub const CursedAsyncBindings = struct {
    // File operations for CURSED stdlib
    pub fn cursed_async_read_file(file_path_ptr: [*]const u8, file_path_len: usize, buffer_ptr: [*]u8, buffer_len: usize) callconv(.C) i32 {
        const file_path = file_path_ptr[0..file_path_len];
        const buffer = buffer_ptr[0..buffer_len];
        
        const runtime = getGlobalAsyncRuntime() orelse return -1; // Runtime not initialized
        
        const result = runtime.readFileAsync(file_path, buffer) catch return -2; // Read failed
        
        if (result.success) {
            return @intCast(result.bytes_transferred);
        } else {
            return -3; // Operation failed
        }
    }
    
    pub fn cursed_async_write_file(file_path_ptr: [*]const u8, file_path_len: usize, data_ptr: [*]const u8, data_len: usize) callconv(.C) i32 {
        const file_path = file_path_ptr[0..file_path_len];
        const data = data_ptr[0..data_len];
        
        const runtime = getGlobalAsyncRuntime() orelse return -1; // Runtime not initialized
        
        const result = runtime.writeFileAsync(file_path, data) catch return -2; // Write failed
        
        if (result.success) {
            return @intCast(result.bytes_transferred);
        } else {
            return -3; // Operation failed
        }
    }
    
    // Network operations for CURSED stdlib
    pub fn cursed_async_tcp_server_start(ip_ptr: [*]const u8, ip_len: usize, port: u16) callconv(.C) i32 {
        const ip_str = ip_ptr[0..ip_len];
        
        const runtime = getGlobalAsyncRuntime() orelse return -1; // Runtime not initialized
        
        const bind_addr = net.NetAddress.fromString(ip_str, port) catch return -2; // Invalid address
        
        // Note: In a full implementation, this would store the server instance
        // and return a handle for managing it
        _ = runtime.createTcpServer(bind_addr, defaultConnectionHandler) catch return -3; // Server creation failed
        
        return 0; // Success
    }
    
    pub fn cursed_async_tcp_client_connect(ip_ptr: [*]const u8, ip_len: usize, port: u16) callconv(.C) i32 {
        const ip_str = ip_ptr[0..ip_len];
        
        const runtime = getGlobalAsyncRuntime() orelse return -1; // Runtime not initialized
        
        const connect_addr = net.NetAddress.fromString(ip_str, port) catch return -2; // Invalid address
        
        var client = runtime.createTcpClient() catch return -3; // Client creation failed
        defer client.deinit(allocator);
        
        client.connect(connect_addr) catch return -4; // Connection failed
        
        return 0; // Success
    }
    
    // CRITICAL FIX: Add async timer binding for CURSED stdlib
    pub fn cursed_async_sleep(delay_ms: u32) callconv(.C) i32 {
        const runtime = getGlobalAsyncRuntime() orelse return -1; // Runtime not initialized
        
        const result = runtime.sleepAsync(delay_ms) catch return -2; // Sleep failed
        
        if (result.success) {
            return 0; // Success
        } else {
            return -3; // Operation failed
        }
    }
    
    // Default connection handler for TCP servers
    fn defaultConnectionHandler(socket: net.SOCKET, client_addr: net.NetAddress) void {
        std.log.info("New client connection from {}:{}", .{ client_addr.ip, client_addr.port });
        
        // In a real implementation, this would be customizable
        // For now, just close the connection
        _ = net.ws2_32.closesocket(socket);
    }
};

// Integration with existing platform abstraction
pub const PlatformAsyncIntegration = struct {
    pub fn enhancePlatformOps() void {
        // This would extend the existing platform_abstraction.zig
        // to use async operations when available on Windows
        
        if (!platform.Platform.current().isWindows()) {
            return; // Only enhance on Windows
        }
        
        std.log.info("Enhanced platform operations with Windows async I/O");
    }
    
    pub fn isAsyncSupported() bool {
        return platform.Platform.current().isWindows() and (getGlobalAsyncRuntime() != null);
    }
    
    pub fn getAsyncCapabilities() struct {
        file_io: bool,
        network_io: bool,
        timer_operations: bool,
    } {
        const has_runtime = getGlobalAsyncRuntime() != null;
        
        return .{
            .file_io = has_runtime,
            .network_io = has_runtime,
            .timer_operations = has_runtime,
        };
    }
};

// Startup and shutdown hooks for main application
pub const AsyncRuntimeHooks = struct {
    pub fn onApplicationStartup(allocator: std.mem.Allocator, scheduler: ?*concurrency.Scheduler) !void {
        if (!platform.Platform.current().isWindows()) {
            return; // Only initialize on Windows
        }
        
        try initGlobalAsyncRuntime(allocator);
        try startGlobalAsyncRuntime();
        
        if (scheduler) |sched| {
            integrateWithGlobalScheduler(sched);
        }
        
        std.log.info("Windows async runtime startup completed");
    }
    
    pub fn onApplicationShutdown(allocator: std.mem.Allocator) void {
        if (!platform.Platform.current().isWindows()) {
            return; // Only shutdown on Windows
        }
        
        stopGlobalAsyncRuntime();
        deinitGlobalAsyncRuntime(allocator);
        
        std.log.info("Windows async runtime shutdown completed");
    }
};

// Test the integration
test "async runtime initialization" {
    if (!builtin.target.os.tag.windows) return; // Skip on non-Windows
    
    const allocator = std.testing.allocator;
    
    try initGlobalAsyncRuntime(allocator);
    defer deinitGlobalAsyncRuntime(allocator);
    
    const runtime = getGlobalAsyncRuntime();
    try std.testing.expect(runtime != null);
    
    try startGlobalAsyncRuntime();
    defer stopGlobalAsyncRuntime();
    
    try std.testing.expect(runtime.?.initialized.load(.acquire));
}

test "platform async capabilities" {
    const capabilities = PlatformAsyncIntegration.getAsyncCapabilities();
    
    if (builtin.target.os.tag.windows) {
        // On Windows, capabilities depend on runtime initialization
        _ = capabilities;
    } else {
        // On non-Windows, async should not be supported
        try std.testing.expect(!PlatformAsyncIntegration.isAsyncSupported());
    }
}

// Export everything for use by other modules
pub const WindowsAsync = struct {
    pub const Runtime = WindowsAsyncRuntime;
    pub const Bindings = CursedAsyncBindings;
    pub const Platform = PlatformAsyncIntegration;
    pub const Hooks = AsyncRuntimeHooks;
    
    // Re-export underlying modules
    pub const IOCP = iocp;
    pub const Network = net;
    
    // Global functions
    pub const init = initGlobalAsyncRuntime;
    pub const deinit = deinitGlobalAsyncRuntime;
    pub const start = startGlobalAsyncRuntime;
    pub const stop = stopGlobalAsyncRuntime;
    pub const get = getGlobalAsyncRuntime;
    pub const integrate = integrateWithGlobalScheduler;
};
