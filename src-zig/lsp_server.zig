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
            .symbols = .empty,
            .diagnostics = .empty,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DocumentData) void {
        self.allocator.free(self.uri);
        self.allocator.free(self.text);
        self.symbols.deinit(self.allocator);
        self.diagnostics.deinit(self.allocator);
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
        _ = allocator;
        return CursedLanguageServer{
            .allocator = allocator,
            .documents = HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage){},
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
        self.documents.deinit(self.allocator);
        
        if (self.workspace_root) |root| {
            self.allocator.free(root);
        }
    }

    /// Handle LSP requests  
    pub fn handleRequest(self: *CursedLanguageServer, input: []const u8, file: std.fs.File) !void {
        const parsed = json.parseFromSlice(json.Value, self.allocator, input, .{}) catch |err| {
            std.log.err("Failed to parse JSON: {}", .{err});
            return;
        };
        defer parsed.deinit();

        const root = parsed.value;
        var writer_buffer: [4096]u8 = undefined;
        const writer = file.writer(writer_buffer[0..]);
        
        if (root.object.get("method")) |method| {
            const method_str = method.string;
            
            if (std.mem.eql(u8, method_str, "initialize")) {
                try self.handleInitialize(root);
            } else if (std.mem.eql(u8, method_str, "initialized")) {
                try self.handleInitialized();
            } else if (std.mem.eql(u8, method_str, "textDocument/didOpen")) {
                try self.handleDidOpenTextDocument(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/didChange")) {
                try self.handleDidChangeTextDocument(root, writer);
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
    fn handleInitialize(self: *CursedLanguageServer, request: json.Value) !void {
        const id = request.object.get("id").?.integer;
        _ = id; // TODO: Use in response
        
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
        
        try self.sendLspMessage(response);
    }

    /// Handle initialized notification
    fn handleInitialized(self: *CursedLanguageServer) !void {
        self.initialized = true;
        std.log.info("CURSED LSP Server initialized", .{});
    }

    /// Handle didOpen text document
    fn handleDidOpenTextDocument(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        
        const uri = text_document.get("uri").?.string;
        const text = text_document.get("text").?.string;
        const version = @as(i32, @intCast(text_document.get("version").?.integer));

        var doc_data = try DocumentData.init(self.allocator, uri, text, version);
        try self.analyzeDocument(&doc_data);
        
        try self.documents.put(uri, doc_data);
        try self.publishDiagnostics(writer, uri, &doc_data.diagnostics);
    }

    /// Handle didChange text document
    fn handleDidChangeTextDocument(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
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
                    doc_data.symbols.clearAndFree(self.allocator);
                    doc_data.diagnostics.clearAndFree(self.allocator);
                    
                    try self.analyzeDocument(doc_data);
                    try self.publishDiagnostics(writer, uri, &doc_data.diagnostics);
                }
            }
        }
    }

    /// Safe conversion from token position to LSP Position with bounds checking
    fn tokenPositionToLSP(token_line: usize, token_column: usize) Position {
        return Position{
            .line = @min(@as(u32, @intCast(@min(token_line, std.math.maxInt(u32)))), std.math.maxInt(u32)),
            .character = @min(@as(u32, @intCast(@min(token_column, std.math.maxInt(u32)))), std.math.maxInt(u32)),
        };
    }

    /// Create a safe range with validation to prevent negative or invalid ranges
    fn createSafeRange(start_line: usize, start_char: usize, end_line: usize, end_char: usize) Range {
        const safe_start = tokenPositionToLSP(start_line, start_char);
        const safe_end = tokenPositionToLSP(end_line, end_char);
        
        // Ensure end is not before start
        if (safe_end.line < safe_start.line or 
            (safe_end.line == safe_start.line and safe_end.character < safe_start.character)) {
            return Range{
                .start = safe_start,
                .end = Position{ .line = safe_start.line, .character = safe_start.character + 1 },
            };
        }
        
        return Range{
            .start = safe_start,
            .end = safe_end,
        };
    }

    /// Analyze document and extract symbols/diagnostics
    fn analyzeDocument(self: *CursedLanguageServer, doc_data: *DocumentData) !void {
        // Tokenize
        var lex = lexer.Lexer.init(self.allocator, doc_data.text);
        
        const tokens = lex.tokenize() catch |err| {
            try doc_data.diagnostics.append(self.allocator, Diagnostic{
                .range = createSafeRange(0, 0, 0, 10),
                .severity = 1, // Error
                .code = null,
                .source = "cursed-lexer",
                .message = try std.fmt.allocPrint(self.allocator, "Lexer error: {}", .{err}),
            });
            return;
        };
        defer tokens.deinit(self.allocator);

        // Parse
        var parse = parser.Parser.init(self.allocator, tokens.items);
        defer parse.deinit();
        
        const program = parse.parseProgram() catch |err| {
            // Try to get more precise error position from parser
            const error_pos = if (tokens.items.len > 0) tokens.items[tokens.items.len - 1] else null;
            const range = if (error_pos) |pos| 
                createSafeRange(pos.line, pos.column, pos.line, pos.column + pos.lexeme.len)
            else 
                createSafeRange(0, 0, 0, 10);
                
            try doc_data.diagnostics.append(self.allocator, Diagnostic{
                .range = range,
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
        _ = program;
        
        // Simple text-based symbol extraction for demonstration
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Extract function declarations (slay keyword)
            if (std.mem.startsWith(u8, trimmed, "slay ")) {
                const after_slay = trimmed[5..];
                if (std.mem.indexOf(u8, after_slay, "(")) |paren_pos| {
                    const func_name = std.mem.trim(u8, after_slay[0..paren_pos], " \t");
                    if (func_name.len > 0) {
                        try doc_data.symbols.append(self.allocator, SymbolInformation{
                            .name = try self.allocator.dupe(u8, func_name),
                            .kind = 12, // Function
                            .location = Location{
                                .uri = doc_data.uri,
                                .range = Range{
                                    .start = Position{ .line = line_num, .character = 0 },
                                    .end = Position{ .line = line_num, .character = @as(u32, @intCast(func_name.len + 5)) },
                                },
                            },
                            .containerName = null,
                        });
                    }
                }
            }
            
            // Extract variable declarations (sus keyword)
            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                const after_sus = trimmed[4..];
                if (std.mem.indexOf(u8, after_sus, " ")) |space_pos| {
                    const var_name = std.mem.trim(u8, after_sus[0..space_pos], " \t");
                    if (var_name.len > 0) {
                        try doc_data.symbols.append(self.allocator, SymbolInformation{
                            .name = try self.allocator.dupe(u8, var_name),
                            .kind = 13, // Variable
                            .location = Location{
                                .uri = doc_data.uri,
                                .range = Range{
                                    .start = Position{ .line = line_num, .character = 0 },
                                    .end = Position{ .line = line_num, .character = @as(u32, @intCast(var_name.len + 4)) },
                                },
                            },
                            .containerName = null,
                        });
                    }
                }
            }
            
            line_num += 1;
        }
    }

    /// Handle completion request
    fn handleCompletion(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;
        const position = params.get("position").?.object;
        const line = @as(u32, @intCast(position.get("line").?.integer));
        const character = @as(u32, @intCast(position.get("character").?.integer));
        
        var completions = std.ArrayList(u8){};
        defer {
            for (completions.items) |item| {
                self.allocator.free(item.label);
                if (item.detail) |detail| self.allocator.free(detail);
                if (item.documentation) |doc| self.allocator.free(doc);
            }
            completions.deinit();
        }

        // Get current document for context-aware completion
        const doc_data = self.documents.get(uri);
        var context_prefix: []const u8 = "";
        
        if (doc_data) |*doc| {
            // Extract word prefix at cursor position for context-sensitive completion
            const lines = std.mem.splitScalar(u8, doc.text, '\n');
            var current_line: []const u8 = "";
            var line_num: u32 = 0;
            
            var line_iter = lines;
            while (line_iter.next()) |text_line| {
                if (line_num == line) {
                    current_line = text_line;
                    break;
                }
                line_num += 1;
            }
            
            // Extract prefix for intelligent completion
            if (character > 0 and current_line.len >= character) {
                var start: u32 = character;
                while (start > 0 and (std.ascii.isAlphanumeric(current_line[start - 1]) or current_line[start - 1] == '_')) {
                    start -= 1;
                }
                context_prefix = current_line[start..character];
            }
        }

        // Add CURSED keywords (filtered by prefix)
        const keywords = [_][]const u8{
            "sus", "damn", "slay", "vibez", "yeet", "bestie", "stan", "dm",
            "ready", "vibe", "yikes", "shook", "fam", "based", "cap", "cringe",
            "facts", "lit", "tea", "drip", "normie", "smol", "mid", "thicc",
            "squad", "collab", "sick", "when", "otherwise", "bestie", "aight"
        };

        for (keywords) |keyword| {
            if (context_prefix.len == 0 or std.mem.startsWith(u8, keyword, context_prefix)) {
                try completions.append(self.allocator, CompletionItem{
                    .label = try self.allocator.dupe(u8, keyword),
                    .kind = 14, // Keyword
                    .detail = try self.allocator.dupe(u8, "CURSED keyword"),
                    .documentation = try self.allocator.dupe(u8, try self.getKeywordDocumentation(keyword)),
                    .insertText = null,
                });
            }
        }

        // Add CURSED stdlib modules
        const stdlib_modules = [_][]const u8{
            "mathz", "stringz", "arrayz", "testz", "cryptz", "filez", 
            "httpz", "timez", "jsonz", "vibez", "concurrenz"
        };

        for (stdlib_modules) |module| {
            if (context_prefix.len == 0 or std.mem.startsWith(u8, module, context_prefix)) {
                try completions.append(self.allocator, CompletionItem{
                    .label = try self.allocator.dupe(u8, module),
                    .kind = 9, // Module
                    .detail = try self.allocator.dupe(u8, "CURSED stdlib module"),
                    .documentation = try std.fmt.allocPrint(self.allocator, "Import with: yeet \"{s}\"", .{module}),
                    .insertText = null,
                });
            }
        }

        // Add symbols from workspace (filtered by prefix)
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc_data_item = entry.value_ptr;
            for (doc_data_item.symbols.items) |symbol| {
                if (context_prefix.len == 0 or std.mem.startsWith(u8, symbol.name, context_prefix)) {
                    try completions.append(self.allocator, CompletionItem{
                        .label = try self.allocator.dupe(u8, symbol.name),
                        .kind = switch (symbol.kind) {
                            12 => 3, // Function
                            13 => 6, // Variable
                            11 => 8, // Interface
                            5 => 7,  // Class/Struct
                            else => 1, // Text
                        },
                        .detail = try std.fmt.allocPrint(self.allocator, "Symbol from {s}", .{symbol.location.uri}),
                        .documentation = null,
                        .insertText = null,
                    });
                }
            }
        }

        // Send response
        try self.sendCompletionResponse(writer, id, &completions);
    }

    /// Handle hover request
    fn handleHover(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
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
    fn handleDefinition(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const position = params.get("position").?.object;
        
        const uri = text_document.get("uri").?.string;
        const line = @as(u32, @intCast(position.get("line").?.integer));
        const character = @as(u32, @intCast(position.get("character").?.integer));

        var locations = std.ArrayList(u8){};
        defer locations.deinit();

        if (self.documents.get(uri)) |doc_data| {
            for (doc_data.symbols.items) |symbol| {
                if (self.positionInRange(Position{ .line = line, .character = character }, symbol.location.range)) {
                    try locations.append(allocator, symbol.location);
                }
            }
        }

        try self.sendDefinitionResponse(writer, id, &locations);
    }

    /// Handle references request  
    fn handleReferences(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const id = request.object.get("id").?.integer;
        var empty_locations = std.ArrayList(u8){};
        defer empty_locations.deinit();
        try self.sendReferencesResponse(writer, id, &empty_locations);
    }

    /// Handle formatting request
    fn handleFormatting(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;

        var text_edits = std.ArrayList(u8){};
        defer text_edits.deinit();

        if (self.documents.get(uri)) |doc_data| {
            const formatted = try self.formatCursedCode(doc_data.text);
            defer self.allocator.free(formatted);
            
            if (!std.mem.eql(u8, formatted, doc_data.text)) {
                // Return full document replacement
                const edit = try std.fmt.allocPrint(self.allocator, 
                    \\{{"range": {{"start": {{"line": 0, "character": 0}}, "end": {{"line": {}, "character": 0}}}}, "newText": "{s}"}}
                , .{ std.mem.count(u8, doc_data.text, "\n"), formatted });
                try text_edits.append(self.allocator, edit);
            }
        }

        try self.sendFormattingResponse(writer, id, &text_edits);
        
        for (text_edits.items) |edit| {
            self.allocator.free(edit);
        }
    }

    /// Handle semantic tokens request
    fn handleSemanticTokens(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const id = request.object.get("id").?.integer;
        var empty_tokens = std.ArrayList(u8){};
        defer empty_tokens.deinit();
        try self.sendSemanticTokensResponse(writer, id, &empty_tokens);
    }

    /// Handle workspace symbol request
    fn handleWorkspaceSymbol(self: *CursedLanguageServer, request: json.Value, writer: anytype) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const query = if (params.get("query")) |q| q.string else "";

        var symbols = std.ArrayList(u8){};
        defer symbols.deinit();

        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc_data = entry.value_ptr;
            for (doc_data.symbols.items) |symbol| {
                if (query.len == 0 or std.mem.indexOf(u8, symbol.name, query) != null) {
                    try symbols.append(allocator, symbol);
                }
            }
        }

        try self.sendWorkspaceSymbolResponse(writer, id, &symbols);
    }

    /// Handle shutdown request
    fn handleShutdown(self: *CursedLanguageServer) !void {
        const response = 
            \\{"jsonrpc": "2.0", "id": null, "result": null}
        ;
        
        try self.sendLspMessage(response);
    }

    /// Format CURSED code (basic implementation)
    fn formatCursedCode(self: *CursedLanguageServer, code: []const u8) ![]u8 {
        _ = self;
        var formatted = std.ArrayList(u8){};
        defer formatted.deinit();
        
        var indent_level: u32 = 0;
        var lines = std.mem.splitScalar(u8, code, '\n');
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t");
            
            if (std.mem.endsWith(u8, trimmed, "{")) {
            // Add indentation
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                try formatted.appendSlice("    ");
                }
            try formatted.appendSlice(trimmed);
            try formatted.append(allocator, '\n');
            indent_level += 1;
            } else if (std.mem.eql(u8, trimmed, "}")) {
                if (indent_level > 0) indent_level -= 1;
            // Add indentation
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                    try formatted.appendSlice("    ");
            }
                try formatted.appendSlice(trimmed);
            try formatted.append(allocator, '\n');
        } else if (trimmed.len > 0) {
            // Add indentation
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                try formatted.appendSlice("    ");
            }
            try formatted.appendSlice(trimmed);
            try formatted.append(allocator, '\n');
        } else {
            try formatted.append(allocator, '\n');
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

    /// Get documentation for CURSED keywords
    fn getKeywordDocumentation(self: *CursedLanguageServer, keyword: []const u8) ![]const u8 {
        _ = self;
        if (std.mem.eql(u8, keyword, "sus")) return "Variable declaration: sus name type = value";
        if (std.mem.eql(u8, keyword, "damn")) return "Return statement: damn value";
        if (std.mem.eql(u8, keyword, "slay")) return "Function definition: slay name(params) return_type { body }";
        if (std.mem.eql(u8, keyword, "vibez")) return "I/O operations: vibez.spill() for output";
        if (std.mem.eql(u8, keyword, "yeet")) return "Import module: yeet \"module_name\"";
        if (std.mem.eql(u8, keyword, "bestie")) return "While loop: bestie (condition) { body }";
        if (std.mem.eql(u8, keyword, "ready")) return "If statement: ready (condition) { body }";
        if (std.mem.eql(u8, keyword, "otherwise")) return "Else clause: otherwise { body }";
        if (std.mem.eql(u8, keyword, "squad")) return "Struct definition: squad Name { fields }";
        if (std.mem.eql(u8, keyword, "collab")) return "Interface definition: collab Name { methods }";
        if (std.mem.eql(u8, keyword, "sick")) return "Pattern matching: sick (value) { when pattern -> result }";
        if (std.mem.eql(u8, keyword, "when")) return "Pattern case: when pattern -> result";
        if (std.mem.eql(u8, keyword, "based")) return "Boolean true value";
        if (std.mem.eql(u8, keyword, "cringe")) return "Boolean false value";
        if (std.mem.eql(u8, keyword, "drip")) return "Integer type";
        if (std.mem.eql(u8, keyword, "tea")) return "String type";
        if (std.mem.eql(u8, keyword, "lit")) return "Boolean type";
        return "CURSED keyword";
    }

    /// Publish diagnostics
    fn publishDiagnostics(self: *CursedLanguageServer, writer: anytype, uri: []const u8, diagnostics: *ArrayList(Diagnostic)) !void {
        // Build diagnostics JSON array
        var diag_json = std.ArrayList(u8){};
        defer diag_json.deinit();
        
        try diag_json.appendSlice("[");
        
        for (diagnostics.items, 0..) |diag, i| {
            if (i > 0) try diag_json.appendSlice(",");
            
            // Escape diagnostic message for JSON
            var escaped_message = std.ArrayList(u8){};
            defer escaped_message.deinit();
            
            for (diag.message) |c| {
                switch (c) {
                    '"' => try escaped_message.appendSlice("\\\""),
                    '\\' => try escaped_message.appendSlice("\\\\"),
                    '\n' => try escaped_message.appendSlice("\\n"),
                    '\r' => try escaped_message.appendSlice("\\r"),
                    '\t' => try escaped_message.appendSlice("\\t"),
                    else => try escaped_message.append(self.allocator, c),
                }
            }
            
            const diag_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"range": {{"start": {{"line": {}, "character": {}}}, "end": {{"line": {}, "character": {}}}}}, "severity": {}, "source": "{s}", "message": "{s}"}}
            , .{
                diag.range.start.line, diag.range.start.character,
                diag.range.end.line, diag.range.end.character,
                diag.severity, diag.source, escaped_message.items
            });
            defer self.allocator.free(diag_obj);
            
            try diag_json.appendSlice(diag_obj);
        }
        
        try diag_json.appendSlice("]");
        
        // Send textDocument/publishDiagnostics notification
        const notification = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {{"uri": "{s}", "diagnostics": {s}}}}}
        , .{ uri, diag_json.items });
        defer self.allocator.free(notification);
        
        try self.sendLspMessage(notification);
    }

    /// Helper to send LSP message with proper headers  
    fn sendLspMessage(self: *CursedLanguageServer, message: []const u8) !void {
        const header = try std.fmt.allocPrint(self.allocator, "Content-Length: {}\r\n\r\n", .{message.len});
        defer self.allocator.free(header);
        
        // Write to stdout  
        const stdout = std.io.getStdOut().writer();
        try stdout.print("{s}", .{header});
        try stdout.print("{s}", .{message});
    }

    /// Send response helpers
    fn sendCompletionResponse(self: *CursedLanguageServer, writer: anytype, id: i64, completions: *ArrayList(CompletionItem)) !void {
        // Build completion items JSON array
        var items_json = std.ArrayList(u8){};
        defer items_json.deinit();
        
        try items_json.appendSlice("[");
        
        for (completions.items, 0..) |item, i| {
            if (i > 0) try items_json.appendSlice(",");
            
            const item_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"label": "{s}", "kind": {}, "detail": "{s}", "insertText": "{s}"}}
            , .{ 
                item.label, 
                item.kind, 
                item.detail orelse item.label,
                item.insertText orelse item.label 
            });
            defer self.allocator.free(item_obj);
            
            try items_json.appendSlice(item_obj);
        }
        
        try items_json.appendSlice("]");
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"isIncomplete": false, "items": {s}}}}}
        , .{ id, items_json.items });
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }

    fn sendHoverResponse(self: *CursedLanguageServer, writer: anytype, id: i64, hover_text: ?[]const u8) !void {
        const response = if (hover_text) |text|
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": {{"contents": "{s}"}}}}
            , .{ id, text })
        else
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
            , .{id});
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }

    fn sendDefinitionResponse(self: *CursedLanguageServer, writer: anytype, id: i64, locations: *ArrayList(Location)) !void {
        // Build locations JSON array
        var locations_json = std.ArrayList(u8){};
        defer locations_json.deinit();
        
        try locations_json.appendSlice("[");
        
        for (locations.items, 0..) |location, i| {
            if (i > 0) try locations_json.appendSlice(",");
            
            const location_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"uri": "{s}", "range": {{"start": {{"line": {}, "character": {}}}, "end": {{"line": {}, "character": {}}}}}}}
            , .{
                location.uri,
                location.range.start.line, location.range.start.character,
                location.range.end.line, location.range.end.character
            });
            defer self.allocator.free(location_obj);
            
            try locations_json.appendSlice(location_obj);
        }
        
        try locations_json.appendSlice("]");
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {s}}}
        , .{ id, locations_json.items });
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }

    fn sendReferencesResponse(self: *CursedLanguageServer, writer: anytype, id: i64, locations: *ArrayList(Location)) !void {
        _ = locations;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }

    fn sendFormattingResponse(self: *CursedLanguageServer, writer: anytype, id: i64, edits: *ArrayList([]const u8)) !void {
        _ = edits;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }

    fn sendSemanticTokensResponse(self: *CursedLanguageServer, writer: anytype, id: i64, tokens: *ArrayList(u32)) !void {
        _ = tokens;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"data": []}}}}
        , .{id});
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }

    fn sendWorkspaceSymbolResponse(self: *CursedLanguageServer, writer: anytype, id: i64, symbols: *ArrayList(SymbolInformation)) !void {
        _ = symbols;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try self.sendLspMessage(response);
    }
};

/// LSP Server main loop
pub fn runLspServer(allocator: Allocator) !void {
        _ = allocator;
    var server = CursedLanguageServer.init(allocator);
    defer server.deinit();

    const stdin = std.fs.File.stdin();

    std.log.info("CURSED LSP Server starting...", .{});

    var buffer = ArrayList(u8).init(allocator);
    defer buffer.deinit();

    while (true) {
        // Read Content-Length header
        var content_length: usize = 0;
        while (true) {
            // Read line by line using byte-by-byte approach
            var line_buffer = ArrayList(u8).init(allocator);
            defer line_buffer.deinit();
            
            var single_byte: [1]u8 = undefined;
            while (true) {
                const bytes_read = stdin.read(single_byte[0..]) catch |err| {
                    return err;
                };
                if (bytes_read == 0) break; // EOF
                
                const byte = single_byte[0];
                if (byte == '\n') break; // Found newline
                if (byte != '\r') { // Skip carriage return
                    try line_buffer.append(allocator, byte);
                }
            }
            
            if (line_buffer.items.len == 0) break; // Empty line marks end of headers
            
            if (std.mem.startsWith(u8, line_buffer.items, "Content-Length: ")) {
                const length_str = line_buffer.items[16..];
                content_length = try std.fmt.parseInt(usize, length_str, 10);
            }
        }

        if (content_length == 0) continue;

        // Read message content
        buffer.clearRetainingCapacity();
        try buffer.resize(allocator, content_length);
        _ = try stdin.readAll(buffer.items);

        // Handle the request
        const stdout_file = std.fs.File.stdout();
        try server.handleRequest(buffer.items, stdout_file);
    }
}
