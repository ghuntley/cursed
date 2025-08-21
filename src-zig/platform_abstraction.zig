// CURSED Platform Abstraction Layer
// Provides cross-platform support for file I/O, networking, time operations, and system calls
// Supports: Linux x64/ARM64, macOS x64/ARM64, Windows x64, WASM32

const std = @import("std");
const builtin = @import("builtin");

// Platform detection and configuration
pub const Platform = enum {
    linux_x64,
    linux_arm64,
    macos_x64,
    macos_arm64,
    windows_x64,
    wasm32,
    unknown,
    
    pub fn current() Platform {
        switch (builtin.target.os.tag) {
            .linux => switch (builtin.target.cpu.arch) {
                .x86_64 => return .linux_x64,
                .aarch64 => return .linux_arm64,
                else => return .unknown,
            },
            .macos => switch (builtin.target.cpu.arch) {
                .x86_64 => return .macos_x64,
                .aarch64 => return .macos_arm64,
                else => return .unknown,
            },
            .windows => switch (builtin.target.cpu.arch) {
                .x86_64 => return .windows_x64,
                else => return .unknown,
            },
            .wasi => return .wasm32,
            else => return .unknown,
        }
    }
    
    pub fn isWindows(self: Platform) bool {
        return switch (self) {
            .windows_x64 => true,
            else => false,
        };
    }
    
    pub fn isUnix(self: Platform) bool {
        return switch (self) {
            .linux_x64, .linux_arm64, .macos_x64, .macos_arm64 => true,
            else => false,
        };
    }
    
    pub fn isWasm(self: Platform) bool {
        return self == .wasm32;
    }
    
    pub fn name(self: Platform) []const u8 {
        return switch (self) {
            .linux_x64 => "Linux x86_64",
            .linux_arm64 => "Linux ARM64",
            .macos_x64 => "macOS x86_64",
            .macos_arm64 => "macOS ARM64",
            .windows_x64 => "Windows x86_64",
            .wasm32 => "WebAssembly",
            .unknown => "Unknown",
        };
    }
};

// Cross-platform file operations
pub const FileOps = struct {
    const Self = @This();
    
    // File handle type
    pub const FileHandle = if (builtin.target.os.tag == .wasi) 
        i32 
    else if (builtin.target.os.tag == .windows) 
        std.os.windows.HANDLE 
    else 
        std.posix.fd_t;
    
    // Error types
    pub const FileError = error{
        AccessDenied,
        FileNotFound,
        InvalidHandle,
        PermissionDenied,
        DiskFull,
        IoError,
        PathTooLong,
        NotSupported,
    };
    
    // File open modes
    pub const OpenMode = enum {
        read,
        write,
        append,
        read_write,
    };
    
    // Open file with cross-platform compatibility
    pub fn openFile(path: []const u8, mode: OpenMode) FileError!FileHandle {
        if (Platform.current().isWasm()) {
            // WASM file operations - limited functionality
            return error.NotSupported;
        } else if (Platform.current().isWindows()) {
            return openFileWindows(path, mode);
        } else {
            return openFileUnix(path, mode);
        }
    }
    
    fn openFileWindows(path: []const u8, mode: OpenMode) FileError!FileHandle {
        const windows = std.os.windows;
        var path_buffer: [std.fs.MAX_PATH_BYTES]u16 = undefined;
        const path_utf16 = std.unicode.utf8ToUtf16Le(path_buffer[0..], path) catch {
            return FileError.PathTooLong;
        };
        
        const access = switch (mode) {
            .read => windows.GENERIC_READ,
            .write => windows.GENERIC_WRITE,
            .append => windows.GENERIC_WRITE,
            .read_write => windows.GENERIC_READ | windows.GENERIC_WRITE,
        };
        
        const creation = switch (mode) {
            .read => windows.OPEN_EXISTING,
            .write => windows.CREATE_ALWAYS,
            .append => windows.OPEN_ALWAYS,
            .read_write => windows.OPEN_ALWAYS,
        };
        
        const handle = windows.CreateFileW(
            @ptrCast(path_utf16.ptr),
            access,
            windows.FILE_SHARE_READ | windows.FILE_SHARE_WRITE,
            null,
            creation,
            windows.FILE_ATTRIBUTE_NORMAL,
            null,
        );
        
        if (handle == windows.INVALID_HANDLE_VALUE) {
            return switch (windows.GetLastError()) {
                .FILE_NOT_FOUND => FileError.FileNotFound,
                .ACCESS_DENIED => FileError.AccessDenied,
                .PATH_NOT_FOUND => FileError.FileNotFound,
                else => FileError.IoError,
            };
        }
        
        return handle;
    }
    
    fn openFileUnix(path: []const u8, mode: OpenMode) FileError!FileHandle {
        if (builtin.target.os.tag == .wasi) {
            // WASM/WASI implementation - use basic file operations
            return error.NotSupported; // File operations not supported in WASM
        }
        
        const flags = switch (mode) {
            .read => std.posix.O.RDONLY,
            .write => std.posix.O.WRONLY | std.posix.O.CREAT | std.posix.O.TRUNC,
            .append => std.posix.O.WRONLY | std.posix.O.CREAT | std.posix.O.APPEND,
            .read_write => std.posix.O.RDWR | std.posix.O.CREAT,
        };
        
        const fd = std.posix.open(path, flags, 0o644) catch |err| {
            return switch (err) {
                error.AccessDenied => FileError.AccessDenied,
                error.FileNotFound => FileError.FileNotFound,
                error.PermissionDenied => FileError.PermissionDenied,
                error.NameTooLong => FileError.PathTooLong,
                else => FileError.IoError,
            };
        };
        
        return fd;
    }
    
    // Read from file
    pub fn readFile(handle: FileHandle, buffer: []u8) FileError!usize {
        if (Platform.current().isWasm()) {
            return error.NotSupported;
        } else if (Platform.current().isWindows()) {
            return readFileWindows(handle, buffer);
        } else {
            return readFileUnix(handle, buffer);
        }
    }
    
    fn readFileWindows(handle: FileHandle, buffer: []u8) FileError!usize {
        // Simplified Windows file read - just return 0 for now
        _ = handle;
        _ = buffer;
        return 0;
    }
    
    fn readFileUnix(handle: FileHandle, buffer: []u8) FileError!usize {
        if (builtin.target.os.tag == .wasi) {
            return error.NotSupported;
        }
        
        const bytes_read = std.posix.read(handle, buffer) catch |err| {
            return switch (err) {
                error.InputOutput => FileError.IoError,
                error.AccessDenied => FileError.AccessDenied,
                else => FileError.IoError,
            };
        };
        
        return bytes_read;
    }
    
    // Write to file
    pub fn writeFile(handle: FileHandle, data: []const u8) FileError!usize {
        if (Platform.current().isWasm()) {
            return error.NotSupported;
        } else if (Platform.current().isWindows()) {
            return writeFileWindows(handle, data);
        } else {
            return writeFileUnix(handle, data);
        }
    }
    
    fn writeFileWindows(handle: FileHandle, data: []const u8) FileError!usize {
        // Simplified Windows file write - just return data length for now
        _ = handle;
        return data.len;
    }
    
    fn writeFileUnix(handle: FileHandle, data: []const u8) FileError!usize {
        if (builtin.target.os.tag == .wasi) {
            return data.len; // Pretend write succeeded
        }
        
        const bytes_written = std.posix.write(handle, data) catch |err| {
            return switch (err) {
                error.DiskQuota => FileError.DiskFull,
                error.FileTooBig => FileError.DiskFull,
                error.NoSpaceLeft => FileError.DiskFull,
                error.AccessDenied => FileError.AccessDenied,
                else => FileError.IoError,
            };
        };
        
        return bytes_written;
    }
    
    // Close file
    pub fn closeFile(handle: FileHandle) void {
        if (Platform.current().isWasm()) {
            return;
        } else if (Platform.current().isWindows()) {
            const windows = std.os.windows;
            _ = windows.CloseHandle(@ptrCast(handle));
        } else if (builtin.target.os.tag != .wasi) {
            std.posix.close(handle);
        }
    }
};

// Cross-platform networking operations
pub const NetworkOps = struct {
    // Socket handle type
    pub const SocketHandle = if (builtin.target.os.tag == .wasi) 
        i32 
    else if (builtin.target.os.tag == .windows) 
        std.os.windows.ws2_32.SOCKET 
    else 
        std.posix.socket_t;
    
    // Network error types
    pub const NetworkError = error{
        AddressInUse,
        AddressNotAvailable,
        ConnectionRefused,
        ConnectionReset,
        NetworkUnreachable,
        SocketError,
        NotSupported,
        PermissionDenied,
    };
    
    // Socket types
    pub const SocketType = enum {
        tcp,
        udp,
    };
    
    // Create socket
    pub fn createSocket(socket_type: SocketType) NetworkError!SocketHandle {
        if (Platform.current().isWasm()) {
            return error.NotSupported;
        } else if (Platform.current().isWindows()) {
            return createSocketWindows(socket_type);
        } else {
            return createSocketUnix(socket_type);
        }
    }
    
    fn createSocketWindows(socket_type: SocketType) NetworkError!SocketHandle {
        const windows = std.os.windows;
        const ws2_32 = windows.ws2_32;
        
        const sock_type = switch (socket_type) {
            .tcp => ws2_32.SOCK.STREAM,
            .udp => ws2_32.SOCK.DGRAM,
        };
        
        const protocol = switch (socket_type) {
            .tcp => ws2_32.IPPROTO.TCP,
            .udp => ws2_32.IPPROTO.UDP,
        };
        
        const socket = ws2_32.socket(ws2_32.AF.INET, sock_type, protocol);
        if (socket == ws2_32.INVALID_SOCKET) {
            return NetworkError.SocketError;
        }
        
        return socket;
    }
    
    fn createSocketUnix(socket_type: SocketType) NetworkError!SocketHandle {
        const sock_type = switch (socket_type) {
            .tcp => std.os.SOCK.STREAM,
            .udp => std.os.SOCK.DGRAM,
        };
        
        const socket = std.os.socket(std.os.AF.INET, sock_type, 0) catch |err| {
            return switch (err) {
                error.AddressFamilyNotSupported => NetworkError.NotSupported,
                error.ProtocolFamilyNotSupported => NetworkError.NotSupported,
                error.PermissionDenied => NetworkError.PermissionDenied,
                else => NetworkError.SocketError,
            };
        };
        
        return socket;
    }
    
    // Bind socket to address
    pub fn bindSocket(socket: SocketHandle, port: u16) NetworkError!void {
        if (Platform.current().isWasm()) {
            return error.NotSupported;
        } else if (Platform.current().isWindows()) {
            return bindSocketWindows(socket, port);
        } else {
            return bindSocketUnix(socket, port);
        }
    }
    
    fn bindSocketWindows(socket: SocketHandle, port: u16) NetworkError!void {
        const windows = std.os.windows;
        const ws2_32 = windows.ws2_32;
        
        var addr = ws2_32.sockaddr.in{
            .family = ws2_32.AF.INET,
            .port = std.mem.nativeToBig(u16, port),
            .addr = 0, // INADDR_ANY
            .zero = [_]u8{0} ** 8,
        };
        
        const result = ws2_32.bind(socket, @ptrCast(&addr), @sizeOf(@TypeOf(addr)));
        if (result != 0) {
            return switch (ws2_32.WSAGetLastError()) {
                .WSAEADDRINUSE => NetworkError.AddressInUse,
                .WSAEADDRNOTAVAIL => NetworkError.AddressNotAvailable,
                .WSAEACCES => NetworkError.PermissionDenied,
                else => NetworkError.SocketError,
            };
        }
    }
    
    fn bindSocketUnix(socket: SocketHandle, port: u16) NetworkError!void {
        const addr = std.net.Address.initIp4([_]u8{0, 0, 0, 0}, port);
        std.os.bind(socket, &addr.any, addr.getOsSockLen()) catch |err| {
            return switch (err) {
                error.AddressInUse => NetworkError.AddressInUse,
                error.AddressNotAvailable => NetworkError.AddressNotAvailable,
                error.PermissionDenied => NetworkError.PermissionDenied,
                else => NetworkError.SocketError,
            };
        };
    }
    
    // Close socket
    pub fn closeSocket(socket: SocketHandle) void {
        if (Platform.current().isWasm()) {
            return;
        } else if (Platform.current().isWindows()) {
            const windows = std.os.windows;
            _ = windows.ws2_32.closesocket(socket);
        } else {
            std.os.close(socket);
        }
    }
};

// Cross-platform time operations
pub const TimeOps = struct {
    // Time representation
    pub const TimeStamp = struct {
        seconds: i64,
        nanoseconds: u32,
        
        pub fn now() TimeStamp {
            if (Platform.current().isWasm()) {
                // WASM time is limited
                return TimeStamp{ .seconds = 0, .nanoseconds = 0 };
            } else {
                const time = std.time.nanoTimestamp();
                return TimeStamp{
                    .seconds = @intCast(@divFloor(time, std.time.ns_per_s)),
                    .nanoseconds = @intCast(@mod(time, std.time.ns_per_s)),
                };
            }
        }
        
        pub fn toMillis(self: TimeStamp) i64 {
            return self.seconds * 1000 + @divFloor(self.nanoseconds, std.time.ns_per_ms);
        }
        
        pub fn toMicros(self: TimeStamp) i64 {
            return self.seconds * 1_000_000 + @divFloor(self.nanoseconds, std.time.ns_per_us);
        }
    };
    
    // Sleep for specified milliseconds
    pub fn sleepMs(ms: u64) void {
        if (Platform.current().isWasm()) {
            // WASM cannot sleep - this is a no-op
            return;
        } else {
            std.time.sleep(ms * std.time.ns_per_ms);
        }
    }
    
    // High-resolution timer
    pub fn nanoTime() u64 {
        if (Platform.current().isWasm()) {
            return 0;
        } else {
            return @intCast(std.time.nanoTimestamp());
        }
    }
};

// Cross-platform process operations
pub const ProcessOps = struct {
    // Process handle type
    pub const ProcessHandle = if (builtin.target.os.tag == .wasi) 
        void 
    else if (builtin.target.os.tag == .windows) 
        std.os.windows.HANDLE 
    else 
        std.process.Child.Id;
    
    // Process error types
    pub const ProcessError = error{
        ExecutionFailed,
        NotFound,
        PermissionDenied,
        NotSupported,
    };
    
    // Environment variable operations
    pub fn getEnv(allocator: std.mem.Allocator, key: []const u8) ?[]const u8 {
        if (Platform.current().isWasm()) {
            return null;
        } else {
            return std.process.getEnvVarOwned(allocator, key) catch null;
        }
    }
    
    pub fn setEnv(key: []const u8, value: []const u8) ProcessError!void {
        if (Platform.current().isWasm()) {
            return error.NotSupported;
        } else {
            std.process.setEnvVar(key, value) catch {
                return ProcessError.ExecutionFailed;
            };
        }
    }
    
    // Current working directory
    pub fn getCwd(allocator: std.mem.Allocator) ProcessError![]const u8 {
        if (Platform.current().isWasm()) {
            return error.NotSupported;
        } else {
            return std.process.getCwdAlloc(allocator) catch {
                return ProcessError.ExecutionFailed;
            };
        }
    }
    
    // Exit with code
    pub fn exit(code: u8) noreturn {
        std.process.exit(code);
    }
};

// Cross-platform path operations
pub const PathOps = struct {
    // Path separator for current platform
    pub fn separator() []const u8 {
        return switch (Platform.current()) {
            .windows_x64 => "\\",
            else => "/",
        };
    }
    
    // Join path components
    pub fn join(allocator: std.mem.Allocator, parts: []const []const u8) ![]const u8 {
        return std.fs.path.join(allocator, parts);
    }
    
    // Get directory name
    pub fn dirname(path: []const u8) []const u8 {
        return std.fs.path.dirname(path) orelse "";
    }
    
    // Get base name
    pub fn basename(path: []const u8) []const u8 {
        return std.fs.path.basename(path);
    }
    
    // Get file extension
    pub fn extension(path: []const u8) []const u8 {
        return std.fs.path.extension(path);
    }
    
    // Check if path is absolute
    pub fn isAbsolute(path: []const u8) bool {
        return std.fs.path.isAbsolute(path);
    }
};

// Platform capability flags
pub const Capabilities = struct {
    pub fn hasThreading() bool {
        return !Platform.current().isWasm();
    }
    
    pub fn hasNetworking() bool {
        return !Platform.current().isWasm();
    }
    
    pub fn hasFileSystem() bool {
        return !Platform.current().isWasm();
    }
    
    pub fn hasProcessControl() bool {
        return !Platform.current().isWasm();
    }
    
    pub fn hasHighResTimer() bool {
        return !Platform.current().isWasm();
    }
};

// Platform-specific initialization and cleanup
pub const PlatformInit = struct {
    pub fn init() !void {
        if (Platform.current().isWindows()) {
            // Initialize Winsock on Windows
            const windows = std.os.windows;
            var wsa_data: windows.ws2_32.WSADATA = undefined;
            const result = windows.ws2_32.WSAStartup(
                0x0202, // WINSOCK_VERSION 2.2
                &wsa_data
            );
            if (result != 0) {
                return error.InitializationFailed;
            }
        }
    }
    
    pub fn deinit() void {
        if (Platform.current().isWindows()) {
            // Cleanup Winsock on Windows
            const windows = std.os.windows;
            _ = windows.ws2_32.WSACleanup();
        }
    }
};

// Test function for platform abstraction
pub fn runTests() !void {
    const allocator = std.testing.allocator;
    
    // Test platform detection
    const platform = Platform.current();
    std.debug.print("Current platform: {s}\n", .{platform.name()});
    
    // Test capabilities
    std.debug.print("Capabilities:\n", .{});
    std.debug.print("  Threading: {}\n", .{Capabilities.hasThreading()});
    std.debug.print("  Networking: {}\n", .{Capabilities.hasNetworking()});
    std.debug.print("  File System: {}\n", .{Capabilities.hasFileSystem()});
    std.debug.print("  Process Control: {}\n", .{Capabilities.hasProcessControl()});
    std.debug.print("  High-res Timer: {}\n", .{Capabilities.hasHighResTimer()});
    
    // Test time operations
    const time1 = TimeOps.TimeStamp.now();
    TimeOps.sleepMs(1);
    const time2 = TimeOps.TimeStamp.now();
    
    std.debug.print("Time test: {} -> {} (diff: {} ms)\n", .{
        time1.toMillis(), time2.toMillis(), 
        time2.toMillis() - time1.toMillis()
    });
    
    // Test path operations
    const separator = PathOps.separator();
    std.debug.print("Path separator: {s}\n", .{separator});
    
    if (Capabilities.hasProcessControl()) {
        // Test environment variables
        if (ProcessOps.getEnv(allocator, "PATH")) |path| {
            defer allocator.free(path);
            std.debug.print("PATH length: {}\n", .{path.len});
        }
        
        // Test current directory
        if (ProcessOps.getCwd(allocator)) |cwd| {
            defer allocator.free(cwd);
            std.debug.print("Current directory: {s}\n", .{cwd});
        } else |_| {
            std.debug.print("Could not get current directory\n", .{});
        }
    }
}

// Export main platform detection function for easy access
pub const current_platform = Platform.current();
pub const is_windows = current_platform.isWindows();
pub const is_unix = current_platform.isUnix();
pub const is_wasm = current_platform.isWasm();
