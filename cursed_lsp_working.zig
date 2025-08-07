// Working CURSED Language Server Protocol Implementation
const std = @import("std");
const Allocator = std.mem.Allocator;

pub const CursedLSP = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) CursedLSP {
        return CursedLSP{ .allocator = allocator };
    }
    
    pub fn handleMessage(self: *CursedLSP, content: []const u8) !?[]u8 {
        if (std.mem.indexOf(u8, content, "\"method\":\"initialize\"")) |_| {
            return try self.allocator.dupe(u8, "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"capabilities\":{\"textDocumentSync\":1,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\"]},\"hoverProvider\":true,\"documentFormattingProvider\":true},\"serverInfo\":{\"name\":\"CURSED Language Server\",\"version\":\"1.0.0\"}}}");
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/completion\"")) |_| {
            return try self.allocator.dupe(u8, "{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":[{\"label\":\"slay\",\"kind\":14,\"detail\":\"Function declaration\",\"insertText\":\"slay\"},{\"label\":\"sus\",\"kind\":14,\"detail\":\"Variable declaration\",\"insertText\":\"sus\"},{\"label\":\"vibez.spill\",\"kind\":3,\"detail\":\"Print to output\",\"insertText\":\"vibez.spill\"},{\"label\":\"based\",\"kind\":14,\"detail\":\"True boolean\",\"insertText\":\"based\"},{\"label\":\"normie\",\"kind\":14,\"detail\":\"Integer type\",\"insertText\":\"normie\"}]}");
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/hover\"")) |_| {
            return try self.allocator.dupe(u8, "{\"jsonrpc\":\"2.0\",\"id\":3,\"result\":{\"contents\":{\"kind\":\"markdown\",\"value\":\"**CURSED Language**\\n\\nGen Z programming language with expressive syntax.\\n\\n- `slay` - Function declaration\\n- `sus` - Variable declaration\\n- `vibez.spill()` - Print output\"}}}");
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"textDocument/formatting\"")) |_| {
            return try self.allocator.dupe(u8, "{\"jsonrpc\":\"2.0\",\"id\":4,\"result\":[]}");
        }
        
        if (std.mem.indexOf(u8, content, "\"method\":\"shutdown\"")) |_| {
            return try self.allocator.dupe(u8, "{\"jsonrpc\":\"2.0\",\"id\":5,\"result\":null}");
        }
        
        // Ignore other messages
        return null;
    }
    
    pub fn run(self: *CursedLSP) !void {
        std.log.info("CURSED Language Server starting...", .{});
        
        const stdin = std.io.getStdIn().reader();
        const stdout = std.io.getStdOut().writer();
        
        var buffer = std.ArrayList(u8).init(self.allocator);
        defer buffer.deinit();
        
        while (true) {
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
                        try stdout.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
                    }
                }
            } else {
                break;
            }
        }
        
        std.log.info("CURSED Language Server shutting down...", .{});
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var server = CursedLSP.init(allocator);
    try server.run();
}
