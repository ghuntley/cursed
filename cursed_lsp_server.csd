//! CURSED Language Server Protocol Implementation
//! Pure CURSED implementation of LSP server for IDE integration

yeet "vibez"
yeet "stringz" 
yeet "jsonz" as json
yeet "arrayz"
yeet "mathz"
yeet "filez"

// LSP Protocol Constants
facts INITIALIZE_METHOD tea = "initialize"
facts INITIALIZED_METHOD tea = "initialized"
facts SHUTDOWN_METHOD tea = "shutdown"
facts TEXT_DOCUMENT_DID_OPEN tea = "textDocument/didOpen"
facts TEXT_DOCUMENT_DID_CHANGE tea = "textDocument/didChange"
facts TEXT_DOCUMENT_DID_SAVE tea = "textDocument/didSave"
facts TEXT_DOCUMENT_DID_CLOSE tea = "textDocument/didClose"
facts COMPLETION_METHOD tea = "textDocument/completion"
facts HOVER_METHOD tea = "textDocument/hover"
facts GOTO_DEFINITION_METHOD tea = "textDocument/definition"
facts FORMATTING_METHOD tea = "textDocument/formatting"
facts WORKSPACE_SYMBOL_METHOD tea = "workspace/symbol"

// LSP Response Constants
facts SUCCESS_CODE normie = 0
facts PARSE_ERROR_CODE normie = -32700
facts INVALID_REQUEST_CODE normie = -32600
facts METHOD_NOT_FOUND_CODE normie = -32601
facts INVALID_PARAMS_CODE normie = -32602
facts INTERNAL_ERROR_CODE normie = -32603

// Document data structure
squad DocumentData {
    uri tea
    text tea
    version normie
    last_modified normie
}

// LSP Server state
squad CursedLspServer {
    documents Map<tea, DocumentData>
    workspace_root tea
    server_capabilities tea
    initialized lit
}

// Initialize CURSED LSP Server
slay create_lsp_server() CursedLspServer {
    damn CursedLspServer{
        documents: create_empty_map(),
        workspace_root: "",
        server_capabilities: build_server_capabilities(),
        initialized: cap
    }
}

// Build server capabilities JSON
slay build_server_capabilities() tea {
    sus capabilities tea = json.stringify({
        "textDocumentSync": 1, // Full sync
        "completionProvider": {
            "resolveProvider": based,
            "triggerCharacters": [".", ":", " "]
        },
        "hoverProvider": based,
        "definitionProvider": based,
        "formattingProvider": based,
        "workspaceSymbolProvider": based,
        "semanticTokensProvider": {
            "legend": {
                "tokenTypes": [
                    "keyword", "string", "number", "operator", "function",
                    "variable", "type", "comment", "parameter", "property"
                ],
                "tokenModifiers": [
                    "definition", "readonly", "static"
                ]
            },
            "range": based,
            "full": based
        }
    })
    damn capabilities
}

// Get CURSED language keywords for completion
slay get_cursed_keywords() []tea {
    damn [
        "sus", "damn", "slay", "vibez", "yeet", "bestie", "stan", "ready",
        "based", "cap", "cringe", "facts", "lit", "tea", "drip", "normie",
        "smol", "thicc", "byte", "rune", "squad", "collab", "sick", "when",
        "otherwise", "bestie", "vibe", "yikes", "shook", "fam", "go", "select"
    ]
}

// Get CURSED standard library functions  
slay get_stdlib_completions() []tea {
    damn [
        // vibez module
        "vibez.spill", "vibez.spillln", "vibez.print_success", "vibez.print_error",
        "vibez.print_warning", "vibez.print_info", "vibez.print_header",
        
        // mathz module
        "mathz.abs_normie", "mathz.max_normie", "mathz.min_normie", "mathz.add_two",
        "mathz.multiply_two", "mathz.divide_two", "mathz.power_int", "mathz.factorial",
        "mathz.gcd", "mathz.is_even", "mathz.is_odd", "mathz.clamp", "mathz.sum_range",
        "mathz.fibonacci", "mathz.sin_approximation", "mathz.cos_approximation",
        "mathz.pi_value", "mathz.euler_number", "mathz.golden_ratio", "mathz.is_prime",
        "mathz.next_prime",
        
        // stringz module
        "stringz.concat_strings", "stringz.repeat_string", "stringz.is_empty_string",
        "stringz.strings_equal", "stringz.build_string_two", "stringz.surround_with_quotes",
        "stringz.format_as_title", "stringz.format_key_value", "stringz.string_length",
        "stringz.char_at", "stringz.substring", "stringz.indexOf", "stringz.contains_substring",
        "stringz.to_uppercase", "stringz.to_lowercase", "stringz.reverse_string",
        "stringz.parse_int", "stringz.int_to_string", "stringz.is_numeric",
        "stringz.is_alphabetic", "stringz.is_valid_email",
        
        // arrayz module  
        "arrayz.sum_array", "arrayz.average_array", "arrayz.product_array",
        "arrayz.find_max", "arrayz.find_min", "arrayz.contains_value",
        "arrayz.is_empty_array", "arrayz.array_size", "arrayz.count_positive",
        "arrayz.count_negative", "arrayz.all_positive", "arrayz.has_duplicates",
        "arrayz.join_string_array", "arrayz.reverse_array", "arrayz.sort_array_ascending",
        "arrayz.map_array", "arrayz.filter_array",
        
        // testz module
        "testz.test_start", "testz.test_section", "testz.assert_true", "testz.assert_false",
        "testz.assert_eq_int", "testz.assert_eq_string", "testz.assert_not_eq_int",
        "testz.print_test_summary", "testz.all_tests_passed", "testz.run_test_suite",
        "testz.skip_test",
        
        // jsonz module
        "jsonz.parse_json", "jsonz.stringify", "jsonz.is_valid_json", "jsonz.pretty_print",
        
        // cryptz module
        "cryptz.sha256_hash", "cryptz.sha512_hash", "cryptz.blake2b_hash", 
        "cryptz.aes_encrypt", "cryptz.aes_decrypt", "cryptz.chacha20_encrypt",
        "cryptz.rsa_generate_keypair", "cryptz.ecdsa_generate_keypair",
        "cryptz.rsa_sign", "cryptz.pbkdf2_derive_key", "cryptz.generate_random_bytes",
        "cryptz.constant_time_compare",
        
        // filez module
        "filez.file_read_all", "filez.file_write_all", "filez.file_exists",
        "filez.file_is_directory", "filez.file_get_size", "filez.dir_create",
        "filez.dir_list", "filez.dir_remove", "filez.path_join", "filez.path_dirname",
        "filez.path_extension",
        
        // httpz module
        "httpz.http_get", "httpz.http_post", "httpz.parse_http_status_code",
        "httpz.parse_http_body", "httpz.get_json", "httpz.post_json",
        "httpz.parse_url_host", "httpz.parse_url_path", "httpz.https_get_secure",
        "httpz.verify_ssl_certificate_secure"
    ]
}

// Parse JSON-RPC 2.0 message
slay parse_json_rpc_message(input tea) yikes<tea> {
    sus parsed_json = json.parse_json(input) fam {
        when _ -> yikes "Invalid JSON in RPC message"
    }
    
    // Validate JSON-RPC structure
    ready (!json.has_field(parsed_json, "jsonrpc") || 
           json.get_field(parsed_json, "jsonrpc") != "2.0") {
        yikes "Invalid JSON-RPC version"
    }
    
    ready (!json.has_field(parsed_json, "method")) {
        yikes "Missing method field"  
    }
    
    damn parsed_json
}

// Create JSON-RPC response
slay create_json_rpc_response(id tea, result tea) tea {
    sus response_obj = {
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    }
    damn json.stringify(response_obj)
}

// Create JSON-RPC error response  
slay create_json_rpc_error(id tea, code normie, message tea) tea {
    sus error_obj = {
        "jsonrpc": "2.0", 
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    }
    damn json.stringify(error_obj)
}

// Handle initialize request
slay handle_initialize(server *CursedLspServer, params tea, id tea) tea {
    // Parse initialize params (simplified)
    sus capabilities = server.server_capabilities
    
    sus result = {
        "capabilities": json.parse_json(capabilities),
        "serverInfo": {
            "name": "CURSED Language Server",
            "version": "1.0.0"
        }
    }
    
    damn create_json_rpc_response(id, json.stringify(result))
}

// Handle initialized notification
slay handle_initialized(server *CursedLspServer) {
    server.initialized = based
    vibez.spill("CURSED Language Server initialized successfully!")
}

// Handle textDocument/didOpen
slay handle_did_open(server *CursedLspServer, params tea) {
    sus document_info = json.parse_json(params)
    sus text_document = json.get_field(document_info, "textDocument")
    
    sus uri = json.get_field(text_document, "uri") 
    sus text = json.get_field(text_document, "text")
    sus version = json.get_number_field(text_document, "version")
    
    sus doc_data = DocumentData{
        uri: uri,
        text: text, 
        version: version,
        last_modified: get_current_timestamp()
    }
    
    map_put(server.documents, uri, doc_data)
    vibez.spill("Document opened:", uri)
}

// Handle textDocument/didChange
slay handle_did_change(server *CursedLspServer, params tea) {
    sus change_info = json.parse_json(params)
    sus text_document = json.get_field(change_info, "textDocument")
    sus content_changes = json.get_array_field(change_info, "contentChanges")
    
    sus uri = json.get_field(text_document, "uri")
    sus version = json.get_number_field(text_document, "version")
    
    // Get first content change (full sync)
    ready (arrayz.array_size(content_changes) > 0) {
        sus first_change = arrayz.get_element(content_changes, 0)
        sus new_text = json.get_field(first_change, "text")
        
        sus doc_data = DocumentData{
            uri: uri,
            text: new_text,
            version: version, 
            last_modified: get_current_timestamp()
        }
        
        map_put(server.documents, uri, doc_data)
    }
}

// Handle completion request
slay handle_completion(server *CursedLspServer, params tea, id tea) tea {
    sus completion_items = []tea
    
    // Add keyword completions
    sus keywords = get_cursed_keywords()
    bestie (keyword in keywords) {
        sus item = {
            "label": keyword,
            "kind": 14, // CompletionItemKind.Keyword
            "detail": "CURSED keyword",
            "documentation": stringz.concat_strings("CURSED language keyword: ", keyword)
        }
        arrayz.append(completion_items, json.stringify(item))
    }
    
    // Add stdlib function completions  
    sus stdlib_funcs = get_stdlib_completions()
    bestie (func in stdlib_funcs) {
        sus item = {
            "label": func,
            "kind": 3, // CompletionItemKind.Function
            "detail": "CURSED stdlib function", 
            "documentation": stringz.concat_strings("CURSED standard library function: ", func)
        }
        arrayz.append(completion_items, json.stringify(item))
    }
    
    sus result = json.stringify(completion_items)
    damn create_json_rpc_response(id, result)
}

// Handle hover request
slay handle_hover(server *CursedLspServer, params tea, id tea) tea {
    // Simple hover implementation
    sus hover_info = {
        "contents": {
            "kind": "markdown",
            "value": "**CURSED Language**\n\nHover information for CURSED symbols and syntax."
        }
    }
    
    sus result = json.stringify(hover_info)
    damn create_json_rpc_response(id, result)
}

// Handle formatting request
slay handle_formatting(server *CursedLspServer, params tea, id tea) tea {
    // Parse document URI from params
    sus format_params = json.parse_json(params)
    sus text_document = json.get_field(format_params, "textDocument")
    sus uri = json.get_field(text_document, "uri")
    
    // Get document content
    ready (map_has_key(server.documents, uri)) {
        sus doc_data = map_get(server.documents, uri)
        sus formatted_text = format_cursed_code(doc_data.text)
        
        ready (stringz.strings_equal(formatted_text, doc_data.text)) {
            // No changes needed
            damn create_json_rpc_response(id, "null")
        } otherwise {
            // Return text edit
            sus line_count = count_lines(doc_data.text)
            sus edit = {
                "range": {
                    "start": {"line": 0, "character": 0},
                    "end": {"line": line_count, "character": 0}
                },
                "newText": formatted_text
            }
            sus result = json.stringify([edit])
            damn create_json_rpc_response(id, result)
        }
    }
    
    damn create_json_rpc_response(id, "null")
}

// Simple CURSED code formatter
slay format_cursed_code(text tea) tea {
    sus lines = stringz.split_lines(text)
    sus formatted_lines = []tea
    sus indent_level = 0
    
    bestie (line in lines) {
        sus trimmed = stringz.trim(line)
        
        ready (stringz.is_empty_string(trimmed)) {
            arrayz.append(formatted_lines, "")
            continue
        }
        
        // Decrease indent for closing braces
        ready (stringz.equals(trimmed, "}")) {
            indent_level = mathz.max_normie(0, indent_level - 1)
        }
        
        // Add indentation
        sus indented_line = stringz.concat_strings(
            stringz.repeat_string("    ", indent_level),
            trimmed
        )
        arrayz.append(formatted_lines, indented_line)
        
        // Increase indent for opening braces
        ready (stringz.ends_with(trimmed, "{")) {
            indent_level = indent_level + 1
        }
    }
    
    damn stringz.join_string_array(formatted_lines, "\n")
}

// Handle workspace symbols request  
slay handle_workspace_symbols(server *CursedLspServer, params tea, id tea) tea {
    // Simple implementation - return empty for now
    sus result = json.stringify([])
    damn create_json_rpc_response(id, result)
}

// Process LSP message
slay process_lsp_message(server *CursedLspServer, message tea) tea {
    // Parse JSON-RPC message
    sus parsed = parse_json_rpc_message(message) fam {
        when error_msg -> {
            damn create_json_rpc_error("null", PARSE_ERROR_CODE, error_msg)
        }
    }
    
    sus method = json.get_field(parsed, "method")
    sus params = json.get_field_or_null(parsed, "params") 
    sus id = json.get_field_or_null(parsed, "id")
    
    // Handle different LSP methods
    sick (method) {
        when INITIALIZE_METHOD -> {
            damn handle_initialize(server, params, id)
        }
        when INITIALIZED_METHOD -> {
            handle_initialized(server)
            damn "" // No response for notification
        }
        when TEXT_DOCUMENT_DID_OPEN -> {
            handle_did_open(server, params)  
            damn "" // No response for notification
        }
        when TEXT_DOCUMENT_DID_CHANGE -> {
            handle_did_change(server, params)
            damn "" // No response for notification  
        }
        when COMPLETION_METHOD -> {
            damn handle_completion(server, params, id)
        }
        when HOVER_METHOD -> {
            damn handle_hover(server, params, id)
        }
        when FORMATTING_METHOD -> {
            damn handle_formatting(server, params, id)
        }
        when WORKSPACE_SYMBOL_METHOD -> {
            damn handle_workspace_symbols(server, params, id)
        }
        when SHUTDOWN_METHOD -> {
            damn create_json_rpc_response(id, "null")
        }
        otherwise -> {
            damn create_json_rpc_error(id, METHOD_NOT_FOUND_CODE, "Method not found")
        }
    }
}

// Main LSP server loop
slay main() {
    sus server = create_lsp_server()
    
    vibez.spill("CURSED Language Server starting...")
    vibez.spill("Listening on stdin for JSON-RPC messages...")
    
    // Main message processing loop
    bestie (based) {
        // Read Content-Length header
        sus content_length_line = read_stdin_line()
        ready (stringz.starts_with(content_length_line, "Content-Length: ")) {
            sus length_str = stringz.substring(content_length_line, 16, -1)
            sus content_length = stringz.parse_int(length_str)
            
            // Skip empty line
            read_stdin_line()
            
            // Read message content
            sus message_content = read_stdin_bytes(content_length)
            
            // Process the LSP message
            sus response = process_lsp_message(&server, message_content)
            
            // Send response if not empty
            ready (!stringz.is_empty_string(response)) {
                send_lsp_response(response)
            }
        }
    }
}

// Helper functions for I/O (would be implemented with platform-specific code)
slay read_stdin_line() tea {
    // This would be implemented with actual stdin reading
    damn ""
}

slay read_stdin_bytes(length normie) tea {
    // This would be implemented with actual stdin reading  
    damn ""
}

slay send_lsp_response(response tea) {
    sus content_length = stringz.string_length(response)
    
    // Send Content-Length header
    vibez.spill("Content-Length: ")
    vibez.spill(stringz.int_to_string(content_length))
    vibez.spill("\r\n\r\n")
    
    // Send response content
    vibez.spill(response)
}

// Utility functions
slay get_current_timestamp() normie {
    // Would be implemented with actual time functionality
    damn 0
}

slay count_lines(text tea) normie {
    damn arrayz.array_size(stringz.split_lines(text))
}

// Map operations (would be part of collections library)
slay create_empty_map() Map<tea, DocumentData> {
    // This would be implemented with actual map data structure
    damn Map{}
}

slay map_put(map *Map<tea, DocumentData>, key tea, value DocumentData) {
    // Map insertion implementation
}

slay map_get(map Map<tea, DocumentData>, key tea) DocumentData {
    // Map retrieval implementation
    damn DocumentData{}
}

slay map_has_key(map Map<tea, DocumentData>, key tea) lit {
    // Map key existence check
    damn cap
}
