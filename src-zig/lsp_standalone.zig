//! Standalone CURSED LSP Server - simplified implementation
//! This version avoids dependencies on the complex main_unified.zig

const std = @import("std");
const json = std.json;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Simplified lexer that doesn't depend on full parser
const SimpleLexer = struct {
    input: []const u8,
    position: usize,
    
    pub fn init(input: []const u8) SimpleLexer {
        return SimpleLexer{
            .input = input,
            .position = 0,
        };
    }
    
    pub fn tokenize(self: *SimpleLexer, allocator: Allocator) ![][]const u8 {
        var tokens = ArrayList([]const u8).init(allocator);
        defer tokens.deinit();
        
        var current_pos: usize = 0;
        while (current_pos < self.input.len) {
            // Skip whitespace
            while (current_pos < self.input.len and std.ascii.isWhitespace(self.input[current_pos])) {
                current_pos += 1;
            }
            
            if (current_pos >= self.input.len) break;
            
            const start = current_pos;
            
            // Simple tokenization - just split on whitespace and special chars
            while (current_pos < self.input.len and 
                   !std.ascii.isWhitespace(self.input[current_pos]) and
                   self.input[current_pos] != '(' and
                   self.input[current_pos] != ')' and
                   self.input[current_pos] != '{' and
                   self.input[current_pos] != '}' and
                   self.input[current_pos] != ';') {
                current_pos += 1;
            }
            
            if (current_pos > start) {
                try tokens.append(self.input[start..current_pos]);
            }
            
            // Handle single character tokens
            if (current_pos < self.input.len) {
                const ch = self.input[current_pos];
                if (ch == '(' or ch == ')' or ch == '{' or ch == '}' or ch == ';') {
                    try tokens.append(self.input[current_pos..current_pos + 1]);
                    current_pos += 1;
                }
            }
        }
        
        return tokens.toOwnedSlice();
    }
};

// LSP Data structures
const Position = struct {
    line: u32,
    character: u32,
};

const Range = struct {
    start: Position,
    end: Position,
};

const Location = struct {
    uri: []const u8,
    range: Range,
};

const Diagnostic = struct {
    range: Range,
    severity: u32,
    message: []const u8,
    source: []const u8,
};

const CompletionItem = struct {
    label: []const u8,
    kind: u32,
    detail: ?[]const u8,
};

const SymbolInformation = struct {
    name: []const u8,
    kind: u32,
    location: Location,
};

// Simple CURSED Language Server
const CursedLspServer = struct {
    allocator: Allocator,
    keywords: []const []const u8,
    
    pub fn init(allocator: Allocator) CursedLspServer {
        const keywords = [_][]const u8{
            "sus", "damn", "slay", "vibez", "yeet", "bestie", "stan", "dm",
            "ready", "vibe", "yikes", "shook", "fam", "based", "cap", "cringe",
            "facts", "lit", "tea", "drip", "normie", "smol", "mid", "thicc",
            "snack", "meal", "byte", "rune", "extra", "sip", "squad", "collab", "spill"
        };
        
        return CursedLspServer{
            .allocator = allocator,
            .keywords = &keywords,
        };
    }
    
    pub fn handleMessage(self: *CursedLspServer, input: []const u8, writer: std.io.AnyWriter) !void {
        const parsed = json.parseFromSlice(json.Value, self.allocator, input, .{}) catch |err| {
            std.log.err("JSON parse error: {}", .{err});
            return;
        };
        defer parsed.deinit();
        
        const root = parsed.value;
        
        if (root.object.get("method")) |method_value| {
            const method = method_value.string;
            
            if (std.mem.eql(u8, method, "initialize")) {
                try self.handleInitialize(root, writer);
            } else if (std.mem.eql(u8, method, "initialized")) {
                // No response needed
            } else if (std.mem.eql(u8, method, "textDocument/didOpen")) {
                try self.handleDidOpen(root, writer);
            } else if (std.mem.eql(u8, method, "textDocument/didChange")) {
                try self.handleDidChange(root, writer);
            } else if (std.mem.eql(u8, method, "textDocument/completion")) {
                try self.handleCompletion(root, writer);
            } else if (std.mem.eql(u8, method, "textDocument/hover")) {
                try self.handleHover(root, writer);
            } else if (std.mem.eql(u8, method, "shutdown")) {
                try self.handleShutdown(root, writer);
            } else if (std.mem.eql(u8, method, "exit")) {
                return; // Exit gracefully
            }
        }
    }
    
    fn handleInitialize(self: *CursedLspServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        const response = std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"capabilities": {{"textDocumentSync": 2, "completionProvider": {{"triggerCharacters": ["."]}}, "hoverProvider": true}}}}}}
        , .{id}) catch return;
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
    }
    
    fn handleDidOpen(self: *CursedLspServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;
        const text = text_document.get("text").?.string;
        
        // Simple syntax checking
        var diagnostics = ArrayList(Diagnostic).init(self.allocator);
        defer diagnostics.deinit();
        
        var lexer = SimpleLexer.init(text);
        const tokens = lexer.tokenize(self.allocator) catch {
            try diagnostics.append(Diagnostic{
                .range = Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 10 },
                },
                .severity = 1, // Error
                .message = "Lexer error",
                .source = "cursed-lsp",
            });
            return;
        };
        defer if (tokens.len > 0) self.allocator.free(tokens);
        
        // Send diagnostics
        try self.publishDiagnostics(uri, &diagnostics, writer);
    }
    
    fn handleDidChange(self: *CursedLspServer, request: json.Value, writer: std.io.AnyWriter) !void {
        _ = self;
        _ = request;
        _ = writer;
        // Handle text changes - simplified
    }
    
    fn handleCompletion(self: *CursedLspServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        var completions = ArrayList(CompletionItem).init(self.allocator);
        defer completions.deinit();
        
        // Add CURSED keywords as completions
        for (self.keywords) |keyword| {
            try completions.append(CompletionItem{
                .label = keyword,
                .kind = 14, // Keyword
                .detail = "CURSED keyword",
            });
        }
        
        // Simple response
        const response = std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"isIncomplete": false, "items": []}}}}
        , .{id}) catch return;
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
    }
    
    fn handleHover(self: *CursedLspServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        const response = std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
        , .{id}) catch return;
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
    }
    
    fn handleShutdown(self: *CursedLspServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        const response = std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
        , .{id}) catch return;
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
    }
    
    fn publishDiagnostics(self: *CursedLspServer, uri: []const u8, diagnostics: *ArrayList(Diagnostic), writer: std.io.AnyWriter) !void {
        _ = diagnostics;
        const notification = std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {{"uri": "{s}", "diagnostics": []}}}}
        , .{uri}) catch return;
        defer self.allocator.free(notification);
        
        try writer.print("Content-Length: {}\r\n\r\n{s}", .{ notification.len, notification });
    }
};

pub fn runLspServer(allocator: Allocator) !void {
    var server = CursedLspServer.init(allocator);
    
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();
    const writer = stdout.any();
    
    std.log.info("CURSED LSP Server starting...", .{});
    
    var buffer = ArrayList(u8).init(allocator);
    defer buffer.deinit();
    
    while (true) {
        // Read Content-Length header
        var content_length: usize = 0;
        
        while (true) {
            var line_buffer: [1024]u8 = undefined;
            const line = stdin.readUntilDelimiter(line_buffer[0..], '\n') catch |err| {
                std.log.err("Error reading line: {}", .{err});
                return;
            };
            
            const trimmed = std.mem.trim(u8, line, "\r\n ");
            if (trimmed.len == 0) break; // Empty line marks end of headers
            
            if (std.mem.startsWith(u8, trimmed, "Content-Length: ")) {
                const length_str = trimmed[16..];
                content_length = std.fmt.parseInt(usize, length_str, 10) catch {
                    std.log.err("Invalid Content-Length: {s}", .{length_str});
                    continue;
                };
            }
        }
        
        if (content_length == 0) continue;
        
        // Read message content
        buffer.clearRetainingCapacity();
        try buffer.resize(content_length);
        _ = try stdin.readAll(buffer.items);
        
        // Handle the message
        try server.handleMessage(buffer.items, writer);
    }
}
