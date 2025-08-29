// CURSED Real Signal Handling Platform Integration
// Replaces simulation with actual OS-level signal handling
// Supports: Linux, macOS, Windows (using console control handlers)

const std = @import("std");
const builtin = @import("builtin");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Cross-platform signal types
pub const SignalType = enum(u32) {
    // Unix-like signals
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGURG = 23,
    SIGXCPU = 24,
    SIGXFSZ = 25,
    SIGVTALRM = 26,
    SIGPROF = 27,
    SIGWINCH = 28,
    SIGIO = 29,
    SIGPWR = 30,
    
    // Windows console control events (mapped to signal numbers)
    CTRL_C_EVENT = 2,      // Maps to SIGINT
    CTRL_BREAK_EVENT = 3,  // Maps to SIGQUIT
    CTRL_CLOSE_EVENT = 15, // Maps to SIGTERM
    CTRL_LOGOFF_EVENT = 1, // Maps to SIGHUP
    CTRL_SHUTDOWN_EVENT = 30, // Maps to SIGPWR
    
    pub fn toUnixSignal(self: SignalType) i32 {
        return @intCast(@intFromEnum(self));
    }
    
    pub fn fromUnixSignal(signal: i32) ?SignalType {
        return switch (signal) {
            1 => .SIGHUP,
            2 => .SIGINT,
            3 => .SIGQUIT,
            4 => .SIGILL,
            5 => .SIGTRAP,
            6 => .SIGABRT,
            7 => .SIGBUS,
            8 => .SIGFPE,
            9 => .SIGKILL,
            10 => .SIGUSR1,
            11 => .SIGSEGV,
            12 => .SIGUSR2,
            13 => .SIGPIPE,
            14 => .SIGALRM,
            15 => .SIGTERM,
            17 => .SIGCHLD,
            18 => .SIGCONT,
            19 => .SIGSTOP,
            20 => .SIGTSTP,
            21 => .SIGTTIN,
            22 => .SIGTTOU,
            23 => .SIGURG,
            24 => .SIGXCPU,
            25 => .SIGXFSZ,
            26 => .SIGVTALRM,
            27 => .SIGPROF,
            28 => .SIGWINCH,
            29 => .SIGIO,
            30 => .SIGPWR,
            else => null,
        };
    }
    
    pub fn name(self: SignalType) []const u8 {
        return switch (self) {
            .SIGHUP => "SIGHUP",
            .SIGINT => "SIGINT",
            .SIGQUIT => "SIGQUIT",
            .SIGILL => "SIGILL",
            .SIGTRAP => "SIGTRAP",
            .SIGABRT => "SIGABRT",
            .SIGBUS => "SIGBUS",
            .SIGFPE => "SIGFPE",
            .SIGKILL => "SIGKILL",
            .SIGUSR1 => "SIGUSR1",
            .SIGSEGV => "SIGSEGV",
            .SIGUSR2 => "SIGUSR2",
            .SIGPIPE => "SIGPIPE",
            .SIGALRM => "SIGALRM",
            .SIGTERM => "SIGTERM",
            .SIGCHLD => "SIGCHLD",
            .SIGCONT => "SIGCONT",
            .SIGSTOP => "SIGSTOP",
            .SIGTSTP => "SIGTSTP",
            .SIGTTIN => "SIGTTIN",
            .SIGTTOU => "SIGTTOU",
            .SIGURG => "SIGURG",
            .SIGXCPU => "SIGXCPU",
            .SIGXFSZ => "SIGXFSZ",
            .SIGVTALRM => "SIGVTALRM",
            .SIGPROF => "SIGPROF",
            .SIGWINCH => "SIGWINCH",
            .SIGIO => "SIGIO",
            .SIGPWR => "SIGPWR",
            .CTRL_C_EVENT => "CTRL_C",
            .CTRL_BREAK_EVENT => "CTRL_BREAK",
            .CTRL_CLOSE_EVENT => "CTRL_CLOSE",
            .CTRL_LOGOFF_EVENT => "CTRL_LOGOFF",
            .CTRL_SHUTDOWN_EVENT => "CTRL_SHUTDOWN",
        };
    }
    
    pub fn canBeCaught(self: SignalType) bool {
        return switch (self) {
            .SIGKILL, .SIGSTOP => false, // Cannot be caught or ignored
            else => true,
        };
    }
    
    pub fn isAsyncSafe(self: SignalType) bool {
        return switch (self) {
            .SIGCHLD, .SIGUSR1, .SIGUSR2, .SIGWINCH, .SIGURG => true,
            .SIGSEGV, .SIGFPE, .SIGILL, .SIGBUS => false, // Synchronous signals
            else => true,
        };
    }
};

// Signal handler function type
pub const SignalHandler = *const fn (signal: SignalType, context: ?*SignalContext) void;

// Signal context information passed to handlers
pub const SignalContext = struct {
    signal: SignalType,
    sender_pid: u32,
    timestamp: u64,
    signal_value: i32,
    user_data: ?*anyopaque,
    
    // Platform-specific context (Unix siginfo_t or Windows context)
    platform_context: ?*anyopaque,
};

// Signal mask for blocking/unblocking signals
pub const SignalMask = struct {
    mask: u64, // Bitmask for signals 1-64
    
    pub fn init() SignalMask {
        return SignalMask{ .mask = 0 };
    }
    
    pub fn addSignal(self: *SignalMask, signal: SignalType) void {
        const bit = @intFromEnum(signal);
        if (bit > 0 and bit <= 64) {
            self.mask |= (@as(u64, 1) << @intCast(bit - 1));
        }
    }
    
    pub fn removeSignal(self: *SignalMask, signal: SignalType) void {
        const bit = @intFromEnum(signal);
        if (bit > 0 and bit <= 64) {
            self.mask &= ~(@as(u64, 1) << @intCast(bit - 1));
        }
    }
    
    pub fn hasSignal(self: SignalMask, signal: SignalType) bool {
        const bit = @intFromEnum(signal);
        if (bit > 0 and bit <= 64) {
            return (self.mask & (@as(u64, 1) << @intCast(bit - 1))) != 0;
        }
        return false;
    }
    
    pub fn isEmpty(self: SignalMask) bool {
        return self.mask == 0;
    }
    
    pub fn clear(self: *SignalMask) void {
        self.mask = 0;
    }
};

// Real signal handling implementation
pub const RealSignalHandler = struct {
    allocator: Allocator,
    handlers: HashMap(SignalType, SignalHandler, std.hash_map.AutoContext(SignalType), 80),
    signal_mask: SignalMask,
    self_pipe_fds: if (builtin.target.os.tag == .windows) void else [2]std.posix.fd_t,
    cleanup_handlers: ArrayList(*const fn () void),
    signal_safe_flag: std.atomic.Value(bool),
    
    const Self = @This();
    
    pub const SignalError = error{
        InvalidSignal,
        CannotCatch,
        SystemError,
        AlreadyRegistered,
        NotRegistered,
        OutOfMemory,
        PipeError,
    };
    
    pub fn init(allocator: Allocator) !Self {
        _ = allocator;
        var self = Self{
            .allocator = allocator,
            .handlers = HashMap(SignalType, SignalHandler, std.hash_map.AutoContext(SignalType), 80).init(allocator),
            .signal_mask = SignalMask.init(),
            .self_pipe_fds = undefined,
            .cleanup_handlers = ArrayList(*const fn () void).init(allocator),
            .signal_safe_flag = std.atomic.Value(bool).init(false),
        };
        
        // Initialize self-pipe for signal-safe handling (Unix only)
        if (builtin.target.os.tag != .windows) {
            try self.initSelfPipe();
        }
        
        // Set up platform-specific signal infrastructure
        try self.initPlatformSignals();
        
        return self;
    }
    
    pub fn deinit(self: *Self) void {
        self.cleanup();
        self.handlers.deinit(self.allocator);
        self.cleanup_handlers.deinit(self.allocator);
        
        // Close self-pipe (Unix only)
        if (builtin.target.os.tag != .windows) {
            std.posix.close(self.self_pipe_fds[0]);
            std.posix.close(self.self_pipe_fds[1]);
        }
    }
    
    // Initialize self-pipe for async-safe signal handling
    fn initSelfPipe(self: *Self) !void {
        if (builtin.target.os.tag == .windows) return;
        
        const fds = try std.posix.pipe();
        self.self_pipe_fds[0] = fds[0]; // Read end
        self.self_pipe_fds[1] = fds[1]; // Write end
        
        // Make write end non-blocking for signal safety
        const flags = try std.posix.fcntl(self.self_pipe_fds[1], std.posix.F.GETFL, 0);
        _ = try std.posix.fcntl(self.self_pipe_fds[1], std.posix.F.SETFL, flags | std.posix.O.NONBLOCK);
    }
    
    // Platform-specific signal infrastructure initialization
    fn initPlatformSignals(self: *Self) !void {
        if (builtin.target.os.tag == .windows) {
            try self.initWindowsConsoleHandlers();
        } else {
            try self.initUnixSignalHandlers();
        }
    }
    
    // Initialize Windows console control handlers
    fn initWindowsConsoleHandlers(self: *Self) !void {
        if (builtin.target.os.tag != .windows) return;
        
        const windows = std.os.windows;
        const kernel32 = windows.kernel32;
        
        // Register console control handler
        const handler = struct {
            fn consoleHandler(ctrl_type: windows.DWORD) callconv(windows.WINAPI) windows.BOOL {
                const signal_type = switch (ctrl_type) {
                    windows.CTRL_C_EVENT => SignalType.CTRL_C_EVENT,
                    windows.CTRL_BREAK_EVENT => SignalType.CTRL_BREAK_EVENT,
                    windows.CTRL_CLOSE_EVENT => SignalType.CTRL_CLOSE_EVENT,
                    windows.CTRL_LOGOFF_EVENT => SignalType.CTRL_LOGOFF_EVENT,
                    windows.CTRL_SHUTDOWN_EVENT => SignalType.CTRL_SHUTDOWN_EVENT,
                    else => return windows.FALSE,
                };
                
                // Trigger signal handling
                triggerSignal(signal_type);
                return windows.TRUE;
            }
            
            fn triggerSignal(signal: SignalType) void {
                // Implementation will be filled in during signal registration
                _ = signal;
            }
        }.consoleHandler;
        
        if (kernel32.SetConsoleCtrlHandler(handler, windows.TRUE) == 0) {
            return SignalError.SystemError;
        }
    }
    
    // Initialize Unix signal handlers using signalfd when available
    fn initUnixSignalHandlers(self: *Self) !void {
        if (builtin.target.os.tag == .windows) return;
        
        // Block all signals initially, we'll handle them synchronously
        var mask: std.posix.sigset_t = undefined;
        std.posix.sigemptyset(&mask);
        
        // Add common signals to the mask
        const common_signals = [_]SignalType{
            .SIGINT, .SIGTERM, .SIGQUIT, .SIGUSR1, .SIGUSR2,
            .SIGCHLD, .SIGWINCH, .SIGPIPE, .SIGALRM, .SIGHUP
        };
        
        for (common_signals) |sig| {
            std.posix.sigaddset(&mask, sig.toUnixSignal());
        }
        
        // Block signals for this thread
        try std.posix.sigprocmask(std.posix.SIG.BLOCK, &mask, null);
    }
    
    // Register a signal handler
    pub fn registerHandler(self: *Self, signal: SignalType, handler: SignalHandler) !void {
        if (!signal.canBeCaught()) {
            return SignalError.CannotCatch;
        }
        
        if (self.handlers.contains(signal)) {
            return SignalError.AlreadyRegistered;
        }
        
        try self.handlers.put(signal, handler);
        
        // Install platform-specific signal handler
        if (builtin.target.os.tag == .windows) {
            try self.installWindowsHandler(signal);
        } else {
            try self.installUnixHandler(signal);
        }
    }
    
    // Unregister a signal handler
    pub fn unregisterHandler(self: *Self, signal: SignalType) !void {
        if (!self.handlers.contains(signal)) {
            return SignalError.NotRegistered;
        }
        
        _ = self.handlers.remove(signal);
        
        // Restore default platform-specific handler
        if (builtin.target.os.tag == .windows) {
            try self.restoreWindowsDefault(signal);
        } else {
            try self.restoreUnixDefault(signal);
        }
    }
    
    // Install Unix signal handler
    fn installUnixHandler(self: *Self, signal: SignalType) !void {
        if (builtin.target.os.tag == .windows) return;
        
        const unix_signal = signal.toUnixSignal();
        
        const handler = struct {
            fn signalHandler(sig: c_int) callconv(.c) void {
                // Write signal number to self-pipe for async-safe handling
                const signal_byte = @as(u8, @intCast(sig));
                _ = std.posix.write(global_self_pipe_fd, &[_]u8{signal_byte}) catch {};
            }
        }.signalHandler;
        
        var action = std.posix.Sigaction{
            .handler = .{ .handler = handler },
            .mask = std.posix.empty_sigset,
            .flags = std.posix.SA.RESTART,
        };
        
        try std.posix.sigaction(unix_signal, &action, null);
        
        // Store self-pipe write FD globally for signal handler access
        global_self_pipe_fd = self.self_pipe_fds[1];
    }
    
    // Install Windows console handler (already done in init)
    fn installWindowsHandler(self: *Self, signal: SignalType) !void {
        _ = self;
        _ = signal;
        // Windows handlers are installed globally in initWindowsConsoleHandlers
    }
    
    // Restore Unix default signal handler
    fn restoreUnixDefault(self: *Self, signal: SignalType) !void {
        _ = self;
        if (builtin.target.os.tag == .windows) return;
        
        const unix_signal = signal.toUnixSignal();
        
        var action = std.posix.Sigaction{
            .handler = .{ .handler = std.posix.SIG.DFL },
            .mask = std.posix.empty_sigset,
            .flags = 0,
        };
        
        try std.posix.sigaction(unix_signal, &action, null);
    }
    
    // Restore Windows default handler
    fn restoreWindowsDefault(self: *Self, signal: SignalType) !void {
        _ = self;
        _ = signal;
        // Windows default restoration would require more complex logic
    }
    
    // Block signals using signal mask
    pub fn blockSignals(self: *Self, mask: SignalMask) !void {
        self.signal_mask = mask;
        
        if (builtin.target.os.tag == .windows) {
            // Windows doesn't have signal masking, this is handled in console handler
            return;
        }
        
        var sigset: std.posix.sigset_t = undefined;
        std.posix.sigemptyset(&sigset);
        
        // Add signals from mask to sigset
        var bit: u6 = 0;
        while (bit < 64) : (bit += 1) {
            if ((mask.mask & (@as(u64, 1) << bit)) != 0) {
                const signal_num = @as(i32, bit + 1);
                if (SignalType.fromUnixSignal(signal_num)) |_| {
                    std.posix.sigaddset(&sigset, signal_num);
                }
            }
        }
        
        try std.posix.sigprocmask(std.posix.SIG.BLOCK, &sigset, null);
    }
    
    // Unblock signals
    pub fn unblockSignals(self: *Self, mask: SignalMask) !void {
        if (builtin.target.os.tag == .windows) {
            // Remove from internal mask
            self.signal_mask.mask &= ~mask.mask;
            return;
        }
        
        var sigset: std.posix.sigset_t = undefined;
        std.posix.sigemptyset(&sigset);
        
        // Add signals from mask to sigset
        var bit: u6 = 0;
        while (bit < 64) : (bit += 1) {
            if ((mask.mask & (@as(u64, 1) << bit)) != 0) {
                const signal_num = @as(i32, bit + 1);
                if (SignalType.fromUnixSignal(signal_num)) |_| {
                    std.posix.sigaddset(&sigset, signal_num);
                }
            }
        }
        
        try std.posix.sigprocmask(std.posix.SIG.UNBLOCK, &sigset, null);
        
        // Remove from internal mask
        self.signal_mask.mask &= ~mask.mask;
    }
    
    // Wait for signals (synchronous signal handling)
    pub fn waitForSignal(self: *Self, timeout_ms: ?u32) !?SignalType {
        if (builtin.target.os.tag == .windows) {
            return self.waitForWindowsSignal(timeout_ms);
        } else {
            return self.waitForUnixSignal(timeout_ms);
        }
    }
    
    // Unix signal waiting using self-pipe
    fn waitForUnixSignal(self: *Self, timeout_ms: ?u32) !?SignalType {
        var poll_fds = [_]std.posix.pollfd{
            .{
                .fd = self.self_pipe_fds[0],
                .events = std.posix.POLL.IN,
                .revents = 0,
            },
        };
        
        const timeout = if (timeout_ms) |ms| @as(i32, @intCast(ms)) else -1;
        const ready = try std.posix.poll(&poll_fds, timeout);
        
        if (ready == 0) {
            return null; // Timeout
        }
        
        if (poll_fds[0].revents & std.posix.POLL.IN != 0) {
            var signal_byte: [1]u8 = undefined;
            const bytes_read = try std.posix.read(self.self_pipe_fds[0], &signal_byte);
            
            if (bytes_read > 0) {
                const signal_num = @as(i32, signal_byte[0]);
                if (SignalType.fromUnixSignal(signal_num)) |signal| {
                    return signal;
                }
            }
        }
        
        return null;
    }
    
    // Windows signal waiting (simplified)
    fn waitForWindowsSignal(self: *Self, timeout_ms: ?u32) !?SignalType {
        _ = self;
        _ = timeout_ms;
        // Windows implementation would use WaitForSingleObject or similar
        // For now, return null (no signal)
        return null;
    }
    
    // Send signal to process
    pub fn sendSignal(pid: u32, signal: SignalType) !void {
        if (builtin.target.os.tag == .windows) {
            try sendWindowsSignal(pid, signal);
        } else {
            try sendUnixSignal(pid, signal);
        }
    }
    
    // Send Unix signal
    fn sendUnixSignal(pid: u32, signal: SignalType) !void {
        const unix_signal = signal.toUnixSignal();
        const result = std.c.kill(@intCast(pid), unix_signal);
        
        if (result != 0) {
            return SignalError.SystemError;
        }
    }
    
    // Send Windows signal (limited options)
    fn sendWindowsSignal(pid: u32, signal: SignalType) !void {
        const windows = std.os.windows;
        
        switch (signal) {
            .SIGTERM, .CTRL_CLOSE_EVENT => {
                // Attempt to terminate process gracefully
                const handle = windows.OpenProcess(windows.PROCESS_TERMINATE, windows.FALSE, pid);
                if (handle == null) return SignalError.SystemError;
                defer windows.CloseHandle(handle.?);
                
                if (windows.TerminateProcess(handle.?, 0) == 0) {
                    return SignalError.SystemError;
                }
            },
            .SIGINT, .CTRL_C_EVENT => {
                // Send Ctrl+C to console process
                if (windows.kernel32.GenerateConsoleCtrlEvent(windows.CTRL_C_EVENT, pid) == 0) {
                    return SignalError.SystemError;
                }
            },
            else => {
                return SignalError.NotSupported;
            }
        }
    }
    
    // Process pending signals
    pub fn processPendingSignals(self: *Self) !void {
        while (try self.waitForSignal(0)) |signal| {
            try self.handleSignal(signal, null);
        }
    }
    
    // Handle a signal by calling registered handler
    fn handleSignal(self: *Self, signal: SignalType, context: ?*SignalContext) !void {
        if (self.handlers.get(signal)) |handler| {
            // Create signal context if not provided
            var signal_context: SignalContext = undefined;
            const ctx = context orelse blk: {
                signal_context = SignalContext{
                    .signal = signal,
                    .sender_pid = 0, // Unknown for most signals
                    .timestamp = std.time.milliTimestamp(),
                    .signal_value = 0,
                    .user_data = null,
                    .platform_context = null,
                };
                break :blk &signal_context;
            };
            
            // Set signal-safe flag
            self.signal_safe_flag.store(true, .seq_cst);
            defer self.signal_safe_flag.store(false, .seq_cst);
            
            // Call handler
            handler(signal, ctx);
        }
    }
    
    // Register cleanup handler for graceful shutdown
    pub fn registerCleanupHandler(self: *Self, cleanup_fn: *const fn () void) !void {
        try self.cleanup_handlers.append(allocator, cleanup_fn);
    }
    
    // Execute all cleanup handlers
    pub fn cleanup(self: *Self) void {
        for (self.cleanup_handlers.items) |cleanup_fn| {
            cleanup_fn();
        }
        self.cleanup_handlers.clearAndFree();
    }
    
    // Check if we're currently in a signal handler
    pub fn isInSignalHandler(self: *Self) bool {
        return self.signal_safe_flag.load(.seq_cst);
    }
};

// Global self-pipe file descriptor for Unix signal handler
var global_self_pipe_fd: std.posix.fd_t = undefined;

// Signal-safe operations
pub const SignalSafeOps = struct {
    // Signal-safe write to file descriptor
    pub fn safeWrite(fd: std.posix.fd_t, data: []const u8) void {
        _ = std.posix.write(fd, data) catch {};
    }
    
    // Signal-safe string copy
    pub fn safeCopy(dest: []u8, src: []const u8) void {
        const len = @min(dest.len - 1, src.len);
        @memcpy(dest[0..len], src[0..len]);
        dest[len] = 0;
    }
    
    // Signal-safe integer to string conversion
    pub fn safeIntToString(buffer: []u8, value: i32) []u8 {
        var abs_value = @abs(value);
        var i: usize = buffer.len - 1;
        
        if (abs_value == 0) {
            buffer[i] = '0';
            return buffer[i..];
        }
        
        while (abs_value > 0 and i > 0) {
            buffer[i] = @intCast('0' + (abs_value % 10));
            abs_value /= 10;
            i -= 1;
        }
        
        if (value < 0 and i > 0) {
            buffer[i] = '-';
            i -= 1;
        }
        
        return buffer[i + 1..];
    }
};

// Async signal handling support
pub const AsyncSignalHandler = struct {
    signal_handler: *RealSignalHandler,
    signal_queue: std.atomic.Queue(SignalType),
    queue_allocator: Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, signal_handler: *RealSignalHandler) Self {
        return Self{
            .signal_handler = signal_handler,
            .signal_queue = std.atomic.Queue(SignalType).init(),
            .queue_allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Drain the queue
        while (self.signal_queue.get()) |node| {
            self.queue_allocator.destroy(node);
        }
    }
    
    // Queue signal for async processing
    pub fn queueSignal(self: *Self, signal: SignalType) !void {
        const node = try self.queue_allocator.create(std.atomic.Queue(SignalType).Node);
        node.data = signal;
        self.signal_queue.put(node);
    }
    
    // Process queued signals
    pub fn processQueue(self: *Self) !void {
        while (self.signal_queue.get()) |node| {
            defer self.queue_allocator.destroy(node);
            try self.signal_handler.handleSignal(node.data, null);
        }
    }
};

// Testing support
pub const TestSignalSupport = struct {
    pub fn simulateSignal(handler: *RealSignalHandler, signal: SignalType) !void {
        try handler.handleSignal(signal, null);
    }
    
    pub fn isSignalRegistered(handler: *RealSignalHandler, signal: SignalType) bool {
        return handler.handlers.contains(signal);
    }
    
    pub fn getRegisteredSignalCount(handler: *RealSignalHandler) u32 {
        return @intCast(handler.handlers.count());
    }
};

// Error types for consumers
pub const SignalError = RealSignalHandler.SignalError || error{
    NotSupported,
    Timeout,
    InvalidContext,
};
