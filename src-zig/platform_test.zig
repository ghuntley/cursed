// Platform abstraction testing executable
const std = @import("std");
const platform = @import("platform_abstraction.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    std.debug.print("CURSED Platform Abstraction Test Suite\n");
    std.debug.print("=====================================\n\n", .{});
    
    // Initialize platform (required for some platforms like Windows)
    platform.PlatformInit.init() catch |err| {
        std.debug.print("Platform initialization failed: {any}\n", .{err});
        return;
    };
    defer platform.PlatformInit.deinit(allocator);
    
    // Test platform detection
    const current_platform = platform.Platform.current();
    std.debug.print("Platform Detection:\n", .{});
    std.debug.print("  Current platform: {s}\n", .{current_platform.name()});
    std.debug.print("  Is Windows: {}\n", .{current_platform.isWindows()});
    std.debug.print("  Is Unix: {}\n", .{current_platform.isUnix()});
    std.debug.print("  Is WASM: {}\n", .{current_platform.isWasm()});
    std.debug.print("\n", .{});
    
    // Test platform capabilities
    std.debug.print("Platform Capabilities:\n", .{});
    std.debug.print("  Threading: {}\n", .{platform.Capabilities.hasThreading()});
    std.debug.print("  Networking: {}\n", .{platform.Capabilities.hasNetworking()});
    std.debug.print("  File System: {}\n", .{platform.Capabilities.hasFileSystem()});
    std.debug.print("  Process Control: {}\n", .{platform.Capabilities.hasProcessControl()});
    std.debug.print("  High-res Timer: {}\n", .{platform.Capabilities.hasHighResTimer()});
    std.debug.print("\n", .{});
    
    // Test time operations
    std.debug.print("Time Operations:\n", .{});
    const time1 = platform.TimeOps.TimeStamp.now();
    platform.TimeOps.sleepMs(10);
    const time2 = platform.TimeOps.TimeStamp.now();
    
    std.debug.print("  Start time: {}ms\n", .{time1.toMillis()});
    std.debug.print("  End time: {}ms\n", .{time2.toMillis()});
    std.debug.print("  Duration: {}ms\n", .{time2.toMillis() - time1.toMillis()});
    
    const nano_time = platform.TimeOps.nanoTime();
    std.debug.print("  Nano time: {}\n", .{nano_time});
    std.debug.print("\n", .{});
    
    // Test path operations
    std.debug.print("Path Operations:\n", .{});
    std.debug.print("  Separator: '{s}'\n", .{platform.PathOps.separator()});
    
    const test_path = "/home/user/test.txt";
    std.debug.print("  Test path: {s}\n", .{test_path});
    std.debug.print("  Dirname: {s}\n", .{platform.PathOps.dirname(test_path)});
    std.debug.print("  Basename: {s}\n", .{platform.PathOps.basename(test_path)});
    std.debug.print("  Extension: {s}\n", .{platform.PathOps.extension(test_path)});
    std.debug.print("  Is absolute: {}\n", .{platform.PathOps.isAbsolute(test_path)});
    
    // Test path joining
    const path_parts = [_][]const u8{ "home", "user", "documents", "file.txt" };
    const joined_path = platform.PathOps.join(allocator, &path_parts) catch {
        std.debug.print("  Path join failed\n", .{});
        return;
    };
    defer allocator.free(joined_path);
    std.debug.print("  Joined path: {s}\n", .{joined_path});
    std.debug.print("\n", .{});
    
    // Test process operations (if supported)
    if (platform.Capabilities.hasProcessControl()) {
        std.debug.print("Process Operations:\n", .{});
        
        // Test environment variables
        if (platform.ProcessOps.getEnv(allocator, "PATH")) |path_env| {
            defer allocator.free(path_env);
            std.debug.print("  PATH env var length: {}\n", .{path_env.len});
        } else {
            std.debug.print("  PATH env var not found\n", .{});
        }
        
        // Test current working directory
        if (platform.ProcessOps.getCwd(allocator)) |cwd| {
            defer allocator.free(cwd);
            std.debug.print("  Current directory: {s}\n", .{cwd});
        } else |err| {
            std.debug.print("  Failed to get current directory: {}\n", .{err});
        }
        
        std.debug.print("\n", .{});
    }
    
    // Test file operations (if supported)
    if (platform.Capabilities.hasFileSystem()) {
        std.debug.print("File Operations:\n", .{});
        
        // Test file writing and reading
        const test_file_path = "platform_test_file.txt";
        const test_content = "Hello from CURSED platform abstraction!";
        
        // Write test file
        const write_handle = platform.FileOps.openFile(test_file_path, .write) catch |err| {
            std.debug.print("  Failed to open file for writing: {}\n", .{err});
            return;
        };
        
        const bytes_written = platform.FileOps.writeFile(write_handle, test_content) catch |err| {
            platform.FileOps.closeFile(write_handle);
            std.debug.print("  Failed to write to file: {}\n", .{err});
            return;
        };
        
        platform.FileOps.closeFile(write_handle);
        std.debug.print("  Wrote {} bytes to {s}\n", .{ bytes_written, test_file_path });
        
        // Read test file
        const read_handle = platform.FileOps.openFile(test_file_path, .read) catch |err| {
            std.debug.print("  Failed to open file for reading: {}\n", .{err});
            return;
        };
        
        var read_buffer: [1024]u8 = undefined;
        const bytes_read = platform.FileOps.readFile(read_handle, &read_buffer) catch |err| {
            platform.FileOps.closeFile(read_handle);
            std.debug.print("  Failed to read from file: {}\n", .{err});
            return;
        };
        
        platform.FileOps.closeFile(read_handle);
        std.debug.print("  Read {} bytes from {s}\n", .{ bytes_read, test_file_path });
        std.debug.print("  Content: {s}\n", .{read_buffer[0..bytes_read]});
        
        // Clean up test file
        std.fs.cwd().deleteFile(test_file_path) catch {};
        
        std.debug.print("\n", .{});
    }
    
    // Test networking operations (if supported)
    if (platform.Capabilities.hasNetworking()) {
        std.debug.print("Network Operations:\n", .{});
        
        // Test socket creation
        const socket = platform.NetworkOps.createSocket(.tcp) catch |err| {
            std.debug.print("  Failed to create socket: {}\n", .{err});
            return;
        };
        
        std.debug.print("  TCP socket created successfully\n", .{});
        
        // Test socket binding (use high port to avoid permission issues)
        const test_port: u16 = 0; // Let OS choose port
        platform.NetworkOps.bindSocket(socket, test_port) catch |err| {
            std.debug.print("  Failed to bind socket: {}\n", .{err});
            platform.NetworkOps.closeSocket(socket);
            return;
        };
        
        std.debug.print("  Socket bound successfully\n", .{});
        
        platform.NetworkOps.closeSocket(socket);
        std.debug.print("  Socket closed\n", .{});
        
        std.debug.print("\n", .{});
    }
    
    std.debug.print("Platform Abstraction Test Suite Completed\n", .{});
    std.debug.print("=========================================\n", .{});
}
