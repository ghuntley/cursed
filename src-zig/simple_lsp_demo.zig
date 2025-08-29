//! Simple CURSED LSP Server Demo
//! Demonstrates basic LSP functionality with CURSED language support

const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const stdin = std.fs.File.stdin();
    const stdout = std.fs.File.stdout();
    
    std.log.info("🚀 CURSED LSP Server Demo starting...", .{});
    
    // Simple LSP server loop
    var buffer: std.ArrayList(u8) = .empty;
    defer buffer.deinit();
    
    while (true) {
        // Read Content-Length header
        var content_length: usize = 0;
        while (true) {
            var line_buf: [1024]u8 = undefined;
            const line = stdin.reader().readUntilDelimiterOrEof(line_buf[0..], '\n') catch break orelse break;
            
            const trimmed = std.mem.trim(u8, line, "\r\n");
            if (trimmed.len == 0) break; // Empty line marks end of headers
            
            if (std.mem.startsWith(u8, trimmed, "Content-Length: ")) {
                const length_str = trimmed[16..];
                content_length = std.fmt.parseInt(usize, length_str, 10) catch 0;
            }
        }
        
        if (content_length == 0) continue;
        
        // Read message content
        buffer.clearRetainingCapacity();
        try buffer.resize(allocator, content_length);
        _ = try stdin.reader().readAll(buffer.items);
        
        // Parse and handle the request
        try handleLspRequest(allocator, buffer.items, stdout.writer());
    }
}

/// Handle LSP request with basic CURSED language support
fn handleLspRequest(allocator: std.mem.Allocator, input: []const u8, writer: std.fs.File.Writer) !void {
    const parsed = std.json.parseFromSlice(std.json.Value, allocator, input, .{}) catch |err| {
        std.log.err("JSON parse error: {}", .{err});
        return;
    };
    defer parsed.deinit();
    
    const root = parsed.value;
    
    if (root.object.get("method")) |method| {
        const method_str = method.string;
        
        if (std.mem.eql(u8, method_str, "initialize")) {
            try handleInitialize(allocator, root, writer);
        } else if (std.mem.eql(u8, method_str, "initialized")) {
            std.log.info("LSP Client initialized", .{});
        } else if (std.mem.eql(u8, method_str, "textDocument/completion")) {
            try handleCompletion(allocator, root, writer);
        } else if (std.mem.eql(u8, method_str, "textDocument/hover")) {
            try handleHover(allocator, root, writer);
        } else if (std.mem.eql(u8, method_str, "shutdown")) {
            try handleShutdown(allocator, root, writer);
        } else if (std.mem.eql(u8, method_str, "exit")) {
            return;
        }
    }
}

/// Handle initialize request
fn handleInitialize(allocator: std.mem.Allocator, request: std.json.Value, writer: std.fs.File.Writer) !void {
    const id = request.object.get("id").?;
    
    const capabilities_response = 
        \\{
        \\  "jsonrpc": "2.0",
        \\  "id": %d,
        \\  "result": {
        \\    "capabilities": {
        \\      "textDocumentSync": {"openClose": true, "change": 2},
        \\      "completionProvider": {"triggerCharacters": ["."], "resolveProvider": false},
        \\      "hoverProvider": true,
        \\      "definitionProvider": true,
        \\      "documentFormattingProvider": true
        \\    },
        \\    "serverInfo": {"name": "cursed-lsp-demo", "version": "1.0.0"}
        \\  }
        \\}
    ;
    
    const response = try std.fmt.allocPrint(allocator, capabilities_response, .{id.integer});
    defer allocator.free(response);
    
    try sendResponse(writer, response);
}

/// Handle completion request with CURSED keywords and stdlib
fn handleCompletion(allocator: std.mem.Allocator, request: std.json.Value, writer: std.fs.File.Writer) !void {
    const id = request.object.get("id").?;
    
    // CURSED completion items
    const completions = 
        \\{
        \\  "jsonrpc": "2.0", 
        \\  "id": %d,
        \\  "result": {
        \\    "isIncomplete": false,
        \\    "items": [
        \\      {"label": "sus", "kind": 14, "detail": "CURSED keyword", "documentation": "Variable declaration: sus name type = value"},
        \\      {"label": "slay", "kind": 14, "detail": "CURSED keyword", "documentation": "Function definition: slay name(params) { body }"},
        \\      {"label": "damn", "kind": 14, "detail": "CURSED keyword", "documentation": "Return statement: damn value"},
        \\      {"label": "vibez", "kind": 9, "detail": "CURSED stdlib", "documentation": "I/O operations module"},
        \\      {"label": "mathz", "kind": 9, "detail": "CURSED stdlib", "documentation": "Mathematical functions module"},
        \\      {"label": "stringz", "kind": 9, "detail": "CURSED stdlib", "documentation": "String manipulation module"},
        \\      {"label": "ready", "kind": 14, "detail": "CURSED keyword", "documentation": "If statement: ready (condition) { body }"},
        \\      {"label": "bestie", "kind": 14, "detail": "CURSED keyword", "documentation": "While loop: bestie (condition) { body }"},
        \\      {"label": "squad", "kind": 14, "detail": "CURSED keyword", "documentation": "Struct definition: squad Name { fields }"},
        \\      {"label": "based", "kind": 14, "detail": "CURSED keyword", "documentation": "Boolean true value"},
        \\      {"label": "cringe", "kind": 14, "detail": "CURSED keyword", "documentation": "Boolean false value"},
        \\      {"label": "drip", "kind": 14, "detail": "CURSED type", "documentation": "Integer type (64-bit signed)"},
        \\      {"label": "tea", "kind": 14, "detail": "CURSED type", "documentation": "String type (UTF-8)"},
        \\      {"label": "lit", "kind": 14, "detail": "CURSED type", "documentation": "Boolean type"}
        \\    ]
        \\  }
        \\}
    ;
    
    const response = try std.fmt.allocPrint(allocator, completions, .{id.integer});
    defer allocator.free(response);
    
    try sendResponse(writer, response);
}

/// Handle hover request with CURSED documentation
fn handleHover(allocator: std.mem.Allocator, request: std.json.Value, writer: std.fs.File.Writer) !void {
    const id = request.object.get("id").?;
    
    const hover_response = 
        \\{
        \\  "jsonrpc": "2.0",
        \\  "id": %d,
        \\  "result": {
        \\    "contents": "**CURSED Language Feature**\n\nThis is a Gen-Z slang programming language with full type inference and modern features.\n\n- `sus` - Variable declaration\n- `slay` - Function definition\n- `vibez.spill()` - Print to output\n- `based`/`cringe` - Boolean values"
        \\  }
        \\}
    ;
    
    const response = try std.fmt.allocPrint(allocator, hover_response, .{id.integer});
    defer allocator.free(response);
    
    try sendResponse(writer, response);
}

/// Handle shutdown request
fn handleShutdown(allocator: std.mem.Allocator, request: std.json.Value, writer: std.fs.File.Writer) !void {
    const id = request.object.get("id").?;
    
    const shutdown_response = 
        \\{"jsonrpc": "2.0", "id": %d, "result": null}
    ;
    
    const response = try std.fmt.allocPrint(allocator, shutdown_response, .{id.integer});
    defer allocator.free(response);
    
    try sendResponse(writer, response);
}

/// Send LSP response with proper headers
fn sendResponse(writer: std.fs.File.Writer, response: []const u8) !void {
    try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
    try writer.writer().writeAll(response);
}
