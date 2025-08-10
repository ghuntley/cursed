// Windows I/O Completion Port (IOCP) Async Poller for CURSED Runtime
// Provides high-performance async I/O on Windows platforms
// Integrates with the existing Zig runtime and goroutine system

const std = @import("std");
const builtin = @import("builtin");
const platform = @import("platform_abstraction.zig");
const concurrency = @import("concurrency.zig");

// Only compile on Windows
comptime {
    if (!builtin.target.os.tag.windows) {
        @compileError("IOCP poller only supports Windows platforms");
    }
}

const windows = std.os.windows;
const BOOL = windows.BOOL;
const DWORD = windows.DWORD;
const HANDLE = windows.HANDLE;
const INVALID_HANDLE_VALUE = windows.INVALID_HANDLE_VALUE;

// IOCP-specific constants and types
const OVERLAPPED = extern struct {
    Internal: *anyopaque,
    InternalHigh: *anyopaque,
    union_field: extern union {
        struct_field: extern struct {
            Offset: DWORD,
            OffsetHigh: DWORD,
        },
        Pointer: *anyopaque,
    },
    hEvent: ?HANDLE,
};

const OVERLAPPED_ENTRY = extern struct {
    lpCompletionKey: *anyopaque,
    lpOverlapped: ?*OVERLAPPED,
    Internal: *anyopaque,
    dwBytesTransferred: DWORD,
};

// Import Windows APIs
extern "kernel32" fn CreateIoCompletionPort(
    FileHandle: HANDLE,
    ExistingCompletionPort: ?HANDLE,
    CompletionKey: *anyopaque,
    NumberOfConcurrentThreads: DWORD,
) callconv(windows.WINAPI) ?HANDLE;

extern "kernel32" fn GetQueuedCompletionStatusEx(
    CompletionPort: HANDLE,
    lpCompletionPortEntries: [*]OVERLAPPED_ENTRY,
    ulCount: DWORD,
    ulNumEntriesRemoved: *DWORD,
    dwMilliseconds: DWORD,
    fAlertable: BOOL,
) callconv(windows.WINAPI) BOOL;

extern "kernel32" fn PostQueuedCompletionStatus(
    CompletionPort: HANDLE,
    dwBytesTransferred: DWORD,
    lpCompletionKey: *anyopaque,
    lpOverlapped: ?*OVERLAPPED,
) callconv(windows.WINAPI) BOOL;

extern "kernel32" fn ReadFileEx(
    hFile: HANDLE,
    lpBuffer: [*]u8,
    nNumberOfBytesToRead: DWORD,
    lpOverlapped: *OVERLAPPED,
    lpCompletionRoutine: ?*const fn(*anyopaque, DWORD, *OVERLAPPED) callconv(windows.WINAPI) void,
) callconv(windows.WINAPI) BOOL;

extern "kernel32" fn WriteFileEx(
    hFile: HANDLE,
    lpBuffer: [*]const u8,
    nNumberOfBytesToWrite: DWORD,
    lpOverlapped: *OVERLAPPED,
    lpCompletionRoutine: ?*const fn(*anyopaque, DWORD, *OVERLAPPED) callconv(windows.WINAPI) void,
) callconv(windows.WINAPI) BOOL;

// Async operation types
pub const AsyncOpType = enum {
    read_file,
    write_file,
    accept_socket,
    connect_socket,
    send_socket,
    recv_socket,
    timer,
    custom,
};

// Async operation context
pub const AsyncOperation = struct {
    const Self = @This();
    
    // Windows OVERLAPPED structure (must be first field for proper alignment)
    overlapped: OVERLAPPED,
    
    // Operation metadata
    op_type: AsyncOpType,
    handle: HANDLE,
    buffer: []u8,
    bytes_transferred: u32,
    error_code: u32,
    completion_callback: ?*const fn(*AsyncOperation) void,
    user_data: ?*anyopaque,
    
    // Goroutine synchronization
    goroutine_id: ?concurrency.GoroutineId,
    completion_channel: ?concurrency.Channel(AsyncResult),
    
    pub fn init(op_type: AsyncOpType, handle: HANDLE) Self {
        return Self{
            .overlapped = std.mem.zeroes(OVERLAPPED),
            .op_type = op_type,
            .handle = handle,
            .buffer = &[_]u8{},
            .bytes_transferred = 0,
            .error_code = 0,
            .completion_callback = null,
            .user_data = null,
            .goroutine_id = null,
            .completion_channel = null,
        };
    }
    
    pub fn setBuffer(self: *Self, buffer: []u8) void {
        self.buffer = buffer;
    }
    
    pub fn setCallback(self: *Self, callback: *const fn(*AsyncOperation) void) void {
        self.completion_callback = callback;
    }
    
    pub fn setUserData(self: *Self, data: *anyopaque) void {
        self.user_data = data;
    }
    
    pub fn bindGoroutine(self: *Self, id: concurrency.GoroutineId, channel: concurrency.Channel(AsyncResult)) void {
        self.goroutine_id = id;
        self.completion_channel = channel;
    }
};

// Async operation result
pub const AsyncResult = struct {
    success: bool,
    bytes_transferred: u32,
    error_code: u32,
    operation: *AsyncOperation,
};

// IOCP Poller errors
pub const IOCPError = error{
    CreatePortFailed,
    AssociateHandleFailed,
    GetCompletionStatusFailed,
    PostCompletionStatusFailed,
    InvalidOperation,
    OutOfMemory,
    SystemResourceLimit,
    AccessDenied,
};

// Main IOCP Poller
pub const IOCPPoller = struct {
    const Self = @This();
    const MAX_CONCURRENT_THREADS = 0; // 0 = system determines optimal number
    const MAX_COMPLETION_ENTRIES = 256;
    
    allocator: std.mem.Allocator,
    completion_port: HANDLE,
    running: std.atomic.Value(bool),
    worker_threads: []std.Thread,
    completion_entries: [MAX_COMPLETION_ENTRIES]OVERLAPPED_ENTRY,
    
    // Integration with CURSED runtime
    runtime_integration: bool,
    goroutine_scheduler: ?*concurrency.Scheduler,
    
    pub fn init(allocator: std.mem.Allocator) IOCPError!Self {
        // Create IOCP handle
        const completion_port = CreateIoCompletionPort(
            INVALID_HANDLE_VALUE,
            null,
            @ptrFromInt(0),
            MAX_CONCURRENT_THREADS,
        ) orelse return IOCPError.CreatePortFailed;
        
        // Determine number of worker threads (CPU count)
        const cpu_count = std.Thread.getCpuCount() catch 4;
        const worker_threads = allocator.alloc(std.Thread, cpu_count) catch {
            _ = windows.CloseHandle(completion_port);
            return IOCPError.OutOfMemory;
        };
        
        return Self{
            .allocator = allocator,
            .completion_port = completion_port,
            .running = std.atomic.Value(bool).init(false),
            .worker_threads = worker_threads,
            .completion_entries = undefined,
            .runtime_integration = false,
            .goroutine_scheduler = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.stop();
        
        // Wait for all worker threads to finish
        for (self.worker_threads) |thread| {
            thread.join();
        }
        
        self.allocator.free(self.worker_threads);
        _ = windows.CloseHandle(self.completion_port);
    }
    
    pub fn enableRuntimeIntegration(self: *Self, scheduler: *concurrency.Scheduler) void {
        self.runtime_integration = true;
        self.goroutine_scheduler = scheduler;
    }
    
    pub fn start(self: *Self) IOCPError!void {
        if (self.running.load(.acquire)) {
            return; // Already running
        }
        
        self.running.store(true, .release);
        
        // Start worker threads
        for (self.worker_threads, 0..) |*thread, i| {
            thread.* = std.Thread.spawn(.{}, workerThread, .{ self, i }) catch {
                self.running.store(false, .release);
                return IOCPError.SystemResourceLimit;
            };
        }
    }
    
    pub fn stop(self: *Self) void {
        if (!self.running.load(.acquire)) {
            return; // Already stopped
        }
        
        self.running.store(false, .release);
        
        // Post shutdown messages to wake up all worker threads
        for (self.worker_threads) |_| {
            _ = PostQueuedCompletionStatus(
                self.completion_port,
                0,
                @ptrFromInt(0xFFFFFFFF), // Special shutdown key
                null,
            );
        }
    }
    
    // Associate a file/socket handle with the completion port
    pub fn associateHandle(self: *Self, handle: HANDLE, completion_key: *anyopaque) IOCPError!void {
        const result = CreateIoCompletionPort(
            handle,
            self.completion_port,
            completion_key,
            0,
        );
        
        if (result == null) {
            return IOCPError.AssociateHandleFailed;
        }
    }
    
    // Async file read
    pub fn readFileAsync(self: *Self, operation: *AsyncOperation) IOCPError!void {
        if (operation.buffer.len == 0) {
            return IOCPError.InvalidOperation;
        }
        
        // Associate handle with completion port if needed
        self.associateHandle(operation.handle, @ptrCast(operation)) catch {};
        
        const success = ReadFileEx(
            operation.handle,
            operation.buffer.ptr,
            @intCast(operation.buffer.len),
            &operation.overlapped,
            null, // No completion routine, use IOCP
        );
        
        if (success == 0) {
            const error_code = windows.GetLastError();
            if (error_code != .IO_PENDING) {
                return IOCPError.InvalidOperation;
            }
        }
    }
    
    // Async file write
    pub fn writeFileAsync(self: *Self, operation: *AsyncOperation, data: []const u8) IOCPError!void {
        if (data.len == 0) {
            return IOCPError.InvalidOperation;
        }
        
        // Associate handle with completion port if needed
        self.associateHandle(operation.handle, @ptrCast(operation)) catch {};
        
        const success = WriteFileEx(
            operation.handle,
            data.ptr,
            @intCast(data.len),
            &operation.overlapped,
            null, // No completion routine, use IOCP
        );
        
        if (success == 0) {
            const error_code = windows.GetLastError();
            if (error_code != .IO_PENDING) {
                return IOCPError.InvalidOperation;
            }
        }
    }
    
    // Post custom completion event
    pub fn postCompletion(self: *Self, operation: *AsyncOperation, bytes_transferred: u32) IOCPError!void {
        const success = PostQueuedCompletionStatus(
            self.completion_port,
            bytes_transferred,
            @ptrCast(operation),
            &operation.overlapped,
        );
        
        if (success == 0) {
            return IOCPError.PostCompletionStatusFailed;
        }
    }
    
    // Worker thread function
    fn workerThread(self: *Self, thread_id: usize) void {
        std.log.info("IOCP worker thread {} started", .{thread_id});
        defer std.log.info("IOCP worker thread {} stopped", .{thread_id});
        
        while (self.running.load(.acquire)) {
            var num_entries: DWORD = 0;
            
            // Wait for completion events (500ms timeout)
            const success = GetQueuedCompletionStatusEx(
                self.completion_port,
                &self.completion_entries,
                MAX_COMPLETION_ENTRIES,
                &num_entries,
                500, // 500ms timeout
                windows.FALSE,
            );
            
            if (success == 0) {
                const error_code = windows.GetLastError();
                if (error_code == .WAIT_TIMEOUT) {
                    continue; // Timeout is expected
                }
                std.log.err("GetQueuedCompletionStatusEx failed: {}", .{error_code});
                continue;
            }
            
            // Process completion events
            for (0..num_entries) |i| {
                const entry = &self.completion_entries[i];
                self.processCompletion(entry);
            }
        }
    }
    
    fn processCompletion(self: *Self, entry: *const OVERLAPPED_ENTRY) void {
        // Check for shutdown signal
        const completion_key = @intFromPtr(entry.lpCompletionKey);
        if (completion_key == 0xFFFFFFFF) {
            return; // Shutdown signal
        }
        
        // Cast completion key back to operation
        const operation: *AsyncOperation = @ptrCast(@alignCast(entry.lpCompletionKey));
        
        // Update operation results
        operation.bytes_transferred = entry.dwBytesTransferred;
        operation.error_code = @intFromPtr(entry.Internal);
        
        // Create result
        const result = AsyncResult{
            .success = operation.error_code == 0,
            .bytes_transferred = operation.bytes_transferred,
            .error_code = operation.error_code,
            .operation = operation,
        };
        
        // Handle completion based on operation type
        self.handleCompletion(operation, result);
    }
    
    fn handleCompletion(self: *Self, operation: *AsyncOperation, result: AsyncResult) void {
        // Execute callback if provided
        if (operation.completion_callback) |callback| {
            callback(operation);
        }
        
        // Integrate with CURSED runtime if enabled
        if (self.runtime_integration) {
            if (operation.goroutine_id) |goroutine_id| {
                // Wake up waiting goroutine
                if (operation.completion_channel) |channel| {
                    channel.send(result) catch |err| {
                        std.log.err("Failed to send async result to channel: {}", .{err});
                    };
                }
                
                // Schedule goroutine for execution
                if (self.goroutine_scheduler) |scheduler| {
                    scheduler.scheduleGoroutine(goroutine_id) catch |err| {
                        std.log.err("Failed to schedule goroutine after async completion: {}", .{err});
                    };
                }
            }
        }
        
        // Log completion for debugging
        std.log.debug("Async operation completed: type={}, success={}, bytes={}", .{
            operation.op_type,
            result.success,
            result.bytes_transferred,
        });
    }
};

// High-level async file operations
pub const AsyncFileOps = struct {
    const Self = @This();
    
    poller: *IOCPPoller,
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator, poller: *IOCPPoller) Self {
        return Self{
            .poller = poller,
            .allocator = allocator,
        };
    }
    
    // Async read file to buffer
    pub fn readFile(self: *Self, file_handle: HANDLE, buffer: []u8) IOCPError!*AsyncOperation {
        const operation = self.allocator.create(AsyncOperation) catch {
            return IOCPError.OutOfMemory;
        };
        
        operation.* = AsyncOperation.init(.read_file, file_handle);
        operation.setBuffer(buffer);
        
        try self.poller.readFileAsync(operation);
        return operation;
    }
    
    // Async write buffer to file
    pub fn writeFile(self: *Self, file_handle: HANDLE, data: []const u8) IOCPError!*AsyncOperation {
        const operation = self.allocator.create(AsyncOperation) catch {
            return IOCPError.OutOfMemory;
        };
        
        operation.* = AsyncOperation.init(.write_file, file_handle);
        
        try self.poller.writeFileAsync(operation, data);
        return operation;
    }
    
    // Wait for operation completion (blocking)
    pub fn waitForCompletion(self: *Self, operation: *AsyncOperation, timeout_ms: u32) bool {
        _ = self;
        
        if (operation.overlapped.hEvent) |event_handle| {
            const result = windows.WaitForSingleObject(event_handle, timeout_ms);
            return result == windows.WAIT_OBJECT_0;
        }
        
        // Fallback: busy wait with sleep (not ideal but works)
        const start_time = std.time.milliTimestamp();
        while (std.time.milliTimestamp() - start_time < timeout_ms) {
            if (operation.bytes_transferred > 0 or operation.error_code != 0) {
                return true;
            }
            std.time.sleep(1 * std.time.ns_per_ms); // 1ms sleep
        }
        
        return false;
    }
    
    pub fn freeOperation(self: *Self, operation: *AsyncOperation) void {
        self.allocator.destroy(operation);
    }
};

// Integration with CURSED goroutine system
pub const AsyncRuntime = struct {
    const Self = @This();
    
    poller: IOCPPoller,
    file_ops: AsyncFileOps,
    scheduler: ?*concurrency.Scheduler,
    
    pub fn init(allocator: std.mem.Allocator) IOCPError!Self {
        var poller = try IOCPPoller.init(allocator);
        const file_ops = AsyncFileOps.init(allocator, &poller);
        
        return Self{
            .poller = poller,
            .file_ops = file_ops,
            .scheduler = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.poller.deinit();
    }
    
    pub fn start(self: *Self) IOCPError!void {
        try self.poller.start();
    }
    
    pub fn stop(self: *Self) void {
        self.poller.stop();
    }
    
    pub fn integrateWithScheduler(self: *Self, scheduler: *concurrency.Scheduler) void {
        self.scheduler = scheduler;
        self.poller.enableRuntimeIntegration(scheduler);
    }
    
    // Goroutine-friendly async read
    pub fn asyncRead(self: *Self, file_handle: HANDLE, buffer: []u8) IOCPError!AsyncResult {
        const operation = try self.file_ops.readFile(file_handle, buffer);
        defer self.file_ops.freeOperation(operation);
        
        // If running in goroutine context, set up channel-based waiting
        if (self.scheduler) |scheduler| {
            if (scheduler.getCurrentGoroutine()) |current_goroutine| {
                const result_channel = concurrency.Channel(AsyncResult).init(self.poller.allocator, 1) catch {
                    return IOCPError.OutOfMemory;
                };
                defer result_channel.deinit();
                
                operation.bindGoroutine(current_goroutine.id, result_channel);
                
                // Yield current goroutine and wait for completion
                const result = result_channel.receive() catch {
                    return AsyncResult{
                        .success = false,
                        .bytes_transferred = 0,
                        .error_code = @intFromEnum(windows.Win32Error.OPERATION_ABORTED),
                        .operation = operation,
                    };
                };
                
                return result;
            }
        }
        
        // Fallback: blocking wait
        if (self.file_ops.waitForCompletion(operation, 30000)) { // 30 second timeout
            return AsyncResult{
                .success = operation.error_code == 0,
                .bytes_transferred = operation.bytes_transferred,
                .error_code = operation.error_code,
                .operation = operation,
            };
        } else {
            return AsyncResult{
                .success = false,
                .bytes_transferred = 0,
                .error_code = @intFromEnum(windows.Win32Error.WAIT_TIMEOUT),
                .operation = operation,
            };
        }
    }
    
    // Goroutine-friendly async write
    pub fn asyncWrite(self: *Self, file_handle: HANDLE, data: []const u8) IOCPError!AsyncResult {
        const operation = try self.file_ops.writeFile(file_handle, data);
        defer self.file_ops.freeOperation(operation);
        
        // If running in goroutine context, set up channel-based waiting
        if (self.scheduler) |scheduler| {
            if (scheduler.getCurrentGoroutine()) |current_goroutine| {
                const result_channel = concurrency.Channel(AsyncResult).init(self.poller.allocator, 1) catch {
                    return IOCPError.OutOfMemory;
                };
                defer result_channel.deinit();
                
                operation.bindGoroutine(current_goroutine.id, result_channel);
                
                // Yield current goroutine and wait for completion
                const result = result_channel.receive() catch {
                    return AsyncResult{
                        .success = false,
                        .bytes_transferred = 0,
                        .error_code = @intFromEnum(windows.Win32Error.OPERATION_ABORTED),
                        .operation = operation,
                    };
                };
                
                return result;
            }
        }
        
        // Fallback: blocking wait
        if (self.file_ops.waitForCompletion(operation, 30000)) { // 30 second timeout
            return AsyncResult{
                .success = operation.error_code == 0,
                .bytes_transferred = operation.bytes_transferred,
                .error_code = operation.error_code,
                .operation = operation,
            };
        } else {
            return AsyncResult{
                .success = false,
                .bytes_transferred = 0,
                .error_code = @intFromEnum(windows.Win32Error.WAIT_TIMEOUT),
                .operation = operation,
            };
        }
    }
};

// Tests and examples
test "IOCP poller initialization" {
    if (!builtin.target.os.tag.windows) return; // Skip on non-Windows
    
    const allocator = std.testing.allocator;
    var poller = try IOCPPoller.init(allocator);
    defer poller.deinit();
    
    try std.testing.expect(!poller.running.load(.acquire));
}

test "async runtime integration" {
    if (!builtin.target.os.tag.windows) return; // Skip on non-Windows
    
    const allocator = std.testing.allocator;
    var runtime = try AsyncRuntime.init(allocator);
    defer runtime.deinit();
    
    try runtime.start();
    defer runtime.stop();
    
    // Test would require actual file handles and scheduler integration
    // This is a basic structure test
}

// Export for use by other modules
pub const WindowsAsyncIO = struct {
    pub const Poller = IOCPPoller;
    pub const FileOps = AsyncFileOps;
    pub const Runtime = AsyncRuntime;
    pub const Operation = AsyncOperation;
    pub const Result = AsyncResult;
    pub const Error = IOCPError;
};
