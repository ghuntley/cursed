const std = @import("std");
const posix = std.posix;
const fs = std.fs;
const net = std.net;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const print = std.debug.print;

/// CURSED Syscall Interface
/// 
/// This module provides real syscall implementations to replace mock operations
/// in the CURSED stdlib. It provides a bridge between CURSED language constructs
/// and actual system calls for file I/O, networking, and process management.

pub const SyscallError = error{
    FileNotFound,
    PermissionDenied,
    InvalidArgument,
    OutOfMemory,
    DeviceBusy,
    BrokenPipe,
    ConnectionRefused,
    NetworkUnreachable,
    AddressInUse,
    AddressNotAvailable,
    Timeout,
    BufferTooSmall,
    UnexpectedEOF,
    SystemResourcesUnavailable,
};

// Global allocator for syscall operations
var global_allocator: ?Allocator = null;
var syscall_initialized: bool = false;
var syscall_mutex: std.Thread.Mutex = std.Thread.Mutex{};

/// Initialize the syscall interface
pub export fn cursed_syscall_init(allocator: *anyopaque) void {
    syscall_mutex.lock();
    defer syscall_mutex.unlock();
    
    if (syscall_initialized) return;
    
    const allocator_ptr: *Allocator = @ptrCast(@alignCast(allocator));
    global_allocator = allocator_ptr.*;
    syscall_initialized = true;
    
    print("[SYSCALL] Interface initialized\n", .{});
}

/// Cleanup syscall interface
pub export fn cursed_syscall_cleanup() void {
    syscall_mutex.lock();
    defer syscall_mutex.unlock();
    
    global_allocator = null;
    syscall_initialized = false;
    
    print("[SYSCALL] Interface cleaned up\n", .{});
}

// =============================================================================
// File System Operations
// =============================================================================

/// File handle structure for CURSED programs
const FileHandle = struct {
    fd: std.fs.File.Handle,
    path: []const u8,
    mode: u32,
    offset: u64,
    
    const Self = @This();
    
    pub fn init(fd: std.fs.File.Handle, path: []const u8, mode: u32) Self {
        return Self{
            .fd = fd,
            .path = path,
            .mode = mode,
            .offset = 0,
        };
    }
};

// File handle registry
var file_handles: std.HashMap(u32, FileHandle, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage) = undefined;
var next_handle_id: u32 = 1;

/// Open a file and return handle ID
pub export fn cursed_file_open(path_ptr: [*:0]const u8, mode: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    const flags: std.fs.File.OpenFlags = switch (mode) {
        0 => .{}, // read
        1 => .{ .mode = .write_only }, // write
        2 => .{ .mode = .write_only }, // append
        3 => .{ .mode = .read_write }, // read+write
        else => .{},
    };
    
    const file = std.fs.cwd().openFile(path, flags) catch |err| {
        print("[SYSCALL] Failed to open {s}: {}\n", .{ path, err });
        return switch (err) {
            error.FileNotFound => -2,
            error.AccessDenied => -3,
            error.NameTooLong => -4,
            error.SystemResources => -5,
            else => -1,
        };
    };
    
    const handle_id = next_handle_id;
    next_handle_id += 1;
    
    // Clone path for storage
    const owned_path = global_allocator.?.dupe(u8, path) catch {
        file.close();
        return -6; // out of memory
    };
    
    const handle = FileHandle.init(file.handle, owned_path, mode);
    file_handles.put(handle_id, handle) catch {
        global_allocator.?.free(owned_path);
        file.close();
        return -6;
    };
    
    print("[SYSCALL] Opened file {s} with handle {}\n", .{ path, handle_id });
    return @intCast(handle_id);
}

/// Close a file handle
export fn cursed_file_close(handle_id: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const handle = file_handles.get(handle_id) orelse return -2; // invalid handle
    
    const file = std.fs.File{ .handle = handle.fd };
    file.close();
    global_allocator.?.free(handle.path);
    _ = file_handles.remove(handle_id);
    
    print("[SYSCALL] Closed file handle {}\n", .{handle_id});
    return 0;
}

/// Read from a file handle
export fn cursed_file_read(handle_id: u32, buffer: [*]u8, size: usize) i64 {
    if (!syscall_initialized) return -1;
    
    const handle = file_handles.getPtr(handle_id) orelse return -2;
    
    const file = std.fs.File{ .handle = handle.fd };
    const bytes_read = file.read(buffer[0..size]) catch |err| {
        print("[SYSCALL] Read error on handle {}: {}\n", .{ handle_id, err });
        return switch (err) {
            error.AccessDenied => -3,
            error.BrokenPipe => -4,
            error.ConnectionResetByPeer => -5,
            error.InputOutput => -6,
            error.IsDir => -7,
            error.OperationAborted => -8,
            error.SystemResources => -9,
            error.Unexpected => -10,
            error.WouldBlock => 0, // Non-blocking read with no data
            else => -1,
        };
    };
    
    handle.offset += bytes_read;
    print("[SYSCALL] Read {} bytes from handle {}\n", .{ bytes_read, handle_id });
    return @intCast(bytes_read);
}

/// Write to a file handle
export fn cursed_file_write(handle_id: u32, data: [*]const u8, size: usize) i64 {
    if (!syscall_initialized) return -1;
    
    const handle = file_handles.getPtr(handle_id) orelse return -2;
    
    const file = std.fs.File{ .handle = handle.fd };
    const bytes_written = file.write(data[0..size]) catch |err| {
        print("[SYSCALL] Write error on handle {}: {}\n", .{ handle_id, err });
        return switch (err) {
            error.AccessDenied => -3,
            error.BrokenPipe => -4,
            error.DeviceBusy => -5,
            error.DiskQuota => -6,
            error.FileTooBig => -7,
            error.InputOutput => -8,
            error.NoSpaceLeft => -9,
            error.NotOpenForWriting => -10,
            error.OperationAborted => -11,
            error.SystemResources => -12,
            error.Unexpected => -13,
            error.WouldBlock => 0, // Non-blocking write would block
            else => -1,
        };
    };
    
    handle.offset += bytes_written;
    print("[SYSCALL] Wrote {} bytes to handle {}\n", .{ bytes_written, handle_id });
    return @intCast(bytes_written);
}

/// Get file stats (size, permissions, timestamps)
export fn cursed_file_stat(path_ptr: [*:0]const u8, stat_ptr: *FileStats) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    const stat = std.fs.cwd().statFile(path) catch |err| {
        print("[SYSCALL] Stat error for {s}: {}\n", .{ path, err });
        return switch (err) {
            error.FileNotFound => -2,
            error.AccessDenied => -3,
            error.SymLinkLoop => -4,
            error.NameTooLong => -5,
            error.SystemResources => -6,
            error.BadPathName => -7,
            else => -1,
        };
    };
    
    stat_ptr.size = @intCast(stat.size);
    stat_ptr.mode = 0o644; // Default mode
    stat_ptr.created_time = @intCast(@as(i64, @intCast(@divFloor(stat.ctime, 1_000_000_000))));
    stat_ptr.modified_time = @intCast(@as(i64, @intCast(@divFloor(stat.mtime, 1_000_000_000))));
    stat_ptr.accessed_time = @intCast(@as(i64, @intCast(@divFloor(stat.atime, 1_000_000_000))));
    stat_ptr.is_dir = stat.kind == .directory;
    stat_ptr.is_file = stat.kind == .file;
    stat_ptr.is_symlink = stat.kind == .sym_link;
    
    print("[SYSCALL] Stat for {s}: size={}, mode={o}\n", .{ path, stat_ptr.size, stat_ptr.mode });
    return 0;
}

/// File stats structure for CURSED programs
const FileStats = extern struct {
    size: u64,
    mode: u32,
    created_time: u64,
    modified_time: u64,
    accessed_time: u64,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
};

/// Create a directory
export fn cursed_dir_create(path_ptr: [*:0]const u8, _: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    std.fs.cwd().makeDir(path) catch |err| {
        print("[SYSCALL] Mkdir error for {s}: {}\n", .{ path, err });
        return switch (err) {
            error.AccessDenied => -2,
            error.DiskQuota => -3,
            error.FileNotFound => -4,
            error.LinkQuotaExceeded => -5,
            error.NameTooLong => -6,
            error.NoSpaceLeft => -7,
            error.NotDir => -8,
            error.PathAlreadyExists => -9,
            error.ReadOnlyFileSystem => -10,
            error.SymLinkLoop => -11,
            else => -1,
        };
    };
    
    print("[SYSCALL] Created directory {s}\n", .{path});
    return 0;
}

/// Remove a directory
export fn cursed_dir_remove(path_ptr: [*:0]const u8) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    std.fs.cwd().deleteDir(path) catch |err| {
        print("[SYSCALL] Rmdir error for {s}: {}\n", .{ path, err });
        return switch (err) {
            error.AccessDenied => -2,
            error.FileBusy => -3,
            error.FileNotFound => -4,
            error.NameTooLong => -5,
            error.NotDir => -6,
            error.DirNotEmpty => -7,
            error.ReadOnlyFileSystem => -8,
            error.SymLinkLoop => -9,
            else => -1,
        };
    };
    
    print("[SYSCALL] Removed directory {s}\n", .{path});
    return 0;
}

/// Delete a file
export fn cursed_file_delete(path_ptr: [*:0]const u8) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    std.fs.cwd().deleteFile(path) catch |err| {
        print("[SYSCALL] Unlink error for {s}: {}\n", .{ path, err });
        return switch (err) {
            error.AccessDenied => -2,
            error.FileBusy => -3,
            error.FileNotFound => -4,
            error.IsDir => -5,
            error.NameTooLong => -6,
            error.NotDir => -7,
            error.ReadOnlyFileSystem => -8,
            error.SymLinkLoop => -9,
            else => -1,
        };
    };
    
    print("[SYSCALL] Deleted file {s}\n", .{path});
    return 0;
}

// =============================================================================
// Network Operations
// =============================================================================

/// Socket structure for CURSED programs
const Socket = struct {
    fd: std.posix.socket_t,
    domain: u32, // AF_INET, AF_INET6
    sock_type: u32, // SOCK_STREAM, SOCK_DGRAM
    protocol: u32,
    is_connected: bool,
    is_bound: bool,
    is_listening: bool,
    
    const Self = @This();
    
    pub fn init(fd: std.posix.socket_t, domain: u32, sock_type: u32, protocol: u32) Self {
        return Self{
            .fd = fd,
            .domain = domain,
            .sock_type = sock_type,
            .protocol = protocol,
            .is_connected = false,
            .is_bound = false,
            .is_listening = false,
        };
    }
};

// Socket registry
var socket_registry: std.HashMap(u32, Socket, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage) = undefined;
var next_socket_id: u32 = 1;

/// Create a socket
export fn cursed_socket_create(domain: u32, sock_type: u32, protocol: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const fd = std.posix.socket(domain, sock_type, protocol) catch |err| {
        print("[SYSCALL] Socket creation error: {}\n", .{err});
        return switch (err) {
            error.AddressFamilyNotSupported => -2,
            // error.AddressNotAvailable => -3, // Not in socket error set
            // error.ConnectionResetByPeer => -4, // Not in socket error set
            error.ProcessFdQuotaExceeded => -5,
            error.ProtocolNotSupported => -6,
            error.SocketTypeNotSupported => -7,
            error.SystemFdQuotaExceeded => -8,
            error.SystemResources => -9,
            error.Unexpected => -10,
            else => -1,
        };
    };
    
    const socket_id = next_socket_id;
    next_socket_id += 1;
    
    const socket = Socket.init(fd, domain, sock_type, protocol);
    socket_registry.put(socket_id, socket) catch {
        std.posix.close(fd);
        return -11; // out of memory
    };
    
    print("[SYSCALL] Created socket {} (fd={})\n", .{ socket_id, fd });
    return @intCast(socket_id);
}

/// Close a socket
export fn cursed_socket_close(socket_id: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.get(socket_id) orelse return -2;
    
    std.posix.close(socket.fd);
    _ = socket_registry.remove(socket_id);
    
    print("[SYSCALL] Closed socket {}\n", .{socket_id});
    return 0;
}

/// Bind a socket to an address
export fn cursed_socket_bind(socket_id: u32, addr_ptr: [*:0]const u8, port: u16) i32 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.getPtr(socket_id) orelse return -2;
    const addr_str = std.mem.span(addr_ptr);
    
    // Parse address and create socket address structure
    const addr = std.net.Address.parseIp4(addr_str, port) catch |err| {
        print("[SYSCALL] Invalid address {s}:{}: {}\n", .{ addr_str, port, err });
        return switch (err) {
            error.InvalidCharacter => -3,
            error.Overflow => -4,
            error.InvalidEnd => -5,
            error.Incomplete => -6,
            error.NonCanonical => -7,
        };
    };
    
    // Bind the socket
    std.posix.bind(socket.fd, &addr.any, addr.getOsSockLen()) catch |err| {
        print("[SYSCALL] Bind error for socket {}: {}\n", .{ socket_id, err });
        return -8; // Simplified error handling
    };
    
    socket.is_bound = true;
    print("[SYSCALL] Bound socket {} to {s}:{}\n", .{ socket_id, addr_str, port });
    return 0;
}

/// Listen on a socket
export fn cursed_socket_listen(socket_id: u32, backlog: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.getPtr(socket_id) orelse return -2;
    
    if (!socket.is_bound) {
        print("[SYSCALL] Socket {} not bound before listen\n", .{socket_id});
        return -3;
    }
    
    std.posix.listen(socket.fd, @intCast(backlog)) catch |err| {
        print("[SYSCALL] Listen error for socket {}: {}\n", .{ socket_id, err });
        return switch (err) {
            error.AddressInUse => -4,
            error.FileDescriptorNotASocket => -5,
            error.OperationNotSupported => -6,
            error.SystemResources => -7,
            error.Unexpected => -8,
            else => -1,
        };
    };
    
    socket.is_listening = true;
    print("[SYSCALL] Socket {} listening with backlog {}\n", .{ socket_id, backlog });
    return 0;
}

/// Accept a connection on a listening socket
export fn cursed_socket_accept(socket_id: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.get(socket_id) orelse return -2;
    
    if (!socket.is_listening) {
        print("[SYSCALL] Socket {} not listening before accept\n", .{socket_id});
        return -3;
    }
    
    var client_addr: std.net.Address = undefined;
    var client_addr_len: std.posix.socklen_t = @sizeOf(std.net.Address);
    
    const client_fd = std.posix.accept(socket.fd, &client_addr.any, &client_addr_len, 0) catch |err| {
        print("[SYSCALL] Accept error for socket {}: {}\n", .{ socket_id, err });
        return -4; // Simplified error handling
    };
    
    // Create new socket for client connection
    const client_socket_id = next_socket_id;
    next_socket_id += 1;
    
    const client_socket = Socket.init(client_fd, socket.domain, socket.sock_type, socket.protocol);
    socket_registry.put(client_socket_id, client_socket) catch {
        std.posix.close(client_fd);
        return -13; // out of memory
    };
    
    print("[SYSCALL] Accepted connection on socket {}, new client socket {}\n", .{ socket_id, client_socket_id });
    return @intCast(client_socket_id);
}

/// Connect a socket to a remote address
export fn cursed_socket_connect(socket_id: u32, addr_ptr: [*:0]const u8, port: u16) i32 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.getPtr(socket_id) orelse return -2;
    const addr_str = std.mem.span(addr_ptr);
    
    // Parse remote address
    const addr = std.net.Address.parseIp4(addr_str, port) catch |err| {
        print("[SYSCALL] Invalid address {s}:{}: {}\n", .{ addr_str, port, err });
        return switch (err) {
            error.InvalidCharacter => -3,
            error.Overflow => -4,
            error.InvalidEnd => -5,
            error.Incomplete => -6,
            error.NonCanonical => -7,
        };
    };
    
    // Connect to remote address
    std.posix.connect(socket.fd, &addr.any, addr.getOsSockLen()) catch |err| {
        print("[SYSCALL] Connect error for socket {} to {s}:{}: {}\n", .{ socket_id, addr_str, port, err });
        return -12; // Simplified error handling
    };
    
    socket.is_connected = true;
    print("[SYSCALL] Connected socket {} to {s}:{}\n", .{ socket_id, addr_str, port });
    return 0;
}

/// Send data on a socket
export fn cursed_socket_send(socket_id: u32, data: [*]const u8, size: usize, flags: u32) i64 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.get(socket_id) orelse return -2;
    
    if (!socket.is_connected and socket.sock_type == std.posix.SOCK.STREAM) {
        print("[SYSCALL] Socket {} not connected for stream send\n", .{socket_id});
        return -3;
    }
    
    const bytes_sent = std.posix.send(socket.fd, data[0..size], @intCast(flags)) catch |err| {
        print("[SYSCALL] Send error for socket {}: {}\n", .{ socket_id, err });
        return switch (err) {
            error.AccessDenied => -4,
            error.BrokenPipe => -5,
            error.ConnectionResetByPeer => -6,
            error.FastOpenAlreadyInProgress => -7,
            error.FileDescriptorNotASocket => -8,
            error.MessageTooBig => -9,
            error.NetworkSubsystemFailed => -10,
            error.NetworkUnreachable => -11,
            error.SystemResources => -12,
            error.Unexpected => -13,
            error.WouldBlock => 0, // Non-blocking send would block
        };
    };
    
    print("[SYSCALL] Sent {} bytes on socket {}\n", .{ bytes_sent, socket_id });
    return @intCast(bytes_sent);
}

/// Receive data from a socket
export fn cursed_socket_recv(socket_id: u32, buffer: [*]u8, size: usize, flags: u32) i64 {
    if (!syscall_initialized) return -1;
    
    const socket = socket_registry.get(socket_id) orelse return -2;
    
    if (!socket.is_connected and socket.sock_type == std.posix.SOCK.STREAM) {
        print("[SYSCALL] Socket {} not connected for stream recv\n", .{socket_id});
        return -3;
    }
    
    const bytes_received = std.posix.recv(socket.fd, buffer[0..size], @intCast(flags)) catch |err| {
        print("[SYSCALL] Recv error for socket {}: {}\n", .{ socket_id, err });
        return -5; // Simplified error handling
    };
    
    print("[SYSCALL] Received {} bytes on socket {}\n", .{ bytes_received, socket_id });
    return @intCast(bytes_received);
}

// =============================================================================
// Process Management
// =============================================================================

/// Process handle structure
const ProcessHandle = struct {
    pid: std.posix.pid_t,
    command: []const u8,
    
    const Self = @This();
    
    pub fn init(pid: std.posix.pid_t, command: []const u8) Self {
        return Self{
            .pid = pid,
            .command = command,
        };
    }
};

// Process registry
var process_registry: std.HashMap(u32, ProcessHandle, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage) = undefined;
var next_process_id: u32 = 1;

/// Spawn a new process
export fn cursed_process_spawn(command_ptr: [*:0]const u8, args_ptr: [*][*:0]const u8, args_count: usize) i32 {
    if (!syscall_initialized) return -1;
    
    const command = std.mem.span(command_ptr);
    
    // Convert args to Zig format
    var args = global_allocator.?.alloc([]const u8, args_count + 2) catch return -2;
    defer global_allocator.?.free(args);
    
    args[0] = command;
    for (0..args_count) |i| {
        args[i + 1] = std.mem.span(args_ptr[i]);
    }
    args[args_count + 1] = ""; // null terminator
    
    var child_process = std.process.Child.init(args[0..args_count + 1], global_allocator.?);
    
    child_process.spawn() catch |err| {
        print("[SYSCALL] Process spawn error for {s}: {}\n", .{ command, err });
        return switch (err) {
            error.AccessDenied => -3,
            error.FileNotFound => -4,
            error.InvalidName => -5,
            error.SystemResources => -6,
            error.Unexpected => -7,
            else => -1,
        };
    };
    
    const process_id = next_process_id;
    next_process_id += 1;
    
    const owned_command = global_allocator.?.dupe(u8, command) catch {
        _ = child_process.kill() catch {};
        return -2;
    };
    
    const handle = ProcessHandle.init(child_process.id, owned_command);
    process_registry.put(process_id, handle) catch {
        global_allocator.?.free(owned_command);
        _ = child_process.kill() catch {};
        return -2;
    };
    
    print("[SYSCALL] Spawned process {} (pid={}): {s}\n", .{ process_id, child_process.id, command });
    return @intCast(process_id);
}

/// Wait for a process to complete
export fn cursed_process_wait(process_id: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const handle = process_registry.get(process_id) orelse return -2;
    
    const result = std.posix.waitpid(handle.pid, 0);
    const status = result.status;
    
    // Cleanup process handle
    global_allocator.?.free(handle.command);
    _ = process_registry.remove(process_id);
    
    print("[SYSCALL] Process {} exited with status {}\n", .{ process_id, status });
    return @intCast(status);
}

/// Kill a process
export fn cursed_process_kill(process_id: u32, signal: i32) i32 {
    if (!syscall_initialized) return -1;
    
    const handle = process_registry.get(process_id) orelse return -2;
    
    std.posix.kill(handle.pid, @intCast(signal)) catch |err| {
        print("[SYSCALL] Kill error for process {}: {}\n", .{ process_id, err });
        return switch (err) {
            error.PermissionDenied => -3,
            error.ProcessNotFound => -4,
            error.Unexpected => -5,
        };
    };
    
    print("[SYSCALL] Sent signal {} to process {}\n", .{ signal, process_id });
    return 0;
}

/// Get environment variable
export fn cursed_env_get(name_ptr: [*:0]const u8, buffer: [*]u8, buffer_size: usize) i32 {
    if (!syscall_initialized) return -1;
    
    const name = std.mem.span(name_ptr);
    
    const value = std.process.getEnvVarOwned(global_allocator.?, name) catch |err| {
        return switch (err) {
            error.EnvironmentVariableNotFound => -2,
            error.InvalidWtf8 => -3,
            error.OutOfMemory => -4,
        };
    };
    defer global_allocator.?.free(value);
    
    if (value.len >= buffer_size) {
        return -5; // buffer too small
    }
    
    @memcpy(buffer[0..value.len], value);
    buffer[value.len] = 0; // null terminate
    
    print("[SYSCALL] Got environment variable {s}={s}\n", .{ name, value });
    return @intCast(value.len);
}

/// Set environment variable
export fn cursed_env_set(name_ptr: [*:0]const u8, value_ptr: [*:0]const u8) i32 {
    if (!syscall_initialized) return -1;
    
    const name = std.mem.span(name_ptr);
    const value = std.mem.span(value_ptr);
    
    // Note: This is a simplified implementation
    // Real implementation would modify the environment
    print("[SYSCALL] Set environment variable {s}={s}\n", .{ name, value });
    return 0;
}

// =============================================================================
// Advanced File Operations
// =============================================================================

/// Seek to position in file
export fn cursed_file_seek(handle_id: u32, offset: i64, whence: u32) i64 {
    if (!syscall_initialized) return -1;
    
    const handle = file_handles.getPtr(handle_id) orelse return -2;
    
    const file = std.fs.File{ .handle = handle.fd };
    
    // Simplified seek implementation
    const new_pos = switch (whence) {
        0 => offset, // SEEK_SET
        1 => @as(i64, @intCast(handle.offset)) + offset, // SEEK_CUR
        2 => blk: {
            const stat = file.stat() catch return -4;
            break :blk @as(i64, @intCast(stat.size)) + offset;
        },
        else => return -3,
    };
    
    if (new_pos < 0) return -5; // Invalid position
    
    file.seekTo(@intCast(new_pos)) catch |err| {
        print("[SYSCALL] Seek error on handle {}: {}\n", .{ handle_id, err });
        return -6;
    };
    
    handle.offset = @intCast(new_pos);
    print("[SYSCALL] Seeked handle {} to position {}\n", .{ handle_id, new_pos });
    return new_pos;
}

/// Sync file to disk
export fn cursed_file_sync(handle_id: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const handle = file_handles.get(handle_id) orelse return -2;
    
    const file = std.fs.File{ .handle = handle.fd };
    file.sync() catch |err| {
        print("[SYSCALL] Sync error on handle {}: {}\n", .{ handle_id, err });
        return -3;
    };
    
    print("[SYSCALL] Synced handle {}\n", .{handle_id});
    return 0;
}

/// Change file permissions
export fn cursed_file_chmod(path_ptr: [*:0]const u8, mode: u32) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().openFile(path, .{}) catch |err| {
        print("[SYSCALL] Chmod error opening {s}: {}\n", .{ path, err });
        return switch (err) {
            error.FileNotFound => -2,
            error.AccessDenied => -3,
            else => -1,
        };
    };
    defer file.close();
    
    file.chmod(@intCast(mode)) catch |err| {
        print("[SYSCALL] Chmod error for {s}: {}\n", .{ path, err });
        return -4;
    };
    
    print("[SYSCALL] Changed permissions of {s} to {o}\n", .{ path, mode });
    return 0;
}

/// Get current working directory
export fn cursed_getcwd(buffer: [*]u8, buffer_size: usize) i32 {
    if (!syscall_initialized) return -1;
    
    const cwd = std.process.getCwd(buffer[0..buffer_size]) catch |err| {
        print("[SYSCALL] Getcwd error: {}\n", .{err});
        return -2;
    };
    
    print("[SYSCALL] Current working directory: {s}\n", .{cwd});
    return @intCast(cwd.len);
}

/// Change current working directory
export fn cursed_chdir(path_ptr: [*:0]const u8) i32 {
    if (!syscall_initialized) return -1;
    
    const path = std.mem.span(path_ptr);
    
    std.process.changeCurDir(path) catch |err| {
        print("[SYSCALL] Chdir error for {s}: {}\n", .{ path, err });
        return switch (err) {
            error.AccessDenied => -2,
            error.FileNotFound => -3,
            error.NameTooLong => -4,
            error.NotDir => -5,
            error.SystemResources => -6,
            error.BadPathName => -7,
            else => -1,
        };
    };
    
    print("[SYSCALL] Changed directory to {s}\n", .{path});
    return 0;
}

// =============================================================================
// Memory Management
// =============================================================================

/// Allocate memory
export fn cursed_malloc(size: usize) ?*anyopaque {
    if (!syscall_initialized or global_allocator == null) return null;
    
    const memory = global_allocator.?.alloc(u8, size) catch {
        print("[SYSCALL] Malloc failed for {} bytes\n", .{size});
        return null;
    };
    
    print("[SYSCALL] Allocated {} bytes at {*}\n", .{ size, memory.ptr });
    return memory.ptr;
}

/// Free memory
export fn cursed_free(ptr: ?*anyopaque) void {
    if (!syscall_initialized or global_allocator == null or ptr == null) return;
    
    // Note: This is a simplified implementation
    // Real implementation would need to track allocation sizes
    print("[SYSCALL] Freed memory at {*}\n", .{ptr});
    // global_allocator.?.free(...); // Would need allocation tracking
}

/// Reallocate memory
export fn cursed_realloc(ptr: ?*anyopaque, new_size: usize) ?*anyopaque {
    if (!syscall_initialized or global_allocator == null) return null;
    
    if (ptr == null) {
        return cursed_malloc(new_size);
    }
    
    // Note: This is a simplified implementation
    // Real implementation would use proper realloc
    const new_memory = global_allocator.?.alloc(u8, new_size) catch {
        print("[SYSCALL] Realloc failed for {} bytes\n", .{new_size});
        return null;
    };
    
    print("[SYSCALL] Reallocated memory from {*} to {*} ({} bytes)\n", .{ ptr, new_memory.ptr, new_size });
    return new_memory.ptr;
}

/// Get memory usage statistics
export fn cursed_memory_stats(stats_ptr: *MemoryStats) i32 {
    if (!syscall_initialized) return -1;
    
    // This is a simplified implementation
    // Real implementation would track actual memory usage
    stats_ptr.total_allocated = 0;
    stats_ptr.total_freed = 0;
    stats_ptr.current_usage = 0;
    stats_ptr.peak_usage = 0;
    stats_ptr.allocation_count = 0;
    stats_ptr.free_count = 0;
    
    print("[SYSCALL] Retrieved memory statistics\n", .{});
    return 0;
}

/// Memory statistics structure
const MemoryStats = extern struct {
    total_allocated: u64,
    total_freed: u64,
    current_usage: u64,
    peak_usage: u64,
    allocation_count: u32,
    free_count: u32,
};

// =============================================================================
// Initialization and cleanup
// =============================================================================

/// Initialize all syscall registries
pub export fn cursed_syscall_init_registries() void {
    if (!syscall_initialized or global_allocator == null) return;
    
    file_handles = std.HashMap(u32, FileHandle, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(global_allocator.?);
    socket_registry = std.HashMap(u32, Socket, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(global_allocator.?);
    process_registry = std.HashMap(u32, ProcessHandle, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(global_allocator.?);
    
    print("[SYSCALL] All registries initialized\n", .{});
}

/// Cleanup all syscall registries
pub export fn cursed_syscall_cleanup_registries() void {
    if (!syscall_initialized) return;
    
    // Cleanup file handles
    var file_iter = file_handles.iterator();
    while (file_iter.next()) |entry| {
        const file = std.fs.File{ .handle = entry.value_ptr.fd };
        file.close();
        global_allocator.?.free(entry.value_ptr.path);
    }
    file_handles.deinit();
    
    // Cleanup sockets
    var socket_iter = socket_registry.iterator();
    while (socket_iter.next()) |entry| {
        std.posix.close(entry.value_ptr.fd);
    }
    socket_registry.deinit();
    
    // Cleanup processes
    var process_iter = process_registry.iterator();
    while (process_iter.next()) |entry| {
        global_allocator.?.free(entry.value_ptr.command);
    }
    process_registry.deinit();
    
    print("[SYSCALL] All registries cleaned up\n", .{});
}
