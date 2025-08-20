const std = @import("std");

/// Basic test for Enhanced CURSED LSP Server functionality
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.log.info("Basic CURSED LSP Test", .{});
    
    // Test LSP server executable exists
    const lsp_exe_path = "./zig-out/bin/cursed-lsp-enhanced";
    std.fs.cwd().access(lsp_exe_path, .{}) catch |err| {
        std.log.err("Enhanced LSP executable not found: {}", .{err});
        return;
    };
    std.log.info("✅ Enhanced LSP executable found", .{});
    
    // Start the Enhanced LSP server process with a timeout
    var child = std.process.Child.init(&[_][]const u8{lsp_exe_path}, allocator);
    child.stdin_behavior = .Pipe;
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;
    
    try child.spawn();
    defer _ = child.kill() catch {};
    
    // Give it a moment to start
    std.time.sleep(500 * std.time.ns_per_ms);
    
    const stdin = child.stdin.?.writer();
    const stdout = child.stdout.?.reader();
    
    // Test basic initialization
    std.log.info("Testing initialization...", .{});
    const init_msg = 
        \\{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":"file:///home/ghuntley/cursed","capabilities":{}}}
    ;
    
    // Send initialization
    try stdin.print("Content-Length: {}\r\n\r\n{s}", .{ init_msg.len, init_msg });
    
    // Try to read response with timeout
    var response_buffer: [8192]u8 = undefined;
    
    // Read headers first
    var line_buffer: [256]u8 = undefined;
    var content_length: usize = 0;
    
    // Simple timeout mechanism
    const start_time = std.time.milliTimestamp();
    const timeout_ms = 5000; // 5 seconds
    
    while (std.time.milliTimestamp() - start_time < timeout_ms) {
        if (stdout.readUntilDelimiterOrEof(line_buffer[0..], '\n')) |maybe_line| {
            if (maybe_line) |line| {
                const trimmed = std.mem.trim(u8, line, "\r\n ");
                if (trimmed.len == 0) {
                    // End of headers
                    break;
                }
                if (std.mem.startsWith(u8, trimmed, "Content-Length:")) {
                    const length_part = std.mem.trim(u8, trimmed[15..], " ");
                    content_length = std.fmt.parseInt(usize, length_part, 10) catch 0;
                }
            } else break;
        } else |_| {
            // No data available yet, sleep briefly
            std.time.sleep(10 * std.time.ns_per_ms);
            continue;
        }
    }
    
    if (content_length > 0 and content_length < response_buffer.len) {
        // Read the actual content
        if (stdout.readAtLeast(response_buffer[0..content_length], content_length)) |bytes_read| {
            if (bytes_read == content_length) {
                const response = response_buffer[0..content_length];
                std.log.info("✅ Received initialization response: {s}", .{response});
                
                // Check if response contains expected LSP capabilities
                if (std.mem.indexOf(u8, response, "capabilities") != null) {
                    std.log.info("✅ LSP capabilities found in response", .{});
                } else {
                    std.log.warn("⚠️  No capabilities found in response", .{});
                }
                
                if (std.mem.indexOf(u8, response, "completionProvider") != null) {
                    std.log.info("✅ Code completion capability advertised", .{});
                } else {
                    std.log.warn("⚠️  No completion capability found", .{});
                }
                
                if (std.mem.indexOf(u8, response, "hoverProvider") != null) {
                    std.log.info("✅ Hover capability advertised", .{});
                } else {
                    std.log.warn("⚠️  No hover capability found", .{});
                }
                
            } else {
                std.log.warn("⚠️  Incomplete response read", .{});
            }
        } else |err| {
            std.log.err("❌ Failed to read response content: {}", .{err});
        }
    } else {
        std.log.err("❌ No valid response received within timeout", .{});
        
        // Try to read any stderr output for debugging
        var stderr_buffer: [1024]u8 = undefined;
        if (child.stderr.?.readAll(&stderr_buffer)) |stderr_bytes| {
            if (stderr_bytes > 0) {
                std.log.err("Stderr output: {s}", .{stderr_buffer[0..stderr_bytes]});
            }
        } else |_| {}
    }
    
    // Send shutdown
    std.log.info("Sending shutdown...", .{});
    const shutdown_msg = 
        \\{"jsonrpc":"2.0","id":2,"method":"shutdown","params":null}
    ;
    
    stdin.print("Content-Length: {}\r\n\r\n{s}", .{ shutdown_msg.len, shutdown_msg }) catch {};
    
    // Send exit
    const exit_msg = 
        \\{"jsonrpc":"2.0","method":"exit","params":null}
    ;
    
    stdin.print("Content-Length: {}\r\n\r\n{s}", .{ exit_msg.len, exit_msg }) catch {};
    
    // Give it time to shutdown
    std.time.sleep(100 * std.time.ns_per_ms);
    
    std.log.info("🎯 Basic LSP test completed", .{});
}
