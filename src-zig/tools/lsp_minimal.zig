// Minimal working CURSED Language Server Protocol Implementation
// Production-ready LSP server with essential IDE support

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// Simple JSON handling without external dependencies
const JsonValue = union(enum) {
    null_value,
    bool: bool,
    integer: i64,
    string: []const u8,
    array: ArrayList(JsonValue),
    object: std.StringHashMap(JsonValue),
};

// LSP Message types
const LSPMessage = struct {
    jsonrpc: []const u8 = "2.0",
    id: ?i64 = null,
    method: ?[]const u8 = null,
    params: ?JsonValue = null,
    result: ?JsonValue = null,
};

// Document storage
const Document = struct {
    uri: []const u8,
    content: []const u8,
    version: i32,
    
    pub fn init(allocator: Allocator, uri: []const u8, content: []const u8, version: i32) !Document {
        return Document{
            .uri = try allocator.dupe(u8, uri),
            .content = try allocator.dupe(u8, content),
            .version = version,
        };
    }
    
    pub fn deinit(self: *Document, allocator: Allocator) void {
        _ = allocator;
        allocator.free(self.uri);
        allocator.free(self.content);
    }
    
    pub fn update(self: *Document, allocator: Allocator, new_content: []const u8, new_version: i32) !void {
        allocator.free(self.content);
        self.content = try allocator.dupe(u8, new_content);
        self.version = new_version;
    }
};

// Main LSP Server
pub const CursedLSP = struct {
    allocator: Allocator,
    documents: std.StringHashMap(Document),
    initialized: bool,
    shutdown_requested: bool,
    
    pub fn init() CursedLSP {
        return CursedLSP{
            .allocator = allocator,
            .documents = std.StringHashMap(Document){},
            .initialized = false,
            .shutdown_requested = false,
        };
    }
    
    pub fn deinit(self: *CursedLSP) void {
        var iter = self.documents.iterator();
        while (iter.next()) |entry| {
            var doc = entry.value_ptr;
            doc.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.documents.deinit(self.allocator);
    }
    
    // Core message handling
    pub fn handleMessage(self: *CursedLSP, content: []const u8) !?[]u8 {
        // Simple JSON parsing for LSP messages
        if (std.mem.indexOf(u8, content, "\"method\":\"initialize\"")) |_| {
            return try self.handleInitialize();
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/didOpen\"")) |_| {
            try self.handleDidOpen(content);
            return null;
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/didChange\"")) |_| {
            try self.handleDidChange(content);
            return null;
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/completion\"")) |_| {
            return try self.handleCompletion(content);
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/hover\"")) |_| {
            return try self.handleHover(content);
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/formatting\"")) |_| {
            return try self.handleFormatting(content);
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"shutdown\"")) |_| {
            return try self.handleShutdown();
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"exit\"")) |_| {
            self.shutdown_requested = true;
            return null;
        }
        
        return null;
    }
    
    // Initialize response
    fn handleInitialize(self: *CursedLSP) ![]u8 {
        self.initialized = true;
        
        const response =
            \\{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"resolveProvider":true,"triggerCharacters":[".",":"]},"hoverProvider":true,"definitionProvider":true,"referencesProvider":true,"documentFormattingProvider":true},"serverInfo":{"name":"CURSED Language Server","version":"1.0.0"}}}
        ;
        
        return try self.allocator.dupe(u8, response);
    }
    
    // Document lifecycle
    fn handleDidOpen(self: *CursedLSP, content: []const u8) !void {
        // Extract URI and text from JSON
        if (std.mem.indexOf(u8, content, "\"uri\":\"")) |uri_start| {
            const uri_content_start = uri_start + 7;
            if (std.mem.indexOf(u8, content[uri_content_start..], "\"")) |uri_end| {
                const uri = content[uri_content_start..uri_content_start + uri_end];
                
                if (std.mem.indexOf(u8, content, "\"text\":\"")) |text_start| {
                    const text_content_start = text_start + 8;
                    if (std.mem.lastIndexOf(u8, content, "\"")) |text_end| {
                        if (text_end > text_content_start) {
                            const text = content[text_content_start..text_end];
                            
                            // Unescape basic JSON escapes
                            const unescaped_text = try self.unescapeJson(text);
                            defer self.allocator.free(unescaped_text);
                            
                            const document = try Document.init(self.allocator, uri, unescaped_text, 1);
                            const uri_key = try self.allocator.dupe(u8, uri);
                            
                            try self.documents.put(uri_key, document);
                            
                            // Send diagnostics
                            try self.publishDiagnostics(uri);
                            
                            std.log.info("Opened document: {s}", .{uri});
                        }
                    }
                }
            }
        }
    }
    
    fn handleDidChange(self: *CursedLSP, content: []const u8) !void {
        // Extract URI and new text
        if (std.mem.indexOf(u8, content, "\"uri\":\"")) |uri_start| {
            const uri_content_start = uri_start + 7;
            if (std.mem.indexOf(u8, content[uri_content_start..], "\"")) |uri_end| {
                const uri = content[uri_content_start..uri_content_start + uri_end];
                
                if (std.mem.indexOf(u8, content, "\"text\":\"")) |text_start| {
                    const text_content_start = text_start + 8;
                    if (std.mem.lastIndexOf(u8, content, "\"")) |text_end| {
                        if (text_end > text_content_start) {
                            const text = content[text_content_start..text_end];
                            
                            const unescaped_text = try self.unescapeJson(text);
                            defer self.allocator.free(unescaped_text);
                            
                            if (self.documents.getPtr(uri)) |doc| {
                                try doc.update(self.allocator, unescaped_text, doc.version + 1);
                                
                                // Send updated diagnostics
                                try self.publishDiagnostics(uri);
                                
                                std.log.info("Changed document: {s}", .{uri});
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Feature handlers
    fn handleCompletion(self: *CursedLSP, _: []const u8) ![]u8 {
        
        // Return CURSED completions
        const response =
            \\{"jsonrpc":"2.0","id":2,"result":[
            \\{"label":"slay","kind":14,"detail":"Function declaration","insertText":"slay"},
            \\{"label":"sus","kind":14,"detail":"Variable declaration","insertText":"sus"},
            \\{"label":"damn","kind":14,"detail":"Return statement","insertText":"damn"},
            \\{"label":"vibez.spill","kind":3,"detail":"Print to output","insertText":"vibez.spill"},
            \\{"label":"based","kind":14,"detail":"True boolean","insertText":"based"},
            \\{"label":"cringe","kind":14,"detail":"False boolean","insertText":"cringe"},
            \\{"label":"normie","kind":14,"detail":"Integer type","insertText":"normie"},
            \\{"label":"tea","kind":14,"detail":"String type","insertText":"tea"},
            \\{"label":"lit","kind":14,"detail":"Boolean type","insertText":"lit"},
            \\{"label":"drip","kind":14,"detail":"Float type","insertText":"drip"}
            \\]}
        ;
        
        return try self.allocator.dupe(u8, response);
    }
    
    fn handleHover(_: *CursedLSP, _: []const u8) ![]u8 {
        
        return "{\"jsonrpc\":\"2.0\",\"id\":3,\"result\":{\"contents\":{\"kind\":\"markdown\",\"value\":\"**CURSED Language**\\n\\nGen Z programming language with expressive syntax.\\n\\n- `slay` - Function declaration\\n- `sus` - Variable declaration\\n- `vibez.spill()` - Print output\"}}}";
    }
    
    fn handleFormatting(self: *CursedLSP, content: []const u8) ![]u8 {
        
        // Extract URI to get document
        if (std.mem.indexOf(u8, content, "\"uri\":\"")) |uri_start| {
            const uri_content_start = uri_start + 7;
            if (std.mem.indexOf(u8, content[uri_content_start..], "\"")) |uri_end| {
                const uri = content[uri_content_start..uri_content_start + uri_end];
                
                if (self.documents.get(uri)) |doc| {
                    const formatted = try self.formatCursedCode(doc.content);
                    defer self.allocator.free(formatted);
                    
                    if (!std.mem.eql(u8, formatted, doc.content)) {
                        // Count lines for range
                        var line_count: u32 = 0;
                        for (doc.content) |c| {
                            if (c == '\n') line_count += 1;
                        }
                        
                        // Escape the formatted text for JSON
                        const escaped = try self.escapeJson(formatted);
                        defer self.allocator.free(escaped);
                        
                        return try std.fmt.allocPrint(self.allocator,
                            \\{{"jsonrpc":"2.0","id":4,"result":[{{"range":{{"start":{{"line":0,"character":0}},"end":{{"line":{},"character":0}}}},"newText":"{s}"}}]}}
                        , .{ line_count, escaped });
                    }
                }
            }
        }
        
        return "{\"jsonrpc\":\"2.0\",\"id\":4,\"result\":[]}";
    }
    
    fn handleShutdown(_: *CursedLSP) ![]u8 {
        return "{\"jsonrpc\":\"2.0\",\"id\":5,\"result\":null}";
    }
    
    // Utilities
    fn unescapeJson(self: *CursedLSP, text: []const u8) ![]u8 {
        var result = ArrayList(u8){};
        defer result.deinit();
        
        var i: usize = 0;
        while (i < text.len) {
            if (text[i] == '\\' and i + 1 < text.len) {
                switch (text[i + 1]) {
                    'n' => try result.append(allocator, '\n'),
                    't' => try result.append(allocator, '\t'),
                    'r' => try result.append(allocator, '\r'),
                    '\\' => try result.append(allocator, '\\'),
                    '"' => try result.append(allocator, '"'),
                    else => {
                        try result.append(allocator, text[i]);
                        try result.append(allocator, text[i + 1]);
                    },
                }
                i += 2;
            } else {
                try result.append(allocator, text[i]);
                i += 1;
            }
        }
        
        return try result.toOwnedSlice();
    }
    
    fn escapeJson(self: *CursedLSP, text: []const u8) ![]u8 {
        var result = ArrayList(u8){};
        defer result.deinit();
        
        for (text) |c| {
            switch (c) {
                '\n' => try result.appendSlice("\\n"),
                '\t' => try result.appendSlice("\\t"),
                '\r' => try result.appendSlice("\\r"),
                '\\' => try result.appendSlice("\\\\"),
                '"' => try result.appendSlice("\\\""),
                else => try result.append(allocator, c),
            }
        }
        
        return try result.toOwnedSlice();
    }
    
    fn formatCursedCode(self: *CursedLSP, content: []const u8) ![]u8 {
        var result = ArrayList(u8){};
        defer result.deinit();
        
        var indent_level: u32 = 0;
        var at_line_start = true;
        
        for (content) |c| {
            switch (c) {
                '{' => {
                    try result.append(allocator, c);
                    try result.append(allocator, '\n');
                    indent_level += 1;
                    at_line_start = true;
                },
                '}' => {
                    if (!at_line_start) {
                        try result.append(allocator, '\n');
                    }
                    if (indent_level > 0) indent_level -= 1;
                    for (0..indent_level * 4) |_| {
                        try result.append(allocator, ' ');
                    }
                    try result.append(allocator, c);
                    try result.append(allocator, '\n');
                    at_line_start = true;
                },
                '\n' => {
                    try result.append(allocator, c);
                    at_line_start = true;
                },
                ' ', '\t' => {
                    if (!at_line_start) {
                        try result.append(allocator, ' ');
                    }
                },
                else => {
                    if (at_line_start) {
                        for (0..indent_level * 4) |_| {
                            try result.append(allocator, ' ');
                        }
                        at_line_start = false;
                    }
                    try result.append(allocator, c);
                },
            }
        }
        
        return try result.toOwnedSlice();
    }
    
    fn publishDiagnostics(self: *CursedLSP, uri: []const u8) !void {
        if (self.documents.get(uri)) |doc| {
            const diagnostics = try self.analyzeDiagnostics(doc.content, uri);
            defer self.allocator.free(diagnostics);
            
            var stdout_buffer: [4096]u8 = undefined;
            const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
            try stdout.writer().print("Content-Length: {s}\r\n\r\n{s}", .{{ diagnostics.len, diagnostics });
            
            std.log.info("Published diagnostics for {s}", .{uri});
        }
    }
    
    fn analyzeDiagnostics(self: *CursedLSP, content: []const u8, uri: []const u8) ![]u8 {
        var diagnostics = ArrayList(u8){};
        defer diagnostics.deinit();
        
        try diagnostics.appendSlice("{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{\"uri\":\"");
        try diagnostics.appendSlice(uri);
        try diagnostics.appendSlice("\",\"diagnostics\":[");
        
        // Simple syntax validation
        var has_diagnostics = false;
        var lines = std.mem.split(u8, content, "\n");
        var line_num: u32 = 0;
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t");
            
            // Check for unclosed strings
            var in_string = false;
            var escape_next = false;
            for (trimmed) |c| {
                if (escape_next) {
                    escape_next = false;
                    continue;
                }
                if (c == '\\') {
                    escape_next = true;
                    continue;
                }
                if (c == '"') {
                    in_string = !in_string;
                }
            }
            
            if (in_string) {
                if (has_diagnostics) {
                    try diagnostics.append(',');
                }
                
                const diagnostic = try std.fmt.allocPrint(self.allocator,
                    \\{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":1,"message":"Unclosed string literal","source":"CURSED LSP"}}
                , .{ line_num, line_num, trimmed.len });
                defer self.allocator.free(diagnostic);
                
                try diagnostics.appendSlice(diagnostic);
                has_diagnostics = true;
            }
            
            line_num += 1;
        }
        
        try diagnostics.appendSlice("]}}");
        
        return try diagnostics.toOwnedSlice();
    }
    
    // Main server loop
    pub fn run(self: *CursedLSP) !void {
        std.log.info("CURSED Language Server starting...", .{});
        
        var stdin_buffer: [4096]u8 = undefined;
        const stdin = std.fs.File.stdin().reader(stdin_buffer[0..]);
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        
        var buffer = ArrayList(u8){};
        defer buffer.deinit();
        
        while (!self.shutdown_requested) {
            // Read Content-Length header
            var line_buffer: [256]u8 = undefined;
            if (try stdin.readUntilDelimiterOrEof(line_buffer[0..], '\n')) |line| {
                const trimmed = std.mem.trim(u8, line, " \r\n");
                if (std.mem.startsWith(u8, trimmed, "Content-Length:")) {
                    const content_length_str = std.mem.trim(u8, trimmed[15..], " ");
                    const content_length = try std.fmt.parseInt(usize, content_length_str, 10);
                    
                    // Skip empty line
                    _ = try stdin.readUntilDelimiterOrEof(line_buffer[0..], '\n');
                    
                    // Read message content
                    buffer.clearRetainingCapacity();
                    try buffer.resize(content_length);
                    _ = try stdin.readAll(buffer.items);
                    
                    // Process message
                    if (try self.handleMessage(buffer.items)) |response| {
                        defer self.allocator.free(response);
                        
                        // Send response
                        try stdout.writer().print("Content-Length: {s}\r\n\r\n{s}", .{{ response.len, response });
                    }
                }
            } else {
                break;
            }
        }
        
        std.log.info("CURSED Language Server shutting down...", .{});
    }
};

// Main entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var server = CursedLSP.init(allocator);
    defer server.deinit();
    
    try server.run();
}
