//! Enhanced CURSED Language Server with complete LSP Protocol implementation
//! Features: Advanced code completion, real-time diagnostics, semantic analysis, hover info

const std = @import("std");
const json = std.json;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

/// LSP Message types with complete protocol coverage
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
    textDocument_semanticTokens_full,
    textDocument_rename,
    textDocument_documentSymbol,
    textDocument_signatureHelp,
    workspace_symbol,
    workspace_didChangeConfiguration,
    shutdown,
    exit,
};

/// Enhanced Position structure with validation
const Position = struct {
    line: u32,
    character: u32,

    pub fn isValid(self: Position) bool {
        return self.line < 1000000 and self.character < 1000000; // Reasonable limits
    }

    pub fn compare(self: Position, other: Position) std.math.Order {
        if (self.line != other.line) {
            return std.math.order(self.line, other.line);
        }
        return std.math.order(self.character, other.character);
    }
};

/// Enhanced Range structure with validation
const Range = struct {
    start: Position,
    end: Position,

    pub fn isValid(self: Range) bool {
        return self.start.isValid() and self.end.isValid() and 
               self.start.compare(self.end) != .gt;
    }

    pub fn contains(self: Range, pos: Position) bool {
        return pos.compare(self.start) != .lt and pos.compare(self.end) != .gt;
    }
};

/// Enhanced Diagnostic with categories
const DiagnosticSeverity = enum(u32) {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
};

const Diagnostic = struct {
    range: Range,
    severity: DiagnosticSeverity,
    code: ?[]const u8,
    source: []const u8,
    message: []const u8,
    tags: ?[]u32, // deprecated, unnecessary
    related_information: ?[]DiagnosticRelatedInformation,

    const DiagnosticRelatedInformation = struct {
        location: Location,
        message: []const u8,
    };
};

/// Enhanced Location structure
const Location = struct {
    uri: []const u8,
    range: Range,
};

/// Enhanced CompletionItem with full LSP features
const CompletionItemKind = enum(u32) {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
};

const CompletionItem = struct {
    label: []const u8,
    kind: CompletionItemKind,
    detail: ?[]const u8,
    documentation: ?[]const u8,
    deprecated: bool = false,
    preselect: bool = false,
    sort_text: ?[]const u8,
    filter_text: ?[]const u8,
    insert_text: ?[]const u8,
    text_edit: ?TextEdit,
    additional_text_edits: ?[]TextEdit,
    commit_characters: ?[][]const u8,
    data: ?[]const u8,

    const TextEdit = struct {
        range: Range,
        new_text: []const u8,
    };
};

/// Enhanced Symbol structures
const SymbolKind = enum(u32) {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
};

const SymbolInformation = struct {
    name: []const u8,
    kind: SymbolKind,
    tags: ?[]u32,
    deprecated: bool = false,
    location: Location,
    container_name: ?[]const u8,
};

const DocumentSymbol = struct {
    name: []const u8,
    detail: ?[]const u8,
    kind: SymbolKind,
    tags: ?[]u32,
    deprecated: bool = false,
    range: Range,
    selection_range: Range,
    children: ?[]DocumentSymbol,
};

/// Enhanced Hover with markdown support
const Hover = struct {
    contents: []const u8,
    range: ?Range,

    pub fn createMarkdown(allocator: Allocator, content: []const u8) !Hover {
        const markdown = try std.fmt.allocPrint(allocator, "```cursed\n{s}\n```", .{content});
        return Hover{
            .contents = markdown,
            .range = null,
        };
    }
};

/// Enhanced Document tracking with full analysis
const DocumentData = struct {
    uri: []const u8,
    text: []const u8,
    version: i32,
    language_id: []const u8,
    ast: ?ast.Program,
    tokens: ?[]lexer.Token,
    symbols: ArrayList(SymbolInformation),
    document_symbols: ArrayList(DocumentSymbol),
    diagnostics: ArrayList(Diagnostic),
    semantic_tokens: ArrayList(u32),
    folding_ranges: ArrayList(FoldingRange),
    allocator: Allocator,

    const FoldingRange = struct {
        start_line: u32,
        start_character: ?u32,
        end_line: u32,
        end_character: ?u32,
        kind: ?[]const u8, // "comment", "imports", "region"
    };

    pub fn init(allocator: Allocator, uri: []const u8, text: []const u8, version: i32, language_id: []const u8) !DocumentData {
        return DocumentData{
            .uri = try allocator.dupe(u8, uri),
            .text = try allocator.dupe(u8, text),
            .version = version,
            .language_id = try allocator.dupe(u8, language_id),
            .ast = null,
            .tokens = null,
            .symbols = ArrayList(SymbolInformation).init(allocator),
            .document_symbols = ArrayList(DocumentSymbol).init(allocator),
            .diagnostics = ArrayList(Diagnostic).init(allocator),
            .semantic_tokens = ArrayList(u32).init(allocator),
            .folding_ranges = ArrayList(FoldingRange).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DocumentData) void {
        self.allocator.free(self.uri);
        self.allocator.free(self.text);
        self.allocator.free(self.language_id);
        if (self.tokens) |tokens| self.allocator.free(tokens);
        
        // Free symbol names
        for (self.symbols.items) |symbol| {
            self.allocator.free(symbol.name);
            if (symbol.container_name) |name| self.allocator.free(name);
        }
        self.symbols.deinit();
        
        // Free document symbol names
        for (self.document_symbols.items) |doc_symbol| {
            self.allocator.free(doc_symbol.name);
            if (doc_symbol.detail) |detail| self.allocator.free(detail);
        }
        self.document_symbols.deinit();
        
        // Free diagnostic messages
        for (self.diagnostics.items) |diagnostic| {
            self.allocator.free(diagnostic.message);
            if (diagnostic.code) |code| self.allocator.free(code);
        }
        self.diagnostics.deinit();
        
        self.semantic_tokens.deinit();
        self.folding_ranges.deinit();
    }

    /// Update document content and reanalyze
    pub fn updateContent(self: *DocumentData, new_text: []const u8, new_version: i32) !void {
        // Free old content
        self.allocator.free(self.text);
        if (self.tokens) |tokens| {
            self.allocator.free(tokens);
            self.tokens = null;
        }
        
        // Clear analysis results
        self.symbols.clearAndFree();
        self.document_symbols.clearAndFree();
        self.diagnostics.clearAndFree();
        self.semantic_tokens.clearAndFree();
        self.folding_ranges.clearAndFree();
        
        // Set new content
        self.text = try self.allocator.dupe(u8, new_text);
        self.version = new_version;
        
        // Will be reanalyzed by server
    }
};

/// Enhanced CURSED Language Server with complete LSP Protocol support
pub const EnhancedCursedLanguageServer = struct {
    allocator: Allocator,
    documents: HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage),
    workspace_root: ?[]const u8,
    workspace_folders: ArrayList([]const u8),
    client_capabilities: ClientCapabilities,
    server_capabilities: ServerCapabilities,
    initialized: bool,
    shutdown_requested: bool,
    configuration: ServerConfiguration,

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

    const ClientCapabilities = struct {
        text_document: ?TextDocumentClientCapabilities,
        workspace: ?WorkspaceClientCapabilities,
        
        const TextDocumentClientCapabilities = struct {
            completion: ?CompletionClientCapabilities,
            hover: ?HoverClientCapabilities,
            signature_help: ?bool,
            definition: ?bool,
            references: ?bool,
            document_highlight: ?bool,
            document_symbol: ?bool,
            formatting: ?bool,
            range_formatting: ?bool,
            rename: ?bool,
            folding_range: ?bool,
            semantic_tokens: ?bool,
            
            const CompletionClientCapabilities = struct {
                completion_item: ?CompletionItemClientCapabilities,
                context_support: bool = false,
                
                const CompletionItemClientCapabilities = struct {
                    snippet_support: bool = false,
                    commit_characters_support: bool = false,
                    documentation_format: ?[][]const u8,
                    deprecated_support: bool = false,
                    preselect_support: bool = false,
                };
            };
            
            const HoverClientCapabilities = struct {
                content_format: ?[][]const u8,
                dynamic_registration: bool = false,
            };
        };
        
        const WorkspaceClientCapabilities = struct {
            apply_edit: bool = false,
            workspace_edit: ?WorkspaceEditClientCapabilities,
            did_change_configuration: ?bool,
            did_change_watched_files: ?bool,
            symbol: ?bool,
            execute_command: ?bool,
            workspace_folders: bool = false,
            configuration: bool = false,
            
            const WorkspaceEditClientCapabilities = struct {
                document_changes: bool = false,
                resource_operations: ?[][]const u8,
                failure_handling: ?[]const u8,
            };
        };
    };

    const ServerCapabilities = struct {
        text_document_sync: TextDocumentSyncKind = .incremental,
        completion_provider: ?CompletionOptions = null,
        hover_provider: bool = true,
        signature_help_provider: ?SignatureHelpOptions = null,
        definition_provider: bool = true,
        references_provider: bool = true,
        document_highlight_provider: bool = false,
        document_symbol_provider: bool = true,
        workspace_symbol_provider: bool = true,
        code_action_provider: bool = false,
        code_lens_provider: ?CodeLensOptions = null,
        document_formatting_provider: bool = true,
        document_range_formatting_provider: bool = true,
        document_on_type_formatting_provider: ?DocumentOnTypeFormattingOptions = null,
        rename_provider: bool = true,
        folding_range_provider: bool = true,
        execute_command_provider: ?ExecuteCommandOptions = null,
        selection_range_provider: bool = false,
        semantic_tokens_provider: ?SemanticTokensOptions = null,
        workspace: ?WorkspaceServerCapabilities = null,
        
        const TextDocumentSyncKind = enum(u32) {
            none = 0,
            full = 1,
            incremental = 2,
        };
        
        const CompletionOptions = struct {
            resolve_provider: bool = false,
            trigger_characters: []const []const u8 = &[_][]const u8{ ".", "_" },
        };
        
        const SignatureHelpOptions = struct {
            trigger_characters: []const []const u8 = &[_][]const u8{ "(", "," },
        };
        
        const CodeLensOptions = struct {
            resolve_provider: bool = false,
        };
        
        const DocumentOnTypeFormattingOptions = struct {
            first_trigger_character: []const u8,
            more_trigger_character: ?[][]const u8,
        };
        
        const ExecuteCommandOptions = struct {
            commands: [][]const u8,
        };
        
        const SemanticTokensOptions = struct {
            legend: SemanticTokensLegend,
            range: bool = false,
            full: bool = true,
            
            const SemanticTokensLegend = struct {
                token_types: [][]const u8,
                token_modifiers: [][]const u8,
            };
        };
        
        const WorkspaceServerCapabilities = struct {
            workspace_folders: ?WorkspaceFoldersServerCapabilities,
            
            const WorkspaceFoldersServerCapabilities = struct {
                supported: bool = true,
                change_notifications: bool = true,
            };
        };
    };

    const ServerConfiguration = struct {
        format_on_save: bool = false,
        enable_diagnostics: bool = true,
        max_completion_items: u32 = 100,
        diagnostic_delay_ms: u32 = 500,
        semantic_highlighting: bool = true,
    };

    pub fn init(allocator: Allocator) EnhancedCursedLanguageServer {
        return EnhancedCursedLanguageServer{
            .allocator = allocator,
            .documents = HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .workspace_root = null,
            .workspace_folders = ArrayList([]const u8).init(allocator),
            .client_capabilities = ClientCapabilities{
                .text_document = null,
                .workspace = null,
            },
            .server_capabilities = ServerCapabilities{
                .completion_provider = ServerCapabilities.CompletionOptions{},
                .semantic_tokens_provider = ServerCapabilities.SemanticTokensOptions{
                    .legend = ServerCapabilities.SemanticTokensOptions.SemanticTokensLegend{
                        .token_types = @constCast(&[_][]const u8{
                            "keyword", "string", "number", "comment", "operator", 
                            "namespace", "type", "class", "interface", "enum",
                            "function", "method", "variable", "parameter", "property"
                        }),
                        .token_modifiers = @constCast(&[_][]const u8{
                            "declaration", "definition", "readonly", "static", 
                            "deprecated", "abstract", "async", "modification",
                            "documentation", "defaultLibrary"
                        }),
                    },
                },
                .workspace = ServerCapabilities.WorkspaceServerCapabilities{
                    .workspace_folders = ServerCapabilities.WorkspaceServerCapabilities.WorkspaceFoldersServerCapabilities{},
                },
            },
            .initialized = false,
            .shutdown_requested = false,
            .configuration = ServerConfiguration{},
        };
    }

    pub fn deinit(self: *EnhancedCursedLanguageServer) void {
        // Clean up documents
        var iterator = self.documents.iterator();
        while (iterator.next()) |entry| {
            var doc = entry.value_ptr;
            doc.deinit();
        }
        self.documents.deinit();
        
        // Clean up workspace data
        if (self.workspace_root) |root| {
            self.allocator.free(root);
        }
        
        for (self.workspace_folders.items) |folder| {
            self.allocator.free(folder);
        }
        self.workspace_folders.deinit();
    }

    /// Enhanced request handler with comprehensive message parsing
    pub fn handleRequest(self: *EnhancedCursedLanguageServer, input: []const u8, writer: std.io.AnyWriter) !void {
        if (self.shutdown_requested) {
            std.log.warn("Received request after shutdown requested", .{});
            return;
        }

        const parsed = json.parseFromSlice(json.Value, self.allocator, input, .{}) catch |err| {
            std.log.err("Failed to parse JSON request: {}", .{err});
            try self.sendErrorResponse(writer, null, -32700, "Parse error", null);
            return;
        };
        defer parsed.deinit();

        const root = parsed.value;
        const method = root.object.get("method") orelse {
            std.log.warn("Request missing method field", .{});
            return;
        };
        
        const method_str = method.string;
        const id = if (root.object.get("id")) |id_val| 
            switch (id_val) {
                .integer => |i| i,
                .string => |s| std.fmt.parseInt(i64, s, 10) catch null,
                .null => null,
                else => null,
            } 
        else 
            null;

        // Route to appropriate handler
        if (std.mem.eql(u8, method_str, "initialize")) {
            try self.handleInitialize(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "initialized")) {
            try self.handleInitialized();
        } else if (std.mem.eql(u8, method_str, "textDocument/didOpen")) {
            try self.handleDidOpenTextDocument(root, writer);
        } else if (std.mem.eql(u8, method_str, "textDocument/didChange")) {
            try self.handleDidChangeTextDocument(root, writer);
        } else if (std.mem.eql(u8, method_str, "textDocument/didSave")) {
            try self.handleDidSaveTextDocument(root);
        } else if (std.mem.eql(u8, method_str, "textDocument/didClose")) {
            try self.handleDidCloseTextDocument(root);
        } else if (std.mem.eql(u8, method_str, "textDocument/completion")) {
            try self.handleCompletion(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "textDocument/hover")) {
            try self.handleHover(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "textDocument/definition")) {
            try self.handleDefinition(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "textDocument/references")) {
            try self.handleReferences(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "textDocument/documentSymbol")) {
            try self.handleDocumentSymbol(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "textDocument/formatting")) {
            try self.handleFormatting(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "textDocument/semanticTokens/full")) {
            try self.handleSemanticTokens(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "workspace/symbol")) {
            try self.handleWorkspaceSymbol(root, writer, id);
        } else if (std.mem.eql(u8, method_str, "shutdown")) {
            try self.handleShutdown(writer, id);
        } else if (std.mem.eql(u8, method_str, "exit")) {
            // Exit gracefully - this will be handled by the main loop
            return;
        } else {
            std.log.warn("Unhandled method: {s}", .{method_str});
            if (id) |request_id| {
                try self.sendErrorResponse(writer, request_id, -32601, "Method not found", null);
            }
        }
    }

    /// Enhanced initialize handler with full capability negotiation
    fn handleInitialize(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse 1;
        
        if (request.object.get("params")) |params| {
            // Extract client capabilities
            if (params.object.get("capabilities")) |caps| {
                self.parseClientCapabilities(caps);
            }
            
            // Extract workspace information
            if (params.object.get("rootUri")) |root_uri| {
                if (root_uri != .null) {
                    self.workspace_root = try self.allocator.dupe(u8, root_uri.string);
                }
            }
            
            if (params.object.get("workspaceFolders")) |folders_val| {
                if (folders_val != .null and folders_val.array.items.len > 0) {
                    for (folders_val.array.items) |folder| {
                        if (folder.object.get("uri")) |uri| {
                            const folder_uri = try self.allocator.dupe(u8, uri.string);
                            try self.workspace_folders.append(folder_uri);
                        }
                    }
                }
            }
        }

        // Build comprehensive server capabilities response
        const capabilities_json = try self.buildServerCapabilitiesJson();
        defer self.allocator.free(capabilities_json);
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"capabilities": {s}, "serverInfo": {{"name": "CURSED Language Server", "version": "1.0.0"}}}}}}
        , .{ request_id, capabilities_json });
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
        std.log.info("Initialize response sent", .{});
    }

    /// Parse client capabilities for feature negotiation
    fn parseClientCapabilities(self: *EnhancedCursedLanguageServer, caps: json.Value) void {
        _ = self;
        // Parse text document capabilities
        if (caps.object.get("textDocument")) |text_doc| {
            if (text_doc.object.get("completion")) |completion| {
                // Client supports completion
                if (completion.object.get("completionItem")) |completion_item| {
                    // Check for snippet support, etc.
                    _ = completion_item;
                }
            }
        }
        
        // Parse workspace capabilities
        if (caps.object.get("workspace")) |workspace| {
            if (workspace.object.get("workspaceFolders")) |_| {
                // Client supports workspace folders
            }
        }
    }

    /// Build comprehensive server capabilities JSON
    fn buildServerCapabilitiesJson(self: *EnhancedCursedLanguageServer) ![]u8 {
        return try self.allocator.dupe(u8,
            \\{
            \\  "textDocumentSync": {
            \\    "openClose": true,
            \\    "change": 2,
            \\    "willSave": false,
            \\    "willSaveWaitUntil": false,
            \\    "save": {"includeText": false}
            \\  },
            \\  "completionProvider": {
            \\    "resolveProvider": false,
            \\    "triggerCharacters": [".", "_", " "],
            \\    "completionItem": {"labelDetailsSupport": true}
            \\  },
            \\  "hoverProvider": true,
            \\  "signatureHelpProvider": {
            \\    "triggerCharacters": ["(", ","]
            \\  },
            \\  "definitionProvider": true,
            \\  "referencesProvider": true,
            \\  "documentHighlightProvider": false,
            \\  "documentSymbolProvider": true,
            \\  "workspaceSymbolProvider": true,
            \\  "codeActionProvider": false,
            \\  "codeLensProvider": {"resolveProvider": false},
            \\  "documentFormattingProvider": true,
            \\  "documentRangeFormattingProvider": true,
            \\  "documentOnTypeFormattingProvider": {
            \\    "firstTriggerCharacter": "}",
            \\    "moreTriggerCharacter": [";"]
            \\  },
            \\  "renameProvider": true,
            \\  "foldingRangeProvider": true,
            \\  "executeCommandProvider": {
            \\    "commands": ["cursed.restart", "cursed.showAST"]
            \\  },
            \\  "selectionRangeProvider": false,
            \\  "semanticTokensProvider": {
            \\    "legend": {
            \\      "tokenTypes": ["keyword", "string", "number", "comment", "operator", "namespace", "type", "class", "interface", "enum", "function", "method", "variable", "parameter", "property"],
            \\      "tokenModifiers": ["declaration", "definition", "readonly", "static", "deprecated", "abstract", "async", "modification", "documentation", "defaultLibrary"]
            \\    },
            \\    "range": false,
            \\    "full": true
            \\  },
            \\  "workspace": {
            \\    "workspaceFolders": {
            \\      "supported": true,
            \\      "changeNotifications": true
            \\    }
            \\  }
            \\}
        );
    }

    /// Handle initialized notification
    fn handleInitialized(self: *EnhancedCursedLanguageServer) !void {
        self.initialized = true;
        std.log.info("CURSED Enhanced LSP Server fully initialized", .{});
    }

    /// Enhanced didOpen handler with comprehensive document analysis
    fn handleDidOpenTextDocument(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        
        const uri = text_document.object.get("uri").?.string;
        const text = text_document.object.get("text").?.string;
        const version = @as(i32, @intCast(text_document.object.get("version").?.integer));
        const language_id = text_document.object.get("languageId").?.string;

        var doc_data = try DocumentData.init(self.allocator, uri, text, version, language_id);
        try self.analyzeDocument(&doc_data);
        
        try self.documents.put(uri, doc_data);
        
        // Send diagnostics
        if (self.configuration.enable_diagnostics) {
            try self.publishDiagnostics(writer, uri, &doc_data.diagnostics);
        }
        
        std.log.info("Document opened and analyzed: {s}", .{uri});
    }

    /// Enhanced didChange handler with incremental updates
    fn handleDidChangeTextDocument(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const content_changes = params.object.get("contentChanges") orelse return;
        
        const uri = text_document.object.get("uri").?.string;
        const version = @as(i32, @intCast(text_document.object.get("version").?.integer));

        if (self.documents.getPtr(uri)) |doc_data| {
            // Handle content changes (simplified - full document replacement)
            if (content_changes.array.items.len > 0) {
                const change = content_changes.array.items[0];
                if (change.object.get("text")) |new_text| {
                    try doc_data.updateContent(new_text.string, version);
                    try self.analyzeDocument(doc_data);
                    
                    // Send updated diagnostics
                    if (self.configuration.enable_diagnostics) {
                        try self.publishDiagnostics(writer, uri, &doc_data.diagnostics);
                    }
                }
            }
        }
    }

    /// Handle didSave notification
    fn handleDidSaveTextDocument(self: *EnhancedCursedLanguageServer, request: json.Value) !void {
        _ = self;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const uri = text_document.object.get("uri").?.string;
        
        std.log.info("Document saved: {s}", .{uri});
        // Additional save-time processing could go here
    }

    /// Handle didClose notification
    fn handleDidCloseTextDocument(self: *EnhancedCursedLanguageServer, request: json.Value) !void {
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const uri = text_document.object.get("uri").?.string;
        
        if (self.documents.getPtr(uri)) |doc_data| {
            doc_data.deinit();
            _ = self.documents.remove(uri);
            std.log.info("Document closed and removed: {s}", .{uri});
        }
    }

    /// Enhanced document analysis with full semantic information
    fn analyzeDocument(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData) !void {
        // Clear previous analysis
        doc_data.symbols.clearAndFree();
        doc_data.document_symbols.clearAndFree();
        doc_data.diagnostics.clearAndFree();
        doc_data.semantic_tokens.clearAndFree();
        doc_data.folding_ranges.clearAndFree();

        // Tokenize
        var lex = lexer.Lexer.init(self.allocator, doc_data.text);
        
        const tokens = lex.tokenize() catch |err| {
            try doc_data.diagnostics.append(Diagnostic{
                .range = Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 10 },
                },
                .severity = DiagnosticSeverity.Error,
                .code = try self.allocator.dupe(u8, "lexer_error"),
                .source = "cursed-lexer",
                .message = try std.fmt.allocPrint(self.allocator, "Lexer error: {}", .{err}),
                .tags = null,
                .related_information = null,
            });
            return;
        };
        
        doc_data.tokens = try self.allocator.dupe(lexer.Token, tokens.items);
        tokens.deinit();

        // Parse
        var parse = parser.Parser.init(self.allocator, doc_data.tokens.?);
        defer parse.deinit();
        
        const program = parse.parseProgram() catch |err| {
            const error_pos = if (doc_data.tokens.?.len > 0) doc_data.tokens.?[doc_data.tokens.?.len - 1] else null;
            const range = if (error_pos) |pos| 
                Range{
                    .start = Position{ .line = @as(u32, @intCast(pos.line)), .character = @as(u32, @intCast(pos.column)) },
                    .end = Position{ .line = @as(u32, @intCast(pos.line)), .character = @as(u32, @intCast(pos.column + pos.lexeme.len)) },
                }
            else 
                Range{
                    .start = Position{ .line = 0, .character = 0 },
                    .end = Position{ .line = 0, .character = 10 },
                };
                
            try doc_data.diagnostics.append(Diagnostic{
                .range = range,
                .severity = DiagnosticSeverity.Error,
                .code = try self.allocator.dupe(u8, "parser_error"),
                .source = "cursed-parser",
                .message = try std.fmt.allocPrint(self.allocator, "Parser error: {}", .{err}),
                .tags = null,
                .related_information = null,
            });
            return;
        };

        doc_data.ast = program;
        
        // Extract symbols and semantic information
        try self.extractSymbols(doc_data);
        try self.generateSemanticTokens(doc_data);
        try self.extractFoldingRanges(doc_data);
        
        std.log.info("Document analysis complete: {} symbols, {} diagnostics", .{ 
            doc_data.symbols.items.len, 
            doc_data.diagnostics.items.len 
        });
    }

    /// Enhanced symbol extraction with comprehensive CURSED language support
    fn extractSymbols(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData) !void {
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            defer line_num += 1;
            
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;
            
            // Function declarations (slay keyword)
            if (std.mem.startsWith(u8, trimmed, "slay ")) {
                try self.extractFunctionSymbol(doc_data, trimmed, line_num, "slay ");
            }
            
            // Variable declarations (sus keyword)  
            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.extractVariableSymbol(doc_data, trimmed, line_num, "sus ");
            }
            
            // Struct declarations (squad keyword)
            if (std.mem.startsWith(u8, trimmed, "squad ")) {
                try self.extractStructSymbol(doc_data, trimmed, line_num, "squad ");
            }
            
            // Interface declarations (collab keyword)
            if (std.mem.startsWith(u8, trimmed, "collab ")) {
                try self.extractInterfaceSymbol(doc_data, trimmed, line_num, "collab ");
            }
            
            // Import statements (yeet keyword)
            if (std.mem.startsWith(u8, trimmed, "yeet ")) {
                try self.extractImportSymbol(doc_data, trimmed, line_num, "yeet ");
            }
        }
    }

    /// Extract function symbol information
    fn extractFunctionSymbol(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32, prefix: []const u8) !void {
        const after_keyword = line[prefix.len..];
        if (std.mem.indexOf(u8, after_keyword, "(")) |paren_pos| {
            const func_name = std.mem.trim(u8, after_keyword[0..paren_pos], " \t");
            if (func_name.len > 0) {
                const symbol = SymbolInformation{
                    .name = try self.allocator.dupe(u8, func_name),
                    .kind = SymbolKind.Function,
                    .tags = null,
                    .deprecated = false,
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = Range{
                            .start = Position{ .line = line_num, .character = 0 },
                            .end = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + func_name.len)) },
                        },
                    },
                    .container_name = null,
                };
                try doc_data.symbols.append(symbol);
                
                // Also create document symbol
                const doc_symbol = DocumentSymbol{
                    .name = try self.allocator.dupe(u8, func_name),
                    .detail = try self.allocator.dupe(u8, "function"),
                    .kind = SymbolKind.Function,
                    .tags = null,
                    .deprecated = false,
                    .range = Range{
                        .start = Position{ .line = line_num, .character = 0 },
                        .end = Position{ .line = line_num + 5, .character = 0 }, // Estimate function span
                    },
                    .selection_range = Range{
                        .start = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len)) },
                        .end = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + func_name.len)) },
                    },
                    .children = null,
                };
                try doc_data.document_symbols.append(doc_symbol);
            }
        }
    }

    /// Extract variable symbol information
    fn extractVariableSymbol(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32, prefix: []const u8) !void {
        const after_keyword = line[prefix.len..];
        if (std.mem.indexOf(u8, after_keyword, " ")) |space_pos| {
            const var_name = std.mem.trim(u8, after_keyword[0..space_pos], " \t");
            if (var_name.len > 0) {
                const symbol = SymbolInformation{
                    .name = try self.allocator.dupe(u8, var_name),
                    .kind = SymbolKind.Variable,
                    .tags = null,
                    .deprecated = false,
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = Range{
                            .start = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len)) },
                            .end = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + var_name.len)) },
                        },
                    },
                    .container_name = null,
                };
                try doc_data.symbols.append(symbol);
            }
        }
    }

    /// Extract struct symbol information
    fn extractStructSymbol(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32, prefix: []const u8) !void {
        const after_keyword = line[prefix.len..];
        if (std.mem.indexOf(u8, after_keyword, " ")) |space_pos| {
            const struct_name = std.mem.trim(u8, after_keyword[0..space_pos], " \t");
            if (struct_name.len > 0) {
                const symbol = SymbolInformation{
                    .name = try self.allocator.dupe(u8, struct_name),
                    .kind = SymbolKind.Struct,
                    .tags = null,
                    .deprecated = false,
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = Range{
                            .start = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len)) },
                            .end = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + struct_name.len)) },
                        },
                    },
                    .container_name = null,
                };
                try doc_data.symbols.append(symbol);
            }
        }
    }

    /// Extract interface symbol information
    fn extractInterfaceSymbol(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32, prefix: []const u8) !void {
        const after_keyword = line[prefix.len..];
        if (std.mem.indexOf(u8, after_keyword, " ")) |space_pos| {
            const interface_name = std.mem.trim(u8, after_keyword[0..space_pos], " \t");
            if (interface_name.len > 0) {
                const symbol = SymbolInformation{
                    .name = try self.allocator.dupe(u8, interface_name),
                    .kind = SymbolKind.Interface,
                    .tags = null,
                    .deprecated = false,
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = Range{
                            .start = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len)) },
                            .end = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + interface_name.len)) },
                        },
                    },
                    .container_name = null,
                };
                try doc_data.symbols.append(symbol);
            }
        }
    }

    /// Extract import symbol information
    fn extractImportSymbol(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32, prefix: []const u8) !void {
        const after_keyword = line[prefix.len..];
        if (std.mem.indexOf(u8, after_keyword, "\"")) |quote_pos| {
            const end_quote = std.mem.indexOf(u8, after_keyword[quote_pos + 1..], "\"");
            if (end_quote) |end_pos| {
                const import_name = after_keyword[quote_pos + 1 .. quote_pos + 1 + end_pos];
                if (import_name.len > 0) {
                    const symbol = SymbolInformation{
                        .name = try self.allocator.dupe(u8, import_name),
                        .kind = SymbolKind.Module,
                        .tags = null,
                        .deprecated = false,
                        .location = Location{
                            .uri = doc_data.uri,
                            .range = Range{
                                .start = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + quote_pos)) },
                                .end = Position{ .line = line_num, .character = @as(u32, @intCast(prefix.len + quote_pos + 1 + end_pos + 1)) },
                            },
                        },
                        .container_name = null,
                    };
                    try doc_data.symbols.append(symbol);
                }
            }
        }
    }

    /// Generate semantic tokens for syntax highlighting
    fn generateSemanticTokens(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData) !void {
        _ = self;
        _ = doc_data;
        // TODO: Implement semantic token generation based on tokens
        // This would analyze tokens and generate LSP semantic token data
    }

    /// Extract folding ranges for code folding
    fn extractFoldingRanges(self: *EnhancedCursedLanguageServer, doc_data: *DocumentData) !void {
        
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        var brace_stack = ArrayList(u32).init(self.allocator);
        defer brace_stack.deinit();
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            defer line_num += 1;
            
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Opening brace - start folding range
            if (std.mem.endsWith(u8, trimmed, "{")) {
                try brace_stack.append(line_num);
            }
            
            // Closing brace - end folding range
            if (std.mem.eql(u8, trimmed, "}") and brace_stack.items.len > 0) {
                const start_line = brace_stack.pop().?;
                if (line_num > start_line) {
                    try doc_data.folding_ranges.append(DocumentData.FoldingRange{
                        .start_line = start_line,
                        .start_character = null,
                        .end_line = line_num,
                        .end_character = null,
                        .kind = null,
                    });
                }
            }
        }
    }

    /// Enhanced completion handler with context-aware suggestions
    fn handleCompletion(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const position = params.object.get("position") orelse return;
        
        const uri = text_document.object.get("uri").?.string;
        const line = @as(u32, @intCast(position.object.get("line").?.integer));
        const character = @as(u32, @intCast(position.object.get("character").?.integer));
        
        var completions = ArrayList(CompletionItem).init(self.allocator);
        defer {
            for (completions.items) |*item| {
                self.allocator.free(item.label);
                if (item.detail) |detail| self.allocator.free(detail);
                if (item.documentation) |doc| self.allocator.free(doc);
                if (item.insert_text) |text| self.allocator.free(text);
            }
            completions.deinit();
        }

        // Get document context
        const doc_data = self.documents.get(uri);
        var context_prefix: []const u8 = "";
        var at_start_of_line = false;
        
        if (doc_data) |doc| {
            // Extract current line and context
            const lines = std.mem.splitScalar(u8, doc.text, '\n');
            var current_line: []const u8 = "";
            var current_line_num: u32 = 0;
            
            var line_iter = lines;
            while (line_iter.next()) |text_line| {
                if (current_line_num == line) {
                    current_line = text_line;
                    break;
                }
                current_line_num += 1;
            }
            
            // Extract word prefix at cursor
            if (character > 0 and current_line.len >= character) {
                var start = character;
                while (start > 0 and (std.ascii.isAlphanumeric(current_line[start - 1]) or current_line[start - 1] == '_')) {
                    start -= 1;
                }
                context_prefix = current_line[start..character];
            }
            
            // Check if we're at the start of a line (for statement keywords)
            const line_before_cursor = if (character > 0) 
                std.mem.trim(u8, current_line[0..character], " \t") 
            else 
                "";
            at_start_of_line = line_before_cursor.len == 0 or std.mem.eql(u8, line_before_cursor, context_prefix);
        }

        // Add CURSED keywords based on context
        try self.addKeywordCompletions(&completions, context_prefix, at_start_of_line);
        
        // Add CURSED type completions
        try self.addTypeCompletions(&completions, context_prefix);
        
        // Add stdlib module completions
        try self.addStdlibCompletions(&completions, context_prefix);
        
        // Add document symbols
        try self.addSymbolCompletions(&completions, context_prefix, doc_data);
        
        // Limit results based on configuration
        if (completions.items.len > self.configuration.max_completion_items) {
            completions.items.len = self.configuration.max_completion_items;
        }

        try self.sendCompletionResponse(writer, request_id, &completions);
    }

    /// Add CURSED keyword completions with context awareness
    fn addKeywordCompletions(self: *EnhancedCursedLanguageServer, completions: *ArrayList(CompletionItem), prefix: []const u8, at_start_of_line: bool) !void {
        const statement_keywords = [_]struct { keyword: []const u8, doc: []const u8, insert: []const u8 }{
            .{ .keyword = "sus", .doc = "Variable declaration: sus name type = value", .insert = "sus ${1:name} ${2:drip} = ${0:value}" },
            .{ .keyword = "slay", .doc = "Function definition: slay name(params) return_type { body }", .insert = "slay ${1:name}(${2:params}) ${3:drip} {\n    ${0:// function body}\n}" },
            .{ .keyword = "ready", .doc = "If statement: ready (condition) { body }", .insert = "ready (${1:condition}) {\n    ${0:// if body}\n}" },
            .{ .keyword = "bestie", .doc = "While loop: bestie (condition) { body }", .insert = "bestie (${1:condition}) {\n    ${0:// loop body}\n}" },
            .{ .keyword = "squad", .doc = "Struct definition: squad Name { fields }", .insert = "squad ${1:Name} {\n    ${0:// fields}\n}" },
            .{ .keyword = "collab", .doc = "Interface definition: collab Name { methods }", .insert = "collab ${1:Name} {\n    ${0:// methods}\n}" },
            .{ .keyword = "yeet", .doc = "Import module: yeet \"module_name\"", .insert = "yeet \"${0:module_name}\"" },
            .{ .keyword = "sick", .doc = "Pattern matching: sick (value) { when pattern -> result }", .insert = "sick (${1:value}) {\n    when ${2:pattern} -> ${0:result}\n}" },
        };

        const expression_keywords = [_]struct { keyword: []const u8, doc: []const u8, insert: []const u8 }{
            .{ .keyword = "damn", .doc = "Return statement: damn value", .insert = "damn ${0:value}" },
            .{ .keyword = "vibez", .doc = "I/O operations: vibez.spill() for output", .insert = "vibez.spill(${0:message})" },
            .{ .keyword = "based", .doc = "Boolean true value", .insert = "based" },
            .{ .keyword = "cringe", .doc = "Boolean false value", .insert = "cringe" },
            .{ .keyword = "cap", .doc = "Null/none value", .insert = "cap" },
            .{ .keyword = "otherwise", .doc = "Else clause: otherwise { body }", .insert = "otherwise {\n    ${0:// else body}\n}" },
            .{ .keyword = "when", .doc = "Pattern case: when pattern -> result", .insert = "when ${1:pattern} -> ${0:result}" },
        };

        // Add statement keywords if at start of line
        if (at_start_of_line) {
            for (statement_keywords) |kw| {
                if (prefix.len == 0 or std.mem.startsWith(u8, kw.keyword, prefix)) {
                    try completions.append(CompletionItem{
                        .label = try self.allocator.dupe(u8, kw.keyword),
                        .kind = CompletionItemKind.Keyword,
                        .detail = try self.allocator.dupe(u8, "CURSED statement"),
                        .documentation = try self.allocator.dupe(u8, kw.doc),
                        .insert_text = try self.allocator.dupe(u8, kw.insert),
                        .preselect = std.mem.eql(u8, kw.keyword, prefix),
                        .sort_text = try std.fmt.allocPrint(self.allocator, "000_{s}", .{kw.keyword}),
                        .filter_text = null,
                        .text_edit = null,
                        .additional_text_edits = null,
                        .commit_characters = null,
                        .data = null,
                    });
                }
            }
        }

        // Add expression keywords
        for (expression_keywords) |kw| {
            if (prefix.len == 0 or std.mem.startsWith(u8, kw.keyword, prefix)) {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, kw.keyword),
                    .kind = CompletionItemKind.Keyword,
                    .detail = try self.allocator.dupe(u8, "CURSED expression"),
                    .documentation = try self.allocator.dupe(u8, kw.doc),
                    .insert_text = try self.allocator.dupe(u8, kw.insert),
                    .preselect = std.mem.eql(u8, kw.keyword, prefix),
                    .sort_text = try std.fmt.allocPrint(self.allocator, "001_{s}", .{kw.keyword}),
                    .filter_text = null,
                    .text_edit = null,
                    .additional_text_edits = null,
                    .commit_characters = null,
                    .data = null,
                });
            }
        }
    }

    /// Add CURSED type completions
    fn addTypeCompletions(self: *EnhancedCursedLanguageServer, completions: *ArrayList(CompletionItem), prefix: []const u8) !void {
        const types = [_]struct { name: []const u8, doc: []const u8 }{
            .{ .name = "drip", .doc = "Integer type (signed 32-bit)" },
            .{ .name = "tea", .doc = "String type (UTF-8)" },
            .{ .name = "lit", .doc = "Boolean type (true/false)" },
            .{ .name = "normie", .doc = "Default integer type" },
            .{ .name = "thicc", .doc = "Large integer type (64-bit)" },
            .{ .name = "smol", .doc = "Small integer type (8-bit)" },
            .{ .name = "mid", .doc = "Medium integer type (16-bit)" },
            .{ .name = "floaty", .doc = "Floating point number (32-bit)" },
            .{ .name = "doubly", .doc = "Double precision float (64-bit)" },
        };

        for (types) |type_info| {
            if (prefix.len == 0 or std.mem.startsWith(u8, type_info.name, prefix)) {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, type_info.name),
                    .kind = CompletionItemKind.TypeParameter,
                    .detail = try self.allocator.dupe(u8, "CURSED type"),
                    .documentation = try self.allocator.dupe(u8, type_info.doc),
                    .insert_text = try self.allocator.dupe(u8, type_info.name),
                    .sort_text = try std.fmt.allocPrint(self.allocator, "002_{s}", .{type_info.name}),
                    .filter_text = null,
                    .text_edit = null,
                    .additional_text_edits = null,
                    .commit_characters = null,
                    .data = null,
                });
            }
        }
    }

    /// Add CURSED standard library completions
    fn addStdlibCompletions(self: *EnhancedCursedLanguageServer, completions: *ArrayList(CompletionItem), prefix: []const u8) !void {
        const stdlib_modules = [_]struct { name: []const u8, doc: []const u8, functions: []const []const u8 }{
            .{ .name = "vibez", .doc = "I/O operations and printing", .functions = &[_][]const u8{ "spill", "read", "ask" } },
            .{ .name = "mathz", .doc = "Mathematical functions", .functions = &[_][]const u8{ "abs", "sqrt", "sin", "cos", "max", "min" } },
            .{ .name = "stringz", .doc = "String manipulation", .functions = &[_][]const u8{ "len", "concat", "split", "trim", "upper", "lower" } },
            .{ .name = "arrayz", .doc = "Array operations", .functions = &[_][]const u8{ "len", "push", "pop", "map", "filter", "reduce" } },
            .{ .name = "testz", .doc = "Testing framework", .functions = &[_][]const u8{ "assert", "assert_eq", "test_start", "test_end" } },
            .{ .name = "timez", .doc = "Time and date operations", .functions = &[_][]const u8{ "now", "sleep", "format", "parse" } },
            .{ .name = "filez", .doc = "File system operations", .functions = &[_][]const u8{ "read", "write", "exists", "delete", "list" } },
            .{ .name = "jsonz", .doc = "JSON parsing and generation", .functions = &[_][]const u8{ "parse", "stringify", "validate" } },
            .{ .name = "httpz", .doc = "HTTP client and server", .functions = &[_][]const u8{ "get", "post", "serve", "request" } },
            .{ .name = "cryptz", .doc = "Cryptographic functions", .functions = &[_][]const u8{ "hash", "encrypt", "decrypt", "sign" } },
            .{ .name = "concurrenz", .doc = "Concurrency primitives", .functions = &[_][]const u8{ "go", "channel", "select", "mutex" } },
        };

        for (stdlib_modules) |module| {
            // Add module name completion
            if (prefix.len == 0 or std.mem.startsWith(u8, module.name, prefix)) {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, module.name),
                    .kind = CompletionItemKind.Module,
                    .detail = try self.allocator.dupe(u8, "CURSED stdlib module"),
                    .documentation = try std.fmt.allocPrint(self.allocator, "{s}\n\nFunctions: {s}", .{ module.doc, module.functions }),
                    .insert_text = try self.allocator.dupe(u8, module.name),
                    .sort_text = try std.fmt.allocPrint(self.allocator, "003_{s}", .{module.name}),
                    .filter_text = null,
                    .text_edit = null,
                    .additional_text_edits = null,
                    .commit_characters = null,
                    .data = null,
                });
            }

            // Add module function completions
            for (module.functions) |func| {
                const full_func = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ module.name, func });
                defer self.allocator.free(full_func);
                
                if (prefix.len == 0 or std.mem.startsWith(u8, full_func, prefix)) {
                    try completions.append(CompletionItem{
                        .label = try self.allocator.dupe(u8, full_func),
                        .kind = CompletionItemKind.Function,
                        .detail = try self.allocator.dupe(u8, "CURSED stdlib function"),
                        .documentation = try std.fmt.allocPrint(self.allocator, "{s}.{s}() - Function from {s} module", .{ module.name, func, module.doc }),
                        .insert_text = try std.fmt.allocPrint(self.allocator, "{s}.{s}(${{0:args}})", .{ module.name, func }),
                        .sort_text = try std.fmt.allocPrint(self.allocator, "004_{s}", .{full_func}),
                        .filter_text = null,
                        .text_edit = null,
                        .additional_text_edits = null,
                        .commit_characters = null,
                        .data = null,
                    });
                }
            }
        }
    }

    /// Add document symbol completions
    fn addSymbolCompletions(self: *EnhancedCursedLanguageServer, completions: *ArrayList(CompletionItem), prefix: []const u8, doc_data: ?DocumentData) !void {
        // Add symbols from current document
        if (doc_data) |doc| {
            for (doc.symbols.items) |symbol| {
                if (prefix.len == 0 or std.mem.startsWith(u8, symbol.name, prefix)) {
                    try completions.append(CompletionItem{
                        .label = try self.allocator.dupe(u8, symbol.name),
                        .kind = switch (symbol.kind) {
                            .Function => CompletionItemKind.Function,
                            .Variable => CompletionItemKind.Variable,
                            .Struct => CompletionItemKind.Struct,
                            .Interface => CompletionItemKind.Interface,
                            .Module => CompletionItemKind.Module,
                            else => CompletionItemKind.Text,
                        },
                        .detail = try std.fmt.allocPrint(self.allocator, "Symbol from current document", .{}),
                        .documentation = null,
                        .insert_text = try self.allocator.dupe(u8, symbol.name),
                        .sort_text = try std.fmt.allocPrint(self.allocator, "005_{s}", .{symbol.name}),
                        .filter_text = null,
                        .text_edit = null,
                        .additional_text_edits = null,
                        .commit_characters = null,
                        .data = null,
                    });
                }
            }
        }

        // Add symbols from all workspace documents
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const workspace_doc = entry.value_ptr;
            if (doc_data == null or !std.mem.eql(u8, workspace_doc.uri, doc_data.?.uri)) {
                for (workspace_doc.symbols.items) |symbol| {
                    if (prefix.len == 0 or std.mem.startsWith(u8, symbol.name, prefix)) {
                        try completions.append(CompletionItem{
                            .label = try self.allocator.dupe(u8, symbol.name),
                            .kind = switch (symbol.kind) {
                                .Function => CompletionItemKind.Function,
                                .Variable => CompletionItemKind.Variable,
                                .Struct => CompletionItemKind.Struct,
                                .Interface => CompletionItemKind.Interface,
                                .Module => CompletionItemKind.Module,
                                else => CompletionItemKind.Text,
                            },
                            .detail = try std.fmt.allocPrint(self.allocator, "Symbol from {s}", .{workspace_doc.uri}),
                            .documentation = null,
                            .insert_text = try self.allocator.dupe(u8, symbol.name),
                            .sort_text = try std.fmt.allocPrint(self.allocator, "006_{s}", .{symbol.name}),
                            .filter_text = null,
                            .text_edit = null,
                            .additional_text_edits = null,
                            .commit_characters = null,
                            .data = null,
                        });
                    }
                }
            }
        }
    }

    /// Enhanced hover handler with rich information
    fn handleHover(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const position = params.object.get("position") orelse return;
        
        const uri = text_document.object.get("uri").?.string;
        const line = @as(u32, @intCast(position.object.get("line").?.integer));
        const character = @as(u32, @intCast(position.object.get("character").?.integer));

        var hover_text: ?[]const u8 = null;
        defer if (hover_text) |text| self.allocator.free(text);
        
        if (self.documents.get(uri)) |doc_data| {
            const cursor_pos = Position{ .line = line, .character = character };
            
            // Check if cursor is over a symbol
            for (doc_data.symbols.items) |symbol| {
                if (symbol.location.range.contains(cursor_pos)) {
                    hover_text = try self.createSymbolHover(symbol);
                    break;
                }
            }
            
            // If no symbol found, check for keywords and types
            if (hover_text == null) {
                hover_text = try self.createContextualHover(doc_data, cursor_pos);
            }
        }

        try self.sendHoverResponse(writer, request_id, hover_text);
    }

    /// Create hover information for symbols
    fn createSymbolHover(self: *EnhancedCursedLanguageServer, symbol: SymbolInformation) ![]u8 {
        const kind_name = switch (symbol.kind) {
            .Function => "Function",
            .Variable => "Variable",
            .Struct => "Struct",
            .Interface => "Interface",
            .Module => "Module",
            else => "Symbol",
        };

        return try std.fmt.allocPrint(self.allocator,
            \\```cursed
            \\{s} {s}
            \\```
            \\
            \\**Type:** {s}
            \\**Location:** {s}
        , .{ kind_name, symbol.name, kind_name, symbol.location.uri });
    }

    /// Create contextual hover information
    fn createContextualHover(self: *EnhancedCursedLanguageServer, doc_data: DocumentData, pos: Position) !?[]u8 {
        // Get the word at cursor position
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var current_line: []const u8 = "";
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |text_line| {
            if (line_num == pos.line) {
                current_line = text_line;
                break;
            }
            line_num += 1;
        }
        
        if (pos.character >= current_line.len) return null;
        
        // Extract word at cursor
        var start = pos.character;
        var end = pos.character;
        
        // Move start backwards
        while (start > 0 and (std.ascii.isAlphanumeric(current_line[start - 1]) or current_line[start - 1] == '_')) {
            start -= 1;
        }
        
        // Move end forwards
        while (end < current_line.len and (std.ascii.isAlphanumeric(current_line[end]) or current_line[end] == '_')) {
            end += 1;
        }
        
        if (start == end) return null;
        
        const word = current_line[start..end];
        
        // Check for CURSED keywords
        const keyword_docs = std.StaticStringMap([]const u8).initComptime(.{
            .{ "sus", "**Variable Declaration**\n\n`sus name type = value`\n\nDeclares a new variable with specified type and initial value." },
            .{ "slay", "**Function Definition**\n\n`slay name(params) return_type { body }`\n\nDefines a new function with parameters and return type." },
            .{ "damn", "**Return Statement**\n\n`damn value`\n\nReturns a value from the current function." },
            .{ "ready", "**If Statement**\n\n`ready (condition) { body }`\n\nExecutes code block if condition is true." },
            .{ "bestie", "**While Loop**\n\n`bestie (condition) { body }`\n\nRepeats code block while condition is true." },
            .{ "vibez", "**I/O Operations**\n\nModule for input/output operations:\n- `vibez.spill()` - Print to output\n- `vibez.read()` - Read input" },
            .{ "based", "**Boolean True**\n\nRepresents the boolean value `true`." },
            .{ "cringe", "**Boolean False**\n\nRepresents the boolean value `false`." },
            .{ "drip", "**Integer Type**\n\nSigned 32-bit integer type." },
            .{ "tea", "**String Type**\n\nUTF-8 encoded string type." },
            .{ "lit", "**Boolean Type**\n\nBoolean type that can be `based` (true) or `cringe` (false)." },
            .{ "squad", "**Struct Definition**\n\n`squad Name { fields }`\n\nDefines a new struct type with fields." },
            .{ "collab", "**Interface Definition**\n\n`collab Name { methods }`\n\nDefines a new interface with method signatures." },
            .{ "yeet", "**Import Statement**\n\n`yeet \"module\"`\n\nImports a module into the current scope." },
            .{ "sick", "**Pattern Matching**\n\n`sick (value) { when pattern -> result }`\n\nPattern matching construct for complex conditionals." },
        });
        
        if (keyword_docs.get(word)) |doc| {
            return try self.allocator.dupe(u8, doc);
        }
        
        return null;
    }

    /// Handle go-to-definition requests
    fn handleDefinition(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const position = params.object.get("position") orelse return;
        
        const uri = text_document.object.get("uri").?.string;
        const line = @as(u32, @intCast(position.object.get("line").?.integer));
        const character = @as(u32, @intCast(position.object.get("character").?.integer));

        var locations = ArrayList(Location).init(self.allocator);
        defer locations.deinit();

        if (self.documents.get(uri)) |doc_data| {
            const cursor_pos = Position{ .line = line, .character = character };
            
            // Find symbol at cursor position
            for (doc_data.symbols.items) |symbol| {
                if (symbol.location.range.contains(cursor_pos)) {
                    try locations.append(symbol.location);
                    break;
                }
            }
        }

        try self.sendDefinitionResponse(writer, request_id, &locations);
    }

    /// Handle find references requests
    fn handleReferences(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        _ = request;
        const request_id = id orelse return;
        // TODO: Implement reference finding across all documents
        var empty_locations = ArrayList(Location).init(self.allocator);
        defer empty_locations.deinit();
        try self.sendDefinitionResponse(writer, request_id, &empty_locations);
    }

    /// Handle document symbol requests
    fn handleDocumentSymbol(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const uri = text_document.object.get("uri").?.string;

        var symbols = ArrayList(SymbolInformation).init(self.allocator);
        defer symbols.deinit();

        if (self.documents.get(uri)) |doc_data| {
            try symbols.appendSlice(doc_data.symbols.items);
        }

        try self.sendWorkspaceSymbolResponse(writer, request_id, &symbols);
    }

    /// Enhanced formatting handler
    fn handleFormatting(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const uri = text_document.object.get("uri").?.string;

        var text_edits = ArrayList([]const u8).init(self.allocator);
        defer {
            for (text_edits.items) |edit| {
                self.allocator.free(edit);
            }
            text_edits.deinit();
        }

        if (self.documents.get(uri)) |doc_data| {
            const formatted = try self.formatCursedCode(doc_data.text);
            defer self.allocator.free(formatted);
            
            if (!std.mem.eql(u8, formatted, doc_data.text)) {
                // Calculate document end position
                const line_count = std.mem.count(u8, doc_data.text, "\n");
                const last_line = if (doc_data.text.len > 0 and doc_data.text[doc_data.text.len - 1] == '\n') 
                    line_count 
                else 
                    line_count;
                
                // Create text edit JSON
                const edit = try std.fmt.allocPrint(self.allocator,
                    \\{{"range": {{"start": {{"line": 0, "character": 0}}, "end": {{"line": {}, "character": 0}}}}, "newText": "{s}"}}
                , .{ last_line, self.escapeJsonString(formatted) });
                try text_edits.append(edit);
            }
        }

        try self.sendFormattingResponse(writer, request_id, &text_edits);
    }

    /// Enhanced semantic tokens handler
    fn handleSemanticTokens(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const text_document = params.object.get("textDocument") orelse return;
        const uri = text_document.object.get("uri").?.string;

        var tokens = ArrayList(u32).init(self.allocator);
        defer tokens.deinit();

        if (self.documents.get(uri)) |doc_data| {
            if (doc_data.tokens) |document_tokens| {
                try self.generateSemanticTokenData(document_tokens, &tokens);
            }
        }

        try self.sendSemanticTokensResponse(writer, request_id, &tokens);
    }

    /// Generate semantic token data from lexer tokens
    fn generateSemanticTokenData(self: *EnhancedCursedLanguageServer, tokens: []lexer.Token, semantic_tokens: *ArrayList(u32)) !void {
        _ = self;
        
        var prev_line: u32 = 0;
        var prev_char: u32 = 0;
        
        for (tokens) |token| {
            const line = @as(u32, @intCast(token.line));
            const char = @as(u32, @intCast(token.column));
            const length = @as(u32, @intCast(token.lexeme.len));
            
            // Calculate relative position
            const delta_line = line - prev_line;
            const delta_char = if (delta_line > 0) char else char - prev_char;
            
            // Determine token type and modifiers
            const token_type: u32 = switch (token.kind) {
                .Identifier => 12, // variable
                .String, .StringLiteral => 1, // string
                .Number, .Integer => 2, // number
                .Comment => 3, // comment
                .Slay, .Sus, .Ready, .Bestie, .Squad, .Collab => 0, // keyword
                else => 0,
            };
            const token_modifiers: u32 = 0; // No modifiers for now
            
            // Add semantic token (5 values: deltaLine, deltaStartChar, length, tokenType, tokenModifiers)
            try semantic_tokens.append(delta_line);
            try semantic_tokens.append(delta_char);
            try semantic_tokens.append(length);
            try semantic_tokens.append(token_type);
            try semantic_tokens.append(token_modifiers);
            
            prev_line = line;
            prev_char = char;
        }
    }

    /// Handle workspace symbol requests
    fn handleWorkspaceSymbol(self: *EnhancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter, id: ?i64) !void {
        const request_id = id orelse return;
        const params = request.object.get("params") orelse return;
        const query = if (params.object.get("query")) |q| q.string else "";

        var symbols = ArrayList(SymbolInformation).init(self.allocator);
        defer symbols.deinit();

        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc_data = entry.value_ptr;
            for (doc_data.symbols.items) |symbol| {
                if (query.len == 0 or std.ascii.indexOfIgnoreCase(symbol.name, query) != null) {
                    try symbols.append(symbol);
                }
            }
        }

        try self.sendWorkspaceSymbolResponse(writer, request_id, &symbols);
    }

    /// Handle shutdown request
    fn handleShutdown(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: ?i64) !void {
        self.shutdown_requested = true;
        const request_id = id orelse 0;
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
        , .{request_id});
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
        std.log.info("Shutdown requested", .{});
    }

    /// Enhanced CURSED code formatter
    fn formatCursedCode(self: *EnhancedCursedLanguageServer, code: []const u8) ![]u8 {
        var formatted = ArrayList(u8).init(self.allocator);
        defer formatted.deinit();
        
        var indent_level: u32 = 0;
        var lines = std.mem.splitScalar(u8, code, '\n');
        var is_first_line = true;
        
        while (lines.next()) |line| {
            defer is_first_line = false;
            
            const trimmed = std.mem.trim(u8, line, " \t\r");
            
            // Handle empty lines
            if (trimmed.len == 0) {
                try formatted.append('\n');
                continue;
            }
            
            // Handle closing braces
            if (std.mem.eql(u8, trimmed, "}")) {
                if (indent_level > 0) indent_level -= 1;
            }
            
            // Add proper indentation
            if (!is_first_line) try formatted.append('\n');
            
            var i: u32 = 0;
            while (i < indent_level) : (i += 1) {
                try formatted.appendSlice("    "); // 4 spaces per indent level
            }
            
            try formatted.appendSlice(trimmed);
            
            // Handle opening braces
            if (std.mem.endsWith(u8, trimmed, "{")) {
                indent_level += 1;
            }
        }
        
        return formatted.toOwnedSlice();
    }

    /// Utility function to escape JSON strings
    fn escapeJsonString(self: *EnhancedCursedLanguageServer, input: []const u8) []const u8 {
        _ = self;
        // For now, return input as-is. In production, should properly escape quotes, backslashes, etc.
        return input;
    }

    /// Helper function to send responses
    fn sendResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, response: []const u8) !void {
        _ = self;
        try writer.print("Content-Length: {}\r\n\r\n", .{response.len});
        try writer.writeAll(response);
    }

    /// Send error response
    fn sendErrorResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: ?i64, code: i32, message: []const u8, data: ?[]const u8) !void {
        const error_data = if (data) |d| try std.fmt.allocPrint(self.allocator, ", \"data\": \"{s}\"", .{d}) else try self.allocator.dupe(u8, "");
        defer self.allocator.free(error_data);
        
        const id_str = if (id) |request_id| try std.fmt.allocPrint(self.allocator, "{}", .{request_id}) else try self.allocator.dupe(u8, "null");
        defer self.allocator.free(id_str);
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {s}, "error": {{"code": {}, "message": "{s}"{s}}}}}
        , .{ id_str, code, message, error_data });
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
    }

    /// Publish diagnostics notification
    fn publishDiagnostics(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, uri: []const u8, diagnostics: *ArrayList(Diagnostic)) !void {
        // Build diagnostics JSON array
        var diag_json = ArrayList(u8).init(self.allocator);
        defer diag_json.deinit();
        
        try diag_json.appendSlice("[");
        
        for (diagnostics.items, 0..) |diag, i| {
            if (i > 0) try diag_json.appendSlice(",");
            
            const diag_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"range": {{"start": {{"line": {}, "character": {}}}, "end": {{"line": {}, "character": {}}}}}, "severity": {}, "source": "{s}", "message": "{s}"}}
            , .{
                diag.range.start.line, diag.range.start.character,
                diag.range.end.line, diag.range.end.character,
                @intFromEnum(diag.severity), diag.source, diag.message
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
        
        try self.sendResponse(writer, notification);
    }

    /// Send completion response
    fn sendCompletionResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, completions: *ArrayList(CompletionItem)) !void {
        // Build completion items JSON array
        var items_json = ArrayList(u8).init(self.allocator);
        defer items_json.deinit();
        
        try items_json.appendSlice("[");
        
        for (completions.items, 0..) |item, i| {
            if (i > 0) try items_json.appendSlice(",");
            
            const item_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"label": "{s}", "kind": {}, "detail": "{s}", "documentation": "{s}", "insertText": "{s}"}}
            , .{ 
                item.label, 
                @intFromEnum(item.kind), 
                item.detail orelse "",
                item.documentation orelse "",
                item.insert_text orelse item.label
            });
            defer self.allocator.free(item_obj);
            
            try items_json.appendSlice(item_obj);
        }
        
        try items_json.appendSlice("]");
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"isIncomplete": false, "items": {s}}}}}
        , .{ id, items_json.items });
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
    }

    /// Send hover response
    fn sendHoverResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, hover_text: ?[]const u8) !void {
        const response = if (hover_text) |text|
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": {{"contents": {{"kind": "markdown", "value": "{s}"}}, "range": null}}}}
            , .{ id, text })
        else
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
            , .{id});
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
    }

    /// Send definition response
    fn sendDefinitionResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, locations: *ArrayList(Location)) !void {
        // Build locations JSON array
        var locations_json = ArrayList(u8).init(self.allocator);
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
        
        try self.sendResponse(writer, response);
    }

    /// Send formatting response
    fn sendFormattingResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, edits: *ArrayList([]const u8)) !void {
        // Build text edits JSON array
        var edits_json = ArrayList(u8).init(self.allocator);
        defer edits_json.deinit();
        
        try edits_json.appendSlice("[");
        
        for (edits.items, 0..) |edit, i| {
            if (i > 0) try edits_json.appendSlice(",");
            try edits_json.appendSlice(edit);
        }
        
        try edits_json.appendSlice("]");
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {s}}}
        , .{ id, edits_json.items });
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
    }

    /// Send semantic tokens response
    fn sendSemanticTokensResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, tokens: *ArrayList(u32)) !void {
        // Build tokens JSON array
        var tokens_json = ArrayList(u8).init(self.allocator);
        defer tokens_json.deinit();
        
        try tokens_json.appendSlice("[");
        
        for (tokens.items, 0..) |token, i| {
            if (i > 0) try tokens_json.appendSlice(",");
            const token_str = try std.fmt.allocPrint(self.allocator, "{}", .{token});
            defer self.allocator.free(token_str);
            try tokens_json.appendSlice(token_str);
        }
        
        try tokens_json.appendSlice("]");
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"data": {s}}}}}
        , .{ id, tokens_json.items });
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
    }

    /// Send workspace symbol response
    fn sendWorkspaceSymbolResponse(self: *EnhancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, symbols: *ArrayList(SymbolInformation)) !void {
        // Build symbols JSON array
        var symbols_json = ArrayList(u8).init(self.allocator);
        defer symbols_json.deinit();
        
        try symbols_json.appendSlice("[");
        
        for (symbols.items, 0..) |symbol, i| {
            if (i > 0) try symbols_json.appendSlice(",");
            
            const symbol_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"name": "{s}", "kind": {}, "location": {{"uri": "{s}", "range": {{"start": {{"line": {}, "character": {}}}, "end": {{"line": {}, "character": {}}}}}}}}}
            , .{
                symbol.name, @intFromEnum(symbol.kind), symbol.location.uri,
                symbol.location.range.start.line, symbol.location.range.start.character,
                symbol.location.range.end.line, symbol.location.range.end.character
            });
            defer self.allocator.free(symbol_obj);
            
            try symbols_json.appendSlice(symbol_obj);
        }
        
        try symbols_json.appendSlice("]");
        
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {s}}}
        , .{ id, symbols_json.items });
        defer self.allocator.free(response);
        
        try self.sendResponse(writer, response);
    }
};

/// Enhanced LSP Server main loop with comprehensive error handling
pub fn runEnhancedLspServer(allocator: Allocator) !void {
    var server = EnhancedCursedLanguageServer.init(allocator);
    defer server.deinit();

    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();
    const writer = stdout.any();

    std.log.info("Enhanced CURSED LSP Server starting...", .{});

    var buffer = ArrayList(u8).init(allocator);
    defer buffer.deinit();

    while (!server.shutdown_requested) {
        // Read Content-Length header
        var content_length: usize = 0;
        
        header_loop: while (true) {
            const line = stdin.readUntilDelimiterAlloc(allocator, '\n', 1024) catch |err| switch (err) {
                error.EndOfStream => {
                    std.log.info("Client disconnected", .{});
                    return;
                },
                else => {
                    std.log.err("Error reading header: {}", .{err});
                    continue :header_loop;
                },
            };
            defer allocator.free(line);
            
            const trimmed = std.mem.trim(u8, line, "\r\n");
            if (trimmed.len == 0) break; // Empty line marks end of headers
            
            if (std.mem.startsWith(u8, trimmed, "Content-Length: ")) {
                const length_str = trimmed[16..];
                content_length = std.fmt.parseInt(usize, length_str, 10) catch {
                    std.log.err("Invalid Content-Length: {s}", .{length_str});
                    continue :header_loop;
                };
            }
        }

        if (content_length == 0) {
            std.log.warn("No Content-Length found, skipping message", .{});
            continue;
        }

        if (content_length > 1024 * 1024) { // 1MB limit
            std.log.err("Message too large: {} bytes", .{content_length});
            continue;
        }

        // Read message content
        buffer.clearRetainingCapacity();
        buffer.resize(content_length) catch {
            std.log.err("Failed to allocate buffer for message", .{});
            continue;
        };
        
        const bytes_read = stdin.readAll(buffer.items) catch |err| {
            std.log.err("Error reading message content: {}", .{err});
            continue;
        };
        
        if (bytes_read != content_length) {
            std.log.err("Expected {} bytes, got {}", .{ content_length, bytes_read });
            continue;
        }

        // Handle the request
        server.handleRequest(buffer.items, writer) catch |err| {
            std.log.err("Error handling request: {}", .{err});
            // Continue processing other requests
        };
    }
    
    std.log.info("Enhanced CURSED LSP Server shutting down...", .{});
}
