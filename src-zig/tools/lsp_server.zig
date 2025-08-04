// Language Server Protocol Implementation for CURSED
// Provides code completion, diagnostics, go-to-definition, and more

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const json = std.json;

// Import CURSED compiler components
const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const ast = @import("../ast.zig");

// LSP Protocol Structures
const Position = struct {
    line: u32,
    character: u32,
};

const Range = struct {
    start: Position,
    end: Position,
};

const TextDocumentIdentifier = struct {
    uri: []const u8,
};

const CompletionItem = struct {
    label: []const u8,
    kind: u32, // CompletionItemKind
    detail: ?[]const u8 = null,
    documentation: ?[]const u8 = null,
    insertText: ?[]const u8 = null,
};

const Diagnostic = struct {
    range: Range,
    severity: u32, // DiagnosticSeverity
    message: []const u8,
    source: ?[]const u8 = "cursed-lsp",
};

const Hover = struct {
    contents: []const u8,
    range: ?Range = null,
};

const Location = struct {
    uri: []const u8,
    range: Range,
};

// LSP Server State
pub const LSPServer = struct {
    allocator: Allocator,
    documents: std.HashMap([]const u8, DocumentState, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    symbols: std.HashMap([]const u8, SymbolInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const DocumentState = struct {
        content: []const u8,
        version: u32,
        ast_tree: ?ast.AST = null,
        diagnostics: ArrayList(Diagnostic),
    };
    
    const SymbolInfo = struct {
        name: []const u8,
        kind: u32,
        location: Location,
        type_info: ?[]const u8 = null,
    };
    
    pub fn init(allocator: Allocator) LSPServer {
        return LSPServer{
            .allocator = allocator,
            .documents = std.HashMap([]const u8, DocumentState, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .symbols = std.HashMap([]const u8, SymbolInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *LSPServer) void {
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            entry.value_ptr.diagnostics.deinit();
            if (entry.value_ptr.ast_tree) |tree| {
                tree.deinit();
            }
        }
        self.documents.deinit();
        self.symbols.deinit();
    }
    
    // Document Management
    pub fn openDocument(self: *LSPServer, uri: []const u8, content: []const u8, version: u32) !void {
        const diagnostics = try self.parseAndAnalyze(content);
        
        const doc_state = DocumentState{
            .content = try self.allocator.dupe(u8, content),
            .version = version,
            .diagnostics = diagnostics,
        };
        
        try self.documents.put(try self.allocator.dupe(u8, uri), doc_state);
        try self.updateSymbols(uri, content);
    }
    
    pub fn updateDocument(self: *LSPServer, uri: []const u8, content: []const u8, version: u32) !void {
        if (self.documents.getPtr(uri)) |doc| {
            self.allocator.free(doc.content);
            doc.diagnostics.deinit();
            
            doc.content = try self.allocator.dupe(u8, content);
            doc.version = version;
            doc.diagnostics = try self.parseAndAnalyze(content);
            
            try self.updateSymbols(uri, content);
        }
    }
    
    pub fn closeDocument(self: *LSPServer, uri: []const u8) void {
        if (self.documents.fetchRemove(uri)) |entry| {
            self.allocator.free(entry.value.content);
            entry.value.diagnostics.deinit();
        }
    }
    
    // Core LSP Features
    pub fn getCompletions(self: *LSPServer, uri: []const u8, position: Position) !ArrayList(CompletionItem) {
        var completions = ArrayList(CompletionItem).init(self.allocator);
        
        // Built-in CURSED keywords
        const keywords = [_][]const u8{
            "sus", "spill", "slay", "damn", "vibes", "bestie", 
            "based", "cringe", "yeet", "stan", "vibez", "lit",
            "drip", "normie", "thicc", "smol", "meal", "tea",
            "collab", "squad", "flex", "ready", "match", "case"
        };
        
        for (keywords) |keyword| {
            try completions.append(CompletionItem{
                .label = keyword,
                .kind = 14, // Keyword
                .detail = "CURSED keyword",
                .insertText = keyword,
            });
        }
        
        // Function completions from current document
        if (self.documents.get(uri)) |doc| {
            if (doc.ast_tree) |tree| {
                try self.addFunctionCompletions(&completions, tree);
            }
        }
        
        // Variable completions
        try self.addVariableCompletions(&completions, uri, position);
        
        return completions;
    }
    
    pub fn getHover(self: *LSPServer, uri: []const u8, position: Position) !?Hover {
        if (self.documents.get(uri)) |doc| {
            // Find symbol at position
            const symbol_name = try self.getSymbolAtPosition(doc.content, position);
            if (symbol_name) |name| {
                if (self.symbols.get(name)) |symbol| {
                    var hover_content = ArrayList(u8).init(self.allocator);
                    defer hover_content.deinit();
                    
                    try hover_content.appendSlice("**");
                    try hover_content.appendSlice(symbol.name);
                    try hover_content.appendSlice("**\n\n");
                    
                    if (symbol.type_info) |type_info| {
                        try hover_content.appendSlice("Type: `");
                        try hover_content.appendSlice(type_info);
                        try hover_content.appendSlice("`\n\n");
                    }
                    
                    try hover_content.appendSlice("Defined in CURSED");
                    
                    return Hover{
                        .contents = try hover_content.toOwnedSlice(),
                    };
                }
            }
        }
        return null;
    }
    
    pub fn getDefinition(self: *LSPServer, uri: []const u8, position: Position) !?Location {
        if (self.documents.get(uri)) |doc| {
            const symbol_name = try self.getSymbolAtPosition(doc.content, position);
            if (symbol_name) |name| {
                if (self.symbols.get(name)) |symbol| {
                    return symbol.location;
                }
            }
        }
        return null;
    }
    
    pub fn getDiagnostics(self: *LSPServer, uri: []const u8) !ArrayList(Diagnostic) {
        if (self.documents.get(uri)) |doc| {
            return try doc.diagnostics.clone();
        }
        return ArrayList(Diagnostic).init(self.allocator);
    }
    
    // Helper Functions
    fn parseAndAnalyze(self: *LSPServer, content: []const u8) !ArrayList(Diagnostic) {
        var diagnostics = ArrayList(Diagnostic).init(self.allocator);
        
        // Tokenize content
        var token_lexer = lexer.Lexer.init(self.allocator, content);
        defer token_lexer.deinit();
        
        const tokens = token_lexer.tokenize() catch |err| {
            try diagnostics.append(Diagnostic{
                .range = Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 10 },
                },
                .severity = 1, // Error
                .message = "Lexical analysis failed",
            });
            return diagnostics;
        };
        
        // Parse tokens
        var cursed_parser = parser.Parser.init(self.allocator, tokens);
        defer cursed_parser.deinit();
        
        const ast_tree = cursed_parser.parse() catch |err| {
            try diagnostics.append(Diagnostic{
                .range = Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 20 },
                },
                .severity = 1, // Error
                .message = "Parse error in CURSED code",
            });
            return diagnostics;
        };
        
        // Semantic analysis would go here
        try self.performSemanticAnalysis(&diagnostics, ast_tree);
        
        return diagnostics;
    }
    
    fn performSemanticAnalysis(self: *LSPServer, diagnostics: *ArrayList(Diagnostic), tree: ast.AST) !void {
        // Type checking
        // Undefined variable detection
        // Function signature validation
        // etc.
        
        // Example: Check for undefined variables
        // This would be expanded with full semantic analysis
    }
    
    fn updateSymbols(self: *LSPServer, uri: []const u8, content: []const u8) !void {
        // Extract symbols from AST and add to symbol table
        // This would walk the AST and collect function definitions, variables, etc.
    }
    
    fn addFunctionCompletions(self: *LSPServer, completions: *ArrayList(CompletionItem), tree: ast.AST) !void {
        // Walk AST to find function definitions and add them as completions
    }
    
    fn addVariableCompletions(self: *LSPServer, completions: *ArrayList(CompletionItem), uri: []const u8, position: Position) !void {
        // Find variables in scope at the given position
    }
    
    fn getSymbolAtPosition(self: *LSPServer, content: []const u8, position: Position) !?[]const u8 {
        // Find the symbol/identifier at the given position
        // This would parse the line and extract the identifier
        return null;
    }
};

// LSP Communication Handler
pub const LSPHandler = struct {
    server: LSPServer,
    
    pub fn init(allocator: Allocator) LSPHandler {
        return LSPHandler{
            .server = LSPServer.init(allocator),
        };
    }
    
    pub fn deinit(self: *LSPHandler) void {
        self.server.deinit();
    }
    
    pub fn handleRequest(self: *LSPHandler, request: []const u8) ![]const u8 {
        // Parse JSON-RPC request
        var parsed = try json.parseFromSlice(json.Value, self.server.allocator, request, .{});
        defer parsed.deinit();
        
        const method = parsed.value.object.get("method").?.string;
        
        if (std.mem.eql(u8, method, "textDocument/completion")) {
            return try self.handleCompletion(parsed.value);
        } else if (std.mem.eql(u8, method, "textDocument/hover")) {
            return try self.handleHover(parsed.value);
        } else if (std.mem.eql(u8, method, "textDocument/definition")) {
            return try self.handleDefinition(parsed.value);
        } else if (std.mem.eql(u8, method, "textDocument/didOpen")) {
            return try self.handleDidOpen(parsed.value);
        } else if (std.mem.eql(u8, method, "textDocument/didChange")) {
            return try self.handleDidChange(parsed.value);
        } else if (std.mem.eql(u8, method, "textDocument/didClose")) {
            return try self.handleDidClose(parsed.value);
        }
        
        return try self.createErrorResponse("Method not found");
    }
    
    fn handleCompletion(self: *LSPHandler, request: json.Value) ![]const u8 {
        // Extract parameters and call server.getCompletions()
        // Return JSON response
        return "{}";
    }
    
    fn handleHover(self: *LSPHandler, request: json.Value) ![]const u8 {
        // Extract parameters and call server.getHover()
        return "{}";
    }
    
    fn handleDefinition(self: *LSPHandler, request: json.Value) ![]const u8 {
        // Extract parameters and call server.getDefinition()
        return "{}";
    }
    
    fn handleDidOpen(self: *LSPHandler, request: json.Value) ![]const u8 {
        // Extract parameters and call server.openDocument()
        return "{}";
    }
    
    fn handleDidChange(self: *LSPHandler, request: json.Value) ![]const u8 {
        // Extract parameters and call server.updateDocument()
        return "{}";
    }
    
    fn handleDidClose(self: *LSPHandler, request: json.Value) ![]const u8 {
        // Extract parameters and call server.closeDocument()
        return "{}";
    }
    
    fn createErrorResponse(self: *LSPHandler, message: []const u8) ![]const u8 {
        return "{ \"error\": { \"message\": \"Method not found\" } }";
    }
};

// Main LSP Server Entry Point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var handler = LSPHandler.init(allocator);
    defer handler.deinit();
    
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();
    
    // LSP communication loop
    while (true) {
        // Read Content-Length header
        var header_buffer: [1024]u8 = undefined;
        const header_line = try stdin.readUntilDelimiterOrEof(header_buffer[0..], '\n');
        if (header_line == null) break;
        
        // Parse Content-Length
        var content_length: usize = 0;
        if (std.mem.startsWith(u8, header_line.?, "Content-Length: ")) {
            const length_str = header_line.?[16..];
            content_length = try std.fmt.parseInt(usize, std.mem.trim(u8, length_str, " \r\n"), 10);
        }
        
        // Skip empty line
        _ = try stdin.readUntilDelimiterOrEof(header_buffer[0..], '\n');
        
        // Read request body
        const request_body = try allocator.alloc(u8, content_length);
        defer allocator.free(request_body);
        _ = try stdin.readAll(request_body);
        
        // Handle request
        const response = handler.handleRequest(request_body) catch |err| {
            std.log.err("Error handling request: {}", .{err});
            continue;
        };
        defer allocator.free(response);
        
        // Send response
        try stdout.print("Content-Length: {}\r\n\r\n{s}", .{ response.len, response });
    }
}
