const std = @import("std");

/// Comprehensive test client for Enhanced CURSED LSP Server
/// Tests all LSP features: initialization, completion, diagnostics, hover, formatting
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.log.info("Enhanced CURSED LSP Client Test Suite", .{});
    
    // Start the Enhanced LSP server process
    var child = std.process.Child.init(&[_][]const u8{
        "./zig-out/bin/cursed-lsp-enhanced"
    }, allocator);
    
    child.stdin_behavior = .Pipe;
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Inherit;
    
    try child.spawn();
    defer _ = child.kill() catch {};
    defer _ = child.wait() catch {};
    
    const stdin = child.stdin.?.writer();
    const stdout = child.stdout.?.reader();
    
    // Test 1: Initialize request
    std.log.info("Test 1: Initialization", .{});
    const initialize_request = 
        \\{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"processId": null, "rootUri": "file:///home/ghuntley/cursed", "capabilities": {"textDocument": {"completion": {"completionItem": {"snippetSupport": true}}, "hover": {"contentFormat": ["markdown"]}, "definition": {}, "references": {}, "documentSymbol": {}, "formatting": {}, "semanticTokens": {}}, "workspace": {"symbol": {}}}}}
    ;
    
    try sendMessage(stdin, initialize_request);
    const init_response = try readMessage(allocator, stdout);
    defer allocator.free(init_response);
    std.log.info("Initialize response: {s}", .{init_response});
    
    // Test 2: Initialized notification
    std.log.info("Test 2: Initialized notification", .{});
    const initialized_notif = 
        \\{"jsonrpc": "2.0", "method": "initialized", "params": {}}
    ;
    
    try sendMessage(stdin, initialized_notif);
    
    // Test 3: Document open with rich CURSED content
    std.log.info("Test 3: Document didOpen", .{});
    const didopen_notif = 
        \\{"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd", "languageId": "cursed", "version": 1, "text": "yeet \"vibez\"\nyeet \"mathz\"\n\nsus counter drip = 0\nsus message tea = \"Hello Gen Z!\"\nsus active lit = based\n\nslay fibonacci(n drip) drip {\n    ready (n <= 1) {\n        damn n\n    }\n    damn fibonacci(n - 1) + fibonacci(n - 2)\n}\n\nsquad Person {\n    name tea\n    age drip\n    active lit\n}\n\ncollab Drawable {\n    slay draw() normie\n    slay get_area() drip\n}\n\nslay main() normie {\n    vibez.spill(\"Testing CURSED LSP!\")\n    sus result drip = fibonacci(10)\n    vibez.spill(\"Fibonacci result:\", result)\n    \n    sus person Person = Person{\n        name: \"Alice\",\n        age: 25,\n        active: based\n    }\n    \n    bestie (counter < 5) {\n        vibez.spill(\"Counter:\", counter)\n        counter = counter + 1\n    }\n    \n    sick (result) {\n        when 0...10 -> vibez.spill(\"Small number\")\n        when 11...100 -> vibez.spill(\"Medium number\")\n        otherwise -> vibez.spill(\"Large number\")\n    }\n    \n    damn 0\n}"}}}\n
    ;
    
    try sendMessage(stdin, didopen_notif);
    
    // Read potential diagnostics response
    if (readMessageTimeout(allocator, stdout, 1000)) |diag_response| {
        defer allocator.free(diag_response);
        std.log.info("Diagnostics received: {s}", .{diag_response});
    } else |_| {
        std.log.info("No diagnostics received within timeout", .{});
    }
    
    // Test 4: Code completion at various positions
    std.log.info("Test 4: Code completion", .{});
    
    // Test completion for keywords at start of line
    const completion_request_1 = 
        \\{"jsonrpc": "2.0", "id": 2, "method": "textDocument/completion", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 40, "character": 0}}}
    ;
    
    try sendMessage(stdin, completion_request_1);
    const comp_response_1 = try readMessage(allocator, stdout);
    defer allocator.free(comp_response_1);
    std.log.info("Keyword completion: {s}", .{comp_response_1});
    
    // Test completion for stdlib functions
    const completion_request_2 = 
        \\{"jsonrpc": "2.0", "id": 3, "method": "textDocument/completion", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 30, "character": 8}}}
    ;
    
    try sendMessage(stdin, completion_request_2);
    const comp_response_2 = try readMessage(allocator, stdout);
    defer allocator.free(comp_response_2);
    std.log.info("Stdlib completion: {s}", .{comp_response_2});
    
    // Test 5: Hover information
    std.log.info("Test 5: Hover information", .{});
    
    // Hover over function name
    const hover_request_1 = 
        \\{"jsonrpc": "2.0", "id": 4, "method": "textDocument/hover", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 7, "character": 5}}}
    ;
    
    try sendMessage(stdin, hover_request_1);
    const hover_response_1 = try readMessage(allocator, stdout);
    defer allocator.free(hover_response_1);
    std.log.info("Hover function: {s}", .{hover_response_1});
    
    // Hover over keyword
    const hover_request_2 = 
        \\{"jsonrpc": "2.0", "id": 5, "method": "textDocument/hover", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 3, "character": 1}}}
    ;
    
    try sendMessage(stdin, hover_request_2);
    const hover_response_2 = try readMessage(allocator, stdout);
    defer allocator.free(hover_response_2);
    std.log.info("Hover keyword: {s}", .{hover_response_2});
    
    // Test 6: Go to definition
    std.log.info("Test 6: Go to definition", .{});
    const definition_request = 
        \\{"jsonrpc": "2.0", "id": 6, "method": "textDocument/definition", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 26, "character": 20}}}
    ;
    
    try sendMessage(stdin, definition_request);
    const def_response = try readMessage(allocator, stdout);
    defer allocator.free(def_response);
    std.log.info("Definition response: {s}", .{def_response});
    
    // Test 7: Document symbols
    std.log.info("Test 7: Document symbols", .{});
    const symbol_request = 
        \\{"jsonrpc": "2.0", "id": 7, "method": "textDocument/documentSymbol", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}}}
    ;
    
    try sendMessage(stdin, symbol_request);
    const symbol_response = try readMessage(allocator, stdout);
    defer allocator.free(symbol_response);
    std.log.info("Document symbols: {s}", .{symbol_response});
    
    // Test 8: Workspace symbols
    std.log.info("Test 8: Workspace symbols", .{});
    const workspace_symbol_request = 
        \\{"jsonrpc": "2.0", "id": 8, "method": "workspace/symbol", "params": {"query": "fibonacci"}}
    ;
    
    try sendMessage(stdin, workspace_symbol_request);
    const workspace_symbol_response = try readMessage(allocator, stdout);
    defer allocator.free(workspace_symbol_response);
    std.log.info("Workspace symbols: {s}", .{workspace_symbol_response});
    
    // Test 9: Document formatting
    std.log.info("Test 9: Document formatting", .{});
    const format_request = 
        \\{"jsonrpc": "2.0", "id": 9, "method": "textDocument/formatting", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "options": {"tabSize": 4, "insertSpaces": true}}}
    ;
    
    try sendMessage(stdin, format_request);
    const format_response = try readMessage(allocator, stdout);
    defer allocator.free(format_response);
    std.log.info("Format response: {s}", .{format_response});
    
    // Test 10: Semantic tokens
    std.log.info("Test 10: Semantic tokens", .{});
    const semantic_tokens_request = 
        \\{"jsonrpc": "2.0", "id": 10, "method": "textDocument/semanticTokens/full", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}}}
    ;
    
    try sendMessage(stdin, semantic_tokens_request);
    const semantic_response = try readMessage(allocator, stdout);
    defer allocator.free(semantic_response);
    std.log.info("Semantic tokens: {s}", .{semantic_response});
    
    // Test 11: Document changes
    std.log.info("Test 11: Document changes", .{});
    const didchange_notif = 
        \\{"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd", "version": 2}, "contentChanges": [{"text": "yeet \"vibez\"\n\nsus updated_variable drip = 42\nvibez.spill(\"Updated content!\")\n\nslay new_function() normie {\n    damn 0\n}"}]}}
    ;
    
    try sendMessage(stdin, didchange_notif);
    
    // Read potential new diagnostics
    if (readMessageTimeout(allocator, stdout, 1000)) |change_diag_response| {
        defer allocator.free(change_diag_response);
        std.log.info("Change diagnostics: {s}", .{change_diag_response});
    } else |_| {
        std.log.info("No change diagnostics received", .{});
    }
    
    // Test completion after changes
    const completion_after_change = 
        \\{"jsonrpc": "2.0", "id": 11, "method": "textDocument/completion", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 3, "character": 0}}}
    ;
    
    try sendMessage(stdin, completion_after_change);
    const comp_change_response = try readMessage(allocator, stdout);
    defer allocator.free(comp_change_response);
    std.log.info("Completion after change: {s}", .{comp_change_response});
    
    // Test 12: References
    std.log.info("Test 12: Find references", .{});
    const references_request = 
        \\{"jsonrpc": "2.0", "id": 12, "method": "textDocument/references", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}, "position": {"line": 3, "character": 5}, "context": {"includeDeclaration": true}}}
    ;
    
    try sendMessage(stdin, references_request);
    const references_response = try readMessage(allocator, stdout);
    defer allocator.free(references_response);
    std.log.info("References response: {s}", .{references_response});
    
    // Test 13: Document close
    std.log.info("Test 13: Document close", .{});
    const didclose_notif = 
        \\{"jsonrpc": "2.0", "method": "textDocument/didClose", "params": {"textDocument": {"uri": "file:///home/ghuntley/cursed/test_enhanced.csd"}}}
    ;
    
    try sendMessage(stdin, didclose_notif);
    
    // Test 14: Shutdown sequence
    std.log.info("Test 14: Shutdown", .{});
    const shutdown_request = 
        \\{"jsonrpc": "2.0", "id": 13, "method": "shutdown", "params": null}
    ;
    
    try sendMessage(stdin, shutdown_request);
    const shutdown_response = try readMessage(allocator, stdout);
    defer allocator.free(shutdown_response);
    std.log.info("Shutdown response: {s}", .{shutdown_response});
    
    // Final exit
    const exit_notif = 
        \\{"jsonrpc": "2.0", "method": "exit", "params": null}
    ;
    
    try sendMessage(stdin, exit_notif);
    
    std.log.info("🎉 Enhanced CURSED LSP Client Test Suite completed successfully!", .{});
}

/// Send LSP message with proper headers
fn sendMessage(writer: std.fs.File.Writer, content: []const u8) !void {
    try writer.print("Content-Length: {}\r\n\r\n{s}", .{ content.len, content });
}

/// Read LSP message with headers
fn readMessage(allocator: std.mem.Allocator, reader: std.fs.File.Reader) ![]u8 {
    // Read Content-Length header
    var content_length: usize = 0;
    while (true) {
        const line = try reader.readUntilDelimiterAlloc(allocator, '\n', 1024);
        defer allocator.free(line);
        
        const trimmed = std.mem.trim(u8, line, "\r\n");
        if (trimmed.len == 0) break; // Empty line marks end of headers
        
        if (std.mem.startsWith(u8, trimmed, "Content-Length: ")) {
            const length_str = trimmed[16..];
            content_length = try std.fmt.parseInt(usize, length_str, 10);
        }
    }

    if (content_length == 0) return error.NoContentLength;

    // Read message content
    const content = try allocator.alloc(u8, content_length);
    _ = try reader.readAll(content);
    
    return content;
}

/// Read LSP message with timeout
fn readMessageTimeout(allocator: std.mem.Allocator, reader: std.fs.File.Reader, timeout_ms: u64) ![]u8 {
    _ = timeout_ms; // TODO: Implement actual timeout
    return readMessage(allocator, reader);
}
