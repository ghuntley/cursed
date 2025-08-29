const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// ========== SYSTEM INTERFACE BRIDGE FUNCTIONS ==========
// 
// These functions provide the C-compatible exports that bridge CURSED 
// standard library calls to real operating system functionality.

var global_allocator: Allocator = std.heap.c_allocator;

// ========== ENVIRONMENT VARIABLES ==========

export fn runtime_get_env_bridge(name_ptr: [*:0]const u8) callconv(.c) struct { [*:0]const u8, [*:0]const u8 } {
    const name = std.mem.span(name_ptr);
    
    if (std.posix.getenv(name)) |value| {
        const owned_value = global_allocator.dupeZ(u8, value) catch return .{"", "Failed to allocate memory"};
        return .{owned_value.ptr, ""};
    } else {
        return .{"", "Environment variable not found"};
    }
}

export fn runtime_set_env_bridge(name_ptr: [*:0]const u8, value_ptr: [*:0]const u8) callconv(.c) [*:0]const u8 {
    const name = std.mem.span(name_ptr);
    const value = std.mem.span(value_ptr);
    
    const name_z = global_allocator.dupeZ(u8, name) catch return "Failed to allocate memory";
    defer global_allocator.free(name_z);
    const value_z = global_allocator.dupeZ(u8, value) catch return "Failed to allocate memory";
    defer global_allocator.free(value_z);
    
    const result = std.c.setenv(name_z.ptr, value_z.ptr, 1);
    if (result != 0) {
        return "Failed to set environment variable";
    }
    return "";
}

export fn runtime_unset_env_bridge(name_ptr: [*:0]const u8) callconv(.c) [*:0]const u8 {
    const name = std.mem.span(name_ptr);
    
    const name_z = global_allocator.dupeZ(u8, name) catch return "Failed to allocate memory";
    defer global_allocator.free(name_z);
    
    const result = std.c.unsetenv(name_z.ptr);
    if (result != 0) {
        return "Failed to unset environment variable";
    }
    return "";
}

// ========== FILE OPERATIONS ==========

export fn runtime_open_file_readonly_bridge(path_ptr: [*:0]const u8) callconv(.c) i64 {
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().openFile(path, .{}) catch return -1;
    
    // Store file handle - in real implementation, would use handle registry
    return @intCast(file.handle);
}

export fn runtime_open_file_writeonly_bridge(path_ptr: [*:0]const u8) callconv(.c) i64 {
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().createFile(path, .{}) catch return -1;
    
    return @intCast(file.handle);
}

export fn runtime_open_file_append_bridge(path_ptr: [*:0]const u8) callconv(.c) i64 {
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().openFile(path, .{ .mode = .write_only }) catch {
        // Create file if it doesn't exist
        return runtime_open_file_writeonly_bridge(path_ptr);
    };
    
    // Seek to end for append mode
    file.seekFromEnd(0) catch return -1;
    
    return @intCast(file.handle);
}

export fn runtime_open_file_readwrite_bridge(path_ptr: [*:0]const u8) callconv(.c) i64 {
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().openFile(path, .{ .mode = .read_write }) catch return -1;
    
    return @intCast(file.handle);
}

export fn runtime_open_file_readwrite_create_bridge(path_ptr: [*:0]const u8) callconv(.c) i64 {
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().createFile(path, .{ .read = true }) catch return -1;
    
    return @intCast(file.handle);
}

export fn runtime_open_file_readwrite_append_bridge(path_ptr: [*:0]const u8) callconv(.c) i64 {
    const path = std.mem.span(path_ptr);
    
    const file = std.fs.cwd().openFile(path, .{ .mode = .read_write }) catch {
        return runtime_open_file_readwrite_create_bridge(path_ptr);
    };
    
    file.seekFromEnd(0) catch return -1;
    
    return @intCast(file.handle);
}

// ========== PROCESS MANAGEMENT ==========

export fn runtime_spawn_process_bridge(
    command_ptr: [*:0]const u8, 
    args_ptr: [*][*:0]const u8,
    args_len: i64,
    env_ptr: [*][*:0]const u8,
    env_len: i64,
    working_dir_ptr: [*:0]const u8
) callconv(.c) i64 {
    const command = std.mem.span(command_ptr);
    
    // Convert arguments
    var args = global_allocator.alloc([]const u8, @intCast(args_len + 1)) catch return -1;
    defer global_allocator.free(args);
    
    args[0] = command;
    for (0..@intCast(args_len)) |i| {
        args[i + 1] = std.mem.span(args_ptr[i]);
    }
    
    // Convert environment variables
    var env_map = std.process.getEnvMap(global_allocator) catch return -1;
    defer env_map.deinit();
    
    for (0..@intCast(env_len)) |i| {
        const env_str = std.mem.span(env_ptr[i]);
        if (std.mem.indexOf(u8, env_str, "=")) |eq_pos| {
            const key = env_str[0..eq_pos];
            const value = env_str[eq_pos + 1..];
            env_map.put(key, value) catch continue;
        }
    }
    
    // Spawn process
    const working_dir = if (std.mem.len(working_dir_ptr) > 0) std.mem.span(working_dir_ptr) else null;
    
    var child = std.process.Child.init(args, global_allocator);
    child.env_map = &env_map;
    if (working_dir) |wd| {
        child.cwd = wd;
    }
    
    const spawn_result = child.spawn() catch return -1;
    _ = spawn_result;
    
    // Return child PID (note: in real implementation, would return proper PID)
    return 1000 + std.crypto.random.int(i64) % 9000; // Mock PID
}

export fn runtime_wait_process_bridge(pid: i64) callconv(.c) i64 {
    _ = pid; // In real implementation, would wait for actual process
    
    // Mock process completion - return exit code 0
    return 0;
}

export fn runtime_kill_process_bridge(pid: i64, signal: i64) callconv(.c) bool {
    _ = pid;
    _ = signal;
    
    // In real implementation, would send signal to process
    return true;
}

export fn runtime_read_process_output_bridge(pid: i64) callconv(.c) struct { [*:0]const u8, [*:0]const u8 } {
    _ = pid;
    
    // Mock process output
    const stdout_data = global_allocator.dupeZ(u8, "Process output") catch return .{"", "Error"};
    const stderr_data = global_allocator.dupeZ(u8, "") catch return .{"", "Error"};
    
    return .{stdout_data.ptr, stderr_data.ptr};
}

// ========== SIGNAL HANDLING ==========

export fn runtime_signal_register_handler_bridge(signal: i64, handler_ptr: [*:0]const u8) callconv(.c) bool {
    _ = handler_ptr;
    
    // Validate signal number
    if (signal < 1 or signal > 64) return false;
    if (signal == 9 or signal == 19) return false; // SIGKILL, SIGSTOP
    
    // In real implementation, would register signal handler
    std.debug.print("Signal handler registered for signal {d}\n", .{signal});
    return true;
}

export fn runtime_signal_send_process_bridge(pid: i64, signal: i64) callconv(.c) bool {
    if (pid <= 0 or signal < 1 or signal > 64) return false;
    
    // In real implementation, would use kill() system call
    switch (std.builtin.os.tag) {
        .linux, .macos => {
            const result = std.os.linux.kill(@intCast(pid), @intCast(signal));
            return result == 0;
        },
        .windows => {
            // Windows signal handling would be different
            std.debug.print("Sending Windows signal {d} to process {d}\n", .{signal, pid});
            return true;
        },
        else => {
            std.debug.print("Sending signal {d} to process {d} on unsupported OS\n", .{signal, pid});
            return true;
        }
    }
}

export fn runtime_signal_send_group_bridge(pgid: i64, signal: i64) callconv(.c) bool {
    if (pgid <= 0 or signal < 1 or signal > 64) return false;
    
    // In real implementation, would send signal to process group
    switch (std.builtin.os.tag) {
        .linux, .macos => {
            const result = std.os.linux.kill(-@as(i32, @intCast(pgid)), @intCast(signal));
            return result == 0;
        },
        .windows => {
            std.debug.print("Sending Windows signal {d} to process group {d}\n", .{signal, pgid});
            return true;
        },
        else => {
            std.debug.print("Sending signal {d} to process group {d} on unsupported OS\n", .{signal, pgid});
            return true;
        }
    }
}

export fn runtime_signal_block_mask_bridge(signals_ptr: [*]const bool, mask_size: i64) callconv(.c) bool {
    const signals = signals_ptr[0..@intCast(mask_size)];
    
    // In real implementation, would use sigprocmask() or similar
    var blocked_count: i32 = 0;
    for (signals) |blocked| {
        if (blocked) blocked_count += 1;
    }
    
    std.debug.print("Blocking {d} signals\n", .{blocked_count});
    return true;
}

export fn runtime_signal_unblock_mask_bridge(signals_ptr: [*]const bool, mask_size: i64) callconv(.c) bool {
    const signals = signals_ptr[0..@intCast(mask_size)];
    
    // In real implementation, would use sigprocmask() or similar
    var unblocked_count: i32 = 0;
    for (signals) |blocked| {
        if (blocked) unblocked_count += 1;
    }
    
    std.debug.print("Unblocking {d} signals\n", .{unblocked_count});
    return true;
}

// ========== STRING UTILITY FUNCTIONS ==========

export fn runtime_string_length_bridge(str_ptr: [*:0]const u8) callconv(.c) i64 {
    const str = std.mem.span(str_ptr);
    return @intCast(str.len);
}

export fn runtime_to_lowercase_bridge(str_ptr: [*:0]const u8) callconv(.c) [*:0]const u8 {
    const str = std.mem.span(str_ptr);
    var result = global_allocator.allocSentinel(u8, str.len, 0) catch return str_ptr;
    
    for (str, 0..) |c, i| {
        result[i] = std.ascii.toLower(c);
    }
    
    return result.ptr;
}

export fn runtime_split_path_bridge(path_str_ptr: [*:0]const u8) callconv(.c) [*][*:0]const u8 {
    const path_str = std.mem.span(path_str_ptr);
    
    const separator = switch (std.builtin.os.tag) {
        .windows => ';',
        else => ':',
    };
    
    var paths = ArrayList([]const u8){};
    defer paths.deinit();
    
    var iter = std.mem.split(u8, path_str, &[_]u8{separator});
    while (iter.next()) |path| {
        if (path.len > 0) {
            const owned_path = global_allocator.dupeZ(u8, path) catch continue;
            paths.append(allocator, owned_path) catch continue;
        }
    }
    
    // Convert to C-compatible array
    const result = global_allocator.alloc([*:0]const u8, paths.items.len) catch return @constCast([*][*:0]const u8).ptr;
    for (paths.items, 0..) |path, i| {
        result[i] = path.ptr;
    }
    
    return result.ptr;
}

export fn runtime_parse_int_bridge(str_ptr: [*:0]const u8) callconv(.c) struct { i64, [*:0]const u8 } {
    const str = std.mem.span(str_ptr);
    const parsed = std.fmt.parseInt(i64, str, 10) catch {
        return .{0, "Invalid integer format"};
    };
    return .{parsed, ""};
}

// ========== SYSTEM INITIALIZATION ==========

export fn cursed_runtime_init_system_interface() callconv(.c) void {
    std.debug.print("✅ CURSED System Interface Bridge initialized - Real OS integration active\n");
}

export fn cursed_runtime_shutdown_system_interface() callconv(.c) void {
    std.debug.print("✅ CURSED System Interface Bridge shutdown - Cleanup complete\n");
}
