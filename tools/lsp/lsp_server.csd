// CURSED Language Server Protocol (LSP) Implementation
// Self-hosting LSP server for IDE integration

yeet "stringz"
yeet "arrayz"
yeet "testz"
yeet "jsonz"  // For LSP message protocol

// LSP Message Types
squad LSPRequest {
    spill id drip
    spill method tea
    spill params tea
}

squad LSPResponse {
    spill id drip
    spill result tea
    spill error tea
}

squad LSPNotification {
    spill method tea
    spill params tea
}

// Document state tracking
squad DocumentInfo {
    spill uri tea
    spill version drip
    spill content tea
    spill language_id tea
}

// Diagnostic information
squad Diagnostic {
    spill range tea
    spill severity drip
    spill code tea
    spill source tea
    spill message tea
}

// Position in a document
squad Position {
    spill line drip
    spill character drip
}

// Range in a document
squad Range {
    spill start Position
    spill end Position
}

// LSP Server state
squad LSPServer {
    spill documents []DocumentInfo
    spill capabilities tea
    spill workspace_folders []tea
    spill initialized lit
}

// Initialize LSP server
slay init_lsp_server() LSPServer {
    sus server LSPServer = LSPServer{
        documents: [],
        capabilities: "",
        workspace_folders: [],
        initialized: cringe
    }
    damn server
}

// Handle LSP initialize request
slay handle_initialize(server LSPServer, params tea) tea {
    sus capabilities tea = json_object()
    
    // Text document sync capabilities
    sus text_sync tea = json_object()
    text_sync = json_set(text_sync, "openClose", json_bool(based))
    text_sync = json_set(text_sync, "change", json_number(2))  // Incremental
    text_sync = json_set(text_sync, "save", json_bool(based))
    
    capabilities = json_set(capabilities, "textDocumentSync", text_sync)
    
    // Completion capabilities
    sus completion tea = json_object()
    sus completion_item tea = json_object()
    completion_item = json_set(completion_item, "snippetSupport", json_bool(based))
    completion = json_set(completion, "completionItem", completion_item)
    capabilities = json_set(capabilities, "completionProvider", completion)
    
    // Hover support
    capabilities = json_set(capabilities, "hoverProvider", json_bool(based))
    
    // Signature help
    sus signature_help tea = json_object()
    signature_help = json_set(signature_help, "triggerCharacters", json_array(["("]))
    capabilities = json_set(capabilities, "signatureHelpProvider", signature_help)
    
    // Go to definition
    capabilities = json_set(capabilities, "definitionProvider", json_bool(based))
    
    // Document formatting
    capabilities = json_set(capabilities, "documentFormattingProvider", json_bool(based))
    
    // Diagnostics
    capabilities = json_set(capabilities, "publishDiagnostics", json_bool(based))
    
    sus result tea = json_object()
    result = json_set(result, "capabilities", capabilities)
    
    damn result
}

// Handle text document open
slay handle_text_document_open(server LSPServer, params tea) {
    sus text_document tea = json_get(params, "textDocument")
    sus uri tea = json_get_string(text_document, "uri")
    sus language_id tea = json_get_string(text_document, "languageId")
    sus version drip = json_get_number(text_document, "version")
    sus text tea = json_get_string(text_document, "text")
    
    sus doc DocumentInfo = DocumentInfo{
        uri: uri,
        version: version,
        content: text,
        language_id: language_id
    }
    
    push(server.documents, doc)
    
    // Run diagnostics on opened document
    publish_diagnostics(server, uri, text)
}

// Handle text document change
slay handle_text_document_change(server LSPServer, params tea) {
    sus text_document tea = json_get(params, "textDocument")
    sus uri tea = json_get_string(text_document, "uri")
    sus version drip = json_get_number(text_document, "version")
    sus changes tea = json_get(params, "contentChanges")
    
    // Find document and update content
    sus i drip = 0
    bestie (i < len(server.documents)) {
        ready (server.documents[i].uri == uri) {
            // For full document sync, replace entire content
            sus change tea = json_get_array_item(changes, 0)
            sus new_text tea = json_get_string(change, "text")
            server.documents[i].content = new_text
            server.documents[i].version = version
            
            // Run diagnostics on changed document
            publish_diagnostics(server, uri, new_text)
            break
        }
        i = i + 1
    }
}

// Provide completion suggestions
slay handle_completion(server LSPServer, params tea) tea {
    sus position tea = json_get(params, "position")
    sus line drip = json_get_number(position, "line")
    sus character drip = json_get_number(position, "character")
    
    sus completions tea = json_array([])
    
    // CURSED keyword completions
    sus keywords []tea = ["sus", "slay", "damn", "ready", "otherwise", "bestie", 
                         "yeet", "stan", "squad", "collab", "spill", "vibez"]
    
    sus i drip = 0
    bestie (i < len(keywords)) {
        sus completion tea = json_object()
        completion = json_set(completion, "label", json_string(keywords[i]))
        completion = json_set(completion, "kind", json_number(14))  // Keyword
        completion = json_set(completion, "detail", json_string("CURSED keyword"))
        
        completions = json_array_push(completions, completion)
        i = i + 1
    }
    
    // Type completions
    sus types []tea = ["drip", "tea", "lit"]
    i = 0
    bestie (i < len(types)) {
        sus completion tea = json_object()
        completion = json_set(completion, "label", json_string(types[i]))
        completion = json_set(completion, "kind", json_number(25))  // TypeParameter
        completion = json_set(completion, "detail", json_string("CURSED type"))
        
        completions = json_array_push(completions, completion)
        i = i + 1
    }
    
    // Standard library completions
    sus stdlib []tea = ["vibez.spill", "len", "concat_str", "substring"]
    i = 0
    bestie (i < len(stdlib)) {
        sus completion tea = json_object()
        completion = json_set(completion, "label", json_string(stdlib[i]))
        completion = json_set(completion, "kind", json_number(3))  // Function
        completion = json_set(completion, "detail", json_string("Standard library function"))
        
        completions = json_array_push(completions, completion)
        i = i + 1
    }
    
    damn completions
}

// Provide hover information
slay handle_hover(server LSPServer, params tea) tea {
    sus position tea = json_get(params, "position")
    sus text_document tea = json_get(params, "textDocument")
    sus uri tea = json_get_string(text_document, "uri")
    
    // Find word at position (simplified)
    sus doc_content tea = get_document_content(server, uri)
    sus hover_text tea = "CURSED language element"
    
    sus hover tea = json_object()
    hover = json_set(hover, "contents", json_string(hover_text))
    
    damn hover
}

// Format document
slay handle_document_formatting(server LSPServer, params tea) tea {
    sus text_document tea = json_get(params, "textDocument")
    sus uri tea = json_get_string(text_document, "uri")
    
    sus doc_content tea = get_document_content(server, uri)
    
    // Use the CURSED formatter
    sus formatted tea = format_cursed_code(doc_content)
    
    // Create text edit for entire document
    sus edit tea = json_object()
    sus range tea = json_object()
    sus start tea = json_object()
    start = json_set(start, "line", json_number(0))
    start = json_set(start, "character", json_number(0))
    sus end tea = json_object()
    end = json_set(end, "line", json_number(999999))  // End of document
    end = json_set(end, "character", json_number(0))
    range = json_set(range, "start", start)
    range = json_set(range, "end", end)
    
    edit = json_set(edit, "range", range)
    edit = json_set(edit, "newText", json_string(formatted))
    
    sus edits tea = json_array([edit])
    damn edits
}

// Simple CURSED syntax checking
slay check_cursed_syntax(content tea) []Diagnostic {
    sus diagnostics []Diagnostic = []
    sus lines []tea = split_str(content, "\n")
    
    sus line_num drip = 0
    bestie (line_num < len(lines)) {
        sus line tea = lines[line_num]
        
        // Check for missing semicolons
        ready (contains_str(line, "sus ") || contains_str(line, "slay ")) {
            ready (!ends_with(trim_str(line), ";") && !ends_with(trim_str(line), "{")) {
                sus diagnostic Diagnostic = Diagnostic{
                    range: create_range(line_num, 0, line_num, len_str(line)),
                    severity: 1,  // Error
                    code: "missing-semicolon",
                    source: "cursed-lsp",
                    message: "Missing semicolon at end of statement"
                }
                push(diagnostics, diagnostic)
            }
        }
        
        // Check for undefined variables (basic check)
        ready (contains_str(line, "vibez.spill") && !contains_str(line, "\"")) {
            // Very basic undefined variable check
            ready (contains_str(line, "undefined_var")) {
                sus diagnostic Diagnostic = Diagnostic{
                    range: create_range(line_num, 0, line_num, len_str(line)),
                    severity: 1,  // Error
                    code: "undefined-variable",
                    source: "cursed-lsp",
                    message: "Undefined variable referenced"
                }
                push(diagnostics, diagnostic)
            }
        }
        
        line_num = line_num + 1
    }
    
    damn diagnostics
}

// Publish diagnostics to client
slay publish_diagnostics(server LSPServer, uri tea, content tea) {
    sus diagnostics []Diagnostic = check_cursed_syntax(content)
    
    sus notification tea = json_object()
    notification = json_set(notification, "method", json_string("textDocument/publishDiagnostics"))
    
    sus params tea = json_object()
    params = json_set(params, "uri", json_string(uri))
    
    sus diag_array tea = json_array([])
    sus i drip = 0
    bestie (i < len(diagnostics)) {
        sus diag tea = diagnostic_to_json(diagnostics[i])
        diag_array = json_array_push(diag_array, diag)
        i = i + 1
    }
    params = json_set(params, "diagnostics", diag_array)
    
    notification = json_set(notification, "params", params)
    
    // Send notification (would be sent over stdio in real implementation)
    vibez.spill("Publishing diagnostics for: " + uri)
}

// Helper functions
slay get_document_content(server LSPServer, uri tea) tea {
    sus i drip = 0
    bestie (i < len(server.documents)) {
        ready (server.documents[i].uri == uri) {
            damn server.documents[i].content
        }
        i = i + 1
    }
    damn ""
}

slay create_range(start_line drip, start_char drip, end_line drip, end_char drip) tea {
    sus range tea = json_object()
    sus start tea = json_object()
    start = json_set(start, "line", json_number(start_line))
    start = json_set(start, "character", json_number(start_char))
    sus end tea = json_object()
    end = json_set(end, "line", json_number(end_line))
    end = json_set(end, "character", json_number(end_char))
    range = json_set(range, "start", start)
    range = json_set(range, "end", end)
    damn range
}

slay diagnostic_to_json(diag Diagnostic) tea {
    sus json_diag tea = json_object()
    json_diag = json_set(json_diag, "range", diag.range)
    json_diag = json_set(json_diag, "severity", json_number(diag.severity))
    json_diag = json_set(json_diag, "code", json_string(diag.code))
    json_diag = json_set(json_diag, "source", json_string(diag.source))
    json_diag = json_set(json_diag, "message", json_string(diag.message))
    damn json_diag
}

// Main LSP server loop (simplified)
slay main() {
    vibez.spill("CURSED Language Server starting...")
    
    sus server LSPServer = init_lsp_server()
    
    // Demonstration of LSP capabilities
    vibez.spill("LSP Server initialized with capabilities:")
    vibez.spill("- Text synchronization")
    vibez.spill("- Code completion")
    vibez.spill("- Hover information")
    vibez.spill("- Document formatting")
    vibez.spill("- Syntax diagnostics")
    
    // Example usage
    sus sample_params tea = json_object()
    sus init_result tea = handle_initialize(server, sample_params)
    vibez.spill("Initialize result: " + init_result)
}
