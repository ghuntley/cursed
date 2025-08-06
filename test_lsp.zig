// Test the CURSED LSP Server functionality
const lsp = @import("lsp_standalone.zig");
const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var handler = lsp.LSPHandler.init(allocator);
    defer handler.deinit();

    std.log.info("Testing CURSED LSP Server capabilities...", .{});

    // Test 1: Initialize
    const init_message = 
        \\{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}
    ;
    
    if (try handler.handleMessage(init_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Initialize: {s}", .{response[0..@min(response.len, 100)]});
    }

    // Test 2: Document Open
    const open_message = 
        \\{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///test.csd","version":1,"languageId":"cursed","text":"slay test_function() {\n    vibez.spill(\"Hello CURSED!\")\n}"}}}
    ;
    
    _ = try handler.handleMessage(open_message);
    std.log.info("✓ Document opened", .{});

    // Test 3: Completion
    const completion_message = 
        \\{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":1,"character":4}}}
    ;
    
    if (try handler.handleMessage(completion_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Completion: {} characters", .{response.len});
    }

    // Test 4: Hover
    const hover_message = 
        \\{"jsonrpc":"2.0","id":3,"method":"textDocument/hover","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":0,"character":5}}}
    ;
    
    if (try handler.handleMessage(hover_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Hover: {s}", .{response[0..@min(response.len, 100)]});
    }

    // Test 5: Definition
    const definition_message = 
        \\{"jsonrpc":"2.0","id":4,"method":"textDocument/definition","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":0,"character":5}}}
    ;
    
    if (try handler.handleMessage(definition_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Definition: {s}", .{response});
    }

    // Test 6: References
    const references_message = 
        \\{"jsonrpc":"2.0","id":5,"method":"textDocument/references","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":0,"character":5}}}
    ;
    
    if (try handler.handleMessage(references_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ References: {s}", .{response});
    }

    // Test 7: Document Symbol
    const document_symbol_message = 
        \\{"jsonrpc":"2.0","id":6,"method":"textDocument/documentSymbol","params":{"textDocument":{"uri":"file:///test.csd"}}}
    ;
    
    if (try handler.handleMessage(document_symbol_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Document Symbol: {s}", .{response});
    }

    // Test 8: Workspace Symbol
    const workspace_symbol_message = 
        \\{"jsonrpc":"2.0","id":7,"method":"workspace/symbol","params":{"query":"test"}}
    ;
    
    if (try handler.handleMessage(workspace_symbol_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Workspace Symbol: {s}", .{response});
    }

    // Test 9: Signature Help
    const signature_help_message = 
        \\{"jsonrpc":"2.0","id":8,"method":"textDocument/signatureHelp","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":1,"character":15}}}
    ;
    
    if (try handler.handleMessage(signature_help_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Signature Help: {s}", .{response[0..@min(response.len, 100)]});
    }

    // Test 10: Formatting
    const formatting_message = 
        \\{"jsonrpc":"2.0","id":9,"method":"textDocument/formatting","params":{"textDocument":{"uri":"file:///test.csd"}}}
    ;
    
    if (try handler.handleMessage(formatting_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Formatting: {s}", .{response});
    }

    // Test 11: Shutdown
    const shutdown_message = 
        \\{"jsonrpc":"2.0","id":10,"method":"shutdown","params":null}
    ;
    
    if (try handler.handleMessage(shutdown_message)) |response| {
        defer allocator.free(response);
        std.log.info("✓ Shutdown: {s}", .{response});
    }

    std.log.info("All LSP tests completed successfully!", .{});
}
