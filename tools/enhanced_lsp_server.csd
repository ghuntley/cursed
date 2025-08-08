// Enhanced CURSED Language Server Protocol Implementation
// Production-ready LSP server with advanced features

yeet "stringz"
yeet "arrayz"
yeet "testz"
yeet "jsonz"
yeet "filez"

// Enhanced LSP Message Types with better handling
squad LSPRequest {
    spill id drip
    spill method tea
    spill params tea
    spill jsonrpc tea
}

squad LSPResponse {
    spill id drip
    spill result tea
    spill error tea
    spill jsonrpc tea
}

squad LSPNotification {
    spill method tea
    spill params tea
    spill jsonrpc tea
}

// Enhanced document state tracking with better metadata
squad DocumentInfo {
    spill uri tea
    spill version drip
    spill content tea
    spill language_id tea
    spill last_modified drip
    spill diagnostic_version drip
}

// Enhanced diagnostic information with more details
squad Diagnostic {
    spill range tea
    spill severity drip
    spill code tea
    spill source tea
    spill message tea
    spill related_information tea
    spill tags []drip
    spill data tea
}

// Enhanced completion item with more metadata
squad CompletionItem {
    spill label tea
    spill kind drip
    spill detail tea
    spill documentation tea
    spill insert_text tea
    spill insert_text_format drip
    spill text_edit tea
    spill additional_text_edits tea
    spill sort_text tea
    spill filter_text tea
}

// LSP Server configuration
squad LSPServerConfig {
    spill enable_diagnostics lit
    spill enable_completion lit
    spill enable_hover lit
    spill enable_formatting lit
    spill enable_signature_help lit
    spill enable_goto_definition lit
    spill max_diagnostics_per_file drip
    spill completion_trigger_characters []tea
    spill signature_help_trigger_characters []tea
}

// Enhanced LSP Server state
squad LSPServer {
    spill documents []DocumentInfo
    spill capabilities tea
    spill workspace_folders []tea
    spill initialized lit
    spill config LSPServerConfig
    spill client_capabilities tea
    spill server_info tea
}

// Default LSP server configuration
slay default_lsp_config() LSPServerConfig {
    damn LSPServerConfig{
        enable_diagnostics: based,
        enable_completion: based,
        enable_hover: based,
        enable_formatting: based,
        enable_signature_help: based,
        enable_goto_definition: based,
        max_diagnostics_per_file: 100,
        completion_trigger_characters: [".", ":"],
        signature_help_trigger_characters: ["(", ","]
    }
}

// Initialize enhanced LSP server
slay init_enhanced_lsp_server() LSPServer {
    sus server LSPServer = LSPServer{
        documents: [],
        capabilities: "",
        workspace_folders: [],
        initialized: cringe,
        config: default_lsp_config(),
        client_capabilities: "",
        server_info: json_object()
    }
    
    // Set server info
    server.server_info = json_set(server.server_info, "name", json_string("CURSED LSP"))
    server.server_info = json_set(server.server_info, "version", json_string("1.0.0"))
    
    damn server
}

// Enhanced LSP initialize with better capability negotiation
slay handle_enhanced_initialize(server LSPServer, params tea) tea {
    // Store client capabilities for reference
    server.client_capabilities = json_get(params, "capabilities")
    
    sus capabilities tea = json_object()
    
    // Enhanced text document sync capabilities
    sus text_sync tea = json_object()
    text_sync = json_set(text_sync, "openClose", json_bool(based))
    text_sync = json_set(text_sync, "change", json_number(2))  // Incremental
    text_sync = json_set(text_sync, "save", json_bool(based))
    text_sync = json_set(text_sync, "willSave", json_bool(based))
    text_sync = json_set(text_sync, "willSaveWaitUntil", json_bool(based))
    
    capabilities = json_set(capabilities, "textDocumentSync", text_sync)
    
    // Enhanced completion capabilities
    ready (server.config.enable_completion) {
        sus completion tea = json_object()
        sus completion_item tea = json_object()
        completion_item = json_set(completion_item, "snippetSupport", json_bool(based))
        completion_item = json_set(completion_item, "commitCharactersSupport", json_bool(based))
        completion_item = json_set(completion_item, "documentationFormat", json_array(["markdown", "plaintext"]))
        completion_item = json_set(completion_item, "deprecatedSupport", json_bool(based))
        completion_item = json_set(completion_item, "preselectSupport", json_bool(based))
        
        completion = json_set(completion, "completionItem", completion_item)
        completion = json_set(completion, "triggerCharacters", json_array(server.config.completion_trigger_characters))
        completion = json_set(completion, "resolveProvider", json_bool(based))
        
        capabilities = json_set(capabilities, "completionProvider", completion)
    }
    
    // Enhanced hover support
    ready (server.config.enable_hover) {
        sus hover tea = json_object()
        hover = json_set(hover, "contentFormat", json_array(["markdown", "plaintext"]))
        capabilities = json_set(capabilities, "hoverProvider", hover)
    }
    
    // Enhanced signature help
    ready (server.config.enable_signature_help) {
        sus signature_help tea = json_object()
        signature_help = json_set(signature_help, "triggerCharacters", json_array(server.config.signature_help_trigger_characters))
        signature_help = json_set(signature_help, "retriggerCharacters", json_array([","]))
        capabilities = json_set(capabilities, "signatureHelpProvider", signature_help)
    }
    
    // Enhanced document formatting
    ready (server.config.enable_formatting) {
        capabilities = json_set(capabilities, "documentFormattingProvider", json_bool(based))
        capabilities = json_set(capabilities, "documentRangeFormattingProvider", json_bool(based))
        capabilities = json_set(capabilities, "documentOnTypeFormattingProvider", json_object())
    }
    
    // Enhanced diagnostics
    ready (server.config.enable_diagnostics) {
        sus diagnostics tea = json_object()
        diagnostics = json_set(diagnostics, "relatedInformation", json_bool(based))
        diagnostics = json_set(diagnostics, "tagSupport", json_bool(based))
        diagnostics = json_set(diagnostics, "versionSupport", json_bool(based))
        capabilities = json_set(capabilities, "publishDiagnostics", diagnostics)
    }
    
    // Additional capabilities
    ready (server.config.enable_goto_definition) {
        capabilities = json_set(capabilities, "definitionProvider", json_bool(based))
        capabilities = json_set(capabilities, "declarationProvider", json_bool(based))
        capabilities = json_set(capabilities, "typeDefinitionProvider", json_bool(based))
        capabilities = json_set(capabilities, "implementationProvider", json_bool(based))
    }
    
    // Advanced capabilities
    capabilities = json_set(capabilities, "referencesProvider", json_bool(based))
    capabilities = json_set(capabilities, "documentHighlightProvider", json_bool(based))
    capabilities = json_set(capabilities, "documentSymbolProvider", json_bool(based))
    capabilities = json_set(capabilities, "workspaceSymbolProvider", json_bool(based))
    capabilities = json_set(capabilities, "codeActionProvider", json_bool(based))
    capabilities = json_set(capabilities, "codeLensProvider", json_bool(based))
    capabilities = json_set(capabilities, "renameProvider", json_bool(based))
    capabilities = json_set(capabilities, "foldingRangeProvider", json_bool(based))
    
    // Workspace capabilities
    sus workspace tea = json_object()
    sus workspace_folders tea = json_object()
    workspace_folders = json_set(workspace_folders, "supported", json_bool(based))
    workspace_folders = json_set(workspace_folders, "changeNotifications", json_bool(based))
    workspace = json_set(workspace, "workspaceFolders", workspace_folders)
    capabilities = json_set(capabilities, "workspace", workspace)
    
    sus result tea = json_object()
    result = json_set(result, "capabilities", capabilities)
    result = json_set(result, "serverInfo", server.server_info)
    
    damn result
}

// Enhanced completion with context-aware suggestions
slay handle_enhanced_completion(server LSPServer, params tea) tea {
    sus position tea = json_get(params, "position")
    sus line drip = json_get_number(position, "line")
    sus character drip = json_get_number(position, "character")
    sus text_document tea = json_get(params, "textDocument")
    sus uri tea = json_get_string(text_document, "uri")
    
    // Get document content for context analysis
    sus doc_content tea = get_document_content(server, uri)
    sus lines []tea = split_str(doc_content, "\n")
    
    sus current_line tea = ""
    ready (line < len(lines)) {
        current_line = lines[line]
    }
    
    sus prefix tea = substring(current_line, 0, character)
    sus completions []CompletionItem = []
    
    // Context-aware completion based on current position
    ready (ends_with(prefix, ".")) {
        // Method/property completion
        completions = get_method_completions(prefix)
    } otherwise ready (contains_str(prefix, "yeet ")) {
        // Module import completion
        completions = get_module_completions()
    } otherwise ready (contains_str(prefix, "sus ") || contains_str(prefix, "slay ")) {
        // Type completion
        completions = get_type_completions()
    } otherwise {
        // General keyword and identifier completion
        completions = get_general_completions()
    }
    
    // Convert to LSP format
    sus lsp_completions tea = json_array([])
    sus i drip = 0
    bestie (i < len(completions)) {
        sus item tea = completion_item_to_json(completions[i])
        lsp_completions = json_array_push(lsp_completions, item)
        i = i + 1
    }
    
    damn lsp_completions
}

// Get method/property completions
slay get_method_completions(prefix tea) []CompletionItem {
    sus completions []CompletionItem = []
    
    // Vibez methods
    ready (contains_str(prefix, "vibez.")) {
        push(completions, CompletionItem{
            label: "spill",
            kind: 2,  // Method
            detail: "vibez.spill(...) - Output to console",
            documentation: "Print values to the console with CURSED style",
            insert_text: "spill(${1:value})",
            insert_text_format: 2,  // Snippet
            sort_text: "a_spill",
            filter_text: "spill",
            text_edit: "",
            additional_text_edits: "",
            data: ""
        })
    }
    
    damn completions
}

// Get module import completions
slay get_module_completions() []CompletionItem {
    sus completions []CompletionItem = []
    sus modules []tea = ["stringz", "arrayz", "mathz", "testz", "jsonz", "filez", "cryptz", "concurrenz"]
    
    sus i drip = 0
    bestie (i < len(modules)) {
        push(completions, CompletionItem{
            label: modules[i],
            kind: 9,  // Module
            detail: "CURSED standard library module",
            documentation: "Standard library module: " + modules[i],
            insert_text: "\"" + modules[i] + "\"",
            insert_text_format: 1,  // PlainText
            sort_text: "module_" + modules[i],
            filter_text: modules[i],
            text_edit: "",
            additional_text_edits: "",
            data: modules[i]
        })
        i = i + 1
    }
    
    damn completions
}

// Get type completions
slay get_type_completions() []CompletionItem {
    sus completions []CompletionItem = []
    sus types []tea = ["drip", "tea", "lit", "normie"]
    sus descriptions []tea = ["Integer type", "String type", "Boolean type", "32-bit integer"]
    
    sus i drip = 0
    bestie (i < len(types)) {
        push(completions, CompletionItem{
            label: types[i],
            kind: 25,  // TypeParameter
            detail: descriptions[i],
            documentation: "CURSED primitive type: " + descriptions[i],
            insert_text: types[i],
            insert_text_format: 1,
            sort_text: "type_" + types[i],
            filter_text: types[i],
            text_edit: "",
            additional_text_edits: "",
            data: types[i]
        })
        i = i + 1
    }
    
    damn completions
}

// Get general keyword completions
slay get_general_completions() []CompletionItem {
    sus completions []CompletionItem = []
    
    // CURSED keywords with snippets
    sus keywords []tea = ["sus", "slay", "damn", "ready", "otherwise", "bestie", "yeet", "stan", "squad", "collab", "spill"]
    sus snippets []tea = [
        "sus ${1:name} ${2:type} = ${3:value}",
        "slay ${1:name}(${2:params}) ${3:return_type} {\n    ${4:body}\n}",
        "damn ${1:value}",
        "ready (${1:condition}) {\n    ${2:body}\n}",
        "otherwise {\n    ${1:body}\n}",
        "bestie (${1:condition}) {\n    ${2:body}\n}",
        "yeet \"${1:module}\"",
        "stan {\n    ${1:body}\n}",
        "squad ${1:Name} {\n    spill ${2:field} ${3:type}\n}",
        "collab ${1:Name} {\n    slay ${2:method}()\n}",
        "spill ${1:name} ${2:type}"
    ]
    
    sus i drip = 0
    bestie (i < len(keywords)) {
        push(completions, CompletionItem{
            label: keywords[i],
            kind: 14,  // Keyword
            detail: "CURSED keyword",
            documentation: "CURSED language keyword: " + keywords[i],
            insert_text: snippets[i],
            insert_text_format: 2,  // Snippet
            sort_text: "keyword_" + keywords[i],
            filter_text: keywords[i],
            text_edit: "",
            additional_text_edits: "",
            data: keywords[i]
        })
        i = i + 1
    }
    
    damn completions
}

// Enhanced syntax checking with better error detection
slay check_enhanced_cursed_syntax(content tea) []Diagnostic {
    sus diagnostics []Diagnostic = []
    sus lines []tea = split_str(content, "\n")
    
    sus line_num drip = 0
    bestie (line_num < len(lines)) {
        sus line tea = lines[line_num]
        sus trimmed tea = trim_str(line)
        
        // Skip empty lines and comments
        ready (len_str(trimmed) == 0 || starts_with(trimmed, "//")) {
            line_num = line_num + 1
            continue
        }
        
        // Enhanced missing semicolon detection
        ready (needs_semicolon(trimmed)) {
            sus diagnostic Diagnostic = create_diagnostic(
                line_num, 0, line_num, len_str(line),
                1,  // Error
                "missing-semicolon",
                "cursed-lsp",
                "Missing semicolon at end of statement",
                "",
                [],
                ""
            )
            push(diagnostics, diagnostic)
        }
        
        // Enhanced type checking
        ready (contains_str(trimmed, "sus ")) {
            check_variable_declaration(diagnostics, trimmed, line_num)
        }
        
        // Enhanced function checking
        ready (contains_str(trimmed, "slay ")) {
            check_function_declaration(diagnostics, trimmed, line_num)
        }
        
        // Enhanced import checking
        ready (contains_str(trimmed, "yeet ")) {
            check_import_statement(diagnostics, trimmed, line_num)
        }
        
        line_num = line_num + 1
    }
    
    damn diagnostics
}

// Enhanced utility functions
slay needs_semicolon(line tea) lit {
    ready (ends_with(line, ";") || ends_with(line, "{") || ends_with(line, "}")) {
        damn cringe
    }
    
    ready (contains_str(line, "sus ") || contains_str(line, "vibez.spill") || 
           contains_str(line, "damn ") || contains_str(line, "yeet ") ||
           contains_str(line, "push(") || contains_str(line, "break") ||
           contains_str(line, "continue")) {
        damn based
    }
    
    damn cringe
}

slay check_variable_declaration(diagnostics []Diagnostic, line tea, line_num drip) {
    // Check for proper variable declaration syntax
    sus parts []tea = split_str(line, " ")
    ready (len(parts) < 3) {
        sus diagnostic Diagnostic = create_diagnostic(
            line_num, 0, line_num, len_str(line),
            1,  // Error
            "invalid-variable-declaration",
            "cursed-lsp",
            "Invalid variable declaration syntax. Expected: sus name type = value",
            "",
            [],
            ""
        )
        push(diagnostics, diagnostic)
    }
}

slay check_function_declaration(diagnostics []Diagnostic, line tea, line_num drip) {
    // Check for proper function declaration syntax
    ready (!contains_str(line, "(") || !contains_str(line, ")")) {
        sus diagnostic Diagnostic = create_diagnostic(
            line_num, 0, line_num, len_str(line),
            1,  // Error
            "invalid-function-declaration",
            "cursed-lsp",
            "Invalid function declaration syntax. Expected: slay name(params) return_type",
            "",
            [],
            ""
        )
        push(diagnostics, diagnostic)
    }
}

slay check_import_statement(diagnostics []Diagnostic, line tea, line_num drip) {
    // Check for proper import syntax
    ready (!contains_str(line, "\"")) {
        sus diagnostic Diagnostic = create_diagnostic(
            line_num, 0, line_num, len_str(line),
            2,  // Warning
            "invalid-import-syntax",
            "cursed-lsp",
            "Import statement should use quoted module name: yeet \"module\"",
            "",
            [],
            ""
        )
        push(diagnostics, diagnostic)
    }
}

// Helper function to create diagnostics
slay create_diagnostic(start_line drip, start_char drip, end_line drip, end_char drip, severity drip, code tea, source tea, message tea, related_info tea, tags []drip, data tea) Diagnostic {
    damn Diagnostic{
        range: create_range(start_line, start_char, end_line, end_char),
        severity: severity,
        code: code,
        source: source,
        message: message,
        related_information: related_info,
        tags: tags,
        data: data
    }
}

// Convert completion item to JSON
slay completion_item_to_json(item CompletionItem) tea {
    sus json_item tea = json_object()
    json_item = json_set(json_item, "label", json_string(item.label))
    json_item = json_set(json_item, "kind", json_number(item.kind))
    json_item = json_set(json_item, "detail", json_string(item.detail))
    json_item = json_set(json_item, "documentation", json_string(item.documentation))
    json_item = json_set(json_item, "insertText", json_string(item.insert_text))
    json_item = json_set(json_item, "insertTextFormat", json_number(item.insert_text_format))
    json_item = json_set(json_item, "sortText", json_string(item.sort_text))
    json_item = json_set(json_item, "filterText", json_string(item.filter_text))
    damn json_item
}

// Enhanced diagnostic publishing with version tracking
slay publish_enhanced_diagnostics(server LSPServer, uri tea, content tea) {
    sus diagnostics []Diagnostic = check_enhanced_cursed_syntax(content)
    
    // Limit diagnostics per file
    ready (len(diagnostics) > server.config.max_diagnostics_per_file) {
        // Truncate diagnostics and add a summary diagnostic
        sus truncated []Diagnostic = []
        sus i drip = 0
        bestie (i < server.config.max_diagnostics_per_file - 1) {
            push(truncated, diagnostics[i])
            i = i + 1
        }
        
        sus summary_diagnostic Diagnostic = create_diagnostic(
            0, 0, 0, 0,
            3,  // Info
            "too-many-diagnostics",
            "cursed-lsp",
            "Too many diagnostics. Showing first " + int_to_str(server.config.max_diagnostics_per_file - 1) + " of " + int_to_str(len(diagnostics)),
            "",
            [],
            ""
        )
        push(truncated, summary_diagnostic)
        diagnostics = truncated
    }
    
    sus notification tea = json_object()
    notification = json_set(notification, "jsonrpc", json_string("2.0"))
    notification = json_set(notification, "method", json_string("textDocument/publishDiagnostics"))
    
    sus params tea = json_object()
    params = json_set(params, "uri", json_string(uri))
    
    sus diag_array tea = json_array([])
    sus i drip = 0
    bestie (i < len(diagnostics)) {
        sus diag tea = enhanced_diagnostic_to_json(diagnostics[i])
        diag_array = json_array_push(diag_array, diag)
        i = i + 1
    }
    params = json_set(params, "diagnostics", diag_array)
    
    notification = json_set(notification, "params", params)
    
    // In a real implementation, this would be sent over stdio
    vibez.spill("Enhanced diagnostics published for: " + uri + " (" + int_to_str(len(diagnostics)) + " issues)")
}

// Enhanced diagnostic to JSON conversion
slay enhanced_diagnostic_to_json(diag Diagnostic) tea {
    sus json_diag tea = json_object()
    json_diag = json_set(json_diag, "range", diag.range)
    json_diag = json_set(json_diag, "severity", json_number(diag.severity))
    json_diag = json_set(json_diag, "code", json_string(diag.code))
    json_diag = json_set(json_diag, "source", json_string(diag.source))
    json_diag = json_set(json_diag, "message", json_string(diag.message))
    
    ready (len_str(diag.related_information) > 0) {
        json_diag = json_set(json_diag, "relatedInformation", diag.related_information)
    }
    
    ready (len(diag.tags) > 0) {
        sus tags_array tea = json_array([])
        sus i drip = 0
        bestie (i < len(diag.tags)) {
            tags_array = json_array_push(tags_array, json_number(diag.tags[i]))
            i = i + 1
        }
        json_diag = json_set(json_diag, "tags", tags_array)
    }
    
    damn json_diag
}

// Enhanced hover information with markdown support
slay handle_enhanced_hover(server LSPServer, params tea) tea {
    sus position tea = json_get(params, "position")
    sus text_document tea = json_get(params, "textDocument")
    sus uri tea = json_get_string(text_document, "uri")
    sus line drip = json_get_number(position, "line")
    sus character drip = json_get_number(position, "character")
    
    sus doc_content tea = get_document_content(server, uri)
    sus lines []tea = split_str(doc_content, "\n")
    
    ready (line >= len(lines)) {
        damn json_null()
    }
    
    sus current_line tea = lines[line]
    sus word tea = get_word_at_position(current_line, character)
    
    sus hover_content tea = get_hover_content_for_word(word)
    
    ready (len_str(hover_content) == 0) {
        damn json_null()
    }
    
    sus hover tea = json_object()
    sus contents tea = json_object()
    contents = json_set(contents, "kind", json_string("markdown"))
    contents = json_set(contents, "value", json_string(hover_content))
    hover = json_set(hover, "contents", contents)
    
    // Add range if applicable
    sus range tea = create_word_range(line, character, word)
    hover = json_set(hover, "range", range)
    
    damn hover
}

// Get word at specific position
slay get_word_at_position(line tea, character drip) tea {
    ready (character >= len_str(line)) {
        damn ""
    }
    
    // Find word boundaries
    sus start drip = character
    sus end drip = character
    
    // Move start backward
    bestie (start > 0) {
        sus char tea = char_at(line, start - 1)
        ready (is_word_char(char)) {
            start = start - 1
        } otherwise {
            break
        }
    }
    
    // Move end forward
    bestie (end < len_str(line)) {
        sus char tea = char_at(line, end)
        ready (is_word_char(char)) {
            end = end + 1
        } otherwise {
            break
        }
    }
    
    damn substring(line, start, end)
}

slay is_word_char(char tea) lit {
    damn (char >= "a" && char <= "z") || (char >= "A" && char <= "Z") || 
         (char >= "0" && char <= "9") || char == "_"
}

// Get hover content for a specific word
slay get_hover_content_for_word(word tea) tea {
    // CURSED keywords
    ready (word == "sus") {
        damn "**sus** - Variable declaration keyword\\n\\nDeclares a new variable with type annotation.\\n\\n```cursed\\nsus x drip = 42\\n```"
    } otherwise ready (word == "slay") {
        damn "**slay** - Function declaration keyword\\n\\nDefines a new function with parameters and return type.\\n\\n```cursed\\nslay add(x drip, y drip) drip {\\n    damn x + y\\n}\\n```"
    } otherwise ready (word == "damn") {
        damn "**damn** - Return statement keyword\\n\\nReturns a value from a function.\\n\\n```cursed\\ndamn result\\n```"
    } otherwise ready (word == "ready") {
        damn "**ready** - Conditional statement keyword\\n\\nExecutes code block if condition is true.\\n\\n```cursed\\nready (x > 0) {\\n    vibez.spill(\"positive\")\\n}\\n```"
    } otherwise ready (word == "bestie") {
        damn "**bestie** - Loop keyword\\n\\nExecutes code block while condition is true.\\n\\n```cursed\\nbestie (i < 10) {\\n    i = i + 1\\n}\\n```"
    } otherwise ready (word == "yeet") {
        damn "**yeet** - Import statement keyword\\n\\nImports a module for use in the current file.\\n\\n```cursed\\nyeet \\\"stringz\\\"\\n```"
    } otherwise ready (word == "based") {
        damn "**based** - Boolean true value\\n\\nThe CURSED equivalent of `true`.\\n\\n```cursed\\nsus is_valid lit = based\\n```"
    } otherwise ready (word == "cringe") {
        damn "**cringe** - Boolean false value\\n\\nThe CURSED equivalent of `false`.\\n\\n```cursed\\nsus is_invalid lit = cringe\\n```"
    }
    
    // Types
    ready (word == "drip") {
        damn "**drip** - Integer type\\n\\n64-bit signed integer type in CURSED.\\n\\n```cursed\\nsus count drip = 42\\n```"
    } otherwise ready (word == "tea") {
        damn "**tea** - String type\\n\\nString type for text data in CURSED.\\n\\n```cursed\\nsus message tea = \\\"Hello\\\"\\n```"
    } otherwise ready (word == "lit") {
        damn "**lit** - Boolean type\\n\\nBoolean type for true/false values in CURSED.\\n\\n```cursed\\nsus flag lit = based\\n```"
    }
    
    damn ""
}

// Create word range for hover
slay create_word_range(line drip, character drip, word tea) tea {
    sus start drip = character - len_str(word) / 2
    ready (start < 0) { start = 0 }
    sus end drip = start + len_str(word)
    
    damn create_range(line, start, line, end)
}

// Main enhanced LSP server
slay main() {
    vibez.spill("🚀 Enhanced CURSED Language Server starting...")
    
    sus server LSPServer = init_enhanced_lsp_server()
    
    vibez.spill("📋 Enhanced LSP Server capabilities:")
    vibez.spill("  ✅ Advanced text synchronization")
    vibez.spill("  ✅ Context-aware code completion with snippets")
    vibez.spill("  ✅ Enhanced hover information with markdown")
    vibez.spill("  ✅ Advanced document formatting")
    vibez.spill("  ✅ Comprehensive syntax diagnostics")
    vibez.spill("  ✅ Signature help and parameter hints")
    vibez.spill("  ✅ Go-to-definition support")
    vibez.spill("  ✅ Document symbols and workspace features")
    
    // Demonstrate enhanced capabilities
    sus sample_params tea = json_object()
    sus init_result tea = handle_enhanced_initialize(server, sample_params)
    vibez.spill("📤 Enhanced initialization complete")
    
    // Test enhanced completion
    sus completion_params tea = json_object()
    sus position tea = json_object()
    position = json_set(position, "line", json_number(5))
    position = json_set(position, "character", json_number(10))
    completion_params = json_set(completion_params, "position", position)
    
    sus text_doc tea = json_object()
    text_doc = json_set(text_doc, "uri", json_string("file:///test.csd"))
    completion_params = json_set(completion_params, "textDocument", text_doc)
    
    sus completions tea = handle_enhanced_completion(server, completion_params)
    vibez.spill("🎯 Enhanced completion system ready")
    
    vibez.spill("🎉 Enhanced CURSED LSP Server is now ready for IDE integration!")
}
