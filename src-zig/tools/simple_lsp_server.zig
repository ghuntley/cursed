// Simple CURSED Language Server Protocol Implementation
// Basic LSP functionality compatible with current Zig APIs

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const ast = @import("../ast.zig");

// Simple LSP Message handling
const LSPMessage = struct {
    method: ?[]const u8 = null,
    id: ?i32 = null,
    content: []const u8,
};

// Document state
const DocumentState = struct {
    uri: []const u8,
    content: []const u8,
    version: i32,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, uri: []const u8, content: []const u8, version: i32) !DocumentState {
        return DocumentState{
            .uri = try allocator.dupe(u8, uri),
            .content = try allocator.dupe(u8, content),
            .version = version,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DocumentState) void {
        self.allocator.free(self.uri);
        self.allocator.free(self.content);
    }
    
    pub fn update(self: *DocumentState, new_content: []const u8, new_version: i32) !void {
        self.allocator.free(self.content);
        self.content = try self.allocator.dupe(u8, new_content);
        self.version = new_version;
    }
};

// CURSED Language Data
const CursedKeywords = [_][]const u8{
    "slay", "sus", "facts", "lowkey", "highkey", "periodt", "stan", "bestie", 
    "flex", "ghosted", "simp", "squad", "collab", "yeet", "vibes", "mood", 
    "basic", "match", "based", "cringe", "normie", "tea", "lit", "drip",
    "thicc", "smol", "meal", "yikes", "shook", "fam", "spill", "ready",
    "later", "dm", "select", "fn", "let", "mut", "if", "else", "while",
    "for", "return", "struct", "interface", "import", "package", "true",
    "false", "nil", "switch", "case", "default"
};

const CursedStdlibFunctions = [_][]const u8{
    "vibez.spill", "vibez.spillf", "vibez.read_line", "len", "append", 
    "make", "cryptz.hash", "cryptz.encrypt", "concurrenz.spawn", 
    "concurrenz.send", "concurrenz.receive"
};

const CursedTypes = [_][]const u8{
    "normie", "tea", "lit", "drip", "thicc", "smol", "meal", "byte", 
    "rune", "[]normie", "[]tea", "[]byte", "dm", "interface{}", 
    "map[tea]normie"
};

// Simple LSP Handler
const SimpleLSPHandler = struct {
    allocator: Allocator,
    documents: std.StringHashMap(DocumentState),
    initialized: bool,
    
    pub fn init(allocator: Allocator) SimpleLSPHandler {
        return SimpleLSPHandler{
            .allocator = allocator,
            .documents = std.StringHashMap(DocumentState).init(allocator),
            .initialized = false,
        };
    }
    
    pub fn deinit(self: *SimpleLSPHandler) void {
        var iterator = self.documents.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.documents.deinit();
    }
    
    pub fn handleMessage(self: *SimpleLSPHandler, message: []const u8) !?[]u8 {
        // Simple message parsing - look for method and extract basic info
        if (std.mem.indexOf(u8, message, "\"method\":\"initialize\"")) |_| {
            return try self.handleInitialize();
        } else if (std.mem.indexOf(u8, message, "\"method\":\"textDocument/didOpen\"")) |_| {
            try self.handleDidOpen(message);
            return null;
        } else if (std.mem.indexOf(u8, message, "\"method\":\"textDocument/didChange\"")) |_| {
            try self.handleDidChange(message);
            return null;
        } else if (std.mem.indexOf(u8, message, "\"method\":\"textDocument/completion\"")) |_| {
            return try self.handleCompletion(message);
        } else if (std.mem.indexOf(u8, message, "\"method\":\"textDocument/hover\"")) |_| {
            return try self.handleHover(message);
        } else if (std.mem.indexOf(u8, message, "\"method\":\"shutdown\"")) |_| {
            return try self.handleShutdown();
        }
        
        return null;
    }
    
    fn handleInitialize(self: *SimpleLSPHandler) ![]u8 {
        self.initialized = true;
        const response = 
            \\{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":[]},"hoverProvider":true,"definitionProvider":true,"documentSymbolProvider":true,"documentFormattingProvider":true},"serverInfo":{"name":"cursed-lsp","version":"1.0.0"}}}
        ;
        return try self.allocator.dupe(u8, response);
    }
    
    fn handleDidOpen(self: *SimpleLSPHandler, message: []const u8) !void {
        // Extract URI and text from message (simplified parsing)
        if (extractJsonString(message, "uri")) |uri| {
            if (extractJsonString(message, "text")) |text| {
                const doc = try DocumentState.init(self.allocator, uri, text, 1);
                try self.documents.put(try self.allocator.dupe(u8, uri), doc);
                
                // Send diagnostics
                try self.sendDiagnostics(uri, text);
            }
        }
    }
    
    fn handleDidChange(self: *SimpleLSPHandler, message: []const u8) !void {
        if (extractJsonString(message, "uri")) |uri| {
            if (extractJsonString(message, "text")) |text| {
                if (self.documents.getPtr(uri)) |doc| {
                    try doc.update(text, doc.version + 1);
                    try self.sendDiagnostics(uri, text);
                }
            }
        }
    }
    
    fn handleCompletion(self: *SimpleLSPHandler, message: []const u8) ![]u8 {
        _ = message;
        
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        try result.appendSlice("{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":[");
        
        // Add keywords
        for (CursedKeywords, 0..) |keyword, i| {
            if (i > 0) try result.appendSlice(",");
            try result.writer().print("{{\"label\":\"{s}\",\"kind\":14,\"detail\":\"CURSED keyword\"}}", .{keyword});
        }
        
        try result.appendSlice(",");
        
        // Add stdlib functions
        for (CursedStdlibFunctions, 0..) |func, i| {
            if (i > 0) try result.appendSlice(",");
            try result.writer().print("{{\"label\":\"{s}\",\"kind\":3,\"detail\":\"CURSED function\"}}", .{func});
        }
        
        try result.appendSlice(",");
        
        // Add types
        for (CursedTypes, 0..) |type_name, i| {
            if (i > 0) try result.appendSlice(",");
            try result.writer().print("{{\"label\":\"{s}\",\"kind\":25,\"detail\":\"CURSED type\"}}", .{type_name});
        }
        
        try result.appendSlice("]}");
        
        return try result.toOwnedSlice();
    }
    
    fn handleHover(self: *SimpleLSPHandler, message: []const u8) ![]u8 {
        _ = message;
        
        // Simple hover response
        const response = 
            \\{"jsonrpc":"2.0","id":3,"result":{"contents":"CURSED programming language - Gen Z syntax for the modern developer"}}
        ;
        return try self.allocator.dupe(u8, response);
    }
    
    fn handleShutdown(self: *SimpleLSPHandler) ![]u8 {
        const response = 
            \\{"jsonrpc":"2.0","id":4,"result":null}
        ;
        return try self.allocator.dupe(u8, response);
    }
    
    fn sendDiagnostics(self: *SimpleLSPHandler, uri: []const u8, content: []const u8) !void {
        // Simple syntax check - look for basic errors
        var diagnostics = ArrayList(u8).init(self.allocator);
        defer diagnostics.deinit();
        
        try diagnostics.writer().print("Content-Length: 200\r\n\r\n{{\"jsonrpc\":\"2.0\",\"method\":\"textDocument/publishDiagnostics\",\"params\":{{\"uri\":\"{s}\",\"diagnostics\":[", .{uri});
        
        var line: u32 = 0;
        var column: u32 = 0;
        var has_errors = false;
        
        // Check for unterminated strings
        var in_string = false;
        for (content, 0..) |char, i| {
            switch (char) {
                '"' => {
                    if (i == 0 or content[i-1] != '\\') {
                        in_string = !in_string;
                    }
                },
                '\n' => {
                    if (in_string) {
                        if (has_errors) try diagnostics.appendSlice(",");
                        try diagnostics.writer().print("{{\"range\":{{\"start\":{{\"line\":{},\"character\":{}}},\"end\":{{\"line\":{},\"character\":{}}}}},\"severity\":1,\"source\":\"cursed-lsp\",\"message\":\"Unterminated string literal\"}}", .{line, column, line, column + 1});
                        has_errors = true;
                        in_string = false;
                    }
                    line += 1;
                    column = 0;
                },
                else => {
                    column += 1;
                },
            }
        }
        
        try diagnostics.appendSlice("]}}");
        
        std.log.info("Sending diagnostics: {s}", .{diagnostics.items});
    }
};

// Helper function to extract JSON string values
fn extractJsonString(json_text: []const u8, key: []const u8) ?[]const u8 {
    const search_pattern = std.fmt.allocPrint(std.heap.page_allocator, "\"{s}\":\"", .{key}) catch return null;
    defer std.heap.page_allocator.free(search_pattern);
    
    if (std.mem.indexOf(u8, json_text, search_pattern)) |start| {
        const value_start = start + search_pattern.len;
        if (std.mem.indexOfScalarPos(u8, json_text, value_start, '"')) |end| {
            return json_text[value_start..end];
        }
    }
    return null;
}

// Main LSP Server
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var handler = SimpleLSPHandler.init(allocator);
    defer handler.deinit();
    
    std.log.info("CURSED Simple Language Server starting...", .{});
    
    const stdin = std.io.getStdIn().reader();
    var buffer = ArrayList(u8).init(allocator);
    defer buffer.deinit();
    
    // Main LSP loop
    while (true) {
        // Read Content-Length header
        var header_buffer: [256]u8 = undefined;
        if (try stdin.readUntilDelimiterOrEof(header_buffer[0..], '\n')) |header_line| {
            const trimmed = std.mem.trim(u8, header_line, " \r\n");
            if (std.mem.startsWith(u8, trimmed, "Content-Length:")) {
                const length_str = std.mem.trim(u8, trimmed[15..], " ");
                const content_length = try std.fmt.parseInt(usize, length_str, 10);
                
                // Skip empty line
                _ = try stdin.readUntilDelimiterOrEof(header_buffer[0..], '\n');
                
                // Read message content
                buffer.clearRetainingCapacity();
                try buffer.resize(content_length);
                _ = try stdin.readAll(buffer.items);
                
                // Process message
                if (try handler.handleMessage(buffer.items)) |response| {
                    defer allocator.free(response);
                    std.log.info("Sending response: Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
                }
            }
        } else {
            break;
        }
    }
    
    std.log.info("CURSED Simple Language Server shutting down...", .{});
}

// Test function for development
pub fn testSimpleLSP() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var handler = SimpleLSPHandler.init(allocator);
    defer handler.deinit();
    
    // Test initialize
    const init_message = 
        \\{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}
    ;
    
    if (try handler.handleMessage(init_message)) |response| {
        defer allocator.free(response);
        std.log.info("Initialize response: {s}", .{response});
    }
    
    // Test completion
    const completion_message = 
        \\{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":1,"character":4}}}
    ;
    
    if (try handler.handleMessage(completion_message)) |response| {
        defer allocator.free(response);
        std.log.info("Completion response: {s}", .{response});
    }
    
    std.log.info("Simple LSP test completed!", .{});
}
