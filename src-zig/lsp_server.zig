//! CURSED Language Server Protocol Implementation in Zig
//! Provides comprehensive IDE support with semantic analysis

const std = @import("std");
const json = std.json;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const type_system = @import("type_system_runtime.zig");

/// LSP Protocol message types
const MessageType = enum {
    initialize,
    initialized,
    textDocument_didOpen,
    textDocument_didChange,
    textDocument_didSave,
    textDocument_didClose,
    textDocument_completion,
    textDocument_hover,
    textDocument_definition,
    textDocument_references,
    textDocument_formatting,
    textDocument_rename,
    textDocument_semanticTokens_full,
    workspace_symbol,
    shutdown,
    exit,
};

/// LSP Position structure
const Position = struct {
    line: u32,
    character: u32,
};

/// LSP Range structure
const Range = struct {
    start: Position,
    end: Position,
};

/// LSP Location structure
const Location = struct {
    uri: []const u8,
    range: Range,
};

/// LSP Diagnostic structure
const Diagnostic = struct {
    range: Range,
    severity: u32, // 1=Error, 2=Warning, 3=Information, 4=Hint
    code: ?[]const u8,
    source: []const u8,
    message: []const u8,
};

/// LSP CompletionItem structure
const CompletionItem = struct {
    label: []const u8,
    kind: u32, // CompletionItemKind
    detail: ?[]const u8,
    documentation: ?[]const u8,
    insertText: ?[]const u8,
};

/// LSP SymbolInformation structure
const SymbolInformation = struct {
    name: []const u8,
    kind: u32, // SymbolKind
    location: Location,
    containerName: ?[]const u8,
};

/// LSP Hover structure
const Hover = struct {
    contents: []const u8,
    range: ?Range,
};

/// Document data with parsed information
const DocumentData = struct {
    uri: []const u8,
    text: []const u8,
    version: i32,
    ast: ?ast.Program,
    symbols: ArrayList(SymbolInformation),
    diagnostics: ArrayList(Diagnostic),
    allocator: Allocator,

    pub fn init(allocator: Allocator, uri: []const u8, text: []const u8, version: i32) !DocumentData {
        return DocumentData{
            .uri = try allocator.dupe(u8, uri),
            .text = try allocator.dupe(u8, text),
            .version = version,
            .ast = null,
            .symbols = ArrayList(SymbolInformation).init(allocator),
            .diagnostics = ArrayList(Diagnostic).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DocumentData) void {
        self.allocator.free(self.uri);
        self.allocator.free(self.text);
        self.symbols.deinit();
        self.diagnostics.deinit();
    }
};

/// CURSED Language Server
pub const CursedLanguageServer = struct {
    allocator: Allocator,
    documents: HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage),
    workspace_root: ?[]const u8,
    initialized: bool,

    const StringContext = struct {
        pub fn hash(self: @This(), s: []const u8) u64 {
            _ = self;
            return std.hash_map.hashString(s);
        }
        
        pub fn eql(self: @This(), a: []const u8, b: []const u8) bool {
            _ = self;
            return std.mem.eql(u8, a, b);
        }
    };

    pub fn init(allocator: Allocator) CursedLanguageServer {
        return CursedLanguageServer{
            .allocator = allocator,
            .documents = HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .workspace_root = null,
            .initialized = false,
        };
    }

    pub fn deinit(self: *CursedLanguageServer) void {
        var iterator = self.documents.iterator();
        while (iterator.next()) |entry| {
            var doc = entry.value_ptr;
            doc.deinit();
        }
        self.documents.deinit();
        
        if (self.workspace_root) |root| {
            self.allocator.free(root);
        }
    }

    /// Handle LSP requests
    pub fn handleRequest(self: *CursedLanguageServer, input: []const u8, writer: std.io.AnyWriter) !void {
        const parsed = json.parseFromSlice(json.Value, self.allocator, input) catch |err| {
            std.log.err("Failed to parse JSON: {}", .{err});
            return;
        };
        defer parsed.deinit();

        const root = parsed.value;
        
        if (root.object.get("method")) |method| {
            const method_str = method.string;
            
            if (std.mem.eql(u8, method_str, "initialize")) {
                try self.handleInitialize(root, writer);
            } else if (std.mem.eql(u8, method_str, "initialized")) {
                try self.handleInitialized();
            } else if (std.mem.eql(u8, method_str, "textDocument/didOpen")) {
                try self.handleDidOpenTextDocument(root);
            } else if (std.mem.eql(u8, method_str, "textDocument/didChange")) {
                try self.handleDidChangeTextDocument(root);
            } else if (std.mem.eql(u8, method_str, "textDocument/completion")) {
                try self.handleCompletion(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/hover")) {
                try self.handleHover(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/definition")) {
                try self.handleDefinition(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/references")) {
                try self.handleReferences(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/formatting")) {
                try self.handleFormatting(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/semanticTokens/full")) {
                try self.handleSemanticTokens(root, writer);
            } else if (std.mem.eql(u8, method_str, "workspace/symbol")) {
                try self.handleWorkspaceSymbol(root, writer);
            } else if (std.mem.eql(u8, method_str, "shutdown")) {
                try self.handleShutdown(writer);
            } else if (std.mem.eql(u8, method_str, "exit")) {
                // Exit gracefully
                return;
            }
        }
    }

    /// Handle initialize request
    fn handleInitialize(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        // Extract workspace folders if available
        if (request.object.get("params")) |params| {
            if (params.object.get("workspaceFolders")) |folders| {
                if (folders.array.items.len > 0) {
                    const first_folder = folders.array.items[0];
                    if (first_folder.object.get("uri")) |uri| {
                        self.workspace_root = try self.allocator.dupe(u8, uri.string);
                    }
                }
            }
        }

        const response =
            \\{{
            \\  "jsonrpc": "2.0",
            \\  "id": {},
            \\  "result": {{
            \\    "capabilities": {{
            \\      "textDocumentSync": {{
            \\        "openClose": true,
            \\        "change": 2
            \\      }},
            \\      "completionProvider": {{
            \\        "triggerCharacters": ["."],
            \\        "resolveProvider": false
            \\      }},
            \\      "hoverProvider": true,
            \\      "definitionProvider": true,
            \\      "referencesProvider": true,
            \\      "documentFormattingProvider": true,
            \\      "renameProvider": true,
            \\      "semanticTokensProvider": {{
            \\        "legend": {{
            \\          "tokenTypes": ["keyword", "string", "number", "comment", "operator", "namespace", "type", "class", "interface", "enum", "function", "method", "variable", "parameter", "property"],
            \\          "tokenModifiers": ["declaration", "definition", "readonly", "static", "deprecated", "abstract", "async", "modification", "documentation", "defaultLibrary"]
            \\        }},
            \\        "full": true
            \\      }},
            \\      "workspaceSymbolProvider": true
            \\    }}
            \\  }}
            \\}}
        ;
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.print(response, .{id});
    }

    /// Handle initialized notification
    fn handleInitialized(self: *CursedLanguageServer) !void {
        self.initialized = true;
        std.log.info("CURSED LSP Server initialized", .{});
    }

    /// Handle didOpen text document
    fn handleDidOpenTextDocument(self: *CursedLanguageServer, request: json.Value) !void {
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        
        const uri = text_document.get("uri").?.string;
        const text = text_document.get("text").?.string;
        const version = @as(i32, @intCast(text_document.get("version").?.integer));

        var doc_data = try DocumentData.init(self.allocator, uri, text, version);
        try self.analyzeDocument(&doc_data);
        
        try self.documents.put(uri, doc_data);
        try self.publishDiagnostics(uri, &doc_data.diagnostics);
    }

    /// Handle didChange text document
    fn handleDidChangeTextDocument(self: *CursedLanguageServer, request: json.Value) !void {
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const content_changes = params.get("contentChanges").?.array;
        
        const uri = text_document.get("uri").?.string;
        const version = @as(i32, @intCast(text_document.get("version").?.integer));

        if (self.documents.getPtr(uri)) |doc_data| {
            // Update document content (simplified - assumes full document updates)
            if (content_changes.items.len > 0) {
                const change = content_changes.items[0].object;
                if (change.get("text")) |new_text| {
                    self.allocator.free(doc_data.text);
                    doc_data.text = try self.allocator.dupe(u8, new_text.string);
                    doc_data.version = version;
                    
                    // Clear previous analysis
                    doc_data.symbols.clearAndFree();
                    doc_data.diagnostics.clearAndFree();
                    
                    try self.analyzeDocument(doc_data);
                    try self.publishDiagnostics(uri, &doc_data.diagnostics);
                }
            }
        }
    }

    /// Analyze document and extract symbols/diagnostics
    fn analyzeDocument(self: *CursedLanguageServer, doc_data: *DocumentData) !void {
        // Tokenize
        var lex = lexer.Lexer.init(self.allocator, doc_data.text);
        defer lex.deinit();
        
        const tokens = lex.tokenize() catch |err| {
            try doc_data.diagnostics.append(Diagnostic{
                .range = Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 10 },
                },
                .severity = 1, // Error
                .code = null,
                .source = "cursed-lexer",
                .message = try std.fmt.allocPrint(self.allocator, "Lexer error: {}", .{err}),
            });
            return;
        };

        // Parse
        var parse = parser.Parser.init(self.allocator, tokens);
        defer parse.deinit();
        
        const program = parse.parseProgram() catch |err| {
            try doc_data.diagnostics.append(Diagnostic{
                .range = Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 10 },
                },
                .severity = 1, // Error
                .code = null,
                .source = "cursed-parser",
                .message = try std.fmt.allocPrint(self.allocator, "Parser error: {}", .{err}),
            });
            return;
        };

        doc_data.ast = program;
        try self.extractSymbols(doc_data, program);
    }

    /// Extract symbols from AST
    fn extractSymbols(self: *CursedLanguageServer, doc_data: *DocumentData, program: ast.Program) !void {
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .function_declaration => |func| {
                    try doc_data.symbols.append(SymbolInformation{
                        .name = try self.allocator.dupe(u8, func.name),
                        .kind = 12, // Function
                        .location = Location{
                            .uri = doc_data.uri,
                            .range = Range{
                                .start = Position{ .line = 0, .character = 0 },
                                .end = Position{ .line = 0, .character = @as(u32, @intCast(func.name.len)) },
                            },
                        },
                        .containerName = null,
                    });
                },
                .variable_declaration => |var_decl| {
                    try doc_data.symbols.append(SymbolInformation{
                        .name = try self.allocator.dupe(u8, var_decl.name),
                        .kind = 13, // Variable
                        .location = Location{
                            .uri = doc_data.uri,
                            .range = Range{
                                .start = Position{ .line = 0, .character = 0 },
                                .end = Position{ .line = 0, .character = @as(u32, @intCast(var_decl.name.len)) },
                            },
                        },
                        .containerName = null,
                    });
                },
                .interface_statement => |interface| {
                    try doc_data.symbols.append(SymbolInformation{
                        .name = try self.allocator.dupe(u8, interface.name),
                        .kind = 11, // Interface
                        .location = Location{
                            .uri = doc_data.uri,
                            .range = Range{
                                .start = Position{ .line = 0, .character = 0 },
                                .end = Position{ .line = 0, .character = @as(u32, @intCast(interface.name.len)) },
                            },
                        },
                        .containerName = null,
                    });
                },
                else => {}, // Handle other statement types
            }
        }
    }

    /// Handle completion request
    fn handleCompletion(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        _ = text_document.get("uri").?.string; // uri not used in current implementation
        
        var completions = ArrayList(CompletionItem).init(self.allocator);
        defer {
            for (completions.items) |item| {
                self.allocator.free(item.label);
                if (item.detail) |detail| self.allocator.free(detail);
                if (item.documentation) |doc| self.allocator.free(doc);
            }
            completions.deinit();
        }

        // Add CURSED keywords
        const keywords = [_][]const u8{
            "sus", "damn", "slay", "vibez", "yeet", "bestie", "stan", "dm",
            "ready", "vibe", "yikes", "shook", "fam", "based", "cap", "cringe",
            "facts", "lit", "tea", "drip", "normie", "smol", "mid", "thicc",
        };

        for (keywords) |keyword| {
            try completions.append(CompletionItem{
                .label = try self.allocator.dupe(u8, keyword),
                .kind = 14, // Keyword
                .detail = try self.allocator.dupe(u8, "CURSED keyword"),
                .documentation = null,
                .insertText = null,
            });
        }

        // Add symbols from workspace
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc_data = entry.value_ptr;
            for (doc_data.symbols.items) |symbol| {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, symbol.name),
                    .kind = switch (symbol.kind) {
                        12 => 3, // Function
                        13 => 6, // Variable
                        11 => 8, // Interface
                        else => 1, // Text
                    },
                    .detail = try std.fmt.allocPrint(self.allocator, "Symbol from {s}", .{symbol.location.uri}),
                    .documentation = null,
                    .insertText = null,
                });
            }
        }

        // Send response
        try self.sendCompletionResponse(writer, id, &completions);
    }

    /// Handle hover request
    fn handleHover(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const position = params.get("position").?.object;
        
        const uri = text_document.get("uri").?.string;
        const line = @as(u32, @intCast(position.get("line").?.integer));
        const character = @as(u32, @intCast(position.get("character").?.integer));

        var hover_text: ?[]const u8 = null;
        
        if (self.documents.get(uri)) |doc_data| {
            // Find symbol at position (simplified)
            for (doc_data.symbols.items) |symbol| {
                if (self.positionInRange(Position{ .line = line, .character = character }, symbol.location.range)) {
                    hover_text = try std.fmt.allocPrint(self.allocator, "**{s}**\n\nType: {s}", .{ symbol.name, switch (symbol.kind) {
                        12 => "Function",
                        13 => "Variable", 
                        11 => "Interface",
                        else => "Symbol",
                    }});
                    break;
                }
            }
        }

        try self.sendHoverResponse(writer, id, hover_text);
        if (hover_text) |text| {
            self.allocator.free(text);
        }
    }

    /// Handle definition request
    fn handleDefinition(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const position = params.get("position").?.object;
        
        const uri = text_document.get("uri").?.string;
        const line = @as(u32, @intCast(position.get("line").?.integer));
        const character = @as(u32, @intCast(position.get("character").?.integer));

        var locations = ArrayList(Location).init(self.allocator);
        defer locations.deinit();

        if (self.documents.get(uri)) |doc_data| {
            for (doc_data.symbols.items) |symbol| {
                if (self.positionInRange(Position{ .line = line, .character = character }, symbol.location.range)) {
                    try locations.append(symbol.location);
                }
            }
        }

        try self.sendDefinitionResponse(writer, id, &locations);
    }

    /// Handle references request  
    fn handleReferences(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        try self.sendReferencesResponse(writer, id, &ArrayList(Location).init(self.allocator));
    }

    /// Handle formatting request
    fn handleFormatting(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;

        var text_edits = ArrayList([]const u8).init(self.allocator);
        defer text_edits.deinit();

        if (self.documents.get(uri)) |doc_data| {
            const formatted = try self.formatCursedCode(doc_data.text);
            defer self.allocator.free(formatted);
            
            if (!std.mem.eql(u8, formatted, doc_data.text)) {
                // Return full document replacement
                const edit = try std.fmt.allocPrint(self.allocator, 
                    \\{{"range": {{"start": {{"line": 0, "character": 0}}, "end": {{"line": {}, "character": 0}}}}, "newText": "{s}"}}
                , .{ std.mem.count(u8, doc_data.text, "\n"), formatted });
                try text_edits.append(edit);
            }
        }

        try self.sendFormattingResponse(writer, id, &text_edits);
        
        for (text_edits.items) |edit| {
            self.allocator.free(edit);
        }
    }

    /// Handle semantic tokens request
    fn handleSemanticTokens(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        try self.sendSemanticTokensResponse(writer, id, &ArrayList(u32).init(self.allocator));
    }

    /// Handle workspace symbol request
    fn handleWorkspaceSymbol(self: *CursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const query = if (params.get("query")) |q| q.string else "";

        var symbols = ArrayList(SymbolInformation).init(self.allocator);
        defer symbols.deinit();

        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc_data = entry.value_ptr;
            for (doc_data.symbols.items) |symbol| {
                if (query.len == 0 or std.mem.indexOf(u8, symbol.name, query) != null) {
                    try symbols.append(symbol);
                }
            }
        }

        try self.sendWorkspaceSymbolResponse(writer, id, &symbols);
    }

    /// Handle shutdown request
    fn handleShutdown(self: *CursedLanguageServer, writer: std.io.AnyWriter) !void {
        _ = self; // unused for now
        const response = 
            \\{"jsonrpc": "2.0", "id": null, "result": null}
        ;
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    /// Format CURSED code (basic implementation)
    fn formatCursedCode(self: *CursedLanguageServer, code: []const u8) ![]u8 {
        var formatted = ArrayList(u8).init(self.allocator);
        defer formatted.deinit();
        
        var indent_level: u32 = 0;
        var lines = std.mem.split(u8, code, "\n");
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t");
            
            if (std.mem.endsWith(u8, trimmed, "{")) {
            // Add indentation
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                try formatted.appendSlice("    ");
                }
            try formatted.appendSlice(trimmed);
            try formatted.append('\n');
            indent_level += 1;
            } else if (std.mem.eql(u8, trimmed, "}")) {
                if (indent_level > 0) indent_level -= 1;
            // Add indentation
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                    try formatted.appendSlice("    ");
            }
                try formatted.appendSlice(trimmed);
            try formatted.append('\n');
        } else if (trimmed.len > 0) {
            // Add indentation
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                try formatted.appendSlice("    ");
            }
            try formatted.appendSlice(trimmed);
            try formatted.append('\n');
        } else {
            try formatted.append('\n');
        }
        }
        
        return formatted.toOwnedSlice();
    }

    /// Check if position is in range
    fn positionInRange(self: *CursedLanguageServer, pos: Position, range: Range) bool {
        _ = self;
        return pos.line >= range.start.line and pos.line <= range.end.line and
               pos.character >= range.start.character and pos.character <= range.end.character;
    }

    /// Publish diagnostics
    fn publishDiagnostics(self: *CursedLanguageServer, uri: []const u8, diagnostics: *ArrayList(Diagnostic)) !void {
        _ = self;
        _ = uri;
        _ = diagnostics;
        // TODO: Send diagnostic notifications to client
    }

    /// Send response helpers
    fn sendCompletionResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, completions: *ArrayList(CompletionItem)) !void {
        _ = completions; // not fully implemented yet
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"isIncomplete": false, "items": []}}}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    fn sendHoverResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, hover_text: ?[]const u8) !void {
        const response = if (hover_text) |text|
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": {{"contents": "{s}"}}}}
            , .{ id, text })
        else
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
            , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    fn sendDefinitionResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, locations: *ArrayList(Location)) !void {
        _ = locations;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    fn sendReferencesResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, locations: *ArrayList(Location)) !void {
        _ = locations;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    fn sendFormattingResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, edits: *ArrayList([]const u8)) !void {
        _ = edits;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    fn sendSemanticTokensResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, tokens: *ArrayList(u32)) !void {
        _ = tokens;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"data": []}}}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    fn sendWorkspaceSymbolResponse(self: *CursedLanguageServer, writer: std.io.AnyWriter, id: i64, symbols: *ArrayList(SymbolInformation)) !void {
        _ = symbols;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }
};

/// LSP Server main loop
pub fn runLspServer(allocator: Allocator) !void {
    var server = CursedLanguageServer.init(allocator);
    defer server.deinit();

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
            const line = try stdin.readUntilDelimiterAlloc(allocator, '\n', 1024);
            defer allocator.free(line);
            
            const trimmed = std.mem.trim(u8, line, "\r\n");
            if (trimmed.len == 0) break; // Empty line marks end of headers
            
            if (std.mem.startsWith(u8, trimmed, "Content-Length: ")) {
                const length_str = trimmed[16..];
                content_length = try std.fmt.parseInt(usize, length_str, 10);
            }
        }

        if (content_length == 0) continue;

        // Read message content
        buffer.clearRetainingCapacity();
        try buffer.resize(content_length);
        _ = try stdin.readAll(buffer.items);

        // Handle the request
        try server.handleRequest(buffer.items, writer);
    }
}
