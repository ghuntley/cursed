// CURSED Language Server Protocol Implementation
// Provides comprehensive IDE support for CURSED programming language

const std = @import("std");
const json = std.json;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

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

// Note: LSP server uses direct file access to avoid module path issues
// These modules are accessed via the build system configuration

// LSP Message Types
pub const LSPMessageType = enum {
    request,
    response,
    notification,
};

pub const LSPMessage = struct {
    jsonrpc: []const u8 = "2.0",
    id: ?json.Value = null,
    method: ?[]const u8 = null,
    params: ?json.Value = null,
    result: ?json.Value = null,
    @"error": ?LSPError = null,
};

pub const LSPError = struct {
    code: i32,
    message: []const u8,
    data: ?json.Value = null,
};

// LSP Data Structures
pub const Position = struct {
    line: u32,
    character: u32,
};

pub const Range = struct {
    start: Position,
    end: Position,
};

pub const Location = struct {
    uri: []const u8,
    range: Range,
};

pub const TextDocumentIdentifier = struct {
    uri: []const u8,
};

pub const VersionedTextDocumentIdentifier = struct {
    uri: []const u8,
    version: i32,
};

pub const TextDocumentItem = struct {
    uri: []const u8,
    languageId: []const u8,
    version: i32,
    text: []const u8,
};

pub const TextDocumentContentChangeEvent = struct {
    range: ?Range = null,
    rangeLength: ?u32 = null,
    text: []const u8,
};

pub const Diagnostic = struct {
    range: Range,
    severity: ?DiagnosticSeverity = null,
    code: ?json.Value = null,
    source: ?[]const u8 = null,
    message: []const u8,
    relatedInformation: ?[]DiagnosticRelatedInformation = null,
};

pub const DiagnosticSeverity = enum(u8) {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
};

pub const DiagnosticRelatedInformation = struct {
    location: Location,
    message: []const u8,
};

pub const CompletionItem = struct {
    label: []const u8,
    kind: ?CompletionItemKind = null,
    detail: ?[]const u8 = null,
    documentation: ?[]const u8 = null,
    sortText: ?[]const u8 = null,
    filterText: ?[]const u8 = null,
    insertText: ?[]const u8 = null,
    insertTextFormat: ?InsertTextFormat = null,
};

pub const CompletionItemKind = enum(u8) {
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

pub const InsertTextFormat = enum(u8) {
    PlainText = 1,
    Snippet = 2,
};

pub const Hover = struct {
    contents: []const u8,
    range: ?Range = null,
};

pub const DocumentSymbol = struct {
    name: []const u8,
    detail: ?[]const u8 = null,
    kind: SymbolKind,
    deprecated: ?bool = null,
    range: Range,
    selectionRange: Range,
    children: ?[]DocumentSymbol = null,
};

pub const SymbolKind = enum(u8) {
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

// Simplified Token for LSP without external dependencies
pub const SimpleToken = struct {
    kind: TokenKind,
    lexeme: []const u8,
    line: u32,
    column: u32,
};

pub const TokenKind = enum {
    // Keywords
    Slay, Sus, Facts, Lowkey, Highkey, Periodt, Stan, Bestie, Flex,
    Ghosted, Simp, Squad, Collab, Yeet, Vibes, Mood, Basic, Match,
    Based, Cringe, Normie, Tea, Lit, Drip, Thicc, Smol, Meal,
    Yikes, Shook, Fam, Spill, Ready, Later, Dm, Select,
    
    // Traditional keywords  
    Fn, Let, Mut, If, Else, While, For, Return, Struct, Interface,
    Import, Package, True, False, Nil, Switch, Case, Default,
    
    // Literals
    Identifier, String, Number,
    
    // Operators
    Assign, Plus, Minus, Star, Slash, Percent,
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    And, Or, Not,
    
    // Delimiters
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
    Comma, Semicolon, Colon, Dot, Arrow,
    
    // Special
    Newline, Eof, Error,
};

// Document Management
pub const DocumentInfo = struct {
    uri: []const u8,
    version: i32,
    content: []const u8,
    tokens: []SimpleToken,
    diagnostics: ArrayList(Diagnostic),
    symbols: ArrayList(DocumentSymbol),
    allocator: Allocator,

    pub fn init(allocator: Allocator, uri: []const u8, version: i32, content: []const u8) !DocumentInfo {
        const uri_copy = try allocator.dupe(u8, uri);
        const content_copy = try allocator.dupe(u8, content);
        
        return DocumentInfo{
            .uri = uri_copy,
            .version = version,
            .content = content_copy,
            .tokens = &[_]SimpleToken{},
            .diagnostics = ArrayList(Diagnostic).init(allocator),
            .symbols = ArrayList(DocumentSymbol).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *DocumentInfo) void {
        self.allocator.free(self.uri);
        self.allocator.free(self.content);
        self.allocator.free(self.tokens);
        self.diagnostics.deinit();
        self.symbols.deinit();
    }

    pub fn updateContent(self: *DocumentInfo, new_content: []const u8, new_version: i32) !void {
        self.allocator.free(self.content);
        self.content = try self.allocator.dupe(u8, new_content);
        self.version = new_version;
        
        // Clear cached data
        self.allocator.free(self.tokens);
        self.tokens = &[_]SimpleToken{};
        self.diagnostics.clearRetainingCapacity();
        self.symbols.clearRetainingCapacity();
    }

    pub fn parse(self: *DocumentInfo) !void {
        // Simple tokenization for LSP features
        var tokens = ArrayList(SimpleToken).init(self.allocator);
        defer tokens.deinit();

        var line: u32 = 0;
        var column: u32 = 0;
        var i: usize = 0;
        
        while (i < self.content.len) {
            const c = self.content[i];
            
            // Skip whitespace except newlines
            if (c == ' ' or c == '\t' or c == '\r') {
                column += 1;
                i += 1;
                continue;
            }
            
            if (c == '\n') {
                const token = SimpleToken{
                    .kind = .Newline,
                    .lexeme = self.content[i..i+1],
                    .line = line,
                    .column = column,
                };
                try tokens.append(token);
                line += 1;
                column = 0;
                i += 1;
                continue;
            }
            
            // Simple identifier/keyword recognition
            if (std.ascii.isAlphabetic(c) or c == '_') {
                const start = i;
                while (i < self.content.len and (std.ascii.isAlphaNumeric(self.content[i]) or self.content[i] == '_')) {
                    i += 1;
                }
                const lexeme = self.content[start..i];
                const kind = self.identifyKeyword(lexeme);
                const token = SimpleToken{
                    .kind = kind,
                    .lexeme = lexeme,
                    .line = line,
                    .column = column,
                };
                try tokens.append(token);
                column += @intCast(lexeme.len);
                continue;
            }
            
            // Simple string recognition
            if (c == '"') {
                const start = i;
                i += 1; // Skip opening quote
                while (i < self.content.len and self.content[i] != '"') {
                    i += 1;
                }
                if (i < self.content.len) i += 1; // Skip closing quote
                const lexeme = self.content[start..i];
                const token = SimpleToken{
                    .kind = .String,
                    .lexeme = lexeme,
                    .line = line,
                    .column = column,
                };
                try tokens.append(token);
                column += @intCast(lexeme.len);
                continue;
            }
            
            // Simple number recognition
            if (std.ascii.isDigit(c)) {
                const start = i;
                while (i < self.content.len and std.ascii.isDigit(self.content[i])) {
                    i += 1;
                }
                const lexeme = self.content[start..i];
                const token = SimpleToken{
                    .kind = .Number,
                    .lexeme = lexeme,
                    .line = line,
                    .column = column,
                };
                try tokens.append(token);
                column += @intCast(lexeme.len);
                continue;
            }
            
            // Single character tokens
            const kind: TokenKind = switch (c) {
                '(' => .LeftParen,
                ')' => .RightParen,
                '{' => .LeftBrace,
                '}' => .RightBrace,
                '[' => .LeftBracket,
                ']' => .RightBracket,
                ',' => .Comma,
                ';' => .Semicolon,
                ':' => .Colon,
                '.' => .Dot,
                '+' => .Plus,
                '-' => .Minus,
                '*' => .Star,
                '/' => .Slash,
                '%' => .Percent,
                '=' => .Assign,
                '<' => .Less,
                '>' => .Greater,
                '!' => .Not,
                else => .Error,
            };
            
            const token = SimpleToken{
                .kind = kind,
                .lexeme = self.content[i..i+1],
                .line = line,
                .column = column,
            };
            try tokens.append(token);
            column += 1;
            i += 1;
        }
        
        // Add EOF token
        const eof_token = SimpleToken{
            .kind = .Eof,
            .lexeme = "",
            .line = line,
            .column = column,
        };
        try tokens.append(eof_token);

        self.tokens = try tokens.toOwnedSlice();

        // Extract symbols from tokens
        try self.extractSymbols();
    }
    
    fn identifyKeyword(self: *DocumentInfo, lexeme: []const u8) TokenKind {
        _ = self;
        
        // CURSED Gen Z keywords
        if (std.mem.eql(u8, lexeme, "slay")) return .Slay;
        if (std.mem.eql(u8, lexeme, "sus")) return .Sus;
        if (std.mem.eql(u8, lexeme, "facts")) return .Facts;
        if (std.mem.eql(u8, lexeme, "based")) return .Based;
        if (std.mem.eql(u8, lexeme, "cringe")) return .Cringe;
        if (std.mem.eql(u8, lexeme, "normie")) return .Normie;
        if (std.mem.eql(u8, lexeme, "tea")) return .Tea;
        if (std.mem.eql(u8, lexeme, "lit")) return .Lit;
        if (std.mem.eql(u8, lexeme, "drip")) return .Drip;
        if (std.mem.eql(u8, lexeme, "thicc")) return .Thicc;
        if (std.mem.eql(u8, lexeme, "smol")) return .Smol;
        if (std.mem.eql(u8, lexeme, "meal")) return .Meal;
        if (std.mem.eql(u8, lexeme, "squad")) return .Squad;
        if (std.mem.eql(u8, lexeme, "collab")) return .Collab;
        if (std.mem.eql(u8, lexeme, "yeet")) return .Yeet;
        if (std.mem.eql(u8, lexeme, "vibes")) return .Vibes;
        if (std.mem.eql(u8, lexeme, "bestie")) return .Bestie;
        if (std.mem.eql(u8, lexeme, "stan")) return .Stan;
        
        // Traditional keywords
        if (std.mem.eql(u8, lexeme, "fn")) return .Fn;
        if (std.mem.eql(u8, lexeme, "let")) return .Let;
        if (std.mem.eql(u8, lexeme, "mut")) return .Mut;
        if (std.mem.eql(u8, lexeme, "if")) return .If;
        if (std.mem.eql(u8, lexeme, "else")) return .Else;
        if (std.mem.eql(u8, lexeme, "while")) return .While;
        if (std.mem.eql(u8, lexeme, "for")) return .For;
        if (std.mem.eql(u8, lexeme, "return")) return .Return;
        if (std.mem.eql(u8, lexeme, "struct")) return .Struct;
        if (std.mem.eql(u8, lexeme, "interface")) return .Interface;
        if (std.mem.eql(u8, lexeme, "import")) return .Import;
        if (std.mem.eql(u8, lexeme, "true")) return .True;
        if (std.mem.eql(u8, lexeme, "false")) return .False;
        if (std.mem.eql(u8, lexeme, "nil")) return .Nil;
        
        return .Identifier;
    }

    fn extractSymbols(self: *DocumentInfo) !void {
        // Simple symbol extraction from tokens
        var i: usize = 0;
        while (i < self.tokens.len) {
            const token = self.tokens[i];
            
            // Look for function definitions (slay keyword followed by identifier)
            if (token.kind == .Slay and i + 1 < self.tokens.len) {
                const next_token = self.tokens[i + 1];
                if (next_token.kind == .Identifier) {
                    const symbol = DocumentSymbol{
                        .name = next_token.lexeme,
                        .kind = .Function,
                        .range = createSafeRange(token.line, token.column, token.line + 1, 0),
                        .selectionRange = createSafeRange(next_token.line, next_token.column, next_token.line, next_token.column + next_token.lexeme.len),
                    };
                    try self.symbols.append(symbol);
                }
            }
            
            // Look for struct definitions (squad keyword followed by identifier)
            if (token.kind == .Squad and i + 1 < self.tokens.len) {
                const next_token = self.tokens[i + 1];
                if (next_token.kind == .Identifier) {
                    const symbol = DocumentSymbol{
                        .name = next_token.lexeme,
                        .kind = .Struct,
                        .range = createSafeRange(token.line, token.column, token.line + 1, 0),
                        .selectionRange = createSafeRange(next_token.line, next_token.column, next_token.line, next_token.column + next_token.lexeme.len),
                    };
                    try self.symbols.append(symbol);
                }
            }
            
            // Look for interface definitions (collab keyword followed by identifier)
            if (token.kind == .Collab and i + 1 < self.tokens.len) {
                const next_token = self.tokens[i + 1];
                if (next_token.kind == .Identifier) {
                    const symbol = DocumentSymbol{
                        .name = next_token.lexeme,
                        .kind = .Interface,
                        .range = createSafeRange(token.line, token.column, token.line + 1, 0),
                        .selectionRange = createSafeRange(next_token.line, next_token.column, next_token.line, next_token.column + next_token.lexeme.len),
                    };
                    try self.symbols.append(symbol);
                }
            }
            
            // Look for variable declarations (sus keyword followed by identifier)
            if (token.kind == .Sus and i + 1 < self.tokens.len) {
                const next_token = self.tokens[i + 1];
                if (next_token.kind == .Identifier) {
                    const symbol = DocumentSymbol{
                        .name = next_token.lexeme,
                        .kind = .Variable,
                        .range = createSafeRange(token.line, token.column, token.line + 1, 0),
                        .selectionRange = createSafeRange(next_token.line, next_token.column, next_token.line, next_token.column + next_token.lexeme.len),
                    };
                    try self.symbols.append(symbol);
                }
            }
            
            i += 1;
        }
    }
};

// CURSED Language Features Database
pub const CursedLanguageData = struct {
    keywords: []const []const u8,
    stdlib_functions: []const StdlibFunction,
    types: []const []const u8,

    const StdlibFunction = struct {
        name: []const u8,
        module: []const u8,
        signature: []const u8,
        description: []const u8,
        example: []const u8,
    };

    pub fn init() CursedLanguageData {
        return CursedLanguageData{
            .keywords = &[_][]const u8{
                // CURSED Gen Z Keywords
                "slay", "sus", "facts", "lowkey", "highkey", "periodt", "stan", "bestie", "flex",
                "ghosted", "simp", "squad", "collab", "yeet", "vibes", "mood", "basic", "match",
                "based", "cringe", "normie", "tea", "lit", "drip", "thicc", "smol", "meal",
                "yikes", "shook", "fam", "spill", "ready", "later", "dm", "select",
                
                // Traditional Keywords
                "fn", "let", "mut", "if", "else", "while", "for", "return", "struct", "interface",
                "import", "package", "true", "false", "nil", "switch", "case", "default",
            },
            .stdlib_functions = &[_]StdlibFunction{
                .{ .name = "spill", .module = "vibez", .signature = "slay spill(message tea)", .description = "Print message to stdout", .example = "vibez.spill(\"Hello CURSED!\")" },
                .{ .name = "spillf", .module = "vibez", .signature = "slay spillf(format tea, args ...interface{})", .description = "Print formatted message", .example = "vibez.spillf(\"Value: {}\", 42)" },
                .{ .name = "read_line", .module = "vibez", .signature = "slay read_line() tea", .description = "Read line from stdin", .example = "sus input tea = vibez.read_line()" },
                .{ .name = "len", .module = "core", .signature = "slay len(arr []T) normie", .description = "Get length of array/slice", .example = "sus length normie = len(my_array)" },
                .{ .name = "append", .module = "core", .signature = "slay append(arr []T, item T) []T", .description = "Append item to array", .example = "my_array = append(my_array, new_item)" },
                .{ .name = "make", .module = "core", .signature = "slay make(T, size normie) []T", .description = "Create array of specified size", .example = "sus arr []normie = make(normie, 10)" },
                .{ .name = "hash", .module = "cryptz", .signature = "slay hash(data []byte) []byte", .description = "Hash data using SHA-256", .example = "sus hashed []byte = cryptz.hash(data)" },
                .{ .name = "encrypt", .module = "cryptz", .signature = "slay encrypt(data []byte, key []byte) []byte", .description = "Encrypt data with AES", .example = "sus encrypted []byte = cryptz.encrypt(data, key)" },
                .{ .name = "spawn", .module = "concurrenz", .signature = "slay spawn(fn slay()) dm", .description = "Start goroutine", .example = "concurrenz.spawn(() => { vibez.spill(\"goroutine!\") })" },
                .{ .name = "send", .module = "concurrenz", .signature = "slay send(ch dm T, value T)", .description = "Send value to channel", .example = "concurrenz.send(ch, value)" },
                .{ .name = "receive", .module = "concurrenz", .signature = "slay receive(ch dm T) T", .description = "Receive value from channel", .example = "sus value T = concurrenz.receive(ch)" },
            },
            .types = &[_][]const u8{
                "normie", "tea", "lit", "drip", "thicc", "smol", "meal", "byte", "rune",
                "[]normie", "[]tea", "[]byte", "dm", "interface{}", "map[tea]normie",
            },
        };
    }
};

// Main LSP Handler
pub const LSPHandler = struct {
    allocator: Allocator,
    documents: HashMap([]const u8, DocumentInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    language_data: CursedLanguageData,
    initialized: bool,
    shutdown_requested: bool,

    pub fn init() LSPHandler {
        return LSPHandler{
            .allocator = allocator,
            .documents = HashMap([]const u8, DocumentInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .language_data = CursedLanguageData.init(),
            .initialized = false,
            .shutdown_requested = false,
        };
    }

    pub fn deinit(self: *LSPHandler) void {
        var iterator = self.documents.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.documents.deinit();
    }

    pub fn handleMessage(self: *LSPHandler, message_text: []const u8) !?[]u8 {
        var parsed = json.parseFromSlice(json.Value, self.allocator, message_text, .{}) catch |err| {
            std.log.err("Failed to parse LSP message: {}", .{err});
            return null;
        };
        defer parsed.deinit();

        const message_obj = parsed.value.object;
        
        if (message_obj.get("method")) |method_value| {
            const method = method_value.string;
            const params = message_obj.get("params");
            const id = message_obj.get("id");
            
            if (id != null) {
                // Request
                return try self.handleRequest(method, params, id.?);
            } else {
                // Notification
                try self.handleNotification(method, params);
                return null;
            }
        }
        
        return null;
    }

    fn handleRequest(self: *LSPHandler, method: []const u8, params: ?json.Value, id: json.Value) ![]u8 {
        if (std.mem.eql(u8, method, "initialize")) {
            return try self.handleInitialize(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/completion")) {
            return try self.handleCompletion(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/hover")) {
            return try self.handleHover(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/definition")) {
            return try self.handleDefinition(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/references")) {
            return try self.handleReferences(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/documentSymbol")) {
            return try self.handleDocumentSymbol(params, id);
        } else if (std.mem.eql(u8, method, "workspace/symbol")) {
            return try self.handleWorkspaceSymbol(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/signatureHelp")) {
            return try self.handleSignatureHelp(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/formatting")) {
            return try self.handleFormatting(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/rangeFormatting")) {
            return try self.handleRangeFormatting(params, id);
        } else if (std.mem.eql(u8, method, "textDocument/rename")) {
            return try self.handleRename(params, id);
        } else if (std.mem.eql(u8, method, "shutdown")) {
            return try self.handleShutdown(id);
        }
        
        // Method not supported
        const error_response = LSPMessage{
            .id = id,
            .@"error" = LSPError{
                .code = -32601,
                .message = "Method not found",
            },
        };
        
        return try json.stringify(error_response, .{}, self.allocator);
    }

    fn handleNotification(self: *LSPHandler, method: []const u8, params: ?json.Value) !void {
        if (std.mem.eql(u8, method, "initialized")) {
            self.initialized = true;
            std.log.info("LSP client initialized", .{});
        } else if (std.mem.eql(u8, method, "textDocument/didOpen")) {
            try self.handleDidOpen(params);
        } else if (std.mem.eql(u8, method, "textDocument/didChange")) {
            try self.handleDidChange(params);
        } else if (std.mem.eql(u8, method, "textDocument/didSave")) {
            try self.handleDidSave(params);
        } else if (std.mem.eql(u8, method, "textDocument/didClose")) {
            try self.handleDidClose(params);
        } else if (std.mem.eql(u8, method, "exit")) {
            std.log.info("LSP client exiting", .{});
        }
    }

    fn handleInitialize(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        _ = params;
        
        const capabilities = json.ObjectMap.init(self.allocator);
        defer capabilities.deinit();
        
        // Text document sync
        try capabilities.put("textDocumentSync", json.Value{ .Integer = 1 }); // Full sync
        
        // Completion support
        const completion_provider = json.ObjectMap.init(self.allocator);
        defer completion_provider.deinit();
        try completion_provider.put("triggerCharacters", json.Value{ .Array = json.Array.init(self.allocator) });
        try capabilities.put("completionProvider", json.Value{ .Object = completion_provider });
        
        // Hover support
        try capabilities.put("hoverProvider", json.Value{ .Bool = true });
        
        // Definition support
        try capabilities.put("definitionProvider", json.Value{ .Bool = true });
        
        // References support
        try capabilities.put("referencesProvider", json.Value{ .Bool = true });
        
        // Document symbol support
        try capabilities.put("documentSymbolProvider", json.Value{ .Bool = true });
        
        // Workspace symbol support
        try capabilities.put("workspaceSymbolProvider", json.Value{ .Bool = true });
        
        // Signature help support
        const signature_provider = json.ObjectMap.init(self.allocator);
        defer signature_provider.deinit();
        try signature_provider.put("triggerCharacters", json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, &[_]json.Value{json.Value{ .String = "(" }}) });
        try capabilities.put("signatureHelpProvider", json.Value{ .Object = signature_provider });
        
        // Formatting support
        try capabilities.put("documentFormattingProvider", json.Value{ .Bool = true });
        try capabilities.put("documentRangeFormattingProvider", json.Value{ .Bool = true });
        
        // Rename support
        const rename_provider = json.ObjectMap.init(self.allocator);
        defer rename_provider.deinit();
        try rename_provider.put("prepareProvider", json.Value{ .Bool = true });
        try capabilities.put("renameProvider", json.Value{ .Object = rename_provider });
        
        // Diagnostics support
        try capabilities.put("publishDiagnostics", json.Value{ .Bool = true });
        
        const result = json.ObjectMap.init(self.allocator);
        defer result.deinit();
        try result.put("capabilities", json.Value{ .Object = capabilities });
        
        const server_info = json.ObjectMap.init(self.allocator);
        defer server_info.deinit();
        try server_info.put("name", json.Value{ .String = "cursed-lsp" });
        try server_info.put("version", json.Value{ .String = "1.0.0" });
        try result.put("serverInfo", json.Value{ .Object = server_info });
        
        const response = LSPMessage{
            .id = id,
            .result = json.Value{ .Object = result },
        };
        
        return try json.stringify(response, .{}, self.allocator);
    }

    fn handleCompletion(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const position = params_obj.get("position").?.Object;
        
        const uri = text_document.get("uri").?.String;
        _ = @as(u32, @intCast(position.get("line").?.Integer));
        _ = @as(u32, @intCast(position.get("character").?.Integer));
        
        var items = ArrayList(CompletionItem).init(self.allocator);
        defer items.deinit();
        
        // Add keywords
        for (self.language_data.keywords) |keyword| {
            try items.append(CompletionItem{
                .label = keyword,
                .kind = .Keyword,
                .detail = "CURSED keyword",
                .insertText = keyword,
            });
        }
        
        // Add stdlib functions
        for (self.language_data.stdlib_functions) |func| {
            try items.append(CompletionItem{
                .label = func.name,
                .kind = .Function,
                .detail = func.signature,
                .documentation = func.description,
                .insertText = func.name,
            });
        }
        
        // Add types
        for (self.language_data.types) |type_name| {
            try items.append(CompletionItem{
                .label = type_name,
                .kind = .TypeParameter,
                .detail = "CURSED type",
                .insertText = type_name,
            });
        }
        
        // Add symbols from current document
        if (self.documents.get(uri)) |doc| {
            for (doc.symbols.items) |symbol| {
                try items.append(CompletionItem{
                    .label = symbol.name,
                    .kind = switch (symbol.kind) {
                        .Function => .Function,
                        .Variable => .Variable,
                        .Constant => .Constant,
                        .Struct => .Struct,
                        .Interface => .Interface,
                        else => .Text,
                    },
                    .detail = symbol.detail orelse "User symbol",
                    .insertText = symbol.name,
                });
            }
        }
        
        const items_json = try self.completionItemsToJson(items.items);
        defer self.allocator.free(items_json);
        
        const response = LSPMessage{
            .id = id,
            .result = items_json,
        };
        
        return try json.stringify(response, .{}, self.allocator);
    }

    fn handleHover(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const position = params_obj.get("position").?.Object;
        
        const uri = text_document.get("uri").?.String;
        const line = @as(u32, @intCast(position.get("line").?.Integer));
        const character = @as(u32, @intCast(position.get("character").?.Integer));
        
        // Find word at position
        if (self.documents.get(uri)) |doc| {
            const word = try self.getWordAtPosition(doc.content, line, character);
            defer self.allocator.free(word);
            
            var hover_content: []const u8 = "No information available";
            
            // Check if it's a keyword
            for (self.language_data.keywords) |keyword| {
                if (std.mem.eql(u8, word, keyword)) {
                    hover_content = try std.fmt.allocPrint(self.allocator, "CURSED keyword: `{s}`", .{keyword});
                    break;
                }
            }
            
            // Check if it's a stdlib function
            for (self.language_data.stdlib_functions) |func| {
                if (std.mem.eql(u8, word, func.name)) {
                    hover_content = try std.fmt.allocPrint(self.allocator, 
                        "**{s}.{s}**\n\n{s}\n\n```cursed\n{s}\n```\n\nExample:\n```cursed\n{s}\n```", 
                        .{ func.module, func.name, func.description, func.signature, func.example }
                    );
                    break;
                }
            }
            
            // Check if it's a type
            for (self.language_data.types) |type_name| {
                if (std.mem.eql(u8, word, type_name)) {
                    hover_content = try std.fmt.allocPrint(self.allocator, "CURSED type: `{s}`", .{type_name});
                    break;
                }
            }
            
            // Check symbols in current document
            for (doc.symbols.items) |symbol| {
                if (std.mem.eql(u8, word, symbol.name)) {
                    hover_content = try std.fmt.allocPrint(self.allocator, 
                        "Symbol: `{s}`\nKind: {s}\n{s}", 
                        .{ symbol.name, @tagName(symbol.kind), symbol.detail orelse "" }
                    );
                    break;
                }
            }
            
            const hover_obj = json.ObjectMap.init(self.allocator);
            defer hover_obj.deinit();
            try hover_obj.put("contents", json.Value{ .String = hover_content });
            
            const response = LSPMessage{
                .id = id,
                .result = json.Value{ .Object = hover_obj },
            };
            
            return try json.stringify(response, .{}, self.allocator);
        }
        
        return try self.createErrorResponse(id, -32603, "Document not found");
    }

    fn handleDefinition(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const position = params_obj.get("position").?.Object;
        
        const uri = text_document.get("uri").?.String;
        const line = @as(u32, @intCast(position.get("line").?.Integer));
        const character = @as(u32, @intCast(position.get("character").?.Integer));
        
        if (self.documents.get(uri)) |doc| {
            const word = try self.getWordAtPosition(doc.content, line, character);
            defer self.allocator.free(word);
            
            // Find definition in symbols
            for (doc.symbols.items) |symbol| {
                if (std.mem.eql(u8, word, symbol.name)) {
                    const location_obj = json.ObjectMap.init(self.allocator);
                    defer location_obj.deinit();
                    try location_obj.put("uri", json.Value{ .String = uri });
                    
                    const range_obj = json.ObjectMap.init(self.allocator);
                    defer range_obj.deinit();
                    
                    const start_obj = json.ObjectMap.init(self.allocator);
                    defer start_obj.deinit();
                    try start_obj.put("line", json.Value{ .Integer = @intCast(symbol.selectionRange.start.line) });
                    try start_obj.put("character", json.Value{ .Integer = @intCast(symbol.selectionRange.start.character) });
                    
                    const end_obj = json.ObjectMap.init(self.allocator);
                    defer end_obj.deinit();
                    try end_obj.put("line", json.Value{ .Integer = @intCast(symbol.selectionRange.end.line) });
                    try end_obj.put("character", json.Value{ .Integer = @intCast(symbol.selectionRange.end.character) });
                    
                    try range_obj.put("start", json.Value{ .Object = start_obj });
                    try range_obj.put("end", json.Value{ .Object = end_obj });
                    try location_obj.put("range", json.Value{ .Object = range_obj });
                    
                    const response = LSPMessage{
                        .id = id,
                        .result = json.Value{ .Object = location_obj },
                    };
                    
                    return try json.stringify(response, .{}, self.allocator);
                }
            }
        }
        
        // No definition found
        const response = LSPMessage{
            .id = id,
            .result = json.Value.Null,
        };
        
        return try json.stringify(response, .{}, self.allocator);
    }

    fn handleDocumentSymbol(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const uri = text_document.get("uri").?.String;
        
        if (self.documents.get(uri)) |doc| {
            const symbols_json = try self.symbolsToJson(doc.symbols.items);
            defer self.allocator.free(symbols_json);
            
            const response = LSPMessage{
                .id = id,
                .result = symbols_json,
            };
            
            return try json.stringify(response, .{}, self.allocator);
        }
        
        return try self.createErrorResponse(id, -32603, "Document not found");
    }

    fn handleFormatting(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const uri = text_document.get("uri").?.String;
        
        if (self.documents.get(uri)) |doc| {
            // Basic formatting: fix indentation and spacing
            const formatted = try self.formatDocument(doc.content);
            defer self.allocator.free(formatted);
            
            const edit_obj = json.ObjectMap.init(self.allocator);
            defer edit_obj.deinit();
            
            // Replace entire document
            const range_obj = json.ObjectMap.init(self.allocator);
            defer range_obj.deinit();
            
            const start_obj = json.ObjectMap.init(self.allocator);
            defer start_obj.deinit();
            try start_obj.put("line", json.Value{ .Integer = 0 });
            try start_obj.put("character", json.Value{ .Integer = 0 });
            
            const end_obj = json.ObjectMap.init(self.allocator);
            defer end_obj.deinit();
            const lines = std.mem.count(u8, doc.content, "\n");
            try end_obj.put("line", json.Value{ .Integer = @intCast(lines) });
            try end_obj.put("character", json.Value{ .Integer = 0 });
            
            try range_obj.put("start", json.Value{ .Object = start_obj });
            try range_obj.put("end", json.Value{ .Object = end_obj });
            try edit_obj.put("range", json.Value{ .Object = range_obj });
            try edit_obj.put("newText", json.Value{ .String = formatted });
            
            const edits = [_]json.Value{json.Value{ .Object = edit_obj }};
            
            const response = LSPMessage{
                .id = id,
                .result = json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, &edits) },
            };
            
            return try json.stringify(response, .{}, self.allocator);
        }
        
        return try self.createErrorResponse(id, -32603, "Document not found");
    }

    fn handleShutdown(self: *LSPHandler, id: json.Value) ![]u8 {
        self.shutdown_requested = true;
        
        const response = LSPMessage{
            .id = id,
            .result = json.Value.Null,
        };
        
        return try json.stringify(response, .{}, self.allocator);
    }

    fn handleReferences(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const position = params_obj.get("position").?.Object;
        
        const uri = text_document.get("uri").?.String;
        const line = @as(u32, @intCast(position.get("line").?.Integer));
        const character = @as(u32, @intCast(position.get("character").?.Integer));
        
        if (self.documents.get(uri)) |doc| {
            const word = try self.getWordAtPosition(doc.content, line, character);
            defer self.allocator.free(word);
            
            var locations = ArrayList(json.Value).init(self.allocator);
            defer locations.deinit();
            
            // Find all references to this word in the document
            var current_line: u32 = 0;
            var current_char: u32 = 0;
            var i: usize = 0;
            
            while (i < doc.content.len) {
                if (std.mem.startsWith(u8, doc.content[i..], word)) {
                    // Found a match, create location
                    const location_obj = json.ObjectMap.init(self.allocator);
                    defer location_obj.deinit();
                    
                    try location_obj.put("uri", json.Value{ .String = uri });
                    
                    const range_obj = json.ObjectMap.init(self.allocator);
                    defer range_obj.deinit();
                    
                    const start_obj = json.ObjectMap.init(self.allocator);
                    defer start_obj.deinit();
                    try start_obj.put("line", json.Value{ .Integer = @intCast(current_line) });
                    try start_obj.put("character", json.Value{ .Integer = @intCast(current_char) });
                    
                    const end_obj = json.ObjectMap.init(self.allocator);
                    defer end_obj.deinit();
                    try end_obj.put("line", json.Value{ .Integer = @intCast(current_line) });
                    try end_obj.put("character", json.Value{ .Integer = @intCast(current_char + word.len) });
                    
                    try range_obj.put("start", json.Value{ .Object = start_obj });
                    try range_obj.put("end", json.Value{ .Object = end_obj });
                    try location_obj.put("range", json.Value{ .Object = range_obj });
                    
                    try locations.append(json.Value{ .Object = location_obj });
                    
                    i += word.len;
                    current_char += @intCast(word.len);
                } else {
                    if (doc.content[i] == '\n') {
                        current_line += 1;
                        current_char = 0;
                    } else {
                        current_char += 1;
                    }
                    i += 1;
                }
            }
            
            const response = LSPMessage{
                .id = id,
                .result = json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, try locations.toOwnedSlice()) },
            };
            
            return try json.stringify(response, .{}, self.allocator);
        }
        
        return try self.createErrorResponse(id, -32603, "Document not found");
    }

    fn handleWorkspaceSymbol(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const query = if (params_obj.get("query")) |q| q.String else "";
        
        var symbols = ArrayList(json.Value).init(self.allocator);
        defer symbols.deinit();
        
        // Search through all documents
        var doc_iterator = self.documents.iterator();
        while (doc_iterator.next()) |entry| {
            const doc = entry.value_ptr.*;
            
            for (doc.symbols.items) |symbol| {
                // Simple query matching
                if (query.len == 0 or std.mem.indexOf(u8, symbol.name, query) != null) {
                    const symbol_obj = json.ObjectMap.init(self.allocator);
                    defer symbol_obj.deinit();
                    
                    try symbol_obj.put("name", json.Value{ .String = symbol.name });
                    try symbol_obj.put("kind", json.Value{ .Integer = @intFromEnum(symbol.kind) });
                    
                    const location_obj = json.ObjectMap.init(self.allocator);
                    defer location_obj.deinit();
                    try location_obj.put("uri", json.Value{ .String = doc.uri });
                    
                    const range_obj = json.ObjectMap.init(self.allocator);
                    defer range_obj.deinit();
                    
                    const start_obj = json.ObjectMap.init(self.allocator);
                    defer start_obj.deinit();
                    try start_obj.put("line", json.Value{ .Integer = @intCast(symbol.range.start.line) });
                    try start_obj.put("character", json.Value{ .Integer = @intCast(symbol.range.start.character) });
                    
                    const end_obj = json.ObjectMap.init(self.allocator);
                    defer end_obj.deinit();
                    try end_obj.put("line", json.Value{ .Integer = @intCast(symbol.range.end.line) });
                    try end_obj.put("character", json.Value{ .Integer = @intCast(symbol.range.end.character) });
                    
                    try range_obj.put("start", json.Value{ .Object = start_obj });
                    try range_obj.put("end", json.Value{ .Object = end_obj });
                    try location_obj.put("range", json.Value{ .Object = range_obj });
                    try symbol_obj.put("location", json.Value{ .Object = location_obj });
                    
                    try symbols.append(json.Value{ .Object = symbol_obj });
                }
            }
        }
        
        const response = LSPMessage{
            .id = id,
            .result = json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, try symbols.toOwnedSlice()) },
        };
        
        return try json.stringify(response, .{}, self.allocator);
    }

    fn handleSignatureHelp(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const position = params_obj.get("position").?.Object;
        
        const uri = text_document.get("uri").?.String;
        const line = @as(u32, @intCast(position.get("line").?.Integer));
        const character = @as(u32, @intCast(position.get("character").?.Integer));
        
        if (self.documents.get(uri)) |doc| {
            const word = try self.getWordAtPosition(doc.content, line, character);
            defer self.allocator.free(word);
            
            // Find matching function signature
            for (self.language_data.stdlib_functions) |func| {
                if (std.mem.eql(u8, func.name, word)) {
                    const signature_obj = json.ObjectMap.init(self.allocator);
                    defer signature_obj.deinit();
                    
                    try signature_obj.put("label", json.Value{ .String = func.signature });
                    try signature_obj.put("documentation", json.Value{ .String = func.description });
                    
                    const signatures = [_]json.Value{json.Value{ .Object = signature_obj }};
                    
                    const result_obj = json.ObjectMap.init(self.allocator);
                    defer result_obj.deinit();
                    try result_obj.put("signatures", json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, &signatures) });
                    try result_obj.put("activeSignature", json.Value{ .Integer = 0 });
                    try result_obj.put("activeParameter", json.Value{ .Integer = 0 });
                    
                    const response = LSPMessage{
                        .id = id,
                        .result = json.Value{ .Object = result_obj },
                    };
                    
                    return try json.stringify(response, .{}, self.allocator);
                }
            }
        }
        
        // No signature found
        const response = LSPMessage{
            .id = id,
            .result = json.Value.Null,
        };
        
        return try json.stringify(response, .{}, self.allocator);
    }

    fn handleRangeFormatting(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const range = params_obj.get("range").?.Object;
        
        const uri = text_document.get("uri").?.String;
        const start_line = @as(u32, @intCast(range.get("start").?.Object.get("line").?.Integer));
        const end_line = @as(u32, @intCast(range.get("end").?.Object.get("line").?.Integer));
        
        if (self.documents.get(uri)) |doc| {
            // Extract the range content and format it
            var lines = std.mem.split(u8, doc.content, "\n");
            var range_content = ArrayList(u8).init(self.allocator);
            defer range_content.deinit();
            
            var current_line: u32 = 0;
            while (lines.next()) |line| {
                if (current_line >= start_line and current_line <= end_line) {
                    try range_content.appendSlice(line);
                    if (current_line < end_line) {
                        try range_content.append('\n');
                    }
                }
                current_line += 1;
            }
            
            const formatted = try self.formatDocument(range_content.items);
            defer self.allocator.free(formatted);
            
            const edit_obj = json.ObjectMap.init(self.allocator);
            defer edit_obj.deinit();
            try edit_obj.put("range", json.Value{ .Object = range.Object });
            try edit_obj.put("newText", json.Value{ .String = formatted });
            
            const edits = [_]json.Value{json.Value{ .Object = edit_obj }};
            
            const response = LSPMessage{
                .id = id,
                .result = json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, &edits) },
            };
            
            return try json.stringify(response, .{}, self.allocator);
        }
        
        return try self.createErrorResponse(id, -32603, "Document not found");
    }

    fn handleRename(self: *LSPHandler, params: ?json.Value, id: json.Value) ![]u8 {
        if (params == null) {
            return try self.createErrorResponse(id, -32602, "Invalid params");
        }
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const position = params_obj.get("position").?.Object;
        const new_name = params_obj.get("newName").?.String;
        
        const uri = text_document.get("uri").?.String;
        const line = @as(u32, @intCast(position.get("line").?.Integer));
        const character = @as(u32, @intCast(position.get("character").?.Integer));
        
        if (self.documents.get(uri)) |doc| {
            const old_name = try self.getWordAtPosition(doc.content, line, character);
            defer self.allocator.free(old_name);
            
            var edits = ArrayList(json.Value).init(self.allocator);
            defer edits.deinit();
            
            // Find all occurrences and create edits
            var current_line: u32 = 0;
            var current_char: u32 = 0;
            var i: usize = 0;
            
            while (i < doc.content.len) {
                if (std.mem.startsWith(u8, doc.content[i..], old_name)) {
                    const edit_obj = json.ObjectMap.init(self.allocator);
                    defer edit_obj.deinit();
                    
                    const range_obj = json.ObjectMap.init(self.allocator);
                    defer range_obj.deinit();
                    
                    const start_obj = json.ObjectMap.init(self.allocator);
                    defer start_obj.deinit();
                    try start_obj.put("line", json.Value{ .Integer = @intCast(current_line) });
                    try start_obj.put("character", json.Value{ .Integer = @intCast(current_char) });
                    
                    const end_obj = json.ObjectMap.init(self.allocator);
                    defer end_obj.deinit();
                    try end_obj.put("line", json.Value{ .Integer = @intCast(current_line) });
                    try end_obj.put("character", json.Value{ .Integer = @intCast(current_char + old_name.len) });
                    
                    try range_obj.put("start", json.Value{ .Object = start_obj });
                    try range_obj.put("end", json.Value{ .Object = end_obj });
                    try edit_obj.put("range", json.Value{ .Object = range_obj });
                    try edit_obj.put("newText", json.Value{ .String = new_name });
                    
                    try edits.append(json.Value{ .Object = edit_obj });
                    
                    i += old_name.len;
                    current_char += @intCast(old_name.len);
                } else {
                    if (doc.content[i] == '\n') {
                        current_line += 1;
                        current_char = 0;
                    } else {
                        current_char += 1;
                    }
                    i += 1;
                }
            }
            
            const changes_obj = json.ObjectMap.init(self.allocator);
            defer changes_obj.deinit();
            try changes_obj.put(uri, json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, try edits.toOwnedSlice()) });
            
            const result_obj = json.ObjectMap.init(self.allocator);
            defer result_obj.deinit();
            try result_obj.put("changes", json.Value{ .Object = changes_obj });
            
            const response = LSPMessage{
                .id = id,
                .result = json.Value{ .Object = result_obj },
            };
            
            return try json.stringify(response, .{}, self.allocator);
        }
        
        return try self.createErrorResponse(id, -32603, "Document not found");
    }

    fn handleDidOpen(self: *LSPHandler, params: ?json.Value) !void {
        if (params == null) return;
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        
        const uri = text_document.get("uri").?.String;
        const version = @as(i32, @intCast(text_document.get("version").?.Integer));
        const text = text_document.get("text").?.String;
        
        var doc = try DocumentInfo.init(self.allocator, uri, version, text);
        try doc.parse();
        
        try self.documents.put(try self.allocator.dupe(u8, uri), doc);
        
        // Send diagnostics
        try self.publishDiagnostics(uri, doc.diagnostics.items);
        
        std.log.info("Opened document: {s}", .{uri});
    }

    fn handleDidChange(self: *LSPHandler, params: ?json.Value) !void {
        if (params == null) return;
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const content_changes = params_obj.get("contentChanges").?.Array;
        
        const uri = text_document.get("uri").?.String;
        const version = @as(i32, @intCast(text_document.get("version").?.Integer));
        
        if (self.documents.getPtr(uri)) |doc| {
            if (content_changes.items.len > 0) {
                const change = content_changes.items[0].Object;
                const new_text = change.get("text").?.String;
                
                try doc.updateContent(new_text, version);
                try doc.parse();
                
                // Send updated diagnostics
                try self.publishDiagnostics(uri, doc.diagnostics.items);
            }
        }
        
        std.log.info("Changed document: {s} (version {})", .{ uri, version });
    }

    fn handleDidSave(self: *LSPHandler, params: ?json.Value) !void {
        _ = self;
        if (params == null) return;
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const uri = text_document.get("uri").?.String;
        
        std.log.info("Saved document: {s}", .{uri});
    }

    fn handleDidClose(self: *LSPHandler, params: ?json.Value) !void {
        if (params == null) return;
        
        const params_obj = params.?.Object;
        const text_document = params_obj.get("textDocument").?.Object;
        const uri = text_document.get("uri").?.String;
        
        if (self.documents.fetchRemove(uri)) |entry| {
            entry.value.deinit();
            self.allocator.free(entry.key);
        }
        
        std.log.info("Closed document: {s}", .{uri});
    }

    fn publishDiagnostics(self: *LSPHandler, uri: []const u8, diagnostics: []const Diagnostic) !void {
        const params_obj = json.ObjectMap.init(self.allocator);
        defer params_obj.deinit();
        try params_obj.put("uri", json.Value{ .String = uri });
        
        const diagnostics_json = try self.diagnosticsToJson(diagnostics);
        defer self.allocator.free(diagnostics_json);
        try params_obj.put("diagnostics", diagnostics_json);
        
        const notification = LSPMessage{
            .method = "textDocument/publishDiagnostics",
            .params = json.Value{ .Object = params_obj },
        };
        
        const message = try json.stringify(notification, .{}, self.allocator);
        defer self.allocator.free(message);
        
        // Send notification (this would go to stdout in real implementation)
        std.log.info("Publishing diagnostics for {s}: {} issues", .{ uri, diagnostics.len });
    }

    // Helper functions
    fn getWordAtPosition(self: *LSPHandler, content: []const u8, line: u32, character: u32) ![]u8 {
        var current_line: u32 = 0;
        var current_char: u32 = 0;
        var line_start: usize = 0;
        
        for (content, 0..) |c, i| {
            if (current_line == line) {
                if (current_char == character) {
                    // Found position, extract word
                    var start = i;
                    var end = i;
                    
                    // Find word boundaries
                    while (start > line_start and (std.ascii.isAlphaNumeric(content[start - 1]) or content[start - 1] == '_')) {
                        start -= 1;
                    }
                    while (end < content.len and (std.ascii.isAlphaNumeric(content[end]) or content[end] == '_')) {
                        end += 1;
                    }
                    
                    if (end > start) {
                        return try self.allocator.dupe(u8, content[start..end]);
                    }
                    break;
                }
                current_char += 1;
            }
            
            if (c == '\n') {
                current_line += 1;
                current_char = 0;
                line_start = i + 1;
            }
        }
        
        return try self.allocator.dupe(u8, "");
    }

    fn formatDocument(self: *LSPHandler, content: []const u8) ![]u8 {
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        var indent_level: u32 = 0;
        var at_line_start = true;
        
        for (content) |c| {
            switch (c) {
                '{' => {
                    try result.append(c);
                    try result.append('\n');
                    indent_level += 1;
                    at_line_start = true;
                },
                '}' => {
                    if (!at_line_start) {
                        try result.append('\n');
                    }
                    if (indent_level > 0) indent_level -= 1;
                    for (0..indent_level * 4) |_| {
                        try result.append(' ');
                    }
                    try result.append(c);
                    try result.append('\n');
                    at_line_start = true;
                },
                '\n' => {
                    try result.append(c);
                    at_line_start = true;
                },
                ' ', '\t' => {
                    if (!at_line_start) {
                        try result.append(c);
                    }
                },
                else => {
                    if (at_line_start) {
                        for (0..indent_level * 4) |_| {
                            try result.append(' ');
                        }
                        at_line_start = false;
                    }
                    try result.append(c);
                },
            }
        }
        
        return try result.toOwnedSlice();
    }

    fn completionItemsToJson(self: *LSPHandler, items: []const CompletionItem) !json.Value {
        var json_items = ArrayList(json.Value).init(self.allocator);
        defer json_items.deinit();
        
        for (items) |item| {
            var item_obj = std.StringHashMap(json.Value).init(self.allocator);
            defer item_obj.deinit();
            
            try item_obj.put("label", json.Value{ .string = item.label });
            if (item.kind) |kind| {
                try item_obj.put("kind", json.Value{ .integer = @intFromEnum(kind) });
            }
            if (item.detail) |detail| {
                try item_obj.put("detail", json.Value{ .string = detail });
            }
            if (item.documentation) |docs| {
                try item_obj.put("documentation", json.Value{ .string = docs });
            }
            if (item.insertText) |insert| {
                try item_obj.put("insertText", json.Value{ .string = insert });
            }
            
            try json_items.append(json.Value{ .object = item_obj });
        }
        
        return json.Value{ .array = json_items };
    }

    fn symbolsToJson(self: *LSPHandler, symbols: []const DocumentSymbol) !json.Value {
        var json_symbols = ArrayList(json.Value).init(self.allocator);
        defer json_symbols.deinit();
        
        for (symbols) |symbol| {
            const symbol_obj = json.ObjectMap.init(self.allocator);
            defer symbol_obj.deinit();
            
            try symbol_obj.put("name", json.Value{ .String = symbol.name });
            try symbol_obj.put("kind", json.Value{ .Integer = @intFromEnum(symbol.kind) });
            
            // Add range
            const range_obj = json.ObjectMap.init(self.allocator);
            defer range_obj.deinit();
            
            const start_obj = json.ObjectMap.init(self.allocator);
            defer start_obj.deinit();
            try start_obj.put("line", json.Value{ .Integer = @intCast(symbol.range.start.line) });
            try start_obj.put("character", json.Value{ .Integer = @intCast(symbol.range.start.character) });
            
            const end_obj = json.ObjectMap.init(self.allocator);
            defer end_obj.deinit();
            try end_obj.put("line", json.Value{ .Integer = @intCast(symbol.range.end.line) });
            try end_obj.put("character", json.Value{ .Integer = @intCast(symbol.range.end.character) });
            
            try range_obj.put("start", json.Value{ .Object = start_obj });
            try range_obj.put("end", json.Value{ .Object = end_obj });
            try symbol_obj.put("range", json.Value{ .Object = range_obj });
            
            // Add selection range
            const sel_range_obj = json.ObjectMap.init(self.allocator);
            defer sel_range_obj.deinit();
            
            const sel_start_obj = json.ObjectMap.init(self.allocator);
            defer sel_start_obj.deinit();
            try sel_start_obj.put("line", json.Value{ .Integer = @intCast(symbol.selectionRange.start.line) });
            try sel_start_obj.put("character", json.Value{ .Integer = @intCast(symbol.selectionRange.start.character) });
            
            const sel_end_obj = json.ObjectMap.init(self.allocator);
            defer sel_end_obj.deinit();
            try sel_end_obj.put("line", json.Value{ .Integer = @intCast(symbol.selectionRange.end.line) });
            try sel_end_obj.put("character", json.Value{ .Integer = @intCast(symbol.selectionRange.end.character) });
            
            try sel_range_obj.put("start", json.Value{ .Object = sel_start_obj });
            try sel_range_obj.put("end", json.Value{ .Object = sel_end_obj });
            try symbol_obj.put("selectionRange", json.Value{ .Object = sel_range_obj });
            
            try json_symbols.append(json.Value{ .Object = symbol_obj });
        }
        
        return json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, try json_symbols.toOwnedSlice()) };
    }

    fn diagnosticsToJson(self: *LSPHandler, diagnostics: []const Diagnostic) !json.Value {
        var json_diagnostics = ArrayList(json.Value).init(self.allocator);
        defer json_diagnostics.deinit();
        
        for (diagnostics) |diagnostic| {
            const diag_obj = json.ObjectMap.init(self.allocator);
            defer diag_obj.deinit();
            
            // Range
            const range_obj = json.ObjectMap.init(self.allocator);
            defer range_obj.deinit();
            
            const start_obj = json.ObjectMap.init(self.allocator);
            defer start_obj.deinit();
            try start_obj.put("line", json.Value{ .Integer = @intCast(diagnostic.range.start.line) });
            try start_obj.put("character", json.Value{ .Integer = @intCast(diagnostic.range.start.character) });
            
            const end_obj = json.ObjectMap.init(self.allocator);
            defer end_obj.deinit();
            try end_obj.put("line", json.Value{ .Integer = @intCast(diagnostic.range.end.line) });
            try end_obj.put("character", json.Value{ .Integer = @intCast(diagnostic.range.end.character) });
            
            try range_obj.put("start", json.Value{ .Object = start_obj });
            try range_obj.put("end", json.Value{ .Object = end_obj });
            try diag_obj.put("range", json.Value{ .Object = range_obj });
            
            if (diagnostic.severity) |severity| {
                try diag_obj.put("severity", json.Value{ .Integer = @intFromEnum(severity) });
            }
            if (diagnostic.source) |source| {
                try diag_obj.put("source", json.Value{ .String = source });
            }
            try diag_obj.put("message", json.Value{ .String = diagnostic.message });
            
            try json_diagnostics.append(json.Value{ .Object = diag_obj });
        }
        
        return json.Value{ .Array = json.Array.fromOwnedSlice(self.allocator, try json_diagnostics.toOwnedSlice()) };
    }

    fn createErrorResponse(self: *LSPHandler, id: json.Value, code: i32, message: []const u8) ![]u8 {
        var response_obj = std.StringHashMap(json.Value).init(self.allocator);
        defer response_obj.deinit();
        
        try response_obj.put("jsonrpc", json.Value{ .string = "2.0" });
        try response_obj.put("id", id);
        
        var error_obj = std.StringHashMap(json.Value).init(self.allocator);
        defer error_obj.deinit();
        try error_obj.put("code", json.Value{ .integer = code });
        try error_obj.put("message", json.Value{ .string = message });
        
        try response_obj.put("error", json.Value{ .object = error_obj });
        
        const response = json.Value{ .object = response_obj };
        return try json.stringifyAlloc(self.allocator, response, .{});
    }
};

// Main LSP Server Entry Point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var handler = LSPHandler.init(allocator);
    defer handler.deinit();

    std.log.info("CURSED Language Server starting...", .{});

    var stdin_buffer: [4096]u8 = undefined;
    const stdin = std.fs.File.stdin().reader(stdin_buffer[0..]);
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);

    var buffer = ArrayList(u8).init(allocator);
    defer buffer.deinit();

    while (!handler.shutdown_requested) {
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
                if (try handler.handleMessage(buffer.items)) |response| {
                    defer allocator.free(response);
                    
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

// Test function for development
pub fn testLSP() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var handler = LSPHandler.init(allocator);
    defer handler.deinit();

    // Test initialize
    const init_message = 
        \\{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}
    ;
    
    if (try handler.handleMessage(init_message)) |response| {
        defer allocator.free(response);
        std.log.info("Initialize response: {s}", .{response});
    }

    // Test document open
    const open_message = 
        \\{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"file:///test.csd","version":1,"languageId":"cursed","text":"slay test_function() {\n    vibez.spill(\"Hello CURSED!\")\n}"}}}
    ;
    
    _ = try handler.handleMessage(open_message);

    // Test completion
    const completion_message = 
        \\{"jsonrpc":"2.0","id":2,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///test.csd"},"position":{"line":1,"character":4}}}
    ;
    
    if (try handler.handleMessage(completion_message)) |response| {
        defer allocator.free(response);
        std.log.info("Completion response: {s}", .{response});
    }

    std.log.info("LSP test completed successfully!", .{});
}
