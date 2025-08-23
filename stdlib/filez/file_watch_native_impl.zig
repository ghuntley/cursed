// Native File Watching Implementation for CURSED Runtime
// Cross-platform file system monitoring with inotify, kqueue, and ReadDirectoryChangesW

const std = @import("std");
const builtin = @import("builtin");

// Platform-specific imports
const linux = std.os.linux;
const darwin = if (builtin.target.os.tag == .macos) std.c else struct {};
const windows = std.os.windows;

// File watching events matching CURSED enum values
pub const WatchEventType = enum(u32) {
    created = 1,
    modified = 2,
    deleted = 3,
    moved = 4,
    attributes = 5,
};

pub const WatchEvent = struct {
    event_type: WatchEventType,
    path: []const u8,
    old_path: []const u8, // For move events
    timestamp: u64,
    is_directory: bool,
};

pub const WatchError = error{
    PlatformNotSupported,
    InitializationFailed,
    WatchCreationFailed,
    InvalidPath,
    ResourceLimitExceeded,
    PermissionDenied,
    SystemError,
};

// Cross-platform file watcher abstraction
pub const FileWatcher = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    platform_handle: PlatformHandle,
    callback: ?*const fn(WatchEvent) void,
    thread_pool: std.Thread.Pool,
    running: std.atomic.Value(bool),
    
    const PlatformHandle = union(enum) {
        linux: LinuxWatcher,
        macos: MacOSWatcher,
        windows: WindowsWatcher,
        unsupported: void,
    };
    
    pub fn init(allocator: std.mem.Allocator) WatchError!Self {
        const thread_pool = std.Thread.Pool.init(std.Thread.Pool.Options{
            .allocator = allocator,
            .n_jobs = null, // Use system default
        }) catch return WatchError.InitializationFailed;
        
        const platform_handle = switch (builtin.target.os.tag) {
            .linux => PlatformHandle{ .linux = LinuxWatcher.init(allocator) catch return WatchError.InitializationFailed },
            .macos => PlatformHandle{ .macos = MacOSWatcher.init(allocator) catch return WatchError.InitializationFailed },
            .windows => PlatformHandle{ .windows = WindowsWatcher.init(allocator) catch return WatchError.InitializationFailed },
            else => return WatchError.PlatformNotSupported,
        };
        
        return Self{
            .allocator = allocator,
            .platform_handle = platform_handle,
            .callback = null,
            .thread_pool = thread_pool,
            .running = std.atomic.Value(bool).init(false),
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.stop();
        self.thread_pool.deinit();
        
        switch (self.platform_handle) {
            .linux => |*linux_watcher| linux_watcher.deinit(),
            .macos => |*macos_watcher| macos_watcher.deinit(),
            .windows => |*windows_watcher| windows_watcher.deinit(),
            .unsupported => {},
        }
    }
    
    pub fn startWatching(self: *Self, path: []const u8, recursive: bool, callback: *const fn(WatchEvent) void) WatchError!void {
        if (self.running.load(.acquire)) {
            return; // Already running
        }
        
        self.callback = callback;
        self.running.store(true, .release);
        
        switch (self.platform_handle) {
            .linux => |*linux_watcher| try linux_watcher.startWatching(path, recursive, self),
            .macos => |*macos_watcher| try macos_watcher.startWatching(path, recursive, self),
            .windows => |*windows_watcher| try windows_watcher.startWatching(path, recursive, self),
            .unsupported => return WatchError.PlatformNotSupported,
        }
    }
    
    pub fn stop(self: *Self) void {
        if (!self.running.load(.acquire)) {
            return; // Already stopped
        }
        
        self.running.store(false, .release);
        
        switch (self.platform_handle) {
            .linux => |*linux_watcher| linux_watcher.stop(),
            .macos => |*macos_watcher| macos_watcher.stop(),
            .windows => |*windows_watcher| windows_watcher.stop(),
            .unsupported => {},
        }
    }
    
    pub fn deliverEvent(self: *Self, event: WatchEvent) void {
        if (self.callback) |callback| {
            callback(event);
        }
    }
    
    pub fn isRunning(self: *Self) bool {
        return self.running.load(.acquire);
    }
};

// Linux inotify-based implementation
const LinuxWatcher = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    inotify_fd: ?std.posix.fd_t,
    watch_descriptors: std.ArrayList(i32),
    event_thread: ?std.Thread,
    
    pub fn init(allocator: std.mem.Allocator) WatchError!Self {
        if (builtin.target.os.tag != .linux) {
            return WatchError.PlatformNotSupported;
        }
        
        return Self{
            .allocator = allocator,
            .inotify_fd = null,
            .watch_descriptors = std.ArrayList(i32).init(allocator),
            .event_thread = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.stop();
        
        if (self.inotify_fd) |fd| {
            std.posix.close(fd);
        }
        
        self.watch_descriptors.deinit();
    }
    
    pub fn startWatching(self: *Self, path: []const u8, recursive: bool, watcher: *FileWatcher) WatchError!void {
        // Initialize inotify
        const inotify_fd = linux.inotify_init1(linux.IN.CLOEXEC) catch {
            return WatchError.InitializationFailed;
        };
        self.inotify_fd = inotify_fd;
        
        // Add watch for the path
        try self.addWatch(path, recursive);
        
        // Start event processing thread
        self.event_thread = std.Thread.spawn(.{}, eventLoop, .{ self, watcher }) catch {
            return WatchError.SystemError;
        };
    }
    
    fn addWatch(self: *Self, path: []const u8, recursive: bool) WatchError!void {
        if (self.inotify_fd == null) return WatchError.InitializationFailed;
        
        // Build inotify mask
        const mask = linux.IN.CREATE | linux.IN.DELETE | linux.IN.MODIFY | 
                    linux.IN.MOVED_FROM | linux.IN.MOVED_TO | linux.IN.ATTRIB;
        
        // Add watch descriptor
        const path_z = self.allocator.dupeZ(u8, path) catch return WatchError.SystemError;
        defer self.allocator.free(path_z);
        
        const wd = linux.inotify_add_watch(self.inotify_fd.?, path_z.ptr, mask) catch {
            return WatchError.WatchCreationFailed;
        };
        
        self.watch_descriptors.append(wd) catch return WatchError.SystemError;
        
        // If recursive, add watches for subdirectories
        if (recursive) {
            try self.addRecursiveWatches(path);
        }
    }
    
    fn addRecursiveWatches(self: *Self, base_path: []const u8) WatchError!void {
        var dir = std.fs.openDirAbsolute(base_path, .{ .iterate = true }) catch {
            return WatchError.InvalidPath;
        };
        defer dir.close();
        
        var iterator = dir.iterate();
        while (iterator.next() catch null) |entry| {
            if (entry.kind == .directory) {
                const full_path = std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ base_path, entry.name }) catch continue;
                defer self.allocator.free(full_path);
                
                // Recursively add watch for subdirectory
                self.addWatch(full_path, true) catch continue;
            }
        }
    }
    
    pub fn stop(self: *Self) void {
        if (self.event_thread) |thread| {
            thread.join();
            self.event_thread = null;
        }
        
        // Remove all watch descriptors
        if (self.inotify_fd) |fd| {
            for (self.watch_descriptors.items) |wd| {
                _ = linux.inotify_rm_watch(fd, wd);
            }
        }
        
        self.watch_descriptors.clearRetainingCapacity();
    }
    
    fn eventLoop(self: *Self, watcher: *FileWatcher) void {
        var buffer: [4096]u8 = undefined;
        
        while (watcher.isRunning()) {
            if (self.inotify_fd == null) break;
            
            const bytes_read = std.posix.read(self.inotify_fd.?, &buffer) catch |err| {
                if (err == error.WouldBlock) {
                    std.time.sleep(10 * std.time.ns_per_ms); // 10ms sleep
                    continue;
                }
                break;
            };
            
            if (bytes_read == 0) continue;
            
            // Parse inotify events
            var offset: usize = 0;
            while (offset < bytes_read) {
                const event = @as(*const linux.inotify_event, @ptrCast(@alignCast(&buffer[offset])));
                
                // Convert to WatchEvent
                const watch_event = self.convertInotifyEvent(event, &buffer[offset + @sizeOf(linux.inotify_event)]);
                if (watch_event) |we| {
                    watcher.deliverEvent(we);
                }
                
                // Move to next event
                offset += @sizeOf(linux.inotify_event) + event.len;
            }
        }
    }
    
    fn convertInotifyEvent(self: *Self, event: *const linux.inotify_event, name_ptr: [*]const u8) ?WatchEvent {
        _ = self;
        
        const event_type: WatchEventType = if (event.mask & linux.IN.CREATE != 0) 
            .created
        else if (event.mask & linux.IN.DELETE != 0)
            .deleted
        else if (event.mask & linux.IN.MODIFY != 0)
            .modified
        else if (event.mask & (linux.IN.MOVED_FROM | linux.IN.MOVED_TO) != 0)
            .moved
        else if (event.mask & linux.IN.ATTRIB != 0)
            .attributes
        else
            return null;
            
        const name = if (event.len > 0) 
            std.mem.sliceTo(@as([*:0]const u8, @ptrCast(name_ptr)), 0)
        else
            "";
            
        return WatchEvent{
            .event_type = event_type,
            .path = name,
            .old_path = "",
            .timestamp = @intCast(std.time.timestamp()),
            .is_directory = (event.mask & linux.IN.ISDIR) != 0,
        };
    }
};

// macOS kqueue-based implementation  
const MacOSWatcher = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    kqueue_fd: ?std.posix.fd_t,
    watched_fds: std.ArrayList(std.posix.fd_t),
    event_thread: ?std.Thread,
    
    pub fn init(allocator: std.mem.Allocator) WatchError!Self {
        if (builtin.target.os.tag != .macos) {
            return WatchError.PlatformNotSupported;
        }
        
        return Self{
            .allocator = allocator,
            .kqueue_fd = null,
            .watched_fds = std.ArrayList(std.posix.fd_t).init(allocator),
            .event_thread = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.stop();
        
        if (self.kqueue_fd) |fd| {
            std.posix.close(fd);
        }
        
        for (self.watched_fds.items) |fd| {
            std.posix.close(fd);
        }
        
        self.watched_fds.deinit();
    }
    
    pub fn startWatching(self: *Self, path: []const u8, recursive: bool, watcher: *FileWatcher) WatchError!void {
        // Create kqueue
        const kqueue_fd = std.posix.kqueue() catch {
            return WatchError.InitializationFailed;
        };
        self.kqueue_fd = kqueue_fd;
        
        // Open path for monitoring
        try self.addWatch(path, recursive);
        
        // Start event processing thread
        self.event_thread = std.Thread.spawn(.{}, eventLoop, .{ self, watcher }) catch {
            return WatchError.SystemError;
        };
    }
    
    fn addWatch(self: *Self, path: []const u8, recursive: bool) WatchError!void {
        if (self.kqueue_fd == null) return WatchError.InitializationFailed;
        
        // Open file/directory for monitoring
        const fd = std.posix.open(path, .{ .ACCMODE = .RDONLY }, 0) catch {
            return WatchError.InvalidPath;
        };
        
        self.watched_fds.append(fd) catch {
            std.posix.close(fd);
            return WatchError.SystemError;
        };
        
        // Set up kevent
        var kevent_struct = std.mem.zeroes(std.posix.Kevent);
        kevent_struct.ident = @intCast(fd);
        kevent_struct.filter = std.posix.system.EVFILT_VNODE;
        kevent_struct.flags = std.posix.system.EV_ADD | std.posix.system.EV_CLEAR;
        kevent_struct.fflags = std.posix.system.NOTE_DELETE | std.posix.system.NOTE_WRITE | 
                              std.posix.system.NOTE_EXTEND | std.posix.system.NOTE_ATTRIB;
        
        const result = std.posix.kevent(self.kqueue_fd.?, &[_]std.posix.Kevent{kevent_struct}, &[_]std.posix.Kevent{}, null);
        if (result < 0) {
            return WatchError.WatchCreationFailed;
        }
        
        // If recursive, add watches for subdirectories
        if (recursive) {
            try self.addRecursiveWatches(path);
        }
    }
    
    fn addRecursiveWatches(self: *Self, base_path: []const u8) WatchError!void {
        var dir = std.fs.openDirAbsolute(base_path, .{ .iterate = true }) catch {
            return WatchError.InvalidPath;
        };
        defer dir.close();
        
        var iterator = dir.iterate();
        while (iterator.next() catch null) |entry| {
            if (entry.kind == .directory) {
                const full_path = std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ base_path, entry.name }) catch continue;
                defer self.allocator.free(full_path);
                
                self.addWatch(full_path, true) catch continue;
            }
        }
    }
    
    pub fn stop(self: *Self) void {
        if (self.event_thread) |thread| {
            thread.join();
            self.event_thread = null;
        }
        
        for (self.watched_fds.items) |fd| {
            std.posix.close(fd);
        }
        
        self.watched_fds.clearRetainingCapacity();
    }
    
    fn eventLoop(self: *Self, watcher: *FileWatcher) void {
        var events: [16]std.posix.Kevent = undefined;
        
        while (watcher.isRunning()) {
            if (self.kqueue_fd == null) break;
            
            const timeout = std.posix.timespec{ .tv_sec = 1, .tv_nsec = 0 };
            const num_events = std.posix.kevent(self.kqueue_fd.?, &[_]std.posix.Kevent{}, &events, &timeout) catch {
                std.time.sleep(10 * std.time.ns_per_ms);
                continue;
            };
            
            for (events[0..@intCast(num_events)]) |event| {
                const watch_event = self.convertKevent(event);
                if (watch_event) |we| {
                    watcher.deliverEvent(we);
                }
            }
        }
    }
    
    fn convertKevent(self: *Self, event: std.posix.Kevent) ?WatchEvent {
        _ = self;
        
        const event_type: WatchEventType = if (event.fflags & std.posix.system.NOTE_DELETE != 0)
            .deleted
        else if (event.fflags & std.posix.system.NOTE_WRITE != 0)
            .modified
        else if (event.fflags & std.posix.system.NOTE_EXTEND != 0)
            .modified
        else if (event.fflags & std.posix.system.NOTE_ATTRIB != 0)
            .attributes
        else
            return null;
            
        return WatchEvent{
            .event_type = event_type,
            .path = "", // Would need to track fd->path mapping
            .old_path = "",
            .timestamp = @intCast(std.time.timestamp()),
            .is_directory = false, // Would need to determine from fd
        };
    }
};

// Windows ReadDirectoryChangesW implementation
const WindowsWatcher = struct {
    const Self = @This();
    
    allocator: std.mem.Allocator,
    dir_handle: ?windows.HANDLE,
    event_thread: ?std.Thread,
    
    pub fn init(allocator: std.mem.Allocator) WatchError!Self {
        if (builtin.target.os.tag != .windows) {
            return WatchError.PlatformNotSupported;
        }
        
        return Self{
            .allocator = allocator,
            .dir_handle = null,
            .event_thread = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.stop();
        
        if (self.dir_handle) |handle| {
            _ = windows.CloseHandle(handle);
        }
    }
    
    pub fn startWatching(self: *Self, path: []const u8, recursive: bool, watcher: *FileWatcher) WatchError!void {
        // Convert path to wide string
        var path_w: [windows.PATH_MAX_WIDE]u16 = undefined;
        const len = std.unicode.utf8ToUtf16Le(path_w[0..], path) catch {
            return WatchError.InvalidPath;
        };
        path_w[len] = 0;
        
        // Open directory handle
        const handle = windows.CreateFileW(
            &path_w,
            windows.GENERIC_READ,
            windows.FILE_SHARE_READ | windows.FILE_SHARE_WRITE | windows.FILE_SHARE_DELETE,
            null,
            windows.OPEN_EXISTING,
            windows.FILE_FLAG_BACKUP_SEMANTICS,
            null,
        ) catch {
            return WatchError.InvalidPath;
        };
        
        if (handle == windows.INVALID_HANDLE_VALUE) {
            return WatchError.WatchCreationFailed;
        }
        
        self.dir_handle = handle;
        
        // Start event processing thread  
        self.event_thread = std.Thread.spawn(.{}, eventLoop, .{ self, watcher, recursive }) catch {
            return WatchError.SystemError;
        };
    }
    
    pub fn stop(self: *Self) void {
        if (self.event_thread) |thread| {
            thread.join();
            self.event_thread = null;
        }
    }
    
    fn eventLoop(self: *Self, watcher: *FileWatcher, recursive: bool) void {
        var buffer: [8192]u8 = undefined;
        var bytes_returned: windows.DWORD = 0;
        
        const notify_filter = windows.FILE_NOTIFY_CHANGE_FILE_NAME |
                             windows.FILE_NOTIFY_CHANGE_DIR_NAME |
                             windows.FILE_NOTIFY_CHANGE_ATTRIBUTES |
                             windows.FILE_NOTIFY_CHANGE_SIZE |
                             windows.FILE_NOTIFY_CHANGE_LAST_WRITE |
                             windows.FILE_NOTIFY_CHANGE_CREATION;
        
        while (watcher.isRunning()) {
            if (self.dir_handle == null) break;
            
            const success = windows.ReadDirectoryChangesW(
                self.dir_handle.?,
                &buffer,
                buffer.len,
                if (recursive) windows.TRUE else windows.FALSE,
                notify_filter,
                &bytes_returned,
                null,
                null,
            ) catch {
                std.time.sleep(100 * std.time.ns_per_ms);
                continue;
            };
            
            if (success == 0 or bytes_returned == 0) {
                std.time.sleep(100 * std.time.ns_per_ms);
                continue;
            }
            
            // Parse FILE_NOTIFY_INFORMATION structures
            self.parseNotifyBuffer(buffer[0..bytes_returned], watcher);
        }
    }
    
    fn parseNotifyBuffer(self: *Self, buffer: []const u8, watcher: *FileWatcher) void {
        var offset: usize = 0;
        
        while (offset < buffer.len) {
            const info = @as(*const windows.FILE_NOTIFY_INFORMATION, @ptrCast(@alignCast(&buffer[offset])));
            
            // Convert filename from UTF-16 to UTF-8
            const filename_utf16 = @as([*]const u16, @ptrCast(&buffer[offset + @sizeOf(windows.FILE_NOTIFY_INFORMATION)]))[0..info.FileNameLength / 2];
            
            var filename_utf8: [256]u8 = undefined;
            const filename_len = std.unicode.utf16leToUtf8(filename_utf8[0..], filename_utf16) catch {
                // Skip if conversion fails
                if (info.NextEntryOffset == 0) break;
                offset += info.NextEntryOffset;
                continue;
            };
            
            const watch_event = self.convertNotifyInfo(info, filename_utf8[0..filename_len]);
            if (watch_event) |we| {
                watcher.deliverEvent(we);
            }
            
            if (info.NextEntryOffset == 0) break;
            offset += info.NextEntryOffset;
        }
    }
    
    fn convertNotifyInfo(self: *Self, info: *const windows.FILE_NOTIFY_INFORMATION, filename: []const u8) ?WatchEvent {
        _ = self;
        
        const event_type: WatchEventType = switch (info.Action) {
            windows.FILE_ACTION_ADDED => .created,
            windows.FILE_ACTION_REMOVED => .deleted,
            windows.FILE_ACTION_MODIFIED => .modified,
            windows.FILE_ACTION_RENAMED_OLD_NAME => .moved,
            windows.FILE_ACTION_RENAMED_NEW_NAME => .moved,
            else => return null,
        };
        
        // Make a copy of the filename since the buffer is temporary
        const filename_copy = std.heap.page_allocator.dupe(u8, filename) catch return null;
        
        return WatchEvent{
            .event_type = event_type,
            .path = filename_copy,
            .old_path = "",
            .timestamp = @intCast(std.time.timestamp()),
            .is_directory = false, // Would need additional logic to determine
        };
    }
};

// Export functions for CURSED runtime integration
export fn cursed_file_watcher_create() ?*FileWatcher {
    const allocator = std.heap.page_allocator; // Use page allocator for simplicity
    
    const watcher = allocator.create(FileWatcher) catch return null;
    watcher.* = FileWatcher.init(allocator) catch {
        allocator.destroy(watcher);
        return null;
    };
    
    return watcher;
}

export fn cursed_file_watcher_start(watcher: *FileWatcher, path_ptr: [*]const u8, path_len: usize, recursive: bool) bool {
    const path = path_ptr[0..path_len];
    
    // Dummy callback for testing - in real implementation this would be connected to CURSED callback system
    const callback = struct {
        fn eventCallback(event: WatchEvent) void {
            std.log.info("File event: type={}, path={s}", .{ event.event_type, event.path });
        }
    }.eventCallback;
    
    watcher.startWatching(path, recursive, callback) catch return false;
    return true;
}

export fn cursed_file_watcher_stop(watcher: *FileWatcher) void {
    watcher.stop();
}

export fn cursed_file_watcher_destroy(watcher: *FileWatcher) void {
    const allocator = watcher.allocator;
    watcher.deinit();
    allocator.destroy(watcher);
}

export fn cursed_file_watcher_is_running(watcher: *FileWatcher) bool {
    return watcher.isRunning();
}

// Additional Windows API definitions not in std
extern "kernel32" fn ReadDirectoryChangesW(
    hDirectory: windows.HANDLE,
    lpBuffer: *anyopaque,
    nBufferLength: windows.DWORD,
    bWatchSubtree: windows.BOOL,
    dwNotifyFilter: windows.DWORD,
    lpBytesReturned: *windows.DWORD,
    lpOverlapped: ?*windows.OVERLAPPED,
    lpCompletionRoutine: ?*const fn(*anyopaque, windows.DWORD, *windows.OVERLAPPED) callconv(windows.WINAPI) void,
) callconv(windows.WINAPI) windows.BOOL;

// Test suite
test "file watcher creation" {
    var watcher = FileWatcher.init(std.testing.allocator) catch return;
    defer watcher.deinit();
    
    try std.testing.expect(!watcher.isRunning());
}

test "platform support" {
    const supported_platforms = [_]std.Target.Os.Tag{ .linux, .macos, .windows };
    
    const is_supported = for (supported_platforms) |platform| {
        if (builtin.target.os.tag == platform) break true;
    } else false;
    
    if (!is_supported) {
        try std.testing.expectError(WatchError.PlatformNotSupported, FileWatcher.init(std.testing.allocator));
    }
}
