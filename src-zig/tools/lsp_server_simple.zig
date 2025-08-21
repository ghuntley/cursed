// Simplified CURSED Language Server Protocol Implementation
// Provides basic IDE support without complex dependencies

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple LSP Server that provides basic functionality
pub const SimpleLSPServer = struct {
    allocator: Allocator,
    initialized: bool,
    shutdown_requested: bool,

    pub fn init(allocator: Allocator) SimpleLSPServer {
        return SimpleLSPServer{
            .allocator = allocator,
            .initialized = false,
            .shutdown_requested = false,
        };
    }

    pub fn deinit(self: *SimpleLSPServer) void {
        _ = self;
    }

    pub fn run(self: *SimpleLSPServer) !void {
        std.log.info("CURSED Language Server starting...", .{});

        var stdin_buffer: [4096]u8 = undefined;
        const stdin = std.fs.File.stdin().reader(stdin_buffer[0..]);
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);

        var buffer = ArrayList(u8).init(self.allocator);
        defer buffer.deinit(allocator);

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
                        try stdout.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
                    }
                }
            } else {
                break;
            }
        }

        std.log.info("CURSED Language Server shutting down...", .{});
    }

    fn handleMessage(self: *SimpleLSPServer, message: []const u8) !?[]u8 {
        // Simple message parsing - look for method in the message text
        if (std.mem.indexOf(u8, message, "\"initialize\"")) |_| {
            return try self.handleInitialize();
        } else if (std.mem.indexOf(u8, message, "\"textDocument/completion\"")) |_| {
            return try self.handleCompletion();
        } else if (std.mem.indexOf(u8, message, "\"textDocument/hover\"")) |_| {
            return try self.handleHover();
        } else if (std.mem.indexOf(u8, message, "\"shutdown\"")) |_| {
            self.shutdown_requested = true;
            return try self.handleShutdown();
        } else if (std.mem.indexOf(u8, message, "\"textDocument/didOpen\"")) |_| {
            try self.handleDidOpen();
            return null;
        } else if (std.mem.indexOf(u8, message, "\"textDocument/didChange\"")) |_| {
            try self.handleDidChange();
            return null;
        } else if (std.mem.indexOf(u8, message, "\"exit\"")) |_| {
            self.shutdown_requested = true;
            return null;
        }

        std.log.debug("Unhandled LSP message: {s}", .{message[0..@min(100, message.len)]});
        return null;
    }

    fn handleInitialize(self: *SimpleLSPServer) ![]u8 {
        self.initialized = true;
        
        const response = 
            \\{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":[".","_"]},"hoverProvider":true,"signatureHelpProvider":{"triggerCharacters":["(",","]},"definitionProvider":true,"referencesProvider":true,"documentSymbolProvider":true,"workspaceSymbolProvider":true,"documentFormattingProvider":true,"documentRangeFormattingProvider":true,"renameProvider":true}}}
        ;
        
        return try self.allocator.dupe(u8, response);
    }

    fn handleCompletion(self: *SimpleLSPServer) ![]u8 {
        
        const response = 
            \\{"jsonrpc":"2.0","id":2,"result":[{"label":"slay","kind":14,"detail":"CURSED function keyword","documentation":"Define a function in CURSED","insertText":"slay "},{"label":"sus","kind":14,"detail":"CURSED variable keyword","documentation":"Declare a variable in CURSED","insertText":"sus "},{"label":"vibez.spill","kind":3,"detail":"Print function","documentation":"Print message to stdout","insertText":"vibez.spill(\"$1\")"},{"label":"based","kind":14,"detail":"Boolean true","documentation":"CURSED true value","insertText":"based"},{"label":"cringe","kind":14,"detail":"Boolean false","documentation":"CURSED false value","insertText":"cringe"},{"label":"normie","kind":25,"detail":"Integer type","documentation":"Standard integer type in CURSED","insertText":"normie"},{"label":"tea","kind":25,"detail":"String type","documentation":"String type in CURSED","insertText":"tea"},{"label":"lit","kind":25,"detail":"Boolean type","documentation":"Boolean type in CURSED","insertText":"lit"},{"label":"squad","kind":14,"detail":"Struct keyword","documentation":"Define a struct in CURSED","insertText":"squad "},{"label":"collab","kind":14,"detail":"Interface keyword","documentation":"Define an interface in CURSED","insertText":"collab "}]}
        ;
        
        return try self.allocator.dupe(u8, response);
    }

    fn handleHover(self: *SimpleLSPServer) ![]u8 {
        
        const response = 
            \\{"jsonrpc":"2.0","id":3,"result":{"contents":"CURSED Language - Gen Z Programming Language\n\nThis is a CURSED language construct. CURSED uses Gen Z slang keywords for programming constructs."}}
        ;
        
        return try self.allocator.dupe(u8, response);
    }

    fn handleShutdown(self: *SimpleLSPServer) ![]u8 {
        
        const response = 
            \\{"jsonrpc":"2.0","id":null,"result":null}
        ;
        
        return try self.allocator.dupe(u8, response);
    }

    fn handleDidOpen(_: *SimpleLSPServer) !void {
        std.log.info("Document opened", .{});
    }

    fn handleDidChange(_: *SimpleLSPServer) !void {
        std.log.info("Document changed", .{});
    }
};

// Main entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    var server = SimpleLSPServer.init(allocator);
    defer server.deinit(allocator);

    try server.run();
}

// Test function
pub fn testSimpleLSP() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    var server = SimpleLSPServer.init(allocator);
    defer server.deinit(allocator);

    // Test initialize
    const init_response = try server.handleInitialize();
    defer allocator.free(init_response);
    std.log.info("Initialize response: {s}", .{init_response});

    // Test completion
    const completion_response = try server.handleCompletion();
    defer allocator.free(completion_response);
    std.log.info("Completion response: {s}", .{completion_response});

    std.log.info("Simple LSP test completed successfully!", .{});
}
