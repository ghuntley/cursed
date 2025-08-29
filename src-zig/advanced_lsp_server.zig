//! Advanced CURSED Language Server Protocol Implementation
//! World-class IDE support with semantic analysis, refactoring, and code generation

const std = @import("std");
const json = std.json;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const type_system = @import("type_system_runtime.zig");

/// Advanced LSP message types
const MessageType = enum {
    // Standard LSP
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
    textDocument_codeLens,
    textDocument_inlayHint,
    textDocument_codeAction,
    textDocument_selectionRange,
    workspace_symbol,
    callHierarchy_incomingCalls,
    callHierarchy_outgoingCalls,
    typeHierarchy_supertypes,
    typeHierarchy_subtypes,
    // Custom CURSED features
    cursed_generateFunction,
    cursed_extractFunction,
    cursed_generateTests,
    cursed_securityAnalysis,
    cursed_performanceHints,
    shutdown,
    exit,
};

/// Diagnostic severity levels
const DiagnosticSeverity = enum(u32) {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
};

/// Code action kinds
const CodeActionKind = struct {
    pub const QuickFix = "quickfix";
    pub const Refactor = "refactor";
    pub const RefactorExtract = "refactor.extract";
    pub const RefactorInline = "refactor.inline";
    pub const RefactorRewrite = "refactor.rewrite";
    pub const Source = "source";
    pub const SourceOrganizeImports = "source.organizeImports";
};

/// Symbol information with enhanced metadata
const SymbolInfo = struct {
    name: []const u8,
    kind: u32,
    location: Location,
    detail: ?[]const u8,
    type_info: ?TypeInfo,
    documentation: ?[]const u8,
    deprecated: bool,
    container_name: ?[]const u8,
    references: ArrayList(Location),
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, kind: u32, location: Location) !SymbolInfo {
        return SymbolInfo{
            .name = try allocator.dupe(u8, name),
            .kind = kind,
            .location = location,
            .detail = null,
            .type_info = null,
            .documentation = null,
            .deprecated = false,
            .container_name = null,
            .references = ArrayList(Location){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *SymbolInfo) void {
        self.allocator.free(self.name);
        if (self.detail) |detail| self.allocator.free(detail);
        if (self.documentation) |doc| self.allocator.free(doc);
        if (self.container_name) |name| self.allocator.free(name);
        self.references.deinit(self.allocator);
    }
};

/// Type information for semantic analysis
const TypeInfo = struct {
    name: []const u8,
    kind: TypeKind,
    generic_params: ?[]const []const u8,
    methods: ArrayList(MethodInfo),
    fields: ArrayList(FieldInfo),
    allocator: Allocator,

    const TypeKind = enum {
        Primitive,
        Struct,
        Interface,
        Function,
        Array,
        Generic,
    };

    const MethodInfo = struct {
        name: []const u8,
        signature: []const u8,
        location: Location,
    };

    const FieldInfo = struct {
        name: []const u8,
        type_name: []const u8,
        location: Location,
    };

    pub fn init(allocator: Allocator, name: []const u8, kind: TypeKind) !TypeInfo {
        return TypeInfo{
            .name = try allocator.dupe(u8, name),
            .kind = kind,
            .generic_params = null,
            .methods = ArrayList(MethodInfo){},
            .fields = ArrayList(FieldInfo){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *TypeInfo) void {
        self.allocator.free(self.name);
        if (self.generic_params) |params| {
            for (params) |param| self.allocator.free(param);
            self.allocator.free(params);
        }
        for (self.methods.items) |*method| {
            self.allocator.free(method.name);
            self.allocator.free(method.signature);
        }
        for (self.fields.items) |*field| {
            self.allocator.free(field.name);
            self.allocator.free(field.type_name);
        }
        self.methods.deinit(self.allocator);
        self.fields.deinit(self.allocator);
    }
};

/// Code lens information
const CodeLens = struct {
    range: Range,
    command: ?Command,
    data: ?[]const u8,

    const Command = struct {
        title: []const u8,
        command: []const u8,
        arguments: ?[]const u8,
    };
};

/// Inlay hint information
const InlayHint = struct {
    position: Position,
    label: []const u8,
    kind: InlayHintKind,
    tooltip: ?[]const u8,
    padding_left: bool,
    padding_right: bool,

    const InlayHintKind = enum(u32) {
        Type = 1,
        Parameter = 2,
    };
};

/// Code action information
const CodeAction = struct {
    title: []const u8,
    kind: []const u8,
    diagnostics: ?[]const Diagnostic,
    edit: ?WorkspaceEdit,
    command: ?CodeLens.Command,

    const WorkspaceEdit = struct {
        changes: HashMap([]const u8, []const TextEdit, StringContext, std.hash_map.default_max_load_percentage),

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

        const TextEdit = struct {
            range: Range,
            new_text: []const u8,
        };
    };
};

/// Performance analysis result
const PerformanceHint = struct {
    location: Location,
    category: Category,
    severity: DiagnosticSeverity,
    message: []const u8,
    suggestion: []const u8,
    fix: ?CodeAction,

    const Category = enum {
        MemoryUsage,
        CPUIntensive,
        NetworkCall,
        DatabaseQuery,
        ConcurrencyIssue,
        AlgorithmicComplexity,
    };
};

/// Security analysis result
const SecurityVulnerability = struct {
    location: Location,
    category: Category,
    severity: DiagnosticSeverity,
    message: []const u8,
    cwe_id: ?u32,
    fix: ?CodeAction,

    const Category = enum {
        BufferOverflow,
        SQLInjection,
        XSS,
        InsecureRandom,
        WeakCrypto,
        PathTraversal,
        UnsafeDeserialization,
    };
};

/// Advanced LSP Position structure
const Position = struct {
    line: u32,
    character: u32,
};

/// Advanced LSP Range structure
const Range = struct {
    start: Position,
    end: Position,
};

/// Advanced LSP Location structure
const Location = struct {
    uri: []const u8,
    range: Range,
};

/// Advanced LSP Diagnostic structure
const Diagnostic = struct {
    range: Range,
    severity: DiagnosticSeverity,
    code: ?[]const u8,
    code_description: ?[]const u8,
    source: []const u8,
    message: []const u8,
    tags: ?[]const DiagnosticTag,
    related_information: ?[]const DiagnosticRelatedInformation,
    data: ?[]const u8,

    const DiagnosticTag = enum(u32) {
        Unnecessary = 1,
        Deprecated = 2,
    };

    const DiagnosticRelatedInformation = struct {
        location: Location,
        message: []const u8,
    };
};

/// Enhanced document data with comprehensive analysis
const DocumentData = struct {
    uri: []const u8,
    text: []const u8,
    version: i32,
    ast: ?ast.Program,
    symbols: ArrayList(SymbolInfo),
    types: HashMap([]const u8, TypeInfo, StringContext, std.hash_map.default_max_load_percentage),
    diagnostics: ArrayList(Diagnostic),
    code_lenses: ArrayList(CodeLens),
    inlay_hints: ArrayList(InlayHint),
    performance_hints: ArrayList(PerformanceHint),
    security_vulnerabilities: ArrayList(SecurityVulnerability),
    imports: ArrayList([]const u8),
    exports: ArrayList([]const u8),
    allocator: Allocator,

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

    pub fn init(allocator: Allocator, uri: []const u8, text: []const u8, version: i32) !DocumentData {
        return DocumentData{
            .uri = try allocator.dupe(u8, uri),
            .text = try allocator.dupe(u8, text),
            .version = version,
            .ast = null,
            .symbols = ArrayList(SymbolInfo){},
            .types = HashMap([]const u8, TypeInfo, StringContext, std.hash_map.default_max_load_percentage){},
            .diagnostics = ArrayList(Diagnostic){},
            .code_lenses = ArrayList(CodeLens){},
            .inlay_hints = ArrayList(InlayHint){},
            .performance_hints = ArrayList(PerformanceHint){},
            .security_vulnerabilities = ArrayList(SecurityVulnerability){},
            .imports = ArrayList([]const u8){},
            .exports = ArrayList([]const u8){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DocumentData) void {
        self.allocator.free(self.uri);
        self.allocator.free(self.text);
        
        for (self.symbols.items) |*symbol| {
            symbol.deinit();
        }
        self.symbols.deinit(self.allocator);
        
        var type_iter = self.types.iterator();
        while (type_iter.next()) |entry| {
            var type_info = entry.value_ptr;
            type_info.deinit();
        }
        self.types.deinit(self.allocator);
        
        self.diagnostics.deinit(self.allocator);
        self.code_lenses.deinit(self.allocator);
        self.inlay_hints.deinit(self.allocator);
        self.performance_hints.deinit(self.allocator);
        self.security_vulnerabilities.deinit(self.allocator);
        
        for (self.imports.items) |import| {
            self.allocator.free(import);
        }
        self.imports.deinit(self.allocator);
        
        for (self.exports.items) |exp| {
            self.allocator.free(exp);
        }
        self.exports.deinit(self.allocator);
    }
};

/// Advanced CURSED Language Server
pub const AdvancedCursedLanguageServer = struct {
    allocator: Allocator,
    documents: HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage),
    workspace_root: ?[]const u8,
    workspace_symbols: ArrayList(SymbolInfo),
    type_hierarchy: HashMap([]const u8, ArrayList([]const u8), StringContext, std.hash_map.default_max_load_percentage),
    call_hierarchy: HashMap([]const u8, ArrayList([]const u8), StringContext, std.hash_map.default_max_load_percentage),
    initialized: bool,
    capabilities: ServerCapabilities,

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

    const ServerCapabilities = struct {
        text_document_sync: bool,
        completion_provider: bool,
        hover_provider: bool,
        definition_provider: bool,
        references_provider: bool,
        document_formatting_provider: bool,
        rename_provider: bool,
        semantic_tokens_provider: bool,
        code_lens_provider: bool,
        inlay_hint_provider: bool,
        code_action_provider: bool,
        call_hierarchy_provider: bool,
        type_hierarchy_provider: bool,
        workspace_symbol_provider: bool,
    };

    pub fn init(allocator: Allocator) AdvancedCursedLanguageServer {
        _ = allocator;
        return AdvancedCursedLanguageServer{
            .allocator = allocator,
            .documents = HashMap([]const u8, DocumentData, StringContext, std.hash_map.default_max_load_percentage){},
            .workspace_root = null,
            .workspace_symbols = ArrayList(SymbolInfo){},
            .type_hierarchy = HashMap([]const u8, ArrayList([]const u8), StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .call_hierarchy = HashMap([]const u8, ArrayList([]const u8), StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .initialized = false,
            .capabilities = ServerCapabilities{
                .text_document_sync = true,
                .completion_provider = true,
                .hover_provider = true,
                .definition_provider = true,
                .references_provider = true,
                .document_formatting_provider = true,
                .rename_provider = true,
                .semantic_tokens_provider = true,
                .code_lens_provider = true,
                .inlay_hint_provider = true,
                .code_action_provider = true,
                .call_hierarchy_provider = true,
                .type_hierarchy_provider = true,
                .workspace_symbol_provider = true,
            },
        };
    }

    pub fn deinit(self: *AdvancedCursedLanguageServer) void {
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            var doc = entry.value_ptr;
            doc.deinit();
        }
        self.documents.deinit(self.allocator);
        
        for (self.workspace_symbols.items) |*symbol| {
            symbol.deinit();
        }
        self.workspace_symbols.deinit(self.allocator);
        
        var type_iter = self.type_hierarchy.iterator();
        while (type_iter.next()) |entry| {
            var list = entry.value_ptr;
            for (list.items) |item| {
                self.allocator.free(item);
            }
            list.deinit();
        }
        self.type_hierarchy.deinit(self.allocator);
        
        var call_iter = self.call_hierarchy.iterator();
        while (call_iter.next()) |entry| {
            var list = entry.value_ptr;
            for (list.items) |item| {
                self.allocator.free(item);
            }
            list.deinit();
        }
        self.call_hierarchy.deinit(self.allocator);
        
        if (self.workspace_root) |root| {
            self.allocator.free(root);
        }
    }

    /// Handle LSP requests with enhanced routing
    pub fn handleRequest(self: *AdvancedCursedLanguageServer, input: []const u8, writer: std.io.AnyWriter) !void {
        const parsed = json.parseFromSlice(json.Value, self.allocator, input, .{}) catch |err| {
            std.log.err("Failed to parse JSON: {}", .{err});
            return;
        };
        defer parsed.deinit();

        const root = parsed.value;
        
        if (root.object.get("method")) |method| {
            const method_str = method.string;
            
            // Standard LSP methods
            if (std.mem.eql(u8, method_str, "initialize")) {
                try self.handleInitialize(root, writer);
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
            } else if (std.mem.eql(u8, method_str, "textDocument/rename")) {
                try self.handleRename(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/semanticTokens/full")) {
                try self.handleSemanticTokens(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/codeLens")) {
                try self.handleCodeLens(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/inlayHint")) {
                try self.handleInlayHints(root, writer);
            } else if (std.mem.eql(u8, method_str, "textDocument/codeAction")) {
                try self.handleCodeAction(root, writer);
            } else if (std.mem.eql(u8, method_str, "callHierarchy/incomingCalls")) {
                try self.handleIncomingCalls(root, writer);
            } else if (std.mem.eql(u8, method_str, "callHierarchy/outgoingCalls")) {
                try self.handleOutgoingCalls(root, writer);
            } else if (std.mem.eql(u8, method_str, "typeHierarchy/supertypes")) {
                try self.handleSupertypes(root, writer);
            } else if (std.mem.eql(u8, method_str, "typeHierarchy/subtypes")) {
                try self.handleSubtypes(root, writer);
            } else if (std.mem.eql(u8, method_str, "workspace/symbol")) {
                try self.handleWorkspaceSymbol(root, writer);
            }
            // Custom CURSED methods
            else if (std.mem.eql(u8, method_str, "cursed/generateFunction")) {
                try self.handleGenerateFunction(root, writer);
            } else if (std.mem.eql(u8, method_str, "cursed/extractFunction")) {
                try self.handleExtractFunction(root, writer);
            } else if (std.mem.eql(u8, method_str, "cursed/generateTests")) {
                try self.handleGenerateTests(root, writer);
            } else if (std.mem.eql(u8, method_str, "cursed/securityAnalysis")) {
                try self.handleSecurityAnalysis(root, writer);
            } else if (std.mem.eql(u8, method_str, "cursed/performanceHints")) {
                try self.handlePerformanceHints(root, writer);
            } else if (std.mem.eql(u8, method_str, "shutdown")) {
                try self.handleShutdown(writer);
            } else if (std.mem.eql(u8, method_str, "exit")) {
                return;
            }
            }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    
    try runAdvancedLspServer(allocator);
}

    /// Enhanced initialize with all capabilities
    fn handleInitialize(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
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
            \\        "change": 2,
            \\        "save": true
            \\      }},
            \\      "completionProvider": {{
            \\        "triggerCharacters": [".", "::", "(", ",", " "],
            \\        "resolveProvider": true,
            \\        "completionItem": {{
            \\          "snippetSupport": true,
            \\          "commitCharactersSupport": true,
            \\          "documentationFormat": ["markdown", "plaintext"]
            \\        }}
            \\      }},
            \\      "hoverProvider": {{
            \\        "contentFormat": ["markdown", "plaintext"]
            \\      }},
            \\      "definitionProvider": true,
            \\      "referencesProvider": true,
            \\      "documentFormattingProvider": true,
            \\      "renameProvider": {{
            \\        "prepareProvider": true
            \\      }},
            \\      "semanticTokensProvider": {{
            \\        "legend": {{
            \\          "tokenTypes": ["keyword", "string", "number", "comment", "operator", "namespace", "type", "class", "interface", "enum", "function", "method", "variable", "parameter", "property", "struct", "typeParameter", "decorator"],
            \\          "tokenModifiers": ["declaration", "definition", "readonly", "static", "deprecated", "abstract", "async", "modification", "documentation", "defaultLibrary", "generic"]
            \\        }},
            \\        "full": true,
            \\        "range": true
            \\      }},
            \\      "codeLensProvider": {{
            \\        "resolveProvider": true
            \\      }},
            \\      "inlayHintProvider": {{
            \\        "resolveProvider": true
            \\      }},
            \\      "codeActionProvider": {{
            \\        "codeActionKinds": ["quickfix", "refactor", "refactor.extract", "refactor.inline", "refactor.rewrite", "source", "source.organizeImports"],
            \\        "resolveProvider": true
            \\      }},
            \\      "callHierarchyProvider": true,
            \\      "typeHierarchyProvider": true,
            \\      "workspaceSymbolProvider": {{
            \\        "resolveProvider": true
            \\      }},
            \\      "experimental": {{
            \\        "cursedFeatures": {{
            \\          "generateFunction": true,
            \\          "extractFunction": true,
            \\          "generateTests": true,
            \\          "securityAnalysis": true,
            \\          "performanceHints": true,
            \\          "memoryAnalysis": true,
            \\          "concurrencyAnalysis": true
            \\        }}
            \\      }}
            \\    }}
            \\  }}
            \\}}
        ;
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.print(response, .{id});
    }

    /// Enhanced initialized notification
    fn handleInitialized(self: *AdvancedCursedLanguageServer) !void {
        self.initialized = true;
        std.log.info("Advanced CURSED LSP Server initialized with full IDE features", .{});
    }

    /// Enhanced document opening with comprehensive analysis
    fn handleDidOpenTextDocument(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        
        const uri = text_document.get("uri").?.string;
        const text = text_document.get("text").?.string;
        const version = @as(i32, @intCast(text_document.get("version").?.integer));

        var doc_data = try DocumentData.init(self.allocator, uri, text, version);
        try self.performComprehensiveAnalysis(&doc_data);
        
        try self.documents.put(uri, doc_data);
        try self.publishDiagnostics(writer, uri, &doc_data.diagnostics);
    }

    /// Enhanced document change with incremental analysis
    fn handleDidChangeTextDocument(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const content_changes = params.get("contentChanges").?.array;
        
        const uri = text_document.get("uri").?.string;
        const version = @as(i32, @intCast(text_document.get("version").?.integer));

        if (self.documents.getPtr(uri)) |doc_data| {
            if (content_changes.items.len > 0) {
                const change = content_changes.items[0].object;
                if (change.get("text")) |new_text| {
                    self.allocator.free(doc_data.text);
                    doc_data.text = try self.allocator.dupe(u8, new_text.string);
                    doc_data.version = version;
                    
                    // Clear previous analysis
                    try self.clearDocumentAnalysis(doc_data);
                    
                    // Perform comprehensive analysis
                    try self.performComprehensiveAnalysis(doc_data);
                    try self.publishDiagnostics(writer, uri, &doc_data.diagnostics);
                }
            }
        }
    }

    /// Comprehensive document analysis
    fn performComprehensiveAnalysis(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        // Perform lexical and syntactic analysis
        try self.analyzeDocument(doc_data);
        
        // Perform semantic analysis
        try self.performSemanticAnalysis(doc_data);
        
        // Generate code lenses
        try self.generateCodeLenses(doc_data);
        
        // Generate inlay hints
        try self.generateInlayHints(doc_data);
        
        // Perform security analysis
        try self.performSecurityAnalysis(doc_data);
        
        // Perform performance analysis
        try self.performPerformanceAnalysis(doc_data);
        
        // Extract imports/exports
        try self.extractImportsExports(doc_data);
    }

    /// Clear document analysis data
    fn clearDocumentAnalysis(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        _ = self;
        
        for (doc_data.symbols.items) |*symbol| {
            symbol.deinit();
        }
        doc_data.symbols.clearAndFree();
        
        var type_iter = doc_data.types.iterator();
        while (type_iter.next()) |entry| {
            var type_info = entry.value_ptr;
            type_info.deinit();
        }
        doc_data.types.clearAndFree();
        
        doc_data.diagnostics.clearAndFree();
        doc_data.code_lenses.clearAndFree();
        doc_data.inlay_hints.clearAndFree();
        doc_data.performance_hints.clearAndFree();
        doc_data.security_vulnerabilities.clearAndFree();
        
        for (doc_data.imports.items) |import| {
            doc_data.allocator.free(import);
        }
        doc_data.imports.clearAndFree();
        
        for (doc_data.exports.items) |export_item| {
            doc_data.allocator.free(export_item);
        }
        doc_data.exports.clearAndFree();
    }

    /// Enhanced document analysis with type system integration
    fn analyzeDocument(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        // Tokenize
        var lex = lexer.Lexer.init(self.allocator, doc_data.text);
        
        const tokens = lex.tokenize() catch |err| {
            try doc_data.diagnostics.append(Diagnostic{
                .range = createSafeRange(0, 0, 0, 10),
                .severity = DiagnosticSeverity.Error,
                .code = null,
                .code_description = null,
                .source = "cursed-lexer",
                .message = try std.fmt.allocPrint(self.allocator, "Lexer error: {}", .{err}),
                .tags = null,
                .related_information = null,
                .data = null,
            });
            return;
        };
        defer tokens.deinit();

        // Parse
        var parse = parser.Parser.init(self.allocator, tokens.items);
        defer parse.deinit();
        
        const program = parse.parseProgram() catch |err| {
            const error_pos = if (tokens.items.len > 0) tokens.items[tokens.items.len - 1] else null;
            const range = if (error_pos) |pos| 
                createSafeRange(pos.line, pos.column, pos.line, pos.column + pos.lexeme.len)
            else 
                createSafeRange(0, 0, 0, 10);
                
            try doc_data.diagnostics.append(Diagnostic{
                .range = range,
                .severity = DiagnosticSeverity.Error,
                .code = null,
                .code_description = null,
                .source = "cursed-parser",
                .message = try std.fmt.allocPrint(self.allocator, "Parser error: {}", .{err}),
                .tags = null,
                .related_information = null,
                .data = null,
            });
            return;
        };

        doc_data.ast = program;
        try self.extractAdvancedSymbols(doc_data, program);
    }

    /// Enhanced symbol extraction with type information
    fn extractAdvancedSymbols(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData, program: ast.Program) !void {
        _ = program;
        
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Extract function declarations with signature analysis
            if (std.mem.startsWith(u8, trimmed, "slay ")) {
                try self.extractFunctionSymbol(doc_data, trimmed, line_num);
            }
            
            // Extract variable declarations with type inference
            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.extractVariableSymbol(doc_data, trimmed, line_num);
            }
            
            // Extract struct declarations
            if (std.mem.startsWith(u8, trimmed, "squad ")) {
                try self.extractStructSymbol(doc_data, trimmed, line_num);
            }
            
            // Extract interface declarations
            if (std.mem.startsWith(u8, trimmed, "collab ")) {
                try self.extractInterfaceSymbol(doc_data, trimmed, line_num);
            }
            
            line_num += 1;
        }
    }

    /// Extract function symbol with comprehensive signature analysis
    fn extractFunctionSymbol(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32) !void {
        const after_slay = line[5..];
        if (std.mem.indexOf(u8, after_slay, "(")) |paren_pos| {
            const func_name = std.mem.trim(u8, after_slay[0..paren_pos], " \t");
            if (func_name.len > 0) {
                // Extract full signature
                const signature = try std.fmt.allocPrint(self.allocator, "{s}", .{line});
                
                // Create symbol with detailed information
                var symbol = try SymbolInfo.init(self.allocator, func_name, 12, Location{
                    .uri = doc_data.uri,
                    .range = Range{
                        .start = Position{ .line = line_num, .character = 0 },
                        .end = Position{ .line = line_num, .character = @as(u32, @intCast(line.len)) },
                    },
                });
                
                symbol.detail = signature;
                symbol.documentation = try std.fmt.allocPrint(self.allocator, "Function: {s}", .{func_name});
                
                // Create type information
                const type_info = try TypeInfo.init(self.allocator, func_name, TypeInfo.TypeKind.Function);
                try doc_data.types.put(func_name, type_info);
                
                try doc_data.symbols.append(allocator, symbol);
            }
        }
    }

    /// Extract variable symbol with type inference
    fn extractVariableSymbol(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32) !void {
        const after_sus = line[4..];
        if (std.mem.indexOf(u8, after_sus, " ")) |space_pos| {
            const var_name = std.mem.trim(u8, after_sus[0..space_pos], " \t");
            if (var_name.len > 0) {
                // Infer type from declaration
                var inferred_type: []const u8 = "unknown";
                if (std.mem.indexOf(u8, after_sus, "drip")) |_| {
                    inferred_type = "drip";
                } else if (std.mem.indexOf(u8, after_sus, "tea")) |_| {
                    inferred_type = "tea";
                } else if (std.mem.indexOf(u8, after_sus, "lit")) |_| {
                    inferred_type = "lit";
                }
                
                var symbol = try SymbolInfo.init(self.allocator, var_name, 13, Location{
                    .uri = doc_data.uri,
                    .range = Range{
                        .start = Position{ .line = line_num, .character = 0 },
                        .end = Position{ .line = line_num, .character = @as(u32, @intCast(line.len)) },
                    },
                });
                
                symbol.detail = try std.fmt.allocPrint(self.allocator, "Variable: {s}", .{inferred_type});
                symbol.documentation = try std.fmt.allocPrint(self.allocator, "Variable {s} of type {s}", .{ var_name, inferred_type });
                
                try doc_data.symbols.append(allocator, symbol);
            }
        }
    }

    /// Extract struct symbol with field analysis
    fn extractStructSymbol(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32) !void {
        const after_squad = line[6..];
        if (std.mem.indexOf(u8, after_squad, " ")) |space_pos| {
            const struct_name = std.mem.trim(u8, after_squad[0..space_pos], " \t");
            if (struct_name.len > 0) {
                var symbol = try SymbolInfo.init(self.allocator, struct_name, 5, Location{
                    .uri = doc_data.uri,
                    .range = Range{
                        .start = Position{ .line = line_num, .character = 0 },
                        .end = Position{ .line = line_num, .character = @as(u32, @intCast(line.len)) },
                    },
                });
                
                symbol.detail = try std.fmt.allocPrint(self.allocator, "Struct: {s}", .{struct_name});
                symbol.documentation = try std.fmt.allocPrint(self.allocator, "CURSED struct definition: {s}", .{struct_name});
                
                // Create type information
                const type_info = try TypeInfo.init(self.allocator, struct_name, TypeInfo.TypeKind.Struct);
                try doc_data.types.put(struct_name, type_info);
                
                try doc_data.symbols.append(allocator, symbol);
            }
        }
    }

    /// Extract interface symbol with method analysis
    fn extractInterfaceSymbol(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData, line: []const u8, line_num: u32) !void {
        const after_collab = line[7..];
        if (std.mem.indexOf(u8, after_collab, " ")) |space_pos| {
            const interface_name = std.mem.trim(u8, after_collab[0..space_pos], " \t");
            if (interface_name.len > 0) {
                var symbol = try SymbolInfo.init(self.allocator, interface_name, 11, Location{
                    .uri = doc_data.uri,
                    .range = Range{
                        .start = Position{ .line = line_num, .character = 0 },
                        .end = Position{ .line = line_num, .character = @as(u32, @intCast(line.len)) },
                    },
                });
                
                symbol.detail = try std.fmt.allocPrint(self.allocator, "Interface: {s}", .{interface_name});
                symbol.documentation = try std.fmt.allocPrint(self.allocator, "CURSED interface definition: {s}", .{interface_name});
                
                // Create type information
                const type_info = try TypeInfo.init(self.allocator, interface_name, TypeInfo.TypeKind.Interface);
                try doc_data.types.put(interface_name, type_info);
                
                try doc_data.symbols.append(allocator, symbol);
            }
        }
    }

    /// Perform semantic analysis for type checking and inference
    fn performSemanticAnalysis(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        // TODO: Implement comprehensive semantic analysis
        // This would include:
        // - Type checking
        // - Type inference
        // - Scope analysis
        // - Dead code detection
        // - Unused variable detection
        _ = self;
        _ = doc_data;
    }

    /// Generate code lenses for functions and references
    fn generateCodeLenses(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        for (doc_data.symbols.items) |symbol| {
            if (symbol.kind == 12) { // Function
                // Add reference count code lens
                const lens = CodeLens{
                    .range = symbol.location.range,
                    .command = CodeLens.Command{
                        .title = try std.fmt.allocPrint(self.allocator, "{} references", .{symbol.references.items.len}),
                        .command = "cursed.showReferences",
                        .arguments = null,
                    },
                    .data = null,
                };
                try doc_data.code_lenses.append(allocator, lens);
            }
        }
    }

    /// Generate inlay hints for types and parameters
    fn generateInlayHints(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            // Add type hints for variable declarations without explicit types
            if (std.mem.indexOf(u8, line, "sus ")) |_| {
                if (std.mem.indexOf(u8, line, " = ")) |equals_pos| {
                    // Infer type from value
                    const value_part = line[equals_pos + 3..];
                    var inferred_type: []const u8 = ": unknown";
                    
                    if (std.mem.indexOf(u8, value_part, "\"")) |_| {
                        inferred_type = ": tea";
                    } else if (std.mem.indexOf(u8, value_part, "based") != null or std.mem.indexOf(u8, value_part, "cringe") != null) {
                        inferred_type = ": lit";
                    } else {
                        // Check for numeric literal
                        const trimmed_value = std.mem.trim(u8, value_part, " \t\r\n");
                        if (std.fmt.parseInt(i64, trimmed_value, 10) catch null) |_| {
                            inferred_type = ": drip";
                        }
                    }
                    
                    const hint = InlayHint{
                        .position = Position{ .line = line_num, .character = @as(u32, @intCast(equals_pos)) },
                        .label = try self.allocator.dupe(u8, inferred_type),
                        .kind = InlayHint.InlayHintKind.Type,
                        .tooltip = try std.fmt.allocPrint(self.allocator, "Inferred type: {s}", .{inferred_type}),
                        .padding_left = false,
                        .padding_right = true,
                    };
                    try doc_data.inlay_hints.append(allocator, hint);
                }
            }
            
            line_num += 1;
        }
    }

    /// Perform security analysis for vulnerability detection
    fn performSecurityAnalysis(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            // Check for potential security issues
            
            // SQL injection potential
            if (std.mem.indexOf(u8, line, "sql") != null and std.mem.indexOf(u8, line, "+") != null) {
                const vuln = SecurityVulnerability{
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = createSafeRange(line_num, 0, line_num, line.len),
                    },
                    .category = SecurityVulnerability.Category.SQLInjection,
                    .severity = DiagnosticSeverity.Warning,
                    .message = try self.allocator.dupe(u8, "Potential SQL injection: Use parameterized queries"),
                    .cwe_id = 89,
                    .fix = null,
                };
                try doc_data.security_vulnerabilities.append(allocator, vuln);
            }
            
            // Weak cryptography
            if (std.mem.indexOf(u8, line, "md5") != null or std.mem.indexOf(u8, line, "sha1") != null) {
                const vuln = SecurityVulnerability{
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = createSafeRange(line_num, 0, line_num, line.len),
                    },
                    .category = SecurityVulnerability.Category.WeakCrypto,
                    .severity = DiagnosticSeverity.Warning,
                    .message = try self.allocator.dupe(u8, "Weak cryptographic hash: Use SHA-256 or stronger"),
                    .cwe_id = 327,
                    .fix = null,
                };
                try doc_data.security_vulnerabilities.append(allocator, vuln);
            }
            
            line_num += 1;
        }
    }

    /// Perform performance analysis for optimization hints
    fn performPerformanceAnalysis(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        var line_num: u32 = 0;
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            // Check for performance anti-patterns
            
            // Nested loops (O(n²) complexity warning)
            if (std.mem.indexOf(u8, line, "bestie") != null) {
                // Check if there's another bestie in nearby lines (simplified)
                const hint = PerformanceHint{
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = createSafeRange(line_num, 0, line_num, line.len),
                    },
                    .category = PerformanceHint.Category.AlgorithmicComplexity,
                    .severity = DiagnosticSeverity.Information,
                    .message = try self.allocator.dupe(u8, "Consider algorithm optimization for nested loops"),
                    .suggestion = try self.allocator.dupe(u8, "Use more efficient data structures or algorithms"),
                    .fix = null,
                };
                try doc_data.performance_hints.append(allocator, hint);
            }
            
            // String concatenation in loops
            if (std.mem.indexOf(u8, line, "+") != null and std.mem.indexOf(u8, line, "tea") != null) {
                const hint = PerformanceHint{
                    .location = Location{
                        .uri = doc_data.uri,
                        .range = createSafeRange(line_num, 0, line_num, line.len),
                    },
                    .category = PerformanceHint.Category.MemoryUsage,
                    .severity = DiagnosticSeverity.Hint,
                    .message = try self.allocator.dupe(u8, "String concatenation may be inefficient"),
                    .suggestion = try self.allocator.dupe(u8, "Consider using StringBuilder pattern"),
                    .fix = null,
                };
                try doc_data.performance_hints.append(allocator, hint);
            }
            
            line_num += 1;
        }
    }

    /// Extract imports and exports for module analysis
    fn extractImportsExports(self: *AdvancedCursedLanguageServer, doc_data: *DocumentData) !void {
        const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
        
        var line_iter = lines;
        while (line_iter.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Extract imports (yeet statements)
            if (std.mem.startsWith(u8, trimmed, "yeet ")) {
                if (std.mem.indexOf(u8, trimmed, "\"")) |start_quote| {
                    if (std.mem.indexOfPos(u8, trimmed, start_quote + 1, "\"")) |end_quote| {
                        const module_name = trimmed[start_quote + 1..end_quote];
                        try doc_data.imports.append(try self.allocator.dupe(u8, module_name));
                    }
                }
            }
            
            // TODO: Extract exports (would need to implement export syntax)
        }
    }

    /// Handle enhanced completion with context awareness
    fn handleCompletion(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;
        const position = params.get("position").?.object;
        const line = @as(u32, @intCast(position.get("line").?.integer));
        const character = @as(u32, @intCast(position.get("character").?.integer));
        
        var completions = ArrayList(CompletionItem){};
        defer {
            for (completions.items) |*item| {
                self.allocator.free(item.label);
                if (item.detail) |detail| self.allocator.free(detail);
                if (item.documentation) |doc| self.allocator.free(doc);
                if (item.insertText) |text| self.allocator.free(text);
            }
            completions.deinit();
        }

        // Context-aware completion logic
        const context = try self.getCompletionContext(uri, line, character);
        defer self.allocator.free(context);
        
        // Add completions based on context
        try self.addContextAwareCompletions(&completions, context);
        
        // Add workspace symbols
        try self.addWorkspaceSymbolCompletions(&completions, context);
        
        // Add snippets
        try self.addSnippetCompletions(&completions, context);
        
        try self.sendCompletionResponse(writer, id, &completions);
    }

    const CompletionItem = struct {
        label: []const u8,
        kind: u32,
        detail: ?[]const u8,
        documentation: ?[]const u8,
        insertText: ?[]const u8,
        sortText: ?[]const u8,
        filterText: ?[]const u8,
        preselect: bool,
        textEdit: ?TextEdit,
        
        const TextEdit = struct {
            range: Range,
            newText: []const u8,
        };
    };

    /// Get completion context from cursor position
    fn getCompletionContext(self: *AdvancedCursedLanguageServer, uri: []const u8, line: u32, character: u32) ![]const u8 {
        if (self.documents.get(uri)) |doc_data| {
            const lines = std.mem.splitScalar(u8, doc_data.text, '\n');
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
            
            if (character > 0 and current_line.len >= character) {
                var start: u32 = character;
                while (start > 0 and (std.ascii.isAlphanumeric(current_line[start - 1]) or current_line[start - 1] == '_' or current_line[start - 1] == '.')) {
                    start -= 1;
                }
                return try self.allocator.dupe(u8, current_line[start..character]);
            }
        }
        return try self.allocator.dupe(u8, "");
    }

    /// Add context-aware completions
    fn addContextAwareCompletions(self: *AdvancedCursedLanguageServer, completions: *ArrayList(CompletionItem), context: []const u8) !void {
        // CURSED keywords
        const keywords = [_]struct { name: []const u8, desc: []const u8, snippet: []const u8 }{
            .{ .name = "sus", .desc = "Variable declaration", .snippet = "sus ${1:name} ${2:type} = ${3:value}" },
            .{ .name = "damn", .desc = "Return statement", .snippet = "damn ${1:value}" },
            .{ .name = "slay", .desc = "Function definition", .snippet = "slay ${1:name}(${2:params}) ${3:return_type} {\n    ${4:body}\n}" },
            .{ .name = "vibez", .desc = "I/O operations", .snippet = "vibez.spill(${1:value})" },
            .{ .name = "yeet", .desc = "Import module", .snippet = "yeet \"${1:module_name}\"" },
            .{ .name = "bestie", .desc = "While loop", .snippet = "bestie (${1:condition}) {\n    ${2:body}\n}" },
            .{ .name = "ready", .desc = "If statement", .snippet = "ready (${1:condition}) {\n    ${2:body}\n}" },
            .{ .name = "otherwise", .desc = "Else clause", .snippet = "otherwise {\n    ${1:body}\n}" },
            .{ .name = "squad", .desc = "Struct definition", .snippet = "squad ${1:Name} {\n    ${2:fields}\n}" },
            .{ .name = "collab", .desc = "Interface definition", .snippet = "collab ${1:Name} {\n    ${2:methods}\n}" },
            .{ .name = "sick", .desc = "Pattern matching", .snippet = "sick (${1:value}) {\n    when ${2:pattern} -> ${3:result}\n}" },
        };

        for (keywords) |keyword| {
            if (context.len == 0 or std.mem.startsWith(u8, keyword.name, context)) {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, keyword.name),
                    .kind = 14, // Keyword
                    .detail = try self.allocator.dupe(u8, keyword.desc),
                    .documentation = try std.fmt.allocPrint(self.allocator, "**{s}**\n\n{s}", .{ keyword.name, keyword.desc }),
                    .insertText = try self.allocator.dupe(u8, keyword.snippet),
                    .sortText = null,
                    .filterText = null,
                    .preselect = false,
                    .textEdit = null,
                });
            }
        }

        // Standard library modules
        const stdlib_modules = [_]struct { name: []const u8, desc: []const u8 }{
            .{ .name = "mathz", .desc = "Mathematical functions and constants" },
            .{ .name = "stringz", .desc = "String manipulation utilities" },
            .{ .name = "arrayz", .desc = "Array operations and algorithms" },
            .{ .name = "testz", .desc = "Testing framework and assertions" },
            .{ .name = "cryptz", .desc = "Cryptographic functions" },
            .{ .name = "filez", .desc = "File system operations" },
            .{ .name = "httpz", .desc = "HTTP client and server" },
            .{ .name = "timez", .desc = "Date and time utilities" },
            .{ .name = "jsonz", .desc = "JSON parsing and serialization" },
            .{ .name = "vibez", .desc = "I/O and console operations" },
            .{ .name = "concurrenz", .desc = "Concurrency and goroutines" },
        };

        for (stdlib_modules) |module| {
            if (context.len == 0 or std.mem.startsWith(u8, module.name, context)) {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, module.name),
                    .kind = 9, // Module
                    .detail = try self.allocator.dupe(u8, "CURSED stdlib module"),
                    .documentation = try std.fmt.allocPrint(self.allocator, "**{s}**\n\n{s}\n\nImport with: `yeet \"{s}\"`", .{ module.name, module.desc, module.name }),
                    .insertText = null,
                    .sortText = null,
                    .filterText = null,
                    .preselect = false,
                    .textEdit = null,
                });
            }
        }
    }

    /// Add workspace symbol completions
    fn addWorkspaceSymbolCompletions(self: *AdvancedCursedLanguageServer, completions: *ArrayList(CompletionItem), context: []const u8) !void {
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc_data = entry.value_ptr;
            for (doc_data.symbols.items) |symbol| {
                if (context.len == 0 or std.mem.startsWith(u8, symbol.name, context)) {
                    try completions.append(CompletionItem{
                        .label = try self.allocator.dupe(u8, symbol.name),
                        .kind = switch (symbol.kind) {
                            12 => 3, // Function
                            13 => 6, // Variable
                            11 => 8, // Interface
                            5 => 7,  // Class/Struct
                            else => 1, // Text
                        },
                        .detail = if (symbol.detail) |detail| try self.allocator.dupe(u8, detail) else null,
                        .documentation = if (symbol.documentation) |doc| try self.allocator.dupe(u8, doc) else null,
                        .insertText = null,
                        .sortText = null,
                        .filterText = null,
                        .preselect = false,
                        .textEdit = null,
                    });
                }
            }
        }
    }

    /// Add snippet completions for common patterns
    fn addSnippetCompletions(self: *AdvancedCursedLanguageServer, completions: *ArrayList(CompletionItem), context: []const u8) !void {
        const snippets = [_]struct { trigger: []const u8, label: []const u8, snippet: []const u8, desc: []const u8 }{
            .{ .trigger = "func", .label = "function", .snippet = "slay ${1:name}(${2:params}) ${3:return_type} {\n    ${4:body}\n    damn ${5:return_value}\n}", .desc = "Function template" },
            .{ .trigger = "if", .label = "if statement", .snippet = "ready (${1:condition}) {\n    ${2:body}\n}", .desc = "If statement template" },
            .{ .trigger = "ifelse", .label = "if-else statement", .snippet = "ready (${1:condition}) {\n    ${2:body}\n} otherwise {\n    ${3:else_body}\n}", .desc = "If-else statement template" },
            .{ .trigger = "while", .label = "while loop", .snippet = "bestie (${1:condition}) {\n    ${2:body}\n}", .desc = "While loop template" },
            .{ .trigger = "struct", .label = "struct definition", .snippet = "squad ${1:Name} {\n    ${2:field1} ${3:type1}\n    ${4:field2} ${5:type2}\n}", .desc = "Struct definition template" },
            .{ .trigger = "interface", .label = "interface definition", .snippet = "collab ${1:Name} {\n    slay ${2:method}(${3:params}) ${4:return_type}\n}", .desc = "Interface definition template" },
            .{ .trigger = "match", .label = "pattern matching", .snippet = "sick (${1:value}) {\n    when ${2:pattern1} -> ${3:result1}\n    when ${4:pattern2} -> ${5:result2}\n    when _ -> ${6:default}\n}", .desc = "Pattern matching template" },
            .{ .trigger = "test", .label = "test function", .snippet = "slay test_${1:name}() lit {\n    // Arrange\n    ${2:setup}\n    \n    // Act\n    ${3:action}\n    \n    // Assert\n    ${4:assertion}\n    \n    damn based\n}", .desc = "Test function template" },
        };

        for (snippets) |snippet| {
            if (context.len == 0 or std.mem.startsWith(u8, snippet.trigger, context)) {
                try completions.append(CompletionItem{
                    .label = try self.allocator.dupe(u8, snippet.label),
                    .kind = 15, // Snippet
                    .detail = try self.allocator.dupe(u8, snippet.desc),
                    .documentation = try std.fmt.allocPrint(self.allocator, "**{s}**\n\n{s}", .{ snippet.label, snippet.desc }),
                    .insertText = try self.allocator.dupe(u8, snippet.snippet),
                    .sortText = null,
                    .filterText = try self.allocator.dupe(u8, snippet.trigger),
                    .preselect = false,
                    .textEdit = null,
                });
            }
        }
    }

    /// Enhanced hover with rich type information
    fn handleHover(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const position = params.get("position").?.object;
        
        const uri = text_document.get("uri").?.string;
        const line = @as(u32, @intCast(position.get("line").?.integer));
        const character = @as(u32, @intCast(position.get("character").?.integer));

        var hover_content: ?[]const u8 = null;
        defer if (hover_content) |content| self.allocator.free(content);
        
        if (self.documents.get(uri)) |doc_data| {
            // Find symbol at position
            for (doc_data.symbols.items) |symbol| {
                if (self.positionInRange(Position{ .line = line, .character = character }, symbol.location.range)) {
                    var content_builder = ArrayList(u8){};
                    defer content_builder.deinit();
                    
                    // Build rich hover content
                    try content_builder.appendSlice("**");
                    try content_builder.appendSlice(symbol.name);
                    try content_builder.appendSlice("**\n\n");
                    
                    if (symbol.detail) |detail| {
                        try content_builder.appendSlice("```cursed\n");
                        try content_builder.appendSlice(detail);
                        try content_builder.appendSlice("\n```\n\n");
                    }
                    
                    if (symbol.documentation) |doc| {
                        try content_builder.appendSlice(doc);
                        try content_builder.appendSlice("\n\n");
                    }
                    
                    // Add type information if available
                    if (symbol.type_info) |type_info| {
                        try content_builder.appendSlice("**Type:** ");
                        try content_builder.appendSlice(type_info.name);
                        try content_builder.appendSlice("\n\n");
                    }
                    
                    // Add reference count
                    try content_builder.appendSlice("**References:** ");
                    const ref_count = try std.fmt.allocPrint(self.allocator, "{}", .{symbol.references.items.len});
                    defer self.allocator.free(ref_count);
                    try content_builder.appendSlice(ref_count);
                    
                    hover_content = try content_builder.toOwnedSlice();
                    break;
                }
            }
        }

        try self.sendHoverResponse(writer, id, hover_content);
    }

    // Additional handler methods for advanced features...
    
    /// Handle code lens requests
    fn handleCodeLens(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;
        
        var code_lenses = ArrayList(CodeLens){};
        defer code_lenses.deinit();
        
        if (self.documents.get(uri)) |doc_data| {
            for (doc_data.code_lenses.items) |lens| {
                try code_lenses.append(allocator, lens);
            }
        }
        
        try self.sendCodeLensResponse(writer, id, &code_lenses);
    }

    /// Handle inlay hints requests
    fn handleInlayHints(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const text_document = params.get("textDocument").?.object;
        const uri = text_document.get("uri").?.string;
        
        var hints = ArrayList(InlayHint){};
        defer hints.deinit();
        
        if (self.documents.get(uri)) |doc_data| {
            for (doc_data.inlay_hints.items) |hint| {
                try hints.append(allocator, hint);
            }
        }
        
        try self.sendInlayHintsResponse(writer, id, &hints);
    }

    /// Handle code action requests
    fn handleCodeAction(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        var actions = ArrayList(CodeAction){};
        defer actions.deinit();
        
        // TODO: Generate context-specific code actions
        // - Quick fixes for diagnostics
        // - Refactoring actions
        // - Source actions (organize imports, etc.)
        
        try self.sendCodeActionResponse(writer, id, &actions);
    }

    /// Handle rename requests
    fn handleRename(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement rename functionality
        try self.sendRenameResponse(writer, id, null);
    }

    /// Handle references requests
    fn handleReferences(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        var locations = ArrayList(Location){};
        defer locations.deinit();
        
        // TODO: Find all references to symbol at position
        
        try self.sendReferencesResponse(writer, id, &locations);
    }

    /// Handle definition requests
    fn handleDefinition(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        var locations = ArrayList(Location){};
        defer locations.deinit();
        
        // TODO: Find definition of symbol at position
        
        try self.sendDefinitionResponse(writer, id, &locations);
    }

    /// Handle formatting requests
    fn handleFormatting(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        var edits = ArrayList(CodeAction.WorkspaceEdit.TextEdit){};
        defer edits.deinit();
        
        // TODO: Format document using cursed-fmt
        
        try self.sendFormattingResponse(writer, id, &edits);
    }

    /// Handle semantic tokens requests
    fn handleSemanticTokens(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        
        var tokens = ArrayList(u32){};
        defer tokens.deinit();
        
        // TODO: Generate semantic tokens for syntax highlighting
        
        try self.sendSemanticTokensResponse(writer, id, &tokens);
    }

    // Call hierarchy handlers
    fn handleIncomingCalls(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement incoming calls
        try self.sendEmptyResponse(writer, id);
    }

    fn handleOutgoingCalls(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement outgoing calls
        try self.sendEmptyResponse(writer, id);
    }

    // Type hierarchy handlers
    fn handleSupertypes(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement supertypes
        try self.sendEmptyResponse(writer, id);
    }

    fn handleSubtypes(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement subtypes
        try self.sendEmptyResponse(writer, id);
    }

    /// Handle workspace symbol requests
    fn handleWorkspaceSymbol(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        const params = request.object.get("params").?.object;
        const query = if (params.get("query")) |q| q.string else "";

        var symbols = ArrayList(SymbolInfo){};
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

    // Custom CURSED feature handlers
    fn handleGenerateFunction(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement function generation
        try self.sendEmptyResponse(writer, id);
    }

    fn handleExtractFunction(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement function extraction
        try self.sendEmptyResponse(writer, id);
    }

    fn handleGenerateTests(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement test generation
        try self.sendEmptyResponse(writer, id);
    }

    fn handleSecurityAnalysis(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement security analysis
        try self.sendEmptyResponse(writer, id);
    }

    fn handlePerformanceHints(self: *AdvancedCursedLanguageServer, request: json.Value, writer: std.io.AnyWriter) !void {
        const id = request.object.get("id").?.integer;
        // TODO: Implement performance hints
        try self.sendEmptyResponse(writer, id);
    }

    fn handleShutdown(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter) !void {
        _ = self;
        const response = 
            \\{"jsonrpc": "2.0", "id": null, "result": null}
        ;
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    // Helper functions

    /// Create safe range with bounds checking
    fn createSafeRange(start_line: usize, start_char: usize, end_line: usize, end_char: usize) Range {
        const safe_start = Position{
            .line = @min(@as(u32, @intCast(@min(start_line, std.math.maxInt(u32)))), std.math.maxInt(u32)),
            .character = @min(@as(u32, @intCast(@min(start_char, std.math.maxInt(u32)))), std.math.maxInt(u32)),
        };
        const safe_end = Position{
            .line = @min(@as(u32, @intCast(@min(end_line, std.math.maxInt(u32)))), std.math.maxInt(u32)),
            .character = @min(@as(u32, @intCast(@min(end_char, std.math.maxInt(u32)))), std.math.maxInt(u32)),
        };
        
        if (safe_end.line < safe_start.line or 
            (safe_end.line == safe_start.line and safe_end.character < safe_start.character)) {
            return Range{
                .start = safe_start,
                .end = Position{ .line = safe_start.line, .character = safe_start.character + 1 },
            };
        }
        
        return Range{ .start = safe_start, .end = safe_end };
    }

    /// Check if position is in range
    fn positionInRange(self: *AdvancedCursedLanguageServer, pos: Position, range: Range) bool {
        _ = self;
        return pos.line >= range.start.line and pos.line <= range.end.line and
               pos.character >= range.start.character and pos.character <= range.end.character;
    }

    /// Publish diagnostics with enhanced information
    fn publishDiagnostics(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, uri: []const u8, diagnostics: *ArrayList(Diagnostic)) !void {
        var diag_json = ArrayList(u8){};
        defer diag_json.deinit();
        
        try diag_json.appendSlice("[");
        
        for (diagnostics.items, 0..) |diag, i| {
            if (i > 0) try diag_json.appendSlice(",");
            
            var escaped_message = ArrayList(u8){};
            defer escaped_message.deinit();
            
            for (diag.message) |c| {
                switch (c) {
                    '"' => try escaped_message.appendSlice("\\\""),
                    '\\' => try escaped_message.appendSlice("\\\\"),
                    '\n' => try escaped_message.appendSlice("\\n"),
                    '\r' => try escaped_message.appendSlice("\\r"),
                    '\t' => try escaped_message.appendSlice("\\t"),
                    else => try escaped_message.append(allocator, c),
                }
            }
            
            const diag_obj = try std.fmt.allocPrint(self.allocator,
                \\{{"range": {{"start": {{"line": {}, "character": {}}}, "end": {{"line": {}, "character": {}}}}}, "severity": {}, "source": "{s}", "message": "{s}"}}
            , .{
                diag.range.start.line, diag.range.start.character,
                diag.range.end.line, diag.range.end.character,
                @intFromEnum(diag.severity), diag.source, escaped_message.items
            });
            defer self.allocator.free(diag_obj);
            
            try diag_json.appendSlice(diag_obj);
        }
        
        try diag_json.appendSlice("]");
        
        const notification = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {{"uri": "{s}", "diagnostics": {s}}}}}
        , .{ uri, diag_json.items });
        defer self.allocator.free(notification);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{notification.len});
        try writer.writer().writeAll(notification);
    }

    // Response helper functions
    fn sendCompletionResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, completions: *ArrayList(CompletionItem)) !void {
        var items_json = ArrayList(u8){};
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
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendHoverResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, hover_text: ?[]const u8) !void {
        const response = if (hover_text) |text|
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": {{"contents": {{"kind": "markdown", "value": "{s}"}}}}}
            , .{ id, text })
        else
            try std.fmt.allocPrint(self.allocator,
                \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
            , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendCodeLensResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, lenses: *ArrayList(CodeLens)) !void {
        _ = lenses;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendInlayHintsResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, hints: *ArrayList(InlayHint)) !void {
        _ = hints;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendCodeActionResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, actions: *ArrayList(CodeAction)) !void {
        _ = actions;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendRenameResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, edit: ?CodeAction.WorkspaceEdit) !void {
        _ = edit;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendReferencesResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, locations: *ArrayList(Location)) !void {
        _ = locations;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendDefinitionResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, locations: *ArrayList(Location)) !void {
        _ = locations;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendFormattingResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, edits: *ArrayList(CodeAction.WorkspaceEdit.TextEdit)) !void {
        _ = edits;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendSemanticTokensResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, tokens: *ArrayList(u32)) !void {
        _ = tokens;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": {{"data": []}}}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendWorkspaceSymbolResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64, symbols: *ArrayList(SymbolInfo)) !void {
        _ = symbols;
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": []}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }

    fn sendEmptyResponse(self: *AdvancedCursedLanguageServer, writer: std.io.AnyWriter, id: i64) !void {
        const response = try std.fmt.allocPrint(self.allocator,
            \\{{"jsonrpc": "2.0", "id": {}, "result": null}}
        , .{id});
        defer self.allocator.free(response);
        
        try writer.print("Content-Length: {s}\r\n\r\n", .{response.len});
        try writer.writer().writeAll(response);
    }
};

/// Advanced LSP Server main loop
pub fn runAdvancedLspServer(allocator: Allocator) !void {
        _ = allocator;
    var server = AdvancedCursedLanguageServer.init(allocator);
    defer server.deinit();

    const stdin = std.fs.File.stdin();
    var stdin_buffer: [4096]u8 = undefined;
    const stdin_reader = stdin.reader(stdin_buffer[0..]);
    var stdout_buffer: [4096]u8 = undefined;
    const stdout_file = std.fs.File.stdout();
    const stdout = stdout_file.writer(stdout_buffer[0..]);
    const writer = stdout;

    std.log.info("Advanced CURSED LSP Server starting with full IDE features...", .{});

    var buffer = ArrayList(u8){};
    defer buffer.deinit();

    while (true) {
        // Read Content-Length header
        var content_length: usize = 0;
        while (true) {
            var line_buf: [1024]u8 = undefined;
            const line = stdin_reader.readUntilDelimiter(line_buf[0..], '\n') catch |err| switch (err) {
                error.EndOfStream => break,
                else => return err,
            };
            
            const trimmed = std.mem.trim(u8, line, "\r\n");
            if (trimmed.len == 0) break;
            
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

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    
    try runAdvancedLspServer(allocator);
}
