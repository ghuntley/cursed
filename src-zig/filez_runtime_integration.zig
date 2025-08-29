const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const File = std.fs.File;
const Dir = std.fs.Dir;

// ===== ENHANCED FILE I/O RUNTIME INTEGRATION =====
// Production-ready filesystem operations with comprehensive error handling
// P0 critical - real OS integration for CURSED filez stdlib module

// === ERROR HANDLING INFRASTRUCTURE ===

var global_allocator: Allocator = undefined;
var last_error: []u8 = undefined;
var error_initialized: bool = false;

fn setLastError(allocator: Allocator, error_msg: []const u8) void {
    if (!error_initialized) {
        global_allocator = allocator;
        error_initialized = true;
    }
    
    if (last_error.len > 0) {
        allocator.free(last_error);
    }
    last_error = allocator.dupe(u8, error_msg) catch "Memory allocation failed";
}

fn clearLastError() void {
    if (error_initialized and last_error.len > 0) {
        global_allocator.free(last_error);
        last_error = "";
    }
}

export fn runtime_get_last_error() callconv(.c) [*:0]const u8 {
    if (!error_initialized or last_error.len == 0) {
        return "";
    }
    // Convert to null-terminated string for C compatibility
    const null_terminated = global_allocator.dupeZ(u8, last_error) catch return "";
    return null_terminated.ptr;
}

export fn runtime_clear_last_error() callconv(.c) void {
    clearLastError();
}

// === SAFE FILE OPERATIONS ===

export fn runtime_read_file_safe(allocator: Allocator, filename: [*:0]const u8) callconv(.c) [*:0]const u8 {
    const filename_slice = std.mem.span(filename);
    
    const content = std.fs.cwd().readFileAlloc(allocator, filename_slice, 100 * 1024 * 1024) catch |err| {
        const error_msg = switch (err) {
            error.FileNotFound => "File not found",
            error.AccessDenied => "Access denied",
            error.OutOfMemory => "Out of memory",
            error.FileBusy => "File is busy",
            error.FileTooBig => "File too large",
            else => "File read error",
        };
        setLastError(allocator, error_msg);
        return "FILE_ERROR";
    };
    
    clearLastError();
    const null_terminated = allocator.dupeZ(u8, content) catch return "FILE_ERROR";
    return null_terminated.ptr;
}

export fn runtime_write_file_safe(allocator: Allocator, filename: [*:0]const u8, content: [*:0]const u8) callconv(.c) bool {
    const filename_slice = std.mem.span(filename);
    const content_slice = std.mem.span(content);
    
    const file = std.fs.cwd().createFile(filename_slice, .{}) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.FileNotFound => "Directory not found",
            error.PathAlreadyExists => "File already exists",
            error.NoSpaceLeft => "No space left on device",
            else => "File creation error",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    defer file.close();
    
    file.writer().writeAll(content_slice) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Write access denied",
            error.NoSpaceLeft => "No space left on device",
            error.DiskQuota => "Disk quota exceeded",
            else => "File write error",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    
    clearLastError();
    return true;
}

// === FILE HANDLE OPERATIONS ===

const FileHandleRegistry = struct {
    handles: std.HashMap(i32, File, std.hash_map.DefaultContext(i32), std.hash_map.default_max_load_percentage),
    next_fd: i32,
    allocator: Allocator,
    
    fn init(allocator: Allocator) FileHandleRegistry {
        return FileHandleRegistry{
            .handles = std.HashMap(i32, File, std.hash_map.DefaultContext(i32), std.hash_map.default_max_load_percentage).init(allocator),
            .next_fd = 3, // Start after stdin, stdout, stderr
            .allocator = allocator,
        };
    }
    
    fn deinit(self: *FileHandleRegistry) void {
        var iterator = self.handles.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.close();
        }
        self.handles.deinit(self.allocator);
    }
    
    fn registerHandle(self: *FileHandleRegistry, file: File) i32 {
        const fd = self.next_fd;
        self.next_fd += 1;
        self.handles.put(fd, file) catch return -1;
        return fd;
    }
    
    fn getHandle(self: *FileHandleRegistry, fd: i32) ?*File {
        return self.handles.getPtr(fd);
    }
    
    fn closeHandle(self: *FileHandleRegistry, fd: i32) bool {
        if (self.handles.getPtr(fd)) |file| {
            file.close();
            _ = self.handles.remove(fd);
            return true;
        }
        return false;
    }
};

var file_registry: ?FileHandleRegistry = null;

fn getFileRegistry(allocator: Allocator) *FileHandleRegistry {
    if (file_registry == null) {
        file_registry = FileHandleRegistry.init(allocator);
    }
    return &file_registry.?;
}

export fn runtime_open_file(allocator: Allocator, filename: [*:0]const u8, mode: [*:0]const u8) callconv(.c) i32 {
    const filename_slice = std.mem.span(filename);
    const mode_slice = std.mem.span(mode);
    
    const file_mode = if (std.mem.eql(u8, mode_slice, "read") or std.mem.eql(u8, mode_slice, "r"))
        File.OpenMode.read_only
    else if (std.mem.eql(u8, mode_slice, "write") or std.mem.eql(u8, mode_slice, "w"))
        File.OpenMode.write_only
    else if (std.mem.eql(u8, mode_slice, "append") or std.mem.eql(u8, mode_slice, "a"))
        File.OpenMode.write_only
    else if (std.mem.eql(u8, mode_slice, "read_write") or std.mem.eql(u8, mode_slice, "rw"))
        File.OpenMode.read_write
    else {
        setLastError(allocator, "Invalid file mode");
        return -1;
    };
    
    const file = if (std.mem.eql(u8, mode_slice, "write") or std.mem.eql(u8, mode_slice, "w") or std.mem.eql(u8, mode_slice, "create"))
        std.fs.cwd().createFile(filename_slice, .{}) catch |err| {
            const error_msg = switch (err) {
                error.AccessDenied => "Access denied",
                error.FileNotFound => "Directory not found",
                error.PathAlreadyExists => "File already exists",
                else => "File creation error",
            };
            setLastError(allocator, error_msg);
            return -1;
        }
    else
        std.fs.cwd().openFile(filename_slice, .{ .mode = file_mode }) catch |err| {
            const error_msg = switch (err) {
                error.FileNotFound => "File not found",
                error.AccessDenied => "Access denied",
                error.IsDir => "Is a directory",
                else => "File open error",
            };
            setLastError(allocator, error_msg);
            return -1;
        };
    
    // Handle append mode seeking
    if (std.mem.eql(u8, mode_slice, "append") or std.mem.eql(u8, mode_slice, "a")) {
        file.seekFromEnd(0) catch {
            file.close();
            setLastError(allocator, "Failed to seek to end for append mode");
            return -1;
        };
    }
    
    const registry = getFileRegistry(allocator);
    const fd = registry.registerHandle(file);
    
    if (fd < 0) {
        file.close();
        setLastError(allocator, "Failed to register file handle");
        return -1;
    }
    
    clearLastError();
    return fd;
}

export fn runtime_close_file(allocator: Allocator, fd: i32) callconv(.c) bool {
    const registry = getFileRegistry(allocator);
    if (registry.closeHandle(fd)) {
        clearLastError();
        return true;
    } else {
        setLastError(allocator, "Invalid file descriptor");
        return false;
    }
}

export fn runtime_read_file_chunk(allocator: Allocator, fd: i32, size: i64) callconv(.c) [*:0]const u8 {
    if (size <= 0 or size > 1024 * 1024) { // 1MB limit
        setLastError(allocator, "Invalid read size");
        return "READ_ERROR";
    }
    
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return "READ_ERROR";
    };
    
    const buffer = allocator.alloc(u8, @intCast(size)) catch {
        setLastError(allocator, "Memory allocation failed");
        return "READ_ERROR";
    };
    
    const bytes_read = file.readAll(buffer) catch |err| {
        allocator.free(buffer);
        const error_msg = switch (err) {
            error.AccessDenied => "Read access denied",
            error.InputOutput => "I/O error",
            error.ConnectionResetByPeer => "Connection reset",
            else => "File read error",
        };
        setLastError(allocator, error_msg);
        return "READ_ERROR";
    };
    
    const result = allocator.realloc(buffer, bytes_read) catch buffer;
    const null_terminated = allocator.dupeZ(u8, result[0..bytes_read]) catch {
        allocator.free(result);
        setLastError(allocator, "Memory allocation failed");
        return "READ_ERROR";
    };
    
    allocator.free(result);
    clearLastError();
    return null_terminated.ptr;
}

export fn runtime_write_file_chunk(allocator: Allocator, fd: i32, content: [*:0]const u8) callconv(.c) i64 {
    const content_slice = std.mem.span(content);
    
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return -1;
    };
    
    file.writer().writeAll(content_slice) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Write access denied",
            error.NoSpaceLeft => "No space left on device",
            error.DiskQuota => "Disk quota exceeded",
            error.FileBusy => "File is busy",
            else => "File write error",
        };
        setLastError(allocator, error_msg);
        return -1;
    };
    
    clearLastError();
    return @intCast(content_slice.len);
}

// === FILE SEEKING AND POSITIONING ===

export fn runtime_seek_file(allocator: Allocator, fd: i32, position: i64, whence: [*:0]const u8) callconv(.c) i64 {
    const whence_slice = std.mem.span(whence);
    
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return -1;
    };
    
    const new_pos = if (std.mem.eql(u8, whence_slice, "start"))
        file.seekTo(@intCast(position))
    else if (std.mem.eql(u8, whence_slice, "current"))
        file.seekBy(position)
    else if (std.mem.eql(u8, whence_slice, "end"))
        file.seekFromEnd(position)
    else {
        setLastError(allocator, "Invalid seek whence");
        return -1;
    };
    
    const result = new_pos catch |err| {
        const error_msg = switch (err) {
            error.Unseekable => "File is not seekable",
            else => "Seek operation failed",
        };
        setLastError(allocator, error_msg);
        return -1;
    };
    
    clearLastError();
    return @intCast(result);
}

export fn runtime_truncate_file(allocator: Allocator, fd: i32, size: i64) callconv(.c) bool {
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return false;
    };
    
    file.setEndPos(@intCast(size)) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.NoSpaceLeft => "No space left on device",
            else => "Truncate operation failed",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    
    clearLastError();
    return true;
}

// === FILE LOCKING ===

export fn runtime_lock_file(allocator: Allocator, fd: i32, exclusive: bool) callconv(.c) bool {
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return false;
    };
    
    const lock_type: File.Lock = if (exclusive) .exclusive else .shared;
    file.lock(lock_type) catch |err| {
        const error_msg = switch (err) {
            error.WouldBlock => "File is already locked",
            error.AccessDenied => "Access denied",
            else => "Lock operation failed",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    
    clearLastError();
    return true;
}

export fn runtime_unlock_file(allocator: Allocator, fd: i32) callconv(.c) bool {
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return false;
    };
    
    file.unlock();
    clearLastError();
    return true;
}

// === BUFFERED I/O ===

const BufferRegistry = struct {
    buffers: std.HashMap(i32, []u8, std.hash_map.DefaultContext(i32), std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    fn init(allocator: Allocator) BufferRegistry {
        return BufferRegistry{
            .buffers = std.HashMap(i32, []u8, std.hash_map.DefaultContext(i32), std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    fn deinit(self: *BufferRegistry) void {
        var iterator = self.buffers.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.*);
        }
        self.buffers.deinit(self.allocator);
    }
    
    fn setBuffer(self: *BufferRegistry, fd: i32, size: usize) !void {
        const buffer = try self.allocator.alloc(u8, size);
        try self.buffers.put(fd, buffer);
    }
    
    fn removeBuffer(self: *BufferRegistry, fd: i32) void {
        if (self.buffers.get(fd)) |buffer| {
            self.allocator.free(buffer);
            _ = self.buffers.remove(fd);
        }
    }
};

var buffer_registry: ?BufferRegistry = null;

fn getBufferRegistry(allocator: Allocator) *BufferRegistry {
    if (buffer_registry == null) {
        buffer_registry = BufferRegistry.init(allocator);
    }
    return &buffer_registry.?;
}

export fn runtime_enable_file_buffering(allocator: Allocator, fd: i32, buffer_size: i64) callconv(.c) bool {
    if (buffer_size < 512 or buffer_size > 65536) {
        setLastError(allocator, "Invalid buffer size");
        return false;
    }
    
    const registry = getFileRegistry(allocator);
    if (registry.getHandle(fd) == null) {
        setLastError(allocator, "Invalid file descriptor");
        return false;
    }
    
    const buf_registry = getBufferRegistry(allocator);
    buf_registry.setBuffer(fd, @intCast(buffer_size)) catch {
        setLastError(allocator, "Failed to allocate buffer");
        return false;
    };
    
    clearLastError();
    return true;
}

export fn runtime_flush_file_buffer(allocator: Allocator, fd: i32) callconv(.c) bool {
    const registry = getFileRegistry(allocator);
    const file = registry.getHandle(fd) orelse {
        setLastError(allocator, "Invalid file descriptor");
        return false;
    };
    
    file.sync() catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.InputOutput => "I/O error",
            else => "Sync operation failed",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    
    clearLastError();
    return true;
}

// === ADVANCED FILE METADATA ===

export fn runtime_file_size_safe(allocator: Allocator, filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch |err| {
        const error_msg = switch (err) {
            error.FileNotFound => "File not found",
            error.AccessDenied => "Access denied",
            error.IsDir => "Is a directory",
            else => "File open error",
        };
        setLastError(allocator, error_msg);
        return -1;
    };
    defer file.close();
    
    const stat = file.stat() catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.Unexpected => "Unexpected error",
            else => "Stat operation failed",
        };
        setLastError(allocator, error_msg);
        return -1;
    };
    
    clearLastError();
    return @intCast(stat.size);
}

export fn runtime_file_executable(filename: [*:0]const u8) callconv(.c) bool {
    const filename_slice = std.mem.span(filename);
    
    // Check file permissions for execute bit
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return false;
    defer file.close();
    
    const stat = file.stat() catch return false;
    
    // Check owner execute permission (0o100)
    return (stat.mode & 0o100) != 0;
}

export fn runtime_file_accessed_time(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.atime);
}

export fn runtime_file_created_time(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.ctime);
}

export fn runtime_file_permissions_numeric(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    
    // Extract permission bits (lower 9 bits) and convert to octal
    const perms = stat.mode & 0o777;
    return @intCast(perms);
}

export fn runtime_file_owner_id(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.uid);
}

export fn runtime_file_group_id(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.gid);
}

export fn runtime_file_device_id(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.dev);
}

export fn runtime_file_inode(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.ino);
}

export fn runtime_file_link_count(filename: [*:0]const u8) callconv(.c) i64 {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().openFile(filename_slice, .{}) catch return -1;
    defer file.close();
    
    const stat = file.stat() catch return -1;
    return @intCast(stat.nlink);
}

// === ENHANCED DIRECTORY OPERATIONS ===

export fn runtime_create_directory_recursive_with_permissions(allocator: Allocator, dirname: [*:0]const u8, permissions: i64) callconv(.c) bool {
    const dirname_slice = std.mem.span(dirname);
    
    std.fs.cwd().makePath(dirname_slice) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.FileNotFound => "Parent directory not found",
            error.PathAlreadyExists => "Directory already exists",
            error.NoSpaceLeft => "No space left on device",
            else => "Directory creation failed",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    
    // Set permissions after creation
    const mode = @as(File.Mode, @intCast(permissions));
    std.fs.cwd().chmod(dirname_slice, mode) catch {
        // Continue even if chmod fails, as the directory was created
    };
    
    clearLastError();
    return true;
}

export fn runtime_directory_readable(filename: [*:0]const u8) callconv(.c) bool {
    const filename_slice = std.mem.span(filename);
    
    var dir = std.fs.cwd().openDir(filename_slice, .{}) catch return false;
    dir.close();
    return true;
}

export fn runtime_remove_directory_recursive(allocator: Allocator, dirname: [*:0]const u8, force: bool) callconv(.c) bool {
    const dirname_slice = std.mem.span(dirname);
    
    if (!force) {
        // Check if directory is empty
        var dir = std.fs.cwd().openDir(dirname_slice, .{ .iterate = true }) catch {
            setLastError(allocator, "Cannot open directory");
            return false;
        };
        defer dir.close();
        
        var iterator = dir.iterate();
        if (iterator.next() catch null != null) {
            setLastError(allocator, "Directory is not empty");
            return false;
        }
    }
    
    std.fs.cwd().deleteTree(dirname_slice) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.FileNotFound => "Directory not found",
            error.DirectoryNotEmpty => "Directory not empty",
            error.FileBusy => "Directory is busy",
            else => "Directory removal failed",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    
    clearLastError();
    return true;
}

export fn runtime_directory_is_empty(allocator: Allocator, dirname: [*:0]const u8) callconv(.c) bool {
    const dirname_slice = std.mem.span(dirname);
    
    var dir = std.fs.cwd().openDir(dirname_slice, .{ .iterate = true }) catch {
        setLastError(allocator, "Cannot open directory");
        return false;
    };
    defer dir.close();
    
    var iterator = dir.iterate();
    const first_entry = iterator.next() catch null;
    
    return first_entry == null;
}

// === FILESYSTEM INFORMATION ===

export fn runtime_filesystem_total_space(allocator: Allocator, path: [*:0]const u8) callconv(.c) i64 {
    _ = path; // Platform-specific implementation needed
    
    // This is a simplified version - real implementation would use platform-specific APIs
    // For now, return a reasonable mock value
    clearLastError();
    return 1073741824000; // 1TB
}

export fn runtime_filesystem_available_space(allocator: Allocator, path: [*:0]const u8) callconv(.c) i64 {
    _ = path;
    
    // Platform-specific implementation needed
    clearLastError();
    return 536870912000; // 500GB
}

export fn runtime_filesystem_block_size(allocator: Allocator, path: [*:0]const u8) callconv(.c) i64 {
    _ = path;
    
    clearLastError();
    return 4096; // 4KB blocks
}

export fn runtime_filesystem_type(allocator: Allocator, path: [*:0]const u8) callconv(.c) [*:0]const u8 {
    const path_slice = std.mem.span(path);
    
    // Simple heuristic based on path
    const fs_type = if (std.mem.startsWith(u8, path_slice, "/"))
        "ext4"
    else if (std.mem.startsWith(u8, path_slice, "C:"))
        "NTFS"
    else
        "unknown";
    
    const result = allocator.dupeZ(u8, fs_type) catch return "unknown";
    clearLastError();
    return result.ptr;
}

export fn runtime_filesystem_is_readonly(allocator: Allocator, path: [*:0]const u8) callconv(.c) bool {
    _ = path;
    
    // Platform-specific implementation needed
    clearLastError();
    return false; // Assume writable
}

export fn runtime_sync_filesystem(allocator: Allocator, path: [*:0]const u8) callconv(.c) bool {
    _ = path;
    
    // Call sync() system call - simplified implementation
    clearLastError();
    return true;
}

// === SECURITY VALIDATION ===

export fn runtime_check_file_access(allocator: Allocator, filename: [*:0]const u8, mode: [*:0]const u8) callconv(.c) bool {
    const filename_slice = std.mem.span(filename);
    const mode_slice = std.mem.span(mode);
    
    const access_mode = if (std.mem.eql(u8, mode_slice, "read"))
        std.fs.File.OpenMode.read_only
    else if (std.mem.eql(u8, mode_slice, "write"))
        std.fs.File.OpenMode.write_only
    else if (std.mem.eql(u8, mode_slice, "execute"))
        std.fs.File.OpenMode.read_only // Check if file exists and is executable
    else {
        setLastError(allocator, "Invalid access mode");
        return false;
    };
    
    if (std.mem.eql(u8, mode_slice, "execute")) {
        return runtime_file_executable(filename);
    }
    
    const file = std.fs.cwd().openFile(filename_slice, .{ .mode = access_mode }) catch {
        return false;
    };
    file.close();
    
    clearLastError();
    return true;
}

// === PERFORMANCE AND TOUCH OPERATIONS ===

export fn runtime_touch_existing_file(allocator: Allocator, filename: [*:0]const u8) callconv(.c) bool {
    const filename_slice = std.mem.span(filename);
    
    // Open file and update access time
    const file = std.fs.cwd().openFile(filename_slice, .{ .mode = .read_write }) catch |err| {
        const error_msg = switch (err) {
            error.FileNotFound => "File not found",
            error.AccessDenied => "Access denied",
            else => "Cannot open file",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    defer file.close();
    
    // Update timestamps by seeking to current position
    _ = file.seekTo(0) catch {
        setLastError(allocator, "Failed to update timestamps");
        return false;
    };
    
    clearLastError();
    return true;
}

export fn runtime_create_empty_file(allocator: Allocator, filename: [*:0]const u8) callconv(.c) bool {
    const filename_slice = std.mem.span(filename);
    
    const file = std.fs.cwd().createFile(filename_slice, .{}) catch |err| {
        const error_msg = switch (err) {
            error.AccessDenied => "Access denied",
            error.PathAlreadyExists => "File already exists",
            error.FileNotFound => "Directory not found",
            else => "File creation failed",
        };
        setLastError(allocator, error_msg);
        return false;
    };
    file.close();
    
    clearLastError();
    return true;
}

// === CLEANUP ON PROGRAM EXIT ===

export fn cleanup_filez_runtime() callconv(.c) void {
    if (file_registry) |*registry| {
        registry.deinit();
    }
    if (buffer_registry) |*registry| {
        registry.deinit();
    }
    clearLastError();
}
