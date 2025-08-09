const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Start the LSP server process
    var child = std.process.Child.init(&[_][]const u8{
        "./zig-out/bin/cursed-lsp"
    }, allocator);
    
    child.stdin_behavior = .Pipe;
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Inherit;
    
    try child.spawn();
    defer _ = child.kill() catch {};
    
    const stdin = child.stdin.?.writer();
    const stdout = child.stdout.?.reader();
    
    // Send initialize request
    const initialize_request = 
        \\{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"processId": null, "rootUri": "file:///home/ghuntley/cursed", "capabilities": {"textDocument": {"completion": {"completionItem": {"snippetSupport": true}}, "hover": {"contentFormat": ["plaintext"]}, "definition": {}}}}}
    ;
    
    try stdin.print("Content-Length: {}\r\n\r\n{s}", .{ initialize_request.len, initialize_request });
    
    // Read response
    var buffer: [4096]u8 = undefined;
    const response_len = try stdout.readAll(&buffer);
    std.log.info("Response: {s}", .{buffer[0..response_len]});
    
    // Send initialized notification
    const initialized_notif = 
        \\{"jsonrpc": "2.0", "method": "initialized", "params": {}}
    ;
    
    try stdin.print("Content-Length: {}\r\n\r\n{s}", .{ initialized_notif.len, initialized_notif });
    
    // Send didOpen notification
    const didopen_notif = 
        \\{"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_lsp_integration.csd", "languageId": "cursed", "version": 1, "text": "sus test_var drip = 42\nslay test_func() drip {\n    damn 42\n}"}}}
    ;
    
    try stdin.print("Content-Length: {}\r\n\r\n{s}", .{ didopen_notif.len, didopen_notif });
    
    // Read diagnostic response
    const diag_len = try stdout.readAll(&buffer);
    std.log.info("Diagnostics: {s}", .{buffer[0..diag_len]});
    
    // Send completion request
    const completion_request = 
        \\{"jsonrpc": "2.0", "id": 2, "method": "textDocument/completion", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_lsp_integration.csd"}, "position": {"line": 1, "character": 5}}}
    ;
    
    try stdin.print("Content-Length: {}\r\n\r\n{s}", .{ completion_request.len, completion_request });
    
    // Read completion response
    const comp_len = try stdout.readAll(&buffer);
    std.log.info("Completion: {s}", .{buffer[0..comp_len]});
    
    std.log.info("LSP test completed successfully!", .{});
}
